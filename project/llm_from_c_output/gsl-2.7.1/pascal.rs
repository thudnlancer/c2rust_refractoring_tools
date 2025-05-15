//! Pascal distribution functions translated from GSL
//!
//! The Pascal distribution is a negative binomial with valued integer n
//!
//! prob(k) = (n - 1 + k)!/(n!(k - 1)!) * p^n (1-p)^k for k = 0, 1, ..., n

use rand::Rng;
use statrs::distribution::{NegativeBinomial, Discrete};

/// Generates a random value from the Pascal distribution
///
/// This is a separate interface for the Pascal distribution so that
/// it can be optimized differently from the negative binomial in
/// future.
///
/// e.g. if n < 10 it might be faster to generate the Pascal
/// distributions as the sum of geometric variates directly.
pub fn ran_pascal<R: Rng>(rng: &mut R, p: f64, n: u32) -> u32 {
    let negative_binomial = NegativeBinomial::new(n as f64, p).unwrap();
    negative_binomial.sample(rng) as u32
}

/// Calculates the probability density function for the Pascal distribution
pub fn ran_pascal_pdf(k: u32, p: f64, n: u32) -> f64 {
    let negative_binomial = NegativeBinomial::new(n as f64, p).unwrap();
    negative_binomial.pmf(k as u64)
}