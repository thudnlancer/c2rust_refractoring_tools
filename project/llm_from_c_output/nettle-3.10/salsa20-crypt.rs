// salsa20-crypt.rs
//
// The Salsa20 stream cipher.
//
// Based on:
// salsa20-ref.c version 20051118
// D. J. Bernstein
// Public domain.

use crate::salsa20::Salsa20Ctx;
use crate::salsa20_internal::nettle_salsa20_crypt;

/// Encrypts or decrypts data using the Salsa20 cipher.
///
/// # Arguments
/// * `ctx` - Salsa20 context containing key and nonce
/// * `length` - Length of the data to process
/// * `c` - Output buffer for the processed data
/// * `m` - Input buffer containing data to process
///
/// # Safety
/// The caller must ensure:
/// - `c` and `m` point to valid memory of at least `length` bytes
/// - The buffers don't overlap
/// - The context is properly initialized
pub fn salsa20_crypt(ctx: &mut Salsa20Ctx, length: usize, c: &mut [u8], m: &[u8]) {
    if length == 0 {
        return;
    }

    assert!(c.len() >= length);
    assert!(m.len() >= length);

    nettle_salsa20_crypt(ctx, 20, length, c, m);
}