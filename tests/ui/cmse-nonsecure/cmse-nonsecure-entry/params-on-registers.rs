//@ compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
//@ needs-llvm-components: arm
//@ assembly-output: emit-asm
//@ needs-asm-support
#![feature(cmse_nonsecure_entry, no_core, lang_items)]
#![no_core]
#![crate_type = "lib"]
#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}
impl Copy for u32 {}

//@ build-pass
// @ compile-flags: -C no-prepopulate-passes -Zbranch-protection=bti

// CHECK-LABEL: @entry_function
#[no_mangle]
pub extern "C-cmse-nonsecure-entry" fn entry_function(_: u32, _: u32, _: u32, d: u32) -> u32 {
    // CHECK: florpsz
    d
}
