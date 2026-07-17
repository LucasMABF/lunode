use lunode_crypto::Sha256;

mod common;
use common::{Hasher, Rng, long_test_string, test_vector};

impl Hasher for Sha256 {
    fn update(&mut self, data: &[u8]) {
        Sha256::update(self, data);
    }

    fn finalize(self) -> Vec<u8> {
        Sha256::finalize(self).to_vec()
    }
}

#[test]
fn core_vectors() {
    let mut rng = Rng::new(0x2545f4914f6cdd1d);
    let hasher = Sha256::new();

    let literals: &[(&[u8], &str)] = &[
        (
            b"",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            b"message digest",
            "f7846f55cf23e14eebeab5b4e1550cad5b509e3348fbc4efa3a1413d393cb650",
        ),
        (
            b"secure hash algorithm",
            "f30ceb2bb2829e79e4ca9753d35a8ecc00262d164cc077080295381cbd643f0d",
        ),
        (
            b"SHA256 is considered to be safe",
            "6819d915c73f4d1e77e4e1b52d1fa0f9cf9beaead3939f15874bd988e2a23630",
        ),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
        (
            b"For this sample, this 63-byte string will be used as input data",
            "f08a78cbbaee082b052ae0708f32fa1e50c5c421aa772ba5dbb406a2ea6be342",
        ),
        (
            b"This is exactly 64 bytes long, not counting the terminating byte",
            "ab64eff7e88e2e46165e29f2bce41826bd4c7b3552f6b382a9e7d3af47c245f8",
        ),
        (
            b"As Bitcoin relies on 80 byte header hashes, we want to have an example for that.",
            "7406e8de7d6e4fffc573daef05aefb8806e7790f55eab5576f31349743cca743",
        ),
    ];

    for (input, expected) in literals {
        test_vector(&hasher, input, expected, &mut rng);
    }

    let million_a = std::vec![b'a'; 1000000];
    test_vector(
        &hasher,
        &million_a,
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0",
        &mut rng,
    );

    let long = long_test_string();
    test_vector(
        &hasher,
        &long,
        "a316d55510b49662420f49d145d42fb83f31ef8dc016aa4e32df049991a91e26",
        &mut rng,
    );
}
