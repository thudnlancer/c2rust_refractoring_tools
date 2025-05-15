//! Logistic distribution implementation
//!
//! The logistic distribution has the form,
//!
//! p(x) dx = (1/a) exp(-x/a) / (1 + exp(-x/a))^2 dx
//!
//! for -∞ < x < ∞

use rand::Rng;
use std::f64::consts::E;

/// Generates a random number from a logistic distribution
///
/// # Arguments
/// * `rng` - Random number generator implementing Rng trait
/// * `a` - Scale parameter of the logistic distribution
///
/// # Returns
/// A random number from the logistic distribution
pub fn logistic_random<R: Rng>(rng: &mut R, a: f64) -> f64 {
    let x = loop {
        let x = rng.gen_range(0.0..1.0);
        if x != 1.0 {
            break x;
        }
    };

    let z = (x / (1.0 - x)).ln();
    a * z
}

/// Calculates the probability density function of the logistic distribution
///
/// # Arguments
/// * `x` - Value at which to evaluate the PDF
/// * `a` - Scale parameter of the logistic distribution
///
/// # Returns
/// The probability density at x
pub fn logistic_pdf(x: f64, a: f64) -> f64 {
    let u = (-x.abs() / a).exp();
    u / (a.abs() * (1.0 + u).powi(2))
}