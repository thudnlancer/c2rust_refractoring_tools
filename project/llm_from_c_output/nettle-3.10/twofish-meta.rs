/* twofish-meta.rs

   Copyright (C) 2002, 2014 Niels Möller

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

use std::mem::size_of;
use crate::twofish::{TwofishCtx, twofish_encrypt, twofish_decrypt};

pub struct NettleCipher {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub set_encrypt_key: fn(&mut TwofishCtx, &[u8]),
    pub set_decrypt_key: fn(&mut TwofishCtx, &[u8]),
    pub encrypt: fn(&TwofishCtx, &mut [u8], &[u8]),
    pub decrypt: fn(&TwofishCtx, &mut [u8], &[u8]),
}

macro_rules! twofish_impl {
    ($bits:expr) => {
        NettleCipher {
            name: concat!("twofish", $bits),
            context_size: size_of::<TwofishCtx>(),
            block_size: TWOFISH_BLOCK_SIZE,
            key_size: TWOFISH_KEY_SIZE,
            set_encrypt_key: twofish_set_key,
            set_decrypt_key: twofish_set_key,
            encrypt: twofish_encrypt,
            decrypt: twofish_decrypt,
        }
    };
}

pub const NETTLE_TWOFISH128: NettleCipher = twofish_impl!("128");
pub const NETTLE_TWOFISH192: NettleCipher = twofish_impl!("192");
pub const NETTLE_TWOFISH256: NettleCipher = twofish_impl!("256");