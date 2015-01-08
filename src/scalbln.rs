use core::{f32, f64};
// TODO: remove this
use libc::c_long;

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
pub extern fn scalblnf(i: f32, x: c_long) -> f32 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    // TODO: switch to unchecked_add, we check for overflows ourselves
    let mut expx = exp + (x as i32);

    if bits & !F32_SIGN_MASK == 0 {
        // 0 * 2^x
        return 0.0;
    } else if exp == F32_NAN_EXP {
        return i + i;
    } else if exp == F32_DENORMAL_EXP {
        // Denormal
        bits = (i * F32_TWO25).as_bits();
        expx = bits.get_exponent() - 25 + (x as i32);
    }
    if expx > F32_MAX_EXP || x > 500 {
        // Overflow
        return copysignf(f32::INFINITY, i);
    } else if expx < F32_MIN_EXP || x < -500 {
        // Underflow
        return copysignf(0.0, i);
    }
    bits &= !F32_EXP_MASK;
    if expx > F32_DENORMAL_EXP {
        bits |= ((expx + F32_MAX_EXP) as u32) << 23;
        return bits.from_bits();
    } else {
        expx += 25; // Still denormal
        bits |= ((expx + F32_MAX_EXP) as u32) << 23;
        return bits.from_bits() * F32_TWOM25;
    }
}

/// Multiply the 64-bit floating-point number by integral power of radix.
#[no_mangle]
pub extern fn scalbln(i: f64, x: c_long) -> f64 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    let mut expx = exp + (x as i32);

    if bits & !F64_SIGN_MASK == 0 {
        // 0 * 2^x
        return 0.0;
    } else if exp == F64_NAN_EXP {
        return i + i;
    } else if exp == F64_DENORMAL_EXP {
        // Denormal
        bits = (i * F64_TWO54).as_bits();
        expx = bits.get_exponent() - 54 + (x as i32);
    }
    if expx > F64_MAX_EXP || x > 5000 {
        // Overflow
        return copysign(f64::INFINITY, i);
    } else if expx < F64_MIN_EXP || x < -5000  {
        // Underflow
        return copysign(0.0, i);
    }
    bits &= !F64_EXP_MASK;
    if expx > F64_DENORMAL_EXP {
        bits |= ((expx + F64_MAX_EXP) as u64) << 52;
        return bits.from_bits();
    } else {
        expx += 54; // Still denormal
        bits |= ((expx + F64_MAX_EXP) as u64) << 52;
        return bits.from_bits() * F64_TWOM54;
    }
}
