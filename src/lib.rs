#![warn(missing_docs, unreachable_pub)]
// TODO: Add documentation.
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(rustdoc::all)]
// #![allow(clippy::multiple_crate_versions)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
// #![warn(clippy::missing_docs_in_private_items)]

// #![no_std]
// TODO: Migrate to no_std.

// Direct import of main struct.
pub use afe4404::AFE4404;

pub use uom;

include!(concat!(env!("OUT_DIR"), "/register_block.rs"));

/// A driver for the AFE4404 pulse oximeter analog frontend.
pub mod afe4404;
mod errors;
pub mod high_level;
mod register;
