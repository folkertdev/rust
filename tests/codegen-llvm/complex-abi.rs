//! Checks that `#[repr(complex)]` `Complex<T>` matches the C `_Complex` ABI in `extern "C"`
//! functions. This is the rustc side of `tests/run-make/complex-c-abi`, which additionally
//! checks these signatures against clang. Revisions are grouped by LLVM component.

//@ add-minicore
//@ compile-flags: -C no-prepopulate-passes -Z codegen-source-order

//@ revisions: X86_64 I686 WINDOWS_MSVC WINDOWS_GNU WIN32_MSVC WIN32_GNU
//@ [X86_64] compile-flags: --target x86_64-unknown-linux-gnu
//@ [X86_64] needs-llvm-components: x86
//@ [I686] compile-flags: --target i686-unknown-linux-gnu
//@ [I686] needs-llvm-components: x86
//@ [WINDOWS_MSVC] compile-flags: --target x86_64-pc-windows-msvc
//@ [WINDOWS_MSVC] needs-llvm-components: x86
//@ [WINDOWS_GNU] compile-flags: --target x86_64-pc-windows-gnu
//@ [WINDOWS_GNU] needs-llvm-components: x86
//@ [WIN32_MSVC] compile-flags: --target i686-pc-windows-msvc
//@ [WIN32_MSVC] needs-llvm-components: x86
//@ [WIN32_GNU] compile-flags: --target i686-pc-windows-gnu
//@ [WIN32_GNU] needs-llvm-components: x86

//@ revisions: AARCH64 AARCH64_DARWIN AARCH64_MSVC ARM64EC
//@ [AARCH64] compile-flags: --target aarch64-unknown-linux-gnu
//@ [AARCH64] needs-llvm-components: aarch64
//@ [AARCH64_DARWIN] compile-flags: --target aarch64-apple-darwin
//@ [AARCH64_DARWIN] needs-llvm-components: aarch64
//@ [AARCH64_MSVC] compile-flags: --target aarch64-pc-windows-msvc
//@ [AARCH64_MSVC] needs-llvm-components: aarch64
//@ [ARM64EC] compile-flags: --target arm64ec-pc-windows-msvc
//@ [ARM64EC] needs-llvm-components: aarch64

//@ revisions: ARM
//@ [ARM] compile-flags: --target arm-unknown-linux-gnueabihf
//@ [ARM] needs-llvm-components: arm

//@ revisions: RISCV64 RISCV32
//@ [RISCV64] compile-flags: --target riscv64gc-unknown-linux-gnu
//@ [RISCV64] needs-llvm-components: riscv
//@ [RISCV32] compile-flags: --target riscv32gc-unknown-linux-gnu
//@ [RISCV32] needs-llvm-components: riscv

//@ revisions: LOONGARCH64 LOONGARCH32
//@ [LOONGARCH64] compile-flags: --target loongarch64-unknown-linux-gnu
//@ [LOONGARCH64] needs-llvm-components: loongarch
//@ [LOONGARCH32] compile-flags: --target loongarch32-unknown-none
//@ [LOONGARCH32] needs-llvm-components: loongarch

//@ revisions: SPARC64 SPARC
//@ [SPARC64] compile-flags: --target sparc64-unknown-linux-gnu
//@ [SPARC64] needs-llvm-components: sparc
//@ [SPARC] compile-flags: --target sparc-unknown-linux-gnu
//@ [SPARC] needs-llvm-components: sparc

//@ revisions: S390X
//@ [S390X] compile-flags: --target s390x-unknown-linux-gnu
//@ [S390X] needs-llvm-components: systemz

//@ revisions: POWERPC POWERPC64LE POWERPC64 AIX
//@ [POWERPC] compile-flags: --target powerpc-unknown-linux-gnu
//@ [POWERPC] needs-llvm-components: powerpc
//@ [POWERPC64LE] compile-flags: --target powerpc64le-unknown-linux-gnu
//@ [POWERPC64LE] needs-llvm-components: powerpc
//@ [POWERPC64] compile-flags: --target powerpc64-unknown-linux-gnu
//@ [POWERPC64] needs-llvm-components: powerpc
//@ [AIX] compile-flags: --target powerpc64-ibm-aix
//@ [AIX] needs-llvm-components: powerpc

//@ revisions: MIPS64EL MIPS
//@ [MIPS64EL] compile-flags: --target mips64el-unknown-linux-gnuabi64
//@ [MIPS64EL] needs-llvm-components: mips
//@ [MIPS] compile-flags: --target mips-unknown-linux-gnu
//@ [MIPS] needs-llvm-components: mips

