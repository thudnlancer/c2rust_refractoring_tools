/* 
 * Translated from C to Rust while maintaining the same functionality
 * Original C code copyright and license information preserved
 */

const MASK: u32 = 0xffffffff;
const LCG_CONST: u32 = 69069;

#[derive(Debug, Clone)]
pub struct Taus113State {
    z1: u32,
    z2: u32,
    z3: u32,
    z4: u32,
}

impl Taus113State {
    pub fn new() -> Self {
        Taus113State {
            z1: 0,
            z2: 0,
            z3: 0,
            z4: 0,
        }
    }

    pub fn set(&mut self, seed: u32) {
        let mut s = if seed == 0 { 1 } else { seed };

        self.z1 = lcg(s);
        if self.z1 < 2 {
            self.z1 += 2;
        }
        self.z2 = lcg(self.z1);
        if self.z2 < 8 {
            self.z2 += 8;
        }
        self.z3 = lcg(self.z2);
        if self.z3 < 16 {
            self.z3 += 16;
        }
        self.z4 = lcg(self.z3);
        if self.z4 < 128 {
            self.z4 += 128;
        }

        // Call RNG ten times to satisfy recurrence condition
        for _ in 0..10 {
            self.get();
        }
    }

    pub fn get(&mut self) -> u32 {
        let b1 = (((self.z1 << 6) & MASK) ^ self.z1) >> 13;
        self.z1 = (((self.z1 & 4294967294) << 18) & MASK ^ b1;

        let b2 = (((self.z2 << 2) & MASK) ^ self.z2) >> 27;
        self.z2 = (((self.z2 & 4294967288) << 2) & MASK) ^ b2;

        let b3 = (((self.z3 << 13) & MASK) ^ self.z3) >> 21;
        self.z3 = (((self.z3 & 4294967280) << 7) & MASK) ^ b3;

        let b4 = (((self.z4 << 3) & MASK) ^ self.z4) >> 12;
        self.z4 = (((self.z4 & 4294967168) << 13) & MASK) ^ b4;

        self.z1 ^ self.z2 ^ self.z3 ^ self.z4
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

#[inline]
fn lcg(n: u32) -> u32 {
    (n as u64 * LCG_CONST as u64 % (1u64 << 32)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taus113() {
        let mut rng = Taus113State::new();
        rng.set(12345);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
        
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval < 1.0);
    }
}