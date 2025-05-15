//! Mersenne Twister pseudorandom number generator implementation in Rust.
//!
//! This is a translation of the original C implementation by Makoto Matsumoto and Takuji Nishimura.
//! The Rust version maintains the same functionality while adhering to Rust's safety principles.
//!
//! Original C implementation copyright (C) 1997 Makoto Matsumoto and Takuji Nishimura.
//! This Rust implementation follows the same algorithm and licensing terms (GPL).

const N: usize = 624; // Period parameters
const M: usize = 397;

const UPPER_MASK: u32 = 0x80000000; // most significant w-r bits
const LOWER_MASK: u32 = 0x7fffffff; // least significant r bits

/// Mersenne Twister state structure
pub struct MT19937 {
    mt: [u32; N],
    mti: usize,
}

impl MT19937 {
    /// Creates a new Mersenne Twister generator with default seed (4357)
    pub fn new() -> Self {
        let mut rng = Self {
            mt: [0; N],
            mti: N + 1, // force initialization
        };
        rng.set_seed(4357);
        rng
    }

    /// Creates a new Mersenne Twister generator with specified seed
    pub fn with_seed(seed: u32) -> Self {
        let mut rng = Self {
            mt: [0; N],
            mti: N + 1, // force initialization
        };
        rng.set_seed(seed);
        rng
    }

    /// Initializes the generator with a seed (2002 version)
    pub fn set_seed(&mut self, seed: u32) {
        let mut s = if seed == 0 { 4357 } else { seed };

        self.mt[0] = s;
        for i in 1..N {
            s = 1812433253u32
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
            self.mt[i] = s;
        }
        self.mti = N;
    }

    /// Initializes the generator with a seed (1999 version)
    pub fn set_seed_1999(&mut self, seed: u32) {
        let mut s = if seed == 0 { 4357 } else { seed };

        for i in 0..N {
            self.mt[i] = s & 0xffff0000;
            s = (69069u32.wrapping_mul(s).wrapping_add(1)) & 0xffffffff;
            self.mt[i] |= (s & 0xffff0000) >> 16;
            s = (69069u32.wrapping_mul(s).wrapping_add(1)) & 0xffffffff;
        }
        self.mti = N;
    }

    /// Initializes the generator with a seed (1998 version)
    pub fn set_seed_1998(&mut self, seed: u32) {
        let mut s = if seed == 0 { 4357 } else { seed };

        self.mt[0] = s;
        for i in 1..N {
            s = (69069u32.wrapping_mul(self.mt[i - 1])) & 0xffffffff;
            self.mt[i] = s;
        }
        self.mti = N;
    }

    /// Generates a random u32
    pub fn gen_u32(&mut self) -> u32 {
        if self.mti >= N {
            self.generate_numbers();
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;

        // Tempering
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        y
    }

    /// Generates a random f64 in [0,1)
    pub fn gen_f64(&mut self) -> f64 {
        self.gen_u32() as f64 / 4294967296.0
    }

    fn generate_numbers(&mut self) {
        const MAGIC: [u32; 2] = [0x0, 0x9908b0df];

        for kk in 0..(N - M) {
            let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
            self.mt[kk] = self.mt[kk + M] ^ (y >> 1) ^ MAGIC[(y & 0x1) as usize];
        }

        for kk in (N - M)..(N - 1) {
            let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
            self.mt[kk] = self.mt[kk + (M - N)] ^ (y >> 1) ^ MAGIC[(y & 0x1) as usize];
        }

        let y = (self.mt[N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
        self.mt[N - 1] = self.mt[M - 1] ^ (y >> 1) ^ MAGIC[(y & 0x1) as usize];

        self.mti = 0;
    }
}

impl Default for MT19937 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_u32() {
        let mut rng = MT19937::with_seed(5489);
        assert_eq!(rng.gen_u32(), 3499211612);
        assert_eq!(rng.gen_u32(), 581869302);
        assert_eq!(rng.gen_u32(), 3890346734);
    }

    #[test]
    fn test_gen_f64() {
        let mut rng = MT19937::with_seed(5489);
        assert!((rng.gen_f64() - 0.8147236863931799).abs() < 1e-10);
        assert!((rng.gen_f64() - 0.1354770040540886).abs() < 1e-10);
        assert!((rng.gen_f64() - 0.9057919341139246).abs() < 1e-10);
    }
}