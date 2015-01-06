use core::intrinsics::transmute;

/// Return a value whose absolute value matches `l` and sign matches `r`
#[no_mangle]
pub extern fn copysign(l: f64, r: f64) -> f64 {
    unsafe {
        let bitsl : u64 = transmute(l);
        let bitsr : u64 = transmute(r);
        transmute((bitsl & 0x7fff_ffff_ffff_ffff) | (bitsr & 0x8000_0000_0000_0000))
    }
}

/// Return a value whose absolute value matches `l` and sign matches `r`
#[no_mangle]
pub extern fn copysignf(l : f32, r : f32) -> f32 {
    unsafe {
        let bitsl : u32 = transmute(l);
        let bitsr : u32 = transmute(r);
        transmute((bitsl & 0x7fff_ffff) | (bitsr & 0x8000_0000))
    }
}

#[test]
fn copysign_f32() {
    use core::f32;
    assert_eq!(copysignf(10.0, 20.0), 10.0);
    assert_eq!(copysignf(20.0, -20.0), -20.0);
    assert_eq!(copysignf(f32::INFINITY, f32::NEG_INFINITY), f32::NEG_INFINITY);
}

#[test]
fn copysign_f64() {
    use core::f64;
    assert_eq!(copysign(10.0, 20.0), 10.0);
    assert_eq!(copysign(20.0, -20.0), -20.0);
    assert_eq!(copysign(f64::INFINITY, f64::NEG_INFINITY), f64::NEG_INFINITY);
}
