//! Bernoulli distribution
//!
//! The bernoulli distribution has the form:
//! prob(0) = 1-p, prob(1) = p

use rand::Rng;

/// Generates a Bernoulli-distributed random number
///
/// # Arguments
/// * `rng` - Random number generator implementing Rng trait
/// * `p` - Probability of success (returning 1)
///
/// # Returns
/// 1 with probability p, 0 with probability 1-p
pub fn bernoulli<R: Rng>(rng: &mut R, p: f64) -> u32 {
    let u = rng.gen::<f64>();
    
    if u < p {
        1
    } else {
        0
    }
}

/// Calculates the probability mass function of the Bernoulli distribution
///
/// # Arguments
/// * `k` - Outcome (0 or 1)
/// * `p` - Probability of success (k=1)
///
/// # Returns
/// Probability of outcome k given p
pub fn bernoulli_pdf(k: u32, p: f64) -> f64 {
    match k {
        0 => 1.0 - p,
        1 => p,
        _ => 0.0,
    }
}