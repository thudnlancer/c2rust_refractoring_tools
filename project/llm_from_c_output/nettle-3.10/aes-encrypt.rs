// aes-encrypt.rs

// Encryption function for the aes/rijndael block cipher.

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

use std::mem;

#[derive(Debug)]
pub enum AesError {
    InvalidKeySize,
}

pub struct AesCtx {
    key_size: usize,
    u: AesCtxUnion,
}

union AesCtxUnion {
    ctx128: aes128::Aes128Ctx,
    ctx192: aes192::Aes192Ctx,
    ctx256: aes256::Aes256Ctx,
}

pub fn aes_encrypt(
    ctx: &AesCtx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), AesError> {
    match ctx.key_size {
        aes128::AES128_KEY_SIZE => {
            let ctx128 = unsafe { &ctx.u.ctx128 };
            aes128::aes128_encrypt(ctx128, length, dst, src);
            Ok(())
        }
        aes192::AES192_KEY_SIZE => {
            let ctx192 = unsafe { &ctx.u.ctx192 };
            aes192::aes192_encrypt(ctx192, length, dst, src);
            Ok(())
        }
        aes256::AES256_KEY_SIZE => {
            let ctx256 = unsafe { &ctx.u.ctx256 };
            aes256::aes256_encrypt(ctx256, length, dst, src);
            Ok(())
        }
        _ => Err(AesError::InvalidKeySize),
    }
}

mod aes128 {
    pub const AES128_KEY_SIZE: usize = 16;
    
    pub struct Aes128Ctx;
    
    pub fn aes128_encrypt(_ctx: &Aes128Ctx, _length: usize, _dst: &mut [u8], _src: &[u8]) {
        // Implementation would go here
    }
}

mod aes192 {
    pub const AES192_KEY_SIZE: usize = 24;
    
    pub struct Aes192Ctx;
    
    pub fn aes192_encrypt(_ctx: &Aes192Ctx, _length: usize, _dst: &mut [u8], _src: &[u8]) {
        // Implementation would go here
    }
}

mod aes256 {
    pub const AES256_KEY_SIZE: usize = 32;
    
    pub struct Aes256Ctx;
    
    pub fn aes256_encrypt(_ctx: &Aes256Ctx, _length: usize, _dst: &mut [u8], _src: &[u8]) {
        // Implementation would go here
    }
}