// salsa20-set-key.rs

// The Salsa20 stream cipher.

// Copyright (C) 2012 Simon Josefsson, Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.

// or

//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

// Based on:
// salsa20-ref.c version 20051118
// D. J. Bernstein
// Public domain.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Salsa20Error {
    InvalidKeySize,
}

impl fmt::Display for Salsa20Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Salsa20Error::InvalidKeySize => write!(f, "Invalid key size for Salsa20"),
        }
    }
}

impl Error for Salsa20Error {}

pub struct Salsa20Context {
    // Context fields would be defined here
}

pub const SALSA20_128_KEY_SIZE: usize = 16;
pub const SALSA20_256_KEY_SIZE: usize = 32;

pub fn salsa20_set_key(
    ctx: &mut Salsa20Context,
    length: usize,
    key: &[u8],
) -> Result<(), Salsa20Error> {
    match length {
        SALSA20_128_KEY_SIZE => {
            salsa20_128_set_key(ctx, key);
            Ok(())
        }
        SALSA20_256_KEY_SIZE => {
            salsa20_256_set_key(ctx, key);
            Ok(())
        }
        _ => Err(Salsa20Error::InvalidKeySize),
    }
}

fn salsa20_128_set_key(_ctx: &mut Salsa20Context, _key: &[u8]) {
    // Implementation would go here
}

fn salsa20_256_set_key(_ctx: &mut Salsa20Context, _key: &[u8]) {
    // Implementation would go here
}