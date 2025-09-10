#![feature(c_variadic, f16, f128)]

fn generic_unconstrained<T>(f: extern "C" fn(...) -> i32, x: T) -> i32 {
    f(x)
    //~^ ERROR the trait bound `T: VaArgSafe` is not satisfied
}

fn generic_constrained<T: core::ffi::VaArgSafe>(f: extern "C" fn(...) -> i32, x: T) -> i32 {
    f(x)
}

extern "C" {
    fn foo(f: isize, x: u8, ...);
}

fn main() {
    unsafe {
        foo(1, 2, true); //~ ERROR can't pass
        foo(1, 2, 'q'); //~ ERROR the trait bound `char: VaArgSafe` is not satisfied

        foo(1, 2, 1i8); //~ ERROR can't pass
        foo(1, 2, 1i16); //~ ERROR can't pass
        foo(1, 2, 1i32);
        foo(1, 2, 1i64);
        foo(1, 2, 1i128); //~ ERROR the trait bound `i128: VaArgSafe` is not satisfied

        foo(1, 2, 1u8); //~ ERROR can't pass
        foo(1, 2, 1u16); //~ ERROR can't pass
        foo(1, 2, 1u32);
        foo(1, 2, 1u64);
        foo(1, 2, 1u128); //~ ERROR the trait bound `u128: VaArgSafe` is not satisfied

        foo(1, 2, 3f16); //~ ERROR the trait bound `f16: VaArgSafe` is not satisfied
        foo(1, 2, 3f32); //~ ERROR can't pass
        foo(1, 2, 3f64);
        foo(1, 2, 3f128); //~ ERROR the trait bound `f128: VaArgSafe` is not satisfied

        foo(1, 2, 1); // defaults to i32, which is accepted
        foo(1, 2, 0.0); // defaults to f64, which is accepted

        foo(1, 2, core::ptr::null());
        foo(1, 2, core::ptr::null_mut());

        foo(1, 2, &0);
        foo(1, 2, &mut 0);

        struct S;
        foo(1, 2, S); //~ ERROR the trait bound `S: VaArgSafe` is not satisfied
    }
}
