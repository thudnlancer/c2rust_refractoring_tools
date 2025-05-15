//! Implementation of Knuth's subtractive generator with Numerical Recipes' ran3 parameters.
//! It is a subtractive lagged Fibonacci generator.

use std::num::Wrapping;

const M_BIG: u32 = 1_000_000_000;
const M_SEED: u32 = 161_803_398;

/// State for the ran3 random number generator
#[derive(Debug, Clone)]
pub struct Ran3State {
    x: usize,
    y: usize,
    buffer: [u32; 56],
}

impl Ran3State {
    /// Creates a new Ran3State initialized with the given seed
    pub fn new(seed: u32) -> Self {
        let mut state = Ran3State {
            x: 0,
            y: 31,
            buffer: [0; 56],
        };
        state.set(seed);
        state
    }

    /// Sets the state using the given seed
    pub fn set(&mut self, mut s: u32) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        let mut j = (M_SEED as i64 - s as i64) % M_BIG as i64;
        if j < 0 {
            j += M_BIG as i64;
        }

        self.buffer[0] = 0; // unused but initialized for consistency
        self.buffer[55] = j as u32;

        let mut k = 1u32;
        for i in 1..55 {
            let n = (21 * i) % 55;
            self.buffer[n] = k;
            let diff = (j - k as i64) as i64;
            k = if diff < 0 {
                (diff + M_BIG as i64) as u32
            } else {
                diff as u32
            };
            j = self.buffer[n] as i64;
        }

        for _ in 0..4 {
            for i in 1..56 {
                let idx = 1 + (i + 30) % 55;
                let t = (self.buffer[i] as i64 - self.buffer[idx] as i64) as i64;
                self.buffer[i] = if t < 0 {
                    (t + M_BIG as i64) as u32
                } else {
                    t as u32
                };
            }
        }

        self.x = 0;
        self.y = 31;
    }

    /// Generates the next random number in the sequence
    pub fn get(&mut self) -> u32 {
        self.x += 1;
        if self.x == 56 {
            self.x = 1;
        }

        self.y += 1;
        if self.y == 56 {
            self.y = 1;
        }

        let j = (Wrapping(self.buffer[self.x]) - Wrapping(self.buffer[self.y])).0;
        let result = if j < 0 {
            j + M_BIG as i32
        } else {
            j
        };

        self.buffer[self.x] = result as u32;
        result as u32
    }

    /// Generates the next random number in the sequence as a double in [0,1)
    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / M_BIG as f64
    }
}

/// Type implementing the ran3 random number generator
#[derive(Debug)]
pub struct Ran3 {
    state: Ran3State,
}

impl Ran3 {
    /// Creates a new ran3 generator with the given seed
    pub fn new(seed: u32) -> Self {
        Ran3 {
            state: Ran3State::new(seed),
        }
    }

    /// Generates the next random number in the sequence
    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    /// Generates the next random number in the sequence as a double in [0,1)
    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ran3() {
        let mut rng = Ran3::new(12345);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
        
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval < 1.0);
    }
}