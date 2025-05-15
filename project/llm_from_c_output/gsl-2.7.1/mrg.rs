/* rng/mrg.rs

This is a translation of the C code for a fifth-order multiple recursive generator.
The original C code was part of the GNU Scientific Library (GSL).

The sequence is:
x_n = (a_1 x_{n-1} + a_5 x_{n-5}) mod m
with a_1 = 107374182, a_2 = a_3 = a_4 = 0, a_5 = 104480 and m = 2^31-1.

The period of this generator is about 2^155.

From: P. L'Ecuyer, F. Blouin, and R. Coutre, "A search for good
multiple recursive random number generators", ACM Transactions on
Modeling and Computer Simulation 3, 87-98 (1993).
*/

use std::num::Wrapping;

const M: i64 = 2147483647;
const A1: i64 = 107374182;
const Q1: i64 = 20;
const R1: i64 = 7;
const A5: i64 = 104480;
const Q5: i64 = 20554;
const R5: i64 = 1727;

#[derive(Debug, Clone)]
pub struct MrgState {
    x1: i64,
    x2: i64,
    x3: i64,
    x4: i64,
    x5: i64,
}

pub struct MrgRng {
    state: MrgState,
}

impl MrgRng {
    pub fn new(seed: u64) -> Self {
        let mut state = MrgState {
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
        };
        Self::set(&mut state, seed);
        Self { state }
    }

    fn set(state: &mut MrgState, mut s: u64) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        fn lcg(n: u64) -> u64 {
            (Wrapping(69069) * Wrapping(n)).0 & 0xffffffff
        }

        s = lcg(s);
        state.x1 = (s % M as u64) as i64;
        s = lcg(s);
        state.x2 = (s % M as u64) as i64;
        s = lcg(s);
        state.x3 = (s % M as u64) as i64;
        s = lcg(s);
        state.x4 = (s % M as u64) as i64;
        s = lcg(s);
        state.x5 = (s % M as u64) as i64;

        // "warm it up" with at least 5 calls to go through all the x values
        for _ in 0..6 {
            Self::get(state);
        }
    }

    fn get(state: &mut MrgState) -> u64 {
        let h5 = state.x5 / Q5;
        let mut p5 = A5 * (state.x5 - h5 * Q5) - h5 * R5;
        if p5 > 0 {
            p5 -= M;
        }

        let h1 = state.x1 / Q1;
        let mut p1 = A1 * (state.x1 - h1 * Q1) - h1 * R1;
        if p1 < 0 {
            p1 += M;
        }

        state.x5 = state.x4;
        state.x4 = state.x3;
        state.x3 = state.x2;
        state.x2 = state.x1;

        state.x1 = p1 + p5;

        if state.x1 < 0 {
            state.x1 += M;
        }

        state.x1 as u64
    }

    pub fn next(&mut self) -> u64 {
        Self::get(&mut self.state)
    }

    pub fn next_double(&mut self) -> f64 {
        self.next() as f64 / 2147483647.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mrg() {
        let mut rng = MrgRng::new(1);
        // Test value from original C code documentation
        let mut last = 0;
        for _ in 0..10000 {
            last = rng.next();
        }
        assert_eq!(last, 2064828650);
    }
}