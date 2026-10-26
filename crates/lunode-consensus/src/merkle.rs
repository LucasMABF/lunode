use alloc::vec::Vec;
use lunode_primitives::{Hash256, MerkleRoot, Transaction};

/// Computes the Merkle root of `hashes`.
///
/// Returns `None` if `hashes` is empty.
///
/// ```
/// let root = lunode_consensus::compute_merkle_root(vec![[0; 32]]);
/// ```
pub fn compute_merkle_root(mut hashes: Vec<[u8; 32]>) -> Option<[u8; 32]> {
    while hashes.len() > 1 {
        if hashes.len() % 2 == 1 {
            hashes.push(hashes[hashes.len() - 1]);
        }

        for i in 0..hashes.len() / 2 {
            let mut hash = Hash256::new();
            hash.update(&hashes[2 * i]);
            hash.update(&hashes[2 * i + 1]);
            hashes[i] = hash.finalize();
        }

        hashes.truncate(hashes.len() / 2);
    }

    hashes.first().copied()
}

/// Computes the Merkle root of `transactions`.
///
/// Returns `None` if `transactions` is empty.
///
/// ```
/// let root = lunode_consensus::merkle_root(&[]);
/// ```
pub fn merkle_root(transactions: &[Transaction]) -> Option<MerkleRoot> {
    let mut hashes = Vec::with_capacity((transactions.len() + 1) & !1);
    for tx in transactions {
        hashes.push(tx.hash().0);
    }

    compute_merkle_root(hashes).map(MerkleRoot)
}
