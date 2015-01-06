use core::mem::transmute;
use core::mem::size_of;
use libc::types::os::arch::c99;
use libc::types::os::arch::c95;
use core::option::Option;
use core::option::Option::{Some,None};

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

/// Round number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, None is returned.
#[inline]
fn lroundf_(i : f32) -> Option<c95::c_long> {
    let mut bits : u32 = unsafe { transmute(i) };
    let exp = (((bits as i32) >> 23) & 0xff) - 0x7f;
    let sign : c95::c_long;
    let result : c95::c_long;

    // Calculate the sign
    if bits & 0x8000_0000 != 0 {
        sign = -1
    } else {
        sign = 1
    }

    // Calculate the result
    bits = (bits & 0x007f_ffff) | 0x0080_0000;
    if exp >= (8 * size_of::<c95::c_long>() - 1) as i32 {
        // The number is too large for the output type
        return None
    } else if exp == -1 {
        return Some(sign);
    } else if exp < -1 {
        return Some(0);
    } else if exp < 23 {
        bits += 0x0040_0000 >> (exp as uint);
        result = (bits >> ((23 - exp) as uint)) as c95::c_long;
    } else {
        result = (bits << ((exp - 23) as uint)) as c95::c_long;
    }
    Some(sign * result)
}

#[no_mangle]
pub extern fn lround(i : f64) -> c95::c_long {
    0
}

/// Round number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn lroundf(i : f32) -> c95::c_long {
    lroundf_(i).unwrap_or(0)
}

#[no_mangle]
pub extern fn llround(i : f64) -> c99::c_longlong {
    0
}

#[no_mangle]
pub extern fn llroundf(i : f32) -> c99::c_longlong {
    0
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
fn lround_f32() {
    use core::f32;
    use core::num::Float;
    assert_eq!(lroundf(0.0), 0);
    assert_eq!(lroundf(1.0), 1);
    assert_eq!(lroundf(1.6), 2);
    assert_eq!(lroundf(-1.5), -2);
    assert_eq!(lroundf(-1.4), -1);
    assert_eq!(lroundf(-0.01), 0);
    // Float can’t hold this value exactly, but it still fits into integer.
    // Allow to be off from expected result by at most 1
    assert!(lroundf(2147483647.0) - 2147483646 <= 2);
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
