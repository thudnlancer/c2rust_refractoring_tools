// hmac-ripemd160.rs
//
// HMAC-RIPEMD160 message authentication code.
//
// Original C code Copyright (C) 2011 Niels Möller
// Rust translation maintains same functionality while following Rust safety practices.

use ripemd160::{Ripemd160, Digest};
use hmac::{Hmac, Mac};

/// HMAC-RIPEMD160 context
pub struct HmacRipemd160 {
    inner: Hmac<Ripemd160>,
}

impl HmacRipemd160 {
    /// Create new HMAC-RIPEMD160 context with the given key
    pub fn new_from_slice(key: &[u8]) -> Result<Self, hmac::crypto_mac::InvalidKeyLength> {
        Ok(Self {
            inner: Hmac::<Ripemd160>::new_from_slice(key)?,
        })
    }

    /// Update the HMAC with input data
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalize the HMAC computation and write the result into `output`
    pub fn finalize(self, output: &mut [u8]) -> Result<(), hmac::crypto_mac::InvalidLength> {
        self.inner.finalize().into_bytes().copy_to_slice(output);
        Ok(())
    }

    /// One-shot convenience function that computes HMAC of input data with given key
    pub fn hmac_ripemd160(key: &[u8], data: &[u8], output: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut mac = Self::new_from_slice(key)?;
        mac.update(data);
        mac.finalize(output)?;
        Ok(())
    }
}