#![allow(unstable)]
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
    // TODO: Issues with representability? (3rd column should be 0.0)
    assert_feq!(round( 4503599627370496.5),   4503599627370497.0, 1.0, 0);
    assert_feq!(round( 4503599627370496.75),  4503599627370497.0, 0.0, 0);
    assert_feq!(round( 4503599627370497.5),   4503599627370498.0, 0.0, 0);
    assert_feq!(round(-4503599627370495.5),  -4503599627370496.0, 0.0, 0);
    assert_feq!(round(-4503599627370496.25), -4503599627370496.0, 0.0, 0);
    // TODO: Issues with representability? (3rd column should be 0.0)
    assert_feq!(round(-4503599627370496.5),  -4503599627370497.0, 1.0, 0);
    assert_feq!(round(-4503599627370496.75), -4503599627370497.0, 0.0, 0);
    assert_feq!(round(-4503599627370497.5),  -4503599627370498.0, 0.0, 0);

    assert_feq!(round(f64::NEG_INFINITY), f64::NEG_INFINITY, 0.0, 0);
    assert_feq!(round(f64::INFINITY),     f64::INFINITY,     0.0, 0);

    assert_feq!(round( f64::NAN),  f64::NAN, 0.0, TEST_NAN_SIGN);
    assert_feq!(round(-f64::NAN), -f64::NAN, 0.0, TEST_NAN_SIGN);
}

#[test]
fn ceil_f32() {
    assert_eq!(ceilf(0.0), 0.0);
    assert_eq!(ceilf(-0.0), 0.0);
    assert_eq!(ceilf(1.0), 1.0);
    assert_eq!(ceilf(1.5), 2.0);
    assert_eq!(ceilf(-1.5), -1.0);
    assert_eq!(ceilf(-1.4), -1.0);
    assert_eq!(ceilf(-0.01), -0.0);
    assert_eq!(ceilf(0.999), 1.0);
    assert_eq!(ceilf(1.7654321E13), 1.7654321E13);
    assert_eq!(ceilf(123456789.500001), 123456790.0);
    assert_eq!(ceilf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(ceilf(f32::NAN).is_nan());
}

#[test]
fn ceil_f64() {
    assert_eq!(ceil(0.0), 0.0);
    assert_eq!(ceil(-0.0), 0.0);
    assert_eq!(ceil(1.0), 1.0);
    assert_eq!(ceil(1.5), 2.0);
    assert_eq!(ceil(-1.5), -1.0);
    assert_eq!(ceil(-1.4), -1.0);
    assert_eq!(ceil(-0.01), -0.0);
    assert_eq!(ceil(0.999), 1.0);
    assert_eq!(ceil(1.7654321E13), 1.7654321E13);
    assert_eq!(ceil(123456789.500001), 123456790.0);
    assert_eq!(ceil(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(ceil(f64::NAN).is_nan());
}

#[test]
fn floor_f32() {
    assert_eq!(floorf(0.0), 0.0);
    assert_eq!(floorf(-0.0), 0.0);
    assert_eq!(floorf(1.0), 1.0);
    assert_eq!(floorf(1.5), 1.0);
    assert_eq!(floorf(-1.5), -2.0);
    assert_eq!(floorf(-1.4), -2.0);
    assert_eq!(floorf(-0.01), -1.0);
    assert_eq!(floorf(0.999), 0.0);
    assert_eq!(floorf(1.7654321E13), 1.7654321E13);
    assert_eq!(floorf(123456789.500001), 123456789.0);
    assert_eq!(floorf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(floorf(f32::NAN).is_nan());
}

#[test]
fn floor_f64() {
    assert_eq!(floor(0.0), 0.0);
    assert_eq!(floorf(-0.0), 0.0);
    assert_eq!(floor(1.0), 1.0);
    assert_eq!(floor(1.5), 1.0);
    assert_eq!(floor(-1.5), -2.0);
    assert_eq!(floor(-1.4), -2.0);
    assert_eq!(floor(-0.01), -1.0);
    assert_eq!(floor(0.999), 0.0);
    assert_eq!(floor(1.7654321E13), 1.7654321E13);
    assert_eq!(floor(123456789.500001), 123456789.0);
    assert_eq!(floor(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(floor(f64::NAN).is_nan());
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
