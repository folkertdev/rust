//@ run-pass
//@ compile-flags: -Zvirtual-function-elimination -Clto=fat -Copt-level=3 -Zverify-llvm-ir
//@ no-prefer-dynamic
//@ ignore-backends: gcc
#![no_main]

use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};

static DROPPED: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
trait T {
    fn method(&self) -> i32;
}

struct S;

impl T for S {
    fn method(&self) -> i32 {
        4
    }
}

impl Drop for S {
    fn drop(&mut self) {
        DROPPED.store(true, Ordering::Relaxed);
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    let t: Box<dyn T + Send> = Box::new(S);
    let send: Box<dyn Send> = black_box(t);
    drop(send);

    assert!(DROPPED.load(Ordering::Relaxed));

    0
}
