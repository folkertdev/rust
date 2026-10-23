//@ compile-flags: -Copt-level=3 -Zvirtual-function-elimination -Clto
//@ needs-asm-support
#![feature(asm_const_ptr)]
#![crate_type = "lib"]

pub trait Tr {
    fn f(&self) -> u32;
}

pub struct A;

impl Tr for A {
    fn f(&self) -> u32 {
        1
    }
}

const A_VTABLE: *const () =
    unsafe { core::mem::transmute::<&dyn Tr, [*const (); 2]>(&A as &dyn Tr)[1] };

core::arch::global_asm!("/* {} */", const A_VTABLE);

// CHECK: module asm
// CHECK: "/* [[VT:_[A-Za-z0-9_.]+]] */"
// CHECK: @[[VT]] = {{.*}}, !type !{{[0-9]+}}, !vcall_visibility
