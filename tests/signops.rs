extern crate math;

use std::{f64, f32};
use math::*;
use testutils::*;

#[macro_use]
mod testutils;

#[test]
fn abs_f32() {
    assert_feq!(fabsf(0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fabsf(-0.0), 0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(fabsf(f32::INFINITY),     f32::INFINITY, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fabsf(f32::NEG_INFINITY), f32::INFINITY, 0.0, TEST_ZERO_SIGN);

    assert_feq!(fabsf(f32::NAN),  f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(fabsf(-f32::NAN), f32::NAN, 0.0, TEST_NAN_SIGN);

    assert_feq!(fabsf(42.0f32),       42.0f32,      0.0, 0);
    assert_feq!(fabsf(-f32::EPSILON), f32::EPSILON, 0.0, 0);
}

#[test]
fn abs_f64() {
    assert_feq!(fabs(0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(fabs(-0.0), 0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(fabs(f64::INFINITY),     f64::INFINITY, 0.0, 0);
    assert_feq!(fabs(f64::NEG_INFINITY), f64::INFINITY, 0.0, 0);

    assert_feq!(fabs(f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(fabs(-f64::NAN), f64::NAN, 0.0, TEST_NAN_SIGN);

    assert_feq!(fabs(42.0f64),       42.0f64,      0.0, 0);
    assert_feq!(fabs(-f64::EPSILON), f64::EPSILON, 0.0, 0);
}

#[test]
fn copysign_f32() {
    assert_feq!(copysignf( 0.0,  4.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf( 0.0, -4.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf(-0.0,  4.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf(-0.0, -4.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(copysignf(f32::INFINITY,      0.0), f32::INFINITY,     0.0, 0);
    assert_feq!(copysignf(f32::NEG_INFINITY,  0.0), f32::INFINITY,     0.0, 0);
    assert_feq!(copysignf(f32::INFINITY,     -0.0), f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(copysignf(f32::NEG_INFINITY, -0.0), f32::NEG_INFINITY, 0.0, 0);

    assert_feq!(copysignf( 0.0, f32::INFINITY),      0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf(-0.0, f32::INFINITY),      0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf( 0.0, f32::NEG_INFINITY), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf(-0.0, f32::NEG_INFINITY), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(copysignf( 0.0,  f32::NAN),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf(-0.0,  f32::NAN),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf( 0.0, -f32::NAN), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysignf(-0.0, -f32::NAN), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(copysignf( f32::NAN,  0.0),  f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(copysignf(-f32::NAN,  0.0),  f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(copysignf( f32::NAN, -0.0), -f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(copysignf(-f32::NAN, -0.0), -f32::NAN, 0.0, TEST_NAN_SIGN);

    assert_feq!(copysignf( 1.0,  2.0),  1.0, 0.0, 0);
    assert_feq!(copysignf( 1.0, -2.0), -1.0, 0.0, 0);
    assert_feq!(copysignf(-1.0,  2.0),  1.0, 0.0, 0);
    assert_feq!(copysignf(-1.0, -2.0), -1.0, 0.0, 0);
}

#[test]
fn copysign_f64() {
    assert_feq!(copysign( 0.0,  4.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign( 0.0, -4.0), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign(-0.0,  4.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign(-0.0, -4.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(copysign(f64::INFINITY,      0.0), f64::INFINITY,     0.0, 0);
    assert_feq!(copysign(f64::NEG_INFINITY,  0.0), f64::INFINITY,     0.0, 0);
    assert_feq!(copysign(f64::INFINITY,     -0.0), f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(copysign(f64::NEG_INFINITY, -0.0), f64::NEG_INFINITY, 0.0, 0);

    assert_feq!(copysign( 0.0, f64::INFINITY),      0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign(-0.0, f64::INFINITY),      0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign( 0.0, f64::NEG_INFINITY), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign(-0.0, f64::NEG_INFINITY), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(copysign( 0.0,  f64::NAN),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign(-0.0,  f64::NAN),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign( 0.0, -f64::NAN), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(copysign(-0.0, -f64::NAN), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(copysign( f64::NAN,  0.0),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(copysign(-f64::NAN,  0.0),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(copysign( f64::NAN, -0.0), -f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(copysign(-f64::NAN, -0.0), -f64::NAN, 0.0, TEST_NAN_SIGN);

    assert_feq!(copysign( 1.0,  2.0),  1.0, 0.0, 0);
    assert_feq!(copysign( 1.0, -2.0), -1.0, 0.0, 0);
    assert_feq!(copysign(-1.0,  2.0),  1.0, 0.0, 0);
    assert_feq!(copysign(-1.0, -2.0), -1.0, 0.0, 0);
}
