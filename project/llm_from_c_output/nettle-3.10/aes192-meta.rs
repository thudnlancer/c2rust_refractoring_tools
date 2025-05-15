/* aes192-meta.rs

   Copyright (C) 2013, 2014 Niels Möller

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

pub const AES_BLOCK_SIZE: usize = 16;
pub const AES192_KEY_SIZE: usize = 24;

pub struct Aes192Ctx {
    // AES-192 context fields would be defined here
}

pub type NettleSetKeyFunc = fn(ctx: &mut Aes192Ctx, key: &[u8]) -> Result<(), &'static str>;
pub type NettleCipherFunc = fn(ctx: &Aes192Ctx, length: usize, dst: &mut [u8], src: &[u8]) -> Result<(), &'static str>;

pub fn aes192_set_encrypt_key(ctx: &mut Aes192Ctx, key: &[u8]) -> Result<(), &'static str> {
    // Implementation would go here
    Ok(())
}

pub fn aes192_set_decrypt_key(ctx: &mut Aes192Ctx, key: &[u8]) -> Result<(), &'static str> {
    // Implementation would go here
    Ok(())
}

pub fn aes192_encrypt(ctx: &Aes192Ctx, length: usize, dst: &mut [u8], src: &[u8]) -> Result<(), &'static str> {
    // Implementation would go here
    Ok(())
}

pub fn aes192_decrypt(ctx: &Aes192Ctx, length: usize, dst: &mut [u8], src: &[u8]) -> Result<(), &'static str> {
    // Implementation would go here
    Ok(())
}

pub struct NettleCipher {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub set_encrypt_key: NettleSetKeyFunc,
    pub set_decrypt_key: NettleSetKeyFunc,
    pub encrypt: NettleCipherFunc,
    pub decrypt: NettleCipherFunc,
}

pub const NETTLE_AES192: NettleCipher = NettleCipher {
    name: "aes192",
    context_size: size_of::<Aes192Ctx>(),
    block_size: AES_BLOCK_SIZE,
    key_size: AES192_KEY_SIZE,
    set_encrypt_key: aes192_set_encrypt_key,
    set_decrypt_key: aes192_set_decrypt_key,
    encrypt: aes192_encrypt,
    decrypt: aes192_decrypt,
};