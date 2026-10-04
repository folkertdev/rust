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
use minicore::*;

pub mod tests {
    use minicore::simd::Simd;

    // regparm doesn't work for "fastcall" calling conv (only 2 inregs)
    // CHECK: @f1(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
    #[no_mangle]
    pub extern "fastcall" fn f1(_: i32, _: i32, _: i32) {}

    // regparm0: @f3(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
    // regparm1: @f3(i32 inreg noundef %_1, i32 noundef %_2, i32 noundef %_3)
    // regparm2: @f3(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
    // regparm3: @f3(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3)
    #[no_mangle]
    pub extern "C" fn f3(_: i32, _: i32, _: i32) {}

    // regparm0: @f4(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
    // regparm1: @f4(i32 inreg noundef %_1, i32 noundef %_2, i32 noundef %_3)
    // regparm2: @f4(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
    // regparm3: @f4(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3)
    #[no_mangle]
    pub extern "cdecl" fn f4(_: i32, _: i32, _: i32) {}

    // regparm0: @f5(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
    // regparm1: @f5(i32 inreg noundef %_1, i32 noundef %_2, i32 noundef %_3)
    // regparm2: @f5(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 noundef %_3)
    // regparm3: @f5(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3)
    #[no_mangle]
    pub extern "stdcall" fn f5(_: i32, _: i32, _: i32) {}

    // regparm doesn't work for thiscall
    // CHECK: @f6(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3)
    #[no_mangle]
    pub extern "thiscall" fn f6(_: i32, _: i32, _: i32) {}

    #[repr(C)]
    struct S1 {
        x1: i32,
    }
    // regparm0: @f7(i32 noundef %_1, i32 noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
    // regparm1: @f7(i32 inreg noundef %_1, i32 noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
    // regparm2: @f7(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
    // regparm3: @f7(i32 inreg noundef %_1, i32 inreg noundef %_2, i32 inreg %0, i32 noundef %_4)
    #[no_mangle]
    pub extern "C" fn f7(_: i32, _: i32, _: S1, _: i32) {}

    #[repr(C)]
    struct S2 {
        x1: i32,
        x2: i32,
    }
    // regparm0: @f8(i32 noundef %_1, i32 noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
    // regparm1: @f8(i32 inreg noundef %_1, i32 noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
    // regparm2: @f8(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
    // regparm3: @f8(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} %_3, i32 noundef %_4)
    #[no_mangle]
    pub extern "C" fn f8(_: i32, _: i32, _: S2, _: i32) {}

    // regparm0: @f9(i1 noundef zeroext %_1, i16 noundef signext %_2, i64 noundef %_3,
    // regparm0-SAME: i128 noundef %_4)
    // regparm1: @f9(i1 inreg noundef zeroext %_1, i16 noundef signext %_2, i64 noundef %_3,
    // regparm1-SAME: i128 noundef %_4)
    // regparm2: @f9(i1 inreg noundef zeroext %_1, i16 inreg noundef signext %_2, i64 noundef %_3,
    // regparm2-SAME: i128 noundef %_4)
    // regparm3: @f9(i1 inreg noundef zeroext %_1, i16 inreg noundef signext %_2, i64 noundef %_3,
    // regparm3-SAME: i128 noundef %_4)
    #[no_mangle]
    pub extern "C" fn f9(_: bool, _: i16, _: i64, _: u128) {}

    // regparm0: @f10(float noundef %_1, double noundef %_2, i1 noundef zeroext %_3,
    // regparm0-SAME: i16 noundef signext %_4)
    // regparm1: @f10(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3,
    // regparm1-SAME: i16 noundef signext %_4)
    // regparm2: @f10(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3,
    // regparm2-SAME: i16 inreg noundef signext %_4)
    // regparm3: @f10(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3,
    // regparm3-SAME: i16 inreg noundef signext %_4)
    #[no_mangle]
    pub extern "C" fn f10(_: f32, _: f64, _: bool, _: i16) {}

    #[allow(non_camel_case_types)]
    type __m128 = Simd<f32, 4>;

    // regparm0: @f11(i32 noundef %_1, <4 x float> %_2, i32 noundef %_3, i32 noundef %_4)
    // regparm1: @f11(i32 inreg noundef %_1, <4 x float> %_2, i32 noundef %_3, i32 noundef %_4)
    // regparm2: @f11(i32 inreg noundef %_1, <4 x float> %_2, i32 inreg noundef %_3,
    // regparm2-SAME: i32 noundef %_4)
    // regparm3: @f11(i32 inreg noundef %_1, <4 x float> %_2, i32 inreg noundef %_3,
    // regparm3-SAME: i32 inreg noundef %_4)
    #[no_mangle]
    pub extern "C" fn f11(_: i32, _: __m128, _: i32, _: i32) {}

    #[allow(non_camel_case_types)]
    type __m256 = Simd<f32, 8>;

    // regparm0: @f12(i32 noundef %_1, <8 x float> %_2, i32 noundef %_3, i32 noundef %_4)
    // regparm1: @f12(i32 inreg noundef %_1, <8 x float> %_2, i32 noundef %_3, i32 noundef %_4)
    // regparm2: @f12(i32 inreg noundef %_1, <8 x float> %_2, i32 inreg noundef %_3,
    // regparm2-SAME: i32 noundef %_4)
    // regparm3: @f12(i32 inreg noundef %_1, <8 x float> %_2, i32 inreg noundef %_3,
    // regparm3-SAME: i32 inreg noundef %_4)
    #[no_mangle]
    pub extern "C" fn f12(_: i32, _: __m256, _: i32, _: i32) {}
}

