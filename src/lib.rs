#![crate_name="math"]
#![crate_type="rlib"]
// #![no_std]
#![experimental]

#![feature(phase)]

#[phase(plugin, link)]
extern crate core;

// #[cfg(test)]
// #[phase(plugin, link)]
// extern crate std;

mod pow;
mod utils;
