#![warn(clippy::arithmetic_side_effects)]

mod atomic;
mod math;
mod simd;

pub use self::atomic::AtomicRmwOp;

#[rustfmt::skip] // prevent `use` reordering
use rand::Rng;
use rustc_abi::Size;
use rustc_middle::{mir, ty};
use rustc_span::{Symbol, sym};

use self::atomic::EvalContextExt as _;
use self::math::EvalContextExt as _;
use self::simd::EvalContextExt as _;
use crate::*;

use crate::machine::VaListKey;
use rustc_const_eval::interpret::Pointer;

/// Compute a stable key for a particular `VaListImpl` object in memory.
///
/// `ap_ref` is the a value of type `&mut VaListImpl<'_>`.
fn va_list_key<'tcx>(
    ecx: &MiriInterpCx<'tcx>,
    ap_ref: &OpTy<'tcx>,
) -> InterpResult<'tcx, VaListKey> {
    let va_list_ty =
        ap_ref.layout().ty.builtin_deref(true).expect("va_list argument should be a reference");

    let base_layout = ecx.layout_of(va_list_ty)?;
    let tag_layout = ecx.layout_of(ecx.tcx.types.usize)?;

    let tag_scalar = ecx.deref_pointer_and_read(ap_ref, 0, base_layout, tag_layout)?;
    let id = tag_scalar.to_u64()?;

    interp_ok(VaListKey(id))
}

fn intrinsic_va_arg<'tcx>(
    ecx: &mut MiriInterpCx<'tcx>,
    va_list: &OpTy<'tcx>,
    dest: &MPlaceTy<'tcx>,
) -> InterpResult<'tcx, ()> {
    // Find which variable argument list we're reading from, and what the current position is.
    let key = va_list_key(ecx, va_list)?;

    // Get and mutate its cursor.
    let cursor = ecx
        .machine
        .vararg_cursors
        .get_mut(&key)
        .ok_or_else(|| err_ub_format!("using an uninitialized va_list in va_arg"))?;

    let va_arg_idx: usize =
        cursor.index.try_into().map_err(|_| err_ub_format!("va_list index overflow"))?;
    cursor.index = cursor.index.strict_add(1);

    let frame_idx = cursor.frame_idx;
    let frame = &ecx.machine.threads.active_thread_stack()[frame_idx];

    // NOTE: the clone is needed to satisfy the borrow checker.
    let Some(mplace) = frame.varargs.get(va_arg_idx).cloned() else {
        return Err(err_ub_format!("va_arg past end of C variadic arguments")).into();
    };

    // A bitcast is allowed, turning e.g. an argument passed as i32 into a u32.
    ecx.copy_op_allow_transmute(&mplace, dest)?;

    interp_ok(())
}

/// Check that the number of args is what we expect.
fn check_intrinsic_arg_count<'a, 'tcx, const N: usize>(
    args: &'a [OpTy<'tcx>],
) -> InterpResult<'tcx, &'a [OpTy<'tcx>; N]>
where
    &'a [OpTy<'tcx>; N]: TryFrom<&'a [OpTy<'tcx>]>,
{
    if let Ok(ops) = args.try_into() {
        return interp_ok(ops);
    }
    throw_ub_format!(
        "incorrect number of arguments for intrinsic: got {}, expected {}",
        args.len(),
        N
    )
}

