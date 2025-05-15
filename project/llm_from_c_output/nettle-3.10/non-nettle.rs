// non-nettle.rs

// Copyright (C) 2002, 2014 Niels Möller
// 
// This file is part of GNU Nettle.
// 
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
// 
//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.
// 
// or
// 
//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.
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

use std::mem;
use std::ptr;
use std::slice;

use crate::nettle_meta::{NettleCipher, NettleAead};
use crate::ocb::{OcbCtx, OcbAes128EncryptKey, OcbBlockSize, OcbNonceSize, OcbDigestSize};
use crate::aes::{Aes128Ctx, Aes192Ctx, Aes256Ctx, AesBlockSize, Aes128KeySize, Aes192KeySize, Aes256KeySize};
use crate::arcfour::{ArcfourCtx, Arcfour128KeySize};
use crate::blowfish::{BlowfishCtx, BlowfishBlockSize, Blowfish128KeySize};
use crate::cbc::{CbcCtx, CbcSetIv};
use crate::chacha::{ChachaCtx, ChachaBlockSize, ChachaKeySize, ChachaNonceSize};
use crate::des::{DesCtx, DesBlockSize, DesKeySize, Des3Ctx, Des3BlockSize, Des3KeySize};
use crate::salsa20::{Salsa20Ctx, Salsa20BlockSize, Salsa20256KeySize, Salsa20NonceSize};

pub const OCB_NONCE_SIZE: usize = 12;

pub struct OcbAes128Ctx {
    pub ocb: OcbCtx,
    pub key: OcbAes128EncryptKey,
    pub decrypt: Aes128Ctx,
}

pub static NETTLE_DES: NettleCipher = NettleCipher {
    name: "des",
    context_size: mem::size_of::<DesCtx>(),
    block_size: DesBlockSize,
    key_size: DesKeySize,
    set_encrypt_key: des_set_key_wrapper,
    set_decrypt_key: des_set_key_wrapper,
    encrypt: des_encrypt as fn(&mut DesCtx, &[u8], &mut [u8]),
    decrypt: des_decrypt as fn(&mut DesCtx, &[u8], &mut [u8]),
};

pub static NETTLE_DES3: NettleCipher = NettleCipher {
    name: "des3",
    context_size: mem::size_of::<Des3Ctx>(),
    block_size: Des3BlockSize,
    key_size: Des3KeySize,
    set_encrypt_key: des3_set_key_wrapper,
    set_decrypt_key: des3_set_key_wrapper,
    encrypt: des3_encrypt as fn(&mut Des3Ctx, &[u8], &mut [u8]),
    decrypt: des3_decrypt as fn(&mut Des3Ctx, &[u8], &mut [u8]),
};

pub static NETTLE_BLOWFISH128: NettleCipher = NettleCipher {
    name: "blowfish128",
    context_size: mem::size_of::<BlowfishCtx>(),
    block_size: BlowfishBlockSize,
    key_size: Blowfish128KeySize,
    set_encrypt_key: blowfish128_set_key_wrapper,
    set_decrypt_key: blowfish128_set_key_wrapper,
    encrypt: blowfish_encrypt as fn(&mut BlowfishCtx, &[u8], &mut [u8]),
    decrypt: blowfish_decrypt as fn(&mut BlowfishCtx, &[u8], &mut [u8]),
};

pub static NETTLE_ARCFOUR128: NettleAead = NettleAead {
    name: "arcfour128",
    context_size: mem::size_of::<ArcfourCtx>(),
    block_size: 1,
    key_size: Arcfour128KeySize,
    nonce_size: 0,
    digest_size: 0,
    set_encrypt_key: arcfour128_set_key as fn(&mut ArcfourCtx, &[u8]),
    set_decrypt_key: arcfour128_set_key as fn(&mut ArcfourCtx, &[u8]),
    set_nonce: None,
    update: None,
    encrypt: arcfour_crypt as fn(&mut ArcfourCtx, &[u8], &mut [u8]),
    decrypt: arcfour_crypt as fn(&mut ArcfourCtx, &[u8], &mut [u8]),
    digest: None,
};

pub static NETTLE_CHACHA: NettleAead = NettleAead {
    name: "chacha",
    context_size: mem::size_of::<ChachaCtx>(),
    block_size: ChachaBlockSize,
    key_size: ChachaKeySize,
    nonce_size: ChachaNonceSize,
    digest_size: 0,
    set_encrypt_key: chacha_set_key as fn(&mut ChachaCtx, &[u8]),
    set_decrypt_key: chacha_set_key as fn(&mut ChachaCtx, &[u8]),
    set_nonce: Some(chacha_set_nonce as fn(&mut ChachaCtx, &[u8])),
    update: None,
    encrypt: chacha_crypt as fn(&mut ChachaCtx, &[u8], &mut [u8]),
    decrypt: chacha_crypt as fn(&mut ChachaCtx, &[u8], &mut [u8]),
    digest: None,
};

