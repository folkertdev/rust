//@revisions: stack tree
//@compile-flags: -Zmiri-strict-provenance
//@[tree]compile-flags: -Zmiri-tree-borrows
#![feature(c_variadic)]

use core::ffi::VaList;

fn ignores_arguments() {
    unsafe extern "C" fn variadic(_: ...) {}

    unsafe { variadic() };
    unsafe { variadic(1, 2, 3) };
}

fn echo() {
    unsafe extern "C" fn variadic(mut ap: ...) -> i32 {
        ap.arg()
    }

    assert_eq!(unsafe { variadic(1) }, 1);
    assert_eq!(unsafe { variadic(3, 2, 1) }, 3);
}

fn forward_by_val() {
    unsafe fn helper(mut ap: VaList) -> i32 {
        ap.arg()
    }

    unsafe extern "C" fn variadic(ap: ...) -> i32 {
        helper(ap)
    }

    assert_eq!(unsafe { variadic(1) }, 1);
    assert_eq!(unsafe { variadic(3, 2, 1) }, 3);
}

fn forward_by_ref() {
    unsafe fn helper(ap: &mut VaList) -> i32 {
        ap.arg()
    }

    unsafe extern "C" fn variadic(mut ap: ...) -> i32 {
        helper(&mut ap)
    }

    assert_eq!(unsafe { variadic(1) }, 1);
    assert_eq!(unsafe { variadic(3, 2, 1) }, 3);
}

fn main() {
    ignores_arguments();
    echo();
    forward_by_val();
    forward_by_ref();
}
