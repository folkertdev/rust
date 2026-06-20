//@ run-pass
//@ only-x86_64

// Exercises the `core::arch::x86_64::f80` (x87 80-bit extended-precision) type: its layout, the
// `Clone`/`Copy`/`PartialEq`/`PartialOrd` impls, and `f64 <-> f80` casts, in both const and runtime
// contexts.

#![feature(x86_f80)]

use std::arch::x86_64::f80;
use std::cmp::Ordering;
use std::mem::{align_of, size_of};

// Layout: on x86-64 `f80` is 16-byte aligned, with the size padded up to the alignment.
const _: () = assert!(size_of::<f80>() == 16);
const _: () = assert!(align_of::<f80>() == 16);

// `f80` constants, built via `f64 as f80` casts (const-evaluated through apfloat).
const ZERO: f80 = 0.0_f64 as f80;
const ONE: f80 = 1.0_f64 as f80;
const TWO: f80 = 2.0_f64 as f80;

// Builtin comparisons in a const context (go through the const interpreter's float ops).
const _: () = assert!(ZERO < ONE);
const _: () = assert!(ONE < TWO);
const _: () = assert!(ONE <= ONE);
const _: () = assert!(!(TWO < ONE));

// Associated constants on the type.
const _: () = assert!(f80::ZERO < f80::ONE);
const _: () = assert!(f80::ONE < f80::INFINITY);
const _: () = assert!(f80::NEG_INFINITY < f80::ZERO);

// `from_*_bytes` / `to_*_bytes` are `const` and round-trip in a const context.
const ONE_LE: [u8; 10] = f80::ONE.to_le_bytes();
// 1.0 in x87 80-bit extended: integer bit + biased exponent 0x3FFF, everything else zero.
const _: () = assert!(matches!(ONE_LE, [0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0x3F]));
const _: () = assert!(f80::from_le_bytes(ONE_LE) as f64 == 1.0);
const _: () = assert!(f80::from_ne_bytes(f80::ONE.to_ne_bytes()) as f64 == 1.0);
const _: () = assert!(f80::from_be_bytes(f80::ONE.to_be_bytes()) as f64 == 1.0);

fn main() {
    let a = 1.5_f64 as f80;
    let b = 2.5_f64 as f80;

    // `Copy` and `Clone` both produce equal values.
    let copied = a;
    let cloned = a.clone();
    assert!(a == copied);
    assert!(a == cloned);

    // `PartialEq`.
    assert!(a == a);
    assert!(a != b);
    assert_eq!(a == b, false);

    // `PartialOrd`.
    assert!(a < b);
    assert!(b > a);
    assert!(a <= copied);
    assert!(a >= copied);
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
    assert_eq!(a.partial_cmp(&copied), Some(Ordering::Equal));

    // NaN compares unequal to everything, including itself.
    let nan = (0.0_f64 / 0.0_f64) as f80;
    assert!(nan != nan);
    assert_eq!(nan.partial_cmp(&nan), None);
    assert!(!(nan < nan));
    assert!(!(nan >= nan));

    // Values round-trip back through `f64`.
    assert_eq!(a as f64, 1.5);
    assert_eq!(ONE as f64, 1.0);
    assert_eq!(TWO as f64, 2.0);

    // Runtime comparisons agree with the constants.
    assert!(a > ONE);
    assert!(b > TWO);

    // Associated constants.
    assert!(f80::NAN != f80::NAN);
    assert_eq!(f80::NAN.partial_cmp(&f80::NAN), None);
    assert!(f80::INFINITY > f80::ONE);
    assert!(f80::NEG_INFINITY < f80::ZERO);
    assert_eq!(f80::ONE as f64, 1.0);
    assert_eq!(f80::ZERO as f64, 0.0);

    // Byte conversions round-trip. (`f80` has no `Debug`, so use `==` rather than `assert_eq!`.)
    assert!(f80::from_ne_bytes(a.to_ne_bytes()) == a);
    assert!(f80::from_le_bytes(a.to_le_bytes()) == a);
    assert!(f80::from_be_bytes(a.to_be_bytes()) == a);

    // `to_be_bytes` is the reverse of `to_le_bytes`.
    let mut le = a.to_le_bytes();
    let be = a.to_be_bytes();
    le.reverse();
    assert_eq!(le, be);

    // Exact x87 representation of `1.0`.
    let one_le: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0x3F];
    assert_eq!(f80::ONE.to_le_bytes(), one_le);
    assert_eq!(f80::from_le_bytes(one_le) as f64, 1.0);

    // Display / ToString / Debug — shortest decimal via the (now `u128`-wide) flt2dec dragon path.
    assert_eq!(f80::ZERO.to_string(), "0");
    assert_eq!(f80::ONE.to_string(), "1");
    assert_eq!((1.5_f64 as f80).to_string(), "1.5");
    assert_eq!((2.5_f64 as f80).to_string(), "2.5");
    assert_eq!((-0.25_f64 as f80).to_string(), "-0.25");
    assert_eq!(format!("{}", f80::INFINITY), "inf");
    assert_eq!(format!("{}", f80::NEG_INFINITY), "-inf");
    assert_eq!(format!("{}", f80::NAN), "NaN");
    assert_eq!(format!("{:?}", f80::ONE), "1");
    assert_eq!(format!("{:+}", f80::ONE), "+1");

    // More precision than `f64`: `1 + 2^-63` rounds to exactly `1.0` as an `f64`, but `f80` keeps
    // it, and the formatter shows the extra digits.
    let one_plus_eps = f80::from_le_bytes([1, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0x3F]);
    assert!(one_plus_eps != f80::ONE);
    assert!(one_plus_eps.to_string().starts_with("1.0000000000000000"));

    // Larger range than `f64`: `2^1024` overflows `f64` to infinity but is finite as `f80`.
    // Display uses fixed-point (like the other floats), so it is a ~309-digit integer.
    let two_pow_1024 = f80::from_le_bytes([0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0x43]);
    let s = two_pow_1024.to_string();
    assert!(s.starts_with("179"));
    assert!(s.len() > 300);
}
