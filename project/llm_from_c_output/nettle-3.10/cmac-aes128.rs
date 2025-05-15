// cmac-aes128.rs
// CMAC using AES128 as the underlying cipher.

// Copyright (C) 2017 Red Hat, Inc.
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

use aes::Aes128;
use cmac::{Cmac, Mac};
use block_cipher::BlockCipher;

pub struct CmacAes128 {
    cmac: Cmac<Aes128>,
}

impl CmacAes128 {
    pub fn new(key: &[u8]) -> Result<Self, String> {
        if key.len() != 16 {
            return Err("AES128 requires 16-byte key".to_string());
        }
        let cipher = Aes128::new_varkey(key).map_err(|e| e.to_string())?;
        Ok(Self {
            cmac: Cmac::new(cipher),
        })
    }

    pub fn update(&mut self, data: &[u8]) {
        self.cmac.input(data);
    }

    pub fn digest(self, output: &mut [u8]) -> Result<(), String> {
        self.cmac.verify_truncated(output)
            .map_err(|e| e.to_string())
    }
}