//@ compile-flags: -Zvirtual-function-elimination -Clto -Copt-level=0
#![crate_type = "lib"]

// A vtable that is only referenced from a const or static must get the same `!type` and
// `!vcall_visibility` metadata as one created by an unsizing coercion. Otherwise
// whole-program devirtualization does not see it as a possible call target.
//
// They previously took different paths in the backend and behavior diverged.

// CHECK: @vtable.0 = {{.*}}, !type ![[TYPE:[0-9]+]], !vcall_visibility ![[VIS:[0-9]+]]
// CHECK: @vtable.1 = {{.*}}, !type ![[TYPE]], !vcall_visibility ![[VIS]]
// CHECK: @vtable.2 = {{.*}}, !type ![[TYPE]], !vcall_visibility ![[VIS]]

pub trait Tr {
    fn f(&self) -> u32;
}

pub struct A;
pub struct B;
pub struct C;

impl Tr for A {
    fn f(&self) -> u32 {
        1
    }
}

impl Tr for B {
    fn f(&self) -> u32 {
        2
    }
}

impl Tr for C {
    fn f(&self) -> u32 {
        3
    }
}

pub static STATIC: &(dyn Tr + Sync) = &A;
pub const CONST: &(dyn Tr + Sync) = &C;

pub fn boxed() -> Box<dyn Tr> {
    Box::new(B)
}

pub fn constant() -> &'static dyn Tr {
    CONST
}

pub fn call(x: &dyn Tr) -> u32 {
    x.f()
}
