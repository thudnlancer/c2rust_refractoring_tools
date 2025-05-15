use std::f64;

pub struct GslRng {
    // Assuming this is a safe wrapper around the C GSL RNG
    // Implementation details would depend on how you want to interface with GSL
}

impl GslRng {
    pub fn ran_gamma(&self, a: f64, b: f64) -> f64 {
        // Safe implementation using GSL bindings
        // Would need proper error handling
        unsafe { gsl_ran_gamma(self.as_ptr(), a, b) }
    }

    pub fn ran_poisson(&self, mu: f64) -> u32 {
        // Safe implementation using GSL bindings
        unsafe { gsl_ran_poisson(self.as_ptr(), mu) }
    }

    fn as_ptr(&self) -> *const gsl_rng {
        // Implementation would depend on how you wrap the C struct
        unimplemented!()
    }
}

pub fn negative_binomial(r: &GslRng, p: f64, n: f64) -> u32 {
    let x = r.ran_gamma(n, 1.0);
    r.ran_poisson(x * (1.0 - p) / p)
}

pub fn negative_binomial_pdf(k: u32, p: f64, n: f64) -> f64 {
    let f = gsl_sf_lngamma(f64::from(k) + n);
    let a = gsl_sf_lngamma(n);
    let b = gsl_sf_lngamma(f64::from(k) + 1.0);
    (f - a - b + n * p.ln() + f64::from(k) * (1.0 - p).ln_1p()).exp()
}

// These would be safe Rust bindings to the GSL functions
extern "C" {
    fn gsl_ran_gamma(r: *const gsl_rng, a: f64, b: f64) -> f64;
    fn gsl_ran_poisson(r: *const gsl_rng, mu: f64) -> u32;
    fn gsl_sf_lngamma(x: f64) -> f64;
}

// C types needed for FFI
#[repr(C)]
pub struct gsl_rng {
    _private: [u8; 0],
}