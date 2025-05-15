//! HKDF implementation (RFC 5869)
//!
//! This module provides functions for HKDF (HMAC-based Extract-and-Expand Key Derivation Function)
//! operations.

use std::error::Error;
use std::fmt;

/// Error type for HKDF operations
#[derive(Debug)]
pub enum HkdfError {
    /// Invalid output length requested
    InvalidLength,
}

impl fmt::Display for HkdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HkdfError::InvalidLength => write!(f, "Invalid output length requested"),
        }
    }
}

impl Error for HkdfError {}

/// Trait for hash functions used in HKDF
pub trait HkdfHash {
    /// Update the hash context with data
    fn update(&mut self, data: &[u8]);
    /// Finalize the hash and write the output
    fn digest(&mut self, output: &mut [u8]);
    /// Get the output size of the hash
    fn digest_size(&self) -> usize;
}

/// HKDF extract operation
pub fn hkdf_extract<H: HkdfHash>(
    mac_ctx: &mut H,
    secret: &[u8],
    dst: &mut [u8],
) -> Result<(), HkdfError> {
    if dst.len() != mac_ctx.digest_size() {
        return Err(HkdfError::InvalidLength);
    }

    mac_ctx.update(secret);
    mac_ctx.digest(dst);
    Ok(())
}

/// HKDF expand operation
pub fn hkdf_expand<H: HkdfHash>(
    mac_ctx: &mut H,
    info: &[u8],
    output: &mut [u8],
) -> Result<(), HkdfError> {
    let digest_size = mac_ctx.digest_size();
    if output.is_empty() {
        return Ok(());
    }

    let mut temp = vec![0u8; digest_size];
    let mut i = 1u8;

    for chunk in output.chunks_mut(digest_size) {
        mac_ctx.update(info);
        mac_ctx.update(&[i]);

        if chunk.len() <= digest_size {
            mac_ctx.digest(chunk);
            break;
        }

        mac_ctx.digest(&mut temp);
        mac_ctx.update(&temp);
        i += 1;
    }

    Ok(())
}