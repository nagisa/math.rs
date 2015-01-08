//! Software implementations of operations on IEEE 754 floating point numbers in Rust.
//!
//! The implementations are not optimised to use any target-specific features yet. The code is
//! supposed to be very well documented (unlike glibc’s libm) so one could learn from it.
//!
//! This is supposed to be linked into Rust binaries on systems where libm is not available or
//! desirable.
//!
//! Note that this library does not provide some of more obscure functionality of libm such as
//! global state (rounding direction, error reporting, etc). Functions that deviate from the libm
//! are explicitly marked.

#![crate_name="math"]
#![crate_type="rlib"]
// Since this is package provides very basic operations, our only dependencies will be Rust’s
// libcore.
#![no_std]
#![experimental]

#[macro_use]
extern crate core;

#[cfg(test)]
#[macro_use]
extern crate std;

// TODO: This dependency should not exist. We need to know types for c_long and c_longlong, though.
extern crate libc;


// Reexport everything
pub use abs::*;
pub use copysign::*;
pub use round::*;
pub use floor::*;
pub use ceil::*;
pub use trunc::*;
pub use lround::*;
pub use llround::*;

pub use dim::*;
pub use ma::*;

pub use logb::*;
pub use ilogb::*;
pub use scalbn::*;
pub use scalbln::*;
pub use modf::*;
pub use nextafter::*;

pub use min::*;
pub use max::*;
pub use hypot::*;

pub use nearbyint::*;
pub use rint::*;
pub use ldexp::*;

// The modules are split and grouped by function class.
//
// Operations on signs:
mod abs;
mod copysign;
// Rounding operations:
mod round;
mod floor;
mod ceil;
mod trunc;
mod lround;
mod llround;
// Simple arithmetic operations:
mod dim;
mod ma;                         // Needs tests
// Introspection:
mod logb;
mod ilogb;
mod scalbn;
mod scalbln;
mod modf;
mod nextafter;
// Other:
mod min;
mod max;
mod hypot;
// Compatibility (possibly non-conforming):
mod nearbyint;
mod rint; // Also provides lrint and llrint
mod ldexp;

// Common functionality.
mod utils;
