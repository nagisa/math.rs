use core::intrinsics::transmute;
use core::{f32, f64};
use sqrt;
use abs;

// Define convenience wrappers converting to and from our working representation, which is bits.
pub trait AsBits<Output> {
    /// Converts from float representation to `bits` representation.
    fn as_bits(&self) -> Output;
}

pub trait Bits<Output> {
    /// Converts from `bits` representation back to float representation.
    fn from_bits(&self) -> Output;
    /// Extracts the exponent from `bits` representation.
    fn get_exponent(self) -> i32;
    /// Chekcs whether represents ±0 float
    fn is_zero(self) -> bool;
}

// Similar to unstable `core::num::Float`, needed due the lack of `std`
pub trait Float: Sized + PartialEq {
    /// Returns `true` if the number is NaN.
    #[inline(always)]
    fn is_nan(&self) -> bool {
        self != self
    }

    /// Returns `true` if the number is neither infinite or NaN.
    #[inline(always)]
    fn is_finite(self) -> bool {
        !(self.is_nan() || self.is_infinite())
    }

    /// Returns `true` if the number is infinite.
    fn is_infinite(self) -> bool;

    /// Take the reciprocal (inverse) of a number, `1/x`.
    fn recip(self) -> Self;

    /// Calculate a square root.
    fn sqrt(self) -> Self;

    /// Take the absolute value of a number.
    fn abs(self) -> Self;
}

impl AsBits<u32> for f32 {
    #[inline(always)]
    fn as_bits(&self) -> u32 {
        unsafe { transmute(*self) }
    }
}

impl AsBits<u64> for f64 {
    #[inline(always)]
    fn as_bits(&self) -> u64 {
        unsafe { transmute(*self) }
    }
}

impl Bits<f32> for u32 {
    #[inline(always)]
    fn from_bits(&self) -> f32 {
        unsafe { transmute(*self) }
    }

    #[inline(always)]
    fn get_exponent(self) -> i32 {
        (((self & F32_EXP_MASK) >> 23) as i32) - F32_MAX_EXP
    }

    #[inline(always)]
    fn is_zero(self) -> bool {
        self & !F32_SIGN_MASK == 0
    }
}

impl Bits<f64> for u64 {
    #[inline(always)]
    fn from_bits(&self) -> f64 {
        unsafe { transmute(*self) }
    }

    #[inline(always)]
    fn get_exponent(self) -> i32 {
        (((self & F64_EXP_MASK) >> 52) as i32) - F64_MAX_EXP
    }

    #[inline(always)]
    fn is_zero(self) -> bool {
        self & !F64_SIGN_MASK == 0
    }
}

impl Float for f32 {
    /// Returns `true` if the number is infinite.
    #[inline(always)]
    fn is_infinite(self) -> bool {
        self == f32::INFINITY || self == f32::NEG_INFINITY
    }

    /// Take the reciprocal (inverse) of a number, `1/x`.
    #[inline(always)]
    fn recip(self) -> Self {
        1.0f32 / self
    }

    /// Calculate a square root.
    #[inline(always)]
    fn sqrt(self) -> Self {
        sqrt::sqrtf(self)
    }

    /// Take the absolute value of a number.
    #[inline(always)]
    fn abs(self) -> Self {
        abs::fabsf(self)
    }
}

impl Float for f64 {
    /// Returns `true` if the number is infinite.
    #[inline(always)]
    fn is_infinite(self) -> bool {
        self == f64::INFINITY || self == f64::NEG_INFINITY
    }

    /// Take the reciprocal (inverse) of a number, `1/x`.
    #[inline(always)]
    fn recip(self) -> Self {
        1.0f64 / self
    }

    /// Calculate a square root.
    #[inline(always)]
    fn sqrt(self) -> Self {
        sqrt::sqrt(self)
    }

    /// Take the absolute value of a number.
    #[inline(always)]
    fn abs(self) -> Self {
        abs::fabs(self)
    }
}

// Common constants
pub const F64_SIGN_MASK: u64 = 1 << 63;
pub const F32_SIGN_MASK: u32 = 1 << 31;
pub const F32_MANTISSA_MASK: u32 = 0x007F_FFFF;
pub const F64_MANTISSA_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
pub const F32_EXP_MASK: u32 = 0x7F80_0000;
pub const F64_EXP_MASK: u64 = 0x7FF0_0000_0000_0000;

pub const F32_MAX_EXP: i32 = 127;
pub const F32_MIN_EXP: i32 = -149;
pub const F32_DENORMAL_EXP: i32 = -127;
pub const F32_NAN_EXP: i32 = 128;

pub const F64_MAX_EXP: i32 = 1023;
pub const F64_MIN_EXP: i32 = -1076;
pub const F64_DENORMAL_EXP: i32 = -1023;
pub const F64_NAN_EXP: i32 = 1024;

pub const FRAC_3PI_4: f64 = 2.3561944901923449288469825374596271631478770495313293;
pub const FRAC_5PI_4: f64 = 3.9269908169872415480783042290993786052464617492188822;
pub const FRAC_3PI_2: f64 = 4.7123889803846898576939650749192543262957540990626587;
pub const FRAC_7PI_4: f64 = 5.4977871437821381673096259207391300473450464489064351;
pub const PI_2: f64 = 6.2831853071795864769252867665590057683943387987502116;
