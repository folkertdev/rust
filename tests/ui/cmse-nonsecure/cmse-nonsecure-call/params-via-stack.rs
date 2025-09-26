//@ add-core-stubs
//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
#![feature(abi_cmse_nonsecure_call, no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

#[repr(C, align(16))]
#[allow(unused)]
pub struct AlignRelevant(u32);

#[no_mangle]
pub fn test(
    f1: extern "cmse-nonsecure-call" fn(u32, u32, u32, u32, x: u32, y: u32), //~ ERROR [E0798]
    f2: extern "cmse-nonsecure-call" fn(u32, u32, u32, u16, u16),            //~ ERROR [E0798]
    f3: extern "cmse-nonsecure-call" fn(u32, u64, u32),                      //~ ERROR [E0798]
    f4: extern "cmse-nonsecure-call" fn(AlignRelevant, u32),                 //~ ERROR [E0798]
    f5: extern "cmse-nonsecure-call" fn([u32; 5]),                           //~ ERROR [E0798]
) {
}

#[repr(Rust)]
pub union ReprRustUnionU64 {
    _unused: u64,
}

#[repr(C)]
pub union ReprCUnionU64 {
    _unused: u64,
    _unused1: u32,
}

#[no_mangle]
pub fn test_union(
    f1: extern "cmse-nonsecure-call" fn(ReprRustUnionU64),
    //~^ WARN: passing a union across the security boundary may leak information
    f2: extern "cmse-nonsecure-call" fn(ReprCUnionU64),
    //~^ WARN: passing a union across the security boundary may leak information
    f3: extern "cmse-nonsecure-call" fn(MaybeUninit<u32>),
    //~^ WARN: passing a union across the security boundary may leak information
    f4: extern "cmse-nonsecure-call" fn(MaybeUninit<u64>),
    //~^ WARN: passing a union across the security boundary may leak information
) {
}
