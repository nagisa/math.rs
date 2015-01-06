#![crate_name="math"]
#![crate_type="rlib"]
#![no_std]
#![experimental]

#![feature(phase)]

#[phase(plugin, link)]
extern crate core;

// For types
extern crate libc;

#[cfg(test)]
#[phase(plugin, link)]
extern crate std;

// Absolute values
pub mod abs;
// Positive difference
pub mod dim;

// Copy sign
pub mod sign;

// Round, Ceil, Floor, Truncate
pub mod rounding;

//
mod pow;
