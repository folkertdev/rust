use rustc_hir::intravisit::FnKind;
use rustc_hir::{self as hir};
use rustc_middle::ty::{self, Ty};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

use crate::lints::DependenceOnUnstableLayoutDetailsLint;
use crate::{LateContext, LateLintPass, LintContext};

declare_lint! {
    /// The `dependence_on_unstable_layout_details` lint detects types that cross an FFI boundary
    /// whose in-memory layout is not guaranteed to be stable across compiler versions or upstream
    /// crate updates.
    ///
    /// ### Example
    ///
    /// ```rust
    /// struct Unstable {
    ///     a: u8,
    ///     b: u16,
    /// }
    ///
    /// #[warn(dependence_on_unstable_layout_details)]
    /// extern "C" fn store(value: Unstable) {
    ///     let _ = value;
    /// }
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// fixme
    pub DEPENDENCE_ON_UNSTABLE_LAYOUT_DETAILS,
    Allow,
    "FFI boundary depends on layout details that are not guaranteed to be stable"
}

declare_lint_pass!(DependenceOnUnstableLayoutDetails => [DEPENDENCE_ON_UNSTABLE_LAYOUT_DETAILS]);

impl<'tcx> LateLintPass<'tcx> for DependenceOnUnstableLayoutDetails {
    fn check_foreign_item(&mut self, cx: &LateContext<'tcx>, it: &'tcx hir::ForeignItem<'tcx>) {
        // The Rust ABI is itself unstable.
        if cx.tcx.hir_get_foreign_abi(it.hir_id()).is_rustic_abi() {
            return;
        }

        match it.kind {
            hir::ForeignItemKind::Fn(sig, _, _) => {
                check_fn_boundary(cx, it.owner_id.def_id, sig.decl);
            }
            hir::ForeignItemKind::Static(hir_ty, _, _) => {
                let ty = cx.tcx.type_of(it.owner_id).instantiate_identity().skip_norm_wip();
                check_boundary_ty(cx, hir_ty.span, ty);
            }
            hir::ForeignItemKind::Type => {}
        }
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx hir::FnDecl<'_>,
        _: &'tcx hir::Body<'_>,
        _: Span,
        id: LocalDefId,
    ) {
        // The Rust ABI is itself unstable. This check also filters out closures.
        if kind.abi().is_rustic_abi() {
            return;
        }

        check_fn_boundary(cx, id, decl);
    }
}

fn check_fn_boundary<'tcx>(cx: &LateContext<'tcx>, def_id: LocalDefId, decl: &hir::FnDecl<'_>) {
    let sig = cx.tcx.fn_sig(def_id).instantiate_identity().skip_norm_wip();
    let sig = cx.tcx.instantiate_bound_regions_with_erased(sig);

    for (input_ty, input_hir) in sig.inputs().iter().zip(decl.inputs) {
        check_boundary_ty(cx, input_hir.span, *input_ty);
    }

    if let hir::FnRetTy::Return(ret_hir) = decl.output {
        check_boundary_ty(cx, ret_hir.span, sig.output());
    }
}

fn check_boundary_ty<'tcx>(cx: &LateContext<'tcx>, span: Span, ty: Ty<'tcx>) {
    if let Some((culprit, reason)) = find_unstable_layout(cx, ty) {
        cx.emit_span_lint(
            DEPENDENCE_ON_UNSTABLE_LAYOUT_DETAILS,
            span,
            DependenceOnUnstableLayoutDetailsLint { culprit, reason },
        );
    }
}

/// Recursively walk a type to check whether there are any unstable parts.
fn find_unstable_layout<'tcx>(
    cx: &LateContext<'tcx>,
    ty: Ty<'tcx>,
) -> Option<(Ty<'tcx>, &'static str)> {
    let tcx = cx.tcx;

    match *ty.kind() {
        // Scalars have a fixed layout.
        ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Float(_) => None,

        // Pointers, references and function pointers have a fixed layout.
        ty::RawPtr(..) | ty::Ref(..) | ty::FnPtr(..) | ty::FnDef(..) => None,

        // Has no layout, which is stable I guess.
        ty::Never => None,

        // Arrays, slices and pattern types are as stable as their element type.
        ty::Array(elem, _) | ty::Slice(elem) | ty::Pat(elem, _) => find_unstable_layout(cx, elem),

        // Same as [u8].
        ty::Str => None,

        // Tuples (other than the unit type) have no layout guarantee.
        ty::Tuple(elems) => {
            let msg = "tuples have an unspecified layout that may change between compiler versions";

            if elems.is_empty() { Some((ty, msg)) } else { None }
        }

        ty::Adt(adt_def, args) => {
            let repr = adt_def.repr();

            // Lint if the surface type has an unstable layout.
            if !(repr.c() || repr.linear() || repr.transparent() || repr.int.is_some()) {
                return Some((
                    ty,
                    "types with the default representation have an unspecified layout that may \
                     change between compiler versions",
                ));
            }

            // Check whether any of the fields has an unstable layout.
            adt_def.all_fields().find_map(|field| {
                let field_ty = field.ty(tcx, args).skip_norm_wip();
                find_unstable_layout(cx, field_ty)
            })
        }

        // Closures and coroutines use the default representation.
        ty::Closure(..)
        | ty::CoroutineClosure(..)
        | ty::Coroutine(..)
        | ty::CoroutineWitness(..) => {
            Some((ty, "closures and coroutines have an unspecified layout"))
        }

        // Various.
        ty::UnsafeBinder(_)
        | ty::Dynamic(_, _)
        | ty::Foreign(_)
        | ty::Alias(_, _)
        | ty::Param(_)
        | ty::Bound(_, _)
        | ty::Placeholder(_)
        | ty::Infer(_)
        | ty::Error(_) => None,
    }
}
