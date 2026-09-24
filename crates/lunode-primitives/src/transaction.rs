use crate::{Amount, Encodable, Hash256, Script, serialize::impl_consensus_encoding};
use alloc::vec::Vec;

/// A transaction identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Txid(pub [u8; 32]);

impl_consensus_encoding!(Txid);

/// A reference to a transaction output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutPoint {
    /// The transaction containing the output.
    pub txid: Txid,
    /// The index of the output within that transaction.
    pub vout: u32,
}

impl_consensus_encoding!(OutPoint, txid, vout);

/// A transaction input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxIn {
    /// The output being spent.
    pub prevout: OutPoint,

    /// The script that unlocks the spent output.
    pub script_sig: Script,

    /// The sequence number, which signals replaceability and relative locktime.
    pub sequence: u32,
}

impl_consensus_encoding!(TxIn, prevout, script_sig, sequence);

/// A transaction output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxOut {
    /// The amount paid.
    pub value: Amount,

    /// The script that must be satisfied to spend it.
    pub script_pubkey: Script,
}

impl_consensus_encoding!(TxOut, value, script_pubkey);

/// A transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    /// The transaction version.
    pub version: u32,

    /// The transaction inputs.
    pub inputs: Vec<TxIn>,

    /// The transaction outputs.
    pub outputs: Vec<TxOut>,

    /// The earliest block height or time at which the transaction may be mined.
    pub lock_time: u32,
}

impl_consensus_encoding!(Transaction, version, inputs, outputs, lock_time);

impl Transaction {
    /// Computes the transaction identifier.
    pub fn hash(&self) -> Txid {
        let mut hasher = Hash256::new();
        self.encode(&mut hasher);
        Txid(hasher.finalize())
    }
}
