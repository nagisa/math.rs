// Read sin.rs for most of implementation commentary.
//
// Interesting property is that tan(π/4) = ctg(π/4). This, as was a case with sines and
// cosines, allows us to reduce our input quite aggressively and be somewhat accurate while using
// polynomial approximations.

use core::f64;
use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

use utils::FRAC_3PI_4;
use copysign::copysign;

fn _tan(i: f64) -> f64 {
    // Taylor series for tan(x) look like this:
    //
    //      x^3     2x^5     17x^7     62x^9     1382x^11     21844x^13
    // x + ----- + ------ + ------- + ------- + ---------- + ----------- + …
    //       3       15       315      2835       155925       6081075
    //
    // Note that this converges much more slowly compared to sine and cosine series.
    // This generally means that the error near the π/4 will be somewhat bigger for tan, compared
    // to sines and cosines.
    const D3: f64 = 0.333333333333333333333333333333;
    const D5: f64 = 1.333333333333333333333333333333E-1;
    const D7: f64 = 5.396825396825397080924346937536E-2;
    const D9: f64 = 2.186948853615520299564778383683E-2;
    const D11: f64 = 8.863235529902197332163815701733E-3;
    const D13: f64 = 3.592128036572481142307822210569E-3;
    let i2 = i * i;
    let i4 = i2 * i2;
    let i6 = i4 * i2;
    let i10 = i6 * i4;

    return i + (i2 * D3 * i) + (i4 * D5 * i) + (i6 * D7 * i) + (i4 * D9 * i4 * i) +
           (i10 * D11 * i) + (i10 * D13 * i2 * i);
}

fn _ctg(i: f64) -> f64 {
    // Taylor series don’t quite work for cotangent. An alternative that fits our use case
    // is Laurent series:
    //
    //  1     x     x^3     2x^5     x^7      2x^9
    // --- - --- - ----- - ------ - ------ - ------- - …
    //  x     3      45      945     4725     93555
    //
    // Interestingly this series converge really (and I mean it) quickly in the range we care
    // about.
    const D1: f64 = -0.333333333333333333333333333333;
    const D3: f64 = -2.222222222222222307030925492199E-2;
    const D5: f64 = -2.116402116402116544841005563171E-3;
    const D7: f64 = -2.116402116402116490630896938896E-4;
    const D9: f64 = -2.137779915557693459103247302089E-5;
    let i2 = i * i;
    let i4 = i2 * i2;

    return (i * D1) + (i2 * D3 * i) + (i4 * D5 * i) + (i4 * D7 * i2 * i) + (i4 * D9 * i4 * i) +
           i.recip();
}


/// Calculate a tangent
pub extern "C" fn tan(i: f64) -> f64 {
    // If x is not finite, the function must return a NAN.
    if !i.is_finite() {
        return f64::NAN;
    }

    let y = i.abs() % PI;
    if y < FRAC_PI_4 {
        copysign(_tan(y), i)
    } else if y <= FRAC_PI_2 {
        copysign(_ctg(y - FRAC_PI_2), i)
    } else if y <= FRAC_3PI_4 {
        copysign(_ctg(y - FRAC_PI_2), -i)
    } else {
        copysign(_tan(y - PI), -i)
    }
}

pub extern "C" fn tanf(i: f32) -> f32 {
    tan(i as f64) as f32
}
