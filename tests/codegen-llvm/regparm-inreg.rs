// Checks how `regparm` flag works with different calling conventions:
// marks function arguments as "inreg" like the C/C++ compilers for the platforms.
// x86 only.

//@ add-minicore
//@ compile-flags: --target i686-unknown-linux-gnu -Cno-prepopulate-passes -Copt-level=3 -Ctarget-feature=+avx
//@ needs-llvm-components: x86

//@ revisions:regparm0 regparm1 regparm2 regparm3
//@[regparm0] compile-flags: -Zregparm=0
//@[regparm1] compile-flags: -Zregparm=1
//@[regparm2] compile-flags: -Zregparm=2
//@[regparm3] compile-flags: -Zregparm=3

// ignore-tidy-file-linelength

#![crate_type = "lib"]
#![no_core]
#![feature(no_core, lang_items, f16, f128)]

extern crate minicore;
use minicore::num::Complex;
use minicore::simd::Simd;

#[repr(C)]
struct WrapCStruct<T>(T);

// regparm doesn't work for "fastcall" calling conv (only 2 inregs)
// CHECK: @extern_fastcall(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
#[no_mangle]
pub extern "fastcall" fn extern_fastcall(_: i32, _: i32, _: i32) {}

// regparm0: @extern_c(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
// regparm1: @extern_c(i32 inreg noundef %_1, i32 noundef %_2, i32 noundef %_3)
// regparm2: @extern_c(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
// regparm3: @extern_c(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3)
#[no_mangle]
pub extern "C" fn extern_c(_: i32, _: i32, _: i32) {}

// regparm0: @extern_cdecl(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
// regparm1: @extern_cdecl(i32 inreg noundef %_1, i32 noundef %_2, i32 noundef %_3)
// regparm2: @extern_cdecl(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
// regparm3: @extern_cdecl(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3)
#[no_mangle]
pub extern "cdecl" fn extern_cdecl(_: i32, _: i32, _: i32) {}

// regparm0: @extern_stdcall(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
// regparm1: @extern_stdcall(i32 inreg noundef %_1, i32 noundef %_2, i32 noundef %_3)
// regparm2: @extern_stdcall(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
// regparm3: @extern_stdcall(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3)
#[no_mangle]
pub extern "stdcall" fn extern_stdcall(_: i32, _: i32, _: i32) {}

// regparm doesn't work for thiscall
// CHECK: @extern_thiscall(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
#[no_mangle]
pub extern "thiscall" fn extern_thiscall(_: i32, _: i32, _: i32) {}

// regparm0: @pass_wrapped_i32(i32 noundef %_1, i32 noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_i32(i32 inreg noundef %_1, i32 noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_i32(_: i32, _: i32, _: WrapCStruct<i32>, _: i32) {}

#[repr(C)]
struct ReprCTuple<T, U> {
    x1: T,
    x2: U,
}

// regparm0: @pass_tuple_i32_i32(i32 noundef %_1, i32 noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
// regparm1: @pass_tuple_i32_i32(i32 inreg noundef %_1, i32 noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
// regparm2: @pass_tuple_i32_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
// regparm3: @pass_tuple_i32_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_tuple_i32_i32(_: i32, _: i32, _: ReprCTuple<i32, i32>, _: i32) {}

// regparm0: @pass_bool_i16_i64_u128(i1 noundef zeroext %_1, i16 noundef signext %_2, i64 noundef %_3, i128 noundef %_4)
// regparm1: @pass_bool_i16_i64_u128(i1 inreg noundef zeroext %_1, i16 noundef signext %_2, i64 noundef %_3, i128 noundef %_4)
// regparm2: @pass_bool_i16_i64_u128(i1 inreg noundef zeroext %_1, i16 inreg noundef signext %_2, i64 noundef %_3, i128 noundef %_4)
// regparm3: @pass_bool_i16_i64_u128(i1 inreg noundef zeroext %_1, i16 inreg noundef signext %_2, i64 noundef %_3, i128 noundef %_4)
#[no_mangle]
pub extern "C" fn pass_bool_i16_i64_u128(_: bool, _: i16, _: i64, _: u128) {}

