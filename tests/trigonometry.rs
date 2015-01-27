#![allow(unstable)]
extern crate math;

use math::{sin, cos, tan};
use std::num::Float;
use std::f64;
use std::f64::consts;

// f64::EPSILON is too accurate for us.
const EPSILON : f64 = 1E-10;
const FRAC_1_SQRT2 : f64 = 0.7071067811865475244008443;

#[test]
fn sin_f64(){
    for i in (-50u16)..50 {
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
    for i in ((-50u16)..50) {
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

#[test]
fn tan_f64(){
    const EPSILON: f64 = 1E-6;
    const BIG: f64 = 1E12;

    for i in ((-50u16)..50) {
        let rad = (i as f64) * consts::PI;
        assert!(tan(rad).abs() < EPSILON);
        assert!(tan(rad + consts::FRAC_PI_4).abs() - 1.0 < EPSILON);
        assert!(tan(rad + consts::FRAC_PI_2).abs() > BIG);
    }

    // Special cases
    assert!(cos(f64::NAN).is_nan());
    assert!(cos(f64::INFINITY).is_nan());
    assert!(cos(f64::NEG_INFINITY).is_nan());
}
