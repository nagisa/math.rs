/// Positive difference of two IEEE 754 64-bit floating-point numbers
///
/// Behaviour mismatch: Does not set ERANGE error on overflow as libm does.
#[no_mangle]
pub extern fn fdim(l: f64, r: f64) -> f64 {
    if l <= r {
        return 0.0
    }
    l - r
}

/// Positive difference of two IEEE 754 32-bit floating-point numbers
///
/// Behaviour mismatch: Does not set ERANGE error on overflow as libm does.
#[no_mangle]
pub extern fn fdimf(l : f32, r : f32) -> f32 {
    if l <= r {
        return 0.0
    }
    l - r
}

#[test]
fn dim_f32() {
    use core::num::Float;
    use core::f32;
    assert_eq!(fdimf(10.0, 20.0), 0.0);
    assert_eq!(fdimf(20.0, -20.0), 40.0);
    assert!(fdimf(f32::NAN, 0.0).is_nan());
    assert!(fdimf(1.0, f32::NAN).is_nan());

    // BEHAVIOUR MISMATCH: see documentation
    assert_eq!(fdimf(f32::MAX_VALUE, f32::MIN_VALUE), f32::INFINITY);
}

#[test]
fn dim_f64() {
    use core::num::Float;
    use core::f64;
    assert_eq!(fdim(10.0, 20.0), 0.0);
    assert_eq!(fdim(20.0, -20.0), 40.0);
    assert_eq!(fdim(1E100, -1E100), 2E100);
    assert!(fdim(f64::NAN, 0.0).is_nan());
    assert!(fdim(1.0, f64::NAN).is_nan());
    assert_eq!(fdim(f64::INFINITY, f64::NEG_INFINITY), f64::INFINITY);
    assert_eq!(fdim(f64::NEG_INFINITY, f64::INFINITY), 0.0);

    // BEHAVIOUR MISMATCH: see documentation
    assert_eq!(fdim(f64::MAX_VALUE, f64::MIN_VALUE), f64::INFINITY);
}
