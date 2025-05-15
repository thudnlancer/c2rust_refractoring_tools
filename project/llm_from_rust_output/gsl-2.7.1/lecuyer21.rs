use std::ffi::CStr;
use std::os::raw::{c_char, c_void, c_ulong, c_long, c_double};

#[derive(Clone, Copy)]
pub struct RanState {
    x: c_ulong,
}

impl RanState {
    pub fn new(seed: c_ulong) -> Self {
        let mut state = RanState { x: 0 };
        state.set(seed);
        state
    }

    pub fn set(&mut self, seed: c_ulong) {
        const MODULUS: c_ulong = 2147483399;
        self.x = if seed % MODULUS == 0 { 1 } else { seed % MODULUS };
    }

    pub fn get(&mut self) -> c_ulong {
        const A: c_long = 40692;
        const M: c_long = 52774;
        const Q: c_long = 3791;
        const R: c_ulong = 2147483399;

        let mut y = self.x as c_long;
        let r = Q * (y / M);
        y = A * (y % M) - r;

        if y < 0 {
            y = (y as c_ulong).wrapping_add(R) as c_long;
        }

        self.x = y as c_ulong;
        self.x
    }

    pub fn get_double(&mut self) -> c_double {
        const MAX: c_double = 2147483399.0;
        self.get() as c_double / MAX
    }
}

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: c_ulong,
    pub min: c_ulong,
    pub size: usize,
}

pub const GSL_RNG_LECUYER21: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"lecuyer21\0") },
    max: 2147483399 - 1,
    min: 1,
    size: std::mem::size_of::<RanState>(),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut state = RanState::new(12345);
        let v1 = state.get();
        let v2 = state.get();
        assert_ne!(v1, v2);

        let d = state.get_double();
        assert!(d >= 0.0 && d < 1.0);
    }
}