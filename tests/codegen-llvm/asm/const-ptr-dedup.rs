//@ compile-flags: -Copt-level=3
//@ needs-asm-support
#![feature(asm_const_ptr)]
#![crate_type = "lib"]

// Check that constants used by global assembly are deduplicated.

// CHECK-LABEL: module asm
// CHECK: /* foo [[SYM:_[A-Za-z0-9_.]+]] */
// CHECK: /* bar [[SYM]] */
core::arch::global_asm!("/* foo {} */", const &42u32);
core::arch::global_asm!("/* bar {} */", const &42u32);
