const INITIAL_STATE: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Computes the SHA-256 digest of `data`.
///
/// ```
/// let digest = lunode_crypto::sha256(b"hello world");
/// ```
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut sha256 = Sha256::new();
    sha256.update(data);
    sha256.finalize()
}

/// A SHA-256 hasher.
///
/// ```
/// use lunode_crypto::Sha256;
///
/// let mut hasher = Sha256::new();
/// hasher.update(b"hello");
/// hasher.update(b" world");
/// let digest = hasher.finalize();
/// ```
#[derive(Clone, Debug)]
pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffered: usize,
    length: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// Creates a new hasher.
    pub fn new() -> Self {
        Self {
            state: INITIAL_STATE,
            buffer: [0; 64],
            buffered: 0,
            length: 0,
        }
    }

    /// Feeds `data` into the hasher.
    pub fn update(&mut self, mut data: &[u8]) {
        self.length += data.len() as u64;

        if self.buffered > 0 {
            let take = (64 - self.buffered).min(data.len());
            self.buffer[self.buffered..self.buffered + take].copy_from_slice(&data[..take]);
            self.buffered += take;
            data = &data[take..];

            if self.buffered < 64 {
                return;
            }

            self.state = compress(self.state, &self.buffer);
            self.buffered = 0;
        }

        let (blocks, remainder) = data.as_chunks::<64>();
        for block in blocks {
            self.state = compress(self.state, block);
        }

        self.buffer[..remainder.len()].copy_from_slice(remainder);
        self.buffered = remainder.len();
    }

    /// Consumes the hasher and returns the digest.
    pub fn finalize(mut self) -> [u8; 32] {
        let size = self.length * 8;
        self.buffer[self.buffered] = 0x80;
        self.buffered += 1;

        if self.buffered > 56 {
            self.buffer[self.buffered..].fill(0);
            self.state = compress(self.state, &self.buffer);
            self.buffer[0..56].fill(0);
        } else {
            self.buffer[self.buffered..56].fill(0);
        }

        self.buffer[56..].copy_from_slice(&size.to_be_bytes());

        self.state = compress(self.state, &self.buffer);

        let mut result = [0_u8; 32];
        let (chunks, _) = result.as_chunks_mut::<4>();
        for (chunk, word) in chunks.iter_mut().zip(self.state) {
            *chunk = word.to_be_bytes();
        }

        result
    }
}

fn compress(mut state: [u32; 8], block: &[u8; 64]) -> [u32; 8] {
    let mut w = [0; 64];

    let (chunks, _) = block.as_chunks::<4>();
    for (word, chunk) in w.iter_mut().zip(chunks) {
        *word = u32::from_be_bytes(*chunk);
    }

    for i in 16..64 {
        w[i] = small_sigma1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(small_sigma0(w[i - 15]))
            .wrapping_add(w[i - 16]);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;

    for (m, k) in w.iter().zip(&K) {
        let t1 = big_sigma1(e)
            .wrapping_add(choice(e, f, g))
            .wrapping_add(h)
            .wrapping_add(*k)
            .wrapping_add(*m);

        let t2 = big_sigma0(a).wrapping_add(majority(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);

    state
}

fn small_sigma0(bits: u32) -> u32 {
    bits.rotate_right(7) ^ bits.rotate_right(18) ^ (bits >> 3)
}

fn small_sigma1(bits: u32) -> u32 {
    bits.rotate_right(17) ^ bits.rotate_right(19) ^ (bits >> 10)
}

fn big_sigma0(bits: u32) -> u32 {
    bits.rotate_right(2) ^ bits.rotate_right(13) ^ bits.rotate_right(22)
}

fn big_sigma1(bits: u32) -> u32 {
    bits.rotate_right(6) ^ bits.rotate_right(11) ^ bits.rotate_right(25)
}

fn choice(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (!a & c)
}

fn majority(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}
