use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK};

/// Absolute value of 32-bit floating-point number.
#[no_mangle]
pub extern "C" fn fabsf(i: f32) -> f32 {
    let mut bits = i.as_bits();
    // Sign bit of a positive float is 0. Just clear the bit.
    bits &= !F32_SIGN_MASK;
    bits.from_bits()
}

/// Absolute value of 64-bit floating-point number.
#[no_mangle]
pub extern "C" fn fabs(i: f64) -> f64 {
    let mut bits = i.as_bits();
    bits &= !F64_SIGN_MASK;
    bits.from_bits()
}
