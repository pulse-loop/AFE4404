#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Documentation lints
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(invalid_doc_attributes)]
#![warn(rustdoc::all)]

#![no_std]

// Direct import of main struct.
pub use afe4404::AFE4404;

mod afe4404;
mod register;
