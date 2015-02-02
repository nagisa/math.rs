#![feature(core, std_misc)]
extern crate math;

use std::num::Float;
use std::{f32, f64};
use math::{fmaxf, fmax, fminf, fmin, hypotf, hypot};
use testutils::*;

#[macro_use]
mod testutils;

#[test]
fn max_f32() {
    assert_feq!(fmaxf( 0.0,  0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmaxf(-0.0, -0.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmaxf( 9.0,  0.0),  9.0, 0.0, 0);
    assert_feq!(fmaxf(-9.0, -0.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmaxf( 0.0,  9.0),  9.0, 0.0, 0);
    assert_feq!(fmaxf(-0.0, -9.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmaxf( 0.0,  9.0),  9.0, 0.0, 0);
    assert_feq!(fmaxf(-0.0, -9.0), -0.0, 0.0, TEST_ZERO_SIGN);


    assert_feq!(fmaxf( f32::INFINITY,  9.0),             f32::INFINITY, 0.0, 0);
    assert_feq!(fmaxf( 9.0,            f32::INFINITY),   f32::INFINITY, 0.0, 0);
    assert_feq!(fmaxf( f32::INFINITY, -9.0),             f32::INFINITY, 0.0, 0);
    assert_feq!(fmaxf(-9.0,            f32::INFINITY),   f32::INFINITY, 0.0, 0);

    assert_feq!(fmaxf( f32::NEG_INFINITY,  9.0),                9.0, 0.0, 0);
    assert_feq!(fmaxf( 9.0,                f32::NEG_INFINITY),  9.0, 0.0, 0);
    assert_feq!(fmaxf( f32::NEG_INFINITY, -9.0),               -9.0, 0.0, 0);
    assert_feq!(fmaxf(-9.0,                f32::NEG_INFINITY), -9.0, 0.0, 0);

    assert_feq!(fmaxf( f32::NAN,  9.0),       9.0,      0.0, 0);
    assert_feq!(fmaxf( f32::NAN, -9.0),      -9.0,      0.0, 0);
    assert_feq!(fmaxf( 9.0,       f32::NAN),  9.0,      0.0, 0);
    assert_feq!(fmaxf(-9.0,       f32::NAN), -9.0,      0.0, 0);
    assert_feq!(fmaxf( f32::NAN,  f32::NAN),  f32::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn max_f64() {
    assert_feq!(fmax( 0.0,  0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmax(-0.0, -0.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmax( 9.0,  0.0),  9.0, 0.0, 0);
    assert_feq!(fmax(-9.0, -0.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmax( 0.0,  9.0),  9.0, 0.0, 0);
    assert_feq!(fmax(-0.0, -9.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmax( 0.0,  9.0),  9.0, 0.0, 0);
    assert_feq!(fmax(-0.0, -9.0), -0.0, 0.0, TEST_ZERO_SIGN);


    assert_feq!(fmax( f64::INFINITY,  9.0),             f64::INFINITY, 0.0, 0);
    assert_feq!(fmax( 9.0,            f64::INFINITY),   f64::INFINITY, 0.0, 0);
    assert_feq!(fmax( f64::INFINITY, -9.0),             f64::INFINITY, 0.0, 0);
    assert_feq!(fmax(-9.0,            f64::INFINITY),   f64::INFINITY, 0.0, 0);

    assert_feq!(fmax( f64::NEG_INFINITY,  9.0),                9.0, 0.0, 0);
    assert_feq!(fmax( 9.0,                f64::NEG_INFINITY),  9.0, 0.0, 0);
    assert_feq!(fmax( f64::NEG_INFINITY, -9.0),               -9.0, 0.0, 0);
    assert_feq!(fmax(-9.0,                f64::NEG_INFINITY), -9.0, 0.0, 0);

    assert_feq!(fmax( f64::NAN,  9.0),       9.0,      0.0, 0);
    assert_feq!(fmax( f64::NAN, -9.0),      -9.0,      0.0, 0);
    assert_feq!(fmax( 9.0,       f64::NAN),  9.0,      0.0, 0);
    assert_feq!(fmax(-9.0,       f64::NAN), -9.0,      0.0, 0);
    assert_feq!(fmax( f64::NAN,  f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn min_f32() {
    assert_feq!(fminf( 0.0,  0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fminf(-0.0, -0.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fminf( 9.0,  0.0),  0.0, 0.0, 0);
    assert_feq!(fminf(-9.0, -0.0), -9.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fminf( 0.0,  9.0),  0.0, 0.0, 0);
    assert_feq!(fminf(-0.0, -9.0), -9.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fminf( 0.0,  9.0),  0.0, 0.0, 0);
    assert_feq!(fminf(-0.0, -9.0), -9.0, 0.0, TEST_ZERO_SIGN);


    assert_feq!(fminf( f32::INFINITY,  9.0),             9.0, 0.0, 0);
    assert_feq!(fminf( 9.0,            f32::INFINITY),   9.0, 0.0, 0);
    assert_feq!(fminf( f32::INFINITY, -9.0),            -9.0, 0.0, 0);
    assert_feq!(fminf(-9.0,            f32::INFINITY),  -9.0, 0.0, 0);

    assert_feq!(fminf( f32::NEG_INFINITY,  9.0),               f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(fminf( 9.0,                f32::NEG_INFINITY), f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(fminf( f32::NEG_INFINITY, -9.0),               f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(fminf(-9.0,                f32::NEG_INFINITY), f32::NEG_INFINITY, 0.0, 0);

    assert_feq!(fminf( f32::NAN,  9.0),       9.0,      0.0, 0);
    assert_feq!(fminf( f32::NAN, -9.0),      -9.0,      0.0, 0);
    assert_feq!(fminf( 9.0,       f32::NAN),  9.0,      0.0, 0);
    assert_feq!(fminf(-9.0,       f32::NAN), -9.0,      0.0, 0);
    assert_feq!(fminf( f32::NAN,  f32::NAN),  f32::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn min_f64() {
    assert_feq!(fmin( 0.0,  0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmin(-0.0, -0.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmin( 9.0,  0.0),  0.0, 0.0, 0);
    assert_feq!(fmin(-9.0, -0.0), -9.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmin( 0.0,  9.0),  0.0, 0.0, 0);
    assert_feq!(fmin(-0.0, -9.0), -9.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fmin( 0.0,  9.0),  0.0, 0.0, 0);
    assert_feq!(fmin(-0.0, -9.0), -9.0, 0.0, TEST_ZERO_SIGN);


    assert_feq!(fmin( f64::INFINITY,  9.0),             9.0, 0.0, 0);
    assert_feq!(fmin( 9.0,            f64::INFINITY),   9.0, 0.0, 0);
    assert_feq!(fmin( f64::INFINITY, -9.0),            -9.0, 0.0, 0);
    assert_feq!(fmin(-9.0,            f64::INFINITY),  -9.0, 0.0, 0);

    assert_feq!(fmin( f64::NEG_INFINITY,  9.0),               f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(fmin( 9.0,                f64::NEG_INFINITY), f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(fmin( f64::NEG_INFINITY, -9.0),               f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(fmin(-9.0,                f64::NEG_INFINITY), f64::NEG_INFINITY, 0.0, 0);

    assert_feq!(fmin( f64::NAN,  9.0),       9.0,      0.0, 0);
    assert_feq!(fmin( f64::NAN, -9.0),      -9.0,      0.0, 0);
    assert_feq!(fmin( 9.0,       f64::NAN),  9.0,      0.0, 0);
    assert_feq!(fmin(-9.0,       f64::NAN), -9.0,      0.0, 0);
    assert_feq!(fmin( f64::NAN,  f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn hypot_f32() {
    assert_feq!(hypotf(f32::consts::PI, 0.0),  f32::consts::PI, 0.0, 0);
    assert_feq!(hypotf(0.0, -f32::consts::PI), f32::consts::PI, 0.0, 0);
    assert_feq!(hypotf(3.0, 4.0),              5.0,             0.0, 0);
    assert_feq!(hypotf(-3.0, -4.0),            5.0,             0.0, 0);

    assert_feq!(hypotf(f32::INFINITY, 0.0),     f32::INFINITY, 0.0, 0);
    assert_feq!(hypotf(f32::NEG_INFINITY, 0.0), f32::INFINITY, 0.0, 0);
    assert_feq!(hypotf(0.0, f32::INFINITY),     f32::INFINITY, 0.0, 0);
    assert_feq!(hypotf(0.0, f32::NEG_INFINITY), f32::INFINITY, 0.0, 0);

    assert_feq!(hypotf( f32::INFINITY, f32::NAN),     f32::INFINITY, 0.0, 0);
    assert_feq!(hypotf( f32::NAN, f32::INFINITY),     f32::INFINITY, 0.0, 0);
    assert_feq!(hypotf( f32::NEG_INFINITY, f32::NAN), f32::INFINITY, 0.0, 0);
    assert_feq!(hypotf( f32::NAN, f32::NEG_INFINITY), f32::INFINITY, 0.0, 0);
    // Sign of NaN is not specified
    assert_feq!(hypotf( f32::NAN,  0.0),              f32::NAN,      0.0, 0);
    assert_feq!(hypotf(-f32::NAN,  0.0),              f32::NAN,      0.0, 0);
    assert_feq!(hypotf( f32::NAN, -f32::NAN),         f32::NAN,      0.0, 0);
}

#[test]
fn hypot_f64() {
    assert_feq!(hypot(f64::consts::PI, 0.0),  f64::consts::PI, 0.0, 0);
    assert_feq!(hypot(0.0, -f64::consts::PI), f64::consts::PI, 0.0, 0);
    assert_feq!(hypot(3.0, 4.0),              5.0,             0.0, 0);
    assert_feq!(hypot(-3.0, -4.0),            5.0,             0.0, 0);

    assert_feq!(hypot(f64::INFINITY, 0.0),     f64::INFINITY, 0.0, 0);
    assert_feq!(hypot(f64::NEG_INFINITY, 0.0), f64::INFINITY, 0.0, 0);
    assert_feq!(hypot(0.0, f64::INFINITY),     f64::INFINITY, 0.0, 0);
    assert_feq!(hypot(0.0, f64::NEG_INFINITY), f64::INFINITY, 0.0, 0);

    assert_feq!(hypot( f64::INFINITY, f64::NAN),     f64::INFINITY, 0.0, 0);
    assert_feq!(hypot( f64::NAN, f64::INFINITY),     f64::INFINITY, 0.0, 0);
    assert_feq!(hypot( f64::NEG_INFINITY, f64::NAN), f64::INFINITY, 0.0, 0);
    assert_feq!(hypot( f64::NAN, f64::NEG_INFINITY), f64::INFINITY, 0.0, 0);
    // Sign of NaN is not specified
    assert_feq!(hypot( f64::NAN,  0.0),              f64::NAN,      0.0, 0);
    assert_feq!(hypot(-f64::NAN,  0.0),              f64::NAN,      0.0, 0);
    assert_feq!(hypot( f64::NAN, -f64::NAN),         f64::NAN,      0.0, 0);
}
