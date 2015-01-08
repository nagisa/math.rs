use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F32_MANTISSA_MASK, F32_NAN_EXP};
use utils::{F64_SIGN_MASK, F64_MANTISSA_MASK, F64_NAN_EXP};

/// Round the 32-bit floating-point number away from zero.
#[no_mangle]
pub extern fn roundf(i : f32) -> f32 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // Exponent of -1 means the number is between (-1; -0.5] ∪ [0.5; 1). Exponents
        // less than -1 make number be somewhere between (-0.5; 0.5).
        // We clear the whole number except its sign, making the number ±0.0.
        bits &= F32_SIGN_MASK;
        if exp == -1 {
            // Set its magnitude to 1
            bits |= 0x3F80_0000;
        }
    } else if exp == F32_NAN_EXP {
        // Value is either ±∞ or NaN.
        return i + i;
    } else if exp >= 23 {
        // Mantissa cannot store more than 23 bits and the number with exponents above that are
        // already integral.
        return i;
    } else {
        // The most complex case. First we check whether the number is already integral by checking
        // how much digits the mantissa holds.
        let mask = F32_MANTISSA_MASK >> (exp as uint);
        if bits & mask == 0 {
            return i;
        }
        // Otherwise armed with a smart selection of constants and operations we squeeze the
        // mantissa to represent the closest integral.
        bits += 0x0040_0000 >> (exp as uint);
        bits &= !mask;
    }
    bits.from_bits()
}

/// Round the 64-bit floating-point number away from zero.
#[no_mangle]
pub extern fn round(i : f64) -> f64 {
    // See roundf for implementation details and commentary. The code structure does not differ,
    // only the relevant constants do.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    if exp < 0 {
        bits &= F64_SIGN_MASK;
        if exp == -1 {
            bits |= 0x3FF0_0000_0000_0000;
        }
    } else if exp == F64_NAN_EXP {
        return i + i;
    } else if exp >= 52 {
        return i;
    } else {
        let mask = F64_MANTISSA_MASK >> (exp as uint);
        if bits & mask == 0 {
            return i;
        }
        bits += 0x0008_0000_0000_0000 >> (exp as uint);
        bits &= !mask;
    }
    bits.from_bits()
}
