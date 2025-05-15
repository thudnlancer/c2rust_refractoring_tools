//! Inverse Rayleigh cumulative distribution functions
//!
//! This module provides implementations of the inverse Rayleigh cumulative distribution functions
//! (Pinv and Qinv) as equivalent safe Rust code.

use std::f64::consts::SQRT_2;
use std::f64::INFINITY;

/// Computes the inverse Rayleigh cumulative distribution function (Pinv)
///
/// # Arguments
/// * `P` - Probability value (0.0 <= P <= 1.0)
/// * `sigma` - Scale parameter of the Rayleigh distribution
///
/// # Returns
/// The value x such that P = P(x; sigma)
pub fn rayleigh_pinv(P: f64, sigma: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        0.0
    } else {
        sigma * SQRT_2 * (-(1.0 - P).ln()).sqrt()
    }
}

/// Computes the inverse Rayleigh complementary cumulative distribution function (Qinv)
///
/// # Arguments
/// * `Q` - Probability value (0.0 <= Q <= 1.0)
/// * `sigma` - Scale parameter of the Rayleigh distribution
///
/// # Returns
/// The value x such that Q = Q(x; sigma)
pub fn rayleigh_qinv(Q: f64, sigma: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        0.0
    } else {
        sigma * SQRT_2 * (-Q.ln()).sqrt()
    }
}