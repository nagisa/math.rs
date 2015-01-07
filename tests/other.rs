extern crate math;

use math::{fmaxf, fmax, fminf, fmin};

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
