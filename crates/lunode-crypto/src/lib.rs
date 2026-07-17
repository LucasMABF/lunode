//! Cryptographic primitives, implemented from scratch in pure Rust.

#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod sha256;
pub use sha256::Sha256;
pub use sha256::sha256;
