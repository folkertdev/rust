// Reference C ABI for `_Complex T`, captured from clang's LLVM IR.
//
// Each function is the identity over a `_Complex` type, so its `define` signature
// records exactly how clang passes and returns that complex type on each target.
// These are the signatures that Rust's `#[repr(complex)]` `core::num::Complex<T>`
// must reproduce in `extern "C"` functions.
//
// `_Complex __int128` is intentionally absent: clang rejects it as invalid C, so
// there is no reference ABI to match. `_Float16`/`__float128` complex only exist on
// the targets whose clang defines `__FLT16_MANT_DIG__`/`__FLOAT128__`.
//
// Each target is checked with the clang invocation the run-make driver uses:
// RUN: %clang --target=x86_64-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=X86_64
// RUN: %clang --target=i686-unknown-linux-gnu -msse2 -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=I686
// RUN: %clang --target=x86_64-pc-windows-msvc -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=WINDOWS_MSVC
// RUN: %clang --target=x86_64-pc-windows-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=WINDOWS_GNU
// RUN: %clang --target=aarch64-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=AARCH64
// RUN: %clang --target=aarch64-apple-darwin -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=AARCH64_DARWIN
// RUN: %clang --target=aarch64-pc-windows-msvc -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=AARCH64_MSVC
// RUN: %clang --target=arm64ec-pc-windows-msvc -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=ARM64EC
// RUN: %clang --target=arm-unknown-linux-gnueabihf -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=ARM
// RUN: %clang --target=riscv64-unknown-linux-gnu -march=rv64gc -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=RISCV64
// RUN: %clang --target=riscv32-unknown-linux-gnu -march=rv32gc -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=RISCV32
// RUN: %clang --target=s390x-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=S390X
// RUN: %clang --target=powerpc64le-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=POWERPC64LE
// RUN: %clang --target=powerpc64-unknown-linux-gnu -mabi=elfv1 -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=POWERPC64
// RUN: %clang --target=powerpc64-ibm-aix -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=AIX
// RUN: %clang --target=powerpc-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=POWERPC
// RUN: %clang --target=sparc64-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=SPARC64
// RUN: %clang --target=mips64el-unknown-linux-gnuabi64 -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=MIPS64EL
// RUN: %clang --target=mips-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=MIPS
// RUN: %clang --target=wasm32-unknown-unknown -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=WASM32
// RUN: %clang --target=wasm64-unknown-unknown -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=WASM64
// RUN: %clang --target=loongarch64-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=LOONGARCH64
// RUN: %clang --target=loongarch32-unknown-none -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=LOONGARCH32
// RUN: %clang --target=sparc-unknown-linux-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=SPARC
// RUN: %clang --target=i686-pc-windows-msvc -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=WIN32_MSVC
// RUN: %clang --target=i686-pc-windows-gnu -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=WIN32_GNU
// RUN: %clang --target=nvptx64-nvidia-cuda -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=NVPTX
// RUN: %clang --target=bpfel -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=BPF
// RUN: %clang --target=csky -O2 -S -emit-llvm -o - %s | FileCheck %s --check-prefix=CSKY

#ifdef __FLT16_MANT_DIG__
// AARCH64:        define{{.*}} { half, half } @cplx_f16([2 x half]{{.*}}
// AARCH64_DARWIN: define{{.*}} { half, half } @cplx_f16([2 x half]{{.*}}
// AARCH64_MSVC:   define{{.*}} { half, half } @cplx_f16([2 x half]{{.*}}
// ARM:            define{{.*}} i32 @cplx_f16([1 x i32]{{.*}}
// ARM64EC:        define{{.*}} { half, half } @cplx_f16([2 x half]{{.*}}
// I686:           define{{.*}} <2 x half> @cplx_f16(ptr{{.*}}byval({ half, half }){{.*}}
// LOONGARCH32:    define{{.*}} { half, half } @cplx_f16(half{{.*}}half{{.*}}
// LOONGARCH64:    define{{.*}} { half, half } @cplx_f16(half{{.*}}half{{.*}}
// NVPTX:          define{{.*}} { half, half } @cplx_f16(ptr{{.*}}byval({ half, half }){{.*}}
// RISCV32:        define{{.*}} { half, half } @cplx_f16(half{{.*}}half{{.*}}
// RISCV64:        define{{.*}} { half, half } @cplx_f16(half{{.*}}half{{.*}}
// S390X:          define{{.*}} void @cplx_f16(ptr{{.*}}sret({ half, half }){{.*}}ptr{{.*}}
// WIN32_GNU:      define{{.*}} <2 x half> @cplx_f16(ptr{{.*}}byval({ half, half }){{.*}}
// WIN32_MSVC:     define{{.*}} <2 x half> @cplx_f16(ptr{{.*}}byval({ half, half }){{.*}}
// WINDOWS_GNU:    define{{.*}} i32 @cplx_f16(i32{{.*}}
// WINDOWS_MSVC:   define{{.*}} i32 @cplx_f16(i32{{.*}}
// X86_64:         define{{.*}} <2 x half> @cplx_f16(<2 x half>{{.*}}
_Complex _Float16 cplx_f16(_Complex _Float16 x) { return x; }
#endif

