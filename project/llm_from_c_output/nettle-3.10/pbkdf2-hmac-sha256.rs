// pbkdf2-hmac-sha256.rs
//
// PKCS #5 PBKDF2 used with HMAC-SHA256, see RFC 2898.
//
// Translated from GNU Nettle implementation
//
// This code is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// This code is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use pbkdf2::pbkdf2;

pub fn pbkdf2_hmac_sha256(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    output: &mut [u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(key)?;
    pbkdf2(&mut hmac, salt, iterations, output);
    Ok(())
}