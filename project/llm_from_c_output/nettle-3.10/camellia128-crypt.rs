/* camellia128-crypt.rs

   Crypt function for the camellia block cipher.

   Copyright (C) 2010, 2013 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

use std::assert;

use crate::camellia_internal::{
    Camellia128Context, 
    CAMELLIA_BLOCK_SIZE,
    _nettle_camellia_crypt,
    _CAMELLIA128_NKEYS,
    _nettle_camellia_table
};

/// The main point on this function is to help the assembler
/// implementations of _nettle_camellia_crypt to get the table pointer.
/// For PIC code, the details can be complex and system dependent.
pub fn camellia128_crypt(
    ctx: &Camellia128Context,
    length: usize,
    dst: &mut [u8],
    src: &[u8]
) -> Result<(), &'static str> {
    assert!(length % CAMELLIA_BLOCK_SIZE == 0, "Length must be a multiple of block size");
    
    if dst.len() < length || src.len() < length {
        return Err("Destination or source buffer too small");
    }

    _nettle_camellia_crypt(
        _CAMELLIA128_NKEYS,
        &ctx.keys,
        &_nettle_camellia_table,
        length,
        dst,
        src
    );

    Ok(())
}