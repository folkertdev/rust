//@ add-core-stubs
//@ needs-llvm-components: x86
//@ compile-flags: --target=i686-pc-windows-msvc --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]

extern crate minicore;
use minicore::*;

extern "stdcall" {
    fn printf(_: *const u8, ...);
    //~^ ERROR: C-variadic functions with the "stdcall" calling convention are not supported
}

fn baz(f: extern "Rust" fn(usize, ...)) {
    //~^ ERROR: C-variadic functions with the "Rust" calling convention are not supported
    f(22, 44);
}

extern "C" {
    fn foo(f: isize, x: u8, ...);
}

extern "C" fn bar(f: isize, x: u8) {}

fn main() {
    unsafe {
        foo(); //~ ERROR function takes at least 2 arguments but 0 arguments were supplied
        foo(1); //~ ERROR function takes at least 2 arguments but 1 argument was supplied

        let x: unsafe extern "C" fn(f: isize, x: u8) = foo; //~ ERROR mismatched types
        let y: extern "C" fn(f: isize, x: u8, ...) = bar; //~ ERROR mismatched types
    }
}