// regparm0: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 noundef zeroext %_3, i16 noundef signext %_4)
// regparm1: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3, i16 noundef signext %_4)
// regparm2: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3, i16 inreg noundef signext %_4)
// regparm3: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3, i16 inreg noundef signext %_4)
#[no_mangle]
pub extern "C" fn pass_f32_f64_bool_i16(_: f32, _: f64, _: bool, _: i16) {}

// regparm0: @pass_f32x4(i32 noundef %_1, <4 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f32x4(i32 inreg noundef %_1, <4 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f32x4(i32 inreg noundef %_1, <4 x float> %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_f32x4(i32 inreg noundef %_1, <4 x float> %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f32x4(_: i32, _: Simd<f32, 4>, _: i32, _: i32) {}

// regparm0: @pass_f32x8(i32 noundef %_1, <8 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f32x8(i32 inreg noundef %_1, <8 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f32x8(i32 inreg noundef %_1, <8 x float> %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_f32x8(i32 inreg noundef %_1, <8 x float> %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f32x8(_: i32, _: Simd<f32, 8>, _: i32, _: i32) {}

// regparm0: @pass_f16(half noundef %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f16(half noundef %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f16(half noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_f16(half noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f16(_: f16, _: i32, _: i32, _: i32) {}

// FIXME(f16b): f16b is guarded by target_has_reliable_f16b.
// FIXME regparm0 @pass_bf16(bfloat noundef %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// FIXME regparm1 @pass_bf16(bfloat noundef %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// FIXME regparm2 @pass_bf16(bfloat noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// FIXME regparm3 @pass_bf16(bfloat noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
// #[no_mangle]
// pub extern "C" fn pass_bf16(_: f16b, _: i32, _: i32, _: i32) {}

// FIXME(x87_f80): enable once we have x87_f80 / c_longdouble.
// FIXME regparm0 @pass_f80(x86_fp80 noundef %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// FIXME regparm1 @pass_f80(x86_fp80 noundef %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// FIXME regparm2 @pass_f80(x86_fp80 noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// FIXME regparm3 @pass_f80(x86_fp80 noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
// #[no_mangle]
// pub extern "C" fn pass_f80(_: core::ffi::c_longdouble, _: i32, _: i32, _: i32) {}

// regparm0: @pass_f128(fp128 noundef %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f128(fp128 noundef %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f128(fp128 noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_f128(fp128 noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f128(_: f128, _: i32, _: i32, _: i32) {}

// regparm0: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_f16(_: WrapCStruct<f16>, _: i32, _: i32, _: i32) {}

// regparm0: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_f128(_: WrapCStruct<f128>, _: i32, _: i32, _: i32) {}

// regparm0: @pass_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_complex_f32(_: Complex<f32>, _: i32, _: i32, _: i32) {}

// regparm0: @pass_complex_i32(i32 noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_complex_i32(i32 inreg noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_complex_i32(i32 inreg noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_complex_i32(i32 inreg noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_complex_i32(_: i32, _: Complex<i32>, _: i32, _: i32) {}

// regparm0: @pass_wrapped_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_complex_f32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_complex_f32(_: WrapCStruct<Complex<f32>>, _: i32, _: i32, _: i32) {}

// regparm0: @pass_wrapped_complex_i32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_complex_i32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_complex_i32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_complex_i32(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_complex_i32(
    _: WrapCStruct<[Complex<i32>; 1]>,
    _: i32,
    _: i32,
    _: i32,
) {
}

// regparm0: @pass_struct_singleton_array_f32(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_struct_singleton_array_f32(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_struct_singleton_array_f32(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_struct_singleton_array_f32(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_struct_singleton_array_f32(
    _: WrapCStruct<[f32; 1]>,
    _: i32,
    _: i32,
    _: i32,
) {
}
