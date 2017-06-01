use utils::AsBits;
use utils::F32_EXP_MASK;
use utils::F64_EXP_MASK;

use utils::Float;

/// Euclidean distance function. 32-bit floating-point version.
///
/// Calculates hypotenuse of right-angled triange with sides l and r. This is also known as a
/// distance between points (0, 0) and (x, y).
#[no_mangle]
#[inline]
pub extern "C" fn hypotf(l: f32, r: f32) -> f32 {
    let lbits = l.abs().as_bits();
    let rbits = r.abs().as_bits();
    if lbits == F32_EXP_MASK {
        return if l == r {
            r.abs()
        } else {
            l.abs()
        };
    } else if rbits == F32_EXP_MASK {
        return if l == r {
            l.abs()
        } else {
            r.abs()
        };
    } else if lbits > F32_EXP_MASK && rbits > F32_EXP_MASK {
        return r + l;
    } else if lbits == 0 {
        return r.abs();
    } else if rbits == 0 {
        return l.abs();
    }
    let (dl, dr) = (l as f64, r as f64);
    (dl * dl + dr * dr).sqrt() as f32
}

// TODO: can be improved wrt error bounds
/// Euclidean distance function. 64-bit floating-point version.
///
/// Calculates hypotenuse of right-angled triange with sides l and r. This is also known as a
/// distance between points (0, 0) and (x, y).
#[no_mangle]
#[inline]
pub extern "C" fn hypot(l: f64, r: f64) -> f64 {
    let lbits = l.abs().as_bits();
    let rbits = r.abs().as_bits();
    if lbits == F64_EXP_MASK {
        return if l == r {
            r.abs()
        } else {
            l.abs()
        };
    } else if rbits == F64_EXP_MASK {
        return if l == r {
            l.abs()
        } else {
            r.abs()
        };
    } else if lbits > F64_EXP_MASK && rbits > F64_EXP_MASK {
        return r + l;
    } else if lbits == 0 {
        return r.abs();
    } else if rbits == 0 {
        return l.abs();
    }
    ((l * l) + (r * r)).sqrt()
}
