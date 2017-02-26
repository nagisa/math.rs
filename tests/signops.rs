extern crate math;
extern crate quickcheck;
extern crate libloading;

use std::{f64, f32};
use testutils::*;

#[macro_use]
mod testutils;

check!(fabs ~ |x: f64| -> f64 {
    math::fabs(x)
},
[(0.0), (-0.0), (f64::INFINITY), (f64::NEG_INFINITY), (f64::NAN), (-f64::NAN), (-f64::EPSILON)]);

check!(fabsf ~ |x: f32| -> f32 {
    math::fabsf(x)
},
[(0.0), (-0.0), (f32::INFINITY), (f32::NEG_INFINITY), (f32::NAN), (-f32::NAN), (-f32::EPSILON)]);

check!(copysign ~ |x: f64, y: f64| -> f64 {
    math::copysign(x, y)
},
[(0.0, 4.0), (0.0, -4.0), (-0.0, 4.0), (-0.0, -4.0),
 (f64::INFINITY, 0.0), (f64::NEG_INFINITY, 0.0), (f64::INFINITY, -0.0), (f64::NEG_INFINITY, -0.0),
 (0.0, f64::INFINITY), (-0.0, f64::INFINITY), (0.0, f64::NEG_INFINITY), (-0.0, f64::NEG_INFINITY),
 (0.0, f64::NAN), (-0.0, f64::NAN), (0.0, -f64::NAN), (-0.0, -f64::NAN),
 (f64::NAN, 0.0), (-f64::NAN, 0.0), (f64::NAN, -0.0), (-f64::NAN, -0.0),
 (1.0,  2.0), (1.0, -2.0), (-1.0,  2.0), (-1.0, -2.0)
]);

check!(copysignf ~ |x: f32, y: f32| -> f32 {
    math::copysignf(x, y)
},
[(0.0, 4.0), (0.0, -4.0), (-0.0, 4.0), (-0.0, -4.0),
 (f32::INFINITY, 0.0), (f32::NEG_INFINITY, 0.0), (f32::INFINITY, -0.0), (f32::NEG_INFINITY, -0.0),
 (0.0, f32::INFINITY), (-0.0, f32::INFINITY), (0.0, f32::NEG_INFINITY), (-0.0, f32::NEG_INFINITY),
 (0.0, f32::NAN), (-0.0, f32::NAN), (0.0, -f32::NAN), (-0.0, -f32::NAN),
 (f32::NAN, 0.0), (-f32::NAN, 0.0), (f32::NAN, -0.0), (-f32::NAN, -0.0),
 (1.0,  2.0), (1.0, -2.0), (-1.0,  2.0), (-1.0, -2.0)
]);
