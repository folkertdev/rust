use rustc_data_structures::thin_vec::ThinVec;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::lang_items::LangItem;
use rustc_index::IndexVec;
use rustc_middle::mir::interpret::{CTFE_ALLOC_SALT, Pointer, Scalar};
use rustc_middle::mir::*;
use rustc_middle::query::Providers;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::Spanned;

/// Returns `true` if the body of `def_id` contains a guaranteed tail call (`become`).
fn uses_tail_call(tcx: TyCtxt<'_>, def_id: LocalDefId) -> bool {
    // `become` can only appear in function-like bodies.
    if !tcx.def_kind(def_id).is_fn_like() {
        return false;
    }

    let body = &*tcx.mir_built(def_id).borrow();

    body.basic_blocks
        .iter()
        .filter_map(|block| block.terminator.as_ref())
        .any(|term| matches!(term.kind, TerminatorKind::TailCall { .. }))
}

/// Captures a copy of `def_id`'s MIR that still contains its `TailCall` terminators, to serve as
/// the source for its tail-call trampoline shim.
///
/// This must be forced *before* `optimized_mir` steals the drops-elaborated body and (on fallback
/// targets) rewrites it into a `tail_eval` trampoline.
fn mir_tail_call_shim_source<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> Body<'tcx> {
    tcx.mir_drops_elaborated_and_const_checked(def_id).borrow().clone()
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { uses_tail_call, mir_tail_call_shim_source, ..*providers };
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
        // Only rewrite the actual function body (not promoteds) of `become`-using functions.
        if body.source.promoted.is_some() {
            return;
        }
        let Some(local) = def_id.as_local() else { return };
        if !tcx.uses_tail_call(local) {
            return;
        }

        // Ship non-generic first: generic `become`-using functions would need a way to name a
        // polymorphic shim instance, which the concrete `reserve_and_set_fn_alloc` const cannot do.
        if tcx.generics_of(def_id).requires_monomorphization(tcx) {
            tcx.dcx().span_err(
                body.span,
                "the portable guaranteed tail-call fallback does not yet support generic functions",
            );
            return;
        }

        let span = body.span;
        let source_info = SourceInfo::outermost(span);

        let ret_ty = body.return_ty();
        let inputs: Vec<Ty<'tcx>> = body.args_iter().map(|l| body.local_decls[l].ty).collect();
        let arg_locals: Vec<Local> = body.args_iter().collect();
        let args_tuple = Ty::new_tup(tcx, &inputs);

        // `TailNext<args_tuple, ret_ty>`.
        let tail_next_ty = Ty::new_adt(
            tcx,
            tcx.adt_def(tcx.require_lang_item(LangItem::TailNext, span)),
            tcx.mk_args(&[args_tuple.into(), ret_ty.into()]),
        );

        // A pointer to this function's own tail-call shim (concrete, since we're non-generic).
        let shim_args = ty::GenericArgs::identity_for_item(tcx, def_id);
        let shim = ty::Instance {
            def: ty::InstanceKind::Shim(ty::ShimKind::TailCall(def_id, shim_args)),
            args: shim_args,
        };
        let shim_fn_ptr_ty =
            Ty::new_fn_ptr(tcx, ty::Binder::dummy(tcx.mk_fn_sig_safe_rust_abi([args_tuple], tail_next_ty)));
        let alloc_id = tcx.reserve_and_set_fn_alloc(shim, CTFE_ALLOC_SALT);
        let shim_ptr = Operand::Constant(Box::new(ConstOperand {
            span,
            user_ty: None,
            const_: Const::Val(
                ConstValue::Scalar(Scalar::from_pointer(Pointer::from(alloc_id), &tcx)),
                shim_fn_ptr_ty,
            ),
        }));

        // `tail_eval::<args_tuple, ret_ty>`.
        let tail_eval_did = tcx.require_lang_item(LangItem::TailEval, span);
        let tail_eval_args = tcx.mk_args(&[args_tuple.into(), ret_ty.into()]);
        let tail_eval = Operand::Constant(Box::new(ConstOperand {
            span,
            user_ty: None,
            const_: Const::zero_sized(Ty::new_fn_def(
                tcx,
                tail_eval_did,
                ty::Binder::dummy(tail_eval_args),
            )),
        }));

        // Drop everything but the return place and the argument locals, then add the tuple local.
        body.local_decls.truncate(body.arg_count + 1);
        let tuple_local = body.local_decls.push(LocalDecl::new(args_tuple, span));
        body.var_debug_info.clear();

        // bb0: `_tuple = (move a, move b, ..); _0 = tail_eval(shim, move _tuple) -> bb1`.
        let tuple_rvalue = Rvalue::Aggregate(
            Box::new(AggregateKind::Tuple),
            arg_locals.iter().map(|&l| Operand::Move(Place::from(l))).collect(),
        );
        let bb0 = BasicBlockData::new_stmts(
            vec![Statement::new(
                source_info,
                StatementKind::Assign(Box::new((Place::from(tuple_local), tuple_rvalue))),
            )],
            Some(Terminator {
                source_info,
                kind: TerminatorKind::Call {
                    func: tail_eval,
                    args: [
                        Spanned { node: shim_ptr, span },
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

        let blocks = body.basic_blocks_mut();
        *blocks = IndexVec::from_raw(vec![bb0, bb1]);
    }

    fn is_required(&self) -> bool {
        true
    }
}
