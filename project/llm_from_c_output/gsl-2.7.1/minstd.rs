/* rng/minstd.rs
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

/* MINSTD is Park and Miller's minimal standard generator (i.e. it's
   not particularly good).

   The sequence is

   x_{n+1} = (a x_n) mod m

   with a = 16807 and m = 2^31 - 1 = 2147483647. The seed specifies
   the initial value, x_1.  

   The theoretical value of x_{10001} is 1043618065, starting with a
   seed of x_1 = 1.

   The period of this generator is 2^31.

   It is used as the RNUN subroutine in the IMSL Library and the RAND
   function in MATLAB. The generator is sometimes known by the acronym
   "GGL" (I'm not sure what that stands for).

   From: Park and Miller, "Random Number Generators: Good ones are
   hard to find" Communications of the ACM, October 1988, Volume 31,
   No 10, pages 1192-1201. */

use std::num::Wrapping;

const M: i64 = 2147483647;
const A: i64 = 16807;
const Q: i64 = 127773;
const R: i64 = 2836;

#[derive(Debug, Clone)]
pub struct MinstdState {
    x: u32,
}

impl MinstdState {
    pub fn new(seed: u32) -> Self {
        let mut s = seed;
        if s == 0 {
            s = 1; // default seed is 1
        }
        MinstdState { x: s }
    }

    pub fn get(&mut self) -> u32 {
        let x = self.x as i64;
        let h = x / Q;
        let t = A * (x - h * Q) - h * R;

        self.x = if t < 0 {
            (t + M) as u32
        } else {
            t as u32
        };

        self.x
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }
}

pub struct MinstdRng {
    state: MinstdState,
}

impl MinstdRng {
    pub fn new(seed: u32) -> Self {
        MinstdRng {
            state: MinstdState::new(seed),
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
    fn test_minstd() {
        let mut rng = MinstdRng::new(1);
        let mut last = 0;
        for _ in 0..10000 {
            last = rng.next();
        }
        assert_eq!(last, 1043618065);
    }
}