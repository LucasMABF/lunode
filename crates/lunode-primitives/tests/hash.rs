use lunode_primitives::{hash160, hash256};

mod common;
use common::hex;

#[test]
fn vectors() {
    let expected_hex = "4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358";
    assert_eq!(
        hash256(b"abc"),
        hex(expected_hex).as_slice(),
        "one-shot failed for {expected_hex}"
    );

    let expected_hex = "bb1be98c142444d7a56aa3981c3942a978e4dc33";
    assert_eq!(
        hash160(b"abc"),
        hex(expected_hex).as_slice(),
        "one-shot failed for {expected_hex}"
    );
}
