use lunode_crypto::Ripemd160;

mod common;
use common::{Hasher, Rng, long_test_string, test_vector};

impl Hasher for Ripemd160 {
    fn update(&mut self, data: &[u8]) {
        Ripemd160::update(self, data);
    }

    fn finalize(self) -> Vec<u8> {
        Ripemd160::finalize(self).to_vec()
    }
}

#[test]
fn core_vectors() {
    let mut rng = Rng::new(0x2545f4914f6cdd1d);
    let hasher = Ripemd160::new();

    let literals: &[(&[u8], &str)] = &[
        (b"", "9c1185a5c5e9fc54612808977ee8f548b2258d31"),
        (b"abc", "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc"),
        (
            b"message digest",
            "5d0689ef49d2fae572b881b123a85ffa21595f36",
        ),
        (
            b"secure hash algorithm",
            "20397528223b6a5f4cbc2808aba0464e645544f9",
        ),
        (
            b"RIPEMD160 is considered to be safe",
            "a7d78608c7af8a8e728778e81576870734122b66",
        ),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "12a053384a9c0c88e405a06c27dcf49ada62eb2b",
        ),
        (
            b"For this sample, this 63-byte string will be used as input data",
            "de90dbfee14b63fb5abf27c2ad4a82aaa5f27a11",
        ),
        (
            b"This is exactly 64 bytes long, not counting the terminating byte",
            "eda31d51d3a623b81e19eb02e24ff65d27d67b37",
        ),
    ];

    for (input, expected) in literals {
        test_vector(&hasher, input, expected, &mut rng);
    }

    let million_a = std::vec![b'a'; 1000000];
    test_vector(
        &hasher,
        &million_a,
        "52783243c1697bdbe16d37f97f68f08325dc1528",
        &mut rng,
    );

    let long = long_test_string();
    test_vector(
        &hasher,
        &long,
        "464243587bd146ea835cdf57bdae582f25ec45f1",
        &mut rng,
    );
}
