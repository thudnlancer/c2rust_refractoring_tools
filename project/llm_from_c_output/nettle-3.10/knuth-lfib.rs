//! The "lagged fibonacci" pseudorandomness generator, described in
//! Knuth, TAoCP, 3.6
//!
//! NOTE: This generator is totally inappropriate for cryptographic
//! applications. It is useful for generating deterministic but
//! random-looking test data.

const KK: usize = 100;
const LL: usize = 37;
const MM: u32 = 1 << 30;
const TT: usize = 70;

/// Knuth's lagged fibonacci PRNG context
pub struct KnuthLfibCtx {
    x: [u32; KK],
    index: usize,
}

impl KnuthLfibCtx {
    /// Initialize the PRNG with a seed value
    pub fn init(seed: u32) -> Self {
        let mut ctx = KnuthLfibCtx {
            x: [0; KK],
            index: 0,
        };
        
        let mut t;
        let mut j;
        let mut x = [0u32; 2 * KK - 1];
        let mut ss = (seed + 2) & (MM - 2);

        for j in 0..KK {
            x[j] = ss;
            ss <<= 1;
            if ss >= MM {
                ss -= MM - 2;
            }
        }
        for j in KK..(2 * KK - 1) {
            x[j] = 0;
        }

        x[1] += 1;

        ss = seed & (MM - 1);
        t = TT - 1;
        while t > 0 {
            for j in (1..KK).rev() {
                x[j + j] = x[j];
            }
            for j in (KK - LL..(2 * KK - 2)).rev().step_by(2) {
                x[2 * KK - 1 - j] = x[j] & !1;
            }
            for j in (KK..(2 * KK - 2)).rev() {
                if x[j] & 1 != 0 {
                    x[j - (KK - LL)] = (x[j - (KK - LL)] - x[j]) & (MM - 1);
                    x[j - KK] = (x[j - KK] - x[j]) & (MM - 1);
                }
            }
            if ss & 1 != 0 {
                for j in (1..=KK).rev() {
                    x[j] = x[j - 1];
                }
                x[0] = x[KK];
                if x[KK] & 1 != 0 {
                    x[LL] = (x[LL] - x[KK]) & (MM - 1);
                }
            }
            if ss != 0 {
                ss >>= 1;
            } else {
                t -= 1;
            }
        }
        for j in 0..LL {
            ctx.x[j + KK - LL] = x[j];
        }
        for j in LL..KK {
            ctx.x[j - LL] = x[j];
        }

        ctx
    }

    /// Get a single number in the range 0 ... 2^30-1
    pub fn get(&mut self) -> u32 {
        assert!(self.index < KK);
        
        let value = self.x[self.index];
        self.x[self.index] = self.x[self.index].wrapping_sub(
            self.x[(self.index + KK - LL) % KK]
        ) & (MM - 1);
        
        self.index = (self.index + 1) % KK;

        value
    }

    /// Get an array of numbers
    pub fn get_array(&mut self, a: &mut [u32]) {
        for item in a.iter_mut() {
            *item = self.get();
        }
    }

    /// Get an array of octets
    pub fn random(&mut self, dst: &mut [u8]) {
        let mut chunks = dst.chunks_exact_mut(3);
        for chunk in &mut chunks {
            let mut value = self.get();
            value ^= value >> 24;
            chunk.copy_from_slice(&value.to_le_bytes()[..3]);
        }

        let remainder = chunks.into_remainder();
        match remainder.len() {
            1 => remainder[0] = self.get() as u8,
            2 => remainder.copy_from_slice(&(self.get() as u16).to_le_bytes()),
            _ => (),
        }
    }
}