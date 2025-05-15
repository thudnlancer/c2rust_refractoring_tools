// aes256-decrypt.rs

// Decryption function for aes/rijndael block cipher.

// Copyright (C) 2002, 2013 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.

// or

//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AesError {
    details: String
}

impl AesError {
    fn new(msg: &str) -> AesError {
        AesError{details: msg.to_string()}
    }
}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AesError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Aes256Ctx {
    keys: Vec<u32>,
}

pub fn aes256_decrypt(
    ctx: &Aes256Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8]
) -> Result<(), AesError> {
    if length % AES_BLOCK_SIZE != 0 {
        return Err(AesError::new("Input length must be multiple of block size"));
    }

    if dst.len() < length || src.len() < length {
        return Err(AesError::new("Output or input buffer too small"));
    }

    _aes_decrypt(
        _AES256_ROUNDS,
        &ctx.keys[4 * _AES256_ROUNDS..],
        &_AES_DECRYPT_TABLE,
        length,
        dst,
        src
    )
}

const AES_BLOCK_SIZE: usize = 16;
const _AES256_ROUNDS: usize = 14;
static _AES_DECRYPT_TABLE: AesDecryptTable = AesDecryptTable::new();

struct AesDecryptTable {
    // Implementation details of the AES decrypt table
}

impl AesDecryptTable {
    const fn new() -> Self {
        // Initialize decrypt table
        AesDecryptTable {}
    }
}

fn _aes_decrypt(
    rounds: usize,
    keys: &[u32],
    table: &AesDecryptTable,
    length: usize,
    dst: &mut [u8],
    src: &[u8]
) -> Result<(), AesError> {
    // Implementation of the actual AES decryption
    Ok(())
}