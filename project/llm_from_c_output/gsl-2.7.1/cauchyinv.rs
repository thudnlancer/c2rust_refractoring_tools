//! Inverse Cauchy cumulative distribution functions
//!
//! Translated from GNU Scientific Library (GSL) C code.

use std::f64::consts::PI;
use std::f64::{INFINITY, NEG_INFINITY};

/// Computes the inverse of the cumulative distribution function P(x) for a Cauchy distribution
/// with scale parameter `a`.
pub fn gsl_cdf_cauchy_Pinv(P: f64, a: f64) -> f64 {
    if P == 1.0 {
        INFINITY
    } else if P == 0.0 {
        NEG_INFINITY
    } else if P > 0.5 {
        a * (PI * (P - 0.5)).tan()
    } else {
        -a / (PI * P).tan()
    }
}

/// Computes the inverse of the complementary cumulative distribution function Q(x) for a Cauchy
/// distribution with scale parameter `a`.
pub fn gsl_cdf_cauchy_Qinv(Q: f64, a: f64) -> f64 {
    if Q == 0.0 {
        INFINITY
    } else if Q == 1.0 {
        NEG_INFINITY
    } else if Q > 0.5 {
        a * (PI * (0.5 - Q)).tan()
    } else {
        a / (PI * Q).tan()
    }
}