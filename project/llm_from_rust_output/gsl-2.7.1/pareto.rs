use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_ulong, c_void};

pub type size_t = c_ulong;

#[derive(Debug, Clone)]
pub struct GslRngType {
    pub name: String,
    pub max: c_ulong,
    pub min: c_ulong,
    pub size: size_t,
    pub set: Option<Box<dyn FnMut(&mut [u8], c_ulong)>>,
    pub get: Option<Box<dyn FnMut(&mut [u8]) -> c_ulong>>,
    pub get_double: Option<Box<dyn FnMut(&mut [u8]) -> c_double>>,
}

#[derive(Debug, Clone)]
pub struct GslRng {
    pub rng_type: GslRngType,
    pub state: Vec<u8>,
}

impl GslRng {
    pub fn uniform_pos(&mut self) -> c_double {
        let get_double = self.rng_type.get_double.as_mut().expect("non-null function pointer");
        loop {
            let x = get_double(&mut self.state);
            if x != 0.0 {
                return x;
            }
        }
    }

    pub fn pareto(&mut self, a: c_double, b: c_double) -> c_double {
        let x = self.uniform_pos();
        let z = x.powf(-1.0 / a);
        b * z
    }
}

pub fn pareto_pdf(x: c_double, a: c_double, b: c_double) -> c_double {
    if x >= b {
        a / b / (x / b).powf(a + 1.0)
    } else {
        0.0
    }
}