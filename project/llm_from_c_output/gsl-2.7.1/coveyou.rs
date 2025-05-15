/*
 * This generator is taken from
 *
 * Donald E. Knuth
 * The Art of Computer Programming
 * Volume 2
 * Third Edition
 * Addison-Wesley
 * Section 3.2.2
 *
 * This implementation copyright (C) 2001 Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 * Carlo Perassi reorganized the code to use the rng framework of GSL.
 */

use std::num::Wrapping;

const MM: u64 = 0xffffffff; // 2^32 - 1

pub struct CoveyouRng {
    x: Wrapping<u64>,
}

impl CoveyouRng {
    pub fn new(seed: u64) -> Self {
        let mut rng = CoveyouRng {
            x: Wrapping(0),
        };
        rng.set(seed);
        rng
    }

    pub fn set(&mut self, s: u64) {
        let diff = ((s % 4) as i64 - 2) as u64 % MM;
        self.x = if diff != 0 {
            Wrapping((s - diff) & MM)
        } else {
            Wrapping(s & MM)
        };
    }

    pub fn get(&mut self) -> u64 {
        self.x = (self.x * (self.x + Wrapping(1))) & Wrapping(MM);
        self.x.0
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut rng = CoveyouRng::new(1234);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
        
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval < 1.0);
    }
}