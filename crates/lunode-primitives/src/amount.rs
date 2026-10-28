use crate::serialize::impl_consensus_encoding;

/// An amount of bitcoin, in satoshis.
///
/// The value is not range-checked: it may exceed [`Amount::MAX_MONEY`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(pub u64);

impl_consensus_encoding!(Amount);

impl Amount {
    /// One bitcoin
    pub const ONE_BTC: Amount = Amount(100_000_000);
    /// The maximum amount allowed by consensus
    pub const MAX_MONEY: Amount = Amount(21_000_000 * Self::ONE_BTC.0);

    /// Adds two amounts, returning `None` on overflow
    pub fn checked_add(self, rhs: Amount) -> Option<Amount> {
        self.0.checked_add(rhs.0).map(Amount)
    }
}
