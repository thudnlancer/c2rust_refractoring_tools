use std::f64::consts;
use libm::{exp, log};

pub type size_t = usize;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslRngType {
    pub name: *const std::os::raw::c_char,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<extern "C" fn(*mut std::ffi::c_void, u64)>,
    pub get: Option<extern "C" fn(*mut std::ffi::c_void) -> u64>,
    pub get_double: Option<extern "C" fn(*mut std::ffi::c_void) -> f64>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslRng {
    pub type_: *const GslRngType,
    pub state: *mut std::ffi::c_void,
}

pub fn gsl_ran_erlang(r: &GslRng, a: f64, n: f64) -> f64 {
    // Assuming safe wrapper exists for gsl_ran_gamma
    unsafe { gsl_ran_gamma(r, n, a) }
}

pub fn gsl_ran_erlang_pdf(x: f64, a: f64, n: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        let lngamma = gsl_sf_lngamma(n);
        exp((n - 1.0) * log(x / a) - x / a - lngamma) / a
    }
}

// These would need to be provided by a safe wrapper crate
extern "C" {
    fn gsl_sf_lngamma(x: f64) -> f64;
    fn gsl_ran_gamma(r: *const GslRng, a: f64, b: f64) -> f64;
}