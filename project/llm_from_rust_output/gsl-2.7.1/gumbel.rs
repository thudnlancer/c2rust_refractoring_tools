use std::f64::consts::E;
use std::f64;

#[derive(Clone)]
pub struct GslRng {
    rng_type: GslRngType,
    state: Vec<u8>,
}

#[derive(Clone)]
pub struct GslRngType {
    name: String,
    max: u64,
    min: u64,
    get_double: fn(&[u8]) -> f64,
}

impl GslRng {
    pub fn uniform_pos(&self) -> f64 {
        let mut x;
        loop {
            x = (self.rng_type.get_double)(&self.state);
            if x != 0.0 {
                break;
            }
        }
        x
    }

    pub fn gumbel1(&self, a: f64, b: f64) -> f64 {
        let x = self.uniform_pos();
        (b.ln() - (-x.ln()).ln()) / a
    }

    pub fn gumbel1_pdf(x: f64, a: f64, b: f64) -> f64 {
        a * b * f64::exp(-(b * f64::exp(-a * x) + a * x)
    }

    pub fn gumbel2(&self, a: f64, b: f64) -> f64 {
        let x = self.uniform_pos();
        (-b / x.ln()).powf(1.0 / a)
    }

    pub fn gumbel2_pdf(x: f64, a: f64, b: f64) -> f64 {
        if x <= 0.0 {
            0.0
        } else {
            b * a * x.powf(-(a + 1.0)) * f64::exp(-b * x.powf(-a))
        }
    }
}