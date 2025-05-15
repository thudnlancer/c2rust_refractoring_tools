//! Inverse hyperbolic functions implementation
//!
//! This module provides implementations of inverse hyperbolic functions:
//! acosh, asinh, and atanh, similar to those in the GNU Scientific Library.
//!
//! The functions are designed to maintain accuracy across different ranges
//! of input values and handle edge cases appropriately.

use std::f64::consts::LN_2;
use std::f64::{INFINITY, NEG_INFINITY, NAN};

/// Inverse hyperbolic cosine function
///
/// # Arguments
/// * `x` - Input value
///
/// # Returns
/// * `f64` - Result of acosh(x), or NAN if x < 1
pub fn gsl_acosh(x: f64) -> f64 {
    if x > 1.0 / f64::EPSILON.sqrt() {
        x.ln() + LN_2
    } else if x > 2.0 {
        (2.0 * x - 1.0 / (x.mul_add(x, -1.0).sqrt() + x)).ln()
    } else if x > 1.0 {
        let t = x - 1.0;
        (t + (2.0 * t + t * t).sqrt()).ln_1p()
    } else if x == 1.0 {
        0.0
    } else {
        NAN
    }
}

/// Inverse hyperbolic sine function
///
/// # Arguments
/// * `x` - Input value
///
/// # Returns
/// * `f64` - Result of asinh(x)
pub fn gsl_asinh(x: f64) -> f64 {
    let a = x.abs();
    let s = if x < 0.0 { -1.0 } else { 1.0 };

    if a > 1.0 / f64::EPSILON.sqrt() {
        s * (a.ln() + LN_2)
    } else if a > 2.0 {
        s * (2.0 * a + 1.0 / (a + a.mul_add(a, 1.0).sqrt())).ln()
    } else if a > f64::EPSILON.sqrt() {
        let a2 = a * a;
        s * (a + a2 / (1.0 + (1.0 + a2).sqrt())).ln_1p()
    } else {
        x
    }
}

/// Inverse hyperbolic tangent function
///
/// # Arguments
/// * `x` - Input value
///
/// # Returns
/// * `f64` - Result of atanh(x), or NAN if |x| > 1
pub fn gsl_atanh(x: f64) -> f64 {
    let a = x.abs();
    let s = if x < 0.0 { -1.0 } else { 1.0 };

    if a > 1.0 {
        NAN
    } else if a == 1.0 {
        if x < 0.0 { NEG_INFINITY } else { INFINITY }
    } else if a >= 0.5 {
        s * 0.5 * (2.0 * a / (1.0 - a)).ln_1p()
    } else if a > f64::EPSILON {
        s * 0.5 * (2.0 * a + 2.0 * a * a / (1.0 - a)).ln_1p()
    } else {
        x
    }
}