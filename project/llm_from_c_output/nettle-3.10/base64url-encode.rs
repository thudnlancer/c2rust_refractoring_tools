// base64url_encode.rs

/* base64url-encode

   Copyright (C) 2015 Amos Jeffries, Niels Möller

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

pub struct Base64EncodeContext {
    word: u32,
    bits: u8,
    alphabet: &'static [u8; 64],
}

impl Base64EncodeContext {
    pub fn new() -> Self {
        const BASE64URL_ENCODE_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                                    abcdefghijklmnopqrstuvwxyz\
                                                    0123456789-_";

        Base64EncodeContext {
            word: 0,
            bits: 0,
            alphabet: BASE64URL_ENCODE_TABLE,
        }
    }
}