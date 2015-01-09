#![allow(unstable)]
extern crate math;

use std::num::Float;
use std::{f32, f64};
use math::{fmaxf, fmax, fminf, fmin, hypotf, hypot};

#[test]
fn max_f32() {
    assert_eq!(fmaxf(0.0, 0.0), 0.0);
    assert_eq!(fmaxf(10.0, 0.0), 10.0);
    assert_eq!(fmaxf(0.0, -10.0), 0.0);
    assert_eq!(fmaxf(f32::NAN, 0.0), 0.0);
    assert_eq!(fmaxf(0.0, f32::NAN), 0.0);
    assert!(fmaxf(f32::NAN, f32::NAN).is_nan());
}

#[test]
fn max_f64() {
    assert_eq!(fmax(0.0, 0.0), 0.0);
    assert_eq!(fmax(10.0, 0.0), 10.0);
    assert_eq!(fmax(0.0, -10.0), 0.0);
    assert_eq!(fmax(f64::NAN, 0.0), 0.0);
    assert_eq!(fmax(0.0, f64::NAN), 0.0);
    assert!(fmax(f64::NAN, f64::NAN).is_nan());
}

#[test]
fn min_f32() {
    assert_eq!(fminf(0.0, 0.0), 0.0);
    assert_eq!(fminf(-10.0, 0.0), -10.0);
    assert_eq!(fminf(0.0, -10.0), -10.0);
    assert_eq!(fminf(f32::NAN, 0.0), 0.0);
    assert_eq!(fminf(0.0, f32::NAN), 0.0);
    assert!(fminf(f32::NAN, f32::NAN).is_nan());
}

#[test]
fn min_f64() {
    assert_eq!(fmin(0.0, 0.0), 0.0);
    assert_eq!(fmin(-10.0, 0.0), -10.0);
    assert_eq!(fmin(0.0, -10.0), -10.0);
    assert_eq!(fmin(f64::NAN, 0.0), 0.0);
    assert_eq!(fmin(0.0, f64::NAN), 0.0);
    assert!(fmin(f64::NAN, f64::NAN).is_nan());
}

#[test]
fn hypot_f32() {
    assert_eq!(hypotf(3.0, 4.0), 5.0);
    assert_eq!(hypotf(-3.0, -4.0), 5.0);
    assert_eq!(hypotf(f32::INFINITY, -4.0), f32::INFINITY);
    assert_eq!(hypotf(0.0, f32::NEG_INFINITY), f32::INFINITY);
    assert!(hypotf(f32::NAN, 0.0).is_nan());
    assert_eq!(hypotf(f32::INFINITY, f32::NAN), f32::INFINITY);
    assert_eq!(hypotf(f32::NAN, f32::INFINITY), f32::INFINITY);
}

#[test]
fn hypot_f64() {
    assert_eq!(hypot(3.0, 4.0), 5.0);
    assert_eq!(hypot(-3.0, -4.0), 5.0);
    assert_eq!(hypot(f64::INFINITY, -4.0), f64::INFINITY);
    assert_eq!(hypot(0.0, f64::NEG_INFINITY), f64::INFINITY);
    assert!(hypot(f64::NAN, 0.0).is_nan());
    assert_eq!(hypot(f64::INFINITY, f64::NAN), f64::INFINITY);
    assert_eq!(hypot(f64::NAN, f64::INFINITY), f64::INFINITY);
}
