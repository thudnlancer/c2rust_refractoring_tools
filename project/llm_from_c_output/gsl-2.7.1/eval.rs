use std::f64;
use std::cmp::min;

pub struct GslChebSeries {
    pub a: f64,
    pub b: f64,
    pub order: usize,
    pub order_sp: usize,
    pub c: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub enum GslMode {
    PrecDouble,
    PrecSingle,
}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_DBL_EPSILON: f64 = f64::EPSILON;

pub fn gsl_cheb_eval(cs: &GslChebSeries, x: f64) -> f64 {
    let mut d1 = 0.0;
    let mut d2 = 0.0;

    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    for i in (1..=cs.order).rev() {
        let temp = d1;
        d1 = y2 * d1 - d2 + cs.c[i];
        d2 = temp;
    }

    y * d1 - d2 + 0.5 * cs.c[0]
}

pub fn gsl_cheb_eval_n(cs: &GslChebSeries, n: usize, x: f64) -> f64 {
    let mut d1 = 0.0;
    let mut d2 = 0.0;

    let eval_order = min(n, cs.order);

    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    for i in (1..=eval_order).rev() {
        let temp = d1;
        d1 = y2 * d1 - d2 + cs.c[i];
        d2 = temp;
    }

    y * d1 - d2 + 0.5 * cs.c[0]
}

pub fn gsl_cheb_eval_err(
    cs: &GslChebSeries,
    x: f64,
    result: &mut f64,
    abserr: &mut f64,
) -> i32 {
    let mut d1 = 0.0;
    let mut d2 = 0.0;

    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    let mut absc = 0.0;

    for i in (1..=cs.order).rev() {
        let temp = d1;
        d1 = y2 * d1 - d2 + cs.c[i];
        d2 = temp;
    }

    *result = y * d1 - d2 + 0.5 * cs.c[0];

    // Estimate cumulative numerical error
    for coeff in &cs.c[0..=cs.order] {
        absc += coeff.abs();
    }

    // Combine truncation error and numerical error
    *abserr = cs.c[cs.order].abs() + absc * GSL_DBL_EPSILON;

    GSL_SUCCESS
}

pub fn gsl_cheb_eval_n_err(
    cs: &GslChebSeries,
    n: usize,
    x: f64,
    result: &mut f64,
    abserr: &mut f64,
) -> i32 {
    let mut d1 = 0.0;
    let mut d2 = 0.0;

    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    let mut absc = 0.0;

    let eval_order = min(n, cs.order);

    for i in (1..=eval_order).rev() {
        let temp = d1;
        d1 = y2 * d1 - d2 + cs.c[i];
        d2 = temp;
    }

    *result = y * d1 - d2 + 0.5 * cs.c[0];

    // Estimate cumulative numerical error
    for coeff in &cs.c[0..=eval_order] {
        absc += coeff.abs();
    }

    // Combine truncation error and numerical error
    *abserr = cs.c[eval_order].abs() + absc * GSL_DBL_EPSILON;

    GSL_SUCCESS
}

pub fn gsl_cheb_eval_mode_e(
    cs: &GslChebSeries,
    x: f64,
    mode: GslMode,
    result: &mut f64,
    abserr: &mut f64,
) -> i32 {
    let mut d1 = 0.0;
    let mut d2 = 0.0;

    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    let mut absc = 0.0;

    let eval_order = match mode {
        GslMode::PrecDouble => cs.order,
        GslMode::PrecSingle => cs.order_sp,
    };

    for i in (1..=eval_order).rev() {
        let temp = d1;
        d1 = y2 * d1 - d2 + cs.c[i];
        d2 = temp;
    }

    *result = y * d1 - d2 + 0.5 * cs.c[0];

    // Estimate cumulative numerical error
    for coeff in &cs.c[0..=eval_order] {
        absc += coeff.abs();
    }

    // Combine truncation error and numerical error
    *abserr = cs.c[eval_order].abs() + absc * GSL_DBL_EPSILON;

    GSL_SUCCESS
}

pub fn gsl_cheb_eval_mode(cs: &GslChebSeries, x: f64, mode: GslMode) -> f64 {
    let mut result = 0.0;
    let mut abserr = 0.0;
    let status = gsl_cheb_eval_mode_e(cs, x, mode, &mut result, &mut abserr);

    if status != GSL_SUCCESS {
        panic!("gsl_cheb_eval_mode: evaluation failed");
    }

    result
}