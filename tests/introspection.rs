extern crate math;

use std::num::Float;
use std::{f32, f64};
use math::{logbf, logb, ilogbf, ilogb, FP_ILOGB0, FP_ILOGBNAN};

#[test]
fn logb_f32() {
    assert_eq!(logbf(0.0), f32::NEG_INFINITY);
    assert_eq!(logbf(0.001), -10.0);
    assert_eq!(logbf(-0.001), -10.0);
    assert_eq!(logbf(2.0), 1.0);
    assert_eq!(logbf(4.0), 2.0);
    assert_eq!(logbf(10.0), 3.0);
    // Denormals
    assert_eq!(logbf(5.877472E-39), -127.0);
    assert_eq!(logbf(1.4E-45), -149.0);
    // Special cases
    assert!(logbf(f32::NAN).is_nan());
    assert_eq!(logbf(f32::INFINITY), f32::INFINITY);
    assert_eq!(logbf(f32::NEG_INFINITY), f32::NEG_INFINITY);
}

#[test]
fn logb_f64() {
    assert_eq!(logb(0.0), f64::NEG_INFINITY);
    assert_eq!(logb(0.001), -10.0);
    assert_eq!(logb(-0.001), -10.0);
    assert_eq!(logb(2.0), 1.0);
    assert_eq!(logb(4.0), 2.0);
    assert_eq!(logb(10.0), 3.0);
    // Denormals
    assert_eq!(logb(1.11253692925360069154511635867E-308), -1023.0);
    assert_eq!(logb(4.94065645841246544176568792868E-324), -1074.0);
    // Special cases
    assert!(logb(f64::NAN).is_nan());
    assert_eq!(logb(f64::INFINITY), f64::INFINITY);
    assert_eq!(logb(f64::NEG_INFINITY), f64::NEG_INFINITY);
}

#[test]
fn ilogb_f32() {
    assert_eq!(ilogbf(0.0), FP_ILOGB0);
    assert_eq!(ilogbf(-0.0), FP_ILOGB0);
    assert_eq!(ilogbf(0.001), -10);
    assert_eq!(ilogbf(-0.001), -10);
    assert_eq!(ilogbf(2.0), 1);
    assert_eq!(ilogbf(4.0), 2);
    assert_eq!(ilogbf(10.0), 3);
    // Denormals
    assert_eq!(ilogbf(5.877472E-39), -127);
    assert_eq!(ilogbf(1.4E-45), -149);
    // Special cases
    assert_eq!(ilogbf(f32::NAN), FP_ILOGBNAN);
    assert_eq!(ilogbf(f32::INFINITY), FP_ILOGBNAN);
    assert_eq!(ilogbf(f32::NEG_INFINITY), FP_ILOGBNAN);
}

#[test]
fn ilogb_f64() {
    assert_eq!(ilogb(0.0), FP_ILOGB0);
    assert_eq!(ilogb(-0.0), FP_ILOGB0);
    assert_eq!(ilogb(0.001), -10);
    assert_eq!(ilogb(-0.001), -10);
    assert_eq!(ilogb(2.0), 1);
    assert_eq!(ilogb(4.0), 2);
    assert_eq!(ilogb(10.0), 3);
    // Denormals
    assert_eq!(ilogb(1.11253692925360069154511635867E-308), -1023);
    assert_eq!(ilogb(4.94065645841246544176568792868E-324), -1074);
    // Special cases
    assert_eq!(ilogb(f64::NAN), FP_ILOGBNAN);
    assert_eq!(ilogb(f64::INFINITY), FP_ILOGBNAN);
    assert_eq!(ilogb(f64::NEG_INFINITY), FP_ILOGBNAN);
}
