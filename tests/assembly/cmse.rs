//@ assembly-output: emit-asm
//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib -Copt-level=1
//@ needs-llvm-components: arm
#![crate_type = "lib"]
#![feature(abi_c_cmse_nonsecure_call, cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}

// CHECK-LABEL: __acle_se_entry_point
// CHECK: entry_point:
//
// Write return argument (two registers since 64bit integer)
// CHECK: movs r0, #0
// CHECK: movs r1, #0
//
// Clear all other registers that might have been used
// CHECK: mov r2, lr
// CHECK: mov r3, lr
// CHECK: mov r12, lr
//
// Clear the flags
// CHECK: msr apsr_nzcvq, lr
//
// Branch back to non-secure side
// CHECK: bxns lr
#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn entry_point() -> i64 {
    0
}

// NOTE to future codegen changes:
// The specific register assignment is not important, however:
// * all registers must be cleared before `blxns` is executed (either by writing arguments or any other value)
// * the lowest bit on the address of the callee must be cleared
// * the flags need to be overwritten
// * `blxns` needs to be called with the callee address (with the lowest bit cleared)
//
// CHECK-LABEL: call_nonsecure
// All arguments are written to (writes r0..=r3 and r12)
// CHECK: mov r12, r0
// CHECK: movs r0, #0
// CHECK: movs r1, #1
// CHECK: movs r2, #2
// CHECK: movs r3, #3
//
// Lowest bit gets cleared on callee address
// CHECK: bic r12, r12, #1
//
// Ununsed registers get cleared (r4..=r11)
// CHECK: mov r4, r12
// CHECK: mov r5, r12
// CHECK: mov r6, r12
// CHECK: mov r7, r12
// CHECK: mov r8, r12
// CHECK: mov r9, r12
// CHECK: mov r10, r12
// CHECK: mov r11, r12
//
// Flags get cleared
// CHECK: msr apsr_nzcvq, r12
//
// Call to non-secure
// CHECK: blxns r12
#[no_mangle]
pub fn call_nonsecure(
    f: unsafe extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32) -> u64,
) -> u64 {
    unsafe { f(0, 1, 2, 3) }
}
