//! Bitcoin primitive types and hash constructions.

#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod hash;
pub use hash::Hash160;
pub use hash::Hash256;
pub use hash::hash160;
pub use hash::hash256;
