// pbkdf2-hmac-sha384.rs

// Copyright (C) 2012 Simon Josefsson
// Copyright (C) 2021 Nicolas Mora
// Copyright (C) 2023 Translation to Rust

// This file is part of GNU Nettle Rust translation.

// GNU Nettle Rust translation is free software: you can redistribute it and/or
// modify it under the terms of either:

//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.

// or

//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.

// or both in parallel, as here.

// GNU Nettle Rust translation is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use hmac::{Hmac, Mac};
use sha2::Sha384;
use pbkdf2::pbkdf2;

pub fn pbkdf2_hmac_sha384(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    output: &mut [u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut hmac = Hmac::<Sha384>::new_from_slice(key)?;
    pbkdf2(&mut hmac, salt, iterations, output)?;
    Ok(())
}