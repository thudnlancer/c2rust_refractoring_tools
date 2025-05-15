use std::ffi::CStr;

#[derive(Clone, Copy)]
pub struct GslRngType {
    pub name: &'static CStr,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: Option<fn(&mut RngState, u64)>,
    pub get: Option<fn(&RngState) -> u64>,
    pub get_double: Option<fn(&RngState) -> f64>,
}

#[derive(Clone)]
pub struct GslRng {
    pub rng_type: &'static GslRngType,
    pub state: Box<RngState>,
}

pub type RngState = [u8];

impl GslRng {
    pub fn uniform(&self) -> f64 {
        self.rng_type
            .get_double
            .expect("non-null function pointer")(&self.state)
    }

    pub fn bernoulli(&self, p: f64) -> u32 {
        let u = self.uniform();
        if u < p {
            1
        } else {
            0
        }
    }
}

pub fn bernoulli_pdf(k: u32, p: f64) -> f64 {
    match k {
        0 => 1.0 - p,
        1 => p,
        _ => 0.0,
    }
}