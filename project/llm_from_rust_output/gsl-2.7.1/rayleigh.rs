use std::f64::consts::E;
use std::ops::Deref;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: String,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<Box<dyn Fn(&mut [u8], u64)>>,
    pub get: Option<Box<dyn Fn(&[u8]) -> u64>>,
    pub get_double: Option<Box<dyn Fn(&[u8]) -> f64>>,
}

#[derive(Clone)]
pub struct GslRng {
    pub type_: Box<GslRngType>,
    pub state: Vec<u8>,
}

impl GslRng {
    fn uniform_pos(&self) -> f64 {
        let get_double = self.type_.get_double.as_ref().expect("non-null function pointer");
        loop {
            let x = get_double(&self.state);
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn ran_rayleigh(&self, sigma: f64) -> f64 {
        let u = self.uniform_pos();
        sigma * (-2.0 * u.ln()).sqrt()
    }

    pub fn ran_rayleigh_tail(&self, a: f64, sigma: f64) -> f64 {
        let u = self.uniform_pos();
        (a * a - 2.0 * sigma * sigma * u.ln()).sqrt()
    }
}

pub fn ran_rayleigh_pdf(x: f64, sigma: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        let u = x / sigma;
        u / sigma * (-u * u / 2.0).exp()
    }
}

pub fn ran_rayleigh_tail_pdf(x: f64, a: f64, sigma: f64) -> f64 {
    if x < a {
        0.0
    } else {
        let u = x / sigma;
        let v = a / sigma;
        u / sigma * ((v + u) * (v - u) / 2.0).exp()
    }
}