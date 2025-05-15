/* chacha-set-nonce.rs

   Setting the nonce the ChaCha stream cipher.
   Based on the Salsa20 implementation in Nettle.

   Copyright (C) 2013 Joachim Strömbergon
   Copyright (C) 2012 Simon Josefsson
   Copyright (C) 2012, 2014 Niels Möller

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

/* Based on:
   ChaCha specification (doc id: 4027b5256e17b9796842e6d0f68b0b5e) and reference 
   implementation dated 2008.01.20
   D. J. Bernstein
   Public domain.
*/

use std::convert::TryInto;

pub struct ChaChaCtx {
    pub state: [u32; 16],
}

impl ChaChaCtx {
    pub fn set_nonce(&mut self, nonce: &[u8]) -> Result<(), &'static str> {
        if nonce.len() < 8 {
            return Err("Nonce must be at least 8 bytes");
        }
        self.state[12] = 0;
        self.state[13] = 0;
        self.state[14] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        self.state[15] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        Ok(())
    }

    pub fn set_nonce96(&mut self, nonce: &[u8]) -> Result<(), &'static str> {
        if nonce.len() < 12 {
            return Err("Nonce must be at least 12 bytes");
        }
        self.state[12] = 0;
        self.state[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        self.state[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        self.state[15] = u32::from_le_bytes(nonce[8..12].try_into().unwrap());
        Ok(())
    }

    pub fn set_counter(&mut self, counter: &[u8]) -> Result<(), &'static str> {
        if counter.len() < 8 {
            return Err("Counter must be at least 8 bytes");
        }
        self.state[12] = u32::from_le_bytes(counter[0..4].try_into().unwrap());
        self.state[13] = u32::from_le_bytes(counter[4..8].try_into().unwrap());
        Ok(())
    }

    pub fn set_counter32(&mut self, counter: &[u8]) -> Result<(), &'static str> {
        if counter.len() < 4 {
            return Err("Counter must be at least 4 bytes");
        }
        self.state[12] = u32::from_le_bytes(counter[0..4].try_into().unwrap());
        Ok(())
    }
}