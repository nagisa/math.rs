// Non-conforming compatibility definitions.
use round::{roundf, round};

/// Round the 32-bit floating-point number away from zero.
///
/// Alias to roundf.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// current global state such as rounding direction and will always round away from zero.
#[no_mangle]
pub extern "C" fn nearbyintf(i: f32) -> f32 {
    roundf(i)
}

/// Round the 64-bit floating-point number away from zero.
///
/// Alias to round.
///
/// Note: this function behaves differently compared to libm implementation. It will not use
/// current global state such as rounding direction and will always round away from zero.
#[no_mangle]
pub extern "C" fn nearbyint(i: f64) -> f64 {
    round(i)
}
