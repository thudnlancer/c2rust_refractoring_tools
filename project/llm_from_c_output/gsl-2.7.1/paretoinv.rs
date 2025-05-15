//! Pareto distribution inverse cumulative distribution functions
//!
//! Translated from C code:
//! Copyright (C) 2003, 2007 Brian Gough
//! Licensed under GNU General Public License version 3 or later

use std::f64;

/// Computes the inverse cumulative distribution function (Pinv) for the Pareto distribution
///
/// # Arguments
/// * `P` - Probability value (0 <= P <= 1)
/// * `a` - Shape parameter
/// * `b` - Scale parameter
///
/// # Returns
/// The value x such that P(X <= x) = P for a Pareto(a, b) random variable
pub fn pareto_pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        f64::INFINITY
    } else if P == 0.0 {
        b
    } else {
        b * (-(1.0 - P).ln_1p() / a).exp()
    }
}

/// Computes the inverse survival function (Qinv) for the Pareto distribution
///
/// # Arguments
/// * `Q` - Survival probability value (0 <= Q <= 1)
/// * `a` - Shape parameter
/// * `b` - Scale parameter
///
/// # Returns
/// The value x such that P(X > x) = Q for a Pareto(a, b) random variable
pub fn pareto_qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        f64::INFINITY
    } else if Q == 1.0 {
        b
    } else {
        b * (-Q.ln() / a).exp()
    }
}