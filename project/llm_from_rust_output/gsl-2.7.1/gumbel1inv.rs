use std::f64::consts::{INFINITY, NEG_INFINITY};

pub fn gsl_cdf_gumbel1_Pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        NEG_INFINITY
    } else {
        (-b / P.ln()).ln() / a
    }
}

pub fn gsl_cdf_gumbel1_Qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        NEG_INFINITY
    } else {
        (-b / (1.0 - Q).ln_1p()).ln() / a
    }
}