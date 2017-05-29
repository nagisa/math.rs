// TODO: remove this
use libc::{c_int, c_long};

use scalbln::{scalblnf, scalbln};

/// Multiply the 32-bit floating-point number by integral power of radix.
#[no_mangle]
#[inline]
pub extern "C" fn scalbnf(i: f32, x: c_int) -> f32 {
    scalblnf(i, x as c_long)
}

/// Multiply the 64-bit floating-point number by integral power of radix.
#[no_mangle]
#[inline]
pub extern "C" fn scalbn(i: f64, x: c_int) -> f64 {
    scalbln(i, x as c_long)
}
