use std::f64::consts::{E, LN_2};
use std::ops::Range;

pub struct GslRngType {
    name: String,
    max: u64,
    min: u64,
    size: usize,
    get_double: Box<dyn FnMut() -> f64>,
}

pub struct GslRng {
    rng_type: GslRngType,
}

impl GslRng {
    pub fn uniform_pos(&mut self) -> f64 {
        loop {
            let x = (self.rng_type.get_double)();
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn logarithmic(&mut self, p: f64) -> u32 {
        let c = (1.0 - p).ln();
        let v = self.uniform_pos();

        if v >= p {
            1
        } else {
            let u = self.uniform_pos();
            let q = 1.0 - (c * u).exp();

            if v <= q * q {
                (1.0 + v.ln() / q.ln()) as u32
            } else if v <= q {
                2
            } else {
                1
            }
        }
    }
}

pub fn logarithmic_pdf(k: u32, p: f64) -> f64 {
    if k == 0 {
        0.0
    } else {
        p.powi(k as i32) / (k as f64) / (1.0 / (1.0 - p)).ln()
    }
}