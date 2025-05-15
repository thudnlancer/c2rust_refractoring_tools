/* rng/vax.rs
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

/* This is the old vax generator MTH$RANDOM. The sequence is,

   x_{n+1} = (a x_n + c) mod m

   with a = 69069, c = 1 and m = 2^32. The seed specifies the initial
   value, x_1.

   The theoretical value of x_{10001} is 3051034865.

   The period of this generator is 2^32. */

use std::num::Wrapping;

#[derive(Clone)]
pub struct VaxState {
    x: u32,
}

impl VaxState {
    pub fn new(seed: u32) -> Self {
        VaxState { x: seed }
    }

    pub fn get(&mut self) -> u32 {
        self.x = (Wrapping(69069) * Wrapping(self.x) + Wrapping(1)).0;
        self.x
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }

    pub fn set(&mut self, seed: u32) {
        self.x = seed;
    }
}

pub struct VaxRng {
    state: VaxState,
}

impl VaxRng {
    pub fn new(seed: u32) -> Self {
        VaxRng {
            state: VaxState::new(seed),
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state.get()
    }

    pub fn next_f64(&mut self) -> f64 {
        self.state.get_double()
    }

    pub fn reseed(&mut self, seed: u32) {
        self.state.set(seed);
    }
}

pub const VAX_RNG_MAX: u32 = 0xffffffff;
pub const VAX_RNG_MIN: u32 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vax_rng() {
        let mut rng = VaxRng::new(0);
        for _ in 0..10000 {
            rng.next_u32();
        }
        assert_eq!(rng.next_u32(), 3051034865);
    }
}