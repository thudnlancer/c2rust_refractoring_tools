//! Erlang distribution functions
//!
//! This module provides functions for working with the Erlang distribution,
//! which is a special case of the Gamma distribution.

use std::f64::consts;
use rand::Rng;
use statrs::function::gamma::ln_gamma;

/// Generates a random number from an Erlang distribution
///
/// The Erlang distribution is the sum of `n` samples from an exponential distribution
/// with scale parameter `a`.
///
/// # Arguments
/// * `rng` - Random number generator
/// * `a` - Scale parameter
/// * `n` - Shape parameter (must be positive integer in original GSL implementation)
pub fn erlang(rng: &mut impl Rng, a: f64, n: f64) -> f64 {
    gamma(rng, n, a)
}

/// Calculates the probability density function (PDF) of the Erlang distribution
///
/// # Arguments
/// * `x` - Value at which to evaluate the PDF
/// * `a` - Scale parameter
/// * `n` - Shape parameter
///
/// # Returns
/// PDF value at `x`
pub fn erlang_pdf(x: f64, a: f64, n: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        let lngamma = ln_gamma(n);
        ((n - 1.0) * (x / a).ln() - x / a - lngamma).exp() / a
    }
}

/// Helper function to generate Gamma-distributed random numbers
fn gamma(rng: &mut impl Rng, shape: f64, scale: f64) -> f64 {
    use rand_distr::{Distribution, Gamma};
    
    let gamma = Gamma::new(shape, scale).unwrap();
    gamma.sample(rng)
}