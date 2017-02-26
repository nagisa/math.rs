extern crate math;
extern crate libloading;
extern crate quickcheck;

use testutils::*;

#[macro_use]
mod testutils;

check!(sin ~ |x: f64| -> f64 {
    math::sin(x)
});

check!(cos ~ |x: f64| -> f64 {
    math::cos(x)
});

check!(tan ~ |x: f64| -> f64 {
    math::tan(x)
});
