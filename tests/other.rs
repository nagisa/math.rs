extern crate math;
extern crate libloading;
extern crate quickcheck;

use std::{f32, f64};
use testutils::*;

#[macro_use]
mod testutils;

check!(fmaxf ~ |x: f32, y: f32| -> f32 {
    math::fmaxf(x, y)
},
[
    ( 0.0,  0.0), (-0.0, -0.0), ( 9.0,  0.0), (-9.0, -0.0), ( 0.0,  9.0), (-0.0, -9.0),
    ( 0.0, 9.0), (-0.0, -9.0),

    (f32::INFINITY,  9.0), (9.0, f32::INFINITY), (f32::INFINITY, -9.0), (-9.0, f32::INFINITY),
    ( f32::NEG_INFINITY,  9.0), ( 9.0, f32::NEG_INFINITY), ( f32::NEG_INFINITY, -9.0),
    (-9.0, f32::NEG_INFINITY),

    (f32::NAN,  9.0), (f32::NAN, -9.0), (9.0, f32::NAN), (-9.0, f32::NAN), (f32::NAN, f32::NAN)
]);

check!(fmax ~ |x: f64, y: f64| -> f64 {
    math::fmax(x, y)
},
[
    ( 0.0,  0.0), (-0.0, -0.0), ( 9.0,  0.0), (-9.0, -0.0), ( 0.0,  9.0), (-0.0, -9.0),
    ( 0.0, 9.0), (-0.0, -9.0),

    (f64::INFINITY,  9.0), (9.0, f64::INFINITY), (f64::INFINITY, -9.0), (-9.0, f64::INFINITY),
    ( f64::NEG_INFINITY,  9.0), ( 9.0, f64::NEG_INFINITY), ( f64::NEG_INFINITY, -9.0),
    (-9.0, f64::NEG_INFINITY),

    (f64::NAN,  9.0), (f64::NAN, -9.0), (9.0, f64::NAN), (-9.0, f64::NAN), (f64::NAN, f64::NAN)
]);

check!(fminf ~ |x: f32, y: f32| -> f32 {
    math::fminf(x, y)
},
[
    ( 0.0,  0.0), (-0.0, -0.0), ( 9.0,  0.0), (-9.0, -0.0), ( 0.0,  9.0), (-0.0, -9.0),
    ( 0.0, 9.0), (-0.0, -9.0),

    (f32::INFINITY,  9.0), (9.0, f32::INFINITY), (f32::INFINITY, -9.0), (-9.0, f32::INFINITY),
    ( f32::NEG_INFINITY,  9.0), ( 9.0, f32::NEG_INFINITY), ( f32::NEG_INFINITY, -9.0),
    (-9.0, f32::NEG_INFINITY),

    (f32::NAN,  9.0), (f32::NAN, -9.0), (9.0, f32::NAN), (-9.0, f32::NAN), (f32::NAN, f32::NAN)
]);

check!(fmin ~ |x: f64, y: f64| -> f64 {
    math::fmin(x, y)
},
[
    ( 0.0,  0.0), (-0.0, -0.0), ( 9.0,  0.0), (-9.0, -0.0), ( 0.0,  9.0), (-0.0, -9.0),
    ( 0.0, 9.0), (-0.0, -9.0),

    (f64::INFINITY,  9.0), (9.0, f64::INFINITY), (f64::INFINITY, -9.0), (-9.0, f64::INFINITY),
    ( f64::NEG_INFINITY,  9.0), ( 9.0, f64::NEG_INFINITY), ( f64::NEG_INFINITY, -9.0),
    (-9.0, f64::NEG_INFINITY),

    (f64::NAN,  9.0), (f64::NAN, -9.0), (9.0, f64::NAN), (-9.0, f64::NAN), (f64::NAN, f64::NAN)
]);

check!(hypotf ~ |x: f32, y: f32| -> f32 {
    math::hypotf(x, y)
},
[
    (f32::consts::PI, 0.0), (0.0, -f32::consts::PI), (3.0, 4.0), (-3.0, -4.0),
    (f32::INFINITY, 0.0), (f32::NEG_INFINITY, 0.0), (0.0, f32::INFINITY), (0.0, f32::NEG_INFINITY),
    (f32::INFINITY, f32::NAN), (f32::NAN, f32::INFINITY), (f32::NEG_INFINITY, f32::NAN),
    (f32::NAN, f32::NEG_INFINITY),

    // Sign of NaN is not specified
    (f32::NAN, 0.0), (-f32::NAN, 0.0), (f32::NAN, -f32::NAN)
]);

check!(hypot ~ |x: f64, y: f64| -> f64 {
    math::hypot(x, y)
},
[
    (f64::consts::PI, 0.0), (0.0, -f64::consts::PI), (3.0, 4.0), (-3.0, -4.0),
    (f64::INFINITY, 0.0), (f64::NEG_INFINITY, 0.0), (0.0, f64::INFINITY), (0.0, f64::NEG_INFINITY),
    (f64::INFINITY, f64::NAN), (f64::NAN, f64::INFINITY), (f64::NEG_INFINITY, f64::NAN),
    (f64::NAN, f64::NEG_INFINITY),

    // Sign of NaN is not specified
    (f64::NAN, 0.0), (-f64::NAN, 0.0), (f64::NAN, -f64::NAN)
]);
