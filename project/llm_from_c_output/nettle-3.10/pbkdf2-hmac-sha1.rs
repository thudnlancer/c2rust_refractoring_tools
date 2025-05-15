// pbkdf2-hmac-sha1.rs
//
// PKCS #5 PBKDF2 used with HMAC-SHA1, see RFC 2898.
//
// This implementation uses Rust's standard cryptographic libraries
// to provide equivalent functionality to the original C code.

use hmac::{Hmac, Mac};
use sha1::Sha1;
use pbkdf2::pbkdf2;

type HmacSha1 = Hmac<Sha1>;

/// Computes PBKDF2 using HMAC-SHA1 as the pseudorandom function.
///
/// # Arguments
/// * `key` - The input key/password
/// * `salt` - The salt value
/// * `iterations` - Number of iterations
/// * `output` - The output buffer to fill with derived bytes
///
/// # Panics
/// This function will panic if the output length is too large.
pub fn pbkdf2_hmac_sha1(
    key: &[u8],
    salt: &[u8],
    iterations: u32,
    output: &mut [u8],
) {
    pbkdf2::<HmacSha1>(key, salt, iterations, output);
}