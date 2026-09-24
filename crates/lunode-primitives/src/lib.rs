//! Bitcoin primitive types and hash constructions.

#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

mod hash;
pub use hash::Hash160;
pub use hash::Hash256;
pub use hash::hash160;
pub use hash::hash256;

mod serialize;
pub use serialize::Decodable;
pub use serialize::DecodeError;
pub use serialize::Encodable;
pub use serialize::Reader;
pub use serialize::Writer;

mod compact_size;
pub use compact_size::CompactSize;

mod vector;

mod block;
pub use block::Block;
pub use block::BlockHash;
pub use block::BlockHeader;
pub use block::CompactTarget;
pub use block::MerkleRoot;

mod transaction;
pub use transaction::OutPoint;
pub use transaction::Transaction;
pub use transaction::TxIn;
pub use transaction::TxOut;
pub use transaction::Txid;

mod script;
pub use script::Script;

mod amount;
pub use amount::Amount;
