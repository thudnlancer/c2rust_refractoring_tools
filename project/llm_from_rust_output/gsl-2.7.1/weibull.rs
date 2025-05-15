use std::f64::consts::E;
use std::ops::{Div, Sub};

pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub get_double: fn(&mut ()) -> f64,
}

pub struct GslRng<'a> {
    pub type_: &'a GslRngType,
    pub state: &'a mut (),
}

fn gsl_rng_uniform_pos(r: &GslRng) -> f64 {
    loop {
        let x = (r.type_.get_double)(r.state);
        if x != 0.0 {
            return x;
        }
    }
}

pub fn gsl_ran_weibull(r: &GslRng, a: f64, b: f64) -> f64 {
    let x = gsl_rng_uniform_pos(r);
    let z = (-x.ln()).powf(1.0 / b);
    a * z
}

pub fn gsl_ran_weibull_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x == 0.0 {
        if b == 1.0 {
            1.0 / a
        } else {
            0.0
        }
    } else if b == 1.0 {
        (-x / a).exp() / a
    } else {
        let ratio = x / a;
        b / a * (-ratio.powf(b) + (b - 1.0) * ratio.ln()).exp()
    }
}