use std::f64::consts::INFINITY;

pub fn gsl_cdf_gumbel2_Pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        0.0
    } else {
        (b / -P.ln()).powf(1.0 / a)
    }
}

pub fn gsl_cdf_gumbel2_Qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        0.0
    } else {
        (b / -(-Q).ln_1p()).powf(1.0 / a)
    }
}