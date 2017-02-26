extern crate math;
extern crate libloading;
extern crate quickcheck;

use std::{f32, f64};
use testutils::*;

#[macro_use]
mod testutils;

check!(sqrtf ~ |x: f32| -> f32 {
    math::sqrtf(x)
},
[
    (0.0), (4.0), (9.0), (20.25), (0.065536), (1.4E-45),
    (f32::NAN), (f32::INFINITY), (-1.0), (f32::NEG_INFINITY), (f32::MIN)
]);

check!(sqrt ~ |x: f64| -> f64 {
    math::sqrt(x)
},
[
    (0.0), (4.0), (9.0), (20.25), (0.065536), (1.4E-45),
    // (4.94065645841246544176568792868E-324),
    (f64::NAN), (f64::INFINITY), (-1.0), (f64::NEG_INFINITY), (f64::MIN)
]);

check!(expf ~ |x: f32| -> f32 {
    math::expf(x)
},
[
    (0.0), (0.6931471806),
    (f32::NAN), (-f32::NAN), (f32::INFINITY), (f32::NEG_INFINITY), (89.0), (-104.0)
]);

check!(exp ~ |x: f64| -> f64 {
    math::exp(x)
},
[
    (0.0), (0.693147180559945309417232121458176568075),
    (f64::NAN), (-f64::NAN), (f64::INFINITY), (f64::NEG_INFINITY), (710.0), (-746.0)
]);
