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
