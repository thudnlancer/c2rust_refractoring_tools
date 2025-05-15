use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct Taus113State {
    z1: u32,
    z2: u32,
    z3: u32,
    z4: u32,
}

impl Taus113State {
    pub fn new(seed: u32) -> Self {
        let mut state = Taus113State {
            z1: 0,
            z2: 0,
            z3: 0,
            z4: 0,
        };
        state.set(seed);
        state
    }

    pub fn set(&mut self, mut s: u32) {
        if s == 0 {
            s = 1;
        }

        self.z1 = 69069_u32.wrapping_mul(s) & 0xFFFF_FFFF;
        if self.z1 < 2 {
            self.z1 += 2;
        }

        self.z2 = 69069_u32.wrapping_mul(self.z1) & 0xFFFF_FFFF;
        if self.z2 < 8 {
            self.z2 += 8;
        }

        self.z3 = 69069_u32.wrapping_mul(self.z2) & 0xFFFF_FFFF;
        if self.z3 < 16 {
            self.z3 += 16;
        }

        self.z4 = 69069_u32.wrapping_mul(self.z3) & 0xFFFF_FFFF;
        if self.z4 < 128 {
            self.z4 += 128;
        }

        // Warm up the generator
        for _ in 0..10 {
            self.get();
        }
    }

    pub fn get(&mut self) -> u32 {
        let b1 = ((self.z1 << 6) & 0xFFFF_FFFF ^ self.z1) >> 13;
        self.z1 = ((self.z1 & 0xFFFF_FFFE) << 18) & 0xFFFF_FFFF ^ b1;

        let b2 = ((self.z2 << 2) & 0xFFFF_FFFF ^ self.z2) >> 27;
        self.z2 = ((self.z2 & 0xFFFF_FFF8) << 2) & 0xFFFF_FFFF ^ b2;

        let b3 = ((self.z3 << 13) & 0xFFFF_FFFF ^ self.z3) >> 21;
        self.z3 = ((self.z3 & 0xFFFF_FFF0) << 7) & 0xFFFF_FFFF ^ b3;

        let b4 = ((self.z4 << 3) & 0xFFFF_FFFF ^ self.z4) >> 12;
        self.z4 = ((self.z4 & 0xFFFF_FF80) << 13) & 0xFFFF_FFFF ^ b4;

        self.z1 ^ self.z2 ^ self.z3 ^ self.z4
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4_294_967_296.0
    }
}

pub struct GslRngTaus113 {
    state: Taus113State,
}

impl GslRngTaus113 {
    pub fn new(seed: u32) -> Self {
        GslRngTaus113 {
            state: Taus113State::new(seed),
        }
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }

    pub fn set(&mut self, seed: u32) {
        self.state.set(seed)
    }
}

pub fn gsl_rng_taus113() -> GslRngTaus113 {
    GslRngTaus113::new(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taus113() {
        let mut rng = GslRngTaus113::new(12345);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
    }
}