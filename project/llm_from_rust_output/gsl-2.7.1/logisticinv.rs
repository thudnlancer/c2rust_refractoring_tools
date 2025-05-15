use std::f64;

pub fn gsl_cdf_logistic_Pinv(P: f64, a: f64) -> f64 {
    if P == 1.0 {
        f64::INFINITY
    } else if P == 0.0 {
        f64::NEG_INFINITY
    } else {
        a * (P / (1.0 - P)).ln()
    }
}

pub fn gsl_cdf_logistic_Qinv(Q: f64, a: f64) -> f64 {
    if Q == 0.0 {
        f64::INFINITY
    } else if Q == 1.0 {
        f64::NEG_INFINITY
    } else {
        a * ((1.0 - Q) / Q).ln()
    }
}