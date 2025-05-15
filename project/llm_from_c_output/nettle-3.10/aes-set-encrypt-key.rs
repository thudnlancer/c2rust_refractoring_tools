/*
   Key setup for the aes/rijndael block cipher.

   Copyright (C) 2000, 2001, 2002 Rafael R. Sevilla, Niels Möller
   Copyright (C) 2013 Niels Möller

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

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AesError {
    InvalidKeySize,
}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AesError::InvalidKeySize => write!(f, "Invalid AES key size"),
        }
    }
}

impl Error for AesError {}

pub struct AesCtx {
    pub key_size: usize,
    pub u: AesCtxUnion,
}

pub union AesCtxUnion {
    pub ctx128: Aes128Ctx,
    pub ctx192: Aes192Ctx,
    pub ctx256: Aes256Ctx,
}

pub struct Aes128Ctx {
    // AES-128 context fields
}

pub struct Aes192Ctx {
    // AES-192 context fields
}

pub struct Aes256Ctx {
    // AES-256 context fields
}

pub fn aes128_set_encrypt_key(ctx: &mut Aes128Ctx, key: &[u8]) {
    // AES-128 key setup implementation
}

pub fn aes192_set_encrypt_key(ctx: &mut Aes192Ctx, key: &[u8]) {
    // AES-192 key setup implementation
}

pub fn aes256_set_encrypt_key(ctx: &mut Aes256Ctx, key: &[u8]) {
    // AES-256 key setup implementation
}

pub const AES128_KEY_SIZE: usize = 16;
pub const AES192_KEY_SIZE: usize = 24;
pub const AES256_KEY_SIZE: usize = 32;

pub fn aes_set_encrypt_key(ctx: &mut AesCtx, key_size: usize, key: &[u8]) -> Result<(), AesError> {
    match key_size {
        AES128_KEY_SIZE => {
            unsafe {
                aes128_set_encrypt_key(&mut ctx.u.ctx128, key);
            }
        }
        AES192_KEY_SIZE => {
            unsafe {
                aes192_set_encrypt_key(&mut ctx.u.ctx192, key);
            }
        }
        AES256_KEY_SIZE => {
            unsafe {
                aes256_set_encrypt_key(&mut ctx.u.ctx256, key);
            }
        }
        _ => return Err(AesError::InvalidKeySize),
    }
    
    ctx.key_size = key_size;
    Ok(())
}