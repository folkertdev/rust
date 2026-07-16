use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::TerminatorKind;
use rustc_middle::query::Providers;
use rustc_middle::ty::TyCtxt;

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

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { uses_tail_call, ..*providers };
}
