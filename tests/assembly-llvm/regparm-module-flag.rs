// Test the regparm ABI with builtin and non-builtin calls
// Issue: https://github.com/rust-lang/rust/issues/145271
//@ add-minicore
//@ assembly-output: emit-asm
//@ compile-flags: -O --target=i686-unknown-linux-gnu -Crelocation-model=static
//@ revisions: REGPARM0 REGPARM1 REGPARM2 REGPARM3
//@[REGPARM0] compile-flags: -Zregparm=0
//@[REGPARM1] compile-flags: -Zregparm=1
//@[REGPARM2] compile-flags: -Zregparm=2
//@[REGPARM3] compile-flags: -Zregparm=3
//@ needs-llvm-components: x86
#![feature(no_core)]
#![no_std]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
use minicore::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn entrypoint(len: usize, ptr: *mut c_void, val: i32) -> *mut c_void {
    unsafe extern "C" {
        fn memset(p: *mut c_void, val: i32, len: usize) -> *mut c_void;
    }

    // REGPARM1-LABEL: entrypoint
    // REGPARM1: movl %e{{.*}}, %ecx
    // REGPARM1: pushl
    // REGPARM1: pushl
    // REGPARM1: calll memset

    // REGPARM2-LABEL: entrypoint
    // REGPARM2: movl 16(%esp), %edx
    // REGPARM2: movl %e{{.*}}, (%esp)
    // REGPARM2: movl %e{{.*}}, %eax
    // REGPARM2: calll memset

    // REGPARM3-LABEL: entrypoint
    // REGPARM3: movl %e{{.*}}, %esi
    // REGPARM3: movl %e{{.*}}, %eax
    // REGPARM3: movl %e{{.*}}, %ecx
    // REGPARM3: jmp memset
    unsafe { memset(ptr, val, len) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn non_builtin_entrypoint(
    len: usize,
    ptr: *mut c_void,
    val: i32,
) -> *mut c_void {
    unsafe extern "C" {
        fn non_builtin_memset(p: *mut c_void, val: i32, len: usize) -> *mut c_void;
    }

    // REGPARM1-LABEL: non_builtin_entrypoint
    // REGPARM1: movl %e{{.*}}, %ecx
    // REGPARM1: pushl
    // REGPARM1: pushl
    // REGPARM1: calll non_builtin_memset

    // REGPARM2-LABEL: non_builtin_entrypoint
    // REGPARM2: movl 16(%esp), %edx
    // REGPARM2: movl %e{{.*}}, (%esp)
    // REGPARM2: movl %e{{.*}}, %eax
    // REGPARM2: calll non_builtin_memset

    // REGPARM3-LABEL: non_builtin_entrypoint
    // REGPARM3: movl %e{{.*}}, %esi
    // REGPARM3: movl %e{{.*}}, %eax
    // REGPARM3: movl %e{{.*}}, %ecx
    // REGPARM3: jmp non_builtin_memset
    unsafe { non_builtin_memset(ptr, val, len) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_i32() -> i32 {
    extern "C" {
        fn test_i32_sink(_: i32, _: i32, _: i32) -> i32;
    }

    // REGPARM0-LABEL: test_i32
    // REGPARM0: subl    $16, %esp
    // REGPARM0: pushl   $3
    // REGPARM0: pushl   $2
    // REGPARM0: pushl   $1
    // REGPARM0: calll   test_i32_sink
    // REGPARM0: addl    $28, %esp
    // REGPARM0: retl

    // REGPARM1-LABEL: test_i32
    // REGPARM1: subl    $20, %esp
    // REGPARM1: movl    $1, %eax
    // REGPARM1: pushl   $3
    // REGPARM1: pushl   $2
    // REGPARM1: calll   test_i32_sink
    // REGPARM1: addl    $28, %esp
    // REGPARM1: retl

    // REGPARM2-LABEL: test_i32
    // REGPARM2: subl    $12, %esp
    // REGPARM2: movl    $1, %eax
    // REGPARM2: movl    $2, %edx
    // REGPARM2: movl    $3, (%esp)
    // REGPARM2: calll   test_i32_sink
    // REGPARM2: addl    $12, %esp
    // REGPARM2: retl

    // REGPARM3-LABEL: test_i32
    // REGPARM3: movl    $1, %eax
    // REGPARM3: movl    $2, %edx
    // REGPARM3: movl    $3, %ecx
    // REGPARM3: jmp     test_i32_sink
    unsafe { test_i32_sink(1, 2, 3) }
}

// A c-variadic function does not use registers for argument passing,
// even when not actually passing any c-variadic arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_i32_variadic() -> i32 {
    extern "C" {
        fn test_i32_variadic_sink(_: i32, _: i32, _: i32, ...) -> i32;
    }

    // CHECK-LABEL: test_i32_variadic
    // CHECK: subl    $16, %esp
    // CHECK: pushl   $3
    // CHECK: pushl   $2
    // CHECK: pushl   $1
    // CHECK: calll   test_i32_variadic_sink
    // CHECK: addl    $28, %esp
    // CHECK: retl
    unsafe { test_i32_variadic_sink(1, 2, 3) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_i64() -> i64 {
    extern "C" {
        fn test_i64_sink(_: i64) -> i64;
    }

    // REGPARM1-LABEL: test_i64
    // REGPARM1: pushl
    // REGPARM1: pushl
    // REGPARM1: calll test_i64_sink

    // REGPARM2-LABEL: test_i64
    // REGPARM2: movl $42, %eax
    // REGPARM2: xorl %edx, %edx
    // REGPARM2: jmp test_i64_sink

    // REGPARM3-LABEL: test_i64
    // REGPARM3: movl $42, %eax
    // REGPARM3: xorl %edx, %edx
    // REGPARM3: jmp test_i64_sink
    unsafe { test_i64_sink(42i64) }
}

#[repr(C)]
struct ThreeRegStruct {
    a: i64,
    b: i32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_struct() {
    extern "C" {
        fn test_struct_sink(_: ThreeRegStruct);
    }

    // REGPARM1-LABEL: test_struct
    // REGPARM1: movl $0, {{.*}}(%esp)
    // REGPARM1: movl $42, {{.*}}(%esp)
    // REGPARM1: movl $1, {{.*}}(%esp)
    // REGPARM1: calll test_struct_sink

    // REGPARM2-LABEL: test_struct
    // REGPARM2: movl $0, {{.*}}(%esp)
    // REGPARM2: movl $42, {{.*}}(%esp)
    // REGPARM2: movl $1, {{.*}}(%esp)
    // REGPARM2: calll test_struct_sink

    // REGPARM3-LABEL: test_struct
    // REGPARM3: movl $42, %eax
    // REGPARM3: xorl %edx, %edx
    // REGPARM3: movl $1, %ecx
    // REGPARM3: jmp test_struct_sink
    unsafe { test_struct_sink(ThreeRegStruct { a: 42, b: 1 }) }
}

#[repr(C)]
struct BigStruct {
    x: [u8; 32],
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_hidden_sret() {
    extern "C" {
        fn force_hidden_sret_sink(a: i32, b: i32, c: i32) -> BigStruct;
    }

    // REGPARM0-LABEL: force_hidden_sret:
    // REGPARM0: leal 8(%esp), %esi
    // REGPARM0: pushl $3
    // REGPARM0: pushl $2
    // REGPARM0: pushl $1
    // REGPARM0: pushl %esi
    // REGPARM0: calll force_hidden_sret_sink

    // REGPARM1-LABEL: force_hidden_sret:
    // REGPARM1: leal 12(%esp), %esi
    // REGPARM1: movl %esi, %eax
    // REGPARM1: pushl $3
    // REGPARM1: pushl $2
    // REGPARM1: pushl $1
    // REGPARM1: calll force_hidden_sret_sink

    // REGPARM2-LABEL: force_hidden_sret:
    // REGPARM2: leal 16(%esp), %esi
    // REGPARM2: movl $1, %edx
    // REGPARM2: movl %esi, %eax
    // REGPARM2: pushl $3
    // REGPARM2: pushl $2
    // REGPARM2: calll force_hidden_sret_sink

    // REGPARM3-LABEL: force_hidden_sret:
    // REGPARM3: leal 8(%esp), %esi
    // REGPARM3: movl $1, %edx
    // REGPARM3: movl $2, %ecx
    // REGPARM3: movl $3, (%esp)
    // REGPARM3: movl %esi, %eax
    // REGPARM3: calll force_hidden_sret_sink

    let b = unsafe { force_hidden_sret_sink(1, 2, 3) };
    minicore::hint::black_box(b);
}
