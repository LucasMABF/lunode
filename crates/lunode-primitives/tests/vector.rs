use lunode_primitives::{Decodable, DecodeError, Encodable, Script};
use proptest::prelude::*;

type Test<T> = (Result<Vec<T>, DecodeError>, Vec<u8>);

#[test]
fn u32_exact_bytes() {
    let tests: &[Test<u32>] = &[
        (Ok(vec![]), vec![0]),
        (Ok(vec![1]), vec![1, 1, 0, 0, 0]),
        (
            Ok(vec![1, 2, 3]),
            vec![3, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0],
        ),
        (
            Ok(vec![0; 253]),
            [vec![0xfd, 0xfd, 0x00], vec![0; 1012]].concat(),
        ),
        (Err(DecodeError::UnexpectedEnd), vec![2, 0, 0, 0, 1]),
        (
            Err(DecodeError::NonCanonical),
            vec![0xfd, 0x01, 0x00, 0x00, 0x02],
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
        assert!(Vec::<u32>::decode(&mut cursor) == *value, "row: {i}");
        if value.is_ok() {
            assert!(cursor.is_empty(), "row: {i}");
        }
    }
}

#[test]
fn script_exact_bytes() {
    let tests: &[Test<Script>] = &[
        (Ok(vec![]), vec![0]),
        (Ok(vec![Script(vec![])]), vec![1, 0]),
        (Ok(vec![Script(vec![1, 2])]), vec![1, 2, 1, 2]),
        (
            Ok(vec![Script(vec![1, 2]), Script(vec![3])]),
            vec![2, 2, 1, 2, 1, 3],
        ),
        (
            Ok(vec![Script(vec![1, 2]), Script(vec![])]),
            vec![2, 2, 1, 2, 0],
        ),
        (
            Ok(vec![Script(vec![]); 500_000]),
            [vec![0xfe, 0x20, 0xa1, 0x07, 0x00], vec![0; 500_000]].concat(),
        ),
        (Err(DecodeError::UnexpectedEnd), vec![1, 5, 1, 2]),
    ];

    for (i, (value, expected)) in tests.iter().enumerate() {
        if let Ok(value) = value {
            let mut serialized = Vec::new();
            value.encode(&mut serialized);

            assert!(serialized == *expected, "row: {i}");
        }

        let mut cursor = &expected[..];
        assert!(Vec::<Script>::decode(&mut cursor) == *value, "row: {i}");
        if value.is_ok() {
            assert!(cursor.is_empty(), "row: {i}");
        }
    }
}

fn arb_u32_vector() -> impl Strategy<Value = Vec<u32>> {
    prop_oneof![
        prop::collection::vec(any::<u32>(), 0..=0xfc),
        prop::collection::vec(any::<u32>(), 0xfd..=300),
    ]
}

proptest! {
    #[test]
    fn vector_roundtrip(items in arb_u32_vector()) {
        let mut serialized = Vec::new();
        items.encode(&mut serialized);

        let prefix_len = match items.len() {
            0..=0xfc => 1,
            0xfd..=0xffff => 3,
            _ => 5,
        };
        prop_assert_eq!(serialized.len(), prefix_len + (items.len() * 4));

        let mut cursor = serialized.as_slice();
        prop_assert_eq!(Vec::<u32>::decode(&mut cursor), Ok(items));
        prop_assert!(cursor.is_empty());
    }
}
