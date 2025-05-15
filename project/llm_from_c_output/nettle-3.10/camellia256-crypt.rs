// camellia256-crypt.rs

// Crypt function for the camellia block cipher.

// Copyright (C) 2010, 2013 Niels Möller

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

use std::convert::TryInto;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BlockSizeError;

impl fmt::Display for BlockSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input length must be a multiple of CAMELLIA_BLOCK_SIZE")
    }
}

impl Error for BlockSizeError {}

pub fn camellia256_crypt(
    ctx: &Camellia256Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), BlockSizeError> {
    if length % CAMELLIA_BLOCK_SIZE != 0 {
        return Err(BlockSizeError);
    }

    _nettle_camellia_crypt(
        _CAMELLIA256_NKEYS,
        &ctx.keys,
        &_nettle_camellia_table,
        length,
        dst,
        src,
    );

    Ok(())
}

// Note: The following types and constants would need to be defined elsewhere in your Rust project:
// - Camellia256Ctx struct
// - CAMELLIA_BLOCK_SIZE constant
// - _CAMELLIA256_NKEYS constant
// - _nettle_camellia_table constant
// - _nettle_camellia_crypt function implementation