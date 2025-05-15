use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct Uni32State {
    i: i32,
    j: i32,
    m: [u32; 17],
}

const M1: u32 = 2147483647;
const M2: u32 = 65536;

impl Uni32State {
    fn new() -> Self {
        Self {
            i: 0,
            j: 0,
            m: [0; 17],
        }
    }

    fn get(&mut self) -> u32 {
        let i = self.i as usize;
        let j = self.j as usize;
        
        let mut k = self.m[i].wrapping_sub(self.m[j]) as i64;
        if k < 0 {
            k += M1 as i64;
        }
        
        self.m[j] = k as u32;
        
        self.i = if self.i == 0 { 16 } else { self.i - 1 };
        self.j = if self.j == 0 { 16 } else { self.j - 1 };
        
        k as u32
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / M1 as f64
    }

    fn set(&mut self, mut seed: u32) {
        if seed >= M1 {
            seed = M1;
        }
        if seed % 2 == 0 {
            seed -= 1;
        }

        let k0 = (9069 % M2) as i64;
        let k1 = (9069 / M2) as i64;
        let mut j0 = (seed % M2) as i64;
        let mut j1 = (seed / M2) as i64;

        for i in 0..17 {
            let seed = j0 * k0;
            j1 = ((seed as u32 / M2) as i64 + j0 * k1 + j1 * k0) % (M2 as i64 / 2);
            j0 = (seed as u32 % M2) as i64;
            self.m[i] = (j0 as u32) + M2 * (j1 as u32);
        }

        self.i = 4;
        self.j = 16;
    }
}

pub struct GslRngUni32 {
    state: Uni32State,
}

impl GslRngUni32 {
    pub fn new() -> Self {
        Self {
            state: Uni32State::new(),
        }
    }

    pub fn set(&mut self, seed: u32) {
        self.state.set(seed);
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

pub static GSL_RNG_UNI32: GslRngUni32 = GslRngUni32::new();