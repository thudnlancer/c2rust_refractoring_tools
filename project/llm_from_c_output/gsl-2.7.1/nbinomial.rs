//! Negative binomial distribution implementation translated from C to Rust
//!
//! Original C code copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 James Theiler, Brian Gough
//! Licensed under GNU General Public License version 3 or later

use rand::Rng;
use rand_distr::{Gamma, Poisson};
use std::f64::consts;

/// The negative binomial distribution has the form,
///
/// prob(k) =  Gamma(n + k)/(Gamma(n) Gamma(k + 1))  p^n (1-p)^k 
///
/// for k = 0, 1, ... . Note that n does not have to be an integer.
///
/// This is the Leger's algorithm (given in the answers in Knuth)
pub fn negative_binomial<R: Rng>(rng: &mut R, p: f64, n: f64) -> u32 {
    let gamma = Gamma::new(n, 1.0).expect("Invalid gamma distribution parameters");
    let x = rng.sample(gamma);
    let lambda = x * (1.0 - p) / p;
    let poisson = Poisson::new(lambda).expect("Invalid poisson distribution parameter");
    rng.sample(poisson)
}

/// Negative binomial probability density function
pub fn negative_binomial_pdf(k: u32, p: f64, n: f64) -> f64 {
    let f = ln_gamma(k as f64 + n);
    let a = ln_gamma(n);
    let b = ln_gamma(k as f64 + 1.0);
    
    (f - a - b + n * p.ln() + (k as f64) * (1.0 - p).ln()).exp()
}

/// Natural logarithm of the gamma function approximation
fn ln_gamma(x: f64) -> f64 {
    // Lanczos approximation implementation
    let x = x - 1.0;
    let tmp = x + 5.5;
    let log = (2.0 * consts::PI).ln() / 2.0 
        + (tmp.ln() * (x + 0.5)) 
        - tmp 
        + (1.000000000190015 
            + 76.18009172947146 / (x + 1.0) 
            - 86.50532032941677 / (x + 2.0) 
            + 24.01409824083091 / (x + 3.0) 
            - 1.231739572450155 / (x + 4.0) 
            + 0.1208650973866179e-2 / (x + 5.0) 
            - 0.5395239384953e-5 / (x + 6.0)).ln();
    log
}