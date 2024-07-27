//@ compile-flags: -C no-prepopulate-passes -Copt-level=0
//@ needs-asm-support
//@ only-x86_64

#![crate_type = "lib"]
#![feature(naked_functions)]
use std::arch::{asm, global_asm};
// CHECK: module asm ".intel_syntax
// CHECK: .pushsection .text.naked_empty,\22ax\22, @progbit
// CHECK: .balign 4
// CHECK: .globl naked_empty
// CHECK-NOT: .hidden naked_empty
// CHECK: .type naked_empty, @function
// CHECK-LABEL: naked_empty:
// CHECK: ret
// CHECK: .popsection
// CHECK: .att_syntax

#[no_mangle]
#[naked]
pub unsafe extern "C" fn naked_empty() {
    asm!("ret", options(noreturn));
}

// CHECK: .intel_syntax
// CHECK: .pushsection .text.naked_with_args_and_return,\22ax\22, @progbits
// CHECK: .balign 4
// CHECK: .globl naked_with_args_and_return
// CHECK-NOT: .hidden naked_with_args_and_return
// CHECK: .type naked_with_args_and_return, @function
// CHECK-LABEL: naked_with_args_and_return:
// CHECK: lea rax, [rdi + rsi]
// CHECK: ret
// CHECK: .size naked_with_args_and_return, . - naked_with_args_and_return
// CHECK: .popsection
// CHECK: .att_syntax

#[no_mangle]
#[naked]
pub unsafe extern "C" fn naked_with_args_and_return(a: isize, b: isize) -> isize {
    asm!("lea rax, [rdi + rsi]", "ret", options(noreturn));
}
