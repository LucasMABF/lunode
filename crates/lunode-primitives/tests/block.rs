use lunode_primitives::{
    Block, BlockHash, BlockHeader, CompactTarget, Decodable, Encodable, MerkleRoot, hash256,
};
use proptest::prelude::*;

mod common;
use common::{genesis_coinbase, hex};

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

    let mut expected_hash = hex("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
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

fn arb_block_hash() -> impl Strategy<Value = BlockHash> {
    any::<[u8; 32]>().prop_map(BlockHash)
}

fn arb_merkle_root() -> impl Strategy<Value = MerkleRoot> {
    any::<[u8; 32]>().prop_map(MerkleRoot)
}

fn arb_compact_target() -> impl Strategy<Value = CompactTarget> {
    any::<u32>().prop_map(CompactTarget)
}

fn arb_block_header() -> impl Strategy<Value = BlockHeader> {
    (
        any::<i32>(),
        arb_block_hash(),
        arb_merkle_root(),
        any::<u32>(),
        arb_compact_target(),
        any::<u32>(),
    )
        .prop_map(
            |(version, prev_block_hash, merkle_root, time, bits, nonce)| BlockHeader {
                version,
                prev_block_hash,
                merkle_root,
                time,
                bits,
                nonce,
            },
        )
}

proptest! {
    #[test]
    fn block_header_roundtrip(header in arb_block_header()) {
        let mut bytes: Vec<u8> = Vec::new();
        header.encode(&mut bytes);

        prop_assert_eq!(bytes.len(), 80);
        prop_assert_eq!(hash256(&bytes), header.hash().0);

        let mut cursor = bytes.as_slice();
        prop_assert_eq!(BlockHeader::decode(&mut cursor).unwrap(), header);
        prop_assert!(cursor.is_empty());
    }
}
