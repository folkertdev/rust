use crate::fmt::{Debug, Display, Formatter, LowerExp, Result, UpperExp};
use crate::mem::MaybeUninit;
use crate::num::imp::{flt2dec, fmt as numfmt};

#[doc(hidden)]
trait GeneralFormat: PartialOrd {
    /// Determines if a value should use exponential based on its magnitude, given the precondition
    /// that it will not be rounded any further before it is displayed.
    fn already_rounded_value_should_use_exponential(&self) -> bool;
}

macro_rules! impl_general_format {
    ($($t:ident)*) => {
        $(impl GeneralFormat for $t {
            fn already_rounded_value_should_use_exponential(&self) -> bool {
                // `max_abs` rounds to infinity for `f16`. This is fine to save us from a more
                // complex macro, it just means a positive-exponent `f16` will never print as
                // scientific notation by default (reasonably, the max is 65504.0).
                #[allow(overflowing_literals)]
                let max_abs = 1e+16;

                let abs = $t::abs(*self);
                (abs != 0.0 && abs < 1e-4) || abs >= max_abs
            }
        })*
    }
}

#[cfg(target_has_reliable_f16)]
impl_general_format! { f16 }
impl_general_format! { f32 f64 }

// Don't inline this so callers don't use the stack space this function
// requires unless they have to.
#[inline(never)]
fn float_to_decimal_common_exact<T>(
    fmt: &mut Formatter<'_>,
    num: &T,
    sign: flt2dec::Sign,
    precision: u16,
) -> Result
where
    T: flt2dec::DecodableFloat,
{
    let mut buf: [MaybeUninit<u8>; 1024] = [MaybeUninit::uninit(); 1024]; // enough for f32 and f64
    let mut parts: [MaybeUninit<numfmt::Part<'_>>; 4] = [MaybeUninit::uninit(); 4];
    let formatted = flt2dec::to_exact_fixed_str(
        flt2dec::strategy::grisu::format_exact,
        *num,
        sign,
        precision.into(),
        &mut buf,
        &mut parts,
    );
    // SAFETY: `to_exact_fixed_str` and `format_exact` produce only ASCII characters.
    unsafe { fmt.pad_formatted_parts(&formatted) }
}

// Don't inline this so callers that call both this and the above won't wind
// up using the combined stack space of both functions in some cases.
#[inline(never)]
fn float_to_decimal_common_shortest<T>(
    fmt: &mut Formatter<'_>,
    num: &T,
    sign: flt2dec::Sign,
    precision: u16,
) -> Result
where
    T: flt2dec::DecodableFloat,
{
    // enough for f32 and f64
    let mut buf: [MaybeUninit<u8>; flt2dec::MAX_SIG_DIGITS] =
        [MaybeUninit::uninit(); flt2dec::MAX_SIG_DIGITS];
    let mut parts: [MaybeUninit<numfmt::Part<'_>>; 4] = [MaybeUninit::uninit(); 4];
    let formatted = flt2dec::to_shortest_str(
        flt2dec::strategy::grisu::format_shortest,
        *num,
        sign,
        precision.into(),
        &mut buf,
        &mut parts,
    );
    // SAFETY: `to_shortest_str` and `format_shortest` produce only ASCII characters.
    unsafe { fmt.pad_formatted_parts(&formatted) }
}

fn float_to_decimal_display<T>(fmt: &mut Formatter<'_>, num: &T) -> Result
where
    T: flt2dec::DecodableFloat,
{
    let force_sign = fmt.sign_plus();
    let sign = match force_sign {
        false => flt2dec::Sign::Minus,
        true => flt2dec::Sign::MinusPlus,
    };

    if let Some(precision) = fmt.options.get_precision() {
        float_to_decimal_common_exact(fmt, num, sign, precision)
    } else {
        let min_precision = 0;
        float_to_decimal_common_shortest(fmt, num, sign, min_precision)
    }
}

// Don't inline this so callers don't use the stack space this function
// requires unless they have to.
#[inline(never)]
fn float_to_exponential_common_exact<T>(
    fmt: &mut Formatter<'_>,
    num: &T,
    sign: flt2dec::Sign,
    precision: u16,
    upper: bool,
) -> Result
where
    T: flt2dec::DecodableFloat,
{
    let mut buf: [MaybeUninit<u8>; 1024] = [MaybeUninit::uninit(); 1024]; // enough for f32 and f64
    let mut parts: [MaybeUninit<numfmt::Part<'_>>; 6] = [MaybeUninit::uninit(); 6];
    let formatted = flt2dec::to_exact_exp_str(
        flt2dec::strategy::grisu::format_exact,
        *num,
        sign,
        precision.into(),
        upper,
        &mut buf,
        &mut parts,
    );
    // SAFETY: `to_exact_exp_str` and `format_exact` produce only ASCII characters.
    unsafe { fmt.pad_formatted_parts(&formatted) }
}

// Don't inline this so callers that call both this and the above won't wind
// up using the combined stack space of both functions in some cases.
#[inline(never)]
fn float_to_exponential_common_shortest<T>(
    fmt: &mut Formatter<'_>,
    num: &T,
    sign: flt2dec::Sign,
    upper: bool,
) -> Result
where
    T: flt2dec::DecodableFloat,
{
    // enough for f32 and f64
    let mut buf: [MaybeUninit<u8>; flt2dec::MAX_SIG_DIGITS] =
        [MaybeUninit::uninit(); flt2dec::MAX_SIG_DIGITS];
    let mut parts: [MaybeUninit<numfmt::Part<'_>>; 6] = [MaybeUninit::uninit(); 6];
    let formatted = flt2dec::to_shortest_exp_str(
        flt2dec::strategy::grisu::format_shortest,
        *num,
        sign,
        (0, 0),
        upper,
        &mut buf,
        &mut parts,
    );
    // SAFETY: `to_shortest_exp_str` and `format_shortest` produce only ASCII characters.
    unsafe { fmt.pad_formatted_parts(&formatted) }
}

// Common code of floating point LowerExp and UpperExp.
fn float_to_exponential_common<T>(fmt: &mut Formatter<'_>, num: &T, upper: bool) -> Result
where
    T: flt2dec::DecodableFloat,
{
    let force_sign = fmt.sign_plus();
    let sign = match force_sign {
        false => flt2dec::Sign::Minus,
        true => flt2dec::Sign::MinusPlus,
    };

    if let Some(precision) = fmt.options.get_precision() {
        float_to_exponential_common_exact(fmt, num, sign, precision, upper)
    } else {
        float_to_exponential_common_shortest(fmt, num, sign, upper)
    }
}

fn float_to_general_debug<T>(fmt: &mut Formatter<'_>, num: &T) -> Result
where
    T: flt2dec::DecodableFloat + GeneralFormat,
{
    let force_sign = fmt.sign_plus();
    let sign = match force_sign {
        false => flt2dec::Sign::Minus,
        true => flt2dec::Sign::MinusPlus,
    };

    if let Some(precision) = fmt.options.get_precision() {
        // this behavior of {:.PREC?} predates exponential formatting for {:?}
        float_to_decimal_common_exact(fmt, num, sign, precision)
    } else {
        // since there is no precision, there will be no rounding
        if num.already_rounded_value_should_use_exponential() {
            let upper = false;
            float_to_exponential_common_shortest(fmt, num, sign, upper)
        } else {
            let min_precision = 1;
            float_to_decimal_common_shortest(fmt, num, sign, min_precision)
        }
    }
}

macro_rules! floating {
    ($($ty:ident)*) => {
        $(
            #[stable(feature = "rust1", since = "1.0.0")]
            impl Debug for $ty {
                fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                    float_to_general_debug(fmt, self)
                }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl Display for $ty {
                fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                    float_to_decimal_display(fmt, self)
                }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl LowerExp for $ty {
                fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                    float_to_exponential_common(fmt, self, false)
                }
            }

            #[stable(feature = "rust1", since = "1.0.0")]
            impl UpperExp for $ty {
                fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                    float_to_exponential_common(fmt, self, true)
                }
            }
        )*
    };
}

floating! { f32 f64 }

#[cfg(target_has_reliable_f16)]
floating! { f16 }

// FIXME(f16): A fallback is used when the backend+target does not support f16 well, in order
// to avoid ICEs.

#[cfg(not(target_has_reliable_f16))]
#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for f16 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:#06x}", self.to_bits())
    }
}

#[cfg(not(target_has_reliable_f16))]
#[stable(feature = "rust1", since = "1.0.0")]
impl Display for f16 {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, fmt)
    }
}

#[cfg(not(target_has_reliable_f16))]
#[stable(feature = "rust1", since = "1.0.0")]
impl LowerExp for f16 {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, fmt)
    }
}

