use std::f64;

pub fn gsl_cdf_pareto_Pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        f64::INFINITY
    } else if P == 0.0 {
        b
    } else {
        b * (-log1p(-P) / a).exp()
    }
}

pub fn gsl_cdf_pareto_Qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        f64::INFINITY
    } else if Q == 1.0 {
        b
    } else {
        b * (-Q.ln() / a).exp()
    }
}

#[inline]
fn log1p(x: f64) -> f64 {
    (1.0 + x).ln()
}