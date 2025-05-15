//! Chi-squared distribution functions
//!
//! This module provides functions for working with the chi-squared distribution,
//! equivalent to the original C code from GNU Scientific Library.

use std::f64::consts::E;
use rand::Rng;
use statrs::function::gamma::ln_gamma;

/// Generates a random chi-squared distributed value
///
/// # Arguments
/// * `rng` - Random number generator implementing Rng trait
/// * `nu` - Degrees of freedom
pub fn chisq_random<R: Rng>(rng: &mut R, nu: f64) -> f64 {
    2.0 * gamma_random(rng, nu / 2.0, 1.0)
}

/// Computes the probability density function of the chi-squared distribution
///
/// # Arguments
/// * `x` - Value at which to evaluate the PDF
/// * `nu` - Degrees of freedom
pub fn chisq_pdf(x: f64, nu: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        if nu == 2.0 {
            E.powf(-x / 2.0) / 2.0
        } else {
            let lngamma = ln_gamma(nu / 2.0);
            E.powf((nu / 2.0 - 1.0) * (x / 2.0).ln() - x / 2.0 - lngamma) / 2.0
        }
    }
}

/// Helper function to generate gamma distributed random numbers
fn gamma_random<R: Rng>(rng: &mut R, shape: f64, scale: f64) -> f64 {
    use rand_distr::{Distribution, Gamma};
    let gamma = Gamma::new(shape, scale).unwrap();
    gamma.sample(rng)
}