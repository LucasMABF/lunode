use alloc::vec::Vec;

/// Upper bound on decoded sizes (32 MiB, Core's `MAX_SIZE`).
const MAX_SIZE: u64 = 0x02000000;
/// Upper bound on a single decode allocation (Core's `MAX_VECTOR_ALLOCATE`).
const MAX_VECTOR_ALLOCATE: usize = 5000000;

const _: () = assert!(
    MAX_SIZE <= usize::MAX as u64,
    "consensus sizes must fit in usize"
);

macro_rules! impl_consensus_encoding {
    ($type:ident, $($field:ident),+ $(,)?) => {
        impl $crate::Encodable for $type {
            fn encode<W: $crate::Writer>(&self, writer: &mut W) {
                $(
                    self.$field.encode(writer);
                )+
            }
        }

        impl $crate::Decodable for $type {
            fn decode<R: $crate::Reader>(
                reader: &mut R,
            ) -> Result<Self, $crate::DecodeError> {
                Ok(Self {
                    $(
                        $field: $crate::Decodable::decode(reader)?,
                    )+
                })
            }
        }
    };
    ($type:ident $(,)?) => {
        impl $crate::Encodable for $type {
            fn encode<W: $crate::Writer>(&self, writer: &mut W) {
                self.0.encode(writer);
            }
        }

        impl $crate::Decodable for $type {
            fn decode<R: $crate::Reader>(
                reader: &mut R,
            ) -> Result<Self, $crate::DecodeError> {
                Ok(Self($crate::Decodable::decode(reader)?))
            }
        }
    };
}

macro_rules! impl_int_encoding {
    ($($type:ident),+ $(,)?) => {
        $(
            impl Encodable for $type {
                fn encode<W: Writer>(&self, writer: &mut W) {
                    writer.write(&self.to_le_bytes());
                }
            }

            impl Decodable for $type {
                fn decode<R: Reader>(reader: &mut R) -> Result<Self, DecodeError> {
                    let mut bytes = [0_u8; size_of::<$type>()];
                    reader.read(&mut bytes)?;
                    Ok(Self::from_le_bytes(bytes))
                }
            }
        )+
    };
}

pub(crate) use impl_consensus_encoding;

/// A sink for serialized bytes.
pub trait Writer {
    /// Writes `data` to the sink.
    fn write(&mut self, data: &[u8]);
}

impl Writer for Vec<u8> {
    fn write(&mut self, data: &[u8]) {
        self.extend_from_slice(data);
    }
}

/// A type with a consensus serialization.
///
/// ```
/// use lunode_primitives::Encodable;
///
/// let mut bytes = Vec::new();
/// 42u32.encode(&mut bytes);
/// assert_eq!(bytes, [42, 0, 0, 0]);
/// ```
pub trait Encodable {
    /// Encodes `self` into `writer`.
    fn encode<W: Writer>(&self, writer: &mut W);
}

impl<const N: usize> Encodable for [u8; N] {
    fn encode<W: Writer>(&self, writer: &mut W) {
        writer.write(self);
    }
}

impl<T: Encodable> Encodable for Vec<T> {
    fn encode<W: Writer>(&self, writer: &mut W) {
        CompactSize(self.len() as u64).encode(writer);
        for item in self {
            item.encode(writer);
        }
    }
}

pub(crate) fn encode_prefixed_bytes<W: Writer>(bytes: &[u8], writer: &mut W) {
    CompactSize(bytes.len() as u64).encode(writer);
    writer.write(bytes);
}

/// An error decoding a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The input ended before the value was complete.
    UnexpectedEnd,
    /// The value was not encoded in its minimal form.
    NonCanonical,
    /// The decoded size exceeds the 32 MiB limit.
    SizeTooLarge,
}

