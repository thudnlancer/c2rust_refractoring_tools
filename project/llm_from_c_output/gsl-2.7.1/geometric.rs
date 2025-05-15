//! Geometric distribution (bernoulli trial with probability p)
//!
//! prob(k) =  p (1 - p)^(k-1) for n = 1, 2, 3, ...
//!
//! It gives the distribution of "waiting times" for an event that
//! occurs with probability p.

use rand::Rng;
use std::f64::consts::E;

/// Generates a random geometric distribution value
pub fn geometric<R: Rng>(rng: &mut R, p: f64) -> u32 {
    let u = rng.gen_range(0.0..1.0);

    if p == 1.0 {
        1
    } else {
        (u.ln() / (1.0 - p).ln() + 1.0) as u32
    }
}

/// Calculates the probability density function for geometric distribution
pub fn geometric_pdf(k: u32, p: f64) -> f64 {
    if k == 0 {
        0.0
    } else if k == 1 {
        p
    } else {
        p * (1.0 - p).powi((k - 1) as i32)
    }
}