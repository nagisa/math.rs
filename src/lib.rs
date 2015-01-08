//! Software implementations of operations on IEEE 754 floating point numbers in Rust.
//!
//! The implementations are not optimised to use any. The code is supposed to be very well
//! documented (as if!) so one could learn from it.
//!
//! # Recommendations
//!
//! This library is indended to be used with Rust programs on targets where libm is not available
//! or linking to it is undesirable. It does not provide 1-to-1 feature parity with libm. Namely:
//!
//! * Any global state is not updated or used (this includes errors, rounding direction etc);
//! * Several functions are omitted. Currently:
//!   * nan, nanf – reliance on libc functions;
//!   * Functions that take or return `long double` – `long double` is usually not a standard IEEE
//!     754 type.
//!
//! # Usage
//!
//! In order to use this library nothing special needs to be done. Use regular functions from the
//! `std` or `core` and simply link to this library. In case the target platform has no hardware
//! support for some operation, software implementations provided by this library will be used
//! automagically.
// TODO: provide instructions to override default libm link and how to link to this library.

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

// TODO: This dependency should not exist. We need to know types for c_{int,long,longlong} though.
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
