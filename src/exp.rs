use core::{f32, f64};

use utils::AsBits;
use utils::F32_SIGN_MASK;
use utils::F64_SIGN_MASK;
use scalbn::{scalbnf, scalbn};

const F32_HIGH: f32 = 88.72283935546875;
const F32_LOW: f32 = -103.972084045410;
const F32_LN2_INV: f32 = 1.44269502163;
const F32_HALFS: [f32; 2] = [0.5, -0.5];
const F32_LN2_HIGH: f32 = 6.9314575195E-1;
const F32_LN2_LOW: f32 = 1.4286067653e-6;

const F64_HIGH: f64 = 709.78271289338402993962517939507961273193359374;
const F64_LOW: f64 = -745.13321910194116526326979510486125946044921874;
const F64_LN2_INV: f64 = 1.44269504088896338700;
const F64_HALFS: [f64; 2] = [0.5, -0.5];
const F64_LN2_LOW: f64 = 1.90821492927058770002E-10;
const F64_LN2_HIGH: f64 = 6.93147180369123816490E-01;


/// Calculate e to the power of i.
#[no_mangle]
pub extern "C" fn expf(mut i: f32) -> f32 {
    if i < F32_LOW {
        // e to the power of less than F32_LOW is +0.
        return 0.0;
    } else if i > F32_HIGH {
        // Overflow, NAN of +∞.
        return f32::MAX * i;
    }

    let mut bits = i.as_bits();
    let sign = bits >> 31;
    let (k, high, low): (i32, f32, f32);
    bits &= !F32_SIGN_MASK;

    if bits > 0x3EB17218 {
        // |i| > 0.5ln(2)
        if bits > 0x3F85_1592 {
            // |i| > 1.5ln(2)
            k = (F32_LN2_INV * i + F32_HALFS[sign as usize]) as i32;
        } else {
            k = 1 - (sign * 2) as i32;
        }
        high = i - (k as f32) * F32_LN2_HIGH;
        low = (k as f32) * F32_LN2_LOW;
        i = high - low;
    } else if bits >= 0x3900_0000 {
        k = 0;
        high = i;
        low = 0.0;
    } else {
        return 1.0 + i;
    }

    let isq = i * i;
    let mut c = -2.7667332906E-3;
    c = 1.6666625440E-1 + isq * c;
    c = i - isq * c;
    let y = 1.0 + (i * c / (2.0 - c) - low + high);
    if k == 0 {
        y
    } else {
        scalbnf(y, k)
    }
}


/// Calculate e to the power of i.
#[no_mangle]
pub extern "C" fn exp(mut i: f64) -> f64 {
    if i < F64_LOW {
        return 0.0;
    } else if i > F64_HIGH {
        // Overflow, NAN of +∞.
        return f64::MAX * i;
    }

    let mut bits = i.as_bits();
    let sign = bits >> 63;
    let (k, high, low): (i32, f64, f64);
    bits &= !F64_SIGN_MASK;

    if bits > 0x3FD6_2E42_FEFA_39EF {
        // |i| > 0.5ln(2)
        if bits > 0x3FF0_A2B2_3F3B_AB73 {
            // |i| > 1.5ln(2)
            k = (F64_LN2_INV * i + F64_HALFS[sign as usize]) as i32;
        } else {
            k = 1 - (sign * 2) as i32;
        }
        high = i - (k as f64) * F64_LN2_HIGH;
        low = (k as f64) * F64_LN2_LOW;
        i = high - low;
    } else if bits >= 0x3e30_0000_0000_0000 {
        k = 0;
        high = i;
        low = 0.0;
    } else {
        return 1.0 + i;
    }

    let isq = i * i;
    let mut c = -1.65339022054652515390e-06;
    c = 4.13813679705723846039e-08 + isq * c;
    c = 6.61375632143793436117e-05 + isq * c;
    c = -2.77777777770155933842e-03 + isq * c;
    c = 1.66666666666666019037e-01 + isq * c;
    c = i - isq * c;
    let y = 1.0 + (i * c / (2.0 - c) - low + high);
    if k == 0 {
        y
    } else {
        scalbn(y, k)
    }
}