//@ revisions: WASM32 WASM64
//@ [WASM32] compile-flags: --target wasm32-unknown-unknown
//@ [WASM32] needs-llvm-components: webassembly
//@ [WASM64] compile-flags: --target wasm64-unknown-unknown
//@ [WASM64] needs-llvm-components: webassembly

//@ revisions: CSKY
//@ [CSKY] compile-flags: --target csky-unknown-linux-gnuabiv2
//@ [CSKY] needs-llvm-components: csky

//@ revisions: NVPTX
//@ [NVPTX] compile-flags: --target nvptx64-nvidia-cuda
//@ [NVPTX] needs-llvm-components: nvptx

//@ revisions: BPF
//@ [BPF] compile-flags: --target bpfel-unknown-none
//@ [BPF] needs-llvm-components: bpf

#![feature(no_core, lang_items, repr_complex, f16, f128)]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
use minicore::num::Complex;

// AARCH64:        define{{.*}} [2 x half] @cplx_f16([2 x half]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x half] @cplx_f16([2 x half]{{.*}}
// AARCH64_MSVC:   define{{.*}} [2 x half] @cplx_f16([2 x half]{{.*}}
// ARM:            define{{.*}} i32 @cplx_f16(i32{{.*}}
// ARM64EC:        define{{.*}} [2 x half] @cplx_f16([2 x half]{{.*}}
// I686:           define{{.*}} <2 x half> @cplx_f16(ptr{{.*}}byval([4 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} { half, half } @cplx_f16({ half, half }{{.*}}
// LOONGARCH64:    define{{.*}} { half, half } @cplx_f16({ half, half }{{.*}}
// NVPTX:          define{{.*}} { half, half } @cplx_f16(ptr{{.*}}byval([4 x i8]){{.*}}
// RISCV32:        define{{.*}} { half, half } @cplx_f16({ half, half }{{.*}}
// RISCV64:        define{{.*}} { half, half } @cplx_f16({ half, half }{{.*}}
// S390X:          define{{.*}} void @cplx_f16(ptr{{.*}}sret([4 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} <2 x half> @cplx_f16(ptr{{.*}}byval([4 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} i32 @cplx_f16(i32{{.*}}
// WINDOWS_MSVC:   define{{.*}} i32 @cplx_f16(i32{{.*}}
// X86_64:         define{{.*}} <2 x half> @cplx_f16(<2 x half>{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f16(x: Complex<f16>) -> Complex<f16> {
    x
}

// AARCH64:        define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// AARCH64_MSVC:   define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// AIX:            define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// ARM:            define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// ARM64EC:        define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// BPF:            define{{.*}} void @cplx_f32(ptr{{.*}}sret([8 x i8]){{.*}}i64{{.*}}
// CSKY:           define{{.*}} [2 x i32] @cplx_f32([2 x i32]{{.*}}
// I686:           define{{.*}} i64 @cplx_f32(ptr{{.*}}byval([8 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// LOONGARCH64:    define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// MIPS:           define{{.*}} { float, float } @cplx_f32([2 x i32]{{.*}}
// MIPS64EL:       define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// NVPTX:          define{{.*}} { float, float } @cplx_f32(ptr{{.*}}byval([8 x i8]){{.*}}
// POWERPC:        define{{.*}} void @cplx_f32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// POWERPC64:      define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// POWERPC64LE:    define{{.*}} [2 x float] @cplx_f32([2 x float]{{.*}}
// RISCV32:        define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// RISCV64:        define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// S390X:          define{{.*}} void @cplx_f32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { float, float } @cplx_f32(ptr{{.*}}byval([8 x i8]){{.*}}
// SPARC64:        define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// WASM32:         define{{.*}} void @cplx_f32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_f32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} i64 @cplx_f32(ptr{{.*}}byval([8 x i8]){{.*}}
// WIN32_MSVC:     define{{.*}} i64 @cplx_f32(ptr{{.*}}byval([8 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} i64 @cplx_f32(i64{{.*}}
// WINDOWS_MSVC:   define{{.*}} i64 @cplx_f32(i64{{.*}}
// X86_64:         define{{.*}} <2 x float> @cplx_f32(<2 x float>{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f32(x: Complex<f32>) -> Complex<f32> {
    x
}

