#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// Documentation lints
// #![warn(clippy::missing_docs_in_private_items)]
#![warn(invalid_doc_attributes)]
#![warn(rustdoc::all)]

// #![no_std]
// TODO: Migrate to no_std.

// Direct import of main struct.
pub use afe4404::AFE4404;

include!(concat!(env!("OUT_DIR"), "/register_block.rs"));

/// A driver for the AFE4404 pulse oximeter analog frontend.
mod afe4404;
mod register;

mod high_level;
