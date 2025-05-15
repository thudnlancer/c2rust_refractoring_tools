use std::num::Wrapping;

const MDIG: u32 = 16;    // Machine digits in int
const M1: u32 = 32767;   // 2^(MDIG-1) - 1
const M2: u32 = 256;     // 2^(MDIG/2)

#[derive(Debug, Clone)]
pub struct UniState {
    i: usize,
    j: usize,
    m: [u32; 17],
}

pub struct UniRng {
    state: UniState,
}

impl UniRng {
    pub fn new(seed: u32) -> Self {
        let mut state = UniState {
            i: 0,
            j: 0,
            m: [0; 17],
        };
        Self::set(&mut state, seed);
        Self { state }
    }

    pub fn set(state: &mut UniState, mut s: u32) {
        // For this routine, the seeding is very elaborate!
        // A flaw in this approach is that seeds 1,2 give exactly the
        // same random number sequence!
        
        s = 2 * s + 1;        // enforce seed be odd
        let seed = if s < M1 { s } else { M1 }; // seed should be less than M1

        let k0 = 9069 % M2;
        let k1 = 9069 / M2;
        let mut j0 = seed % M2;
        let mut j1 = seed / M2;

        for i in 0..17 {
            let seed = j0 * k0;
            j1 = (seed / M2 + j0 * k1 + j1 * k0) % (M2 / 2);
            j0 = seed % M2;
            state.m[i] = j0 + M2 * j1;
        }
        state.i = 4;
        state.j = 16;
    }

    pub fn get(&mut self) -> u32 {
        let i = self.state.i;
        let j = self.state.j;

        // important k not be unsigned
        let k = Wrapping(self.state.m[i]) - Wrapping(self.state.m[j]);
        let mut k = k.0 as i32;

        if k < 0 {
            k += M1 as i32;
        }
        self.state.m[j] = k as u32;

        if i == 0 {
            self.state.i = 16;
        } else {
            self.state.i -= 1;
        }

        if j == 0 {
            self.state.j = 16;
        } else {
            self.state.j -= 1;
        }

        k as u32
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 32767.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut rng = UniRng::new(152);
        let z1 = rng.get_double();
        let z2 = rng.get_double();
        let z3 = rng.get_double();
        
        // Compare with expected values from original C code comments
        assert!((z1 - 0.027832881).abs() < 1e-8);
        assert!((z2 - 0.56102176).abs() < 1e-8);
        assert!((z3 - 0.41456343).abs() < 1e-8);
    }
}