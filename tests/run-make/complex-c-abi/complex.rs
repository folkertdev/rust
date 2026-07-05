//! Rust side of the `_Complex` ABI check: `#[repr(complex)]` `Complex<T>` in `extern "C"`
//! functions must be ABI-compatible with the clang C `_Complex` reference in `complex.c`.
//!
//! The `// <REV>:` lines record rustc's actual lowering. Where it differs textually from
//! `complex.c` the two are still ABI-compatible (identical registers / stack slots), verified
//! by assembly: rustc uses a single `{re, im}` aggregate where clang uses two scalar args, and
//! `[N x i8]` where clang types an indirect slot `{T, T}`. Only targets/types whose callconv
//! already reproduces the C ABI are listed; `rmake.rs` checks exactly those.

#![feature(no_core, lang_items, repr_complex, f16, f128)]
#![no_std]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
use minicore::num::Complex;

// X86_64:       define{{.*}} <2 x half> @cplx_f16(<2 x half>{{.*}}
// AARCH64:      define{{.*}} [2 x half] @cplx_f16([2 x half]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x half] @cplx_f16([2 x half]{{.*}}
// RISCV64:      define{{.*}} { half, half } @cplx_f16({ half, half }{{.*}}
// RISCV32:      define{{.*}} { half, half } @cplx_f16({ half, half }{{.*}}
// WINDOWS_MSVC: define{{.*}} i32 @cplx_f16(i32{{.*}}
// WINDOWS_GNU:  define{{.*}} i32 @cplx_f16(i32{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f16(x: Complex<f16>) -> Complex<f16> {
    x
}

// X86_64:       define{{.*}} <2 x float> @cplx_f32(<2 x float>{{.*}}
// AARCH64:      define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// ARM:          define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// RISCV64:      define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// RISCV32:      define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// SPARC64:      define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// WINDOWS_MSVC: define{{.*}} i64 @cplx_f32(i64{{.*}}
// WINDOWS_GNU:  define{{.*}} i64 @cplx_f32(i64{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f32(x: Complex<f32>) -> Complex<f32> {
    x
}

// X86_64:       define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// AARCH64:      define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// ARM:          define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// RISCV64:      define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// RISCV32:      define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// SPARC64:      define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// WINDOWS_MSVC: define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WINDOWS_GNU:  define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f64(x: Complex<f64>) -> Complex<f64> {
    x
}

// X86_64:       define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}byval([32 x i8]){{.*}}
// WINDOWS_GNU:  define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f128(x: Complex<f128>) -> Complex<f128> {
    x
}

// X86_64:       define{{.*}} i16 @cplx_i8(i16{{.*}}
// AARCH64:      define{{.*}} i64 @cplx_i8(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i8(i64{{.*}}
// ARM:          define{{.*}} i32 @cplx_i8(i32{{.*}}
// RISCV64:      define{{.*}} i64 @cplx_i8(i64{{.*}}
// RISCV32:      define{{.*}} i32 @cplx_i8(i32{{.*}}
// SPARC64:      define{{.*}} i64 @cplx_i8(i64{{.*}}
// WINDOWS_MSVC: define{{.*}} i16 @cplx_i8(i16{{.*}}
// WINDOWS_GNU:  define{{.*}} i16 @cplx_i8(i16{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i8(x: Complex<i8>) -> Complex<i8> {
    x
}

// X86_64:       define{{.*}} i32 @cplx_i16(i32{{.*}}
// AARCH64:      define{{.*}} i64 @cplx_i16(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i16(i64{{.*}}
// ARM:          define{{.*}} i32 @cplx_i16(i32{{.*}}
// RISCV64:      define{{.*}} i64 @cplx_i16(i64{{.*}}
// RISCV32:      define{{.*}} i32 @cplx_i16(i32{{.*}}
// SPARC64:      define{{.*}} i64 @cplx_i16(i64{{.*}}
// WINDOWS_MSVC: define{{.*}} i32 @cplx_i16(i32{{.*}}
// WINDOWS_GNU:  define{{.*}} i32 @cplx_i16(i32{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i16(x: Complex<i16>) -> Complex<i16> {
    x
}

// X86_64:       define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64:      define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i32(i64{{.*}}
// ARM:          define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}[2 x i32]{{.*}}
// RISCV64:      define{{.*}} i64 @cplx_i32(i64{{.*}}
// RISCV32:      define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// SPARC64:      define{{.*}} i64 @cplx_i32(i64{{.*}}
// WINDOWS_MSVC: define{{.*}} i64 @cplx_i32(i64{{.*}}
// WINDOWS_GNU:  define{{.*}} i64 @cplx_i32(i64{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i32(x: Complex<i32>) -> Complex<i32> {
    x
}

// X86_64:       define{{.*}} { i64, i64 } @cplx_i64({ i64, i64 }{{.*}}
// AARCH64:      define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// ARM:          define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}[2 x i64]{{.*}}
// RISCV64:      define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// RISCV32:      define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// SPARC64:      define{{.*}} { i64, i64 } @cplx_i64({ i64, i64 }{{.*}}
// WINDOWS_MSVC: define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WINDOWS_GNU:  define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i64(x: Complex<i64>) -> Complex<i64> {
    x
}
