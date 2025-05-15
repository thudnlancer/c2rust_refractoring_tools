use std::f64;

pub struct GslRngType {
    pub name: String,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: Option<fn(&mut Vec<u8>, u64)>,
    pub get: Option<fn(&Vec<u8>) -> u64>,
    pub get_double: Option<fn(&Vec<u8>) -> f64>,
}

pub struct GslRng {
    pub type_: &'static GslRngType,
    pub state: Vec<u8>,
}

impl GslRng {
    fn uniform_pos(&self) -> f64 {
        loop {
            let x = (self.type_.get_double.unwrap())(&self.state);
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn geometric(&self, p: f64) -> u32 {
        let u = self.uniform_pos();
        if p == 1.0 {
            1
        } else {
            (f64::ln(u) / f64::ln(1.0 - p) + 1.0) as u32
        }
    }
}

pub fn geometric_pdf(k: u32, p: f64) -> f64 {
    match k {
        0 => 0.0,
        1 => p,
        _ => p * f64::powf(1.0 - p, k as f64 - 1.0),
    }
}