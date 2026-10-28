use lunode_primitives::Amount;

#[test]
fn constants_and_checked_add() {
    assert_eq!(Amount::ONE_BTC, Amount(100_000_000));
    assert_eq!(Amount::MAX_MONEY, Amount(2_100_000_000_000_000));

    assert_eq!(Amount(1).checked_add(Amount(2)), Some(Amount(3)));
    assert_eq!(Amount(u64::MAX).checked_add(Amount(1)), None);
}
