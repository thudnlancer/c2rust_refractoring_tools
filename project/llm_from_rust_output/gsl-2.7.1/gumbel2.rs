use libm::{exp, expm1, pow};

pub fn gsl_cdf_gumbel2_P(x: f64, a: f64, b: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        let u = pow(x, a);
        exp(-b / u)
    }
}

pub fn gsl_cdf_gumbel2_Q(x: f64, a: f64, b: f64) -> f64 {
    if x == 0.0 {
        1.0
    } else {
        let u = pow(x, a);
        -expm1(-b / u)
    }
}