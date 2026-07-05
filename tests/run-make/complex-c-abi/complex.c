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
// Regenerate with the generator in the PR description; validate per target with:
//   clang --target=<triple> <flags> -O2 -S -emit-llvm complex.c | FileCheck complex.c --check-prefix=<REV>

#ifdef __FLT16_MANT_DIG__
// X86_64:         define{{.*}} <2 x half> @cplx_f16(<2 x half>{{.*}}
// I686:           define{{.*}} <2 x half> @cplx_f16(ptr{{.*}}byval({ half, half }){{.*}}
// WINDOWS_MSVC:   define{{.*}} i32 @cplx_f16(i32{{.*}}
// WINDOWS_GNU:    define{{.*}} i32 @cplx_f16(i32{{.*}}
// AARCH64:        define{{.*}} { half, half } @cplx_f16([2 x half]{{.*}}
// AARCH64_DARWIN: define{{.*}} { half, half } @cplx_f16([2 x half]{{.*}}
// ARM:            define{{.*}} i32 @cplx_f16([1 x i32]{{.*}}
// RISCV64:        define{{.*}} { half, half } @cplx_f16(half{{.*}}half{{.*}}
// RISCV32:        define{{.*}} { half, half } @cplx_f16(half{{.*}}half{{.*}}
// S390X:          define{{.*}} void @cplx_f16(ptr{{.*}}sret({ half, half }){{.*}}ptr{{.*}}
_Complex _Float16 cplx_f16(_Complex _Float16 x) { return x; }
#endif

// X86_64:         define{{.*}} <2 x float> @cplx_f32(<2 x float>{{.*}}
// I686:           define{{.*}} i64 @cplx_f32(ptr{{.*}}byval({ float, float }){{.*}}
// WINDOWS_MSVC:   define{{.*}} i64 @cplx_f32(i64{{.*}}
// WINDOWS_GNU:    define{{.*}} i64 @cplx_f32(i64{{.*}}
// AARCH64:        define{{.*}} { float, float } @cplx_f32([2 x float]{{.*}}
// AARCH64_DARWIN: define{{.*}} { float, float } @cplx_f32([2 x float]{{.*}}
// ARM:            define{{.*}} { float, float } @cplx_f32({ float, float }{{.*}}
// RISCV64:        define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// RISCV32:        define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// S390X:          define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}
// POWERPC64LE:    define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// POWERPC:        define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}byval({ float, float }){{.*}}
// SPARC64:        define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// MIPS64EL:       define{{.*}} { float, float } @cplx_f32(float{{.*}}float{{.*}}
// MIPS:           define{{.*}} { float, float } @cplx_f32(i32{{.*}}i32{{.*}}
// WASM32:         define{{.*}} void @cplx_f32(ptr{{.*}}sret({ float, float }){{.*}}ptr{{.*}}byval({ float, float }){{.*}}
_Complex float cplx_f32(_Complex float x) { return x; }

// X86_64:         define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// I686:           define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// WINDOWS_MSVC:   define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}
// AARCH64:        define{{.*}} { double, double } @cplx_f64([2 x double]{{.*}}
// AARCH64_DARWIN: define{{.*}} { double, double } @cplx_f64([2 x double]{{.*}}
// ARM:            define{{.*}} { double, double } @cplx_f64({ double, double }{{.*}}
// RISCV64:        define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// RISCV32:        define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// S390X:          define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}
// POWERPC64LE:    define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// POWERPC:        define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
// SPARC64:        define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// MIPS64EL:       define{{.*}} { double, double } @cplx_f64(double{{.*}}double{{.*}}
// MIPS:           define{{.*}} { double, double } @cplx_f64(i32{{.*}}i32{{.*}}i32{{.*}}i32{{.*}}
// WASM32:         define{{.*}} void @cplx_f64(ptr{{.*}}sret({ double, double }){{.*}}ptr{{.*}}byval({ double, double }){{.*}}
_Complex double cplx_f64(_Complex double x) { return x; }

#if defined(__FLOAT128__) || defined(__SIZEOF_FLOAT128__)
// X86_64:         define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
// I686:           define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}
// WASM32:         define{{.*}} void @cplx_f128(ptr{{.*}}sret({ fp128, fp128 }){{.*}}ptr{{.*}}byval({ fp128, fp128 }){{.*}}
_Complex __float128 cplx_f128(_Complex __float128 x) { return x; }
#endif

