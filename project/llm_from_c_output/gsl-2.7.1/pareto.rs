//! Pareto distribution
//!
//! The Pareto distribution has the form,
//!
//! p(x) dx = (a/b) / (x/b)^(a+1) dx     for x >= b

use rand::Rng;
use std::f64;

/// Generates a random value from the Pareto distribution
pub fn pareto(rng: &mut impl Rng, a: f64, b: f64) -> f64 {
    let x = rng.gen_range(0.0..1.0);
    let z = x.powf(-1.0 / a);
    b * z
}

/// Calculates the probability density function of the Pareto distribution
pub fn pareto_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x >= b {
        (a / b) / (x / b).powf(a + 1.0)
    } else {
        0.0
    }
}