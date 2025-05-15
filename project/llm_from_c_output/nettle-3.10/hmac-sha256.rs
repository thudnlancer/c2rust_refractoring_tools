// hmac-sha256.rs
//
// HMAC-SHA256 message authentication code.
//
// Translated from C to Rust, maintaining the same functionality
// while adhering to Rust's safety practices.

use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

/// HMAC-SHA256 context
pub struct HmacSha256 {
    inner: Hmac<Sha256>,
}

impl HmacSha256 {
    /// Create a new HMAC-SHA256 context with the given key
    pub fn new(key: &[u8]) -> Self {
        Self {
            inner: Hmac::<Sha256>::new_from_slice(key)
                .expect("HMAC can take keys of any size"),
        }
    }

    /// Update the HMAC computation with additional input data
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalize the HMAC computation and write the output
    pub fn digest(self, output: &mut [u8]) -> Result<(), String> {
        if output.len() < 32 {
            return Err("Output buffer too small".to_string());
        }
        let result = self.inner.finalize();
        output.copy_from_slice(&result.into_bytes());
        Ok(())
    }
}