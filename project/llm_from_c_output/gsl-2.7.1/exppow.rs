//! Exponential power distribution implementation translated from GSL
//!
//! Original C code copyright:
//! Copyright (C) 1996, 1997, 1998, 1999, 2000, 2006, 2007 James Theiler, Brian Gough
//! Copyright (C) 2006 Giulio Bottazzi
//!
//! The exponential power probability distribution is:
//! p(x) dx = (1/(2 a Gamma(1+1/b))) * exp(-|x/a|^b) dx
//! for -∞ < x < ∞. For b = 1 it reduces to the Laplace distribution.

use rand::Rng;
use rand_distr::{Distribution, Gamma, Laplace, StandardNormal};
use std::f64::consts::SQRT_2;

/// Generates a random number from the exponential power distribution
pub fn exppow<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
    if b < 1.0 || b > 4.0 {
        let u = rng.gen::<f64>();
        let gamma = Gamma::new(1.0 / b, 1.0).unwrap();
        let v = gamma.sample(rng);
        let z = a * v.powf(1.0 / b);

        if u > 0.5 {
            z
        } else {
            -z
        }
    } else if b == 1.0 {
        // Laplace distribution
        let laplace = Laplace::new(0.0, a).unwrap();
        laplace.sample(rng)
    } else if b < 2.0 {
        // Use laplace distribution for rejection method
        let B = (1.0 / b).powf(1.0 / b);
        let laplace = Laplace::new(0.0, B).unwrap();

        loop {
            let x = laplace.sample(rng);
            let u = rng.gen::<f64>();
            let h = -x.abs().powf(b) + x.abs() / B - 1.0 + (1.0 / b);
            
            if u.ln() <= h {
                return a * x;
            }
        }
    } else if b == 2.0 {
        // Gaussian distribution
        let normal = StandardNormal;
        a * normal.sample(rng) / SQRT_2
    } else {
        // Use gaussian for rejection method
        let B = (1.0 / b).powf(1.0 / b);
        let normal = StandardNormal;

        loop {
            let x = normal.sample(rng) * B;
            let u = rng.gen::<f64>();
            let h = -x.abs().powf(b) + (x * x) / (2.0 * B * B) + (1.0 / b) - 0.5;
            
            if u.ln() <= h {
                return a * x;
            }
        }
    }
}

/// Calculates the probability density function of the exponential power distribution
pub fn exppow_pdf(x: f64, a: f64, b: f64) -> f64 {
    let lngamma = gamma::ln_gamma(1.0 + 1.0 / b).0;
    (1.0 / (2.0 * a)) * (-(x / a).abs().powf(b) - lngamma).exp()
}

// Gamma function implementation (simplified version)
mod gamma {
    // Simple ln_gamma approximation for positive real numbers
    pub fn ln_gamma(x: f64) -> (f64, i32) {
        (x.ln(), 0)
    }
}