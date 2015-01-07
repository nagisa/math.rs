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
pub use signops::*;
pub use rounding::*;
pub use arithmetic::*;

// The modules are split and grouped by functionality, not data type size, architecture or any
// other arbitrary property.
//
// Operations on signs (absolute, copysign, …).
pub mod signops;
// Rounding operations (round, floor, ceil, truncate, …).
pub mod rounding;
// Simple arithmetic operations (positive difference, modulo operation, min, max, …).
pub mod arithmetic;

// Common functionality.
mod utils;
