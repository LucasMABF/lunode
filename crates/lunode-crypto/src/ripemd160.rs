const INITIAL_STATE: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

const LEFT: usize = 0;
const RIGHT: usize = 1;

const KL: [u32; 5] = [0x00000000, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e];
const KR: [u32; 5] = [0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x7a6d76e9, 0x00000000];
const K: [[u32; 5]; 2] = [KL, KR];

const RL: [u8; 80] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5,
    2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4,
    13, 3, 7, 15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
];
const RR: [u8; 80] = [
    5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12,
    4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5,
    12, 2, 13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
];
const R: [[u8; 80]; 2] = [RL, RR];

const SL: [u8; 80] = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15,
    9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14,
    15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
];
const SR: [u8; 80] = [
    8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12,
    7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14,
    6, 14, 6, 9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
];
const S: [[u8; 80]; 2] = [SL, SR];

type RoundFn = fn(u32, u32, u32) -> u32;
const FL: [RoundFn; 5] = [f, g, h, i, j];
const FR: [RoundFn; 5] = [j, i, h, g, f];
const F: [[RoundFn; 5]; 2] = [FL, FR];

/// Computes the RIPEMD-160 digest of `data`.
///
/// ```
/// let digest = lunode_crypto::ripemd160(b"hello world");
/// ```
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(data);
    ripemd160.finalize()
}

/// A RIPEMD-160 hasher.
///
/// ```
/// use lunode_crypto::Ripemd160;
///
/// let mut hasher = Ripemd160::new();
/// hasher.update(b"hello");
/// hasher.update(b" world");
/// let digest = hasher.finalize();
/// ```
#[derive(Clone, Debug)]
pub struct Ripemd160 {
    state: [u32; 5],
    buffer: [u8; 64],
    buffered: usize,
    length: u64,
}

impl Default for Ripemd160 {
    fn default() -> Self {
        Self::new()
    }
}

impl Ripemd160 {
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
    pub fn finalize(mut self) -> [u8; 20] {
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

        self.buffer[56..].copy_from_slice(&size.to_le_bytes());

        self.state = compress(self.state, &self.buffer);

        let mut result = [0_u8; 20];
        let (chunks, _) = result.as_chunks_mut::<4>();
        for (chunk, word) in chunks.iter_mut().zip(self.state) {
            *chunk = word.to_le_bytes();
        }

        result
    }
}

#[expect(
    clippy::many_single_char_names,
    reason = "working variables follow the RIPEMD-160 paper's pseudocode"
)]
fn compress(mut state: [u32; 5], block: &[u8; 64]) -> [u32; 5] {
    let mut x = [0; 16];

    let (chunks, _) = block.as_chunks::<4>();
    for (word, chunk) in x.iter_mut().zip(chunks) {
        *word = u32::from_le_bytes(*chunk);
    }

    let mut a = [state[0]; 2];
    let mut b = [state[1]; 2];
    let mut c = [state[2]; 2];
    let mut d = [state[3]; 2];
    let mut e = [state[4]; 2];

    for step in 0..80_usize {
        let round = step / 16;

        for line in 0..2 {
            let t = a[line]
                .wrapping_add(F[line][round](b[line], c[line], d[line]))
                .wrapping_add(x[usize::from(R[line][step])])
                .wrapping_add(K[line][round])
                .rotate_left(u32::from(S[line][step]))
                .wrapping_add(e[line]);

            a[line] = e[line];
            e[line] = d[line];
            d[line] = c[line].rotate_left(10);
            c[line] = b[line];
            b[line] = t;
        }
    }

    let t = state[1].wrapping_add(c[LEFT]).wrapping_add(d[RIGHT]);
    state[1] = state[2].wrapping_add(d[LEFT]).wrapping_add(e[RIGHT]);
    state[2] = state[3].wrapping_add(e[LEFT]).wrapping_add(a[RIGHT]);
    state[3] = state[4].wrapping_add(a[LEFT]).wrapping_add(b[RIGHT]);
    state[4] = state[0].wrapping_add(b[LEFT]).wrapping_add(c[RIGHT]);
    state[0] = t;

    state
}

fn f(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    (x | !y) ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn j(x: u32, y: u32, z: u32) -> u32 {
    x ^ (y | !z)
}
