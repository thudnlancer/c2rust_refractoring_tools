//! Gumbel Type-2 distribution cumulative distribution functions
//!
//! Translated from C code:
//! Copyright (C) 2003, 2007 Brian Gough
//! Licensed under GNU General Public License version 3 or later

pub fn gsl_cdf_gumbel2_p(x: f64, a: f64, b: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        let u = x.powf(a);
        (-b / u).exp()
    }
}

pub fn gsl_cdf_gumbel2_q(x: f64, a: f64, b: f64) -> f64 {
    if x == 0.0 {
        1.0
    } else {
        let u = x.powf(a);
        -(-b / u).exp_m1()
    }
}