pub static NETTLE_SALSA20: NettleAead = NettleAead {
    name: "salsa20",
    context_size: mem::size_of::<Salsa20Ctx>(),
    block_size: Salsa20BlockSize,
    key_size: Salsa20256KeySize,
    nonce_size: Salsa20NonceSize,
    digest_size: 0,
    set_encrypt_key: salsa20_256_set_key as fn(&mut Salsa20Ctx, &[u8]),
    set_decrypt_key: salsa20_256_set_key as fn(&mut Salsa20Ctx, &[u8]),
    set_nonce: Some(salsa20_set_nonce as fn(&mut Salsa20Ctx, &[u8])),
    update: None,
    encrypt: salsa20_crypt as fn(&mut Salsa20Ctx, &[u8], &mut [u8]),
    decrypt: salsa20_crypt as fn(&mut Salsa20Ctx, &[u8], &mut [u8]),
    digest: None,
};

pub static NETTLE_SALSA20R12: NettleAead = NettleAead {
    name: "salsa20r12",
    context_size: mem::size_of::<Salsa20Ctx>(),
    block_size: Salsa20BlockSize,
    key_size: Salsa20256KeySize,
    nonce_size: Salsa20NonceSize,
    digest_size: 0,
    set_encrypt_key: salsa20_256_set_key as fn(&mut Salsa20Ctx, &[u8]),
    set_decrypt_key: salsa20_256_set_key as fn(&mut Salsa20Ctx, &[u8]),
    set_nonce: Some(salsa20_set_nonce as fn(&mut Salsa20Ctx, &[u8])),
    update: None,
    encrypt: salsa20r12_crypt as fn(&mut Salsa20Ctx, &[u8], &mut [u8]),
    decrypt: salsa20r12_crypt as fn(&mut Salsa20Ctx, &[u8], &mut [u8]),
    digest: None,
};

pub static NETTLE_CBC_AES128: NettleAead = NettleAead {
    name: "cbc_aes128",
    context_size: mem::size_of::<CbcCtx<Aes128Ctx>>(),
    block_size: AesBlockSize,
    key_size: Aes128KeySize,
    nonce_size: AesBlockSize,
    digest_size: 0,
    set_encrypt_key: cbc_aes128_set_encrypt_key as fn(&mut CbcCtx<Aes128Ctx>, &[u8]),
    set_decrypt_key: None,
    set_nonce: Some(cbc_aes128_set_iv as fn(&mut CbcCtx<Aes128Ctx>, &[u8])),
    update: None,
    encrypt: cbc_aes128_encrypt_wrapper as fn(&mut CbcCtx<Aes128Ctx>, &[u8], &mut [u8]),
    decrypt: None,
    digest: None,
};

pub static NETTLE_CBC_AES192: NettleAead = NettleAead {
    name: "cbc_aes192",
    context_size: mem::size_of::<CbcCtx<Aes192Ctx>>(),
    block_size: AesBlockSize,
    key_size: Aes192KeySize,
    nonce_size: AesBlockSize,
    digest_size: 0,
    set_encrypt_key: cbc_aes192_set_encrypt_key as fn(&mut CbcCtx<Aes192Ctx>, &[u8]),
    set_decrypt_key: None,
    set_nonce: Some(cbc_aes192_set_iv as fn(&mut CbcCtx<Aes192Ctx>, &[u8])),
    update: None,
    encrypt: cbc_aes192_encrypt_wrapper as fn(&mut CbcCtx<Aes192Ctx>, &[u8], &mut [u8]),
    decrypt: None,
    digest: None,
};

pub static NETTLE_CBC_AES256: NettleAead = NettleAead {
    name: "cbc_aes256",
    context_size: mem::size_of::<CbcCtx<Aes256Ctx>>(),
    block_size: AesBlockSize,
    key_size: Aes256KeySize,
    nonce_size: AesBlockSize,
    digest_size: 0,
    set_encrypt_key: cbc_aes256_set_encrypt_key as fn(&mut CbcCtx<Aes256Ctx>, &[u8]),
    set_decrypt_key: None,
    set_nonce: Some(cbc_aes256_set_iv as fn(&mut CbcCtx<Aes256Ctx>, &[u8])),
    update: None,
    encrypt: cbc_aes256_encrypt_wrapper as fn(&mut CbcCtx<Aes256Ctx>, &[u8], &mut [u8]),
    decrypt: None,
    digest: None,
};

