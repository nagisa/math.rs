extern crate math;
extern crate libloading;
extern crate quickcheck;
extern crate cty;

use std::{f32, f64};
use testutils::*;

#[macro_use]
mod testutils;

// use math::{roundf,round,ceilf,ceil,floorf,floor,truncf,trunc,lroundf,lround,llroundf,llround};

check!(roundf ~ |x: f32| -> f32 {
    math::roundf(x)
},
[
    (0.0), (-0.0), (0.2), (-0.2), (0.5), (-0.5), (0.8), (-0.8), (1.5), (-1.5),
    ( 0.1),  ( 0.25), ( 0.625), (-0.1),  (-0.25), (-0.625),
    ( 2097152.5), (-2097152.5),
    (f32::NEG_INFINITY), (f32::INFINITY), ( f32::NAN), (-f32::NAN)
]);

check!(round ~ |x: f64| -> f64 {
    math::round(x)
},
[
    (0.0), (-0.0), (0.2), (-0.2), (0.5), (-0.5), (0.8), (-0.8), (1.5), (-1.5),
    ( 0.1),  ( 0.25), ( 0.625), (-0.1),  (-0.25), (-0.625),
    ( 2097152.5), (-2097152.5),
    (f64::NEG_INFINITY), (f64::INFINITY), ( f64::NAN), (-f64::NAN)
]);

check!(ceilf ~ |x: f32| -> f32 {
    math::ceilf(x)
},
[
    (0.0), (-0.0),
    (f32::consts::PI), (-f32::consts::PI),
    // ( F32_MIN_SUBNORM), (-F32_MIN_SUBNORM),
    (f32::MIN_POSITIVE), (-f32::MIN_POSITIVE),
    (f32::MAX), (f32::MIN),
    ( 0.1), ( 0.25), ( 0.625), (-0.1),  (-0.25), (-0.625),
    (f32::NEG_INFINITY), (f32::INFINITY),
    ( f32::NAN), (-f32::NAN)
]);

check!(ceil ~ |x: f64| -> f64 {
    math::ceil(x)
},
[
    (0.0), (-0.0),
    (f64::consts::PI), (-f64::consts::PI),
    // ( F64_MIN_SUBNORM), (-F64_MIN_SUBNORM),
    (f64::MIN_POSITIVE), (-f64::MIN_POSITIVE),
    (f64::MAX), (f64::MIN),
    ( 0.1), ( 0.25), ( 0.625), (-0.1),  (-0.25), (-0.625),
    (f64::NEG_INFINITY), (f64::INFINITY),
    ( f64::NAN), (-f64::NAN)
]);

check!(floorf ~ |x: f32| -> f32 {
    math::floorf(x)
},
[
    (0.0), (-0.0),
    (f32::consts::PI), (-f32::consts::PI),
    // ( F32_MIN_SUBNORM), (-F32_MIN_SUBNORM),
    (f32::MIN_POSITIVE), (-f32::MIN_POSITIVE),
    (f32::MAX), (f32::MIN),
    ( 0.1), ( 0.25), ( 0.625), (-0.1),  (-0.25), (-0.625),
    (f32::NEG_INFINITY), (f32::INFINITY),
    ( f32::NAN), (-f32::NAN)
]);

check!(floor ~ |x: f64| -> f64 {
    math::floor(x)
},
[
    (0.0), (-0.0),
    (f64::consts::PI), (-f64::consts::PI),
    // ( F32_MIN_SUBNORM), (-F32_MIN_SUBNORM),
    (f64::MIN_POSITIVE), (-f64::MIN_POSITIVE),
    (f64::MAX), (f64::MIN),
    ( 0.1), ( 0.25), ( 0.625), (-0.1),  (-0.25), (-0.625),
    (f64::NEG_INFINITY), (f64::INFINITY),
    ( f64::NAN), (-f64::NAN)
]);

check!(truncf ~ |x: f32| -> f32 {
    math::truncf(x)
},
[
    (0.0), (-0.0),
    (f32::consts::PI), (-f32::consts::PI),
    // ( F32_MIN_SUBNORM), (-F32_MIN_SUBNORM),
    (f32::MIN_POSITIVE), (-f32::MIN_POSITIVE),
    (f32::MAX), (f32::MIN),
    ( 0.1), ( 0.25), ( 0.625), ( 1.0), ( 1.625), ( 1048580.625), ( 8388610.125),
    ( 4294967296.625), (-0.1), (-0.25), (-0.625), (-1.0), (-1.625), (-1048580.625),
    (-8388610.125), (-4294967296.625),
    (f32::NEG_INFINITY), (f32::INFINITY),
    ( f32::NAN), (-f32::NAN)
]);

check!(trunc ~ |x: f64| -> f64 {
    math::trunc(x)
},
[
    (0.0), (-0.0),
    (f64::consts::PI), (-f64::consts::PI),
    // ( F32_MIN_SUBNORM), (-F32_MIN_SUBNORM),
    (f64::MIN_POSITIVE), (-f64::MIN_POSITIVE),
    (f64::MAX), (f64::MIN),
    ( 0.1), ( 0.25), ( 0.625), ( 1.0), ( 1.625), ( 1048580.625), ( 8388610.125),
    ( 4294967296.625), (-0.1), (-0.25), (-0.625), (-1.0), (-1.625), (-1048580.625),
    (-8388610.125), (-4294967296.625),
    (f64::NEG_INFINITY), (f64::INFINITY),
    ( f64::NAN), (-f64::NAN)
]);

check!(lroundf ~ |x: f32| -> cty::c_long {
    math::lroundf(x)
},
[(0.0), (1.0), (1.6), (-1.5), (-1.4), (-0.01), (2147483647.0)]);

check!(lround ~ |x: f64| -> cty::c_long {
    math::lround(x)
},
[(0.0), (1.0), (1.6), (-1.5), (-1.4), (-0.01), (2147483647.0)]);

check!(llroundf ~ |x: f32| -> cty::c_longlong {
    math::llroundf(x)
},
[(0.0), (1.0), (1.6), (-1.5), (-1.4), (-0.01), (2147483647.0)]);

check!(llround ~ |x: f64| -> cty::c_long {
    math::llround(x)
},
[(0.0), (1.0), (1.6), (-1.5), (-1.4), (-0.01), (2147483647.0)]);
