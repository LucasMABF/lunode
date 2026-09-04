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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Decodable, test_utils::hex};
    use alloc::vec;

    #[test]
    fn outpoint_exact_bytes() {
        let mut first_spent_txid =
            hex("0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9");
        first_spent_txid.reverse();
        let mut pizza_spent_txid =
            hex("12e5bdfd3c73f383802a03b763c0afffdc217fcb38b408cecd1ad15de38595b6");
        pizza_spent_txid.reverse();

        let tests: &[(OutPoint, Vec<u8>)] = &[
            (
                OutPoint {
                    txid: Txid([0; 32]),
                    vout: u32::MAX,
                },
                hex("0000000000000000000000000000000000000000000000000000000000000000ffffffff"),
            ),
            (
                OutPoint {
                    txid: Txid(first_spent_txid.try_into().unwrap()),
                    vout: 0,
                },
                hex("c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd370400000000"),
            ),
            (
                OutPoint {
                    txid: Txid(pizza_spent_txid.try_into().unwrap()),
                    vout: 42,
                },
                hex("b69585e35dd11acdce08b438cb7f21dcffafc063b7032a8083f3733cfdbde5122a000000"),
            ),
        ];

        for (i, (value, expected)) in tests.iter().enumerate() {
            let mut bytes = Vec::new();
            value.encode(&mut bytes);
            assert_eq!(bytes.len(), 36, "row: {i}");
            assert_eq!(&bytes, expected, "row: {i}");

            let mut cursor = bytes.as_slice();
            assert_eq!(OutPoint::decode(&mut cursor), Ok(*value), "row: {i}");
            assert!(cursor.is_empty(), "row: {i}");
        }
    }

    #[test]
    fn txin_exact_bytes() {
        let tx_in = TxIn {
            prevout: OutPoint {
                txid: Txid([0; 32]),
                vout: u32::MAX,
            },
            script_sig: Script(hex(
                "04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73",
            )),
            sequence: 0xffffffff,
        };
        let expected = hex(
            "0000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff",
        );

        let mut bytes = Vec::new();
        tx_in.encode(&mut bytes);
        assert_eq!(bytes, expected);

        let mut cursor = bytes.as_slice();
        assert_eq!(TxIn::decode(&mut cursor), Ok(tx_in));
        assert!(cursor.is_empty());
    }

    #[test]
    fn txout_exact_bytes() {
        let tx_out = TxOut {
            value: Amount(5_000_000_000),
            script_pubkey: Script(hex(
                "4104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac",
            )),
        };
        let expected = hex(
            "00f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac",
        );

        let mut bytes = Vec::new();
        tx_out.encode(&mut bytes);
        assert_eq!(bytes, expected);

        let mut cursor = bytes.as_slice();
        assert_eq!(TxOut::decode(&mut cursor), Ok(tx_out));
        assert!(cursor.is_empty());
    }

    #[test]
    fn transaction_exact_bytes() {
        let mut genesis_coinbase_txid =
            hex("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b");
        genesis_coinbase_txid.reverse();
        let mut first_payment_txid =
            hex("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        first_payment_txid.reverse();
        let mut first_spent_txid =
            hex("0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9");
        first_spent_txid.reverse();

        let tests: &[(Transaction, Vec<u8>, Txid)] = &[
            (
                Transaction {
                    version: 1,
                    inputs: vec![TxIn {
                        prevout: OutPoint {
                            txid: Txid([0; 32]),
                            vout: u32::MAX,
                        },
                        script_sig: Script(hex(
                            "04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73",
                        )),
                        sequence: 0xffffffff,
                    }],
                    outputs: vec![TxOut {
                        value: Amount(5_000_000_000),
                        script_pubkey: Script(hex(
                            "4104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac",
                        )),
                    }],
                    lock_time: 0,
                },
                hex(
                    "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
                ),
                Txid(genesis_coinbase_txid.try_into().unwrap()),
            ),
            (
                Transaction {
                    version: 1,
                    inputs: vec![TxIn {
                        prevout: OutPoint {
                            txid: Txid(first_spent_txid.try_into().unwrap()),
                            vout: 0,
                        },
                        script_sig: Script(hex(
                            "47304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901",
                        )),
                        sequence: 0xffffffff,
                    }],
                    outputs: vec![
                        TxOut {
                            value: Amount(1_000_000_000),
                            script_pubkey: Script(hex(
                                "4104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac",
                            )),
                        },
                        TxOut {
                            value: Amount(4_000_000_000),
                            script_pubkey: Script(hex(
                                "410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac",
                            )),
                        },
                    ],
                    lock_time: 0,
                },
                hex(
                    "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000",
                ),
                Txid(first_payment_txid.try_into().unwrap()),
            ),
        ];

        for (i, (value, expected, txid)) in tests.iter().enumerate() {
            let mut bytes = Vec::new();
            value.encode(&mut bytes);
            assert!(&bytes == expected, "row: {i}");
            assert_eq!(value.hash(), *txid, "row: {i}");

            let mut cursor = bytes.as_slice();
            assert!(
                Transaction::decode(&mut cursor).as_ref() == Ok(value),
                "row: {i}"
            );
            assert!(cursor.is_empty(), "row: {i}");
        }
    }
}
