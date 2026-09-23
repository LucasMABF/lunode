use alloc::vec::Vec;

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
