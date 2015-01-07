use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK, F32_MANTISSA_MASK, F64_MANTISSA_MASK};

/// Round the 32-bit floating-point number towards -∞ (negative infinity).
#[no_mangle]
pub extern fn floorf(i : f32) -> f32 {
    // Implementation is similar to roundf. Same commentary applies.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        if bits > F32_SIGN_MASK {
            return -1.0;
        } else {
            return 0.0;
        }
    } else if exp == 0x80 {
        return i + i;
    } else if exp >= 23 {
        return i;
    } else {
        let mask = F32_MANTISSA_MASK >> (exp as uint);
        if bits & mask == 0 {
            return i;
        }
        if bits > F32_SIGN_MASK {
            bits += 0x0080_0000 >> (exp as uint);
        }
        bits &= !mask;
    }
    bits.from_bits()
}

/// Round the 64-bit floating-point number towards -∞ (negative infinity).
#[no_mangle]
pub extern fn floor(i : f64) -> f64 {
    // Implementation is exactly the same as floorf with constants adapted for f64.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        if bits > F64_SIGN_MASK {
            return -1.0;
        } else {
            return 0.0;
        }
    } else if exp == 0x400 {
        return i + i;
    } else if exp >= 52 {
        return i;
    } else {
        let mask = F64_MANTISSA_MASK >> (exp as uint);
        if bits & mask == 0 {
            return i;
        }
        if bits > F64_SIGN_MASK {
            bits += 0x0010_0000_0000_0000;
        }
        bits &= !mask;
    }
    bits.from_bits()
}
