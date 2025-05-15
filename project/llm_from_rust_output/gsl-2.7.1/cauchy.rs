use std::f64::consts::PI;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: String,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut Vec<u8>, u64)>,
    pub get: Option<fn(&Vec<u8>) -> u64>,
    pub get_double: Option<fn(&Vec<u8>) -> f64>,
}

#[derive(Clone)]
pub struct GslRng {
    pub type_: Box<GslRngType>,
    pub state: Vec<u8>,
}

impl GslRng {
    pub fn uniform(&self) -> f64 {
        (self.type_.get_double).expect("non-null function pointer")(&self.state)
    }

    pub fn cauchy(&self, a: f64) -> f64 {
        loop {
            let u = self.uniform();
            if u != 0.5 {
                return a * (PI * u).tan();
            }
        }
    }
}

pub fn cauchy_pdf(x: f64, a: f64) -> f64 {
    let u = x / a;
    1.0 / (PI * a) / (1.0 + u * u)
}