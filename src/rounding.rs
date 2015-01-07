//! Rounding operations on IEEE 754 floating-point numbers.

use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK, F32_MANTISSA_MASK, F64_MANTISSA_MASK};

use core::mem::size_of;
// TODO: get rid on all these
use libc::types::os::arch::c99;
use libc::types::os::arch::c95;


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
    } else if exp == 0x80 {
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
    } else if exp == 0x400 {
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

/// Truncate the 32-bit floating-point number to nearest integral value not larger than the number.
#[no_mangle]
pub extern fn truncf(i : f32) -> f32 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // The number ∈ (-1; 1) therefore the result is ±0.
        bits &= F32_SIGN_MASK;
    } else if exp == 0x80 {
        return i + i;
    } else if exp >= 23 {
        return i;
    } else {
        // Perserve the sign and simply truncate mantissa as necessary.
        bits = (bits & F32_SIGN_MASK) | (bits & !(F32_MANTISSA_MASK >> (exp as uint)));
    }
    bits.from_bits()
}

/// Truncate the 64-bit floating-point number to nearest integral value not larger than the number.
#[no_mangle]
pub extern fn trunc(i : f64) -> f64 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // The number ∈ (-1; 1) therefore the result is ±0.
        bits &= F64_SIGN_MASK;
    } else if exp == 0x400 {
        return i + i;
    } else if exp >= 52 {
        return i;
    } else {
        // Perserve the sign and simply truncate mantissa as necessary.
        bits = (bits & F64_SIGN_MASK) | (bits & !(F64_MANTISSA_MASK >> (exp as uint)));
    }
    bits.from_bits()
}



/// Round the 32-bit floating-point number number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn lroundf(i : f32) -> c95::c_long {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    // Minus one, because one bit has to be reserved for sign.
    let target_size = 8 * size_of::<c95::c_long>() - 1;
    let sign = if bits & F32_SIGN_MASK == 0 { 1 } else { -1 };

    if exp < -1 {
        return 0;
    } else if exp == -1 {
        return sign;
    } else if exp >= (target_size as i32) {
        // Result doesn’t fit. 0 is as arbitrary as any other value.
        return 0;
    } else if exp >= 23 {
        bits &= F32_MANTISSA_MASK;
        bits |= 0x0080_0000;
        return sign * ((bits >> ((exp - 23) as uint)) as c95::c_long);
    } else {
        bits &= F32_MANTISSA_MASK;
        bits |= 0x0080_0000;
        bits += 0x0040_0000 >> (exp as uint);
        return sign * ((bits >> ((23 - exp) as uint)) as c95::c_long);
    };
}

/// Round the 64-bit floating-point number number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn lround(i : f64) -> c95::c_long {
    // Same thing as lroundf with constants adapted.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    let target_size = 8 * size_of::<c95::c_long>() - 1;
    let sign = if bits & F64_SIGN_MASK == 0 { 1 } else { -1 };

    if exp < -1 {
        return 0;
    } else if exp == -1 {
        return sign;
    } else if exp >= (target_size as i32) {
        return 0;
    } else if exp >= 52 {
        bits &= F64_MANTISSA_MASK;
        bits |= 0x0010_0000_0000_0000;
        return sign * ((bits >> ((exp - 52) as uint)) as c95::c_long);
    } else {
        bits &= F64_MANTISSA_MASK;
        bits |= 0x0010_0000_0000_0000;
        bits += 0x0008_0000_0000_0000 >> (exp as uint);
        return sign * ((bits >> ((52 - exp) as uint)) as c95::c_long);
    }
}

/// Round the 32-bit floating-point number number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn llroundf(i : f32) -> c99::c_longlong {
    // Same thing as lroundf with different target_size.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    // Minus one, because one bit has to be reserved for sign.
    let target_size = 8 * size_of::<c99::c_longlong>() - 1;
    let sign = if bits & F32_SIGN_MASK == 0 { 1 } else { -1 };

    if exp < -1 {
        return 0;
    } else if exp == -1 {
        return sign;
    } else if exp >= (target_size as i32) {
        // Result doesn’t fit. 0 is as arbitrary as any other value.
        return 0;
    } else if exp >= 23 {
        bits &= F32_MANTISSA_MASK;
        bits |= 0x0080_0000;
        return sign * ((bits >> ((exp - 23) as uint)) as c95::c_long);
    } else {
        bits &= F32_MANTISSA_MASK;
        bits |= 0x0080_0000;
        bits += 0x0040_0000 >> (exp as uint);
        return sign * ((bits >> ((23 - exp) as uint)) as c95::c_long);
    };
}

/// Round the 64-bit floating-point number number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn llround(i : f64) -> c99::c_longlong {
    // Same thing as lround with different target_size.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    let target_size = 8 * size_of::<c99::c_longlong>() - 1;
    let sign = if bits & F64_SIGN_MASK == 0 { 1 } else { -1 };

    if exp < -1 {
        return 0;
    } else if exp == -1 {
        return sign;
    } else if exp >= (target_size as i32) {
        return 0;
    } else if exp >= 52 {
        bits &= F64_MANTISSA_MASK;
        bits |= 0x0010_0000_0000_0000;
        return sign * ((bits >> ((exp - 52) as uint)) as c95::c_long);
    } else {
        bits &= F64_MANTISSA_MASK;
        bits |= 0x0010_0000_0000_0000;
        bits += 0x0008_0000_0000_0000 >> (exp as uint);
        return sign * ((bits >> ((52 - exp) as uint)) as c95::c_long);
    }
}
