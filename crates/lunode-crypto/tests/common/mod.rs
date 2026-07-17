pub fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        debug_assert!(seed != 0);
        Self(seed)
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.0;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.0 = x;
        x
    }

    pub fn randrange(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

pub trait Hasher: Clone {
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> Vec<u8>;
}

pub fn test_vector<H: Hasher>(hasher: &H, input: &[u8], expected_hex: &str, rng: &mut Rng) {
    let expected = hex(expected_hex);

    let mut h = hasher.clone();
    h.update(input);
    assert_eq!(h.finalize(), expected, "one-shot failed for {expected_hex}");

    for i in 0..32 {
        let mut pos = 0;
        let mut h = hasher.clone();
        while pos < input.len() {
            let len = rng.randrange((input.len() - pos).div_ceil(2) + 1);
            h.update(&input[pos..pos + len]);
            pos += len;

            if pos > 0 && pos + (2 * expected.len()) > input.len() && pos < input.len() {
                let mut copy = h.clone();
                copy.update(&input[pos..]);
                assert_eq!(
                    copy.finalize(),
                    expected,
                    "clone failed for {expected_hex} (iter {i})"
                );
            }
        }
        assert_eq!(
            h.finalize(),
            expected,
            "chunked failed for {expected_hex} (iter {i})"
        );
    }
}

pub fn long_test_string() -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..200000 {
        result.push(i as u8);
        result.push((i >> 4) as u8);
        result.push((i >> 8) as u8);
        result.push((i >> 12) as u8);
        result.push((i >> 16) as u8);
    }

    result
}
