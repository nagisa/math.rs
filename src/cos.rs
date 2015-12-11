// Read sin.rs for implementation details.

use core::f64;
use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

use utils::{FRAC_3PI_4, FRAC_5PI_4, FRAC_3PI_2, FRAC_7PI_4, PI_2};
use sin::_sin;


pub fn _cos(i: f64) -> f64 {
    // Taylor series for cos(x) look like this:
    //
    //     x^2    x^4     x^6       x^8         x^10        x^12
    // 1 - --- + ----- - ------ + -------- - ---------- + ---------- - …
    //      2!     4!      6!        8!          10!         12!
    const D2: f64 = -0.5;
    const D4: f64 = 4.166666666666666666666666666666E-2;
    const D6: f64 = -1.388888888888888888888888888888E-3;
    const D8: f64 = 2.480158730158730158730158730158E-5;
    const D10: f64 = -2.755731922398589065255731922398E-7;
    const D12: f64 = 2.087675698786809897921009032120E-9;
    let i2 = i * i;
    let i4 = i2 * i2;
    let i6 = i4 * i2;

    return 1.0 + (i * D2 * i) + (i2 * D4 * i2) + (i4 * D6 * i2) + (i4 * D8 * i4) +
           (i6 * D10 * i4) + (i6 * D12 * i6);
}

/// Calculate the cosine of an input.
#[no_mangle]
pub extern "C" fn cos(mut i: f64) -> f64 {
    // If x is not finite, the function must return a NAN.
    if !i.is_finite() {
        return f64::NAN;
    }

    i = i % PI_2;
    let iabs = i.abs();

    if iabs <= FRAC_PI_4 {
        _cos(i)
    } else if iabs <= FRAC_3PI_4 {
        -_sin(iabs - FRAC_PI_2)
    } else if iabs <= FRAC_5PI_4 {
        -_cos(iabs - PI)
    } else if iabs <= FRAC_7PI_4 {
        _sin(iabs - FRAC_3PI_2)
    } else {
        _cos(iabs - PI_2)
    }
}

/// Calculate the cosine of an input.
#[no_mangle]
pub extern "C" fn cosf(i: f32) -> f32 {
    cos(i as f64) as f32
}
