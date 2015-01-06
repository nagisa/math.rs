use core::mem::transmute;
use libc::types::os::arch::c99;
use libc::types::os::arch::c95;

#[no_mangle]
pub fn lround(i : f64) -> c95::c_long {
    0
}

#[no_mangle]
pub fn lroundf(i : f32) -> c95::c_long {
    0
}

#[no_mangle]
pub fn llround(i : f64) -> c99::c_longlong {
    0
}

#[no_mangle]
pub fn llroundf(i : f32) -> c99::c_longlong {
    0
}

/// Rounds the number away from zero.
#[no_mangle]
pub fn round(i : f64) -> f64 {
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
        // Infinite or NaN
        if exp == 0x400 {
            return i + i;
        } else {
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

#[no_mangle]
pub fn roundf(i : f32) -> f32 {
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
        // Infinite or NaN
        if exp == 0x80 {
            return i + i;
        } else {
            return i;
        }
    }
    unsafe { transmute(bits) }
}

#[no_mangle]
pub fn floor(i : f64) -> f64 {
    0.0
}

#[no_mangle]
pub fn floorf(i : f32) -> f32 {
    0.0
}

#[no_mangle]
pub fn ceil(i : f64) -> f64 {
    0.0
}

#[no_mangle]
pub fn ceilf(i : f32) -> f32 {
    0.0
}

#[no_mangle]
pub fn trunc(i : f64) -> f64 {
    0.0
}

#[no_mangle]
pub fn truncf(i : f32) -> f32 {
    0.0
}

#[test]
fn round_f64() {
    use core::f64;
    use core::num::Float;
    assert_eq!(round(0.0f64), 0.0);
    assert_eq!(round(1.0f64), 1.0);
    assert_eq!(round(1.5f64), 2.0);
    assert_eq!(round(-1.5f64), -2.0);
    assert_eq!(round(-1.4f64), -1.0);
    assert_eq!(round(-0.01f64), 0.0);
    assert_eq!(round(1.7654321E13), 1.7654321E13);
    assert_eq!(round(123456789.500001), 123456790.0);
    assert_eq!(round(f64::NEG_INFINITY), f64::NEG_INFINITY);
    assert!(round(f64::NAN).is_nan());
}

#[test]
fn round_f32() {
    use core::f32;
    use core::num::Float;
    assert_eq!(roundf(0.0f32), 0.0);
    assert_eq!(roundf(1.0f32), 1.0);
    assert_eq!(roundf(1.5f32), 2.0);
    assert_eq!(roundf(-1.5f32), -2.0);
    assert_eq!(roundf(-1.4f32), -1.0);
    assert_eq!(roundf(-0.01f32), 0.0);
    assert_eq!(roundf(1.7654321E13), 1.7654321E13);
    assert_eq!(roundf(123456789.500001), 123456790.0);
    assert_eq!(roundf(f32::NEG_INFINITY), f32::NEG_INFINITY);
    assert!(roundf(f32::NAN).is_nan());
}
