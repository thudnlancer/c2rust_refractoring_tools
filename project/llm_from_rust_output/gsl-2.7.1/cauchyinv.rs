use std::f64::consts::PI;
use std::f64::{INFINITY, NEG_INFINITY};

pub fn gsl_cdf_cauchy_Pinv(P: f64, a: f64) -> f64 {
    if P == 1.0 {
        return INFINITY;
    } else if P == 0.0 {
        return NEG_INFINITY;
    }

    if P > 0.5 {
        a * (PI * (P - 0.5)).tan()
    } else {
        -a / (PI * P).tan()
    }
}

pub fn gsl_cdf_cauchy_Qinv(Q: f64, a: f64) -> f64 {
    if Q == 0.0 {
        return INFINITY;
    } else if Q == 1.0 {
        return NEG_INFINITY;
    }

    if Q > 0.5 {
        a * (PI * (0.5 - Q)).tan()
    } else {
        a / (PI * Q).tan()
    }
}