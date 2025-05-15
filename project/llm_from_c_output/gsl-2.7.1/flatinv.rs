//! Inverse cumulative distribution functions for the flat distribution
//!
//! This module provides functions to compute the inverse cumulative distribution
//! functions (quantile functions) for the flat (uniform) distribution.

/// Computes the inverse cumulative distribution function (Pinv) for the flat distribution
///
/// # Arguments
/// * `P` - The cumulative probability value (0 ≤ P ≤ 1)
/// * `a` - The lower bound of the distribution
/// * `b` - The upper bound of the distribution (must be > a)
///
/// # Returns
/// The quantile corresponding to the cumulative probability P
pub fn flat_pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        b
    } else if P == 0.0 {
        a
    } else {
        (1.0 - P) * a + P * b
    }
}

/// Computes the inverse survival function (Qinv) for the flat distribution
///
/// # Arguments
/// * `Q` - The survival probability value (0 ≤ Q ≤ 1)
/// * `a` - The lower bound of the distribution
/// * `b` - The upper bound of the distribution (must be > a)
///
/// # Returns
/// The quantile corresponding to the survival probability Q
pub fn flat_qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        b
    } else if Q == 1.0 {
        a
    } else {
        Q * a + (1.0 - Q) * b
    }
}