// AARCH64:        define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// AARCH64_MSVC:   define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// AIX:            define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// ARM:            define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// ARM64EC:        define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// BPF:            define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}[2 x i64]{{.*}}
// CSKY:           define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}[4 x i32]{{.*}}
// I686:           define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}byval([16 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// LOONGARCH64:    define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// MIPS:           define{{.*}} { double, double } @cplx_f64([4 x i32]{{.*}}
// MIPS64EL:       define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// NVPTX:          define{{.*}} { double, double } @cplx_f64(ptr{{.*}}byval([16 x i8]){{.*}}
// POWERPC:        define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// POWERPC64:      define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// POWERPC64LE:    define{{.*}} [2 x double] @cplx_f64([2 x double]{{.*}}
// RISCV32:        define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// RISCV64:        define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// S390X:          define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { double, double } @cplx_f64(ptr{{.*}}byval([16 x i8]){{.*}}
// SPARC64:        define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// WASM32:         define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}byval([16 x i8]){{.*}}
// WIN32_MSVC:     define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}byval([16 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WINDOWS_MSVC:   define{{.*}} void @cplx_f64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// X86_64:         define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
#[no_mangle]
pub extern "C" fn cplx_f64(x: Complex<f64>) -> Complex<f64> {
    x
}

// I686:           define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}byval([32 x i8]){{.*}}
// WASM32:         define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}byval([32 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}
// X86_64:         define{{.*}} void @cplx_f128(ptr{{.*}}sret([32 x i8]){{.*}}ptr{{.*}}byval([32 x i8]){{.*}}
#[no_mangle]
pub extern "C" fn cplx_f128(x: Complex<f128>) -> Complex<f128> {
    x
}

// AARCH64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i8(i64{{.*}}
// AARCH64_MSVC:   define{{.*}} i64 @cplx_i8(i64{{.*}}
// ARM:            define{{.*}} i32 @cplx_i8(i32{{.*}}
// ARM64EC:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// BPF:            define{{.*}} void @cplx_i8(ptr{{.*}}sret([2 x i8]){{.*}}i16{{.*}}
// CSKY:           define{{.*}} i32 @cplx_i8(i32{{.*}}
// I686:           define{{.*}} i16 @cplx_i8(ptr{{.*}}byval([2 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} i32 @cplx_i8(i32{{.*}}
// LOONGARCH64:    define{{.*}} i64 @cplx_i8(i64{{.*}}
// MIPS:           define{{.*}} { i8, i8 } @cplx_i8(i32{{.*}}
// MIPS64EL:       define{{.*}} i64 @cplx_i8(i64{{.*}}
// NVPTX:          define{{.*}} { i8, i8 } @cplx_i8(ptr{{.*}}byval([2 x i8]){{.*}}
// POWERPC:        define{{.*}} void @cplx_i8(ptr{{.*}}sret([2 x i8]){{.*}}ptr{{.*}}
// RISCV32:        define{{.*}} i32 @cplx_i8(i32{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// S390X:          define{{.*}} void @cplx_i8(ptr{{.*}}sret([2 x i8]){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i8, i8 } @cplx_i8(ptr{{.*}}byval([2 x i8]){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i8(ptr{{.*}}sret([2 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_i8(ptr{{.*}}sret([2 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} i16 @cplx_i8(ptr{{.*}}byval([2 x i8]){{.*}}
// WIN32_MSVC:     define{{.*}} i16 @cplx_i8(ptr{{.*}}byval([2 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} i16 @cplx_i8(i16{{.*}}
// WINDOWS_MSVC:   define{{.*}} i16 @cplx_i8(i16{{.*}}
// X86_64:         define{{.*}} i16 @cplx_i8(i16{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i8(x: Complex<i8>) -> Complex<i8> {
    x
}

