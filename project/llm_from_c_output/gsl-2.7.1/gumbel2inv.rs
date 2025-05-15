//! Inverse Gumbel Type 2 distribution functions
//!
//! This module provides the inverse cumulative distribution functions (CDF)
//! for the Gumbel Type 2 distribution, equivalent to the C functions from
//! the GNU Scientific Library.

use std::f64;

/// Computes the inverse cumulative distribution function P for the Gumbel Type 2 distribution.
///
/// # Arguments
/// * `P` - Probability value (0 ≤ P ≤ 1)
/// * `a` - Shape parameter
/// * `b` - Scale parameter
///
/// # Returns
/// The value x such that P = P(x) under the Gumbel Type 2 distribution.
/// Returns f64::INFINITY when P == 1.0 and 0.0 when P == 0.0.
pub fn gumbel2_pinv(P: f64, a: f64, b: f64) -> f64 {
    if P == 1.0 {
        return f64::INFINITY;
    } else if P == 0.0 {
        return 0.0;
    }

    (b / (-P.ln())).powf(1.0 / a)
}

/// Computes the inverse complementary cumulative distribution function Q for the Gumbel Type 2 distribution.
///
/// # Arguments
/// * `Q` - Probability value (0 ≤ Q ≤ 1)
/// * `a` - Shape parameter
/// * `b` - Scale parameter
///
/// # Returns
/// The value x such that Q = Q(x) under the Gumbel Type 2 distribution.
/// Returns f64::INFINITY when Q == 0.0 and 0.0 when Q == 1.0.
pub fn gumbel2_qinv(Q: f64, a: f64, b: f64) -> f64 {
    if Q == 0.0 {
        return f64::INFINITY;
    } else if Q == 1.0 {
        return 0.0;
    }

    (b / (-(1.0 - Q).ln())).powf(1.0 / a)
}