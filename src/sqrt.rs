use core::num::Int;

use utils::{AsBits, Bits};
use utils::{F32_NAN_EXP, F32_SIGN_MASK, F32_DENORMAL_EXP, F32_MANTISSA_MASK};
use utils::{F64_NAN_EXP, F64_SIGN_MASK, F64_DENORMAL_EXP, F64_MANTISSA_MASK};

/// Calculate a square root.
#[no_mangle]
pub extern fn sqrtf(i: f32) -> f32 {
    let mut bits = i.as_bits();
    let mut exp = bits.get_exponent();

    if exp == F32_NAN_EXP {
        // sqrt(NAN) = NAN, sqrt(+∞) = +∞, sqrt(-∞) = sNAN
        return i * i + i;
    } else if bits & !F32_SIGN_MASK == 0 {
        // sqrt(±0) = ±0
        return i;
    } else if (bits as i32) < 0 {
        // sqrt(-x) = sNAN
        return (i - i) / (i - i);
    }

    if exp == F32_DENORMAL_EXP {
        exp -= (bits.leading_zeros() as i32) - 9;
    }

    bits = (bits & F32_MANTISSA_MASK) | 0x0080_0000;

    if (exp & 1) == 1 {
        bits += bits;
    }
    exp >>= 1;
    bits += bits;

    // Generate sqrt(x) for each bit
    let mut s = 0i32;
    let mut q = 0i32;
    let mut r = 0x0100_0000i32;
    let mut t;
    while r != 0 {
        t = s + r;
        if t < (bits as i32) {
            s = t + r;
            bits -= t as u32;
            q = q + r;
        }
        bits += bits;
        r >>= 1;
    }
    // Round the result.
    if bits != 0 {
        let mut temp = 1.0 - 1E-30;
        if temp >= 1.0 {
            temp = 1.0 + 1E-30;
        }
        if temp > 1.0 {
            q += 2;
        } else {
            q += q & 1;
        }
    }
    bits = ((q >> 1) + 0x3F00_0000) as u32;
    bits += (exp as u32) << 23;
    bits.from_bits()
}

/// Calculate a square root.
#[no_mangle]
pub extern fn sqrt(i: f64) -> f64 {
    let mut bits = i.as_bits();
    let mut exp = bits.get_exponent();

    if exp == F64_NAN_EXP {
        // sqrt(NAN) = NAN, sqrt(+∞) = +∞, sqrt(-∞) = sNAN
        return i * i + i;
    } else if bits & !F64_SIGN_MASK == 0 {
        // sqrt(±0) = ±0
        return i;
    } else if (bits as i64) < 0 {
        // sqrt(-x) = sNAN
        return (i - i) / (i - i);
    }

    if exp == F64_DENORMAL_EXP {
        exp -= (bits.leading_zeros() as i32) - 12;
    }

    bits = (bits & F64_MANTISSA_MASK) | 0x0010_0000_0000_0000;

    if (exp & 1) == 1 {
        bits += bits;
    }
    exp >>= 1;
    bits += bits;

    // Generate sqrt(x) for each bit
    let mut s = 0i64;
    let mut q = 0i64;
    let mut r = 0x0020_0000_0000_0000;
    let mut t;
    while r != 0 {
        t = s + r;
        if t < (bits as i64) {
            s = t + r;
            bits -= t as u64;
            q = q + r;
        }
        bits += bits;
        r >>= 1;
    }
    // Round the result.
    if bits != 0 {
        let mut temp = 1.0 - 1E-300;
        if temp >= 1.0 {
            temp = 1.0 + 1E-300;
        }
        if temp > 1.0 {
            q += 2;
        } else {
            q += q & 1;
        }
    }
    bits = ((q >> 1) + 0x3FE0_0000_0000_0000) as u64;
    bits += (exp as u64) << 52;
    bits.from_bits()
}
