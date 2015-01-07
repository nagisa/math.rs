extern crate math;

use math::{fdimf, fdim, fmaxf, fmax, fminf, fmin};


#[test]
fn dim_f32() {
    use std::num::Float;
    use std::f32;
    assert_eq!(fdimf(10.0, 20.0), 0.0);
    assert_eq!(fdimf(20.0, -20.0), 40.0);
    assert!(fdimf(f32::NAN, 0.0).is_nan());
    assert!(fdimf(1.0, f32::NAN).is_nan());
    assert_eq!(fdimf(f32::MAX_VALUE, f32::MIN_VALUE), f32::INFINITY);
}

#[test]
fn dim_f64() {
    use std::num::Float;
    use std::f64;
    assert_eq!(fdim(10.0, 20.0), 0.0);
    assert_eq!(fdim(20.0, -20.0), 40.0);
    assert_eq!(fdim(1E100, -1E100), 2E100);
    assert!(fdim(f64::NAN, 0.0).is_nan());
    assert!(fdim(1.0, f64::NAN).is_nan());
    assert_eq!(fdim(f64::INFINITY, f64::NEG_INFINITY), f64::INFINITY);
    assert_eq!(fdim(f64::NEG_INFINITY, f64::INFINITY), 0.0);
    assert_eq!(fdim(f64::MAX_VALUE, f64::MIN_VALUE), f64::INFINITY);
}

#[test]
fn max_f32() {
    use std::num::Float;
    use std::f32;
    assert_eq!(fmaxf(0.0, 0.0), 0.0);
    assert_eq!(fmaxf(10.0, 0.0), 10.0);
    assert_eq!(fmaxf(0.0, -10.0), 0.0);
    assert_eq!(fmaxf(f32::NAN, 0.0), 0.0);
    assert_eq!(fmaxf(0.0, f32::NAN), 0.0);
    assert!(fmaxf(f32::NAN, f32::NAN).is_nan());
}

#[test]
fn max_f64() {
    use std::num::Float;
    use std::f64;
    assert_eq!(fmax(0.0, 0.0), 0.0);
    assert_eq!(fmax(10.0, 0.0), 10.0);
    assert_eq!(fmax(0.0, -10.0), 0.0);
    assert_eq!(fmax(f64::NAN, 0.0), 0.0);
    assert_eq!(fmax(0.0, f64::NAN), 0.0);
    assert!(fmax(f64::NAN, f64::NAN).is_nan());
}

#[test]
fn min_f32() {
    use std::num::Float;
    use std::f32;
    assert_eq!(fminf(0.0, 0.0), 0.0);
    assert_eq!(fminf(-10.0, 0.0), -10.0);
    assert_eq!(fminf(0.0, -10.0), -10.0);
    assert_eq!(fminf(f32::NAN, 0.0), 0.0);
    assert_eq!(fminf(0.0, f32::NAN), 0.0);
    assert!(fminf(f32::NAN, f32::NAN).is_nan());
}

#[test]
fn min_f64() {
    use std::num::Float;
    use std::f64;
    assert_eq!(fmin(0.0, 0.0), 0.0);
    assert_eq!(fmin(-10.0, 0.0), -10.0);
    assert_eq!(fmin(0.0, -10.0), -10.0);
    assert_eq!(fmin(f64::NAN, 0.0), 0.0);
    assert_eq!(fmin(0.0, f64::NAN), 0.0);
    assert!(fmin(f64::NAN, f64::NAN).is_nan());
}
