// So, this will calculate sine. We take Taylor series approach to evaluation.
//
// First of all, input size needs to be reduced to something more manageable. Remembering
// sines have a period of 2π, reducing the input to [-π; π] comes to mind. However we will
// abuse the fact that:
// * ∀x ∈ [π/4; 3π/4]: sin(x) = cos(x - π/2)
// * ∀x ∈ [3π/4; 5π/4]: sin(x) = -sin(x - π)
// * ∀x ∈ [5π/4; 7π/4]: sin(x) = -cos(x - 3π/2)
// * ∀x ∈ [7π/4; 2π]: sin(x) = sin(x - 2π)
//
// This is necessary as Taylor series’ accuracy decreases quickly the farther away from
// 0 the argument is and magnitude of numbers involved in series doesn’t quite allow us to do a
// lot of iterations of the series.
//
// _sin function will deal with calculating sine in the reduced range.

use core::f64;
use core::f64::consts::{PI_2, PI, FRAC_PI_2, FRAC_PI_4};
use core::num::Float;

use utils::{FRAC_3PI_4, FRAC_5PI_4, FRAC_3PI_2, FRAC_7PI_4};
use copysign::{copysign};
use cos::{_cos};


pub fn _sin(i: f64) -> f64 {
    // We will use order 13 Taylor series for the sine – this gives us about 6-7 digits of
    // precision:
    //
    //     x^3    x^5     x^7       x^9         x^11        x^13
    // x - --- + ----- - ------ + -------- - ---------- + ---------- - …
    //      3!     5!      7!        9!          11!         13!
    const D3: f64 = -1.666666666666666666666666666666E-1;
    const D5: f64 =  8.333333333333333333333333333333E-3;
    const D7: f64 = -1.984126984126984126984126984126E-4;
    const D9: f64 =  2.755731922398589065255731922398E-6;
    const D11: f64 = -2.505210838544171877505210838544E-8;
    const D13: f64 =  1.605904383682161459939237717015E-10;
    let i2 = i * i;
    let i4 = i2 * i2;
    let i6 = i4 * i2;
    let i10 = i6 * i4;

    return i + (i2 * D3 * i) + (i4 * D5 * i) + (i6 * D7 * i) + (i4 * D9 * i4 * i)
             + (i10 * D11 * i) + (i10 * D13 * i2 * i);
}

/// Calculate the sine of an input.
#[no_mangle]
pub extern fn sin(mut i: f64) -> f64 {
    // If x is not finite, the function must return a NAN.
    if !i.is_finite() {
        return f64::NAN;
    }

    // Now, our task is to cut i into π/2 slices so we could pass them to corresponding _sin or
    // _cos function.

    // Wrap i into [-2π; 2π]:
    i = i % PI_2;
    let iabs = i.abs();

    if iabs <= FRAC_PI_4 {
        copysign(_sin(i), i)
    } else if iabs <= FRAC_3PI_4 {
        copysign(_cos(iabs - FRAC_PI_2), i)
    } else if iabs <= PI {
        copysign(_sin(iabs - PI), i)
    } else if iabs <= FRAC_5PI_4 {
        copysign(_sin(iabs - PI), -i)
    } else if iabs <= FRAC_7PI_4 {
        copysign(_cos(iabs - FRAC_3PI_2), -i)
    } else {
        copysign(_sin(iabs - PI_2), -i)
    }
}

/// Calculate the sine of an input.
#[no_mangle]
pub extern fn sinf(i: f32) -> f32 {
    sin(i as f64) as f32
}
