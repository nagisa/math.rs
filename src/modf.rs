use core::num::Float;

use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F32_MANTISSA_MASK};
use utils::{F64_SIGN_MASK, F64_MANTISSA_MASK};

/// Extract signed integral and fractional values of of 32-bit floating-point number.
///
/// Each part has the same sign as the input. Integral part is stored in the location pointed to by
/// `o` and fractional part is returned.
#[no_mangle]
pub extern fn modff(i: f32, o: *mut f32) -> f32 {
    // Unsafes booyah!
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // No integral part. Set ±0 for integral.
        unsafe { *o = (bits & F32_SIGN_MASK).from_bits(); }
        return i;
    } else if exp >= 23 {
        // No fraction, NaN or ±∞.
        unsafe { *o = i * 1.0; }
        if i.is_nan() {
            return i + i;
        }
        bits &= F32_SIGN_MASK;
    } else {
        let mask = F32_MANTISSA_MASK >> (exp as uint);
        if bits & mask == 0 {
            // Input is integral
            unsafe { *o = i; }
            bits &= F32_SIGN_MASK;
        } else {
            let frac = (bits & !mask).from_bits();
            unsafe { *o = frac; }
            return i - frac;
        }
    }
    bits.from_bits()
}

/// Extract signed integral and fractional values of of 64-bit floating-point number.
///
/// Each part has the same sign as the input. Integral part is stored in the location pointed to by
/// `o` and fractional part is returned.
#[no_mangle]
pub extern fn modf(i: f64, o: *mut f64) -> f64 {
    // Unsafes booyah!
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // No integral part. Set ±0 for integral.
        unsafe { *o = (bits & F64_SIGN_MASK).from_bits(); }
        return i;
    } else if exp >= 52 {
        // No fraction, NaN or ±∞.
        unsafe { *o = i * 1.0; }
        if i.is_nan() {
            return i + i;
        }
        bits &= F64_SIGN_MASK;
    } else {
        let mask = F64_MANTISSA_MASK >> (exp as uint);
        if bits & mask == 0 {
            // Input is integral
            unsafe { *o = i; }
            bits &= F64_SIGN_MASK;
        } else {
            let frac = (bits & !mask).from_bits();
            unsafe { *o = frac; }
            return i - frac;
        }
    }
    bits.from_bits()
}
