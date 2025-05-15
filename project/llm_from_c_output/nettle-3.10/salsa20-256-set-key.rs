// salsa20-256-set-key.rs

// The Salsa20 stream cipher. Key setup for 256-bit keys.

// Based on:
// salsa20-ref.c version 20051118
// D. J. Bernstein
// Public domain.

use std::convert::TryInto;

pub struct Salsa20Ctx {
    input: [u32; 16],
}

impl Salsa20Ctx {
    pub fn new() -> Self {
        Salsa20Ctx {
            input: [0u32; 16],
        }
    }

    pub fn set_key_256(&mut self, key: &[u8]) -> Result<(), &'static str> {
        if key.len() != 32 {
            return Err("Key must be 32 bytes long");
        }

        self.input[1] = u32::from_le_bytes(key[0..4].try_into().unwrap());
        self.input[2] = u32::from_le_bytes(key[4..8].try_into().unwrap());
        self.input[3] = u32::from_le_bytes(key[8..12].try_into().unwrap());
        self.input[4] = u32::from_le_bytes(key[12..16].try_into().unwrap());

        self.input[11] = u32::from_le_bytes(key[16..20].try_into().unwrap());
        self.input[12] = u32::from_le_bytes(key[20..24].try_into().unwrap());
        self.input[13] = u32::from_le_bytes(key[24..28].try_into().unwrap());
        self.input[14] = u32::from_le_bytes(key[28..32].try_into().unwrap());

        // "expand 32-byte k"
        self.input[0]  = 0x61707865;
        self.input[5]  = 0x3320646e;
        self.input[10] = 0x79622d32;
        self.input[15] = 0x6b206574;

        Ok(())
    }
}