//! A maximally equidistributed combined Tausworthe generator implementation in Rust.

use std::num::Wrapping;

/// State for the Tausworthe generator
#[derive(Debug, Clone)]
pub struct TausState {
    s1: u32,
    s2: u32,
    s3: u32,
}

impl TausState {
    /// Creates a new TausState initialized with the given seed
    pub fn new(seed: u32) -> Self {
        let mut state = TausState {
            s1: 0,
            s2: 0,
            s3: 0,
        };
        state.set(seed);
        state
    }

    /// Creates a new TausState with the stricter seeding requirements
    pub fn new_taus2(seed: u32) -> Self {
        let mut state = TausState {
            s1: 0,
            s2: 0,
            s3: 0,
        };
        state.set_taus2(seed);
        state
    }

    /// Generates the next random number in the sequence
    pub fn get(&mut self) -> u32 {
        const MASK: u32 = 0xffffffff;

        fn tausworthe(s: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
            let s = Wrapping(s);
            let left = (s & Wrapping(c)) << d;
            let right = ((s << a) ^ s) >> b;
            (left ^ right).0 & MASK
        }

        self.s1 = tausworthe(self.s1, 13, 19, 4294967294, 12);
        self.s2 = tausworthe(self.s2, 2, 25, 4294967288, 4);
        self.s3 = tausworthe(self.s3, 3, 11, 4294967280, 17);

        self.s1 ^ self.s2 ^ self.s3
    }

    /// Generates the next random f64 in the range [0, 1)
    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }

    /// Initializes the generator state with the given seed
    pub fn set(&mut self, mut s: u32) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        fn lcg(n: u32) -> u32 {
            (Wrapping(69069) * Wrapping(n) & Wrapping(0xffffffff)
        }.0

        self.s1 = lcg(s);
        self.s2 = lcg(self.s1);
        self.s3 = lcg(self.s2);

        // "warm it up"
        for _ in 0..6 {
            self.get();
        }
    }

    /// Initializes the generator state with stricter seeding requirements
    pub fn set_taus2(&mut self, mut s: u32) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        fn lcg(n: u32) -> u32 {
            (Wrapping(69069) * Wrapping(n) & Wrapping(0xffffffff)).0
        }

        self.s1 = lcg(s);
        if self.s1 < 2 {
            self.s1 += 2;
        }
        self.s2 = lcg(self.s1);
        if self.s2 < 8 {
            self.s2 += 8;
        }
        self.s3 = lcg(self.s2);
        if self.s3 < 16 {
            self.s3 += 16;
        }

        // "warm it up"
        for _ in 0..6 {
            self.get();
        }
    }
}

/// Tausworthe generator type
pub struct TausGenerator {
    state: TausState,
}

impl TausGenerator {
    /// Creates a new TausGenerator with the given seed
    pub fn new(seed: u32) -> Self {
        TausGenerator {
            state: TausState::new(seed),
        }
    }

    /// Creates a new TausGenerator with stricter seeding requirements
    pub fn new_taus2(seed: u32) -> Self {
        TausGenerator {
            state: TausState::new_taus2(seed),
        }
    }

    /// Generates the next random number in the sequence
    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    /// Generates the next random f64 in the range [0, 1)
    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taus_sequence() {
        let mut rng = TausGenerator::new(1);
        // Test after 10000 iterations (6 warmup + 10000 actual)
        for _ in 0..6 + 10000 {
            rng.get();
        }
        assert_eq!(rng.get(), 2733957125);
    }

    #[test]
    fn test_taus2_seeding() {
        let mut rng = TausGenerator::new_taus2(1);
        assert!(rng.state.s1 >= 2);
        assert!(rng.state.s2 >= 8);
        assert!(rng.state.s3 >= 16);
    }
}