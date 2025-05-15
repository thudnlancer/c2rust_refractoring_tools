/* rng/ran0.rs
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
   Recipe's ran0 generator. It is the same as MINSTD with an XOR mask
   of 123459876 on the seed.

   The period of this generator is 2^31.  

   Note, if you choose a seed of 123459876 it would give a degenerate
   series 0,0,0,0, ...  I've made that into an error. */

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Ran0Error {
    details: String
}

impl Ran0Error {
    fn new(msg: &str) -> Ran0Error {
        Ran0Error{details: msg.to_string()}
    }
}

impl fmt::Display for Ran0Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for Ran0Error {
    fn description(&self) -> &str {
        &self.details
    }
}

const M: i64 = 2147483647;
const A: i64 = 16807;
const Q: i64 = 127773;
const R: i64 = 2836;
const MASK: u64 = 123459876;

#[derive(Clone)]
struct Ran0State {
    x: u64,
}

impl Ran0State {
    fn new() -> Self {
        Ran0State { x: 0 }
    }

    fn get(&mut self) -> u64 {
        let x = self.x as i64;
        let h = x / Q;
        let t = A * (x - h * Q) - h * R;

        self.x = if t < 0 {
            (t + M) as u64
        } else {
            t as u64
        };

        self.x
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }

    fn set(&mut self, s: u64) -> Result<(), Ran0Error> {
        if s == MASK {
            return Err(Ran0Error::new("ran0 should not use seed == mask"));
        }

        self.x = s ^ MASK;
        Ok(())
    }
}

pub struct Ran0 {
    state: Ran0State,
}

impl Ran0 {
    pub fn new() -> Self {
        Ran0 {
            state: Ran0State::new(),
        }
    }

    pub fn with_seed(seed: u64) -> Result<Self, Ran0Error> {
        let mut state = Ran0State::new();
        state.set(seed)?;
        Ok(Ran0 { state })
    }

    pub fn get(&mut self) -> u64 {
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
    fn test_ran0_seed_error() {
        let rng = Ran0::with_seed(MASK);
        assert!(rng.is_err());
    }

    #[test]
    fn test_ran0_values() {
        let mut rng = Ran0::with_seed(1).unwrap();
        assert_eq!(rng.get(), 16807);
        assert_eq!(rng.get(), 282475249);
        assert_eq!(rng.get(), 1622650073);
    }
}