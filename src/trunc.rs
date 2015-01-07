use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK, F32_MANTISSA_MASK, F64_MANTISSA_MASK};

/// Truncate the 32-bit floating-point number to nearest integral value not larger than the number.
#[no_mangle]
pub extern fn truncf(i : f32) -> f32 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // The number ∈ (-1; 1) therefore the result is ±0.
        bits &= F32_SIGN_MASK;
    } else if exp == 0x80 {
        return i + i;
    } else if exp >= 23 {
        return i;
    } else {
        // Perserve the sign and simply truncate mantissa as necessary.
        bits = (bits & F32_SIGN_MASK) | (bits & !(F32_MANTISSA_MASK >> (exp as uint)));
    }
    bits.from_bits()
}

/// Truncate the 64-bit floating-point number to nearest integral value not larger than the number.
#[no_mangle]
pub extern fn trunc(i : f64) -> f64 {
    let mut bits = i.as_bits();
    let exp = bits.get_exponent();

    if exp < 0 {
        // The number ∈ (-1; 1) therefore the result is ±0.
        bits &= F64_SIGN_MASK;
    } else if exp == 0x400 {
        return i + i;
    } else if exp >= 52 {
        return i;
    } else {
        // Perserve the sign and simply truncate mantissa as necessary.
        bits = (bits & F64_SIGN_MASK) | (bits & !(F64_MANTISSA_MASK >> (exp as uint)));
    }
    bits.from_bits()
}
