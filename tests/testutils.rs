#![allow(unstable)]
use std::mem::transmute;
use std::num::Float;
use std::fmt::UpperExp;

pub const TEST_NAN_SIGN: u32 = 1 << 0;
pub const TEST_ZERO_SIGN: u32 = 1 << 1;

// pub const F32_MIN_SUBNORM: f32 = 1.4E-45;
// pub const F64_MIN_SUBNORM: f32 = 4.94065645841246544176568792868E-324;

pub trait SignBit {
    fn signbit(self) -> bool;
}

impl SignBit for f32 {
    fn signbit(self) -> bool {
        unsafe{transmute::<f32, u32>(self) & 0x8000_0000u32 > 0 }
    }
}

impl SignBit for f64 {
    fn signbit(self) -> bool {
        unsafe{transmute::<f64, u64>(self) & 0x8000_0000_0000_0000u64 > 0 }
    }
}

pub fn _assert_feq<F: Float + SignBit + UpperExp>(i: F, o: F, maxdiff: F, flags: u32) -> Option<String> {
    use std::num::Float;
    if i.is_finite() && o.is_finite() {
        if (i == Float::zero() || o == Float::zero()) && flags & TEST_ZERO_SIGN > 0
                                    && i.signbit() != o.signbit() {
            return Some(format!("assertion failed: {:E} ≠ {:E} (sign bits differ)", i, o));
        }
        if maxdiff == Float::zero() && i != o {
            return Some(format!("assertion failed: {:.30E} ≠ {:.30E}", i, o));
        } else {
            if (i - o).abs() > maxdiff {
                return Some(format!("assertion failed: {:.30E} ≠ {:.30E} within certain treshold (difference: {:.30E})",
                                    i, o, i - o));
            }
        }
    } else {
        if i.is_infinite() || o.is_infinite() {
            if i != o {
                return Some(format!("assertion failed: {:E} and {:E} are not equal", i, o));
            }
        } else {
            if !i.is_nan() || !o.is_nan() {
                return Some(format!("assertion failed: either {:E} or {:E} is NaN, but not both of them",
                                    i, o));
            } else if flags & TEST_NAN_SIGN > 0 {
                if i.signbit() != o.signbit() {
                    return Some(format!("assertion failed: sign bits of {:E} and {:E} do not match",
                                        i, o));
                }
            }
        }
    }
    None
}

/// Takes two floating point numbers and maximum allowed difference between them.
/// Also flags for comparison.
///
/// Notable exceptions:
/// * If either side is non-finite value, both sides must be equal
///
#[macro_export]
macro_rules! assert_feq {
    ($i:expr, $o:expr, $maxdiff:expr, $flags:expr) => ({
        use std::num::Float;
        if let Some(msg) = _assert_feq($i, $o, $maxdiff.abs(), $flags) {
            panic!(msg)
        }
    })
}
