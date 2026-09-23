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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Decodable,
        test_utils::{genesis_coinbase, hex},
    };
    use alloc::vec;

    fn genesis_header() -> BlockHeader {
        let mut merkle_root_hash =
            hex("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b");
        merkle_root_hash.reverse();

        BlockHeader {
            version: 1,
            prev_block_hash: BlockHash([0_u8; 32]),
            merkle_root: MerkleRoot(merkle_root_hash.try_into().unwrap()),
            time: 1231006505,
            bits: CompactTarget(0x1d00ffff),
            nonce: 0x7c2bac1d,
        }
    }

    #[test]
    fn block_header_exact_bytes() {
        let genesis = genesis_header();

        let mut bytes: Vec<u8> = Vec::new();
        genesis.encode(&mut bytes);
        assert_eq!(bytes.len(), 80);

        let genesis_header_hex = hex(
            "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c",
        );
        assert_eq!(bytes, genesis_header_hex);

        let mut expected_hash =
            hex("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        expected_hash.reverse();

        assert_eq!(genesis.hash(), BlockHash(expected_hash.try_into().unwrap()));

        let mut cursor = genesis_header_hex.as_slice();
        assert_eq!(genesis, BlockHeader::decode(&mut cursor).unwrap());
        assert!(cursor.is_empty());
    }

    #[test]
    fn block_exact_bytes() {
        let header = genesis_header();
        let coinbase = genesis_coinbase();
        let block = Block {
            header,
            transactions: vec![coinbase],
        };

        let mut bytes: Vec<u8> = Vec::new();
        block.encode(&mut bytes);

        let genesis_block_hex = hex(
            "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
        );
        assert_eq!(bytes, genesis_block_hex);

        let mut cursor = genesis_block_hex.as_slice();
        assert_eq!(block, Block::decode(&mut cursor).unwrap());
        assert!(cursor.is_empty());
    }
}