// regparm0: @pass_f16(half noundef %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f16(half noundef %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f16(half noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_f16(half noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f16(_: f16, _: i32, _: i32, _: i32) {}

// FIXME: f16b is guarded by target_has_reliable_f16b.
// FIXME regparm0 @pass_bf16(bfloat noundef %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// FIXME regparm1 @pass_bf16(bfloat noundef %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// FIXME regparm2 @pass_bf16(bfloat noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// FIXME regparm3 @pass_bf16(bfloat noundef %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
// #[no_mangle]
// pub extern "C" fn pass_bf16(_: f16b, _: i32, _: i32, _: i32) {}

// FIXME: enable once we have x87_f80 / c_longdouble.
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

#[repr(C)]
pub struct S14 {
    pub x1: f16,
}

// regparm0: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_f16(ptr {{.*}} byval([2 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_f16(_: S14, _: i32, _: i32, _: i32) {}

#[repr(C)]
pub struct S15 {
    pub x1: f128,
}

// regparm0: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_f128(ptr {{.*}} byval([16 x i8]) {{.*}} %0, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_f128(_: S15, _: i32, _: i32, _: i32) {}

// regparm0: @pass_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_complex_float(_: Complex<f32>, _: i32, _: i32, _: i32) {}

// regparm0: @pass_complex_int(i32 noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_complex_int(i32 inreg noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_complex_int(i32 inreg noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_complex_int(i32 inreg noundef %_1, ptr {{.*}} byval([8 x i8]) {{.*}} %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_complex_int(_: i32, _: Complex<i32>, _: i32, _: i32) {}

#[repr(C)]
struct S16 {
    x1: Complex<f32>,
}

// regparm0: @pass_wrapped_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_complex_float(_: S16, _: i32, _: i32, _: i32) {}

#[repr(C)]
struct S17 {
    x1: [Complex<i32>; 1],
}

// regparm0: @pass_wrapped_complex_int(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_wrapped_complex_int(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_complex_int(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_complex_int(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_wrapped_complex_int(_: S17, _: i32, _: i32, _: i32) {}

#[repr(C)]
pub struct S18 {
    pub x1: [f32; 1],
}

// regparm0: @pass_struct_singleton_array_float(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_struct_singleton_array_float(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_struct_singleton_array_float(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
// regparm3: @pass_struct_singleton_array_float(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_struct_singleton_array_float(_: S18, _: i32, _: i32, _: i32) {}

#[repr(C)]
pub union U1 {
    pub x1: f32,
}

// Unions are passed like integers.
//
// regparm0: @pass_union_float(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_union_float(i32 inreg %0, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_union_float(i32 inreg %0, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm3: @pass_union_float(i32 inreg %0, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
#[no_mangle]
pub extern "C" fn pass_union_float(_: U1, _: i32, _: i32, _: i32) {}

#[repr(C)]
struct S5 {
    x1: f32,
}
impl Copy for S5 {}

#[repr(C)]
pub union U2 {
    pub x1: S5,
}

// regparm0: @pass_union_struct_float(ptr {{.*}} byval([4 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_union_struct_float(i32 inreg %0, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_union_struct_float(i32 inreg %0, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm3: @pass_union_struct_float(i32 inreg %0, i32 inreg noundef %_2, i32 inreg noundef %_3, i32 noundef %_4)
#[no_mangle]
pub extern "C" fn pass_union_struct_float(_: U2, _: i32, _: i32, _: i32) {}

#[repr(C)]
union U3 {
    x1: Complex<f32>,
}

// regparm0: @pass_union_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_union_complex_float(ptr {{.*}} byval([8 x i8]) {{.*}} %_1, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_union_complex_float([2 x i32] inreg %0, i32 noundef %_2, i32 noundef %_3, i32 noundef %_4)
// regparm3: @pass_union_complex_float([2 x i32] inreg %0, i32 inreg noundef %_2, i32 noundef %_3, i32 noundef %_4)
#[no_mangle]
pub extern "C" fn pass_union_complex_float(_: U3, _: i32, _: i32, _: i32) {}

#[repr(C)]
struct BigStruct {
    x: [u8; 32],
}

// regparm0: @force_hidden_sret(ptr{{.*}} sret([32 x i8]){{.*}} %_0, i32 noundef %a, i32 noundef %b, i32 noundef %c)
// regparm1: @force_hidden_sret(ptr{{.*}} inreg{{.*}} sret([32 x i8]){{.*}} %_0, i32 noundef %a, i32 noundef %b, i32 noundef %c)
// regparm2: @force_hidden_sret(ptr{{.*}} inreg{{.*}} sret([32 x i8]){{.*}} %_0, i32 inreg noundef %a, i32 noundef %b, i32 noundef %c)
// regparm3: @force_hidden_sret(ptr{{.*}} inreg{{.*}} sret([32 x i8]){{.*}} %_0, i32 inreg noundef %a, i32 inreg noundef %b, i32 noundef %c)
#[unsafe(no_mangle)]
pub extern "C" fn force_hidden_sret(a: i32, b: i32, c: i32) -> BigStruct {
    loop {}
}
