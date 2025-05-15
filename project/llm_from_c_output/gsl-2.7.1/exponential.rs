//! Exponential distribution functions
//!
//! The exponential distribution has the form
//!
//! p(x) dx = exp(-x/mu) dx/mu
//!
//! for x = 0 ... +∞

use rand::Rng;
use std::f64;

/// Generates a random number from an exponential distribution with mean `mu`
pub fn exponential(rng: &mut impl Rng, mu: f64) -> f64 {
    let u = rng.gen_range(0.0..1.0);
    -mu * f64::ln(1.0 - u)
}

/// Calculates the probability density function of the exponential distribution at `x`
pub fn exponential_pdf(x: f64, mu: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        f64::exp(-x / mu) / mu
    }
}