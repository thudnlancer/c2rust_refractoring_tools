/* 
 * Translated from C to Rust
 * Original source: rng/randu.c
 * 
 * Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 James Theiler, Brian Gough
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

/* This is a reincarnation of the infamously bad RANDU generator.
   The sequence is,

   x_{n+1} = (a x_n) mod m

   with a = 65539 and m = 2^31 = 2147483648. The seed specifies
   the initial value, x_1.

   The theoretical value of x_{10001} is 1623524161.

   The period of this generator is 2^29.

   Note: Knuth describes this generator as "really horrible". 

   From: Park and Miller, "Random Number Generators: Good ones are
   hard to find" Communications of the ACM, October 1988, Volume 31,
   No 10, pages 1192-1201. */

use std::num::Wrapping;

const A: u32 = 65539;
const M: u32 = 0x7FFFFFFF; // 2^31 - 1
const RAND_MAX: u32 = 0x7FFFFFFF;
const RAND_MIN: u32 = 1;

#[derive(Debug, Clone)]
pub struct RanduState {
    x: u32,
}

impl RanduState {
    pub fn new(seed: u32) -> Self {
        let x = if seed == 0 { 1 } else { seed };
        RanduState { x }
    }

    pub fn get(&mut self) -> u32 {
        // Using Wrapping to handle overflow explicitly
        self.x = (Wrapping(A) * Wrapping(self.x)).0 & M;
        self.x
    }

    pub fn get_double(&mut self) -> f64 {
        f64::from(self.get()) / 2147483648.0
    }
}

pub struct RanduGenerator {
    state: RanduState,
}

impl RanduGenerator {
    pub fn new(seed: u32) -> Self {
        RanduGenerator {
            state: RanduState::new(seed),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state.get()
    }

    pub fn next_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randu_sequence() {
        let mut rng = RanduGenerator::new(1);
        let mut x = 0;
        for _ in 0..10000 {
            x = rng.next();
        }
        assert_eq!(x, 1623524161);
    }
}