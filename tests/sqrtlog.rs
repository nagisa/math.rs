#![feature(std_misc)]
extern crate math;
use std::num::Float;
use std::{f32, f64};

use math::{sqrtf, sqrt};
use math::{expf, exp};
use testutils::*;

#[macro_use]
mod testutils;

#[test]
fn sqrt_f32() {
    assert_feq!(sqrtf(0.0),      0.0,                       0.0,   TEST_ZERO_SIGN);
    assert_feq!(sqrtf(4.0),      2.0,                       0.0,   0);
    assert_feq!(sqrtf(9.0),      3.0,                       0.0,   0);
    assert_feq!(sqrtf(20.25),    4.5,                       0.0,   0);
    assert_feq!(sqrtf(0.065536), 0.256,                     0.0,   0);
    assert_feq!(sqrtf(1.4E-45),  3.7416573867739413855E-23, 1E-25, 0);
    assert_feq!(sqrtf(f32::NAN),           f32::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(sqrtf(f32::INFINITY),      f32::INFINITY, 0.0, 0);
    assert_feq!(sqrtf(-1.0),              -f32::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(sqrtf(f32::NEG_INFINITY), -f32::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(sqrtf(f32::MIN_VALUE),    -f32::NAN,      0.0, TEST_NAN_SIGN);
}

#[test]
fn sqrt_f64() {
    assert_feq!(sqrt(0.0),      0.0,                       0.0,   TEST_ZERO_SIGN);
    assert_feq!(sqrt(4.0),      2.0,                       0.0,   0);
    assert_feq!(sqrt(9.0),      3.0,                       0.0,   0);
    assert_feq!(sqrt(20.25),    4.5,                       0.0,   0);
    assert_feq!(sqrt(0.065536), 0.256,                     0.0,   0);
    assert_feq!(sqrt(1.4E-45),  3.741657386773941385583748732316549301756019807E-23, 1E-35, 0);

    assert_feq!(sqrt(4.94065645841246544176568792868E-324), 2.22275874948507748344271341427006E-162,
                1E-35, 0);

    assert_feq!(sqrt(f64::NAN),           f64::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(sqrt(f64::INFINITY),      f64::INFINITY, 0.0, 0);
    assert_feq!(sqrt(-1.0),              -f64::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(sqrt(f64::NEG_INFINITY), -f64::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(sqrt(f64::MIN_VALUE),    -f64::NAN,      0.0, TEST_NAN_SIGN);
}

#[test]
fn exp_f32() {
    assert_feq!(expf(0.0),          1.0, 0.0, 0);
    assert_feq!(expf(0.6931471806), 2.0, 0.0, 0);

    assert_feq!(expf(f32::NAN),          f32::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(expf(-f32::NAN),        -f32::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(expf(f32::INFINITY),     f32::INFINITY, 0.0, 0);
    assert_feq!(expf(f32::NEG_INFINITY), 0.0,           0.0, TEST_ZERO_SIGN);
    assert_feq!(expf(89.0),              f32::INFINITY, 0.0, 0);
    assert_feq!(expf(-104.0),            0.0,           0.0, TEST_ZERO_SIGN);
}

#[test]
fn exp_f64() {
    assert_feq!(exp(0.0),                                       1.0, 0.0, 0);
    assert_feq!(exp(0.693147180559945309417232121458176568075), 2.0, 0.0, 0);

    assert_feq!(exp(f64::NAN),          f64::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(exp(-f64::NAN),        -f64::NAN,      0.0, TEST_NAN_SIGN);
    assert_feq!(exp(f64::INFINITY),     f64::INFINITY, 0.0, 0);
    assert_feq!(exp(f64::NEG_INFINITY), 0.0,           0.0, TEST_ZERO_SIGN);
    assert_feq!(exp(710.0),             f64::INFINITY, 0.0, 0);
    assert_feq!(exp(-746.0),            0.0,           0.0, TEST_ZERO_SIGN);
}