impl<'tcx> EvalContextExt<'tcx> for crate::MiriInterpCx<'tcx> {}
pub trait EvalContextExt<'tcx>: crate::MiriInterpCxExt<'tcx> {
    fn call_intrinsic(
        &mut self,
        instance: ty::Instance<'tcx>,
        args: &[OpTy<'tcx>],
        dest: &PlaceTy<'tcx>,
        ret: Option<mir::BasicBlock>,
        unwind: mir::UnwindAction,
    ) -> InterpResult<'tcx, Option<ty::Instance<'tcx>>> {
        let this = self.eval_context_mut();

        // Force use of fallback body, if available.
        if this.machine.force_intrinsic_fallback
            && !this.tcx.intrinsic(instance.def_id()).unwrap().must_be_overridden
        {
            return interp_ok(Some(ty::Instance {
                def: ty::InstanceKind::Item(instance.def_id()),
                args: instance.args,
            }));
        }

        // See if the core engine can handle this intrinsic.
        if this.eval_intrinsic(instance, args, dest, ret)? {
            return interp_ok(None);
        }
        let intrinsic_name = this.tcx.item_name(instance.def_id());
        let intrinsic_name = intrinsic_name.as_str();

        // FIXME: avoid allocating memory
        let dest = this.force_allocation(dest)?;

        match this.emulate_intrinsic_by_name(intrinsic_name, instance.args, args, &dest, ret)? {
            EmulateItemResult::NotSupported => {
                // We haven't handled the intrinsic, let's see if we can use a fallback body.
                if this.tcx.intrinsic(instance.def_id()).unwrap().must_be_overridden {
                    throw_unsup_format!("unimplemented intrinsic: `{intrinsic_name}`")
                }
                let intrinsic_fallback_is_spec = Symbol::intern("intrinsic_fallback_is_spec");
                if this
                    .tcx
                    .get_attrs_by_path(instance.def_id(), &[sym::miri, intrinsic_fallback_is_spec])
                    .next()
                    .is_none()
                {
                    throw_unsup_format!(
                        "Miri can only use intrinsic fallback bodies that exactly reflect the specification: they fully check for UB and are as non-deterministic as possible. After verifying that `{intrinsic_name}` does so, add the `#[miri::intrinsic_fallback_is_spec]` attribute to it; also ping @rust-lang/miri when you do that"
                    );
                }
                interp_ok(Some(ty::Instance {
                    def: ty::InstanceKind::Item(instance.def_id()),
                    args: instance.args,
                }))
            }
            EmulateItemResult::NeedsReturn => {
                trace!("{:?}", this.dump_place(&dest.clone().into()));
                this.return_to_block(ret)?;
                interp_ok(None)
            }
            EmulateItemResult::NeedsUnwind => {
                // Jump to the unwind block to begin unwinding.
                this.unwind_to_block(unwind)?;
                interp_ok(None)
            }
            EmulateItemResult::AlreadyJumped => interp_ok(None),
        }
    }

    /// Emulates a Miri-supported intrinsic (not supported by the core engine).
    /// Returns `Ok(true)` if the intrinsic was handled.
    fn emulate_intrinsic_by_name(
        &mut self,
        intrinsic_name: &str,
        generic_args: ty::GenericArgsRef<'tcx>,
        args: &[OpTy<'tcx>],
        dest: &MPlaceTy<'tcx>,
        ret: Option<mir::BasicBlock>,
    ) -> InterpResult<'tcx, EmulateItemResult> {
        let this = self.eval_context_mut();

        if let Some(name) = intrinsic_name.strip_prefix("atomic_") {
            return this.emulate_atomic_intrinsic(name, generic_args, args, dest);
        }
        if let Some(name) = intrinsic_name.strip_prefix("simd_") {
            return this.emulate_simd_intrinsic(name, args, dest);
        }

        match intrinsic_name {
            // Basic control flow
            "abort" => {
                throw_machine_stop!(TerminationInfo::Abort(
                    "the program aborted execution".to_owned()
                ));
            }
            "catch_unwind" => {
                let [try_fn, data, catch_fn] = check_intrinsic_arg_count(args)?;
                this.handle_catch_unwind(try_fn, data, catch_fn, dest, ret)?;
                // This pushed a stack frame, don't jump to `ret`.
                return interp_ok(EmulateItemResult::AlreadyJumped);
            }

            // Raw memory accesses
            "volatile_load" => {
                let [place] = check_intrinsic_arg_count(args)?;
                let place = this.deref_pointer(place)?;
                this.copy_op(&place, dest)?;
            }
            "volatile_store" => {
                let [place, dest] = check_intrinsic_arg_count(args)?;
                let place = this.deref_pointer(place)?;
                this.copy_op(dest, &place)?;
            }

            "volatile_set_memory" => {
                let [ptr, val_byte, count] = check_intrinsic_arg_count(args)?;
                this.write_bytes_intrinsic(ptr, val_byte, count, "volatile_set_memory")?;
            }

            // Memory model / provenance manipulation
            "ptr_mask" => {
                let [ptr, mask] = check_intrinsic_arg_count(args)?;

                let ptr = this.read_pointer(ptr)?;
                let mask = this.read_target_usize(mask)?;

                let masked_addr = Size::from_bytes(ptr.addr().bytes() & mask);

                this.write_pointer(Pointer::new(ptr.provenance, masked_addr), dest)?;
            }

            // We want to return either `true` or `false` at random, or else something like
            // ```
            // if !is_val_statically_known(0) { unreachable_unchecked(); }
            // ```
            // Would not be considered UB, or the other way around (`is_val_statically_known(0)`).
            "is_val_statically_known" => {
                let [_arg] = check_intrinsic_arg_count(args)?;
                // FIXME: should we check for validity here? It's tricky because we do not have a
                // place. Codegen does not seem to set any attributes like `noundef` for intrinsic
                // calls, so we don't *have* to do anything.
                let branch: bool = this.machine.rng.get_mut().random();
                this.write_scalar(Scalar::from_bool(branch), dest)?;
            }

            // Other
            "breakpoint" => {
                let [] = check_intrinsic_arg_count(args)?;
                // normally this would raise a SIGTRAP, which aborts if no debugger is connected
                throw_machine_stop!(TerminationInfo::Abort(format!("trace/breakpoint trap")))
            }

            "va_arg" => {
                let [va_list] = check_intrinsic_arg_count(args)?;
                intrinsic_va_arg(this, va_list, dest)?;
            }

            "assert_inhabited" | "assert_zero_valid" | "assert_mem_uninitialized_valid" => {
                // Make these a NOP, so we get the better Miri-native error messages.
            }

            _ => return this.emulate_math_intrinsic(intrinsic_name, generic_args, args, dest),
        }

        interp_ok(EmulateItemResult::NeedsReturn)
    }
}
