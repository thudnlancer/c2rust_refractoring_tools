//! Rayleigh distribution functions
//!
//! The Rayleigh distribution has the form
//!
//! p(x) dx = (x / sigma^2) exp(-x^2/(2 sigma^2)) dx
//!
//! for x = 0 ... +infty

use rand::Rng;
use std::f64::consts::E;

/// Generates a random sample from a Rayleigh distribution
pub fn rayleigh(rng: &mut impl Rng, sigma: f64) -> f64 {
    let u = rng.gen_range(0.0..1.0);
    sigma * (-2.0 * u.ln()).sqrt()
}

/// Computes the probability density function of the Rayleigh distribution
pub fn rayleigh_pdf(x: f64, sigma: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        let u = x / sigma;
        (u / sigma) * (-u * u / 2.0).exp()
    }
}

/// The Rayleigh tail distribution has the form
///
/// p(x) dx = (x / sigma^2) exp((a^2 - x^2)/(2 sigma^2)) dx
///
/// for x = a ... +infty

/// Generates a random sample from a Rayleigh tail distribution
pub fn rayleigh_tail(rng: &mut impl Rng, a: f64, sigma: f64) -> f64 {
    let u = rng.gen_range(0.0..1.0);
    (a * a - 2.0 * sigma * sigma * u.ln()).sqrt()
}

/// Computes the probability density function of the Rayleigh tail distribution
pub fn rayleigh_tail_pdf(x: f64, a: f64, sigma: f64) -> f64 {
    if x < a {
        0.0
    } else {
        let u = x / sigma;
        let v = a / sigma;
        (u / sigma) * ((v + u) * (v - u) / 2.0).exp()
    }
}