use lunode_primitives::{CompactSize, Decodable, DecodeError, Encodable};
use proptest::prelude::*;

const MAX_SIZE: u64 = 0x02000000;

#[test]
fn boundary_sweep() {
    let mut buf = Vec::new();
    let mut i = 1;
    while i <= MAX_SIZE {
        CompactSize(i - 1).encode(&mut buf);
        CompactSize(i).encode(&mut buf);
        i *= 2;
    }

    let mut cursor = &buf[..];
    let mut i = 1;
    while i <= MAX_SIZE {
        assert_eq!(CompactSize::decode(&mut cursor), Ok(CompactSize(i - 1)));
        assert_eq!(CompactSize::decode(&mut cursor), Ok(CompactSize(i)));
        i *= 2;
    }

    assert!(cursor.is_empty());
}

#[test]
fn exact_bytes() {
    let tests: &[(u64, &[u8])] = &[
        (0, &[0x00]),
        (0xfc, &[0xfc]),
        (0xfd, &[0xfd, 0xfd, 0x00]),
        (0xffff, &[0xfd, 0xff, 0xff]),
        (0x10000, &[0xfe, 0x00, 0x00, 0x01, 0x00]),
        (
            0x100000000,
            &[0xff, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
        ),
    ];

    for &(value, expected) in tests {
        let compact_size = CompactSize(value);
        let mut bytes = Vec::new();
        compact_size.encode(&mut bytes);
        assert_eq!(&bytes, expected);

        let mut slice = bytes.as_slice();
        let result = CompactSize::decode(&mut slice);
        if value <= MAX_SIZE {
            assert_eq!(result, Ok(CompactSize(value)));
            assert!(slice.is_empty());
        } else {
            assert_eq!(result, Err(DecodeError::SizeTooLarge));
        }
    }
}

#[test]
fn decode_errors() {
    let tests: &[(&[u8], DecodeError)] = &[
        (&[0xfd, 0x00, 0x00], DecodeError::NonCanonical),
        (&[0xfd, 0xfc, 0x00], DecodeError::NonCanonical),
        (&[0xfe, 0x00, 0x00, 0x00, 0x00], DecodeError::NonCanonical),
        (&[0xfe, 0xff, 0xff, 0x00, 0x00], DecodeError::NonCanonical),
        (
            &[0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            DecodeError::NonCanonical,
        ),
        (
            &[0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00, 0x00],
            DecodeError::NonCanonical,
        ),
        (&[0xfe, 0x01, 0x00, 0x00, 0x02], DecodeError::SizeTooLarge),
        (&[0xfd], DecodeError::UnexpectedEnd),
    ];

    for &(bytes, expected) in tests {
        let result = CompactSize::decode(&mut &bytes[..]);
        assert_eq!(result, Err(expected), "input: {bytes:02x?}");
    }
}

fn arb_compact_size() -> impl Strategy<Value = CompactSize> {
    prop_oneof![
        0..=0xfc_u64,
        0xfd..=u64::from(u16::MAX),
        (u64::from(u16::MAX) + 1)..=u64::from(u32::MAX),
        (u64::from(u32::MAX) + 1)..=u64::MAX,
    ]
    .prop_map(CompactSize)
}

fn arb_non_canonical() -> impl Strategy<Value = Vec<u8>> {
    fn widened(marker: u8, width: usize, n: u64) -> Vec<u8> {
        let mut bytes = vec![marker];
        bytes.extend_from_slice(&n.to_le_bytes()[..width]);
        bytes
    }

    prop_oneof![
        (0..=0xfc_u64).prop_map(|n| widened(0xfd, 2, n)),
        (0..=u64::from(u16::MAX)).prop_map(|n| widened(0xfe, 4, n)),
        (0..=u64::from(u32::MAX)).prop_map(|n| widened(0xff, 8, n)),
    ]
}

proptest! {
    #[test]
    fn compact_size_roundtrip(compact_size in arb_compact_size()) {
        let mut bytes = Vec::new();
        compact_size.encode(&mut bytes);

        let expected_len = match compact_size.0 {
            0..=0xfc => 1,
            0xfd..=0xffff => 3,
            0x10000..=0xffffffff => 5,
            _ => 9,
        };
        prop_assert_eq!(bytes.len(), expected_len);

        let mut slice = bytes.as_slice();
        let result = CompactSize::decode(&mut slice);
        if compact_size.0 <= MAX_SIZE {
            prop_assert_eq!(result, Ok(compact_size));
            prop_assert!(slice.is_empty());
        } else {
            prop_assert_eq!(result, Err(DecodeError::SizeTooLarge));
        }
    }

    #[test]
    fn non_canonical(bytes in arb_non_canonical()) {
        let mut slice = bytes.as_slice();
        prop_assert_eq!(CompactSize::decode(&mut slice), Err(DecodeError::NonCanonical));
    }
}
