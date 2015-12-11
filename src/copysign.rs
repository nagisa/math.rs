use utils::{AsBits, Bits};
use utils::{F32_SIGN_MASK, F64_SIGN_MASK};


/// Return a value whose absolute value matches `l` and sign matches `r`.
#[no_mangle]
pub extern "C" fn copysignf(l: f32, r: f32) -> f32 {
    let (mut lbits, rbits) = (l.as_bits(), r.as_bits());
    // Clear the sign of left operand…
    lbits &= !F32_SIGN_MASK;
    // …and then or it with the sign or the right operand.
    lbits |= rbits & F32_SIGN_MASK;
    lbits.from_bits()
}

/// Return a value whose absolute value matches `l` and sign matches `r`.
#[no_mangle]
pub extern "C" fn copysign(l: f64, r: f64) -> f64 {
    let (mut lbits, rbits) = (l.as_bits(), r.as_bits());
    lbits &= !F64_SIGN_MASK;
    lbits |= rbits & F64_SIGN_MASK;
    lbits.from_bits()
}
