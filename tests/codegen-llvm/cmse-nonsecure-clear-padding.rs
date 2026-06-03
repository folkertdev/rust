// Verifies that padding/uninitialized bytes of values passed in registers across
// the secure/non-secure boundary are zeroed, so that secure-world data cannot
// leak to the non-secure world.
//
// - `cmse-nonsecure-entry` returns: padding in the returned value is zeroed
//   before the value is loaded into the return register(s).
// - `cmse-nonsecure-call` arguments: padding in by-value arguments is zeroed
//   before the value is loaded into the argument register(s).
//
//@ add-minicore
//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib -Cno-prepopulate-passes -Copt-level=1
//@ needs-llvm-components: arm
//@ ignore-backends: gcc
#![feature(abi_cmse_nonsecure_call, cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
use minicore::*;

// A 4-byte aggregate with one interior padding byte (byte at offset 1, between
// the `u8` at offset 0 and the `u16` at offset 2). It fits in a single register
// and is therefore passed/returned by-value (`PassMode::Cast`).
#[repr(C)]
pub struct WithPadding {
    a: u8,
    b: u16,
}

// CHECK-LABEL: @ret_with_padding
// The interior padding byte is zeroed before the value is returned.
// CHECK: call void @llvm.memset.p0.i32(ptr {{.*}}, i8 0, i32 1, i1 false)
// CHECK: ret i32
#[no_mangle]
pub extern "cmse-nonsecure-entry" fn ret_with_padding(a: u8, b: u16) -> WithPadding {
    WithPadding { a, b }
}

// A plain scalar return has no padding, so no clearing is emitted.
// CHECK-LABEL: @ret_no_padding
// CHECK-NOT: call void @llvm.memset
// CHECK: ret i32
#[no_mangle]
pub extern "cmse-nonsecure-entry" fn ret_no_padding(a: u32) -> u32 {
    a
}

// CHECK-LABEL: @call_with_padding
// The interior padding byte of the argument is zeroed before the call.
// CHECK: call void @llvm.memset.p0.i32(ptr {{.*}}, i8 0, i32 1, i1 false)
// CHECK: call void %f
#[no_mangle]
pub fn call_with_padding(f: unsafe extern "cmse-nonsecure-call" fn(WithPadding), x: WithPadding) {
    unsafe { f(x) }
}

// Passing only scalars to a non-secure call needs no padding clearing.
// CHECK-LABEL: @call_no_padding
// CHECK-NOT: call void @llvm.memset
// CHECK: call void %f
#[no_mangle]
pub fn call_no_padding(f: unsafe extern "cmse-nonsecure-call" fn(u32, u32), x: u32, y: u32) {
    unsafe { f(x, y) }
}
