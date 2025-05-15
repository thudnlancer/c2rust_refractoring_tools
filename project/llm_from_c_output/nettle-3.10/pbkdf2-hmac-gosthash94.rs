// pbkdf2-hmac-gosthash94.rs
//
// PKCS #5 PBKDF2 used with HMAC-GOSTHASH94CP.
//
// Copyright (C) 2016 Dmitry Eremin-Solenikov
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
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
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::error::Error;

use hmac::{Hmac, Mac};
use gost94::{Gost94s, Digest};

const GOSTHASH94CP_DIGEST_SIZE: usize = 32;

pub fn pbkdf2_hmac_gosthash94cp(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    output: &mut [u8],
) -> Result<(), Box<dyn Error>> {
    let mut hmac = Hmac::<Gost94s>::new_from_slice(key)?;
    pbkdf2::pbkdf2(&mut hmac, salt, iterations, output)?;
    Ok(())
}