/* This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 3 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.  You should have received
   a copy of the GNU General Public License along with this library; if
   not, write to the Free Software Foundation, Inc., 51 Franklin Street,
   Fifth Floor, Boston, MA 02110-1301, USA.

   From Robert M. Ziff, "Four-tap shift-register-sequence
   random-number generators," Computers in Physics 12(4), Jul/Aug
   1998, pp 385-392.
*/

const A: usize = 471;
const B: usize = 1586;
const C: usize = 6988;
const D: usize = 9689;
const M: usize = 16383; // = 2^14-1
const ARRAY_SIZE: usize = M + 1;

pub struct Gfsr4 {
    nd: usize,
    ra: [u32; ARRAY_SIZE],
}

impl Gfsr4 {
    pub fn new(seed: u32) -> Self {
        let mut state = Gfsr4 {
            nd: 0,
            ra: [0; ARRAY_SIZE],
        };
        state.set(seed);
        state
    }

    fn lcg(n: u32) -> u32 {
        (69069 * n) & 0xffffffff
    }

    pub fn set(&mut self, mut s: u32) {
        if s == 0 {
            s = 4357; // default seed
        }

        let mut msb = 0x80000000;
        let mut mask = 0xffffffff;

        // Initialize state using LCG
        for i in 0..ARRAY_SIZE {
            let mut t = 0;
            let mut bit = msb;
            for _ in 0..32 {
                s = Self::lcg(s);
                if s & msb != 0 {
                    t |= bit;
                }
                bit >>= 1;
            }
            self.ra[i] = t;
        }

        // Perform orthogonalization
        for i in 0..32 {
            let k = 7 + i * 3;
            self.ra[k] &= mask;
            self.ra[k] |= msb;
            mask >>= 1;
            msb >>= 1;
        }

        self.nd = 32;
    }

    pub fn get(&mut self) -> u32 {
        self.nd = (self.nd + 1) & M;
        self.ra[self.nd] = self.ra[(self.nd + ARRAY_SIZE - A) & M]
            ^ self.ra[(self.nd + ARRAY_SIZE - B) & M]
            ^ self.ra[(self.nd + ARRAY_SIZE - C) & M]
            ^ self.ra[(self.nd + ARRAY_SIZE - D) & M];
        self.ra[self.nd]
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gfsr4() {
        let mut rng = Gfsr4::new(1234);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
        
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval < 1.0);
    }
}