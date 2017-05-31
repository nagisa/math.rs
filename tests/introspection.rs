extern crate math;
extern crate libloading;
extern crate quickcheck;
extern crate cty;

#[macro_use]
mod testutils;

use std::{f32, f64};
use testutils::*;

check!(logbf ~ |x: f32| -> f32 {
    math::logbf(x)
},
[
    (0.0), (0.001), (-0.001), (2.0), (4.0), (10.0),
    (5.877472E-39), (1.4E-45),
    (f32::NAN), (f32::INFINITY), (f32::NEG_INFINITY)
]);

check!(logb ~ |x: f64| -> f64 {
    math::logb(x)
},
[
    (0.0), (0.001), (-0.001), (2.0), (4.0), (10.0),
    // (1.11253692925360069154511635867E-308), (4.94065645841246544176568792868E-324),
    (f64::NAN), (f64::INFINITY), (f64::NEG_INFINITY)
]);

check!(ilogbf ~ |x: f32| -> cty::c_int {
    math::ilogbf(x)
},
[
    (0.0), (0.001), (-0.001), (2.0), (4.0), (10.0),
    (5.877472E-39), (1.4E-45),
    (f32::NAN), (f32::INFINITY), (f32::NEG_INFINITY)
]);

check!(ilogb ~ |x: f64| -> cty::c_int {
    math::ilogb(x)
},
[
    (0.0), (0.001), (-0.001), (2.0), (4.0), (10.0),
    // (1.11253692925360069154511635867E-308), (4.94065645841246544176568792868E-324),
    (f64::NAN), (f64::INFINITY), (f64::NEG_INFINITY)
]);

check!(scalbnf ~ |x: f32, y: cty::c_int| -> f32 {
    math::scalbnf(x, y)
},
[
    (0.0, 100), (-0.0, 100), (0.0, 0),   (2.0, 2),
    (1.1754942E-38, -23), // Goes out of range
    (1.4E-45, -1),        // Out of range
    (1.1754942E-38, -1),  // Stays denormal
    (1.1754942E-38, 1),   // Becomes normal
    (1.4E-45, 24),        // Becomes normal
    // special cases
    (f32::NAN, 100), (f32::INFINITY, -100), (f32::NEG_INFINITY, 100), (1.0, 128), (-1.0, 128)
]);

check!(scalbn ~ |x: f64, y: cty::c_int| -> f64 {
    math::scalbn(x, y)
},
[
    (0.0, 100), (-0.0, 100), (0.0, 0),   (2.0, 2),
    // (1.11253692925360069154511635867E-308, -52), // Goes out of range
    // (4.94065645841246544176568792868E-324, -1),  // Out of range
    // (1.11253692925360069154511635867E-308, -1),  // Stays denormal
    // (1.11253692925360069154511635867E-308, 1),   // Becomes normal
    // (4.94065645841246544176568792868E-324, 52),  // Becomes normal
    // special cases
    (f64::NAN, 100), (f64::INFINITY, -100), (f64::NEG_INFINITY, 100), (1.0, 1024), (-1.0, 1024)
]);

check!(modff ~ |x: f32| -> (f32, f32) {
    let mut y = 0.0f32;
    let x = math::modff(x, &mut y as *mut _);
    (x, y)
}, |f: unsafe extern fn(f32, *mut f32) -> f32, x| {
    let mut y = 0.0f32;
    let x = unsafe { f(x, &mut y as *mut _) };
    (x, y)
}, [
    (2.4), (f32::NAN), (f32::INFINITY), (f32::NEG_INFINITY)
]);

check!(modf ~ |x: f64| -> (f64, f64) {
    let mut y = 0.0f64;
    let x = math::modf(x, &mut y as *mut _);
    (x, y)
}, |f: unsafe extern fn(f64, *mut f64) -> f64, x| {
    let mut y = 0.0f64;
    let x = unsafe { f(x, &mut y as *mut _) };
    (x, y)
}, [
    (2.4), (f64::NAN), (f64::INFINITY), (f64::NEG_INFINITY)
]);

check!(nextafterf ~ |x: f32, y: f32| -> f32 {
    math::nextafterf(x, y)
},
[
    (1.0, 10.0), (1.0, 0.0), (1.0, 1.0),
    (1.4E-45, f32::NEG_INFINITY), (0.0, f32::INFINITY), (0.0, f32::NEG_INFINITY),
    (f32::NAN, 0.0), (0.0, f32::NAN)
]);

check!(nextafter ~ |x: f64, y: f64| -> f64 {
    math::nextafter(x, y)
},
[
    (1.0, 10.0), (1.0, 0.0), (1.0, 1.0),
    (1.4E-45, f64::NEG_INFINITY), (0.0, f64::INFINITY), (0.0, f64::NEG_INFINITY),
    (f64::NAN, 0.0), (0.0, f64::NAN)
]);
