use rustc_serialize::hex::ToHex;
use std::path::PathBuf;

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

    pub fn path(&self, base: &str) -> PathBuf {
        let mut path = PathBuf::from(base);
        path.push(self.0[0..1].to_hex());
        path.push(self.0[1..].to_hex());
        path
    }
}
