use core::mem::transmute;
use libc::types::os::arch::c99;
use libc::types::os::arch::c95;

#[no_mangle]
pub extern fn lround(i : f64) -> c95::c_long {
    0
}

#[no_mangle]
pub extern fn lroundf(i : f32) -> c95::c_long {
    0
}

#[no_mangle]
pub extern fn llround(i : f64) -> c99::c_longlong {
    0
}

#[no_mangle]
pub extern fn llroundf(i : f32) -> c99::c_longlong {
    0
}

/// Round the number away from zero.
#[no_mangle]
pub extern fn round(i : f64) -> f64 {
    // Port of glibc libm implementation.
    let mut bits : u64 = unsafe { transmute(i) };
    let high : u32 = (bits >> 32) as u32;
    let exp : i32 = (((high >> 20) as i32) & 0x7ff) - 0x3ff;
    let low : u32 = bits as u32;

    if exp < 0 {
        // Disrepancy: Original implementation raises overflow here
        bits &= 0x8000_0000_0000_0000;
        if exp == -1 {
            bits |= 0x3ff0_0000_0000_0000;
        }
    } else if exp < 20 {
        let temp : u32 = 0x000f_ffff >> (exp as uint);
        if ((high & temp) | low) == 0 {
            // The input is already integral
            return i;
        }
        // Disrepancy: Original implementation raises overflow here
        bits += 0x0008_0000_0000_0000 >> (exp as uint);
        bits &= (!temp as u64) << 32;
    } else if exp > 51 {
        if exp == 0x400 {
            // Infinite or NaN
            return i + i;
        } else {
            // The input is already integral
            return i;
        }
    } else {
        let temp = 0xffff_ffff >> ((exp - 20) as uint);
        if low & temp == 0 {
            // The input is already integral
            return i;
        }
        // Disrepancy: Original implementation raises overflow here
        let temp2 = low + (1 << (51 - (exp as uint)));
        if temp2 < low {
            bits += 1 << 32;
        }
        bits = (bits & 0xffff_ffff_0000_0000) | (temp2 & !temp) as u64;
    }
    unsafe { transmute(bits) }
}

/// Round the number away from zero.
#[no_mangle]
pub extern fn roundf(i : f32) -> f32 {
    // Port of glibc libm implementation.
    let mut bits : u32 = unsafe { transmute(i) };
    let exp = (((bits as i32) >> 23) & 0xff) - 0x7f;
    if exp < 0 {
        bits &= 0x8000_0000;
        if exp == -1 {
            bits |= 0x3f80_0000;
        }
    } else if exp < 23 {
        let temp = 0x007f_ffff >> (exp as uint);
        if bits & temp == 0 {
            // The input is already integral
            return i;
        }
        bits += 0x0040_0000 >> (exp as uint);
        bits &= !temp;
    } else {
        if exp == 0x80 {
            // Infinite or NaN
            return i + i;
        } else {
            // The input is already integral
            return i;
        }
    }
    unsafe { transmute(bits) }
}

#[no_mangle]
pub extern fn floor(i : f64) -> f64 {
    0.0
}

/// Round the number towards -∞ (negative infinity).
#[no_mangle]
pub extern fn floorf(i : f32) -> f32 {
    let mut bits : i32 = unsafe { transmute(i) };
    let exp : i32 = ((bits >> 23) & 0xff) - 0x7f;
    if exp < 0 {
        if bits >= 0 {
            bits = 0;
        } else if bits & 0x7fff_ffff != 0 {
            bits = 0xbf80_0000u32 as i32;
        }
    } else if exp < 23 {
        let temp = 0x007f_ffff >> (exp as uint);
        if bits & temp == 0 {
            // The input is already integral
            return i;
        }
        if bits < 0 {
            bits += 0x0080_0000 >> (exp as uint);
        }
        bits &= !temp;
    } else {
        if exp == 0x80 {
            // Infinite or NaN
            return i + i;
        } else {
            // The input is already integral
            return i;
        }
    }
    unsafe { transmute(bits) }
}

#[no_mangle]
pub extern fn ceil(i : f64) -> f64 {
    0.0
}