// AARCH64:        define{{.*}} { float, float } @cplx_f32([2 x float]{{.*}}
// AARCH64_DARWIN: define{{.*}} { float, float } @cplx_f32([2 x float]{{.*}}
// AARCH64_MSVC:   define{{.*}} { float, float } @cplx_f32([2 x float]{{.*}}
// AIX:            define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// ARM:            define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// ARM64EC:        define{{.*}} { float, float } @cplx_f32([2 x float]{{.*}}
// BPF:            define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}i64{{.*}}
// CSKY:           define{{.*}} [2 x i32] @cplx_f32([2 x i32]{{.*}}
// I686:           define{{.*}} i64 @cplx_f32(ptr{{.*}}byval({ float, float }){{.*}}
// LOONGARCH32:    define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// LOONGARCH64:    define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// MIPS:           define{{.*}} { float, float } @cplx_f32(i32{{.*}}i32{{.*}}
// MIPS64EL:       define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// NVPTX:          define{{.*}} { float, float } @cplx_f32(ptr{{.*}}byval({ float, float }){{.*}}
// POWERPC:        define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}byval({ float, float }){{.*}}
// POWERPC64:      define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// POWERPC64LE:    define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// RISCV32:        define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// RISCV64:        define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// S390X:          define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { float, float } @cplx_f32(ptr{{.*}}byval({ float, float }){{.*}}
// SPARC64:        define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// WASM32:         define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}byval({ float, float }){{.*}}
// WASM64:         define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}byval({ float, float }){{.*}}
// WIN32_GNU:      define{{.*}} i64 @cplx_f32(ptr{{.*}}byval({ float, float }){{.*}}
// WIN32_MSVC:     define{{.*}} i64 @cplx_f32(ptr{{.*}}byval({ float, float }){{.*}}
// WINDOWS_GNU:    define{{.*}} i64 @cplx_f32(i64{{.*}}
// WINDOWS_MSVC:   define{{.*}} i64 @cplx_f32(i64{{.*}}
// X86_64:         define{{.*}} <2 x float> @cplx_f32(<2 x float>{{.*}}
_Complex float cplx_f32(_Complex float x) { return x; }

// AARCH64:        define{{.*}} { double, double } @cplx_f64([2 x double]{{.*}}
// AARCH64_DARWIN: define{{.*}} { double, double } @cplx_f64([2 x double]{{.*}}
// AARCH64_MSVC:   define{{.*}} { double, double } @cplx_f64([2 x double]{{.*}}
// AIX:            define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// ARM:            define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// ARM64EC:        define{{.*}} { double, double } @cplx_f64([2 x double]{{.*}}
// BPF:            define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}[2 x i64]{{.*}}
// CSKY:           define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}[4 x i32]{{.*}}
// I686:           define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// LOONGARCH32:    define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// LOONGARCH64:    define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// MIPS:           define{{.*}} { double, double } @cplx_f64(i32{{.*}}i32{{.*}}i32{{.*}}i32{{.*}}
// MIPS64EL:       define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// NVPTX:          define{{.*}} { double, double } @cplx_f64(ptr{{.*}}byval({ double, double }){{.*}}
// POWERPC:        define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// POWERPC64:      define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// POWERPC64LE:    define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// RISCV32:        define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// RISCV64:        define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// S390X:          define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { double, double } @cplx_f64(ptr{{.*}}byval({ double, double }){{.*}}
// SPARC64:        define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// WASM32:         define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// WASM64:         define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// WIN32_GNU:      define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// WIN32_MSVC:     define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}
// WINDOWS_MSVC:   define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}
// X86_64:         define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
_Complex double cplx_f64(_Complex double x) { return x; }

