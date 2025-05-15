use std::f64::consts::E;

pub fn gsl_cdf_gumbel1_P(x: f64, a: f64, b: f64) -> f64 {
    let u = a * x - b.ln();
    E.powf(-E.powf(-u))
}

pub fn gsl_cdf_gumbel1_Q(x: f64, a: f64, b: f64) -> f64 {
    let u = a * x - b.ln();
    let p = E.powf(-E.powf(-u));
    if p < 0.5 {
        1.0 - p
    } else {
        -(-E.powf(-u)).exp_m1()
    }
}