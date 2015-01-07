use core::num::Int;
// TODO: remove this
use libc::c_int;

use utils::{AsBits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK};

/// Target and library dependent constant (i.e. might not be compatible with math.h).
pub const FP_ILOGB0 : c_int = -0x8000_0000;
/// Target and library dependent constant (i.e. might not be compatible with math.h).
pub const FP_ILOGBNAN : c_int = 0x7FFF_FFFF;

/// Get exponent of of a 32-bit floating-point value as an int.
#[no_mangle]
pub extern fn ilogbf(i: f32) -> c_int {
    let mut bits = i.as_bits();
    bits &= !F32_SIGN_MASK;
    if bits == 0 {
        return FP_ILOGB0;
    } else if bits < 0x0080_0000 {
        // Denormal
        let mut exp = bits >> 23;
        exp -= (bits.leading_zeros() as u32) - 9;
        return exp as c_int;
    } else if bits < 0x7F80_0000 {
        return ((bits >> 23) as c_int) - 0x7F;
    }
    FP_ILOGBNAN
}

/// Get exponent of of a 64-bit floating-point value.
#[no_mangle]
pub extern fn ilogb(i: f64) -> c_int {
    let mut bits = i.as_bits();
    bits &= !F64_SIGN_MASK;
    if bits == 0 {
        return FP_ILOGB0;
    } else if bits < 0x0010_0000_0000_0000 {
        // Denormal
        let mut exp = bits >> 52;
        exp -= (bits.leading_zeros() as u64) - 12;
        return exp as c_int;
    } else if bits < 0x7FF0_0000_0000_0000 {
        return ((bits >> 52) as c_int) - 0x3FF;
    }
    FP_ILOGBNAN
}
