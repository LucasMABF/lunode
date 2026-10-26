mod common;
use common::hex;
use lunode_consensus::compute_merkle_root;

type Test = (Vec<[u8; 32]>, Option<[u8; 32]>);

fn reversed_hex(s: &str) -> [u8; 32] {
    let mut bytes = hex(s);
    bytes.reverse();
    bytes.try_into().unwrap()
}

#[test]
fn vectors() {
    let tests: &[Test] = &[
        (vec![], None),
        (vec![[7; 32]], Some([7; 32])),
        (
            vec![
                reversed_hex("8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87"),
                reversed_hex("fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4"),
                reversed_hex("6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4"),
                reversed_hex("e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d"),
            ],
            Some(reversed_hex(
                "f3e94742aca4b5ef85488dc37c06c3282295ffec960994b2c0d5ac2a25a95766",
            )),
        ),
        (
            vec![
                reversed_hex("a335b243f5e343049fccac2cf4d70578ad705831940d3eef48360b0ea3829ed4"),
                reversed_hex("d5fd11cb1fabd91c75733f4cf8ff2f91e4c0d7afa4fd132f792eacb3ef56a46c"),
                reversed_hex("0441cb66ef0cbf78c9ecb3d5a7d0acf878bfdefae8a77541b3519a54df51e7fd"),
                reversed_hex("1a8a27d690889b28d6cb4dacec41e354c62f40d85a7f4b2d7a54ffc736c6ff35"),
                reversed_hex("1d543d550676f82bf8bf5b0cc410b16fc6fc353b2a4fd9a0d6a2312ed7338701"),
            ],
            Some(reversed_hex(
                "5766798857e436d6243b46b5c1e0af5b6806aa9c2320b3ffd4ecff7b31fd4647",
            )),
        ),
    ];

    for (i, (value, expected)) in tests.iter().enumerate() {
        assert_eq!(compute_merkle_root(value.clone()), *expected, "row: {i}");
    }
}
