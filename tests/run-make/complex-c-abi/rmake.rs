//! Validates the `_Complex T` ABI, matching clang.
//!
//! `complex.c` records, per target, how clang passes and returns each `_Complex` type (its
//! `define` signature). This test first checks that clang still emits those signatures, then
//! checks that the equivalent `#[repr(complex)]` `Complex<T>` from Rust reproduces them in
//! `extern "C"` functions (ABI-compatible; see `complex.rs`).
//!
//! The Rust side only runs targets whose callconv already implements the complex ABI
//! (`RUST_TARGETS`); the commented-out entries still need per-target callconv work.

//@ needs-force-clang-based-tests

use run_make_support::{clang, llvm_filecheck, rfs, rustc, rustc_minicore};

/// `(FileCheck prefix, clang target triple, extra clang flags, rustc target triple)`.
const TARGETS: &[(&str, &str, &[&str], &str)] = &[
    ("X86_64", "x86_64-unknown-linux-gnu", &[], "x86_64-unknown-linux-gnu"),
    ("I686", "i686-unknown-linux-gnu", &["-msse2"], "i686-unknown-linux-gnu"),
    ("WINDOWS_MSVC", "x86_64-pc-windows-msvc", &[], "x86_64-pc-windows-msvc"),
    ("WINDOWS_GNU", "x86_64-pc-windows-gnu", &[], "x86_64-pc-windows-gnu"),
    ("AARCH64", "aarch64-unknown-linux-gnu", &[], "aarch64-unknown-linux-gnu"),
    ("AARCH64_DARWIN", "aarch64-apple-darwin", &[], "aarch64-apple-darwin"),
    ("AARCH64_MSVC", "aarch64-pc-windows-msvc", &[], "aarch64-pc-windows-msvc"),
    ("ARM64EC", "arm64ec-pc-windows-msvc", &[], "arm64ec-pc-windows-msvc"),
    ("ARM", "arm-unknown-linux-gnueabihf", &[], "arm-unknown-linux-gnueabihf"),
    ("RISCV64", "riscv64-unknown-linux-gnu", &["-march=rv64gc"], "riscv64gc-unknown-linux-gnu"),
    ("RISCV32", "riscv32-unknown-linux-gnu", &["-march=rv32gc"], "riscv32gc-unknown-linux-gnu"),
    ("S390X", "s390x-unknown-linux-gnu", &[], "s390x-unknown-linux-gnu"),
    ("POWERPC64LE", "powerpc64le-unknown-linux-gnu", &[], "powerpc64le-unknown-linux-gnu"),
    ("POWERPC64", "powerpc64-unknown-linux-gnu", &["-mabi=elfv1"], "powerpc64-unknown-linux-gnu"),
    ("AIX", "powerpc64-ibm-aix", &[], "powerpc64-ibm-aix"),
    ("POWERPC", "powerpc-unknown-linux-gnu", &[], "powerpc-unknown-linux-gnu"),
    ("SPARC64", "sparc64-unknown-linux-gnu", &[], "sparc64-unknown-linux-gnu"),
    ("MIPS64EL", "mips64el-unknown-linux-gnuabi64", &[], "mips64el-unknown-linux-gnuabi64"),
    ("MIPS", "mips-unknown-linux-gnu", &[], "mips-unknown-linux-gnu"),
    ("WASM32", "wasm32-unknown-unknown", &[], "wasm32-unknown-unknown"),
    ("WASM64", "wasm64-unknown-unknown", &[], "wasm64-unknown-unknown"),
    ("LOONGARCH64", "loongarch64-unknown-linux-gnu", &[], "loongarch64-unknown-linux-gnu"),
    ("LOONGARCH32", "loongarch32-unknown-none", &[], "loongarch32-unknown-none"),
    ("SPARC", "sparc-unknown-linux-gnu", &[], "sparc-unknown-linux-gnu"),
    ("WIN32_MSVC", "i686-pc-windows-msvc", &[], "i686-pc-windows-msvc"),
    ("WIN32_GNU", "i686-pc-windows-gnu", &[], "i686-pc-windows-gnu"),
    ("NVPTX", "nvptx64-nvidia-cuda", &[], "nvptx64-nvidia-cuda"),
    ("BPF", "bpfel", &[], "bpfel-unknown-none"),
    // clang rejects the `gnuabiv2` version in the triple, so the C side uses bare `csky`.
    ("CSKY", "csky", &[], "csky-unknown-linux-gnuabiv2"),
];

/// Prefixes from `TARGETS` whose Rust callconv already reproduces the C `_Complex` ABI. The
/// commented-out targets still deviate and need per-target callconv work before they can run.
const RUST_TARGETS: &[&str] = &[
    "X86_64",
    "AARCH64",
    "ARM",
    "RISCV64",
    "RISCV32",
    "SPARC64",
    "WINDOWS_MSVC",
    "WINDOWS_GNU",
    "AARCH64_DARWIN",
    "AARCH64_MSVC",
    "ARM64EC",
    "WASM32",
    "S390X",
    "I686",
    "POWERPC",
    "POWERPC64LE", // int-complex excluded in complex.rs: clang spreads 2 GPRs, rustc packs one
    "MIPS64EL",
    "WASM64",
    "LOONGARCH64",
    "LOONGARCH32",
    "WIN32_GNU",
    "WIN32_MSVC", // f16 excluded in complex.rs: clang returns `<2 x half>`, rustc `i32`
    "CSKY",
    "POWERPC64",   // int-complex excluded (same as POWERPC64LE)
    "AIX",         // int-complex excluded (same as POWERPC64LE)
    "SPARC",
    "NVPTX",
    "BPF",
    "MIPS",
];

fn main() {
    // The C reference: clang must emit the recorded signatures.
    for (prefix, ctriple, cflags, _) in TARGETS {
        let ir = clang()
            .target(ctriple)
            .args(*cflags)
            .args(["-O2", "-S", "-emit-llvm", "-o", "-", "complex.c"])
            .run()
            .stdout_utf8();
        llvm_filecheck().patterns("complex.c").check_prefix(prefix).stdin_buf(ir).run();
    }

    // The Rust side: `#[repr(complex)]` must reproduce the same ABI.
    for prefix in RUST_TARGETS {
        let (_, _, _, rtriple) =
            TARGETS.iter().find(|(p, ..)| p == prefix).expect("RUST_TARGETS prefix in TARGETS");
        rustc_minicore().target(rtriple).panic("abort").output("libminicore.rlib").run();
        rustc()
            .input("complex.rs")
            .target(rtriple)
            .panic("abort")
            .extern_("minicore", "libminicore.rlib")
            .arg("-Cno-prepopulate-passes")
            .arg("-Zcodegen-source-order")
            .emit("llvm-ir")
            .output("complex.ll")
            .run();
        let ir = rfs::read_to_string("complex.ll");
        llvm_filecheck().patterns("complex.rs").check_prefix(prefix).stdin_buf(ir).run();
    }
}
