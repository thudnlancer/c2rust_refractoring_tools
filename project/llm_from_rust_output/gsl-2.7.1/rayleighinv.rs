use std::f64::consts::SQRT_2;
use std::f64::INFINITY;

pub fn gsl_cdf_rayleigh_Pinv(P: f64, sigma: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        0.0
    } else {
        sigma * SQRT_2 * (-(1.0 - P).ln_1p()).sqrt()
    }
}

pub fn gsl_cdf_rayleigh_Qinv(Q: f64, sigma: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        0.0
    } else {
        sigma * SQRT_2 * (-Q.ln()).sqrt()
    }
}