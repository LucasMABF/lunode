use crate::{Decodable, DecodeError, Encodable, Reader, Writer};

/// Upper bound on decoded sizes (32 MiB, Core's `MAX_SIZE`).
const MAX_SIZE: u64 = 0x02000000;

const _: () = assert!(
    MAX_SIZE <= usize::MAX as u64,
    "consensus sizes must fit in usize"
);

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
