//! Software implementations of operations on IEEE 754 floating point numbers in Rust.
//!
//! The implementations are not optimised to use any target-specific features yet. The code is
//! supposed to be very well documented (unlike glibc’s libm) so one could learn from it.
//!
//! This is supposed to be linked into Rust binaries on systems where libm is not available or
//! desirable.
//!
//! Note that this library does not provide some of more obscure functionality of libm such as
//! global state (rounding direction, error reporting, etc).

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
pub use min::*;
pub use max::*;

pub use nearbyint::*;
pub use rint::*;

// The modules are split and grouped by function class.
//
// Operations on signs:
pub mod abs;
pub mod copysign;
// Rounding operations:
pub mod round;
pub mod floor;
pub mod ceil;
pub mod trunc;
pub mod lround;
pub mod llround;
// Simple arithmetic operations:
pub mod dim;
pub mod ma;                         // Needs tests
// Introspection:
pub mod logb;                       // Needs tests
pub mod ilogb;                      // Needs tests
// Other:
pub mod min;
pub mod max;
// Compatibility (possibly non-conforming):
pub mod nearbyint;
pub mod rint; // Also provides lrint and llrint

// Common functionality.
mod utils;
