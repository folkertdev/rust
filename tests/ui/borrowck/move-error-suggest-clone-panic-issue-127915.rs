#![allow(dead_code)]
#![feature(c_variadic)]

extern "C" {
    fn rust_interesting_average(_: i64, ...) -> f64;
}

fn test<T, U>(a: i64, b: i64, c: i64, d: i64, e: i64, f: T, g: U) -> i64
where
    T: core::ffi::VaArgSafe,
    U: core::ffi::VaArgSafe,
{
    unsafe {
        rust_interesting_average(
            6, a as f64, b, b as f64, f, c as f64, d, d as f64, e, e as f64,
            f, //~ ERROR use of moved value: `f` [E0382]
            g,
        ) as i64
    }
}

fn main() {}
