//! Bitcoin consensus rules.

#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

mod merkle;
pub use merkle::compute_merkle_root;
