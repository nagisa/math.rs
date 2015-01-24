use core::f64::consts::{FRAC_PI_2};
use core::num::Float;

/// Calculate the arc cosine of an input.
#[no_mangle]
pub extern fn acos(i: f64) -> f64 {
    if i > 0.0 {
        FRAC_PI_2 - i.asin()
    } else {
        FRAC_PI_2 + i.abs().asin()
    }
}

/// Calculate the arc cosine of an input.
#[no_mangle]
pub extern fn acosf(i: f32) -> f32 {
    acos(i as f64) as f32
}
