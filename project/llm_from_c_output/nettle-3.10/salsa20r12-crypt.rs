// salsa20r12-crypt.rs

// The Salsa20 stream cipher, reduced round variant.

// Copyright (C) 2013 Nikos Mavrogiannopoulos

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

// Based on:
// salsa20-ref.c version 20051118
// D. J. Bernstein
// Public domain.

use crate::salsa20::{Salsa20Context, salsa20_crypt};

/// Encrypts or decrypts data using Salsa20 with 12 rounds
///
/// # Arguments
///
/// * `ctx` - Salsa20 context containing key and nonce
/// * `length` - Length of the data to process
/// * `output` - Output buffer for the result
/// * `input` - Input data to process
///
/// # Returns
///
/// * `()` - Nothing is returned, the result is written to output buffer
pub fn salsa20r12_crypt(ctx: &mut Salsa20Context, length: usize, output: &mut [u8], input: &[u8]) {
    if length == 0 {
        return;
    }

    salsa20_crypt(ctx, 12, length, output, input);
}