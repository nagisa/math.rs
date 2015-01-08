use core::{f32, f64};
// TODO: remove this
use libc::c_int;

use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F32_EXP_MASK, F32_MAX_EXP, F32_MIN_EXP, F32_DENORMAL_EXP, F32_NAN_EXP};
use utils::{F64_SIGN_MASK, F64_EXP_MASK, F64_MAX_EXP, F64_MIN_EXP, F64_DENORMAL_EXP, F64_NAN_EXP};
use copysign::{copysignf, copysign};

const F32_TWO25 : f32 = 3.355443200e+07; // 0x4c00_0000
const F32_TWOM25 : f32 = 2.9802322388e-08; // 0x3300_0000
const F64_TWO54 : f64 = 1.80143985094819840000e+16;  // 0x4350_0000_0000_0000
const F64_TWOM54 : f64 = 5.55111512312578270212e-17; // 0x3C90_0000_0000_0000

/// Multiply the 32-bit floating-point number by integral power of radix.
#[no_mangle]
pub extern fn scalbnf(i: f32, x: c_int) -> f32 {
    let mut bits = i.as_bits();
    let mut exp = bits.get_exponent();

    if bits & !F32_SIGN_MASK == 0 {
        // 0 * 2^x
        return 0.0;
    } else if exp == F32_NAN_EXP {
        return i + i;
    } else if exp == F32_DENORMAL_EXP {
        // Denormal
        bits = (i * F32_TWO25).as_bits();
        exp = bits.get_exponent() - 25;
    }
    if exp + x > F32_MAX_EXP {
        // Overflow
        return copysignf(f32::INFINITY, i);
    } else if exp + x < F32_MIN_EXP {
        // Underflow
        return copysignf(0.0, i);
    }
    exp += x;
    bits &= !F32_EXP_MASK;
    if exp > F32_DENORMAL_EXP {
        bits |= (((exp + F32_MAX_EXP) as u32) << 23);
        return bits.from_bits();
    } else {
        exp += 25; // Still denormal
        bits |= (((exp + F32_MAX_EXP) as u32) << 23);
        return bits.from_bits() * F32_TWOM25;
    }
}

/// Multiply the 64-bit floating-point number by integral power of radix.
#[no_mangle]
pub extern fn scalbn(i: f64, x: c_int) -> f64 {
    let mut bits = i.as_bits();
    let mut exp = bits.get_exponent();

    if bits & !F64_SIGN_MASK == 0 {
        // 0 * 2^x
        return 0.0;
    } else if exp == F64_NAN_EXP {
        return i + i;
    } else if exp == F64_DENORMAL_EXP {
        // Denormal
        bits = (i * F64_TWO54).as_bits();
        exp = bits.get_exponent() - 54;
    }
    if exp + x > F64_MAX_EXP {
        // Overflow
        return copysign(f64::INFINITY, i);
    } else if exp + x < F64_MIN_EXP {
        // Underflow
        return copysign(0.0, i);
    }
    exp += x;
    bits &= !F64_EXP_MASK;
    if exp > F64_DENORMAL_EXP {
        bits |= (((exp + F64_MAX_EXP) as u64) << 52);
        return bits.from_bits();
    } else {
        exp += 54; // Still denormal
        bits |= (((exp + F64_MAX_EXP) as u64) << 52);
        return bits.from_bits() * F64_TWOM54;
    }
}
