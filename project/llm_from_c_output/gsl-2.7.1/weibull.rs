//! Weibull distribution implementation
//!
//! The Weibull distribution has the form,
//!
//! p(x) dx = (b/a) (x/a)^(b-1) exp(-(x/a)^b) dx

use rand::Rng;
use std::f64::consts::E;

/// Generates a random Weibull-distributed number
pub fn weibull(rng: &mut impl Rng, a: f64, b: f64) -> f64 {
    let x = rng.gen_range(0.0..1.0);
    let z = (-x.ln()).powf(1.0 / b);
    a * z
}

/// Calculates the Weibull probability density function
pub fn weibull_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x == 0.0 {
        if b == 1.0 {
            1.0 / a
        } else {
            0.0
        }
    } else if b == 1.0 {
        (-x / a).exp() / a
    } else {
        let ratio = x / a;
        (b / a) * (-ratio.powf(b) + (b - 1.0) * ratio.ln()).exp()
    }
}