// hmac-streebog.rs

// HMAC-Streebog message authentication code.

// Copyright (C) 2016 Dmitry Eremin-Solenikov

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

use hmac::{Hmac, Mac};
use streebog::{Streebog256, Streebog512};
use generic_array::GenericArray;
use typenum::{U32, U64};

pub struct HmacStreebog512 {
    inner: Hmac<Streebog512>,
}

impl HmacStreebog512 {
    pub fn new(key: &[u8]) -> Self {
        Self {
            inner: Hmac::<Streebog512>::new_from_slice(key).expect("HMAC can take key of any size"),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn finalize(self) -> GenericArray<u8, U64> {
        self.inner.finalize().into_bytes()
    }
}

pub struct HmacStreebog256 {
    inner: Hmac<Streebog256>,
}

impl HmacStreebog256 {
    pub fn new(key: &[u8]) -> Self {
        Self {
            inner: Hmac::<Streebog256>::new_from_slice(key).expect("HMAC can take key of any size"),
        }
    }

    pub fn finalize(self) -> GenericArray<u8, U32> {
        self.inner.finalize().into_bytes()
    }
}