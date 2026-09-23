use crate::{CompactSize, Decodable, DecodeError, Encodable, Reader, Writer};
use alloc::vec::Vec;

/// Upper bound on a single decode allocation (Core's `MAX_VECTOR_ALLOCATE`).
const MAX_VECTOR_ALLOCATE: usize = 5000000;

impl<T: Encodable> Encodable for Vec<T> {
    fn encode<W: Writer>(&self, writer: &mut W) {
        CompactSize(self.len() as u64).encode(writer);
        for item in self {
            item.encode(writer);
        }
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "capped by CompactSize::decode at MAX_SIZE, asserted to fit usize"
    )]
    fn decode<R: Reader>(reader: &mut R) -> Result<Self, DecodeError> {
        const {
            assert!(size_of::<T>() > 0 && size_of::<T>() <= MAX_VECTOR_ALLOCATE);
        }
        let size = CompactSize::decode(reader)?.0 as usize;
        let mut items = Vec::new();

        let mut i = 0;
        while i < size {
            let chunk = (size - i).min(MAX_VECTOR_ALLOCATE / size_of::<T>());

            items.reserve(chunk);

            for _ in 0..chunk {
                items.push(T::decode(reader)?);
            }
            i += chunk;
        }

        Ok(items)
    }
}

pub(crate) fn encode_prefixed_bytes<W: Writer>(bytes: &[u8], writer: &mut W) {
    CompactSize(bytes.len() as u64).encode(writer);
    writer.write(bytes);
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "capped by CompactSize::decode at MAX_SIZE, asserted to fit usize"
)]
pub(crate) fn decode_prefixed_bytes<R: Reader>(reader: &mut R) -> Result<Vec<u8>, DecodeError> {
    let size = CompactSize::decode(reader)?.0 as usize;
    let mut bytes = Vec::new();

    let mut i = 0;
    while i < size {
        let chunk = (size - i).min(MAX_VECTOR_ALLOCATE);

        bytes.resize(i + chunk, 0);

        reader.read(&mut bytes[i..i + chunk])?;
        i += chunk;
    }

    Ok(bytes)
}
