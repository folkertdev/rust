//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
#![feature(cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
impl Copy for u32 {}

#[repr(C)]
struct Wrapper<T>(T);

impl<T: Copy> Wrapper<T> {
    extern "C-cmse-nonsecure-entry" fn ambient_generic(_: T, _: u32, _: u32, _: u32) -> u64 {
        //~^ ERROR [E0798]
        0
    }

    extern "C-cmse-nonsecure-entry" fn ambient_generic_nested(
        //~^ ERROR [E0798]
        _: Wrapper<T>,
        _: u32,
        _: u32,
        _: u32,
    ) -> u64 {
        0
    }
}

extern "C-cmse-nonsecure-entry" fn introduced_generic<U: Copy>(
    //~^ ERROR [E0798]
    _: U,
    _: u32,
    _: u32,
    _: u32,
) -> u64 {
    0
}

extern "C-cmse-nonsecure-entry" fn impl_trait(_: impl Copy, _: u32, _: u32, _: u32) -> u64 {
    //~^ ERROR [E0798]
    0
}
