/* ocb-aes128.rs

   Copyright (C) 2022 Niels Möller

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

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use generic_array::GenericArray;

pub struct OcbAes128EncryptKey {
    encrypt: Aes128,
    ocb: Ocb,
}

impl OcbAes128EncryptKey {
    pub fn set_encrypt_key(&mut self, key: &[u8; 16]) {
        self.encrypt = Aes128::new(GenericArray::from_slice(key));
        self.ocb.set_key(&self.encrypt);
    }

    pub fn set_decrypt_key(
        &mut self,
        decrypt: &mut Aes128,
        key: &[u8; 16],
    ) {
        self.set_encrypt_key(key);
        *decrypt = self.encrypt.clone();
    }
}

pub fn ocb_aes128_set_nonce(
    ctx: &mut OcbCtx,
    key: &OcbAes128EncryptKey,
    tag_length: usize,
    nonce_length: usize,
    nonce: &[u8],
) {
    ctx.set_nonce(
        &key.encrypt,
        tag_length,
        nonce_length,
        nonce,
    );
}

pub fn ocb_aes128_update(
    ctx: &mut OcbCtx,
    key: &OcbAes128EncryptKey,
    length: usize,
    data: &[u8],
) {
    ctx.update(
        &key.ocb,
        &key.encrypt,
        length,
        data,
    );
}

pub fn ocb_aes128_encrypt(
    ctx: &mut OcbCtx,
    key: &OcbAes128EncryptKey,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    ctx.encrypt(
        &key.ocb,
        &key.encrypt,
        length,
        dst,
        src,
    );
}

pub fn ocb_aes128_decrypt(
    ctx: &mut OcbCtx,
    key: &OcbAes128EncryptKey,
    decrypt: &Aes128,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    ctx.decrypt(
        &key.ocb,
        &key.encrypt,
        decrypt,
        length,
        dst,
        src,
    );
}

pub fn ocb_aes128_digest(
    ctx: &mut OcbCtx,
    key: &OcbAes128EncryptKey,
    length: usize,
    digest: &mut [u8],
) {
    ctx.digest(
        &key.ocb,
        &key.encrypt,
        length,
        digest,
    );
}

pub fn ocb_aes128_encrypt_message(
    key: &OcbAes128EncryptKey,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    tlength: usize,
    clength: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    Ocb::encrypt_message(
        &key.ocb,
        &key.encrypt,
        nlength,
        nonce,
        alength,
        adata,
        tlength,
        clength,
        dst,
        src,
    );
}

pub fn ocb_aes128_decrypt_message(
    key: &OcbAes128EncryptKey,
    decrypt: &Aes128,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    tlength: usize,
    mlength: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    Ocb::decrypt_message(
        &key.ocb,
        &key.encrypt,
        decrypt,
        nlength,
        nonce,
        alength,
        adata,
        tlength,
        mlength,
        dst,
        src,
    )
}

// Note: The following structs and their implementations are placeholders
// for the actual OCB implementation which would need to be properly
// implemented according to the OCB specification.
struct Ocb;
struct OcbCtx;

impl Ocb {
    fn set_key(&mut self, _encrypt: &Aes128) {}
    fn encrypt_message(
        _ocb: &Self,
        _encrypt: &Aes128,
        _nlength: usize,
        _nonce: &[u8],
        _alength: usize,
        _adata: &[u8],
        _tlength: usize,
        _clength: usize,
        _dst: &mut [u8],
        _src: &[u8],
    ) {}
    fn decrypt_message(
        _ocb: &Self,
        _encrypt: &Aes128,
        _decrypt: &Aes128,
        _nlength: usize,
        _nonce: &[u8],
        _alength: usize,
        _adata: &[u8],
        _tlength: usize,
        _mlength: usize,
        _dst: &mut [u8],
        _src: &[u8],
    ) -> Result<(), &'static str> {
        Ok(())
    }
}

impl OcbCtx {
    fn set_nonce(
        &mut self,
        _encrypt: &Aes128,
        _tag_length: usize,
        _nonce_length: usize,
        _nonce: &[u8],
    ) {}
    fn update(
        &mut self,
        _ocb: &Ocb,
        _encrypt: &Aes128,
        _length: usize,
        _data: &[u8],
    ) {}
    fn encrypt(
        &mut self,
        _ocb: &Ocb,
        _encrypt: &Aes128,
        _length: usize,
        _dst: &mut [u8],
        _src: &[u8],
    ) {}
    fn decrypt(
        &mut self,
        _ocb: &Ocb,
        _encrypt: &Aes128,
        _decrypt: &Aes128,
        _length: usize,
        _dst: &mut [u8],
        _src: &[u8],
    ) {}
    fn digest(
        &mut self,
        _ocb: &Ocb,
        _encrypt: &Aes128,
        _length: usize,
        _digest: &mut [u8],
    ) {}
}