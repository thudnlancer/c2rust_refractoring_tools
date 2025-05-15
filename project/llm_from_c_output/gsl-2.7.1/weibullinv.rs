//! Inverse Weibull distribution functions
//!
//! Translated from C code:
//! Copyright (C) 2003, 2007 Brian Gough
//! Licensed under GNU General Public License version 3 or later

use std::f64;

pub fn gsl_cdf_weibull_pinv(p: f64, a: f64, b: f64) -> f64 {
    if p == 1.0 {
        f64::INFINITY
    } else if p == 0.0 {
        0.0
    } else {
        a * (-(1.0 - p).ln()).powf(1.0 / b)
    }
}

pub fn gsl_cdf_weibull_qinv(q: f64, a: f64, b: f64) -> f64 {
    if q == 0.0 {
        f64::INFINITY
    } else if q == 1.0 {
        0.0
    } else {
        a * (-q.ln()).powf(1.0 / b)
    }
}