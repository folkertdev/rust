//@ assembly-output: emit-asm
//@ compile-flags: -Copt-level=3
//@ revisions: sysv64 sysv32 win64 win32
//@ add-minicore
//@[sysv64] needs-llvm-components: x86
//@[sysv64] compile-flags: --target x86_64-unknown-linux-gnu
//@[sysv32] needs-llvm-components: x86
//@[sysv32] compile-flags: --target i686-unknown-linux-gnu
//@[win64] needs-llvm-components: x86
//@[win64] compile-flags: --target x86_64-pc-windows-msvc
//@[win32] needs-llvm-components: x86
//@[win32] compile-flags: --target i686-pc-windows-msvc

// Checks the ABI of the x87 80-bit extended-precision float `f80`
//
// System V (both 32- and 64-bit): `f80` is passed on the stack (it uses the X87/X87UP register
// classes, so it consumes neither an integer nor an SSE register) and returned in `st0`.
//
// Windows x64: `f80` is 16 bytes, so per the Win64 ABI it is passed by reference and returned via
// a caller-provided pointer (`sret`), matching clang/gcc for `x86_fp80`.
//
// Windows x86: like System V 32-bit, passed on the stack and returned in `st0`.

#![feature(no_core)]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
#[cfg(target_arch = "x86")]
use minicore::arch::x86::f80;
#[cfg(target_arch = "x86_64")]
use minicore::arch::x86_64::f80;
use minicore::*;

// CHECK-LABEL: f80_id:
#[no_mangle]
extern "C" fn f80_id(x: f80) -> f80 {
    // sysv64: fldt 8(%rsp)
    // sysv64-NEXT: retq

    // sysv32: fldt 4(%esp)
    // sysv32-NEXT: retl

    // win64: movq %rcx, %rax
    // win64-NEXT: fldt (%rdx)
    // win64-NEXT: fstpt (%rcx)
    // win64-NEXT: retq

    // win32: fldt {{[0-9]+}}(%ebp)
    // win32: retl
    x
}

// CHECK-LABEL: f80_second:
#[no_mangle]
extern "C" fn f80_second(_a: f80, b: f80) -> f80 {
    // sysv64: fldt 24(%rsp)
    // sysv64-NEXT: retq

    // sysv32: fldt 16(%esp)
    // sysv32-NEXT: retl

    // win64: fldt (%r8)
    // win64-NEXT: fstpt (%rcx)
    // win64-NEXT: retq

    // win32: fldt {{[0-9]+}}(%ebp)
    // win32: retl
    b
}

/// `f80` is passed via the stack, and does not consume an SSE slot.
#[no_mangle]
extern "C" fn f80_after_f64(_a: f64, b: f80) -> f80 {
    // CHECK-LABEL: f80_after_f64:

    // sysv64: fldt 8(%rsp)
    // sysv64-NEXT: retq

    // sysv32: fldt 12(%esp)
    // sysv32-NEXT: retl

    // win64: fldt (%r8)
    // win64-NEXT: fstpt (%rcx)
    // win64-NEXT: retq

    // win32: fldt {{[0-9]+}}(%ebp)
    // win32: retl
    b
}

/// `f80` is passed via the stack, and does not consume an SSE slot.
#[no_mangle]
extern "C" fn f80_between_f64(_a: f64, b: f80, _c: f64) -> f80 {
    // CHECK-LABEL: f80_between_f64:

    // sysv64: fldt 8(%rsp)
    // sysv64-NEXT: retq

    // sysv32: fldt 12(%esp)
    // sysv32-NEXT: retl

    // win64: fldt (%r8)
    // win64-NEXT: fstpt (%rcx)
    // win64-NEXT: retq

    // win32: fldt {{[0-9]+}}(%ebp)
    // win32: retl
    b
}

/// `f80` is passed via the stack, and does not consume a GPR.
#[no_mangle]
extern "C" fn f80_after_int(_a: i32, b: f80) -> f80 {
    // CHECK-LABEL: f80_after_int:

    // sysv64: fldt 8(%rsp)
    // sysv64-NEXT: retq

    // sysv32: fldt 8(%esp)
    // sysv32-NEXT: retl

    // win64: fldt (%r8)
    // win64-NEXT: fstpt (%rcx)
    // win64-NEXT: retq

    // win32: fldt {{[0-9]+}}(%ebp)
    // win32: retl
    b
}

/// A `ScalarPair` containing an `f80` is never passed via registers.
#[no_mangle]
extern "C" fn f80_pair_arg(p: (f80, i32)) -> f80 {
    // CHECK-LABEL: f80_pair_arg:

    // sysv64: fldt 8(%rsp)
    // sysv64-NEXT: retq

    // sysv32: fldt 4(%esp)
    // sysv32-NEXT: retl

    // win64: movq %rcx, %rax
    // win64-NEXT: fldt (%rdx)
    // win64-NEXT: fstpt (%rcx)
    // win64-NEXT: retq

    // win32: fldt 4(%esp)
    // win32-NEXT: retl
    p.0
}

/// A `ScalarPair` containing an `f80` uses sret on every target.
#[no_mangle]
extern "C" fn f80_pair_ret(x: f80, y: i32) -> (f80, i32) {
    // CHECK-LABEL: f80_pair_ret:

    // sysv64: fstpt (%rdi)
    // sysv64-NEXT: movl %esi, 16(%rdi)
    // sysv64-NEXT: retq

    // sysv32: fstpt (%eax)
    // sysv32-NEXT: movl %ecx, 12(%eax)
    // sysv32-NEXT: retl $4

    // win64: fstpt (%rcx)
    // win64-NEXT: movl %r8d, 16(%rcx)
    // win64-NEXT: retq

    // win32: fstpt (%eax)
    // win32-NEXT: movl %ecx, 16(%eax)
    (x, y)
}