// X86_64:         define{{.*}} i16 @cplx_i8(i16{{.*}}
// I686:           define{{.*}} i16 @cplx_i8(ptr{{.*}}byval({ i8, i8 }){{.*}}
// WINDOWS_MSVC:   define{{.*}} i16 @cplx_i8(i16{{.*}}
// WINDOWS_GNU:    define{{.*}} i16 @cplx_i8(i16{{.*}}
// AARCH64:        define{{.*}} i16 @cplx_i8(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i16 @cplx_i8(i64{{.*}}
// ARM:            define{{.*}} i16 @cplx_i8([1 x i32]{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// RISCV32:        define{{.*}} i32 @cplx_i8(i32{{.*}}
// S390X:          define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}
// POWERPC64LE:    define{{.*}} { i8, i8 } @cplx_i8(i8{{.*}}i8{{.*}}
// POWERPC:        define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}byval({ i8, i8 }){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i8(i64{{.*}}
// MIPS64EL:       define{{.*}} { i8, i8 } @cplx_i8(i16{{.*}}
// MIPS:           define{{.*}} { i8, i8 } @cplx_i8(i16{{.*}}
// WASM32:         define{{.*}} void @cplx_i8(ptr{{.*}}sret({ i8, i8 }){{.*}}ptr{{.*}}byval({ i8, i8 }){{.*}}
_Complex __INT8_TYPE__ cplx_i8(_Complex __INT8_TYPE__ x) { return x; }

// X86_64:         define{{.*}} i32 @cplx_i16(i32{{.*}}
// I686:           define{{.*}} i32 @cplx_i16(ptr{{.*}}byval({ i16, i16 }){{.*}}
// WINDOWS_MSVC:   define{{.*}} i32 @cplx_i16(i32{{.*}}
// WINDOWS_GNU:    define{{.*}} i32 @cplx_i16(i32{{.*}}
// AARCH64:        define{{.*}} i32 @cplx_i16(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i32 @cplx_i16(i64{{.*}}
// ARM:            define{{.*}} i32 @cplx_i16([1 x i32]{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// RISCV32:        define{{.*}} i32 @cplx_i16(i32{{.*}}
// S390X:          define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}
// POWERPC64LE:    define{{.*}} { i16, i16 } @cplx_i16(i16{{.*}}i16{{.*}}
// POWERPC:        define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}byval({ i16, i16 }){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i16(i64{{.*}}
// MIPS64EL:       define{{.*}} { i16, i16 } @cplx_i16(i32{{.*}}
// MIPS:           define{{.*}} { i16, i16 } @cplx_i16(i32{{.*}}
// WASM32:         define{{.*}} void @cplx_i16(ptr{{.*}}sret({ i16, i16 }){{.*}}ptr{{.*}}byval({ i16, i16 }){{.*}}
_Complex __INT16_TYPE__ cplx_i16(_Complex __INT16_TYPE__ x) { return x; }

// X86_64:         define{{.*}} i64 @cplx_i32(i64{{.*}}
// I686:           define{{.*}} i64 @cplx_i32(ptr{{.*}}byval({ i32, i32 }){{.*}}
// WINDOWS_MSVC:   define{{.*}} i64 @cplx_i32(i64{{.*}}
// WINDOWS_GNU:    define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// AARCH64_DARWIN: define{{.*}} i64 @cplx_i32(i64{{.*}}
// ARM:            define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}[2 x i32]{{.*}}
// RISCV64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// RISCV32:        define{{.*}} [2 x i32] @cplx_i32([2 x i32]{{.*}}
// S390X:          define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}
// POWERPC64LE:    define{{.*}} { i32, i32 } @cplx_i32(i32{{.*}}i32{{.*}}
// POWERPC:        define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}byval({ i32, i32 }){{.*}}
// SPARC64:        define{{.*}} i64 @cplx_i32(i64{{.*}}
// MIPS64EL:       define{{.*}} { i32, i32 } @cplx_i32(i64{{.*}}
// MIPS:           define{{.*}} { i32, i32 } @cplx_i32(i32{{.*}}i32{{.*}}
// WASM32:         define{{.*}} void @cplx_i32(ptr{{.*}}sret({ i32, i32 }){{.*}}ptr{{.*}}byval({ i32, i32 }){{.*}}
_Complex __INT32_TYPE__ cplx_i32(_Complex __INT32_TYPE__ x) { return x; }

// X86_64:         define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// I686:           define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// WINDOWS_MSVC:   define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// WINDOWS_GNU:    define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// AARCH64:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// AARCH64_DARWIN: define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// ARM:            define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}[2 x i64]{{.*}}
// RISCV64:        define{{.*}} [2 x i64] @cplx_i64([2 x i64]{{.*}}
// RISCV32:        define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// S390X:          define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}
// POWERPC64LE:    define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// POWERPC:        define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
// SPARC64:        define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// MIPS64EL:       define{{.*}} { i64, i64 } @cplx_i64(i64{{.*}}i64{{.*}}
// MIPS:           define{{.*}} { i64, i64 } @cplx_i64(i32{{.*}}i32{{.*}}i32{{.*}}i32{{.*}}
// WASM32:         define{{.*}} void @cplx_i64(ptr{{.*}}sret({ i64, i64 }){{.*}}ptr{{.*}}byval({ i64, i64 }){{.*}}
_Complex __INT64_TYPE__ cplx_i64(_Complex __INT64_TYPE__ x) { return x; }
