//! Inverse logistic cumulative distribution functions
//!
//! This module provides functions to compute the inverse of the logistic cumulative distribution function.
//! 
//! The code is translated from C to Rust following the original logic while adhering to Rust's safety practices.

use std::f64::{INFINITY, NEG_INFINITY};

/// Computes the inverse of the logistic cumulative distribution function P.
///
/// # Arguments
/// * `P` - Probability value, must be in range [0, 1]
/// * `a` - Scale parameter of the logistic distribution
///
/// # Returns
/// The value x such that P = P(X ≤ x) for a logistic random variable X
pub fn logistic_pinv(p: f64, a: f64) -> f64 {
    if p == 1.0 {
        INFINITY
    } else if p == 0.0 {
        NEG_INFINITY
    } else {
        a * (p / (1.0 - p)).ln()
    }
}

/// Computes the inverse of the logistic survival function Q.
///
/// # Arguments
/// * `Q` - Survival probability value, must be in range [0, 1]
/// * `a` - Scale parameter of the logistic distribution
///
/// # Returns
/// The value x such that Q = P(X > x) for a logistic random variable X
pub fn logistic_qinv(q: f64, a: f64) -> f64 {
    if q == 0.0 {
        INFINITY
    } else if q == 1.0 {
        NEG_INFINITY
    } else {
        a * ((1.0 - q) / q).ln()
    }
}