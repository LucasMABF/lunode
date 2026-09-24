#![allow(dead_code)]
use lunode_primitives::{Amount, OutPoint, Script, Transaction, TxIn, TxOut, Txid};

pub fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub fn genesis_coinbase() -> Transaction {
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
    }
}
