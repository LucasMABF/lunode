use crate::{Amount, Script, serialize::impl_consensus_encoding};

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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Decodable, Encodable, test_utils::hex};
    use alloc::vec::Vec;

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
}
