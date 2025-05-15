use libm::{exp, log};
use gsl_sys::{gsl_sf_lngamma, gsl_ran_gamma, gsl_rng, gsl_rng_type};

pub fn gsl_ran_chisq(r: &gsl_rng, nu: f64) -> f64 {
    2.0 * unsafe { gsl_ran_gamma(r, nu / 2.0, 1.0) }
}

pub fn gsl_ran_chisq_pdf(x: f64, nu: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if nu == 2.0 {
        exp(-x / 2.0) / 2.0
    } else {
        let lngamma = unsafe { gsl_sf_lngamma(nu / 2.0) };
        exp(
            (nu / 2.0 - 1.0) * log(x / 2.0) - x / 2.0 - lngamma,
        ) / 2.0
    }
}