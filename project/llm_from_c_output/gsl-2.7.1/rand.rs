/* rng/rand.rs
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

/* This is the old BSD rand() generator. The sequence is

   x_{n+1} = (a x_n + c) mod m 

   with a = 1103515245, c = 12345 and m = 2^31 = 2147483648. The seed
   specifies the initial value, x_1.

   The theoretical value of x_{10001} is 1910041713.

   The period of this generator is 2^31.

   The rand() generator is not very good -- the low bits of successive
   numbers are correlated. */

pub struct RandState {
    x: u32,
}

impl RandState {
    pub fn new(seed: u32) -> Self {
        RandState { x: seed }
    }

    pub fn get(&mut self) -> u32 {
        // The following line relies on unsigned 32-bit arithmetic
        self.x = (1103515245_u32.wrapping_mul(self.x).wrapping_add(12345)) & 0x7fffffff;
        self.x
    }

    pub fn get_double(&mut self) -> f64 {
        f64::from(self.get()) / 2147483648.0
    }

    pub fn set(&mut self, seed: u32) {
        self.x = seed;
    }
}

pub struct RandType {
    pub name: &'static str,
    pub max: u32,
    pub min: u32,
}

pub const RAND_TYPE: RandType = RandType {
    name: "rand",
    max: 0x7fffffff,
    min: 0,
};

pub const GSL_RNG_RAND: &RandType = &RAND_TYPE;