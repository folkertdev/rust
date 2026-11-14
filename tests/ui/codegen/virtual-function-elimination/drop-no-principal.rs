//@ run-pass
//@ compile-flags: -Zvirtual-function-elimination -Clto=fat -Copt-level=3 -Zverify-llvm-ir
//@ no-prefer-dynamic
//@ ignore-backends: gcc
#![no_main]

use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};

static X_DROPPED: AtomicBool = AtomicBool::new(false);
static Y_DROPPED: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
trait T {
    fn method(&self) -> i32;
}

struct X;

impl T for X {
    fn method(&self) -> i32 {
        4
    }
}

impl Drop for X {
    fn drop(&mut self) {
        X_DROPPED.store(true, Ordering::Relaxed);
    }
}

struct Y;

impl Drop for Y {
    fn drop(&mut self) {
        Y_DROPPED.store(true, Ordering::Relaxed);
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    let x: Box<dyn T> = black_box(Box::new(X));
    drop(x);
    assert!(X_DROPPED.load(Ordering::Relaxed));
    X_DROPPED.load(Ordering::Relaxed);

    let y: Box<dyn Send> = black_box(Box::new(Y));
    drop(y);
    assert!(Y_DROPPED.load(Ordering::Relaxed));

    0
}
