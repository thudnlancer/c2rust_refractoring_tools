//! Gumbel Type-1 cumulative distribution functions
//!
//! Translated from C code:
//! Copyright (C) 2003, 2007, 2009 Brian Gough
//! Licensed under GNU General Public License version 3 or later

/// Computes the cumulative distribution function P(x) for the Gumbel Type-1 distribution
/// with location parameter a and scale parameter b.
pub fn gumbel1_p(x: f64, a: f64, b: f64) -> f64 {
    let u = a * x - b.ln();
    (-(-u).exp()).exp()
}

/// Computes the survival function Q(x) for the Gumbel Type-1 distribution
/// with location parameter a and scale parameter b.
pub fn gumbel1_q(x: f64, a: f64, b: f64) -> f64 {
    let u = a * x - b.ln();
    let p = (-(-u).exp()).exp();
    
    if p < 0.5 {
        1.0 - p
    } else {
        -(-(-u).exp()).exp_m1()
    }
}