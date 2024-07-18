//@ build-fail
//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
#![feature(cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
impl Copy for u32 {}
impl Copy for u8 {}

#[repr(C)]
pub struct ReprCU64(u64);

#[repr(C)]
pub struct ReprCBytes(u8, u8, u8, u8, u8);

#[repr(C)]
pub struct U64Compound(u32, u32);

#[repr(C, align(16))]
pub struct ReprCAlign16(u16);

#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn f1() -> ReprCU64 {
    ReprCU64(0)
}
#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn f2() -> ReprCBytes {
    ReprCBytes(0, 1, 2, 3, 4)
}
#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn f3() -> U64Compound {
    U64Compound(2, 3)
}
#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn f4() -> ReprCAlign16 {
    ReprCAlign16(4)
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C-cmse-nonsecure-entry" fn f5() -> [u8; 5] {
    [0xAA; 5]
}
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C-cmse-nonsecure-entry" fn u128() -> u128 {
    123
}
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C-cmse-nonsecure-entry" fn i128() -> i128 {
    456
}

#[repr(Rust)]
pub union ReprRustUnionU64 {
    _unused: u64,
}

#[repr(C)]
pub union ReprCUnionU64 {
    _unused: u64,
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C-cmse-nonsecure-entry" fn union_rust() -> ReprRustUnionU64 {
    ReprRustUnionU64 { _unused: 1 }
}
#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn union_c() -> ReprCUnionU64 {
    ReprCUnionU64 { _unused: 2 }
}
