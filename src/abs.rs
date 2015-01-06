use core::intrinsics::transmute;

/// Absolute value of IEEE 754 64-bit floating-point number
#[no_mangle]
pub extern fn fabs(i : f64) -> f64 {
    unsafe {
        let bits : u64 = transmute(i);
        transmute(bits & 0x7fff_ffff_ffff_ffff)
    }
}

/// Absolute value of IEEE 754 32-bit floating-point number
#[no_mangle]
pub extern fn fabsf(i : f32) -> f32 {
    unsafe {
        let bits : u32 = transmute(i);
        transmute(bits & 0x7fff_ffff)
    }
}

#[test]
fn abs_f32() {
    assert_eq!(fabsf(0.1f32), 0.1f32);
    assert_eq!(fabsf(-0.1f32), 0.1f32);
    assert_eq!(fabsf(-2f32), 2f32);
    assert_eq!(fabsf(2f32), 2f32);
}

#[test]
fn abs_f64() {
    assert_eq!(fabs(0.1f64), 0.1f64);
    assert_eq!(fabs(-0.1f64), 0.1f64);
    assert_eq!(fabs(-2f64), 2f64);
    assert_eq!(fabs(2f64), 2f64);
}
