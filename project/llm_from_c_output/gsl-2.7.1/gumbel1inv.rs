//! Inverse Gumbel Type 1 distribution functions
//!
//! This module provides the inverse cumulative distribution functions (CDF)
//! for the Gumbel Type 1 distribution, equivalent to the C functions from GSL.

use std::f64::{INFINITY, NEG_INFINITY};

/// Computes the inverse cumulative distribution function P for Gumbel Type 1
///
/// # Arguments
/// * `P` - Probability value (0 ≤ P ≤ 1)
/// * `a` - Location parameter
/// * `b` - Scale parameter (must be positive)
///
/// # Returns
/// The value x such that P = P(X ≤ x) for X ~ Gumbel1(a, b)
pub fn gumbel1_pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        NEG_INFINITY
    } else {
        (-b / P.ln()).ln() / a
    }
}

/// Computes the inverse survival function Q for Gumbel Type 1
///
/// # Arguments
/// * `Q` - Probability value (0 ≤ Q ≤ 1)
/// * `a` - Location parameter
/// * `b` - Scale parameter (must be positive)
///
/// # Returns
/// The value x such that Q = P(X > x) for X ~ Gumbel1(a, b)
pub fn gumbel1_qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        NEG_INFINITY
    } else {
        (-b / (1.0 - Q).ln()).ln() / a
    }
}