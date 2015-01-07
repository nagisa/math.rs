use core::mem::size_of;
// TODO: this needs to be gotten rid of
use libc::c_long;

use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK, F32_MANTISSA_MASK, F64_MANTISSA_MASK};

/// Round the 32-bit floating-point number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn lroundf(i : f32) -> c_long {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    // Minus one, because one bit has to be reserved for sign.
    let target_size = 8 * size_of::<c_long>() - 1;
    let sign = if bits & F32_SIGN_MASK == 0 { 1 } else { -1 };

    if exp < -1 {
        return 0;
    } else if exp == -1 {
        return sign;
    } else if exp >= (target_size as i32) {
        // Result doesn’t fit. 0 is as arbitrary as any other value.
        return 0;
    } else if exp >= 23 {
        bits &= F32_MANTISSA_MASK;
        bits |= 0x0080_0000;
        return sign * ((bits >> ((exp - 23) as uint)) as c_long);
    } else {
        bits &= F32_MANTISSA_MASK;
        bits |= 0x0080_0000;
        bits += 0x0040_0000 >> (exp as uint);
        return sign * ((bits >> ((23 - exp) as uint)) as c_long);
    };
}

/// Round the 64-bit floating-point number away from zero.
///
/// If the number is ∞, NaN or doesn’t fit into the result value, arbitrary value may be returned.
#[no_mangle]
pub extern fn lround(i : f64) -> c_long {
    // Same thing as lroundf with constants adapted.
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();
    let target_size = 8 * size_of::<c_long>() - 1;
    let sign = if bits & F64_SIGN_MASK == 0 { 1 } else { -1 };

    if exp < -1 {
        return 0;
    } else if exp == -1 {
        return sign;
    } else if exp >= (target_size as i32) {
        return 0;
    } else if exp >= 52 {
        bits &= F64_MANTISSA_MASK;
        bits |= 0x0010_0000_0000_0000;
        return sign * ((bits >> ((exp - 52) as uint)) as c_long);
    } else {
        bits &= F64_MANTISSA_MASK;
        bits |= 0x0010_0000_0000_0000;
        bits += 0x0008_0000_0000_0000 >> (exp as uint);
        return sign * ((bits >> ((52 - exp) as uint)) as c_long);
    }
}
