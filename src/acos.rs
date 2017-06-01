use core::f64::consts::FRAC_PI_2;
use asin::asin;

use utils::Float;

/// Calculate the arc cosine of an input.
#[no_mangle]
#[inline]
pub extern "C" fn acos(i: f64) -> f64 {
    if i > 0.0 {
        FRAC_PI_2 - asin(i)
    } else {
        FRAC_PI_2 + asin(i.abs())
    }
}

/// Calculate the arc cosine of an input.
#[no_mangle]
#[inline]
pub extern "C" fn acosf(i: f32) -> f32 {
    acos(i as f64) as f32
}
