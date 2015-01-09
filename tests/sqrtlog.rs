extern crate math;
use std::num::Float;
use std::{f32, f64};

use math::{sqrtf, sqrt};

#[test]
fn sqrt_f32() {
    assert_eq!(sqrtf(0.0), 0.0);
    assert_eq!(sqrtf(4.0), 2.0);
    assert_eq!(sqrtf(9.0), 3.0);
    assert_eq!(sqrtf(10000.0), 100.0);
    assert_eq!(sqrtf(110.25), 10.5);

    assert_eq!(sqrt(1.4E-45), 3.741657386773942e-23);

    assert_eq!(sqrtf(f32::INFINITY), f32::INFINITY);
    assert!(sqrtf(-1.0).is_nan());
    assert!(sqrtf(f32::NAN).is_nan());
    assert!(sqrtf(f32::NEG_INFINITY).is_nan());
}

#[test]
fn sqrt_f64() {
    assert_eq!(sqrt(0.0), 0.0);
    assert_eq!(sqrt(4.0), 2.0);
    assert_eq!(sqrt(9.0), 3.0);
    assert_eq!(sqrt(10000.0), 100.0);
    assert_eq!(sqrt(110.25), 10.5);

    assert_eq!(sqrt(4.94065645841246544176568792868E-324), 2.2227587494850775e-162);

    assert_eq!(sqrt(f64::INFINITY), f64::INFINITY);
    assert!(sqrt(-1.0).is_nan());
    assert!(sqrt(f64::NAN).is_nan());
    assert!(sqrt(f64::NEG_INFINITY).is_nan());
}
