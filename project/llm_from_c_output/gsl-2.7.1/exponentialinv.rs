//! Inverse exponential cumulative distribution functions
//!
//! This module provides functions to compute the inverse of the cumulative
//! distribution functions (CDF) for the exponential distribution.
//!
//! The implementation is based on the GNU Scientific Library (GSL) functions
//! `gsl_cdf_exponential_Pinv` and `gsl_cdf_exponential_Qinv`.
//!
//! Copyright notice preserved from original C code:
//! Copyright (C) 2003, 2007 Brian Gough
//!
//! This program is free software; you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 3 of the License, or (at
//! your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but
//! WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//! General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program; if not, write to the Free Software
//! Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

/// Computes the inverse of the cumulative distribution function P(x) for the exponential distribution
/// with mean parameter `mu`.
///
/// # Arguments
/// * `P` - Probability value, must be in range [0, 1)
/// * `mu` - Mean parameter of the exponential distribution, must be positive
///
/// # Returns
/// The value x such that P(x) = P for the exponential distribution
///
/// # Panics
/// Panics if P is not in [0, 1) or mu is not positive
pub fn exponential_pinv(P: f64, mu: f64) -> f64 {
    assert!(P >= 0.0 && P < 1.0, "P must be in [0, 1)");
    assert!(mu > 0.0, "mu must be positive");
    -mu * (1.0 - P).ln()
}

/// Computes the inverse of the complementary cumulative distribution function Q(x) for the exponential
/// distribution with mean parameter `mu`.
///
/// # Arguments
/// * `Q` - Probability value, must be in range (0, 1]
/// * `mu` - Mean parameter of the exponential distribution, must be positive
///
/// # Returns
/// The value x such that Q(x) = Q for the exponential distribution
///
/// # Panics
/// Panics if Q is not in (0, 1] or mu is not positive
pub fn exponential_qinv(Q: f64, mu: f64) -> f64 {
    assert!(Q > 0.0 && Q <= 1.0, "Q must be in (0, 1]");
    assert!(mu > 0.0, "mu must be positive");
    -mu * Q.ln()
}