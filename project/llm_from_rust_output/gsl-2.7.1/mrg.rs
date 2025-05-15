use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: fn(&mut MrgState, u64),
    pub get: fn(&mut MrgState) -> u64,
    pub get_double: fn(&mut MrgState) -> f64,
}

#[derive(Clone, Copy)]
pub struct MrgState {
    x1: i64,
    x2: i64,
    x3: i64,
    x4: i64,
    x5: i64,
}

const M: i64 = 2147483647;
const A1: i64 = 107374182;
const Q1: i64 = 20;
const R1: i64 = 7;
const A5: i64 = 104480;
const Q5: i64 = 20554;
const R5: i64 = 1727;

impl MrgState {
    fn get(&mut self) -> u64 {
        let h5 = self.x5 / Q5;
        let mut p5 = A5 * (self.x5 - h5 * Q5) - h5 * R5;
        if p5 > 0 {
            p5 -= M;
        }

        let h1 = self.x1 / Q1;
        let mut p1 = A1 * (self.x1 - h1 * Q1) - h1 * R1;
        if p1 < 0 {
            p1 += M;
        }

        self.x5 = self.x4;
        self.x4 = self.x3;
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = p1 + p5;

        if self.x1 < 0 {
            self.x1 += M;
        }

        self.x1 as u64
    }

    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }

    fn set(&mut self, mut s: u64) {
        if s == 0 {
            s = 1;
        }

        s = (69069u64).wrapping_mul(s) & 0xffffffff;
        self.x1 = (s % M as u64) as i64;
        
        s = (69069u64).wrapping_mul(s) & 0xffffffff;
        self.x2 = (s % M as u64) as i64;
        
        s = (69069u64).wrapping_mul(s) & 0xffffffff;
        self.x3 = (s % M as u64) as i64;
        
        s = (69069u64).wrapping_mul(s) & 0xffffffff;
        self.x4 = (s % M as u64) as i64;
        
        s = (69069u64).wrapping_mul(s) & 0xffffffff;
        self.x5 = (s % M as u64) as i64;

        for _ in 0..6 {
            self.get();
        }
    }
}

pub static GSL_RNG_MRG: GslRngType = GslRngType {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"mrg\0") },
    max: 2147483646,
    min: 0,
    size: std::mem::size_of::<MrgState>(),
    set: MrgState::set,
    get: MrgState::get,
    get_double: MrgState::get_double,
};