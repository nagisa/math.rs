use core::{f32, f64};
use core::num::{Int, Float};

use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F32_DENORMAL_EXP, F32_MANTISSA_MASK, F32_MAX_EXP};
use utils::{F64_SIGN_MASK, F64_DENORMAL_EXP, F64_MANTISSA_MASK, F64_MAX_EXP};

/// Compute the floating point remainder of dividing `l` by `r`.
#[no_mangle]
#[inline]
pub extern fn fmodf(l: f32, r: f32) -> f32 {
    let (mut lbits, mut rbits) = (l.as_bits(), r.as_bits());
    let (mut lexp, mut rexp) = (lbits.get_exponent(), rbits.get_exponent());
    let lsign = lbits & F32_SIGN_MASK;

    // Special cases
    if !l.is_finite() || r.is_nan() || rbits.is_zero() {
        return f32::NAN;
    } else if l == 0.0 {
        return l;
    }


    // Normalise l
    if lexp == F32_DENORMAL_EXP {
        let sh = (lbits.leading_zeros() as i32) - 9;
        lexp -= sh;
        lbits <<= sh + 1;
    } else {
        lbits &= F32_MANTISSA_MASK;
        lbits |= 0x0080_0000;
    }

    // Normalise r
    if rexp == F32_DENORMAL_EXP {
        let sh = (rbits.leading_zeros() as i32) - 9;
        rexp -= sh;
        rbits <<= sh + 1;
    } else {
        rbits &= F32_MANTISSA_MASK;
        rbits |= 0x0080_0000;
    }

    // If the input was denormal, our {l,r}exp <= F32_DENORMAL_EXP

    while lexp > rexp {
        lexp -= 1;
        let temp = lbits - rbits;
        if temp == 0 {
            return 0.0 * l;
        } else if temp >> 31 == 0 {
            lbits = temp;
        }
        lbits = lbits << 1;
    }

    {
        // lexp <= rexp
        let temp = lbits - rbits;
        if temp == 0 {
            return 0.0 * l;
        } else if temp >> 31 == 0 {
            lbits = temp;
        }
    }

    // Normalise again!
    {
        let sh = (lbits.leading_zeros() as i32) - 8;
        lexp -= sh;
        lbits <<= sh;
    }

    if lexp > -F32_MAX_EXP {
        lbits -= 0x0080_0000;
        lbits |= ((lexp + F32_MAX_EXP) as u32) << 23;
    } else {
        lbits >>= 1 - lexp - F32_MAX_EXP;
    }
    lbits |= lsign;
    lbits.from_bits()
}

/// Compute the floating point remainder of dividing `l` by `r`.
#[no_mangle]
#[inline]
pub extern fn fmod(l: f64, r: f64) -> f64 {
    let (mut lbits, mut rbits) = (l.as_bits(), r.as_bits());
    let (mut lexp, mut rexp) = (lbits.get_exponent(), rbits.get_exponent());
    let lsign = lbits & F64_SIGN_MASK;

    if !l.is_finite() || r.is_nan() || rbits.is_zero() {
        return f64::NAN;
    } else if l == 0.0 {
        return l;
    }

    if lexp == F64_DENORMAL_EXP {
        let sh = (lbits.leading_zeros() as i32) - 12;
        lexp -= sh;
        lbits <<= sh + 1;
    } else {
        lbits &= F64_MANTISSA_MASK;
        lbits |= 0x0010_0000_0000_0000;
    }
    if rexp == F64_DENORMAL_EXP {
        let sh = (rbits.leading_zeros() as i32) - 12;
        rexp -= sh;
        rbits <<= sh + 1;
    } else {
        rbits &= F64_MANTISSA_MASK;
        rbits |= 0x0010_0000_0000_0000;
    }

    while lexp > rexp {
        lexp -= 1;
        let temp = lbits - rbits;
        if temp == 0 {
            return 0.0 * l;
        } else if temp >> 63 == 0 {
            lbits = temp;
        }
        lbits = lbits << 1;
    }
    {
        // lexp <= rexp
        let temp = lbits - rbits;
        if temp == 0 {
            return 0.0 * l;
        } else if temp >> 63 == 0 {
            lbits = temp;
        }
    }
    {
        let sh = (lbits.leading_zeros() as i32) - 11;
        lexp -= sh;
        lbits <<= sh;
    }

    if lexp > -F64_MAX_EXP {
        lbits -= 0x0010_0000_0000_0000;
        lbits |= ((lexp + F64_MAX_EXP) as u64) << 52;
    } else {
        lbits >>= 1 - lexp - F64_MAX_EXP;
    }
    lbits |= lsign;
    lbits.from_bits()
}
