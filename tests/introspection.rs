extern crate math;

use std::{f32, f64};
use math::{logbf, logb, ilogbf, ilogb, FP_ILOGB0, FP_ILOGBNAN};
use math::{scalbnf, scalbn};
use math::{modff, modf};
use math::{nextafterf, nextafter};

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

#[test]
fn scalbn_f32() {
    assert_eq!(scalbnf(0.0, 100), 0.0);
    assert_eq!(scalbnf(-0.0, 100), 0.0);
    assert_eq!(scalbnf(0.0, 0), 0.0);
    assert_eq!(scalbnf(2.0, 2), 8.0);

    // Denormals
    assert_eq!(scalbnf(1.1754942E-38, -23), 0.0); // Goes out of range
    assert_eq!(scalbnf(1.4E-45, -1), 0.0); // Out of range
    assert_eq!(scalbnf(1.1754942E-38, -1), 5.877472e-39); // Stays denormal
    // - Becomes normal
    let normal1 = scalbnf(1.1754942E-38, 1) - 2.350988e-38f32;
    assert!(normal1 < 1e-38 && normal1 > -1e-38);
    // - Becomes normal
    let normal2 = scalbnf(1.4E-45, 24) - 2.350989e-38;
    assert!(normal2 < 1e-38 && normal2 > -1e-38);

    // Special cases
    assert!(scalbnf(f32::NAN, 100).is_nan());
    assert_eq!(scalbnf(f32::INFINITY, -100), f32::INFINITY);
    assert_eq!(scalbnf(f32::NEG_INFINITY, 100), f32::NEG_INFINITY);
    assert_eq!(scalbnf(1.0, 128), f32::INFINITY); // Out of range
    assert_eq!(scalbnf(-1.0, 128), f32::NEG_INFINITY); // Out of range
}

#[test]
fn scalbn_f64() {
    assert_eq!(scalbn(0.0, 100), 0.0);
    assert_eq!(scalbn(-0.0, 100), 0.0);
    assert_eq!(scalbn(0.0, 0), 0.0);
    assert_eq!(scalbn(2.0, 2), 8.0);

    // Denormals
    assert_eq!(scalbn(1.11253692925360069154511635867E-308, -52), 0.0); // Goes out of range
    assert_eq!(scalbn(4.94065645841246544176568792868E-324, -1), 0.0); // Out of range
    assert_eq!(scalbn(1.11253692925360069154511635867E-308, -1),
               5.56268464626800345772558179333e-309); // Stays denormal
    // - Becomes normal
    let normal1 = scalbn(1.11253692925360069154511635867E-308, 1)
                - 2.22507385850720138309023271733e-308;
    assert!(normal1 < 1e-308 && normal1 > -1e-308);
    // - Becomes normal
    let normal2 = scalbn(4.94065645841246544176568792868E-324, 52)
                - 2.22507385850720138309023271733e-308;
    assert!(normal2 < 1e-308 && normal2 > -1e-308);

    // Special cases
    assert!(scalbn(f64::NAN, 100).is_nan());
    assert_eq!(scalbn(f64::INFINITY, -100), f64::INFINITY);
    assert_eq!(scalbn(f64::NEG_INFINITY, 100), f64::NEG_INFINITY);
    assert_eq!(scalbn(1.0, 1024), f64::INFINITY); // Out of range
    assert_eq!(scalbn(-1.0, 1024), f64::NEG_INFINITY); // Out of range
}

#[test]
fn modf_f32() {
    let mut o = 0.0f32;
    assert!(modff(2.4, &mut o as *mut f32) - 0.4 < f32::EPSILON);
    assert_eq!(o, 2.0);
    // Special cases
    assert!(modff(f32::NAN, &mut o as *mut f32).is_nan());
    assert!(o.is_nan());
    assert_eq!(modff(f32::INFINITY, &mut o as *mut f32), 0.0);
    assert_eq!(o, f32::INFINITY);
    assert_eq!(modff(f32::NEG_INFINITY, &mut o as *mut f32), 0.0);
    assert_eq!(o, f32::NEG_INFINITY);
}

#[test]
fn modf_f64() {
    let mut o = 0.0f64;
    assert!(modf(2.4, &mut o as *mut f64) - 0.4 < f64::EPSILON);
    assert_eq!(o, 2.0);
    // Special cases
    assert!(modf(f64::NAN, &mut o as *mut f64).is_nan());
    assert!(o.is_nan());
    assert_eq!(modf(f64::INFINITY, &mut o as *mut f64), 0.0);
    assert_eq!(o, f64::INFINITY);
    assert_eq!(modf(f64::NEG_INFINITY, &mut o as *mut f64), -0.0);
    assert_eq!(o, f64::NEG_INFINITY);
}

#[test]
fn nextafter_f32() {
    assert_eq!(nextafterf(1.0, 10.0), 1.0000001);
    assert_eq!(nextafterf(1.0, 0.0), 0.99999994);
    assert_eq!(nextafterf(1.0, 1.0), 1.0);
    // Subnormals
    assert_eq!(nextafterf(1.4E-45, f32::NEG_INFINITY), 0.0);
    assert_eq!(nextafterf(0.0, f32::INFINITY), 1.4E-45);
    assert_eq!(nextafterf(0.0, f32::NEG_INFINITY), -1.4E-45);
    // Special cases
    assert!(nextafterf(f32::NAN, 0.0).is_nan());
    assert!(nextafterf(0.0, f32::NAN).is_nan());
}

#[test]
fn nextafter_f64() {
    assert_eq!(nextafter(1.0, 10.0), 1.00000000000000022204460492503);
    assert_eq!(nextafter(1.0, 0.0), 0.999999999999999888977697537484);
    assert_eq!(nextafter(1.0, 1.0), 1.0);
    // Subnormals
    assert_eq!(nextafter(4.94065645841246544176568792868E-324, f64::NEG_INFINITY), 0.0);
    assert_eq!(nextafter(0.0, f64::INFINITY), 4.94065645841246544176568792868E-324);
    assert_eq!(nextafter(0.0, f64::NEG_INFINITY), -4.94065645841246544176568792868E-324);
    // Special cases
    assert!(nextafter(f64::NAN, 0.0).is_nan());
    assert!(nextafter(0.0, f64::NAN).is_nan());
}
