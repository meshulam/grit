use std::path::PathBuf;
use hex::ToHex;

// Type alias for 20-byte SHA-1 hash
pub struct ShaHash ([u8; 20]);

impl ShaHash {
    pub fn new() -> ShaHash { ShaHash::new_with_value([0u8; 20]) }

    fn new_with_value(bytes: [u8; 20]) -> ShaHash {
        ShaHash(bytes)
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub fn to_hex(&self) -> String { self.0.to_hex() }

    pub fn apply_path(&self, path: &mut PathBuf) {
        path.push(self.0[0..1].to_hex());
        path.push(self.0[1..].to_hex());
    }
}
