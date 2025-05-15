//! Inverse of the lognormal cumulative distribution function.
//!
//! This module provides functions to compute the inverse of the lognormal
//! cumulative distribution function (CDF) for given probabilities.

use std::f64::{INFINITY, EPSILON};

/// Computes the inverse of the lognormal CDF for a given probability `P`,
/// location parameter `zeta`, and scale parameter `sigma`.
///
/// # Arguments
/// * `P` - The probability value (0.0 <= P <= 1.0)
/// * `zeta` - The location parameter
/// * `sigma` - The scale parameter (must be positive)
///
/// # Returns
/// The inverse CDF value, or infinity if P == 1.0, or 0.0 if P == 0.0
pub fn lognormal_pinv(P: f64, zeta: f64, sigma: f64) -> f64 {
    if (P - 1.0).abs() < EPSILON {
        INFINITY
    } else if P.abs() < EPSILON {
        0.0
    } else {
        let u = ugaussian_pinv(P);
        (zeta + sigma * u).exp()
    }
}

/// Computes the inverse of the lognormal complementary CDF for a given probability `Q`,
/// location parameter `zeta`, and scale parameter `sigma`.
///
/// # Arguments
/// * `Q` - The probability value (0.0 <= Q <= 1.0)
/// * `zeta` - The location parameter
/// * `sigma` - The scale parameter (must be positive)
///
/// # Returns
/// The inverse complementary CDF value, or infinity if Q == 0.0, or 0.0 if Q == 1.0
pub fn lognormal_qinv(Q: f64, zeta: f64, sigma: f64) -> f64 {
    if Q.abs() < EPSILON {
        INFINITY
    } else if (Q - 1.0).abs() < EPSILON {
        0.0
    } else {
        let u = ugaussian_qinv(Q);
        (zeta + sigma * u).exp()
    }
}

/// Helper function to compute the inverse of the standard normal CDF
fn ugaussian_pinv(P: f64) -> f64 {
    // Implementation of the inverse standard normal CDF
    // This would typically use an existing crate like `statrs` in production
    // For this translation, we'll use a simple approximation
    // Note: In a real implementation, this should be replaced with a proper implementation
    // or use a well-tested crate like `statrs::function::erf::erf_inv`
    const A: f64 = 2.50662823884;
    const B: f64 = -18.61500062529;
    const C: f64 = 41.39119773534;
    const D: f64 = -25.44106049637;

    let x = P - 0.5;
    if x.abs() < 0.42 {
        let y = x * x;
        x * (((D * y + C) * y + B) * y + A) / ((((D * y + C) * y + B) * y + 1.0)
    } else {
        let y = if x < 0.0 { P } else { 1.0 - P };
        let y = (-2.0 * y.ln()).sqrt();
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        sign * (y - (2.515517 + 0.802853 * y + 0.010328 * y * y) /
                (1.0 + 1.432788 * y + 0.189269 * y * y + 0.001308 * y * y * y))
    }
}

/// Helper function to compute the inverse of the standard normal complementary CDF
fn ugaussian_qinv(Q: f64) -> f64 {
    // This is equivalent to ugaussian_pinv(1.0 - Q)
    ugaussian_pinv(1.0 - Q)
}