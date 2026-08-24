use crate::serialize::impl_consensus_encoding;

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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Decodable, Encodable, test_utils::hex};
    use alloc::vec::Vec;

    #[test]
    fn exact_bytes() {
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
}
