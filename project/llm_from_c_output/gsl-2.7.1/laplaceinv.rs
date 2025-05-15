//! Inverse Laplace cumulative distribution functions
//!
//! This module provides functions to compute the inverse of the cumulative
//! distribution function (CDF) for the Laplace distribution.

use std::f64::{INFINITY, NEG_INFINITY};

/// Computes the inverse of the Laplace cumulative distribution function (Pinv)
///
/// # Arguments
/// * `P` - Probability value in [0, 1]
/// * `a` - Scale parameter of the Laplace distribution
///
/// # Returns
/// The value x such that P(X ≤ x) = P for a Laplace distribution with scale parameter a
pub fn laplace_pinv(P: f64, a: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        NEG_INFINITY
    } else if P < 0.5 {
        a * (2.0 * P).ln()
    } else {
        -a * (2.0 * (1.0 - P)).ln()
    }
}

/// Computes the inverse of the Laplace complementary cumulative distribution function (Qinv)
///
/// # Arguments
/// * `Q` - Probability value in [0, 1]
/// * `a` - Scale parameter of the Laplace distribution
///
/// # Returns
/// The value x such that P(X > x) = Q for a Laplace distribution with scale parameter a
pub fn laplace_qinv(Q: f64, a: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        NEG_INFINITY
    } else if Q < 0.5 {
        -a * (2.0 * Q).ln()
    } else {
        a * (2.0 * (1.0 - Q)).ln()
    }
}