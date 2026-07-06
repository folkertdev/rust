use arrayvec::ArrayVec;
use rustc_abi::{BackendRepr, Float, HasDataLayout, Primitive, Reg, RegKind, Size, TyAbiInterface};

use super::CastTarget;
use crate::callconv::{ArgAbi, FnAbi, Uniform};

/// The register holding one component of a `_Complex` (its real or imaginary part).
fn complex_component_reg<'a, Ty, C>(cx: &C, arg: &ArgAbi<'a, Ty>) -> Option<Reg>
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    let field = arg.layout.field(cx, 0);
    match field.backend_repr {
        BackendRepr::Scalar(s) => match s.primitive() {
            Primitive::Float(Float::F16) => Some(Reg { kind: RegKind::Float, size: field.size }),
            Primitive::Float(Float::F32) => Some(Reg::f32()),
            Primitive::Float(Float::F64) => Some(Reg::f64()),
            Primitive::Float(Float::F128) => Some(Reg::f128()),
            Primitive::Int(..) => Some(Reg { kind: RegKind::Integer, size: field.size }),
            Primitive::Pointer(_) => None,
        },
        _ => None,
    }
}

fn classify_ret<'a, Ty, C>(cx: &C, ret: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if ret.layout.is_complex()
        && let Some(reg) = complex_component_reg(cx, ret)
    {
        // clang returns a `_Complex` in registers, one per component.
        ret.cast_to(CastTarget::pair(reg, reg));
    } else if ret.layout.is_aggregate() && ret.layout.is_sized() {
        classify_aggregate(ret)
    } else if ret.layout.size.bits() < 32 && ret.layout.is_sized() {
        ret.extend_integer_width_to(32);
    }
}

fn classify_arg<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if arg.layout.pass_indirectly_in_non_rustic_abis(cx) {
        arg.make_indirect();
        return;
    }
    if arg.layout.is_complex() {
        // clang passes a `_Complex` argument `byval`.
        arg.pass_by_stack_offset(Some(arg.layout.align.abi));
    } else if arg.layout.is_aggregate() && arg.layout.is_sized() {
        classify_aggregate(arg)
    } else if arg.layout.size.bits() < 32 && arg.layout.is_sized() {
        arg.extend_integer_width_to(32);
    }
}

/// the pass mode used for aggregates in arg and ret position
fn classify_aggregate<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    let align_bytes = arg.layout.align.bytes();
    let size = arg.layout.size;

    let reg = match align_bytes {
        1 => Reg::i8(),
        2 => Reg::i16(),
        4 => Reg::i32(),
        8 => Reg::i64(),
        16 => Reg::i128(),
        _ => unreachable!("Align is given as power of 2 no larger than 16 bytes"),
    };

    if align_bytes == size.bytes() {
        let mut prefix = ArrayVec::new();
        prefix.push(reg);
        arg.cast_to(CastTarget::prefixed(prefix, Uniform::new(Reg::i8(), Size::ZERO)));
    } else {
        arg.cast_to(Uniform::new(reg, size));
    }
}

fn classify_arg_kernel<'a, Ty, C>(_cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    match arg.mode {
        super::PassMode::Ignore | super::PassMode::Direct(_) => return,
        super::PassMode::Pair(_, _) => {}
        super::PassMode::Cast { .. } => unreachable!(),
        super::PassMode::Indirect { .. } => {}
    }

    // FIXME only allow structs and wide pointers here
    // panic!(
    //     "`extern \"ptx-kernel\"` doesn't allow passing types other than primitives and structs"
    // );

    let align_bytes = arg.layout.align.bytes();

    let unit = match align_bytes {
        1 => Reg::i8(),
        2 => Reg::i16(),
        4 => Reg::i32(),
        8 => Reg::i64(),
        16 => Reg::i128(),
        _ => unreachable!("Align is given as power of 2 no larger than 16 bytes"),
    };
    if arg.layout.size.bytes() / align_bytes == 1 {
        // Make sure we pass the struct as array at the LLVM IR level and not as a single integer.
        let mut prefix = ArrayVec::new();
        prefix.push(unit);
        arg.cast_to(CastTarget::prefixed(prefix, Uniform::new(unit, Size::ZERO)));
    } else {
        arg.cast_to(Uniform::new(unit, arg.layout.size));
    }
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret(cx, &mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(cx, arg);
    }
}

pub(crate) fn compute_ptx_kernel_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if !fn_abi.ret.layout.is_unit() && !fn_abi.ret.layout.is_never() {
        panic!("Kernels should not return anything other than () or !");
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_kernel(cx, arg);
    }
}
