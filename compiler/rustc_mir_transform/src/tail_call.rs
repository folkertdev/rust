use rustc_data_structures::thin_vec::ThinVec;
use rustc_hir::Safety;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_hir::lang_items::LangItem;
use rustc_index::IndexVec;
use rustc_middle::mir::*;
use rustc_middle::query::Providers;
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{DUMMY_SP, Span, Spanned};

/// Returns `true` if the body of `def_id` contains a guaranteed tail call (`become`).
///
/// Cross-crate results are read from metadata; this (local) provider only runs for local defs.
fn uses_tail_call(tcx: TyCtxt<'_>, def_id: LocalDefId) -> bool {
    // `become` can only appear in function-like bodies.
    tcx.def_kind(def_id).is_fn_like()
        && tcx
            .mir_built(def_id)
            .borrow()
            .basic_blocks
            .iter()
            .filter_map(|block| block.terminator.as_ref())
            .any(|term| matches!(term.kind, TerminatorKind::TailCall { .. }))
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { uses_tail_call, ..*providers };
}

/// The continuation type `TailNext<Args, Ret>` driven by the tail-call trampoline, where `Args` is
/// a function's (tupled) argument list and `Ret` its return type.
pub(crate) fn tail_next_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    args_tuple: Ty<'tcx>,
    ret: Ty<'tcx>,
) -> Ty<'tcx> {
    Ty::new_adt(
        tcx,
        tcx.adt_def(tcx.require_lang_item(LangItem::TailNext, DUMMY_SP)),
        tcx.mk_args(&[args_tuple.into(), ret.into()]),
    )
}

/// Adds a fresh local holding a `fn(Args) -> TailNext<Args, Ret>` pointer to `callee`'s tail-call
/// shim, and returns that local together with the statement that initializes it.
///
/// The shim is named through the `tail_shim` lang item and reified with `ReifyFnPointer`, so the
/// reference goes through the ordinary `fn`-item machinery and monomorphizes correctly. This is
/// what makes generic callees work — a concrete function-pointer constant could not.
pub(crate) fn reify_tail_call_shim<'tcx>(
    tcx: TyCtxt<'tcx>,
    local_decls: &mut IndexVec<Local, LocalDecl<'tcx>>,
    callee: DefId,
    callee_args: ty::GenericArgsRef<'tcx>,
    args_tuple: Ty<'tcx>,
    ret: Ty<'tcx>,
    span: Span,
) -> (Local, Statement<'tcx>) {
    // `fn(Args) -> TailNext<Args, Ret>`, the type of every tail-call shim.
    let ptr_ty = Ty::new_fn_ptr(
        tcx,
        ty::Binder::dummy(
            tcx.mk_fn_sig_safe_rust_abi([args_tuple], tail_next_ty(tcx, args_tuple, ret)),
        ),
    );
    let callee_fn_ty = Ty::new_fn_def(tcx, callee, ty::Binder::dummy(callee_args));
    let handle = Operand::Constant(Box::new(ConstOperand {
        span,
        user_ty: None,
        const_: Const::zero_sized(Ty::new_fn_def(
            tcx,
            tcx.require_lang_item(LangItem::TailShim, span),
            ty::Binder::dummy(tcx.mk_args(&[callee_fn_ty.into(), args_tuple.into(), ret.into()])),
        )),
    }));
    let cast = Rvalue::Cast(
        CastKind::PointerCoercion(
            PointerCoercion::ReifyFnPointer(Safety::Safe),
            CoercionSource::Implicit,
        ),
        handle,
        ptr_ty,
    );
    let local = local_decls.push(LocalDecl::new(ptr_ty, span));
    let stmt = Statement::new(
        SourceInfo::outermost(span),
        StatementKind::Assign(Box::new((Place::from(local), cast))),
    );
    (local, stmt)
}

/// Replaces the body of a `become`-using function with a trampoline that drives its tail-call shim
/// through `core::tail_call::tail_eval`, for use on targets that cannot lower guaranteed tail calls
/// directly.
///
/// `fn f(a, b) -> Ret { .. become .. }` becomes `fn f(a, b) -> Ret { tail_eval(f_shim, (a, b)) }`.
pub(super) struct LowerTailCall;

impl<'tcx> crate::MirPass<'tcx> for LowerTailCall {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        let def_id = body.source.def_id();
        // Only rewrite `become`-using function bodies (not promoteds), and only on targets that use
        // the fallback rather than native `musttail`.
        if !tcx.sess.use_tail_call_fallback()
            || body.source.promoted.is_some()
            || !tcx.uses_tail_call(def_id)
        {
            return;
        }

        let span = body.span;
        let source_info = SourceInfo::outermost(span);

        let ret_ty = body.return_ty();
        let inputs: Vec<Ty<'tcx>> = body.args_iter().map(|l| body.local_decls[l].ty).collect();
        let arg_locals: Vec<Local> = body.args_iter().collect();
        let args_tuple = Ty::new_tup(tcx, &inputs);

        // `tail_eval::<Args, Ret>`.
        let tail_eval = Operand::Constant(Box::new(ConstOperand {
            span,
            user_ty: None,
            const_: Const::zero_sized(Ty::new_fn_def(
                tcx,
                tcx.require_lang_item(LangItem::TailEval, span),
                ty::Binder::dummy(tcx.mk_args(&[args_tuple.into(), ret_ty.into()])),
            )),
        }));

        // Drop everything but the return place and the argument locals, then add fresh locals.
        body.local_decls.truncate(body.arg_count + 1);
        let tuple_local = body.local_decls.push(LocalDecl::new(args_tuple, span));
        let (shim_ptr_local, shim_stmt) = reify_tail_call_shim(
            tcx,
            &mut body.local_decls,
            def_id,
            ty::GenericArgs::identity_for_item(tcx, def_id),
            args_tuple,
            ret_ty,
            span,
        );
        body.var_debug_info.clear();

        // bb0: `_shim = &f_shim; _tuple = (move a, ..); _0 = tail_eval(move _shim, move _tuple) -> bb1`.
        let tuple_stmt = Statement::new(
            source_info,
            StatementKind::Assign(Box::new((
                Place::from(tuple_local),
                Rvalue::Aggregate(
                    Box::new(AggregateKind::Tuple),
                    arg_locals.iter().map(|&l| Operand::Move(Place::from(l))).collect(),
                ),
            ))),
        );
        let bb0 = BasicBlockData::new_stmts(
            vec![shim_stmt, tuple_stmt],
            Some(Terminator {
                source_info,
                kind: TerminatorKind::Call {
                    func: tail_eval,
                    args: [
                        Spanned { node: Operand::Move(Place::from(shim_ptr_local)), span },
                        Spanned { node: Operand::Move(Place::from(tuple_local)), span },
                    ]
                    .into_iter()
                    .collect(),
                    destination: Place::return_place(),
                    target: Some(BasicBlock::from_usize(1)),
                    unwind: UnwindAction::Continue,
                    call_source: CallSource::Misc,
                    fn_span: span,
                },
                attributes: ThinVec::new(),
            }),
            false,
        );
        // bb1: `return`.
        let bb1 = BasicBlockData::new_stmts(
            vec![],
            Some(Terminator {
                source_info,
                kind: TerminatorKind::Return,
                attributes: ThinVec::new(),
            }),
            false,
        );

        *body.basic_blocks_mut() = IndexVec::from_raw(vec![bb0, bb1]);
    }

    fn is_required(&self) -> bool {
        true
    }
}
