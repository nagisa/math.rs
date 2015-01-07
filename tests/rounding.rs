extern crate math;
use math::{roundf,round,ceilf,ceil,floorf,floor,truncf,trunc};

#[test]
fn round_f32() {
    use std::f32;
    use std::num::Float;
    assert_eq!(roundf(0.0), 0.0);
    assert_eq!(roundf(1.0), 1.0);
    assert_eq!(roundf(1.5), 2.0);
    assert_eq!(roundf(-1.5), -2.0);
    assert_eq!(roundf(-1.4), -1.0);
    assert_eq!(roundf(-0.01), 0.0);
    assert_eq!(roundf(1.7654321E13), 1.7654321E13);
    assert_eq!(roundf(123456789.500001), 123456790.0);
    assert_eq!(roundf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(roundf(f32::NAN).is_nan());
}

#[test]
fn round_f64() {
    use std::f64;
    use std::num::Float;
    assert_eq!(round(0.0), 0.0);
    assert_eq!(round(1.0), 1.0);
    assert_eq!(round(1.5), 2.0);
    assert_eq!(round(-1.5), -2.0);
    assert_eq!(round(-1.4), -1.0);
    assert_eq!(round(-0.01), 0.0);
    assert_eq!(round(1.7654321E13), 1.7654321E13);
    assert_eq!(round(123456789.500001), 123456790.0);
    assert_eq!(round(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(round(f64::NAN).is_nan());
}

#[test]
fn ceil_f32() {
    use std::f32;
    use std::num::Float;
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
    use std::f64;
    use std::num::Float;
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
    use std::f32;
    use std::num::Float;
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
    use std::f64;
    use std::num::Float;
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
    use std::f32;
    use std::num::Float;
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
    use std::f64;
    use std::num::Float;
    assert_eq!(trunc(1.1341241), 1.0);
    assert_eq!(trunc(0.01), 0.0);
    assert_eq!(trunc(-0.01), -0.0);
    assert_eq!(trunc(100.230), 100.0);
    assert_eq!(trunc(123456789.123456789), 123456789.0);
    assert_eq!(trunc(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(trunc(f64::NAN).is_nan());
}


// #[test]
// fn lround_f32() {
//     use std::f32;
//     use std::num::Float;
//     assert_eq!(lroundf(0.0), 0);
//     assert_eq!(lroundf(1.0), 1);
//     assert_eq!(lroundf(1.6), 2);
//     assert_eq!(lroundf(-1.5), -2);
//     assert_eq!(lroundf(-1.4), -1);
//     assert_eq!(lroundf(-0.01), 0);
//     // Float can’t hold this value exactly, but it still fits into integer.
//     // Allow to be off from expected result by at most 1
//     assert!(lroundf(2147483647.0) - 2147483646 <= 2);
// }
