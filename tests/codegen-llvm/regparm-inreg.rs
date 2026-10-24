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
#![feature(no_core, lang_items)]

extern crate minicore;

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
// regparm1: @pass_wrapped_i32(i32 inreg noundef %_1, i32 noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}}  %_3, i32 noundef %_4)
// regparm2: @pass_wrapped_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3, i32 noundef %_4)
// regparm3: @pass_wrapped_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} byval([4 x i8]) {{.*}} %_3,
// regparm3-SAME: i32 inreg noundef %_4)
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
// regparm3: @pass_tuple_i32_i32(i32 inreg noundef %_1, i32 inreg noundef %_2, ptr {{.*}} %_3,
// regparm3-SAME: i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_tuple_i32_i32(_: i32, _: i32, _: ReprCTuple<i32, i32>, _: i32) {}

// regparm0: @pass_bool_i16_i64_u128(i1 noundef zeroext %_1, i16 noundef signext %_2, i64 noundef %_3,
// regparm0-SAME: i128 noundef %_4)
// regparm1: @pass_bool_i16_i64_u128(i1 inreg noundef zeroext %_1, i16 noundef signext %_2, i64 noundef %_3,
// regparm1-SAME: i128 noundef %_4)
// regparm2: @pass_bool_i16_i64_u128(i1 inreg noundef zeroext %_1, i16 inreg noundef signext %_2, i64 noundef %_3,
// regparm2-SAME: i128 noundef %_4)
// regparm3: @pass_bool_i16_i64_u128(i1 inreg noundef zeroext %_1, i16 inreg noundef signext %_2, i64 noundef %_3,
// regparm3-SAME: i128 noundef %_4)
#[no_mangle]
pub extern "C" fn pass_bool_i16_i64_u128(_: bool, _: i16, _: i64, _: u128) {}

// regparm0: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 noundef zeroext %_3,
// regparm0-SAME: i16 noundef signext %_4)
// regparm1: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3,
// regparm1-SAME: i16 noundef signext %_4)
// regparm2: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3,
// regparm2-SAME: i16 inreg noundef signext %_4)
// regparm3: @pass_f32_f64_bool_i16(float noundef %_1, double noundef %_2, i1 inreg noundef zeroext %_3,
// regparm3-SAME: i16 inreg noundef signext %_4)
#[no_mangle]
pub extern "C" fn pass_f32_f64_bool_i16(_: f32, _: f64, _: bool, _: i16) {}

// regparm0: @pass_f32x4(i32 noundef %_1, <4 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f32x4(i32 inreg noundef %_1, <4 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f32x4(i32 inreg noundef %_1, <4 x float> %_2, i32 inreg noundef %_3,
// regparm2-SAME: i32 noundef %_4)
// regparm3: @pass_f32x4(i32 inreg noundef %_1, <4 x float> %_2, i32 inreg noundef %_3,
// regparm3-SAME: i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f32x4(_: i32, _: Simd<f32, 4>, _: i32, _: i32) {}

// regparm0: @pass_f32x8(i32 noundef %_1, <8 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm1: @pass_f32x8(i32 inreg noundef %_1, <8 x float> %_2, i32 noundef %_3, i32 noundef %_4)
// regparm2: @pass_f32x8(i32 inreg noundef %_1, <8 x float> %_2, i32 inreg noundef %_3,
// regparm2-SAME: i32 noundef %_4)
// regparm3: @pass_f32x8(i32 inreg noundef %_1, <8 x float> %_2, i32 inreg noundef %_3,
// regparm3-SAME: i32 inreg noundef %_4)
#[no_mangle]
pub extern "C" fn pass_f32x8(_: i32, _: Simd<f32, 8>, _: i32, _: i32) {}
