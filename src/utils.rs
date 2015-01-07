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
    fn get_exponent(&self) -> i32;
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
    fn get_exponent(&self) -> i32 {
        (((*self as i32) >> 23) & 0xff) - 0x7f
    }
}

impl Bits<f64> for u64 {
    #[inline(always)]
    fn from_bits(&self) -> f64 {
        unsafe { transmute(*self) }
    }

    #[inline(always)]
    fn get_exponent(&self) -> i32 {
        ((((*self as i64) >> 52) as i32) & 0x7ff) - 0x3ff
    }
}



