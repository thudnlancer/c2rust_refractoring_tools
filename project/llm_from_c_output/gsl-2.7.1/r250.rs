/* rng/r250.rs

 * This is a translation of the r250.c random number generator from GNU Scientific Library.
 * Original C code copyright (C) 1996-2000, 2007 James Theiler, Brian Gough
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or (at
 * your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
 */

pub struct R250 {
    i: usize,
    x: [u32; 250],
}

impl R250 {
    pub fn new(seed: u32) -> Self {
        let mut state = R250 {
            i: 0,
            x: [0; 250],
        };
        state.set(seed);
        state
    }

    fn lcg(n: u32) -> u32 {
        (69069_u32).wrapping_mul(n)
    }

    pub fn set(&mut self, mut s: u32) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        self.i = 0;

        // Fill the buffer
        for i in 0..250 {
            s = Self::lcg(s);
            self.x[i] = s;
        }

        // Masks for turning on the diagonal bit and turning off the leftmost bits
        let mut msb = 0x80000000_u32;
        let mut mask = 0xffffffff_u32;

        for i in 0..32 {
            let k = 7 * i + 3; // Select a word to operate on
            self.x[k] &= mask; // Turn off bits left of the diagonal
            self.x[k] |= msb;  // Turn on the diagonal bit
            mask >>= 1;
            msb >>= 1;
        }
    }

    pub fn get(&mut self) -> u32 {
        let i = self.i;
        let j = if i >= 147 { i - 147 } else { i + 103 };

        let k = self.x[i] ^ self.x[j];
        self.x[i] = k;

        self.i = if i >= 249 { 0 } else { i + 1 };

        k
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r250() {
        let mut rng = R250::new(1);
        for _ in 0..10000 {
            rng.get();
        }
        assert_eq!(rng.get(), 1100653588);
    }
}