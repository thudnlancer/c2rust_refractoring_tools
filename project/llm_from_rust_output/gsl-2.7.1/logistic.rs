use std::f64::consts::E;
use std::ops::Deref;

pub type SizeT = usize;

#[derive(Debug, Clone)]
pub struct GslRngType {
    pub name: String,
    pub max: u64,
    pub min: u64,
    pub size: SizeT,
    pub set: Option<Box<dyn FnMut(&mut [u8], u64)>>,
    pub get: Option<Box<dyn FnMut(&mut [u8]) -> u64>>,
    pub get_double: Option<Box<dyn FnMut(&mut [u8]) -> f64>>,
}

#[derive(Debug, Clone)]
pub struct GslRng {
    pub rng_type: GslRngType,
    pub state: Vec<u8>,
}

impl GslRng {
    fn uniform_pos(&mut self) -> f64 {
        let get_double = self.rng_type.get_double.as_mut().expect("non-null function pointer");
        loop {
            let x = get_double(&mut self.state);
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn logistic(&mut self, a: f64) -> f64 {
        loop {
            let x = self.uniform_pos();
            if x != 1.0 {
                let z = (x / (1.0 - x)).ln();
                return a * z;
            }
        }
    }
}

pub fn logistic_pdf(x: f64, a: f64) -> f64 {
    let u = E.powf(-x.abs() / a);
    u / (a.abs() * (1.0 + u).powi(2))
}