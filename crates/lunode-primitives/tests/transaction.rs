use lunode_primitives::{
    Amount, Decodable, Encodable, OutPoint, Script, Transaction, TxIn, TxOut, Txid, hash256,
};
use proptest::prelude::*;

fn arb_txid() -> impl Strategy<Value = Txid> {
    any::<[u8; 32]>().prop_map(Txid)
}

fn arb_short_script() -> impl Strategy<Value = Script> {
    prop::collection::vec(any::<u8>(), 0..=100).prop_map(Script)
}

fn arb_outpoint() -> impl Strategy<Value = OutPoint> {
    (arb_txid(), any::<u32>()).prop_map(|(txid, vout)| OutPoint { txid, vout })
}

fn arb_txin() -> impl Strategy<Value = TxIn> {
    (arb_outpoint(), arb_short_script(), any::<u32>()).prop_map(
        |(prevout, script_sig, sequence)| TxIn {
            prevout,
            script_sig,
            sequence,
        },
    )
}

fn arb_amount() -> impl Strategy<Value = Amount> {
    any::<u64>().prop_map(Amount)
}

fn arb_txout() -> impl Strategy<Value = TxOut> {
    (arb_amount(), arb_short_script()).prop_map(|(value, script_pubkey)| TxOut {
        value,
        script_pubkey,
    })
}

fn arb_transaction() -> impl Strategy<Value = Transaction> {
    (
        any::<u32>(),
        prop::collection::vec(arb_txin(), 0..=5),
        prop::collection::vec(arb_txout(), 0..=5),
        any::<u32>(),
    )
        .prop_map(|(version, inputs, outputs, lock_time)| Transaction {
            version,
            inputs,
            outputs,
            lock_time,
        })
}

proptest! {
    #[test]
    fn transaction_roundtrip(transaction in arb_transaction()) {
        let mut bytes: Vec<u8> = Vec::new();
        transaction.encode(&mut bytes);

        prop_assert_eq!(hash256(&bytes), transaction.hash().0);

        let mut cursor = bytes.as_slice();
        prop_assert_eq!(Transaction::decode(&mut cursor).unwrap(), transaction);
        prop_assert!(cursor.is_empty());
    }
}
