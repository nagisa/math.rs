// TODO: remove this
use libc::c_int;

use scalbn::{scalbnf, scalbn};

/// Multiply the 32-bit floating-point number by integral power of radix.
///
/// Alias to scalbnf.
///
/// Note: This function does not report overflow/underflow errors like the original implementation
/// does.
#[no_mangle]
pub extern "C" fn ldexpf(i: f32, x: c_int) -> f32 {
    scalbnf(i, x)
}

/// Multiply the 64-bit floating-point number by integral power of radix.
///
/// Alias to scalbn.
///
/// Note: This function does not report overflow/underflow errors like the original implementation
/// does.
#[no_mangle]
pub extern "C" fn ldexp(i: f64, x: c_int) -> f64 {
    scalbn(i, x)
}