// AARCH64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i16(i64{{.*}}
// AARCH64_MSVC:   define{{.*}} i64 @cplx_i16(i64{{.*}}
// ARM:            define{{.*}} i32 @cplx_i16(i32{{.*}}
// ARM64EC:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// BPF:            define{{.*}} void @cplx_i16(ptr{{.*}}sret([4 x i8]){{.*}}i32{{.*}}
// CSKY:           define{{.*}} i32 @cplx_i16(i32{{.*}}
// I686:           define{{.*}} i32 @cplx_i16(ptr{{.*}}byval([4 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} i32 @cplx_i16(i32{{.*}}
// LOONGARCH64:    define{{.*}} i64 @cplx_i16(i64{{.*}}
// MIPS:           define{{.*}} { i16, i16 } @cplx_i16(i32{{.*}}
// MIPS64EL:       define{{.*}} i64 @cplx_i16(i64{{.*}}
// NVPTX:          define{{.*}} { i16, i16 } @cplx_i16(ptr{{.*}}byval([4 x i8]){{.*}}
// POWERPC:        define{{.*}} void @cplx_i16(ptr{{.*}}sret([4 x i8]){{.*}}ptr{{.*}}
// RISCV32:        define{{.*}} i32 @cplx_i16(i32{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// S390X:          define{{.*}} void @cplx_i16(ptr{{.*}}sret([4 x i8]){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i16, i16 } @cplx_i16(ptr{{.*}}byval([4 x i8]){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i16(ptr{{.*}}sret([4 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_i16(ptr{{.*}}sret([4 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} i32 @cplx_i16(ptr{{.*}}byval([4 x i8]){{.*}}
// WIN32_MSVC:     define{{.*}} i32 @cplx_i16(ptr{{.*}}byval([4 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} i32 @cplx_i16(i32{{.*}}
// WINDOWS_MSVC:   define{{.*}} i32 @cplx_i16(i32{{.*}}
// X86_64:         define{{.*}} i32 @cplx_i16(i32{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i16(x: Complex<i16>) -> Complex<i16> {
    x
}

// AARCH64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64_MSVC:   define{{.*}} i64 @cplx_i32(i64{{.*}}
// ARM:            define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}[2 x i32]{{.*}}
// ARM64EC:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// BPF:            define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}i64{{.*}}
// CSKY:           define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// I686:           define{{.*}} i64 @cplx_i32(ptr{{.*}}byval([8 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// LOONGARCH64:    define{{.*}} i64 @cplx_i32(i64{{.*}}
// MIPS:           define{{.*}} { i32, i32 } @cplx_i32([2 x i32]{{.*}}
// MIPS64EL:       define{{.*}} i64 @cplx_i32(i64{{.*}}
// NVPTX:          define{{.*}} { i32, i32 } @cplx_i32(ptr{{.*}}byval([8 x i8]){{.*}}
// POWERPC:        define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// RISCV32:        define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// S390X:          define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i32, i32 } @cplx_i32(ptr{{.*}}byval([8 x i8]){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_i32(ptr{{.*}}sret([8 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} i64 @cplx_i32(ptr{{.*}}byval([8 x i8]){{.*}}
// WIN32_MSVC:     define{{.*}} i64 @cplx_i32(ptr{{.*}}byval([8 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} i64 @cplx_i32(i64{{.*}}
// WINDOWS_MSVC:   define{{.*}} i64 @cplx_i32(i64{{.*}}
// X86_64:         define{{.*}} i64 @cplx_i32(i64{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i32(x: Complex<i32>) -> Complex<i32> {
    x
}

// AARCH64:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AARCH64_MSVC:   define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AIX:            define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// ARM:            define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}[2 x i64]{{.*}}
// ARM64EC:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// BPF:            define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}[2 x i64]{{.*}}
// CSKY:           define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}[4 x i32]{{.*}}
// I686:           define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}byval([16 x i8]){{.*}}
// LOONGARCH32:    define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// LOONGARCH64:    define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// MIPS:           define{{.*}} { i64, i64 } @cplx_i64([4 x i32]{{.*}}
// MIPS64EL:       define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// NVPTX:          define{{.*}} { i64, i64 } @cplx_i64(ptr{{.*}}byval([16 x i8]){{.*}}
// POWERPC:        define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// POWERPC64:      define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// POWERPC64LE:    define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// RISCV32:        define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// RISCV64:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// S390X:          define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i64, i64 } @cplx_i64(ptr{{.*}}byval([16 x i8]){{.*}}
// SPARC64:        define{{.*}} { i64, i64 } @cplx_i64({ i64, i64 }{{.*}}
// WASM32:         define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WASM64:         define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}byval([16 x i8]){{.*}}
// WIN32_MSVC:     define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}byval([16 x i8]){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// WINDOWS_MSVC:   define{{.*}} void @cplx_i64(ptr{{.*}}sret([16 x i8]){{.*}}ptr{{.*}}
// X86_64:         define{{.*}} { i64, i64 } @cplx_i64({ i64, i64 }{{.*}}
#[no_mangle]
pub extern "C" fn cplx_i64(x: Complex<i64>) -> Complex<i64> {
    x
}
