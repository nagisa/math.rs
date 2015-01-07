use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK, F32_MANTISSA_MASK, F64_MANTISSA_MASK};

/// Round the 32-bit floating-point number towards +∞ (positive infinity).
#[no_mangle]
pub extern fn ceilf(i : f32) -> f32 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        if bits >= F32_SIGN_MASK || bits == 0 {
            return 0.0;
        } else {
            return 1.0;
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
        if bits < F32_SIGN_MASK {
            bits += 0x0080_0000 >> (exp as uint);
        }
        bits &= !mask;
    }
    bits.from_bits()
}

/// Round the 64-bit floating-point number towards +∞ (positive infinity).
#[no_mangle]
pub extern fn ceil(i : f64) -> f64 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        if bits >= F64_SIGN_MASK || bits == 0 {
            return 0.0;
        } else {
            return 1.0;
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
        if bits < F64_SIGN_MASK {
            bits += 0x0010_0000_0000_0000 >> (exp as uint);
        }
        bits &= !mask;
    }
    bits.from_bits()
}
