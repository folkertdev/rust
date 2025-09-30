use rustc_abi::{BackendRepr, ExternAbi, Float, Integer, Primitive, Scalar};
use rustc_errors::{DiagCtxtHandle, E0781, struct_span_code_err};
use rustc_hir::{self as hir, HirId};
use rustc_middle::bug;
use rustc_middle::ty::layout::{LayoutError, TyAndLayout};
use rustc_middle::ty::{self, TyCtxt};

use crate::errors;

/// Check conditions on inputs and outputs that the cmse ABIs impose: arguments and results MUST be
/// returned via registers (i.e. MUST NOT spill to the stack). LLVM will also validate these
/// conditions, but by checking them here rustc can emit nicer error messages.
pub(crate) fn validate_cmse_abi<'tcx>(
    tcx: TyCtxt<'tcx>,
    dcx: DiagCtxtHandle<'_>,
    hir_id: HirId,
    abi: ExternAbi,
    fn_sig: ty::PolyFnSig<'tcx>,
) {
    match abi {
        ExternAbi::CmseNonSecureCall => {
            let hir_node = tcx.hir_node(hir_id);
            let hir::Node::Ty(hir::Ty {
                span: fn_ptr_span,
                kind: hir::TyKind::FnPtr(fn_ptr_ty),
                ..
            }) = hir_node
            else {
                let span = match tcx.parent_hir_node(hir_id) {
                    hir::Node::Item(hir::Item {
                        kind: hir::ItemKind::ForeignMod { .. },
                        span,
                        ..
                    }) => *span,
                    _ => tcx.hir_span(hir_id),
                };
                struct_span_code_err!(
                    dcx,
                    span,
                    E0781,
                    "the `\"cmse-nonsecure-call\"` ABI is only allowed on function pointers"
                )
                .emit();
                return;
            };

            if let Err(layout_err) = is_valid_cmse_call(tcx, dcx, fn_sig, fn_ptr_ty.decl) {
                if should_emit_generic_error(abi, layout_err) {
                    dcx.emit_err(errors::CmseCallGeneric { span: *fn_ptr_span });
                }
            }
        }
        ExternAbi::CmseNonSecureEntry => {
            let hir_node = tcx.hir_node(hir_id);
            let Some(hir::FnSig { decl, span: fn_sig_span, .. }) = hir_node.fn_sig() else {
                // might happen when this ABI is used incorrectly. That will be handled elsewhere
                return;
            };

            // An `extern "cmse-nonsecure-entry"` function cannot be c-variadic. We run
            // into https://github.com/rust-lang/rust/issues/132142 if we don't explicitly bail.
            if decl.c_variadic {
                return;
            }

            if let Err(layout_err) = is_valid_cmse_entry(tcx, dcx, fn_sig, decl) {
                if should_emit_generic_error(abi, layout_err) {
                    dcx.emit_err(errors::CmseEntryGeneric { span: *fn_sig_span });
                }
            }
        }
        _ => (),
    }
}
/// Validate the signature of a cmse-nonsecure-call function
///
/// - the arguments must fit in 4 registers
/// - the output layout must fit in the output registers
fn is_valid_cmse_call<'tcx>(
    tcx: TyCtxt<'tcx>,
    dcx: DiagCtxtHandle<'_>,
    fn_sig: ty::PolyFnSig<'tcx>,
    fn_decl: &hir::FnDecl<'tcx>,
) -> Result<(), &'tcx LayoutError<'tcx>> {
    let abi = ExternAbi::CmseNonSecureCall;
    let mut accum = 0u64;
    let mut excess_argument_spans = Vec::new();

    // this type is only used for layout computation, which does not rely on regions
    let fn_sig = tcx.instantiate_bound_regions_with_erased(fn_sig);
    let fn_sig = tcx.erase_and_anonymize_regions(fn_sig);
    let typing_env = ty::TypingEnv::fully_monomorphized();

    for (ty, hir_ty) in fn_sig.inputs().iter().zip(fn_decl.inputs) {
        let layout = tcx.layout_of(typing_env.as_query_input(*ty))?;

        let align = layout.layout.align().bytes();
        let size = layout.layout.size().bytes();

        accum += size;
        accum = accum.next_multiple_of(Ord::max(4, align));

        // i.e. exceeds 4 32-bit registers
        if accum > 16 {
            excess_argument_spans.push(hir_ty.span);
        }
    }

    if !excess_argument_spans.is_empty() {
        // fn f(x: u32, y: u32, z: u32, w: u16, q: u16) -> u32,
        //                                      ^^^^^^
        let plural = excess_argument_spans.len() != 1;
        dcx.emit_err(errors::CmseInputsStackSpill { spans: excess_argument_spans, plural, abi });
    }

    let ret_layout = tcx.layout_of(typing_env.as_query_input(fn_sig.output()))?;
    if !is_valid_cmse_output_layout(ret_layout) {
        let span = fn_decl.output.span();
        dcx.emit_err(errors::CmseOutputStackSpill { span, abi });
    }

    Ok(())
}

