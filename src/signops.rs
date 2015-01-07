/// Operations on IEEE 754 floating-point number sign.
use utils::{AsBits, Bits};

const F64_SIGN_MASK : u64 = 1 << 63;
const F32_SIGN_MASK : u32 = 1 << 31;

/// Absolute value of IEEE 754 32-bit floating-point number
#[no_mangle]
pub extern fn fabsf(i : f32) -> f32 {
    let mut bits = i.as_bits();
    // Sign bit of a positive float is 0. Just clear the bit.
    bits &= !F32_SIGN_MASK;
    bits.from_bits()
}

/// Absolute value of IEEE 754 64-bit floating-point number
#[no_mangle]
pub extern fn fabs(i : f64) -> f64 {
    let mut bits = i.as_bits();
    bits &= !F64_SIGN_MASK;
    bits.from_bits()
}

/// Return a value whose absolute value matches `l` and sign matches `r`
#[no_mangle]
pub extern fn copysignf(l : f32, r : f32) -> f32 {
    let (mut lbits, rbits) = (l.as_bits(), r.as_bits());
    // Clear the sign of left operand…
    lbits &= !F32_SIGN_MASK;
    // …and then or it with the sign or the right operand.
    lbits |= rbits & F32_SIGN_MASK;
    lbits.from_bits()
}

/// Return a value whose absolute value matches `l` and sign matches `r`
#[no_mangle]
pub extern fn copysign(l : f64, r : f64) -> f64 {
    let (mut lbits, rbits) = (l.as_bits(), r.as_bits());
    lbits &= !F64_SIGN_MASK;
    lbits |= rbits & F64_SIGN_MASK;
    lbits.from_bits()
}
