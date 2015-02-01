#![feature(std_misc, core)]
extern crate math;

use std::{f32, f64};
use std::num::Float;
use testutils::*;

#[macro_use]
mod testutils;

use math::{roundf,round,ceilf,ceil,floorf,floor,truncf,trunc,lroundf,lround,llroundf,llround};

#[test]
fn round_f32() {
    assert_feq!(roundf( 0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf(-0.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(roundf( 0.2),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf(-0.2), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf( 0.5),  1.0, 0.0, 0);
    assert_feq!(roundf(-0.5), -1.0, 0.0, 0);
    assert_feq!(roundf( 0.8),  1.0, 0.0, 0);
    assert_feq!(roundf(-0.8), -1.0, 0.0, 0);
    assert_feq!(roundf( 1.5),  2.0, 0.0, 0);
    assert_feq!(roundf(-1.5), -2.0, 0.0, 0);

    assert_feq!(roundf( 0.1),    0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf( 0.25),   0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf( 0.625),  1.0, 0.0, 0);
    assert_feq!(roundf(-0.1),   -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf(-0.25),  -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(roundf(-0.625), -1.0, 0.0, 0);

    assert_feq!(roundf( 2097152.5),  2097153.0, 0.0, 0);
    assert_feq!(roundf(-2097152.5), -2097153.0, 0.0, 0);

    assert_feq!(roundf(f32::NEG_INFINITY), f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(roundf(f32::INFINITY),     f32::INFINITY,     0.0, 0);

    assert_feq!(roundf( f32::NAN),  f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(roundf(-f32::NAN), -f32::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn round_f64() {
    assert_feq!(round( 0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round(-0.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(round( 0.2),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round(-0.2), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round( 0.5),  1.0, 0.0, 0);
    assert_feq!(round(-0.5), -1.0, 0.0, 0);
    assert_feq!(round( 0.8),  1.0, 0.0, 0);
    assert_feq!(round(-0.8), -1.0, 0.0, 0);
    assert_feq!(round( 1.5),  2.0, 0.0, 0);
    assert_feq!(round(-1.5), -2.0, 0.0, 0);

    assert_feq!(round( 0.1),    0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round( 0.25),   0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round( 0.625),  1.0, 0.0, 0);
    assert_feq!(round(-0.1),   -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round(-0.25),  -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(round(-0.625), -1.0, 0.0, 0);

    assert_feq!(round( 2097152.5),  2097153.0, 0.0, 0);
    assert_feq!(round(-2097152.5), -2097153.0, 0.0, 0);

    assert_feq!(round( 4503599627370495.5),   4503599627370496.0, 0.0, 0);
    assert_feq!(round( 4503599627370496.25),  4503599627370496.0, 0.0, 0);
    assert_feq!(round( 4503599627370496.75),  4503599627370497.0, 0.0, 0);
    assert_feq!(round( 4503599627370497.5),   4503599627370498.0, 0.0, 0);
    assert_feq!(round(-4503599627370495.5),  -4503599627370496.0, 0.0, 0);
    assert_feq!(round(-4503599627370496.25), -4503599627370496.0, 0.0, 0);
    assert_feq!(round(-4503599627370496.75), -4503599627370497.0, 0.0, 0);
    assert_feq!(round(-4503599627370497.5),  -4503599627370498.0, 0.0, 0);
    // TODO: Issues with representability? (3rd column should be 0.0)
    assert_feq!(round( 4503599627370496.5),   4503599627370497.0, 1.0, 0);
    assert_feq!(round(-4503599627370496.5),  -4503599627370497.0, 1.0, 0);

    assert_feq!(round(f64::NEG_INFINITY), f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(round(f64::INFINITY),     f64::INFINITY,     0.0, 0);

    assert_feq!(round( f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(round(-f64::NAN), -f64::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn ceil_f32() {
    assert_feq!(ceilf( 0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceilf(-0.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(ceilf( f32::consts::PI),  4.0, 0.0, 0);
    assert_feq!(ceilf(-f32::consts::PI), -3.0, 0.0, 0);

    assert_feq!(ceilf( F32_MIN_SUBNORM),     1.0, 0.0, 0);
    assert_feq!(ceilf(-F32_MIN_SUBNORM),    -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceilf( f32::MIN_POS_VALUE),  1.0, 0.0, 0);
    assert_feq!(ceilf(-f32::MIN_POS_VALUE), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceilf(f32::MAX_VALUE),       f32::MAX_VALUE, 0.0, 0);
    assert_feq!(ceilf(f32::MIN_VALUE),       f32::MIN_VALUE, 0.0, 0);

    assert_feq!(ceilf( 0.1),                 1.0, 0.0, 0);
    assert_feq!(ceilf( 0.25),                1.0, 0.0, 0);
    assert_feq!(ceilf( 0.625),               1.0, 0.0, 0);
    assert_feq!(ceilf(-0.1),                -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceilf(-0.25),               -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceilf(-0.625),              -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(ceilf(f32::NEG_INFINITY), f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(ceilf(f32::INFINITY),     f32::INFINITY,     0.0, 0);

    assert_feq!(ceilf( f32::NAN),  f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(ceilf(-f32::NAN), -f32::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn ceil_f64() {
    assert_feq!(ceil( 0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceil(-0.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(ceil( f64::consts::PI),  4.0, 0.0, 0);
    assert_feq!(ceil(-f64::consts::PI), -3.0, 0.0, 0);

    assert_feq!(ceil( F64_MIN_SUBNORM),     1.0, 0.0, 0);
    assert_feq!(ceil(-F64_MIN_SUBNORM),    -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceil( f64::MIN_POS_VALUE),  1.0, 0.0, 0);
    assert_feq!(ceil(-f64::MIN_POS_VALUE), -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceil(f64::MAX_VALUE),       f64::MAX_VALUE, 0.0, 0);
    assert_feq!(ceil(f64::MIN_VALUE),       f64::MIN_VALUE, 0.0, 0);

    assert_feq!(ceil( 0.1),                 1.0, 0.0, 0);
    assert_feq!(ceil( 0.25),                1.0, 0.0, 0);
    assert_feq!(ceil( 0.625),               1.0, 0.0, 0);
    assert_feq!(ceil(-0.1),                -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceil(-0.25),               -0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(ceil(-0.625),              -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(ceil(f64::NEG_INFINITY), f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(ceil(f64::INFINITY),     f64::INFINITY,     0.0, 0);

    assert_feq!(ceil( f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(ceil(-f64::NAN), -f64::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn floor_f32() {
    assert_feq!(floorf( 0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floorf(-0.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(floorf( f32::consts::PI),  3.0, 0.0, 0);
    assert_feq!(floorf(-f32::consts::PI), -4.0, 0.0, 0);

    assert_feq!(floorf( F32_MIN_SUBNORM),     0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floorf(-F32_MIN_SUBNORM),    -1.0, 0.0, 0);
    assert_feq!(floorf( f32::MIN_POS_VALUE),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floorf(-f32::MIN_POS_VALUE), -1.0, 0.0, 0);
    assert_feq!(floorf(f32::MAX_VALUE),       f32::MAX_VALUE, 0.0, 0);
    assert_feq!(floorf(f32::MIN_VALUE),       f32::MIN_VALUE, 0.0, 0);

    assert_feq!(floorf( 0.1),                 0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floorf( 0.25),                0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floorf( 0.625),               0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floorf(-0.1),                -1.0, 0.0, 0);
    assert_feq!(floorf(-0.25),               -1.0, 0.0, 0);
    assert_feq!(floorf(-0.625),              -1.0, 0.0, 0);

    assert_feq!(floorf(f32::NEG_INFINITY), f32::NEG_INFINITY, 0.0, 0);
    assert_feq!(floorf(f32::INFINITY),     f32::INFINITY,     0.0, 0);

    assert_feq!(floorf( f32::NAN),  f32::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(floorf(-f32::NAN), -f32::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn floor_f64() {
    assert_feq!(floor( 0.0),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floor(-0.0), -0.0, 0.0, TEST_ZERO_SIGN);

    assert_feq!(floor( f64::consts::PI),  3.0, 0.0, 0);
    assert_feq!(floor(-f64::consts::PI), -4.0, 0.0, 0);

    assert_feq!(floor( F64_MIN_SUBNORM),     0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floor(-F64_MIN_SUBNORM),    -1.0, 0.0, 0);
    assert_feq!(floor( f64::MIN_POS_VALUE),  0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floor(-f64::MIN_POS_VALUE), -1.0, 0.0, 0);
    assert_feq!(floor(f64::MAX_VALUE),       f64::MAX_VALUE, 0.0, 0);
    assert_feq!(floor(f64::MIN_VALUE),       f64::MIN_VALUE, 0.0, 0);

    assert_feq!(floor( 0.1),                 0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floor( 0.25),                0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floor( 0.625),               0.0, 0.0, TEST_ZERO_SIGN);
    assert_feq!(floor(-0.1),                -1.0, 0.0, 0);
    assert_feq!(floor(-0.25),               -1.0, 0.0, 0);
    assert_feq!(floor(-0.625),              -1.0, 0.0, 0);

    assert_feq!(floor(f64::NEG_INFINITY), f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(floor(f64::INFINITY),     f64::INFINITY,     0.0, 0);

    assert_feq!(floor( f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(floor(-f64::NAN), -f64::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn trunc_f32() {
    assert_eq!(truncf(1.1341241), 1.0);
    assert_eq!(truncf(0.01), 0.0);
    assert_eq!(truncf(-0.01), -0.0);
    assert_eq!(truncf(100.230), 100.0);
    assert_eq!(truncf(123456789.123456789), 123456789.0);
    assert_eq!(truncf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(truncf(f32::NAN).is_nan());
}

#[test]
fn trunc_f64() {
    assert_eq!(trunc(1.1341241), 1.0);
    assert_eq!(trunc(0.01), 0.0);
    assert_eq!(trunc(-0.01), -0.0);
    assert_eq!(trunc(100.230), 100.0);
    assert_eq!(trunc(123456789.123456789), 123456789.0);
    assert_eq!(trunc(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(trunc(f64::NAN).is_nan());
}


#[test]
fn lround_f32() {
    assert_eq!(lroundf(0.0), 0);
    assert_eq!(lroundf(1.0), 1);
    assert_eq!(lroundf(1.6), 2);
    assert_eq!(lroundf(-1.5), -2);
    assert_eq!(lroundf(-1.4), -1);
    assert_eq!(lroundf(-0.01), 0);
    // f32 can’t hold this value exactly, but it still fits into 32-bit integer.
    // Allow to be off from expected result by at most 1
    assert!(lroundf(2147483647.0) - 2147483646 <= 2);
}

#[test]
fn llround_f32() {
    // Exactly the same thing as lround_f32…
    assert_eq!(llroundf(0.0), 0);
    assert_eq!(llroundf(1.0), 1);
    assert_eq!(llroundf(1.6), 2);
    assert_eq!(llroundf(-1.5), -2);
    assert_eq!(llroundf(-1.4), -1);
    assert_eq!(llroundf(-0.01), 0);
    // f32 can’t hold this value exactly, but it still fits into integer.
    // Allow to be off from expected result by at most 1
    assert!(llroundf(2147483647.0) - 2147483646 <= 2);
}

#[test]
fn lround_f64() {
    assert_eq!(lround(0.0), 0);
    assert_eq!(lround(1.0), 1);
    assert_eq!(lround(1.6), 2);
    assert_eq!(lround(-1.5), -2);
    assert_eq!(lround(-1.4), -1);
    assert_eq!(lround(-0.01), 0);
    // f64, on the other hand, can hold the value exactly.
    assert_eq!(lround(2147483647.0), 2147483647);
}

#[test]
fn llround_f64() {
    assert_eq!(llround(0.0), 0);
    assert_eq!(llround(1.0), 1);
    assert_eq!(llround(1.6), 2);
    assert_eq!(llround(-1.5), -2);
    assert_eq!(llround(-1.4), -1);
    assert_eq!(llround(-0.01), 0);
    // f64, on the other hand, can hold the value exactly.
    assert_eq!(llround(2147483647.0), 2147483647);
}
