use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK};

use utils::Float;

/// Get exponent of a 32-bit floating-point value.
#[no_mangle]
#[inline]
pub extern "C" fn logbf(i: f32) -> f32 {
    let mut bits = i.as_bits();
    bits &= !F32_SIGN_MASK;
    if bits == 0 {
        return -1.0 / i.abs();
    } else if bits >= 0x7F80_0000 {
        // Value is either ±∞ or NaN.
        return i + i;
    }

    let mut exp = bits.get_exponent();
    if exp == -0x7F {
        // Denormal
        exp -= (bits.leading_zeros() as i32) - 9;
    }
    return exp as f32;
}

/// Get exponent of a 64-bit floating-point value.
#[no_mangle]
#[inline]
pub extern "C" fn logb(i: f64) -> f64 {
    let mut bits = i.as_bits();
    bits &= !F64_SIGN_MASK;
    if bits == 0 {
        return -1.0 / i.abs();
    } else if bits >= 0x7FF0_0000_0000_0000 {
        // Value is either ±∞ or NaN.
        return i + i;
    }
    let mut exp = bits.get_exponent();
    if exp == -0x3FF {
        // Denormal
        exp -= (bits.leading_zeros() as i32) - 12;
    }
    return exp as f64;
}