pub static NETTLE_OCB_AES128: NettleAead = NettleAead {
    name: "ocb_aes128",
    context_size: mem::size_of::<OcbAes128Ctx>(),
    block_size: OcbBlockSize,
    key_size: Aes128KeySize,
    nonce_size: OcbNonceSize,
    digest_size: OcbDigestSize,
    set_encrypt_key: ocb_aes128_set_encrypt_key_wrapper as fn(&mut OcbAes128Ctx, &[u8]),
    set_decrypt_key: Some(ocb_aes128_set_decrypt_key_wrapper as fn(&mut OcbAes128Ctx, &[u8])),
    set_nonce: Some(ocb_aes128_set_nonce_wrapper as fn(&mut OcbAes128Ctx, &[u8])),
    update: Some(ocb_aes128_update_wrapper as fn(&mut OcbAes128Ctx, &[u8])),
    encrypt: ocb_aes128_encrypt_wrapper as fn(&mut OcbAes128Ctx, &[u8], &mut [u8]),
    decrypt: Some(ocb_aes128_decrypt_wrapper as fn(&mut OcbAes128Ctx, &[u8], &mut [u8])),
    digest: Some(ocb_aes128_digest_wrapper as fn(&mut OcbAes128Ctx, &mut [u8])),
};

fn des_set_key_wrapper(ctx: &mut DesCtx, key: &[u8]) {
    des_set_key(ctx, key);
}

fn des3_set_key_wrapper(ctx: &mut Des3Ctx, key: &[u8]) {
    des3_set_key(ctx, key);
}

fn blowfish128_set_key_wrapper(ctx: &mut BlowfishCtx, key: &[u8]) {
    blowfish128_set_key(ctx, key);
}

fn cbc_aes128_set_encrypt_key(ctx: &mut CbcCtx<Aes128Ctx>, key: &[u8]) {
    aes128_set_encrypt_key(&mut ctx.ctx, key);
}

fn cbc_aes128_set_iv(ctx: &mut CbcCtx<Aes128Ctx>, iv: &[u8]) {
    CbcSetIv(ctx, iv);
}

fn cbc_aes128_encrypt_wrapper(ctx: &mut CbcCtx<Aes128Ctx>, src: &[u8], dst: &mut [u8]) {
    cbc_aes128_encrypt(&ctx.ctx, &ctx.iv, src, dst);
}

fn cbc_aes192_set_encrypt_key(ctx: &mut CbcCtx<Aes192Ctx>, key: &[u8]) {
    aes192_set_encrypt_key(&mut ctx.ctx, key);
}

fn cbc_aes192_set_iv(ctx: &mut CbcCtx<Aes192Ctx>, iv: &[u8]) {
    CbcSetIv(ctx, iv);
}

fn cbc_aes192_encrypt_wrapper(ctx: &mut CbcCtx<Aes192Ctx>, src: &[u8], dst: &mut [u8]) {
    cbc_aes192_encrypt(&ctx.ctx, &ctx.iv, src, dst);
}

fn cbc_aes256_set_encrypt_key(ctx: &mut CbcCtx<Aes256Ctx>, key: &[u8]) {
    aes256_set_encrypt_key(&mut ctx.ctx, key);
}

fn cbc_aes256_set_iv(ctx: &mut CbcCtx<Aes256Ctx>, iv: &[u8]) {
    CbcSetIv(ctx, iv);
}

fn cbc_aes256_encrypt_wrapper(ctx: &mut CbcCtx<Aes256Ctx>, src: &[u8], dst: &mut [u8]) {
    cbc_aes256_encrypt(&ctx.ctx, &ctx.iv, src, dst);
}

fn ocb_aes128_set_encrypt_key_wrapper(ctx: &mut OcbAes128Ctx, key: &[u8]) {
    ocb_aes128_set_encrypt_key(&mut ctx.key, key);
}

fn ocb_aes128_set_decrypt_key_wrapper(ctx: &mut OcbAes128Ctx, key: &[u8]) {
    ocb_aes128_set_decrypt_key(&mut ctx.key, &mut ctx.decrypt, key);
}

fn ocb_aes128_set_nonce_wrapper(ctx: &mut OcbAes128Ctx, nonce: &[u8]) {
    ocb_aes128_set_nonce(&mut ctx.ocb, &ctx.key, OcbDigestSize, OcbNonceSize, nonce);
}

fn ocb_aes128_update_wrapper(ctx: &mut OcbAes128Ctx, data: &[u8]) {
    ocb_aes128_update(&mut ctx.ocb, &ctx.key, data);
}

fn ocb_aes128_encrypt_wrapper(ctx: &mut OcbAes128Ctx, src: &[u8], dst: &mut [u8]) {
    ocb_aes128_encrypt(&mut ctx.ocb, &ctx.key, src, dst);
}

fn ocb_aes128_decrypt_wrapper(ctx: &mut OcbAes128Ctx, src: &[u8], dst: &mut [u8]) {
    ocb_aes128_decrypt(&mut ctx.ocb, &ctx.key, &mut ctx.decrypt, src, dst);
}

fn ocb_aes128_digest_wrapper(ctx: &mut OcbAes128Ctx, digest: &mut [u8]) {
    ocb_aes128_digest(&mut ctx.ocb, &ctx.key, digest);
}