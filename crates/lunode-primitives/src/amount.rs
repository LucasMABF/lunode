use crate::serialize::impl_consensus_encoding;

/// An amount of bitcoin, in satoshis.
///
/// The value is not range-checked: it may exceed the total supply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Amount(pub u64);

impl_consensus_encoding!(Amount);
