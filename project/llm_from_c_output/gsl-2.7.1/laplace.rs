//! Laplace distribution implementation
//!
//! The two-sided exponential probability distribution is
//!
//! p(x) dx = (1/(2 a)) * exp(-|x/a|) dx
//!
//! for -∞ < x < ∞. It is also known as the Laplace distribution.

use rand::Rng;
use std::f64::consts::E;

/// Generates a random sample from the Laplace distribution
///
/// # Arguments
/// * `rng` - Random number generator implementing Rng trait
/// * `a` - Scale parameter of the distribution
///
/// # Returns
/// A random sample from the Laplace distribution
pub fn laplace<R: Rng>(rng: &mut R, a: f64) -> f64 {
    let mut u;
    loop {
        u = 2.0 * rng.gen::<f64>() - 1.0;
        if u != 0.0 {
            break;
        }
    }

    if u < 0.0 {
        a * (-u).ln()
    } else {
        -a * u.ln()
    }
}

/// Calculates the probability density function of the Laplace distribution
///
/// # Arguments
/// * `x` - Point at which to evaluate the PDF
/// * `a` - Scale parameter of the distribution
///
/// # Returns
/// The probability density at point x
pub fn laplace_pdf(x: f64, a: f64) -> f64 {
    (1.0 / (2.0 * a)) * (-x.abs() / a).exp()
}