#if defined(__FLOAT128__) || defined(__SIZEOF_FLOAT128__)
// I686:           define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
// WASM32:         define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
// WASM64:         define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
// WIN32_GNU:      define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}
// X86_64:         define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
_Complex __float128 cplx_f128(_Complex __float128 x) { return x; }
#endif

// AARCH64:        define{{.*}} i16 @cplx_i8(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i16 @cplx_i8(i64{{.*}}
// AARCH64_MSVC:   define{{.*}} i16 @cplx_i8(i64{{.*}}
// AIX:            define{{.*}} { i8, i8 } @cplx_i8(i8{{.*}}i8{{.*}}
// ARM:            define{{.*}} i16 @cplx_i8([1 x i32]{{.*}}
// ARM64EC:        define{{.*}} i16 @cplx_i8(i64{{.*}}
// BPF:            define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}i16{{.*}}
// CSKY:           define{{.*}} i32 @cplx_i8(i32{{.*}}
// I686:           define{{.*}} i16 @cplx_i8(ptr{{.*}}byval({ i8, i8 }){{.*}}
// LOONGARCH32:    define{{.*}} i32 @cplx_i8(i32{{.*}}
// LOONGARCH64:    define{{.*}} i64 @cplx_i8(i64{{.*}}
// MIPS:           define{{.*}} { i8, i8 } @cplx_i8(i16{{.*}}
// MIPS64EL:       define{{.*}} { i8, i8 } @cplx_i8(i16{{.*}}
// NVPTX:          define{{.*}} { i8, i8 } @cplx_i8(ptr{{.*}}byval({ i8, i8 }){{.*}}
// POWERPC:        define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}byval({ i8, i8 }){{.*}}
// POWERPC64:      define{{.*}} { i8, i8 } @cplx_i8(i8{{.*}}i8{{.*}}
// POWERPC64LE:    define{{.*}} { i8, i8 } @cplx_i8(i8{{.*}}i8{{.*}}
// RISCV32:        define{{.*}} i32 @cplx_i8(i32{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// S390X:          define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i8, i8 } @cplx_i8(ptr{{.*}}byval({ i8, i8 }){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}byval({ i8, i8 }){{.*}}
// WASM64:         define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}byval({ i8, i8 }){{.*}}
// WIN32_GNU:      define{{.*}} i16 @cplx_i8(ptr{{.*}}byval({ i8, i8 }){{.*}}
// WIN32_MSVC:     define{{.*}} i16 @cplx_i8(ptr{{.*}}byval({ i8, i8 }){{.*}}
// WINDOWS_GNU:    define{{.*}} i16 @cplx_i8(i16{{.*}}
// WINDOWS_MSVC:   define{{.*}} i16 @cplx_i8(i16{{.*}}
// X86_64:         define{{.*}} i16 @cplx_i8(i16{{.*}}
_Complex __INT8_TYPE__ cplx_i8(_Complex __INT8_TYPE__ x) { return x; }

// AARCH64:        define{{.*}} i32 @cplx_i16(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i32 @cplx_i16(i64{{.*}}
// AARCH64_MSVC:   define{{.*}} i32 @cplx_i16(i64{{.*}}
// AIX:            define{{.*}} { i16, i16 } @cplx_i16(i16{{.*}}i16{{.*}}
// ARM:            define{{.*}} i32 @cplx_i16([1 x i32]{{.*}}
// ARM64EC:        define{{.*}} i32 @cplx_i16(i64{{.*}}
// BPF:            define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}i32{{.*}}
// CSKY:           define{{.*}} i32 @cplx_i16(i32{{.*}}
// I686:           define{{.*}} i32 @cplx_i16(ptr{{.*}}byval({ i16, i16 }){{.*}}
// LOONGARCH32:    define{{.*}} i32 @cplx_i16(i32{{.*}}
// LOONGARCH64:    define{{.*}} i64 @cplx_i16(i64{{.*}}
// MIPS:           define{{.*}} { i16, i16 } @cplx_i16(i32{{.*}}
// MIPS64EL:       define{{.*}} { i16, i16 } @cplx_i16(i32{{.*}}
// NVPTX:          define{{.*}} { i16, i16 } @cplx_i16(ptr{{.*}}byval({ i16, i16 }){{.*}}
// POWERPC:        define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}byval({ i16, i16 }){{.*}}
// POWERPC64:      define{{.*}} { i16, i16 } @cplx_i16(i16{{.*}}i16{{.*}}
// POWERPC64LE:    define{{.*}} { i16, i16 } @cplx_i16(i16{{.*}}i16{{.*}}
// RISCV32:        define{{.*}} i32 @cplx_i16(i32{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// S390X:          define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i16, i16 } @cplx_i16(ptr{{.*}}byval({ i16, i16 }){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}byval({ i16, i16 }){{.*}}
// WASM64:         define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}byval({ i16, i16 }){{.*}}
// WIN32_GNU:      define{{.*}} i32 @cplx_i16(ptr{{.*}}byval({ i16, i16 }){{.*}}
// WIN32_MSVC:     define{{.*}} i32 @cplx_i16(ptr{{.*}}byval({ i16, i16 }){{.*}}
// WINDOWS_GNU:    define{{.*}} i32 @cplx_i16(i32{{.*}}
// WINDOWS_MSVC:   define{{.*}} i32 @cplx_i16(i32{{.*}}
// X86_64:         define{{.*}} i32 @cplx_i16(i32{{.*}}
_Complex __INT16_TYPE__ cplx_i16(_Complex __INT16_TYPE__ x) { return x; }

