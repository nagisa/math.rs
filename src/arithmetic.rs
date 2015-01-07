//! Simple arithmetic operations on IEEE 754 floating-point numbers

/// Positive difference of two 32-bit floating-point numbers.
#[no_mangle]
pub extern fn fdimf(l : f32, r : f32) -> f32 {
    if l <= r {
        return 0.0
    }
    l - r
}

/// Positive difference of two 64-bit floating-point numbers.
#[no_mangle]
pub extern fn fdim(l: f64, r: f64) -> f64 {
    if l <= r {
        return 0.0
    }
    l - r
}

/// Returns the bigger of two 32-bit floating point numbers.
///
/// If one of the arguments is NaN, the other argument is returned.
/// If both arguments are NaN, NaN is returned.
#[no_mangle]
pub extern fn fmaxf(l: f32, r: f32) -> f32 {
    use core::num::Float;
    if l >= r || r.is_nan() {
        l
    } else {
        r
    }
}

/// Returns the bigger of two 64-bit floating point numbers.
///
/// If one of the arguments is NaN, the other argument is returned.
/// If both arguments are NaN, NaN is returned.
#[no_mangle]
pub extern fn fmax(l: f64, r: f64) -> f64 {
    use core::num::Float;
    if l >= r || r.is_nan() {
        l
    } else {
        r
    }
}

/// Returns the smaller of two 32-bit floating point numbers.
///
/// If one of the arguments is NaN, the other argument is returned.
/// If both arguments are NaN, NaN is returned.
#[no_mangle]
pub extern fn fminf(l: f32, r: f32) -> f32 {
    use core::num::Float;
    if l <= r || r.is_nan() {
        l
    } else {
        r
    }
}

/// Returns the smaller of two 64-bit floating point numbers.
///
/// If one of the arguments is NaN, the other argument is returned.
/// If both arguments are NaN, NaN is returned.
#[no_mangle]
pub extern fn fmin(l: f64, r: f64) -> f64 {
    use core::num::Float;
    if l <= r || r.is_nan() {
        l
    } else {
        r
    }
}