/// Round the number towards +∞ (positive infinity).
#[no_mangle]
pub extern fn ceilf(i : f32) -> f32 {
    let mut bits : i32 = unsafe { transmute(i) };
    let exp = ((bits >> 23) & 0xff) - 0x7f;
    if exp < 0 {
        if bits < 0 {
            bits = 0x8000_0000u32 as i32;
        } else if bits != 0 {
            bits = 0x3f80_0000u32 as i32;
        }
    } else if exp < 23 {
        let temp = 0x007f_ffff >> (exp as uint);
        if bits & temp == 0 {
            // The input is already integral
            return i;
        }
        if bits > 0 {
            bits += 0x0080_0000 >> (exp as uint);
        }
        bits &= !temp;
    } else {
        if exp == 0x80 {
            // Infinite or NaN
            return i + i;
        } else {
            // The input is already integral
            return i;
        }
    }
    unsafe { transmute(bits) }
}

// Interferes with assert_eq output for f64 tests
// #[no_mangle]
// pub extern fn trunc(i : f64) -> f64 {
//     0.0
// }

/// Truncate number to nearest integral value not larger than the number.
#[no_mangle]
pub extern fn truncf(i : f32) -> f32 {
    let mut bits : u32 = unsafe { transmute(i) };
    let zerosign = bits & 0x8000_0000;
    let exp = (((bits as i32) >> 23) & 0xff) - 0x7f;
    if exp < 0 {
        // The magnitude of the number is <1 therefore the result is ±0.
        bits = zerosign;
    } else if exp < 23 {
        bits = zerosign | (bits & !(0x007f_ffff >> (exp as uint)));
    } else {
        if exp == 0x80 {
            // Infinite or NaN
            return i + i;
        } else {
            return i;
        }
    }
    unsafe { transmute(bits) }
}

#[test]
fn round_f64() {
    use core::f64;
    use core::num::Float;
    assert_eq!(round(0.0), 0.0);
    assert_eq!(round(1.0), 1.0);
    assert_eq!(round(1.5), 2.0);
    assert_eq!(round(-1.5), -2.0);
    assert_eq!(round(-1.4), -1.0);
    assert_eq!(round(-0.01), 0.0);
    assert_eq!(round(1.7654321E13), 1.7654321E13);
    assert_eq!(round(123456789.500001), 123456790.0);
    assert_eq!(round(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(round(f64::NAN).is_nan());
}

#[test]
fn round_f32() {
    use core::f32;
    use core::num::Float;
    assert_eq!(roundf(0.0), 0.0);
    assert_eq!(roundf(1.0), 1.0);
    assert_eq!(roundf(1.5), 2.0);
    assert_eq!(roundf(-1.5), -2.0);
    assert_eq!(roundf(-1.4), -1.0);
    assert_eq!(roundf(-0.01), 0.0);
    assert_eq!(roundf(1.7654321E13), 1.7654321E13);
    assert_eq!(roundf(123456789.500001), 123456790.0);
    assert_eq!(roundf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(roundf(f32::NAN).is_nan());
}

#[test]
fn ceil_f32() {
    use core::f32;
    use core::num::Float;
    assert_eq!(ceilf(0.0), 0.0);
    assert_eq!(ceilf(1.0), 1.0);
    assert_eq!(ceilf(1.5), 2.0);
    assert_eq!(ceilf(-1.5), -1.0);
    assert_eq!(ceilf(-1.4), -1.0);
    assert_eq!(ceilf(-0.01), -0.0);
    assert_eq!(ceilf(0.999), 1.0);
    assert_eq!(ceilf(1.7654321E13), 1.7654321E13);
    assert_eq!(ceilf(123456789.500001), 123456790.0);
    assert_eq!(ceilf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(ceilf(f32::NAN).is_nan());
}

#[test]
fn floor_f32() {
    use core::f32;
    use core::num::Float;
    assert_eq!(floorf(0.0), 0.0);
    assert_eq!(floorf(1.0), 1.0);
    assert_eq!(floorf(1.5), 1.0);
    assert_eq!(floorf(-1.5), -2.0);
    assert_eq!(floorf(-1.4), -2.0);
    assert_eq!(floorf(-0.01), -1.0);
    assert_eq!(floorf(0.999), 0.0);
    assert_eq!(floorf(1.7654321E13), 1.7654321E13);
    assert_eq!(floorf(123456789.500001), 123456789.0);
    assert_eq!(floorf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(floorf(f32::NAN).is_nan());
}

#[test]
fn trunc_f32() {
    use core::f32;
    use core::num::Float;
    assert_eq!(truncf(1.1341241), 1.0);
    assert_eq!(truncf(0.01), 0.0);
    assert_eq!(truncf(-0.01), -0.0);
    assert_eq!(truncf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(truncf(f32::NAN).is_nan());
}
