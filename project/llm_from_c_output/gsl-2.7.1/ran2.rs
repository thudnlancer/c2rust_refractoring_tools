/* rng/ran2.rs
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

/* This is an implementation of the algorithm used in Numerical
   Recipe's ran2 generator.  It is a L'Ecuyer combined recursive
   generator with a 32-element shuffle-box.

   As far as I can tell, in general the effects of adding a shuffle
   box cannot be proven theoretically, so the period of this generator
   is unknown. 

   The period of the underlying combined generator is O(2^60). */

const M1: i64 = 2147483563;
const A1: i64 = 40014;
const Q1: i64 = 53668;
const R1: i64 = 12211;

const M2: i64 = 2147483399;
const A2: i64 = 40692;
const Q2: i64 = 52774;
const R2: i64 = 3791;

const N_SHUFFLE: usize = 32;
const N_DIV: i64 = 1 + 2147483562 / N_SHUFFLE as i64;

#[derive(Clone)]
pub struct Ran2State {
    x: i64,
    y: i64,
    n: i64,
    shuffle: [i64; N_SHUFFLE],
}

impl Ran2State {
    pub fn new(seed: i64) -> Self {
        let mut state = Ran2State {
            x: 0,
            y: 0,
            n: 0,
            shuffle: [0; N_SHUFFLE],
        };
        state.set(seed);
        state
    }

    pub fn set(&mut self, mut s: i64) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        self.y = s;

        for _ in 0..8 {
            let h = s / Q1;
            let t = A1 * (s - h * Q1) - h * R1;
            s = if t < 0 { t + M1 } else { t };
        }

        for i in (0..N_SHUFFLE).rev() {
            let h = s / Q1;
            let t = A1 * (s - h * Q1) - h * R1;
            s = if t < 0 { t + M1 } else { t };
            self.shuffle[i] = s;
        }

        self.x = s;
        self.n = s;
    }

    pub fn get(&mut self) -> i64 {
        let x = self.x;
        let y = self.y;

        let h1 = x / Q1;
        let mut t1 = A1 * (x - h1 * Q1) - h1 * R1;

        let h2 = y / Q2;
        let mut t2 = A2 * (y - h2 * Q2) - h2 * R2;

        if t1 < 0 {
            t1 += M1;
        }

        if t2 < 0 {
            t2 += M2;
        }

        self.x = t1;
        self.y = t2;

        let j = (self.n / N_DIV) as usize;
        let mut delta = self.shuffle[j] - t2;
        if delta < 1 {
            delta += M1 - 1;
        }
        self.n = delta;
        self.shuffle[j] = t1;

        self.n
    }

    pub fn get_double(&mut self) -> f64 {
        const X_MAX: f32 = 1.0 - 1.2e-7; // Numerical Recipes version of 1-FLT_EPS

        let x = self.get() as f32 / 2147483563.0f32;
        
        if x > X_MAX {
            X_MAX as f64
        } else {
            x as f64
        }
    }
}

pub struct Ran2 {
    state: Ran2State,
}

impl Ran2 {
    pub fn new(seed: i64) -> Self {
        Ran2 {
            state: Ran2State::new(seed),
        }
    }

    pub fn set(&mut self, seed: i64) {
        self.state.set(seed);
    }

    pub fn get(&mut self) -> i64 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ran2() {
        let mut rng = Ran2::new(12345);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
    }
}