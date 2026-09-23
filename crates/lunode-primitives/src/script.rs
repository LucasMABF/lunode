use alloc::vec::Vec;

use crate::{
    Decodable, DecodeError, Encodable, Reader, Writer,
    vector::{decode_prefixed_bytes, encode_prefixed_bytes},
};

/// A script.
///
/// The bytes are not checked to form a valid opcode sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script(pub Vec<u8>);

impl Encodable for Script {
    fn encode<W: Writer>(&self, writer: &mut W) {
        encode_prefixed_bytes(&self.0, writer);
    }
}

impl Decodable for Script {
    fn decode<R: Reader>(reader: &mut R) -> Result<Self, DecodeError> {
        let bytes = decode_prefixed_bytes(reader)?;
        Ok(Script(bytes))
    }
}
