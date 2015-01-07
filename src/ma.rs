/// Compute l * r + s.
#[no_mangle]
pub extern fn fmaf(l: f32, r: f32, s: f32) -> f32 {
    (l * r) + s
}

/// Compute l * r + s.
#[no_mangle]
pub extern fn fma(l: f64, r: f64, s: f64) -> f64 {
    (l * r) + s
}
