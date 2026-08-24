use lunode_primitives::{
    BlockHash, BlockHeader, CompactTarget, Decodable, Encodable, MerkleRoot, hash256,
};
use proptest::prelude::*;

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
