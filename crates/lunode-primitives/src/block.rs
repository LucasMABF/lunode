use crate::{Encodable, Hash256, Transaction, serialize::impl_consensus_encoding};
use alloc::vec::Vec;

/// A block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// The block header.
    pub header: BlockHeader,

    /// The block's transactions.
    pub transactions: Vec<Transaction>,
}

impl_consensus_encoding!(Block, header, transactions);

/// A block header.
///
/// ```
/// use lunode_primitives::{BlockHash, BlockHeader, CompactTarget, MerkleRoot};
///
/// let header = BlockHeader {
///     version: 1,
///     prev_block_hash: BlockHash([0; 32]),
///     merkle_root: MerkleRoot([0; 32]),
///     time: 1231006505,
///     bits: CompactTarget(0x1d00ffff),
///     nonce: 2083236893,
/// };
/// let hash = header.hash();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    /// The block version.
    pub version: i32,

    /// The hash of the previous block's header.
    pub prev_block_hash: BlockHash,

    /// The Merkle root of the block's transactions.
    pub merkle_root: MerkleRoot,

    /// The block timestamp (Unix time).
    pub time: u32,

    /// The proof-of-work target, in compact form.
    pub bits: CompactTarget,

    /// The proof-of-work nonce.
    pub nonce: u32,
}

impl BlockHeader {
    /// Computes the block hash.
    pub fn hash(&self) -> BlockHash {
        let mut hasher = Hash256::new();
        self.encode(&mut hasher);
        BlockHash(hasher.finalize())
    }
}

impl_consensus_encoding!(
    BlockHeader,
    version,
    prev_block_hash,
    merkle_root,
    time,
    bits,
    nonce,
);

/// A proof-of-work target, in compact form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompactTarget(pub u32);

impl_consensus_encoding!(CompactTarget);

/// A block hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockHash(pub [u8; 32]);

impl_consensus_encoding!(BlockHash);

/// A Merkle root.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MerkleRoot(pub [u8; 32]);

impl_consensus_encoding!(MerkleRoot);
