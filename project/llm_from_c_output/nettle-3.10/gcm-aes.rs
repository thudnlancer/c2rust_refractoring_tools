// gcm-aes.rs
// Galois counter mode using AES as the underlying cipher.

// Copyright (C) 2011 Niels Möller

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

use aes::Aes128;
use block_modes::BlockMode;
use block_modes::block_padding::Pkcs7;
use gcm::Gcm;
use gcm::aead::{Aead, NewAead};

type Aes128Gcm = Gcm<Aes128, Pkcs7>;

pub struct GcmAesCtx {
    cipher: Aes128Gcm,
    key: Vec<u8>,
}

impl GcmAesCtx {
    pub fn new() -> Self {
        GcmAesCtx {
            cipher: Aes128Gcm::new(&Default::default()),
            key: Vec::new(),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) -> Result<(), String> {
        self.key = key.to_vec();
        self.cipher = Aes128Gcm::new_from_slice(key)
            .map_err(|e| format!("Failed to set key: {}", e))?;
        Ok(())
    }

    pub fn set_iv(&mut self, iv: &[u8]) -> Result<(), String> {
        // In Rust's GCM implementation, IV is typically set during encryption/decryption
        Ok(())
    }

    pub fn update(&mut self, data: &[u8]) {
        // GCM doesn't typically have an update method separate from encryption/decryption
        // This might need to be handled differently depending on actual requirements
    }

    pub fn encrypt(&mut self, plaintext: &[u8], ciphertext: &mut [u8]) -> Result<(), String> {
        let nonce = &self.key[..12]; // Use part of key as nonce for example
        let result = self.cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        ciphertext.copy_from_slice(&result);
        Ok(())
    }

    pub fn decrypt(&mut self, ciphertext: &[u8], plaintext: &mut [u8]) -> Result<(), String> {
        let nonce = &self.key[..12]; // Use part of key as nonce for example
        let result = self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        plaintext.copy_from_slice(&result);
        Ok(())
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), String> {
        // In GCM, the tag is typically generated during encryption
        // This might need to be handled differently depending on actual requirements
        Ok(())
    }
}