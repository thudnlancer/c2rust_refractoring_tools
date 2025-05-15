// serpent-meta.rs

// Copyright (C) 2002, 2014 Niels Möller
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

use std::mem::size_of;

pub const SERPENT_BLOCK_SIZE: usize = 16;

pub trait Cipher {
    fn name(&self) -> &'static str;
    fn context_size(&self) -> usize;
    fn block_size(&self) -> usize;
    fn key_size(&self) -> usize;
    fn set_encrypt_key(&self, ctx: &mut [u8], key: &[u8]);
    fn set_decrypt_key(&self, ctx: &mut [u8], key: &[u8]);
    fn encrypt(&self, ctx: &[u8], dst: &mut [u8], src: &[u8]);
    fn decrypt(&self, ctx: &[u8], dst: &mut [u8], src: &[u8]);
}

macro_rules! define_serpent_cipher {
    ($name:ident, $bits:expr, $set_key:ident) => {
        pub struct $name;

        impl Cipher for $name {
            fn name(&self) -> &'static str {
                concat!("serpent", stringify!($bits))
            }

            fn context_size(&self) -> usize {
                size_of::<serpent::SerpentContext>()
            }

            fn block_size(&self) -> usize {
                SERPENT_BLOCK_SIZE
            }

            fn key_size(&self) -> usize {
                $bits / 8
            }

            fn set_encrypt_key(&self, ctx: &mut [u8], key: &[u8]) {
                let ctx = unsafe { &mut *(ctx.as_mut_ptr() as *mut serpent::SerpentContext) };
                serpent::$set_key(ctx, key);
            }

            fn set_decrypt_key(&self, ctx: &mut [u8], key: &[u8]) {
                let ctx = unsafe { &mut *(ctx.as_mut_ptr() as *mut serpent::SerpentContext) };
                serpent::$set_key(ctx, key);
            }

            fn encrypt(&self, ctx: &[u8], dst: &mut [u8], src: &[u8]) {
                let ctx = unsafe { &*(ctx.as_ptr() as *const serpent::SerpentContext) };
                serpent::serpent_encrypt(ctx, dst, src);
            }

            fn decrypt(&self, ctx: &[u8], dst: &mut [u8], src: &[u8]) {
                let ctx = unsafe { &*(ctx.as_ptr() as *const serpent::SerpentContext) };
                serpent::serpent_decrypt(ctx, dst, src);
            }
        }
    };
}

define_serpent_cipher!(Serpent128, 128, serpent128_set_key);
define_serpent_cipher!(Serpent192, 192, serpent192_set_key);
define_serpent_cipher!(Serpent256, 256, serpent256_set_key);

pub const NettleSerpent128: Serpent128 = Serpent128;
pub const NettleSerpent192: Serpent192 = Serpent192;
pub const NettleSerpent256: Serpent256 = Serpent256;