//! Cauchy (Lorentzian) probability distribution functions
//! 
//! The Cauchy probability distribution is:
//! 
//! p(x) dx = (1/(pi a)) (1 + (x/a)^2)^(-1) dx
//! 
//! It is also known as the Lorentzian probability distribution

use std::f64::consts::PI;
use rand::Rng;

/// Generates a random number from a Cauchy distribution
/// 
/// # Arguments
/// * `rng` - Random number generator implementing Rng trait
/// * `a` - Scale parameter of the distribution
/// 
/// # Returns
/// A random number from the Cauchy distribution
pub fn cauchy(rng: &mut impl Rng, a: f64) -> f64 {
    let mut u;
    loop {
        u = rng.gen::<f64>();
        if u != 0.5 {
            break;
        }
    }
    a * (PI * u).tan()
}

/// Calculates the probability density function of the Cauchy distribution
/// 
/// # Arguments
/// * `x` - Value at which to evaluate the PDF
/// * `a` - Scale parameter of the distribution
/// 
/// # Returns
/// The probability density at x
pub fn cauchy_pdf(x: f64, a: f64) -> f64 {
    let u = x / a;
    (1.0 / (PI * a)) / (1.0 + u * u)
}