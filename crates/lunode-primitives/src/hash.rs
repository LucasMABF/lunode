use lunode_crypto::{Sha256, ripemd160, sha256};

/// Computes the HASH256 (double SHA-256) digest of `data`.
///
/// ```
/// let digest = lunode_primitives::hash256(b"hello world");
/// ```
pub fn hash256(data: &[u8]) -> [u8; 32] {
    let mut hash256 = Hash256::new();
    hash256.update(data);
    hash256.finalize()
}

/// Computes the HASH160 (SHA-256 then RIPEMD-160) digest of `data`.
///
/// ```
/// let digest = lunode_primitives::hash160(b"hello world");
/// ```
pub fn hash160(data: &[u8]) -> [u8; 20] {
    let mut hash160 = Hash160::new();
    hash160.update(data);
    hash160.finalize()
}

/// A HASH256 (double SHA-256) hasher.
///
/// ```
/// use lunode_primitives::Hash256;
///
/// let mut hasher = Hash256::new();
/// hasher.update(b"hello");
/// hasher.update(b" world");
/// let digest = hasher.finalize();
/// ```
#[derive(Clone, Debug)]
pub struct Hash256(Sha256);

impl Default for Hash256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash256 {
    /// Creates a new hasher.
    pub fn new() -> Self {
        Self(Sha256::new())
    }

    /// Feeds `data` into the hasher.
    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    /// Consumes the hasher and returns the digest.
    pub fn finalize(self) -> [u8; 32] {
        sha256(&self.0.finalize())
    }
}

/// A HASH160 (SHA-256 then RIPEMD-160) hasher.
///
/// ```
/// use lunode_primitives::Hash160;
///
/// let mut hasher = Hash160::new();
/// hasher.update(b"hello");
/// hasher.update(b" world");
/// let digest = hasher.finalize();
/// ```
#[derive(Clone, Debug)]
pub struct Hash160(Sha256);

impl Default for Hash160 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash160 {
    /// Creates a new hasher.
    pub fn new() -> Self {
        Self(Sha256::new())
    }

    /// Feeds `data` into the hasher.
    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    /// Consumes the hasher and returns the digest.
    pub fn finalize(self) -> [u8; 20] {
        ripemd160(&self.0.finalize())
    }
}

#[cfg(test)]
mod tests {
    use crate::{hash160, hash256};

    extern crate std;

    fn hex(s: &str) -> std::vec::Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

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
}