// AARCH64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64_MSVC:   define{{.*}} i64 @cplx_i32(i64{{.*}}
// AIX:            define{{.*}} { i32, i32 } @cplx_i32(i32{{.*}}i32{{.*}}
// ARM:            define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}[2 x i32]{{.*}}
// ARM64EC:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// BPF:            define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}i64{{.*}}
// CSKY:           define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// I686:           define{{.*}} i64 @cplx_i32(ptr{{.*}}byval({ i32, i32 }){{.*}}
// LOONGARCH32:    define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// LOONGARCH64:    define{{.*}} i64 @cplx_i32(i64{{.*}}
// MIPS:           define{{.*}} { i32, i32 } @cplx_i32(i32{{.*}}i32{{.*}}
// MIPS64EL:       define{{.*}} { i32, i32 } @cplx_i32(i64{{.*}}
// NVPTX:          define{{.*}} { i32, i32 } @cplx_i32(ptr{{.*}}byval({ i32, i32 }){{.*}}
// POWERPC:        define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}byval({ i32, i32 }){{.*}}
// POWERPC64:      define{{.*}} { i32, i32 } @cplx_i32(i32{{.*}}i32{{.*}}
// POWERPC64LE:    define{{.*}} { i32, i32 } @cplx_i32(i32{{.*}}i32{{.*}}
// RISCV32:        define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// S390X:          define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i32, i32 } @cplx_i32(ptr{{.*}}byval({ i32, i32 }){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}byval({ i32, i32 }){{.*}}
// WASM64:         define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}byval({ i32, i32 }){{.*}}
// WIN32_GNU:      define{{.*}} i64 @cplx_i32(ptr{{.*}}byval({ i32, i32 }){{.*}}
// WIN32_MSVC:     define{{.*}} i64 @cplx_i32(ptr{{.*}}byval({ i32, i32 }){{.*}}
// WINDOWS_GNU:    define{{.*}} i64 @cplx_i32(i64{{.*}}
// WINDOWS_MSVC:   define{{.*}} i64 @cplx_i32(i64{{.*}}
// X86_64:         define{{.*}} i64 @cplx_i32(i64{{.*}}
_Complex __INT32_TYPE__ cplx_i32(_Complex __INT32_TYPE__ x) { return x; }

// AARCH64:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AARCH64_MSVC:   define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AIX:            define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// ARM:            define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}[2 x i64]{{.*}}
// ARM64EC:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// BPF:            define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}[2 x i64]{{.*}}
// CSKY:           define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}[4 x i32]{{.*}}
// I686:           define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// LOONGARCH32:    define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// LOONGARCH64:    define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// MIPS:           define{{.*}} { i64, i64 } @cplx_i64(i32{{.*}}i32{{.*}}i32{{.*}}i32{{.*}}
// MIPS64EL:       define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// NVPTX:          define{{.*}} { i64, i64 } @cplx_i64(ptr{{.*}}byval({ i64, i64 }){{.*}}
// POWERPC:        define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// POWERPC64:      define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// POWERPC64LE:    define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// RISCV32:        define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// RISCV64:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// S390X:          define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// SPARC:          define{{.*}} { i64, i64 } @cplx_i64(ptr{{.*}}byval({ i64, i64 }){{.*}}
// SPARC64:        define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// WASM32:         define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// WASM64:         define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// WIN32_GNU:      define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// WIN32_MSVC:     define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// WINDOWS_MSVC:   define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// X86_64:         define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
_Complex __INT64_TYPE__ cplx_i64(_Complex __INT64_TYPE__ x) { return x; }
