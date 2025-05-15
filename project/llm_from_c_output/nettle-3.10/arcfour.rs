//! The arcfour/rc4 stream cipher implementation in Rust.

use std::convert::TryInto;

/// Minimum and maximum keysizes, and a reasonable default. In octets.
pub const ARCFOUR_MIN_KEY_SIZE: usize = 1;
pub const ARCFOUR_MAX_KEY_SIZE: usize = 256;
pub const ARCFOUR_KEY_SIZE: usize = 16;
pub const ARCFOUR128_KEY_SIZE: usize = 16;

/// Arcfour cipher context
pub struct ArcfourCtx {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl ArcfourCtx {
    /// Creates a new Arcfour context
    pub fn new() -> Self {
        ArcfourCtx {
            s: [0; 256],
            i: 0,
            j: 0,
        }
    }

    /// Sets the key for the Arcfour cipher
    pub fn set_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
        if key.len() < ARCFOUR_MIN_KEY_SIZE || key.len() > ARCFOUR_MAX_KEY_SIZE {
            return Err("Invalid key length");
        }

        // Initialize state
        for (i, val) in self.s.iter_mut().enumerate() {
            *val = i as u8;
        }

        let mut j = 0u8;
        for i in 0..256 {
            j = j.wrapping_add(self.s[i]).wrapping_add(key[i % key.len()]);
            self.s.swap(i, j as usize);
        }

        self.i = 0;
        self.j = 0;
        Ok(())
    }

    /// Sets a 128-bit key for the Arcfour cipher
    pub fn set_key_128(&mut self, key: &[u8; ARCFOUR128_KEY_SIZE]) -> Result<(), &'static str> {
        self.set_key(key)
    }

    /// Encrypts or decrypts data using the Arcfour cipher
    pub fn crypt(&mut self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        if src.len() != dst.len() {
            return Err("Source and destination buffers must be of equal length");
        }

        let mut i = self.i;
        let mut j = self.j;

        for (k, &val) in src.iter().enumerate() {
            i = i.wrapping_add(1);
            let si = self.s[i as usize];
            j = j.wrapping_add(si);
            let sj = self.s[j as usize];
            
            self.s.swap(i as usize, j as usize);
            dst[k] = val ^ self.s[(si.wrapping_add(sj)) as usize];
        }

        self.i = i;
        self.j = j;
        Ok(())
    }
}

impl Default for ArcfourCtx {
    fn default() -> Self {
        Self::new()
    }
}