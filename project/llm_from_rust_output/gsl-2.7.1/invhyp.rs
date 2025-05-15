use std::f64::consts::LN_2;
use std::f64::{self, NAN, INFINITY};

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

pub fn gsl_asinh(x: f64) -> f64 {
    let a = x.abs();
    let s = x.signum();
    
    if a > 1.0 / f64::EPSILON.sqrt() {
        s * (a.ln() + LN_2)
    } else if a > 2.0 {
        s * (2.0 * a + 1.0 / (a + a.mul_add(a, 1.0).sqrt())).ln()
    } else if a > f64::EPSILON.sqrt() {
        let a2 = a * a;
        s * (a + a2 / (1.0 + (1.0 + a2).sqrt()).ln_1p()
    } else {
        x
    }
}

pub fn gsl_atanh(x: f64) -> f64 {
    let a = x.abs();
    let s = x.signum();
    
    if a > 1.0 {
        NAN
    } else if a == 1.0 {
        s * INFINITY
    } else if a >= 0.5 {
        s * 0.5 * (2.0 * a / (1.0 - a)).ln_1p()
    } else if a > 2.2204460492503131e-16 {
        s * 0.5 * (2.0 * a + 2.0 * a * a / (1.0 - a)).ln_1p()
    } else {
        x
    }
}