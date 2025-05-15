//! Beta distribution implementation translated from GSL
//!
//! Original C code copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 James Theiler, Brian Gough
//! under GNU General Public License

use rand::Rng;
use std::f64::consts::E;
use std::f64;

/// Generates a random number from a beta distribution
pub fn beta<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
    if a <= 1.0 && b <= 1.0 {
        loop {
            let u = rng.gen_range(0.0..1.0);
            let v = rng.gen_range(0.0..1.0);
            let x = u.powf(1.0 / a);
            let y = v.powf(1.0 / b);
            
            if x + y <= 1.0 {
                if x + y > 0.0 {
                    return x / (x + y);
                } else {
                    let log_u = u.ln();
                    let log_v = v.ln();
                    let log_x = log_u / a;
                    let log_y = log_v / b;
                    let log_m = log_x.max(log_y);
                    let log_x = log_x - log_m;
                    let log_y = log_y - log_m;
                    return (log_x - (log_x.exp() + log_y.exp()).ln()).exp();
                }
            }
        }
    } else {
        let x1 = gamma(rng, a, 1.0);
        let x2 = gamma(rng, b, 1.0);
        x1 / (x1 + x2)
    }
}

/// Calculates the probability density function of the beta distribution
pub fn beta_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x < 0.0 || x > 1.0 {
        0.0
    } else {
        let gab = ln_gamma(a + b);
        let ga = ln_gamma(a);
        let gb = ln_gamma(b);
        
        if x == 0.0 || x == 1.0 {
            if a > 1.0 && b > 1.0 {
                0.0
            } else {
                (gab - ga - gb).exp() * x.powf(a - 1.0) * (1.0 - x).powf(b - 1.0)
            }
        } else {
            (gab - ga - gb + x.ln() * (a - 1.0) + (-x).ln_1p() * (b - 1.0)).exp()
        }
    }
}

// Helper functions that would need to be implemented or imported
// These are stand-ins for the GSL functions used in the original code

/// Natural logarithm of the gamma function
fn ln_gamma(x: f64) -> f64 {
    // Implementation or use a crate like statrs
    x.ln_gamma().0
}

/// Generates a random number from a gamma distribution
fn gamma<R: Rng>(rng: &mut R, a: f64, _theta: f64) -> f64 {
    // Simplified implementation - would need proper gamma distribution
    // For now using rand_distr crate's Gamma distribution would be better
    let scale = 1.0;
    let gamma = rand_distr::Gamma::new(a, scale).unwrap();
    rng.sample(gamma)
}