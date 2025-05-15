//! Gumbel distribution functions
//!
//! The Type I Gumbel distribution has the form,
//!
//! p(x) dx = a b exp(-(b exp(-ax) + ax)) dx
//!
//! and the Type II Gumbel distribution has the form,
//!
//! p(x) dx = b a x^-(a+1) exp(-b x^-a)) dx
//!

use rand::Rng;
use std::f64::consts::E;

/// Generates a random number from Type I Gumbel distribution
pub fn gumbel1<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
    let x = rng.gen_range(0.0..1.0);
    (b.ln() - (-x.ln()).ln()) / a
}

/// Calculates the PDF of Type I Gumbel distribution
pub fn gumbel1_pdf(x: f64, a: f64, b: f64) -> f64 {
    a * b * (-(b * (-a * x).exp() + a * x)).exp()
}

/// Generates a random number from Type II Gumbel distribution
pub fn gumbel2<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
    let x = rng.gen_range(0.0..1.0);
    (-b / x.ln()).powf(1.0 / a)
}

/// Calculates the PDF of Type II Gumbel distribution
pub fn gumbel2_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        b * a * x.powf(-(a + 1.0)) * (-b * x.powf(-a)).exp()
    }
}