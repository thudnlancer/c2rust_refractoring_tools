/* 
 * RANLUX random number generator translated from C to Rust
 * Original C code copyright (C) 1996-2000, 2007 James Theiler, Brian Gough
 * 
 * This Rust translation maintains the same functionality while following Rust's safety practices.
 */

const MASK_LO: u32 = 0x00ffffff;  // 2^24 - 1
const MASK_HI: u32 = !0x00ffffff;
const TWO24: u32 = 16777216;      // 2^24

#[derive(Clone)]
struct RanluxState {
    i: usize,
    j: usize,
    n: usize,
    skip: usize,
    carry: u32,
    u: [u32; 24],
}

impl RanluxState {
    fn new() -> Self {
        RanluxState {
            i: 0,
            j: 0,
            n: 0,
            skip: 0,
            carry: 0,
            u: [0; 24],
        }
    }

    fn increment_state(&mut self) -> u32 {
        let delta = (self.u[self.j]).wrapping_sub(self.u[self.i]).wrapping_sub(self.carry);
        let (new_delta, new_carry) = if delta & MASK_HI != 0 {
            (delta & MASK_LO, 1)
        } else {
            (delta, 0)
        };

        self.u[self.i] = new_delta;
        self.carry = new_carry;

        self.i = if self.i == 0 { 23 } else { self.i - 1 };
        self.j = if self.j == 0 { 23 } else { self.j - 1 };

        new_delta
    }

    fn get(&mut self) -> u32 {
        let r = self.increment_state();
        self.n += 1;

        if self.n == 24 {
            self.n = 0;
            for _ in 0..self.skip {
                self.increment_state();
            }
        }

        r
    }

    fn get_double(&mut self) -> f64 {
        f64::from(self.get()) / f64::from(TWO24)
    }

    fn set_lux(&mut self, mut s: u32, luxury: usize) {
        if s == 0 {
            s = 314159265; // default seed
        }

        let mut seed = s as i32;

        // Initialization algorithm of F. James
        for i in 0..24 {
            let k = seed / 53668;
            seed = 40014 * (seed - k * 53668) - k * 12211;
            if seed < 0 {
                seed += 2147483563;
            }
            self.u[i] = (seed as u32) % TWO24;
        }

        self.i = 23;
        self.j = 9;
        self.n = 0;
        self.skip = luxury - 24;
        self.carry = if self.u[23] & MASK_HI != 0 { 1 } else { 0 };
    }

    fn set(&mut self, s: u32) {
        self.set_lux(s, 223);
    }

    fn set389(&mut self, s: u32) {
        self.set_lux(s, 389);
    }
}

pub struct Ranlux {
    state: RanluxState,
}

impl Ranlux {
    pub fn new(seed: u32) -> Self {
        let mut state = RanluxState::new();
        state.set(seed);
        Ranlux { state }
    }

    pub fn new389(seed: u32) -> Self {
        let mut state = RanluxState::new();
        state.set389(seed);
        Ranlux { state }
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
    fn test_ranlux() {
        let mut rng = Ranlux::new(1);
        let val = rng.next();
        assert!(val <= MASK_LO);
    }

    #[test]
    fn test_ranlux389() {
        let mut rng = Ranlux::new389(1);
        let val = rng.next_double();
        assert!(val >= 0.0 && val < 1.0);
    }
}