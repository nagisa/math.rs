use core::num::{Float, Int};

use utils::{AsBits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK};

/// Get exponent of a 32-bit floating-point value.
#[no_mangle]
pub extern fn logbf(i: f32) -> f32 {
    let mut bits = i.as_bits();
    bits &= !F32_SIGN_MASK;
    if bits == 0 {
        return -1.0 / i.abs();
    }  else if bits >= 0x7F80_0000 {
        // Value is either ±∞ or NaN.
        return i + i;
    }
    let mut exp = bits >> 23;
    if exp == 0 {
        // Denormal
        exp -= (bits.leading_zeros() as u32) - 9;
    }
    return exp as f32;
}

/// Get exponent of a 64-bit floating-point value.
#[no_mangle]
pub extern fn logb(i: f64) -> f64 {
    let mut bits = i.as_bits();
    bits &= !F64_SIGN_MASK;
    if bits == 0 {
        return -1.0 / i.abs();
    }  else if bits >= 0x7FF0_0000_0000_0000 {
        // Value is either ±∞ or NaN.
        return i + i;
    }
    let mut exp = bits >> 52;
    if exp == 0 {
        exp -= (bits.leading_zeros() as u64) - 12;
    }
    return exp as f64;
}
