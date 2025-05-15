use std::ffi::CStr;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: String,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<Box<dyn FnMut(&mut [u8], u64)>>,
    pub get: Option<Box<dyn FnMut(&mut [u8]) -> u64>>,
    pub get_double: Option<Box<dyn FnMut(&mut [u8]) -> f64>>,
}

pub struct GslRng {
    pub type_: Box<GslRngType>,
    pub state: Vec<u8>,
}

impl GslRng {
    pub fn uniform(&mut self) -> f64 {
        self.type_
            .get_double
            .as_mut()
            .map_or(0.0, |f| f(&mut self.state))
    }

    pub fn ran_flat(&mut self, a: f64, b: f64) -> f64 {
        let u = self.uniform();
        a * (1.0 - u) + b * u
    }
}

pub fn ran_flat_pdf(x: f64, a: f64, b: f64) -> f64 {
    if (a..b).contains(&x) {
        1.0 / (b - a)
    } else {
        0.0
    }
}