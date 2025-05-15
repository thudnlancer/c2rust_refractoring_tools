// cmac-des3.rs
//
// CMAC using TripleDES as the underlying cipher.
//
// Copyright (C) 2019 Dmitry Eremin-Solenikov
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

use cmac::Cmac;
use des::TdesEde3;
use cipher::{BlockCipher, BlockEncrypt, KeyInit};

pub struct CmacDes3Ctx {
    cmac: Cmac<TdesEde3>,
}

impl CmacDes3Ctx {
    pub fn new(key: &[u8]) -> Result<Self, cipher::InvalidLength> {
        let cipher = TdesEde3::new_from_slice(key)?;
        Ok(Self {
            cmac: Cmac::new(cipher),
        })
    }

    pub fn update(&mut self, data: &[u8]) {
        self.cmac.update(data);
    }

    pub fn finalize(self, digest: &mut [u8]) -> Result<(), cipher::InvalidLength> {
        self.cmac.finalize_into(digest.into())
    }
}