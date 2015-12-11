// Non-conforming compatibility definitions.
use round::{roundf, round};
use lround::{lroundf, lround};
use llround::{llroundf, llround};
use libc::{c_long, c_longlong};

/// Round the 32-bit floating-point number away from zero.
///
/// Alias to roundf.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// or set current global state such as rounding direction or error reporting and will always round
/// away from zero.
#[no_mangle]
pub extern "C" fn rintf(i: f32) -> f32 {
    roundf(i)
}

/// Round the 64-bit floating-point number away from zero.
///
/// Alias to round.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// or set current global state such as rounding direction or error reporting and will always round
/// away from zero.
#[no_mangle]
pub extern "C" fn rint(i: f64) -> f64 {
    round(i)
}

/// Round the 32-bit floating-point number away from zero.
///
/// Alias to lroundf.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// or set current global state such as rounding direction or error reporting and will always round
/// away from zero.
#[no_mangle]
pub extern "C" fn lrintf(i: f32) -> c_long {
    lroundf(i)
}

/// Round the 64-bit floating-point number away from zero.
///
/// Alias to lround.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// or set current global state such as rounding direction or error reporting and will always round
/// away from zero.
#[no_mangle]
pub extern "C" fn lrint(i: f64) -> c_long {
    lround(i)
}

/// Round the 32-bit floating-point number away from zero.
///
/// Alias to lroundf.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// or set current global state such as rounding direction or error reporting and will always round
/// away from zero.
#[no_mangle]
pub extern "C" fn llrintf(i: f32) -> c_longlong {
    llroundf(i)
}

/// Round the 64-bit floating-point number away from zero.
///
/// Alias to lround.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// or set current global state such as rounding direction or error reporting and will always round
/// away from zero.
#[no_mangle]
pub extern "C" fn llrint(i: f64) -> c_longlong {
    llround(i)
}
