#![feature(std_misc)]
extern crate math;
use std::num::Float;
use std::{f32, f64};

use math::{fdimf, fdim};
// use math::{fmodf, fmod};

#[test]
fn dim_f32() {
    assert_eq!(fdimf(10.0, 20.0), 0.0);
    assert_eq!(fdimf(20.0, -20.0), 40.0);
    assert!(fdimf(f32::NAN, 0.0).is_nan());
    assert!(fdimf(1.0, f32::NAN).is_nan());
    assert_eq!(fdimf(f32::MAX_VALUE, f32::MIN_VALUE), f32::INFINITY);
}

#[test]
fn dim_f64() {
    assert_eq!(fdim(10.0, 20.0), 0.0);
    assert_eq!(fdim(20.0, -20.0), 40.0);
    assert_eq!(fdim(1E100, -1E100), 2E100);
    assert!(fdim(f64::NAN, 0.0).is_nan());
    assert!(fdim(1.0, f64::NAN).is_nan());
    assert_eq!(fdim(f64::INFINITY, f64::NEG_INFINITY), f64::INFINITY);
    assert_eq!(fdim(f64::NEG_INFINITY, f64::INFINITY), 0.0);
    assert_eq!(fdim(f64::MAX_VALUE, f64::MIN_VALUE), f64::INFINITY);
}


// #[test]
// fn fmod_f32() {
//     // Special cases
//     assert!(fmodf(f32::NAN, 1.0).is_nan());
//     assert!(fmodf(1.0, f32::NAN).is_nan());
//     assert!(fmodf(1.0, 0.0).is_nan());
//     assert!(fmodf(f32::INFINITY, 1.0).is_nan());
//     assert!(fmodf(0.0, 0.0).is_nan());
//     assert_eq!(fmodf(0.0, 1.0), 0.0);
//     // Results in 0
//     assert_eq!(fmodf(1.0, 1.0), 0.0);
//     assert_eq!(fmodf(5.0, 2.5), 0.0);
//     assert_eq!(fmodf(25.0, 5.0), 0.0);
//
//     assert_eq!(fmodf(5.5, 3.0), 2.5);
//     assert_eq!(fmodf(842.105, 632.105), 210.0);
// }
//
// #[test]
// fn fmod_f64() {
//     // Special cases
//     assert!(fmod(f64::NAN, 1.0).is_nan());
//     assert!(fmod(1.0, f64::NAN).is_nan());
//     assert!(fmod(1.0, 0.0).is_nan());
//     assert!(fmod(f64::INFINITY, 1.0).is_nan());
//     assert!(fmod(0.0, 0.0).is_nan());
//     assert_eq!(fmod(0.0, 1.0), 0.0);
//     // Results in 0
//     assert_eq!(fmod(1.0, 1.0), 0.0);
//     assert_eq!(fmod(5.0, 2.5), 0.0);
//     assert_eq!(fmod(25.0, 5.0), 0.0);
//
//     assert_eq!(fmod(5.5, 3.0), 2.5);
//     assert_eq!(fmod(842.105, 632.105), 210.0);
// }