/// Validate the signature of a cmse-nonsecure-entry function
///
/// - the arguments must fit in 4 registers
/// - the output layout must fit in the output registers
fn is_valid_cmse_entry<'tcx>(
    tcx: TyCtxt<'tcx>,
    dcx: DiagCtxtHandle<'_>,
    fn_sig: ty::PolyFnSig<'tcx>,
    fn_decl: &hir::FnDecl<'tcx>,
) -> Result<(), &'tcx LayoutError<'tcx>> {
    let abi = ExternAbi::CmseNonSecureEntry;
    let mut accum = 0u64;
    let mut excess_argument_spans = Vec::new();

    // this type is only used for layout computation, which does not rely on regions
    let fn_sig = tcx.instantiate_bound_regions_with_erased(fn_sig);
    let fn_sig = tcx.erase_and_anonymize_regions(fn_sig);
    let typing_env = ty::TypingEnv::fully_monomorphized();

    for (ty, hir_ty) in fn_sig.inputs().iter().zip(fn_decl.inputs) {
        let layout = tcx.layout_of(typing_env.as_query_input(*ty))?;

        let align = layout.layout.align().bytes();
        let size = layout.layout.size().bytes();

        accum += size;
        accum = accum.next_multiple_of(Ord::max(4, align));

        // i.e. exceeds 4 32-bit registers
        if accum > 16 {
            excess_argument_spans.push(hir_ty.span);
        }
    }

    if !excess_argument_spans.is_empty() {
        // fn f(x: u32, y: u32, z: u32, w: u16, q: u16) -> u32,
        //                                      ^^^^^^
        let plural = excess_argument_spans.len() != 1;
        dcx.emit_err(errors::CmseInputsStackSpill { spans: excess_argument_spans, plural, abi });
    }

    let ret_layout = tcx.layout_of(typing_env.as_query_input(fn_sig.output()))?;
    if !is_valid_cmse_output_layout(ret_layout) {
        let span = fn_decl.output.span();
        dcx.emit_err(errors::CmseOutputStackSpill { span, abi });
    }

    Ok(())
}

/// Returns whether the output will fit into the available registers
fn is_valid_cmse_output_layout<'tcx>(layout: TyAndLayout<'tcx>) -> bool {
    let size = layout.layout.size().bytes();

    if size <= 4 {
        return true;
    } else if size > 8 {
        return false;
    }

    // Accept scalar 64-bit types.
    let BackendRepr::Scalar(scalar) = layout.layout.backend_repr else {
        return false;
    };

    let Scalar::Initialized { value, .. } = scalar else {
        return false;
    };

    matches!(value, Primitive::Int(Integer::I64, _) | Primitive::Float(Float::F64))
}

fn should_emit_generic_error<'tcx>(abi: ExternAbi, layout_err: &'tcx LayoutError<'tcx>) -> bool {
    use LayoutError::*;

    match layout_err {
        TooGeneric(ty) => {
            match abi {
                ExternAbi::CmseNonSecureCall => {
                    // prevent double reporting of this error
                    !ty.is_impl_trait()
                }
                ExternAbi::CmseNonSecureEntry => true,
                _ => bug!("invalid ABI: {abi}"),
            }
        }
        Unknown(..)
        | SizeOverflow(..)
        | InvalidSimd { .. }
        | NormalizationFailure(..)
        | ReferencesError(..)
        | Cycle(..) => {
            false // not our job to report these
        }
    }
}
