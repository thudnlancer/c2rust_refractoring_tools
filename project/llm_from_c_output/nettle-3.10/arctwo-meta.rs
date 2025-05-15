/* arctwo-meta.rs

   Copyright (C) 2004 Simon Josefsson
   Copyright (C) 2014 Niels Möller

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
use nettle_meta::{NettleCipher, NettleSetKeyFunc, NettleCipherFunc};
use arctwo::{ArctwoCtx, ARCTWO_BLOCK_SIZE, arctwo_encrypt, arctwo_decrypt};

macro_rules! arctwo_cipher {
    ($bits:expr) => {
        NettleCipher {
            name: concat!("arctwo", stringify!($bits)),
            context_size: size_of::<ArctwoCtx>(),
            block_size: ARCTWO_BLOCK_SIZE,
            key_size: $bits / 8,
            set_encrypt_key: arctwo_set_key!($bits) as NettleSetKeyFunc,
            set_decrypt_key: arctwo_set_key!($bits) as NettleSetKeyFunc,
            encrypt: arctwo_encrypt as NettleCipherFunc,
            decrypt: arctwo_decrypt as NettleCipherFunc,
        }
    };
}

macro_rules! arctwo_set_key {
    ($bits:expr) => {
        paste::item! {
            [<arctwo $bits _set_key>]
        }
    };
}

pub const NETTLE_ARCTWO40: NettleCipher = arctwo_cipher!(40);
pub const NETTLE_ARCTWO64: NettleCipher = arctwo_cipher!(64);
pub const NETTLE_ARCTWO128: NettleCipher = arctwo_cipher!(128);

/* Gutmann variant. */
pub const NETTLE_ARCTWO_GUTMANN128: NettleCipher = NettleCipher {
    name: "arctwo_gutmann128",
    context_size: size_of::<ArctwoCtx>(),
    block_size: ARCTWO_BLOCK_SIZE,
    key_size: 16,
    set_encrypt_key: arctwo128_set_key_gutmann as NettleSetKeyFunc,
    set_decrypt_key: arctwo128_set_key_gutmann as NettleSetKeyFunc,
    encrypt: arctwo_encrypt as NettleCipherFunc,
    decrypt: arctwo_decrypt as NettleCipherFunc,
};