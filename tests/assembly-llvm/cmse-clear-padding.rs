// Verifies that padding/uninitialized bytes of values passed in registers across
// the secure/non-secure boundary are zeroed, so that secure-world data cannot
// leak to the non-secure world.
//
//@ add-minicore
//@ revisions: hard soft
//@ assembly-output: emit-asm
//@ [hard] compile-flags: --target thumbv8m.main-none-eabihf --crate-type lib -Copt-level=1
//@ [soft] compile-flags: --target thumbv8m.main-none-eabi --crate-type lib -Copt-level=1
//@ [hard] needs-llvm-components: arm
//@ [soft] needs-llvm-components: arm
#![crate_type = "lib"]
#![feature(abi_cmse_nonsecure_call, cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

// A 4-byte aggregate with one interior padding byte (offset 1, between the `u8`
// at offset 0 and the `u16` at offset 2). It is passed/returned by value in a
// single register.
#[repr(C)]
pub struct WithPadding {
    a: u8,
    b: u16,
}

// The return value is packed into `r0` with the interior padding byte (byte 1)
// cleared: `uxtb` zeroes bytes 1..=3 of `r0`, then `b` is OR'd into bytes 2..=3,
// leaving byte 1 as zero. Without the padding clearing, byte 1 would retain
// whatever (potentially secret) value was in the register.
//
// CHECK-LABEL: ret_with_padding:
// CHECK: uxtb r0, r0
// CHECK-NEXT: orr.w r0, r0, r1, lsl #16
#[no_mangle]
pub extern "cmse-nonsecure-entry" fn ret_with_padding(a: u8, b: u16) -> WithPadding {
    WithPadding { a, b }
}

// Likewise, the argument is packed into `r0` with its padding byte cleared
// before the call to the non-secure callee.
//
// CHECK-LABEL: call_with_padding:
// CHECK: uxtb r0, r1
// CHECK-NEXT: orr.w r0, r0, r2, lsl #16
#[no_mangle]
pub fn call_with_padding(f: unsafe extern "cmse-nonsecure-call" fn(WithPadding), x: WithPadding) {
    unsafe { f(x) }
}
