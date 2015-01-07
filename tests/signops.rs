extern crate math;
use std::{f64, f32};

use math::{fabs,fabsf,copysignf,copysign};

#[test]
fn abs_f32() {
    assert_eq!(fabsf(0.1f32), 0.1f32);
    assert_eq!(fabsf(-0.1f32), 0.1f32);
    assert_eq!(fabsf(-2f32), 2f32);
    assert_eq!(fabsf(2f32), 2f32);
}

#[test]
fn abs_f64() {
    assert_eq!(fabs(0.1f64), 0.1f64);
    assert_eq!(fabs(-0.1f64), 0.1f64);
    assert_eq!(fabs(-2f64), 2f64);
    assert_eq!(fabs(2f64), 2f64);
}

#[test]
fn copysign_f32() {
    assert_eq!(copysignf(10.0, 20.0), 10.0);
    assert_eq!(copysignf(20.0, -20.0), -20.0);
    assert_eq!(copysignf(f32::INFINITY, f32::NEG_INFINITY), f32::NEG_INFINITY);
}

#[test]
fn copysign_f64() {
    assert_eq!(copysign(10.0, 20.0), 10.0);
    assert_eq!(copysign(20.0, -20.0), -20.0);
    assert_eq!(copysign(f64::INFINITY, f64::NEG_INFINITY), f64::NEG_INFINITY);
}
