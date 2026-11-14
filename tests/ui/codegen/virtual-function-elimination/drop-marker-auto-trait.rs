//@ run-pass
//@ compile-flags: -Zvirtual-function-elimination -Clto=fat -Copt-level=3 -Zverify-llvm-ir
//@ no-prefer-dynamic
//@ ignore-backends: gcc
#![no_main]

use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};

static DROPPED: AtomicBool = AtomicBool::new(false);

trait Marker {}

struct S;

impl Marker for S {}

impl Drop for S {
    fn drop(&mut self) {
        DROPPED.store(true, Ordering::Relaxed);
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    // Create the vtable, but don't perform any checked loads on it.
    let marker: Box<dyn Marker> = black_box(Box::new(S));
    std::mem::forget(marker);

    let send: Box<dyn Send> = black_box(Box::new(S));
    drop(send);

    assert!(DROPPED.load(Ordering::Relaxed));

    0
}
