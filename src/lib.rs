#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(rustdoc::all)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
// #![warn(clippy::missing_docs_in_private_items)]
#![allow(clippy::must_use_candidate)]
#![no_std]
#![doc = include_str!("../README.md")]
#![allow(clippy::module_name_repetitions)]

extern crate alloc;

include!(concat!(env!("OUT_DIR"), "/register_block.rs"));

pub mod adc;
pub mod clock;
pub mod device;
mod errors;
pub mod led_current;
pub mod measurement_window;
pub mod modes;
mod register;
pub mod system;
pub mod tia;
pub mod value_reading;

// TODO: Prelude.
