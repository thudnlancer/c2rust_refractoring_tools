use std::f64;

pub fn gsl_cdf_weibull_Pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        f64::INFINITY
    } else if P == 0.0 {
        0.0
    } else {
        a * (-(1.0 - P).ln_1p()).powf(1.0 / b)
    }
}

pub fn gsl_cdf_weibull_Qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        f64::INFINITY
    } else if Q == 1.0 {
        0.0
    } else {
        a * (-Q.ln()).powf(1.0 / b)
    }
}