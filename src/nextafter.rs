use utils::{AsBits, Bits};
use utils::F32_SIGN_MASK;
use utils::F64_SIGN_MASK;

#[cfg(not(test))]
use utils::Float;

/// Calculate next representable floating-point value following `i` in direction of `d`
///
/// * If `d` < `i`, function returns largest representable number less than `i`;
/// * If `d` > `i`, function returns smallest representable number more than `i`;
/// * If `d` = `i`, function returns `d`.
#[no_mangle]
#[inline]
pub extern "C" fn nextafterf(i: f32, d: f32) -> f32 {
    let mut ibits = i.as_bits();
    let dbits = d.as_bits();
    if i == d {
        return d;
    } else if i.is_nan() || d.is_nan() {
        return i + d;
    } else if ibits & !F32_SIGN_MASK == 0 {
        // Input is 0. Return ± minimal subnormal.
        return ((dbits & F32_SIGN_MASK) | 1).from_bits();
        // Disrepancy: no underflow flag
    } else if (ibits as i32) < 0 {
        // Input is positive
        if ibits > dbits {
            ibits -= 1;
        } else {
            ibits += 1;
        }
    } else {
        // Input is negative
        if (dbits as i32) < 0 || (ibits as i32) > (dbits as i32) {
            ibits -= 1;
        } else {
            ibits += 1;
        }
    }
    // Disrepancy: no underflow/overflow check
    ibits.from_bits()
}

/// Calculate next representable floating-point value following `i` in direction of `d`
///
/// * If `d` < `i`, function returns largest representable number less than `i`;
/// * If `d` > `i`, function returns smallest representable number more than `i`;
/// * If `d` = `i`, function returns `d`.
#[no_mangle]
#[inline]
pub extern "C" fn nextafter(i: f64, d: f64) -> f64 {
    let mut ibits = i.as_bits();
    let dbits = d.as_bits();
    if i == d {
        return d;
    } else if i.is_nan() || d.is_nan() {
        return i + d;
    } else if ibits & !F64_SIGN_MASK == 0 {
        // Input is 0. Return ± minimal subnormal.
        return ((dbits & F64_SIGN_MASK) | 1).from_bits();
        // Disrepancy: no underflow flag
    } else if (ibits as i64) < 0 {
        // Input is positive
        if ibits > dbits {
            ibits -= 1;
        } else {
            ibits += 1;
        }
    } else {
        // Input is negative
        if (dbits as i64) < 0 || (ibits as i64) > (dbits as i64) {
            ibits -= 1;
        } else {
            ibits += 1;
        }
    }
    // Disrepancy: no underflow/overflow check
    ibits.from_bits()
}
