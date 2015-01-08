use core::intrinsics::transmute;

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
        ((((self as u32) & F32_EXP_MASK) >> 23) as i32) - F32_MAX_EXP
    }
}

impl Bits<f64> for u64 {
    #[inline(always)]
    fn from_bits(&self) -> f64 {
        unsafe { transmute(*self) }
    }

    #[inline(always)]
    fn get_exponent(self) -> i32 {
        ((((self as u64) & F64_EXP_MASK) >> 52) as i32) - F64_MAX_EXP
    }
}

// Common constants
pub const F64_SIGN_MASK : u64 = 1 << 63;
pub const F32_SIGN_MASK : u32 = 1 << 31;
pub const F32_MANTISSA_MASK : u32 = 0x007F_FFFF;
pub const F64_MANTISSA_MASK : u64 = 0x000F_FFFF_FFFF_FFFF;
pub const F32_EXP_MASK : u32 = 0x7F80_0000;
pub const F64_EXP_MASK : u64 = 0x7FF0_0000_0000_0000;

pub const F32_MAX_EXP : i32 = 127;
pub const F32_MIN_EXP : i32 = -149;
pub const F32_DENORMAL_EXP : i32 = -127;
pub const F32_NAN_EXP : i32 = 128;

pub const F64_MAX_EXP : i32 = 1023;
pub const F64_MIN_EXP : i32 = -1076;
pub const F64_DENORMAL_EXP : i32 = -1023;
pub const F64_NAN_EXP : i32 = 1024;
