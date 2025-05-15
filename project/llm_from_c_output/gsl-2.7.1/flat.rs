//! Uniform distribution over the range [a, b)
//!
//! p(x) dx = 1/(b-a) dx   if  a <= x < b
//! .....   = 0            otherwise 

use rand::Rng;

/// Generates a random number from a uniform distribution in [a, b)
pub fn ran_flat(rng: &mut impl Rng, a: f64, b: f64) -> f64 {
    let u = rng.gen::<f64>();
    a * (1.0 - u) + b * u
}

/// Calculates the probability density function for the uniform distribution at x
pub fn ran_flat_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x >= a && x < b {
        1.0 / (b - a)
    } else {
        0.0
    }
}