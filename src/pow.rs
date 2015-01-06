use core::num::Float;

pub fn ieee754_pow64(x : f64, y: f64) -> f64 {
    // Simple cases
    if x.is_nan() {
        return x;
    }
    if y == 1.0 {
        return x;
    }
    if y == 2.0 {
        return x * x;
    }
    if y == -1.0 {
        return 1.0 / x;
    }
    if y == 0.0 {
        return 1.0;
    }

    // TODO

    return 0.0;
}

#[test]
fn pow_simple() {
    assert_eq!(ieee754_pow64(2.0, 2.0), 4.0);
    assert_eq!(ieee754_pow64(2.0, 1.0), 2.0);
    assert_eq!(ieee754_pow64(2.0, 0.0), 1.0);
    assert_eq!(ieee754_pow64(2.0, -1.0), 0.5);
}

#[test]
fn pow_nan() {
    use core::num::Float;
    assert!(ieee754_pow64(0.0_f64 / 0.0_f64, 3.0).is_nan());
    assert!(ieee754_pow64(0.0_f64 / 0.0_f64, 1024.0).is_nan());
    assert!(ieee754_pow64(0.0_f64 / 0.0_f64, 0.0).is_nan());
}
