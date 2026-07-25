//@ check-pass
//@ aux-build:unstable_layout_dep.rs
#![allow(dead_code, unused_variables, improper_ctypes, improper_ctypes_definitions)]
#![warn(dependence_on_unstable_layout_details)]
#![crate_type = "lib"]

extern crate unstable_layout_dep;

use unstable_layout_dep::{AllPublicFields, HasPrivateField};

struct DefaultRepr {
    a: u8,
    b: u16,
}

#[repr(C)]
struct StableReprC {
    a: u8,
    b: u32,
}

// A prototypical example: return a value witn an unstable repr.
extern "C" fn def_returns_default() -> DefaultRepr {
    //~^ WARN the layout of `DefaultRepr` is not guaranteed to be stable
    DefaultRepr { a: 0, b: 0 }
}

extern "C" {
    fn decl_stable(x: StableReprC);

    static STATIC_STABLE: StableReprC;

    fn decl_default_repr(x: DefaultRepr);
    //~^ WARN the layout of `DefaultRepr` is not guaranteed to be stable

    static STATIC_DEFAULT_REPR: DefaultRepr;
    //~^ WARN the layout of `DefaultRepr` is not guaranteed to be stable

    fn decl_public_fields(x: AllPublicFields);

    static STATIC_PUBLIC_FIELDS: AllPublicFields;

    fn decl_private_field(x: HasPrivateField);
    //~^ WARN the layout of `HasPrivateField` is not guaranteed to be stable

    static STATIC_PRIVATE_FIELD: HasPrivateField;
    //~^ WARN the layout of `HasPrivateField` is not guaranteed to be stable

}

// `extern "Rust"` makes no layout promise, so nothing to warn about.
extern "Rust" fn ordinary(x: DefaultRepr) {}

extern "C" fn scalar(_: i32, _: f32, _: bool, _: char) {}
extern "C" fn pointer(_: *const i32, _: *mut f32) {}
extern "C" fn slice(_: &[u8]) {}
extern "C" fn str_slice(_: &str) {}
extern "C" fn array(_: [u8; 8]) {}
extern "C" fn unit(_: ()) {}

#[repr(transparent)]
struct TransparentWrapper<T>(T);

extern "C" fn transparent_stable(_: TransparentWrapper<i32>) {}
extern "C" fn transparent_unstable(_: TransparentWrapper<DefaultRepr>) {}
//~^ WARN the layout of `DefaultRepr` is not guaranteed to be stable

#[repr(u8)]
enum RGB {
    R,
    G,
    B,
}

#[repr(i32)]
enum StableOption<T> {
    Just(T),
    Nothing,
}

extern "C" fn repr_c_struct(_: StableReprC) {}
extern "C" fn repr_primitive_c_enum(_: RGB) {}
extern "C" fn repr_primitive_enum_with_fields(_: StableOption<u8>) {}
extern "C" fn repr_primitive_enum_with_fields(_: AllPublicFields) {}

// Non-unit tuples has no layout guarantee.
extern "C" fn tuple(pair: (u8, u16)) {}
//~^ WARN the layout of `(u8, u16)` is not guaranteed to be stable

// Arrays have the layout guarantee of their elements.
extern "C" fn tuple(pair: [(u8, u16); 4]) {}
//~^ WARN the layout of `(u8, u16)` is not guaranteed to be stable

// Foreign `repr(C)` type with a private field.
extern "C" fn def_takes_foreign_private(x: HasPrivateField) {}
//~^ WARN the layout of `HasPrivateField` is not guaranteed to be stable