#[cfg(not(target_has_reliable_f16))]
#[stable(feature = "rust1", since = "1.0.0")]
impl UpperExp for f16 {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, fmt)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for f128 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:#034x}", self.to_bits())
    }
}

/// Formats the x87 80-bit extended float (`core::arch::{x86,x86_64}::f80`) into its shortest
/// decimal form, given its 80-bit little-endian bit pattern in the low bits of `bits`.
///
/// `f80` cannot implement [`flt2dec::DecodableFloat`] — the generic decoder assumes an implicit
/// integer bit, while x87 stores it explicitly — so it decodes itself here. It also can't use the
/// grisu fast path (whose mantissa must stay below `2^61`), so it always uses the arbitrary
/// precision dragon strategy. This is reachable because `flt2dec`'s mantissa was widened to `u128`.
pub(crate) fn f80_to_shortest_str(fmt: &mut Formatter<'_>, bits: u128) -> Result {
    let sign = match fmt.sign_plus() {
        false => flt2dec::Sign::Minus,
        true => flt2dec::Sign::MinusPlus,
    };
    let (negative, full_decoded) = decode_f80(bits);
    // `f80` carries up to 21 significant decimal digits, more than `MAX_SIG_DIGITS` (sized for f64).
    let mut buf = [MaybeUninit::<u8>::uninit(); 24];
    let mut parts = [MaybeUninit::<numfmt::Part<'_>>::uninit(); 4];
    let formatted = flt2dec::to_shortest_str_decoded(
        flt2dec::strategy::dragon::format_shortest,
        negative,
        full_decoded,
        sign,
        0,
        &mut buf,
        &mut parts,
    );
    // SAFETY: `to_shortest_str_decoded` and `format_shortest` produce only ASCII characters.
    unsafe { fmt.pad_formatted_parts(&formatted) }
}

/// Decodes an `f80` from its 80-bit (little-endian) representation into a sign and `FullDecoded`,
/// mirroring [`flt2dec::decode`] but for the x87 layout (explicit integer bit, 15-bit exponent).
fn decode_f80(bits: u128) -> (bool, flt2dec::FullDecoded) {
    use flt2dec::{Decoded, FullDecoded};

    const EXP_BIAS: i16 = 16383;

    let negative = (bits >> 79) & 1 == 1;
    let exp_field = ((bits >> 64) & 0x7fff) as i16;
    // The full 64-bit significand, including the explicit integer (most significant) bit.
    let significand = (bits & 0xffff_ffff_ffff_ffff) as u64;
    let even = significand & 1 == 0;

    let full = if exp_field == 0x7fff {
        // An integer bit set over a zero fraction is infinity; everything else is a NaN.
        if significand == 0x8000_0000_0000_0000 { FullDecoded::Infinite } else { FullDecoded::Nan }
    } else if exp_field == 0 {
        if significand == 0 {
            FullDecoded::Zero
        } else {
            // Subnormal: value = significand * 2^(1 - EXP_BIAS - 63). The `<< 1` lets the half-ULP
            // error bounds be expressed as `minus`/`plus` of 1.
            FullDecoded::Finite(Decoded {
                mant: (significand as u128) << 1,
                minus: 1,
                plus: 1,
                exp: (1 - EXP_BIAS - 63) - 1,
                inclusive: even,
            })
        }
    } else {
        // Normal: value = significand * 2^(exp_field - EXP_BIAS - 63).
        let exp = exp_field - EXP_BIAS - 63;
        if significand == 0x8000_0000_0000_0000 {
            // A power of two: the gap to the previous value is half the gap to the next one.
            FullDecoded::Finite(Decoded {
                mant: (significand as u128) << 2,
                minus: 1,
                plus: 2,
                exp: exp - 2,
                inclusive: even,
            })
        } else {
            FullDecoded::Finite(Decoded {
                mant: (significand as u128) << 1,
                minus: 1,
                plus: 1,
                exp: exp - 1,
                inclusive: even,
            })
        }
    };

    (negative, full)
}
