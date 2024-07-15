//@ build-fail
//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
#![feature(cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]
#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}
impl Copy for u32 {}

#[cmse_nonsecure_entry]
pub extern "C" fn works1(_x1: u32, _x2: u32, _x3: u32, _x4: u32) {}

#[cmse_nonsecure_entry]
pub extern "C" fn works2(_x1: u64, _x2: u64) {}

#[cmse_nonsecure_entry]
pub extern "C" fn fails1(_x1: u32, _x2: u32, _x3: u32, _x4: u32, _x5: u32) {}

#[cmse_nonsecure_entry]
pub extern "C" fn fails2(_x1: u32, _x2: u32, _x3: u32, _x4: u16, _x5: u16) {}

#[cmse_nonsecure_entry]
pub extern "C" fn fails3(_x1: u32, _x2: u64, _x3: u32) {}

#[repr(C, align(16))]
#[allow(unused)]
pub struct AlignRelevant(u32);

#[cmse_nonsecure_entry]
extern "C" fn fails4(_x1: AlignRelevant, _x2: u32) {}
