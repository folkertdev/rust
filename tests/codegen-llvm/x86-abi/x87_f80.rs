//@ add-minicore
//
//@ revisions: X86 X86_64 DARWIN WIN64 OPT0
//@ [X86] compile-flags: --target i686-unknown-linux-gnu
//@ [X86_64] compile-flags: --target x86_64-unknown-linux-gnu
//@ [DARWIN] compile-flags: --target i686-apple-darwin
//@ [WIN64] compile-flags: --target x86_64-pc-windows-gnu
//@ [OPT0] compile-flags: --target x86_64-unknown-linux-gnu -Copt-level=0
//@ compile-flags: -Copt-level=3 --crate-type=lib -Zmerge-functions=disabled
//@ needs-llvm-components: x86

#![feature(no_core)]
#![no_std]
#![no_core]

extern crate minicore;
#[cfg(target_arch = "x86")]
use minicore::arch::x86::x87_f80;
#[cfg(target_arch = "x86_64")]
use minicore::arch::x86_64::x87_f80;

#[repr(C)]
struct Single {
    a: x87_f80,
}

#[repr(C)]
struct Pair {
    a: x87_f80,
    b: x87_f80,
}

#[repr(C)]
struct Mixed {
    a: f64,
    b: x87_f80,
}

// X86-LABEL: x86_fp80 @scalar_second(x86_fp80 noundef %_a, x86_fp80 noundef returned %b)
// X86_64-LABEL: x86_fp80 @scalar_second(x86_fp80 noundef %_a, x86_fp80 noundef returned %b)
// DARWIN-LABEL: x86_fp80 @scalar_second(x86_fp80 noundef %_a, x86_fp80 noundef returned %b)
// WIN64-LABEL: void @scalar_second(ptr {{.*}}sret([16 x i8]) {{.*}}, ptr {{.*}}, ptr {{.*}})
#[unsafe(no_mangle)]
extern "C" fn scalar_second(_a: x87_f80, b: x87_f80) -> x87_f80 {
    b
}

// X86-LABEL: void @single(ptr {{.*}}sret([12 x i8]) align 4 {{.*}}, ptr {{.*}}byval([12 x i8]) align 4 {{.*}})
// X86_64-LABEL: x86_fp80 @single(ptr {{.*}}byval([16 x i8]) align 16 {{.*}})
// DARWIN-LABEL: void @single(ptr {{.*}}sret([16 x i8]) align 16 {{.*}}, ptr {{.*}}byval([16 x i8]) align 4 {{.*}})
// WIN64-LABEL: void @single(ptr {{.*}}sret([16 x i8]) align 16 {{.*}}, ptr {{.*}})
#[unsafe(no_mangle)]
extern "C" fn single(x: Single) -> Single {
    x
}

// X86-LABEL: void @pair(ptr {{.*}}sret([24 x i8]) align 4 {{.*}}, ptr {{.*}}byval([24 x i8]) align 4 {{.*}})
// X86_64-LABEL: void @pair(ptr {{.*}}sret([32 x i8]) align 16 {{.*}}, ptr {{.*}}byval([32 x i8]) align 16 {{.*}})
// DARWIN-LABEL: void @pair(ptr {{.*}}sret([32 x i8]) align 16 {{.*}}, ptr {{.*}}byval([32 x i8]) align 4 {{.*}})
// WIN64-LABEL: void @pair(ptr {{.*}}sret([32 x i8]) align 16 {{.*}}, ptr {{.*}})
#[unsafe(no_mangle)]
extern "C" fn pair(x: Pair) -> Pair {
    x
}

// Projecting an x87_f80 out of an aggregate used to ICE, because code asserted that
// a field has the same size as the scalar it contains. On higher optimization levels
// this projection is optimized out, hence the OPT0 revision.
//
// OPT0-LABEL: @pair_first(
#[unsafe(no_mangle)]
fn pair_first(x: Pair) -> x87_f80 {
    x.a
}

// OPT0-LABEL: @pair_second(
#[unsafe(no_mangle)]
fn pair_second(x: Pair) -> x87_f80 {
    x.b
}

// X86-LABEL: x86_fp80 @mixed(ptr {{.*}}byval([20 x i8]) align 4 {{.*}})
// X86_64-LABEL: x86_fp80 @mixed(ptr {{.*}}byval([32 x i8]) align 16 {{.*}})
// DARWIN-LABEL: x86_fp80 @mixed(ptr {{.*}}byval([32 x i8]) align 4 {{.*}})
// WIN64-LABEL: void @mixed(ptr {{.*}}sret([16 x i8]) align 16 {{.*}}, ptr {{.*}})
#[unsafe(no_mangle)]
extern "C" fn mixed(x: Mixed) -> x87_f80 {
    x.b
}

// X86-LABEL: x86_fp80 @many(x86_fp80 noundef %_a, x86_fp80 noundef %_b, x86_fp80 noundef %_c, x86_fp80 noundef %_d, x86_fp80 noundef %_e, x86_fp80 noundef %_f, x86_fp80 noundef %_g, x86_fp80 noundef %_h, x86_fp80 noundef returned %i)
// X86_64-LABEL: x86_fp80 @many(x86_fp80 noundef %_a, x86_fp80 noundef %_b, x86_fp80 noundef %_c, x86_fp80 noundef %_d, x86_fp80 noundef %_e, x86_fp80 noundef %_f, x86_fp80 noundef %_g, x86_fp80 noundef %_h, x86_fp80 noundef returned %i)
// DARWIN-LABEL: x86_fp80 @many(x86_fp80 noundef %_a, x86_fp80 noundef %_b, x86_fp80 noundef %_c, x86_fp80 noundef %_d, x86_fp80 noundef %_e, x86_fp80 noundef %_f, x86_fp80 noundef %_g, x86_fp80 noundef %_h, x86_fp80 noundef returned %i)
// WIN64-LABEL: void @many(ptr {{.*}}sret([16 x i8]) {{.*}}
#[unsafe(no_mangle)]
extern "C" fn many(
    _a: x87_f80,
    _b: x87_f80,
    _c: x87_f80,
    _d: x87_f80,
    _e: x87_f80,
    _f: x87_f80,
    _g: x87_f80,
    _h: x87_f80,
    i: x87_f80,
) -> x87_f80 {
    i
}
