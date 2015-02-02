#![feature(std_misc)]
extern crate math;
use std::num::Float;
use std::{f32, f64};

use math::{fdimf, fdim};
// use math::{fmodf, fmod};
use testutils::*;


#[macro_use]
mod testutils;


#[test]
fn dim_f32() {
    assert_feq!(fdimf( 0.0,  0.0), 0.0, 0.0, 0);
    assert_feq!(fdimf( 9.0,  0.0), 9.0, 0.0, 0);
    assert_feq!(fdimf( 0.0,  9.0), 0.0, 0.0, 0);
    assert_feq!(fdimf(-9.0,  0.0), 0.0, 0.0, 0);
    assert_feq!(fdimf( 0.0, -9.0), 9.0, 0.0, 0);

    assert_feq!(fdimf(f32::INFINITY, 9.0),                   f32::INFINITY, 0.0, 0);
    assert_feq!(fdimf(f32::INFINITY, -9.0),                  f32::INFINITY, 0.0, 0);
    assert_feq!(fdimf(f32::NEG_INFINITY, 9.0),               0.0,           0.0, 0);
    assert_feq!(fdimf(f32::NEG_INFINITY, -9.0),              0.0,           0.0, 0);
    assert_feq!(fdimf( 9.0, f32::NEG_INFINITY),              f32::INFINITY, 0.0, 0);
    assert_feq!(fdimf(-9.0, f32::NEG_INFINITY),              f32::INFINITY, 0.0, 0);
    assert_feq!(fdimf( 9.0, f32::INFINITY),                  0.0,           0.0, 0);
    assert_feq!(fdimf(-9.0, f32::INFINITY),                  0.0,           0.0, 0);
    assert_feq!(fdimf(f32::INFINITY, f32::INFINITY),         0.0,           0.0, 0);
    assert_feq!(fdimf(f32::INFINITY, f32::NEG_INFINITY),     f32::INFINITY, 0.0, 0);
    assert_feq!(fdimf(f32::NEG_INFINITY, f32::INFINITY),     0.0,           0.0, 0);
    assert_feq!(fdimf(f32::NEG_INFINITY, f32::NEG_INFINITY), 0.0,           0.0, 0);

    assert_feq!(fdimf(0.0, f32::NAN),               f32::NAN, 0.0, 0);
    assert_feq!(fdimf(9.0, f32::NAN),               f32::NAN, 0.0, 0);
    assert_feq!(fdimf(-9.0, f32::NAN),              f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::INFINITY, f32::NAN),     f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NEG_INFINITY, f32::NAN), f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NAN, 0.0),               f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NAN, 9.0),               f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NAN, -9.0),              f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NAN, f32::INFINITY),     f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NAN, f32::NEG_INFINITY), f32::NAN, 0.0, 0);
    assert_feq!(fdimf(f32::NAN, f32::NAN),          f32::NAN, 0.0, 0);
}


#[test]
fn dim_f64() {
    assert_feq!(fdim( 0.0,  0.0), 0.0, 0.0, 0);
    assert_feq!(fdim( 9.0,  0.0), 9.0, 0.0, 0);
    assert_feq!(fdim( 0.0,  9.0), 0.0, 0.0, 0);
    assert_feq!(fdim(-9.0,  0.0), 0.0, 0.0, 0);
    assert_feq!(fdim( 0.0, -9.0), 9.0, 0.0, 0);

    assert_feq!(fdim(f64::INFINITY, 9.0),                   f64::INFINITY, 0.0, 0);
    assert_feq!(fdim(f64::INFINITY, -9.0),                  f64::INFINITY, 0.0, 0);
    assert_feq!(fdim(f64::NEG_INFINITY, 9.0),               0.0,           0.0, 0);
    assert_feq!(fdim(f64::NEG_INFINITY, -9.0),              0.0,           0.0, 0);
    assert_feq!(fdim( 9.0, f64::NEG_INFINITY),              f64::INFINITY, 0.0, 0);
    assert_feq!(fdim(-9.0, f64::NEG_INFINITY),              f64::INFINITY, 0.0, 0);
    assert_feq!(fdim( 9.0, f64::INFINITY),                  0.0,           0.0, 0);
    assert_feq!(fdim(-9.0, f64::INFINITY),                  0.0,           0.0, 0);
    assert_feq!(fdim(f64::INFINITY, f64::INFINITY),         0.0,           0.0, 0);
    assert_feq!(fdim(f64::INFINITY, f64::NEG_INFINITY),     f64::INFINITY, 0.0, 0);
    assert_feq!(fdim(f64::NEG_INFINITY, f64::INFINITY),     0.0,           0.0, 0);
    assert_feq!(fdim(f64::NEG_INFINITY, f64::NEG_INFINITY), 0.0,           0.0, 0);

    assert_feq!(fdim(0.0, f64::NAN),               f64::NAN, 0.0, 0);
    assert_feq!(fdim(9.0, f64::NAN),               f64::NAN, 0.0, 0);
    assert_feq!(fdim(-9.0, f64::NAN),              f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::INFINITY, f64::NAN),     f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NEG_INFINITY, f64::NAN), f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NAN, 0.0),               f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NAN, 9.0),               f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NAN, -9.0),              f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NAN, f64::INFINITY),     f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NAN, f64::NEG_INFINITY), f64::NAN, 0.0, 0);
    assert_feq!(fdim(f64::NAN, f64::NAN),          f64::NAN, 0.0, 0);
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
