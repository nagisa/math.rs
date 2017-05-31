use core::f64;
use core::f64::consts::FRAC_PI_2;

use utils::{AsBits, Bits};
use copysign::copysign;

#[cfg(not(test))]
use utils::Float;

fn r(i: f64) -> f64 {
    const P0: f64 = 1.66666666666666657415E-01;
    const P1: f64 = -3.25565818622400915405E-01;
    const P2: f64 = 2.01212532134862925881E-01;
    const P3: f64 = -4.00555345006794114027E-02;
    const P4: f64 = 7.91534994289814532176E-04;
    const P5: f64 = 3.47933107596021167570E-05;
    const Q1: f64 = -2.40339491173441421878E+00;
    const Q2: f64 = 2.02094576023350569471E+00;
    const Q3: f64 = -6.88283971605453293030E-01;
    const Q4: f64 = 7.70381505559019352791E-02;
    let p = i * (P0 + i * (P1 + i * (P2 + i * (P3 + i * (P4 + i * P5)))));
    let q = 1.0 + i * (Q1 + i * (Q2 + i * (Q3 + i * Q4)));
    return p / q;
}

/// Calculate the arc sine of an input.
#[no_mangle]
#[inline]
pub extern "C" fn asin(i: f64) -> f64 {
    const P2_HI: f64 = 1.57079632679489655800E+00;
    const P2_LO: f64 = 6.12323399573676603587E-17;

    if !i.is_finite() || i > 1.0 {
        f64::NAN
    } else if i == 0.0 {
        i
    } else if i.abs() == 1.0 {
        copysign(FRAC_PI_2, i)
    } else if i.abs() < 0.5 {
        if i.as_bits().get_exponent() < -26 {
            return i;
        }
        i + i * r(i * i)
    } else {
        let z = (1.0 - i.abs()) * 0.5;
        let s = z.sqrt();
        let r_ = r(z);
        if i.abs() > 0.975 {
            copysign(P2_HI - (2.0 * (s + s * r_) - P2_LO), i)
        } else {
            let f = (s.as_bits() & 0xFFFF_FFFF_0000_0000).from_bits();
            let c = (z - f * f) / (s + f);
            copysign(0.5 * P2_HI - (2.0 * s * r_ - (P2_LO - 2.0 * c) - (0.5 * P2_HI - 2.0 * f)), i)
        }
    }
}

/// Calculate the arc sine of an input.
#[no_mangle]
#[inline]
pub extern "C" fn asinf(i: f32) -> f32 {
    asin(i as f64) as f32
}
