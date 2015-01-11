#![allow(unstable)]
extern crate math;

use math::{sin, cos};
use std::num::Float;
use std::f64;
use std::f64::consts;

// f64::EPSILON is too accurate for us.
const EPSILON : f64 = 1E-10;
const FRAC_1_SQRT2 : f64 = 0.7071067811865475244008443;

#[test]
fn sin_f64(){
    for i in (-50)..50 {
        let rad = (i as f64) * consts::PI;
        assert!(sin(rad).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 + consts::FRAC_PI_2;
        assert!((sin(rad) - 1.0).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 - consts::FRAC_PI_2;
        assert!((sin(rad) + 1.0).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 + consts::FRAC_PI_4;
        assert!((sin(rad) - FRAC_1_SQRT2).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 + 3.0 * consts::FRAC_PI_4;
        assert!((sin(rad) - FRAC_1_SQRT2).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 - consts::FRAC_PI_4;
        assert!((sin(rad) + FRAC_1_SQRT2).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 - 3.0 * consts::FRAC_PI_4;
        assert!((sin(rad) + FRAC_1_SQRT2).abs() < EPSILON);
    }

    // Special cases
    assert!(sin(f64::NAN).is_nan());
    assert!(sin(f64::INFINITY).is_nan());
    assert!(sin(f64::NEG_INFINITY).is_nan());
}

#[test]
fn cos_f64(){
    for i in ((-50)..50) {
        let rad = (i as f64) * consts::PI;
        if i % 2 == 0 {
            assert!((cos(rad) - 1.0).abs() < EPSILON)
        } else {
            assert!((cos(rad) + 1.0).abs() < EPSILON)
        }
        let rad = (i as f64) * consts::PI_2 + consts::FRAC_PI_2;
        assert!((cos(rad)).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 + consts::FRAC_PI_4;
        assert!((cos(rad) - FRAC_1_SQRT2).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 + 3.0 * consts::FRAC_PI_4;
        assert!((cos(rad) + FRAC_1_SQRT2).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 - consts::FRAC_PI_4;
        assert!((cos(rad) - FRAC_1_SQRT2).abs() < EPSILON);
        let rad = (i as f64) * consts::PI_2 - 3.0 * consts::FRAC_PI_4;
        assert!((cos(rad) + FRAC_1_SQRT2).abs() < EPSILON);
    }

    // Special cases
    assert!(cos(f64::NAN).is_nan());
    assert!(cos(f64::INFINITY).is_nan());
    assert!(cos(f64::NEG_INFINITY).is_nan());
}
