use crate::{Encodable, Hash256, serialize::impl_consensus_encoding};

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

    use crate::{Decodable, DecodeError};
    use alloc::vec::Vec;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn genesis_block() {
        let mut merkle_root_hash =
            hex("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b");
        merkle_root_hash.reverse();

        let genesis = BlockHeader {
            version: 1,
            prev_block_hash: BlockHash([0_u8; 32]),
            merkle_root: MerkleRoot(merkle_root_hash.try_into().unwrap()),
            time: 1231006505,
            bits: CompactTarget(0x1d00ffff),
            nonce: 0x7c2bac1d,
        };

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

        let mut slice = genesis_header_hex.as_slice();
        assert_eq!(genesis, BlockHeader::decode(&mut slice).unwrap());
        assert_eq!(slice.len(), 0);

        let mut slice = &genesis_header_hex[..78];
        assert_eq!(
            BlockHeader::decode(&mut slice),
            Err(DecodeError::UnexpectedEnd)
        );
    }
}
