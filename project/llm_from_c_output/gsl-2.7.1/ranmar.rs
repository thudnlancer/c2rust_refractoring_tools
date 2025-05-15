/* 
 * Translated from C to Rust while maintaining the same functionality
 * Original C code copyright (C) 1996-2007 James Theiler, Brian Gough
 * Licensed under GNU General Public License version 3 or later
 */

use std::num::Wrapping;

const TWO_24: u32 = 16777216;  // 2^24
const DEFAULT_SEED: u32 = 362436;

#[derive(Debug, Clone)]
pub struct RanmarState {
    i: usize,
    j: usize,
    carry: i32,
    u: [u32; 97],
}

impl RanmarState {
    pub fn new(seed: u32) -> Self {
        let mut state = RanmarState {
            i: 0,
            j: 0,
            carry: 0,
            u: [0; 97],
        };
        state.set(seed);
        state
    }

    fn set(&mut self, s: u32) {
        let ij = s / 30082;
        let kl = s % 30082;
        
        let mut i = ((ij / 177) % 177 + 2) as i32;
        let mut j = (ij % 177 + 2) as i32;
        let mut k = ((kl / 169) % 178 + 1) as i32;
        let mut l = (kl % 169) as i32;

        for a in 0..97 {
            let mut sum = 0u32;
            let mut t = TWO_24;

            for _ in 0..24 {
                let m = (((i * j) % 179) * k) % 179;
                i = j;
                j = k;
                k = m;
                l = (53 * l + 1) % 169;
                t >>= 1;
                
                if (l * m) % 64 >= 32 {
                    sum += t;
                }
            }

            self.u[a] = sum;
        }

        self.i = 96;
        self.j = 32;
        self.carry = DEFAULT_SEED as i32;
    }

    pub fn get(&mut self) -> u32 {
        let i = self.i;
        let j = self.j;
        let mut carry = self.carry;

        let mut delta = self.u[i] as i32 - self.u[j] as i32;
        if delta < 0 {
            delta += TWO_24 as i32;
        }

        self.u[i] = delta as u32;

        self.i = if i == 0 { 96 } else { i - 1 };
        self.j = if j == 0 { 96 } else { j - 1 };

        carry -= 7654321;
        if carry < 0 {
            carry += (TWO_24 - 3) as i32;
        }
        self.carry = carry;

        delta -= carry;
        if delta < 0 {
            delta += TWO_24 as i32;
        }

        delta as u32
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / TWO_24 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranmar() {
        let mut rng = RanmarState::new(12345);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
        
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval < 1.0);
    }
}