use std::num::Wrapping;

const MDIG: u32 = 32; // Machine digits in int
const M1: u32 = 2147483647; // 2^(MDIG-1) - 1
const M2: u32 = 65536; // 2^(MDIG/2)

#[derive(Clone)]
pub struct Uni32State {
    i: usize,
    j: usize,
    m: [u32; 17],
}

pub struct Uni32Rng {
    state: Uni32State,
}

impl Uni32Rng {
    pub fn new(seed: u32) -> Self {
        let mut state = Uni32State {
            i: 0,
            j: 0,
            m: [0; 17],
        };
        Self::set(&mut state, seed);
        Self { state }
    }

    pub fn set(state: &mut Uni32State, s: u32) {
        // For this routine, the seeding is very elaborate!
        // A flaw in this approach is that seeds 1,2 give exactly the
        // same random number sequence!
        let mut seed = if s < M1 { s } else { M1 }; // seed should be less than m1
        if seed % 2 == 0 {
            seed -= 1; // enforce seed be odd
        }

        let k0 = 9069 % M2;
        let k1 = 9069 / M2;
        let mut j0 = seed % M2;
        let mut j1 = seed / M2;

        for i in 0..17 {
            let seed_temp = j0 * k0;
            j1 = (seed_temp / M2 + j0 * k1 + j1 * k0) % (M2 / 2);
            j0 = seed_temp % M2;
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
        let k = if k.0 > 0 { k.0 } else { k.0 + M1 };
        self.state.m[j] = k;

        self.state.i = if i == 0 { 16 } else { i - 1 };
        self.state.j = if j == 0 { 16 } else { j - 1 };

        k
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut rng = Uni32Rng::new(305);
        let z1 = rng.get_double();
        let z2 = rng.get_double();
        let z3 = rng.get_double();
        
        assert!((z1 - 0.027832881).abs() < 1e-8);
        assert!((z2 - 0.56102176).abs() < 1e-8);
        assert!((z3 - 0.41456343).abs() < 1e-8);
    }
}