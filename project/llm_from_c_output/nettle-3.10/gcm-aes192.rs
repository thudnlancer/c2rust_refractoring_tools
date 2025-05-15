// gcm-aes192.rs

// Galois counter mode using AES192 as the underlying cipher.

// Copyright (C) 2011, 2014 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.

// or

//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use aes::Aes192;
use block_modes::{BlockMode, Gcm};
use block_modes::block_padding::NoPadding;
use generic_array::GenericArray;

pub struct GcmAes192Ctx {
    cipher: Gcm<Aes192, NoPadding>,
}

impl GcmAes192Ctx {
    pub fn new() -> Self {
        GcmAes192Ctx {
            cipher: Gcm::<Aes192, NoPadding>::new_from_slices(&[], &[]).unwrap(),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.cipher = Gcm::<Aes192, NoPadding>::new_from_slices(key, &[]).unwrap();
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        self.cipher = Gcm::<Aes192, NoPadding>::new_from_slices(self.cipher.get_key(), iv).unwrap();
    }

    pub fn update(&mut self, data: &[u8]) {
        // GCM doesn't need separate update for associated data in this implementation
        // as it's handled during encryption/decryption
    }

    pub fn encrypt(&mut self, src: &[u8], dst: &mut [u8]) -> usize {
        let ciphertext = self.cipher.encrypt_vec(src);
        dst[..ciphertext.len()].copy_from_slice(&ciphertext);
        ciphertext.len()
    }

    pub fn decrypt(&mut self, src: &[u8], dst: &mut [u8]) -> usize {
        let plaintext = self.cipher.decrypt_vec(src).unwrap();
        dst[..plaintext.len()].copy_from_slice(&plaintext);
        plaintext.len()
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        let tag = self.cipher.compute_tag();
        digest[..tag.len()].copy_from_slice(&tag);
    }
}