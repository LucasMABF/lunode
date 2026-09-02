use lunode_primitives::{Decodable, DecodeError, Encodable, Script};
use proptest::prelude::*;

#[test]
fn exact_bytes() {
    let tests: &[(Result<Script, DecodeError>, Vec<u8>)] = &[
        (Ok(Script(vec![])), vec![0]),
        (
            Ok(Script(vec![0xaa, 0xbb, 0xcc])),
            vec![3, 0xaa, 0xbb, 0xcc],
        ),
        (
            Ok(Script(vec![0xab; 252])),
            [vec![0xfc], vec![0xab; 252]].concat(),
        ),
        (
            Ok(Script(vec![0xab; 253])),
            [vec![0xfd, 0xfd, 0x00], vec![0xab; 253]].concat(),
        ),
        (
            Ok(Script(vec![0; 5_000_001])),
            [
                vec![0xfe],
                5_000_001_u32.to_le_bytes().to_vec(),
                vec![0; 5_000_001],
            ]
            .concat(),
        ),
        (Err(DecodeError::UnexpectedEnd), vec![0x01]),
        (Err(DecodeError::UnexpectedEnd), vec![0x03, 0xaa]),
        (
            Err(DecodeError::NonCanonical),
            vec![0xfd, 0x03, 0x00, 0xaa, 0xbb, 0xcc],
        ),
        (
            Err(DecodeError::SizeTooLarge),
            vec![0xfe, 0x01, 0x00, 0x00, 0x02],
        ),
    ];

    for (i, (value, expected)) in tests.iter().enumerate() {
        if let Ok(value) = value {
            let mut serialized = Vec::new();
            value.encode(&mut serialized);
            assert!(serialized == *expected, "row: {i}");
        }

        let mut cursor = &expected[..];
        assert!(Script::decode(&mut cursor) == *value, "row: {i}");
        if value.is_ok() {
            assert!(cursor.is_empty(), "row: {i}");
        }
    }
}

fn arb_script() -> impl Strategy<Value = Script> {
    prop_oneof![
        prop::collection::vec(any::<u8>(), 0..=0xfc),
        prop::collection::vec(any::<u8>(), 0xfd..=u16::MAX as usize),
        prop::collection::vec(any::<u8>(), u16::MAX as usize + 1..=80_000),
    ]
    .prop_map(Script)
}

proptest! {
    #[test]
    fn script_roundtrip(script in arb_script()) {
        let mut serialized = Vec::new();
        script.encode(&mut serialized);

        let prefix_len = match script.0.len() {
            0..=0xfc => 1,
            0xfd..=0xffff => 3,
            _ => 5,
        };
        prop_assert_eq!(serialized.len(), prefix_len + script.0.len());

        let mut cursor = serialized.as_slice();
        prop_assert_eq!(Script::decode(&mut cursor), Ok(script));
        prop_assert!(cursor.is_empty());
    }
}
