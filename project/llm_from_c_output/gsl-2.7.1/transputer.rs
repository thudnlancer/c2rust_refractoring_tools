/* rng/transputer.rs
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

/// This is the INMOS Transputer Development System generator. The sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// with a = 1664525 and m = 2^32. The seed specifies the initial
/// value, x_1.
///
/// The theoretical value of x_{10001} is 1244127297.
///
/// The period of this generator is 2^30.
#[derive(Debug, Clone)]
pub struct TransputerState {
    x: u32,
}

impl TransputerState {
    /// Creates a new TransputerState with the given seed
    pub fn new(seed: u32) -> Self {
        let s = if seed == 0 { 1 } else { seed }; // default seed is 1
        TransputerState { x: s }
    }

    /// Generates the next random number in the sequence
    pub fn get(&mut self) -> u32 {
        self.x = self.x.wrapping_mul(1664525);
        self.x
    }

    /// Generates the next random double in the range [0,1)
    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

/// Transputer RNG type implementing the GSL RNG interface
pub struct TransputerRng {
    state: TransputerState,
}

impl TransputerRng {
    pub fn new(seed: u32) -> Self {
        TransputerRng {
            state: TransputerState::new(seed),
        }
    }

    pub fn get(&mut self) -> u32 {
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
    fn test_transputer() {
        let mut rng = TransputerRng::new(1);
        let mut x = 0;
        for _ in 0..10000 {
            x = rng.get();
        }
        assert_eq!(x, 1244127297);
    }
}