use std::ffi::CStr;
use std::mem;

#[derive(Clone, Copy)]
pub struct RanState {
    x0: u32,
    x1: u32,
}

impl RanState {
    fn new(seed: u32) -> Self {
        let seed = if seed % 0x7FFFFFFF == 0 { 1 } else { seed % 0x7FFFFFFF };
        RanState {
            x0: seed,
            x1: seed,
        }
    }

    fn schrage(a: u32, b: u32, m: u32) -> u32 {
        if a == 0 {
            return 0;
        }
        let q = m / a;
        let mut t = (2 * m).wrapping_sub((m % a).wrapping_mul(b / q));
        if t >= m {
            t -= m;
        }
        t = t.wrapping_add(a.wrapping_mul(b % q));
        if t >= m { t - m } else { t }
    }

    fn schrage_mult(a: u32, b: u32, m: u32, sqrtm: u32) -> u32 {
        let t0 = Self::schrage(sqrtm, b, m);
        let t1 = Self::schrage(a / sqrtm, t0, m);
        let t2 = Self::schrage(a % sqrtm, b, m);
        let t = t1.wrapping_add(t2);
        if t >= m { t - m } else { t }
    }

    fn next(&mut self) -> u32 {
        let xtmp = self.x1;
        self.x1 = Self::schrage_mult(271828183, self.x1, 0x7FFFFFFF, 46341)
            .wrapping_add(Self::schrage_mult(1833324378, self.x0, 0x7FFFFFFF, 46341));
        if self.x1 >= 0x7FFFFFFF {
            self.x1 -= 0x7FFFFFFF;
        }
        self.x0 = xtmp;
        self.x1
    }

    fn next_double(&mut self) -> f64 {
        self.next() as f64 / 2147483647.0
    }
}

pub struct GslRngKnuthran2 {
    state: RanState,
}

impl GslRngKnuthran2 {
    pub fn new(seed: u32) -> Self {
        GslRngKnuthran2 {
            state: RanState::new(seed),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state.next()
    }

    pub fn next_double(&mut self) -> f64 {
        self.state.next_double()
    }
}

pub fn gsl_rng_knuthran2_new(seed: u32) -> GslRngKnuthran2 {
    GslRngKnuthran2::new(seed)
}

#[repr(C)]
pub struct GslRngType {
    name: &'static str,
    max: u32,
    min: u32,
    size: usize,
}

pub static GSL_RNG_KNUTHRAN2: GslRngType = GslRngType {
    name: "knuthran2",
    max: 0x7FFFFFFF - 1,
    min: 0,
    size: mem::size_of::<RanState>(),
};