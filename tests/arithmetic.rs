extern crate math;
extern crate libloading;
extern crate quickcheck;

use std::{f32, f64};
use testutils::*;

#[macro_use]
mod testutils;

check!(fdimf ~ |x: f32, y: f32| -> f32 {
    math::fdimf(x, y)
},
[
    ( 0.0,  0.0), ( 9.0,  0.0), ( 0.0,  9.0), (-9.0,  0.0), ( 0.0, -9.0),

    (f32::INFINITY, 9.0), (f32::INFINITY, -9.0), (f32::NEG_INFINITY, 9.0),
    (f32::NEG_INFINITY, -9.0), ( 9.0, f32::NEG_INFINITY), (-9.0, f32::NEG_INFINITY),
    ( 9.0, f32::INFINITY), (-9.0, f32::INFINITY), (f32::INFINITY, f32::INFINITY),
    (f32::INFINITY, f32::NEG_INFINITY), (f32::NEG_INFINITY, f32::INFINITY),
    (f32::NEG_INFINITY, f32::NEG_INFINITY),

    (0.0, f32::NAN), (9.0, f32::NAN), (-9.0, f32::NAN), (f32::INFINITY, f32::NAN),
    (f32::NEG_INFINITY, f32::NAN), (f32::NAN, 0.0), (f32::NAN, 9.0), (f32::NAN, -9.0),
    (f32::NAN, f32::INFINITY), (f32::NAN, f32::NEG_INFINITY), (f32::NAN, f32::NAN)
]);

check!(fdim ~ |x: f64, y: f64| -> f64 {
    math::fdim(x, y)
},
[
    ( 0.0,  0.0), ( 9.0,  0.0), ( 0.0,  9.0), (-9.0,  0.0), ( 0.0, -9.0),

    (f64::INFINITY, 9.0), (f64::INFINITY, -9.0), (f64::NEG_INFINITY, 9.0),
    (f64::NEG_INFINITY, -9.0), ( 9.0, f64::NEG_INFINITY), (-9.0, f64::NEG_INFINITY),
    ( 9.0, f64::INFINITY), (-9.0, f64::INFINITY), (f64::INFINITY, f64::INFINITY),
    (f64::INFINITY, f64::NEG_INFINITY), (f64::NEG_INFINITY, f64::INFINITY),
    (f64::NEG_INFINITY, f64::NEG_INFINITY),

    (0.0, f64::NAN), (9.0, f64::NAN), (-9.0, f64::NAN), (f64::INFINITY, f64::NAN),
    (f64::NEG_INFINITY, f64::NAN), (f64::NAN, 0.0), (f64::NAN, 9.0), (f64::NAN, -9.0),
    (f64::NAN, f64::INFINITY), (f64::NAN, f64::NEG_INFINITY), (f64::NAN, f64::NAN)
]);


// check!(fmodf ~ |x: f32, y: f32| -> f32 {
//     math::fmodf(x, y)
// },
// [
//     (f32::NAN, 1.0), (1.0, f32::NAN), (1.0, 0.0), (f32::INFINITY, 1.0), (0.0, 0.0), (0.0, 1.0),
//     (1.0, 1.0), (5.0, 2.5), (25.0, 5.0),
//     (5.5, 3.0), (842.105, 632.105)
// ]);
//
// check!(fmod ~ |x: f64, y: f64| -> f64 {
//     math::fmod(x, y)
// },
// [
//     (f64::NAN, 1.0), (1.0, f64::NAN), (1.0, 0.0), (f64::INFINITY, 1.0), (0.0, 0.0), (0.0, 1.0),
//     (1.0, 1.0), (5.0, 2.5), (25.0, 5.0),
//     (5.5, 3.0), (842.105, 632.105)
// ]);


// check!(modf ~ |x: f32| -> (f32, f32) {
//     let mut y = 0.0f32;
//     let x = math::modff(x, &mut y as *mut _);
//     (x, y)
// }, |f: unsafe extern fn(f32, *mut f32) -> f32, x: f32| -> (f32, f32) {
//     let mut y = 0.0f32;
//     let x = unsafe { f(x, &mut y as *mut _) };
//     (x, y)
//
// },
// [
// ]);