/// A source of serialized bytes.
pub trait Reader {
    /// Fills `buf` with the next `buf.len()` bytes from the source.
    ///
    /// # Errors
    ///
    /// Implementations signal exhaustion with [`DecodeError::UnexpectedEnd`].
    fn read(&mut self, buf: &mut [u8]) -> Result<(), DecodeError>;
}

impl Reader for &[u8] {
    fn read(&mut self, buf: &mut [u8]) -> Result<(), DecodeError> {
        if self.len() < buf.len() {
            return Err(DecodeError::UnexpectedEnd);
        }

        let (head, tail) = self.split_at(buf.len());
        buf.copy_from_slice(head);
        *self = tail;

        Ok(())
    }
}

/// A type with a consensus deserialization.
///
/// ```
/// use lunode_primitives::Decodable;
///
/// let mut bytes = &[42, 0, 0, 0][..];
/// let n = u32::decode(&mut bytes).unwrap();
/// assert_eq!(n, 42);
/// ```
pub trait Decodable: Sized {
    /// Decodes a value from `reader`.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::UnexpectedEnd`] if the input ends early. Types with
    /// a length prefix may also return [`DecodeError::NonCanonical`] or
    /// [`DecodeError::SizeTooLarge`].
    fn decode<R: Reader>(reader: &mut R) -> Result<Self, DecodeError>;
}

impl_int_encoding!(u8, u16, u32, u64, i32);

impl<const N: usize> Decodable for [u8; N] {
    fn decode<R: Reader>(reader: &mut R) -> Result<Self, DecodeError> {
        let mut bytes = [0; N];
        reader.read(&mut bytes)?;
        Ok(bytes)
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

/// A variable-length integer encoding, used for lengths in the wire format.
///
/// Values encode to 1, 3, 5, or 9 bytes depending on magnitude. Every value
/// has exactly one valid encoding: decoding rejects non-minimal forms and
/// sizes above 32 MiB.
///
/// ```
/// use lunode_primitives::{CompactSize, Encodable};
///
/// let mut bytes = Vec::new();
/// CompactSize(253).encode(&mut bytes);
/// assert_eq!(bytes, [0xfd, 0xfd, 0x00]);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompactSize(pub u64);

impl Encodable for CompactSize {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "each branch tests the value against the target width first"
    )]
    #[expect(
        clippy::checked_conversions,
        reason = "bounds mirror the decoder's canonicality floors"
    )]
    fn encode<W: Writer>(&self, writer: &mut W) {
        if self.0 <= 0xfc {
            (self.0 as u8).encode(writer);
        } else if self.0 <= u64::from(u16::MAX) {
            0xfd_u8.encode(writer);
            (self.0 as u16).encode(writer);
        } else if self.0 <= u64::from(u32::MAX) {
            0xfe_u8.encode(writer);
            (self.0 as u32).encode(writer);
        } else {
            0xff_u8.encode(writer);
            self.0.encode(writer);
        }
    }
}

impl Decodable for CompactSize {
    #[expect(
        clippy::checked_conversions,
        reason = "bounds mirror the encoder's canonicality floors"
    )]
    fn decode<R: Reader>(reader: &mut R) -> Result<Self, DecodeError> {
        let marker = u8::decode(reader)?;

        let n = match marker {
            n @ 0..=0xfc => u64::from(n),
            0xfd => {
                let n = u64::from(u16::decode(reader)?);
                if n <= 0xfc {
                    return Err(DecodeError::NonCanonical);
                }
                n
            }
            0xfe => {
                let n = u64::from(u32::decode(reader)?);
                if n <= u64::from(u16::MAX) {
                    return Err(DecodeError::NonCanonical);
                }
                n
            }
            0xff => {
                let n = u64::decode(reader)?;
                if n <= u64::from(u32::MAX) {
                    return Err(DecodeError::NonCanonical);
                }
                n
            }
        };

        if n > MAX_SIZE {
            return Err(DecodeError::SizeTooLarge);
        }

        Ok(CompactSize(n))
    }
}
