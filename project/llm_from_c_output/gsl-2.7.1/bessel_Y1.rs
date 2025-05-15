use std::f64::consts::{PI, FRAC_2_PI};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone)]
struct ChebSeries {
    data: Vec<f64>,
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

impl ChebSeries {
    fn new(data: Vec<f64>, order: usize, a: f64, b: f64, order_sp: usize) -> Self {
        ChebSeries {
            data,
            order,
            a,
            b,
            order_sp,
        }
    }
}

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut SfResult) -> i32 {
    // Simplified implementation - actual implementation would need proper Chebyshev evaluation
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        e += f64::abs(y2 * temp) + f64::abs(dd) + f64::abs(cs.data[j]);
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.data[0];
    e += f64::abs(y * temp) + f64::abs(dd) + 0.5 * f64::abs(cs.data[0]);
    
    result.val = d;
    result.err = f64::EPSILON * e + f64::abs(cs.data[cs.order]);
    
    0 // Success
}

fn gsl_sf_bessel_J1_e(x: f64, result: &mut SfResult) -> i32 {
    // Placeholder - actual implementation would need proper Bessel J1 evaluation
    result.val = x.sin() / x;
    result.err = f64::EPSILON * f64::abs(result.val);
    0
}

fn gsl_sf_bessel_cos_pi4_e(x: f64, t: f64, result: &mut SfResult) -> i32 {
    // Placeholder - actual implementation would need proper cosine evaluation
    let z = x - 0.75 * PI;
    result.val = (z + t).cos();
    result.err = f64::EPSILON * f64::abs(result.val) + f64::abs(t.sin() * t);
    0
}

static BY1_DATA: [f64; 14] = [
    0.03208047100611908629,
    1.262707897433500450,
    0.00649996189992317500,
    -0.08936164528860504117,
    0.01325088122175709545,
    -0.00089790591196483523,
    0.00003647361487958306,
    -0.00000100137438166600,
    0.00000001994539657390,
    -0.00000000030230656018,
    0.00000000000360987815,
    -0.00000000000003487488,
    0.00000000000000027838,
    -0.00000000000000000186,
];

lazy_static::lazy_static! {
    static ref BY1_CS: ChebSeries = ChebSeries::new(BY1_DATA.to_vec(), 13, -1.0, 1.0, 10);
    static ref BESSEL_AMP_PHASE_BM1_CS: ChebSeries = ChebSeries::new(Vec::new(), 0, 0.0, 0.0, 0);
    static ref BESSEL_AMP_PHASE_BTH1_CS: ChebSeries = ChebSeries::new(Vec::new(), 0, 0.0, 0.0, 0);
}

pub fn gsl_sf_bessel_Y1_e(x: f64, result: &mut SfResult) -> i32 {
    let two_over_pi = FRAC_2_PI;
    let xmin = 1.571 * f64::MIN_POSITIVE;
    let x_small = 2.0 * f64::sqrt(f64::EPSILON);
    let xmax = 1.0 / f64::EPSILON;

    if x <= 0.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        libc::EDOM
    } else if x < xmin {
        result.val = f64::NEG_INFINITY;
        result.err = f64::INFINITY;
        libc::ERANGE
    } else if x < x_small {
        let lnterm = (0.5 * x).ln();
        let mut J1 = SfResult { val: 0.0, err: 0.0 };
        let mut c = SfResult { val: 0.0, err: 0.0 };
        let status = gsl_sf_bessel_J1_e(x, &mut J1);
        cheb_eval_e(&BY1_CS, -1.0, &mut c);
        result.val = two_over_pi * lnterm * J1.val + (0.5 + c.val) / x;
        result.err = lnterm.abs() * (f64::EPSILON * J1.val.abs() + J1.err) + c.err / x;
        status
    } else if x < 4.0 {
        let lnterm = (0.5 * x).ln();
        let mut J1 = SfResult { val: 0.0, err: 0.0 };
        let mut c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&BY1_CS, 0.125 * x * x - 1.0, &mut c);
        let status = gsl_sf_bessel_J1_e(x, &mut J1);
        result.val = two_over_pi * lnterm * J1.val + (0.5 + c.val) / x;
        result.err = lnterm.abs() * (f64::EPSILON * J1.val.abs() + J1.err) + c.err / x;
        status
    } else if x < xmax {
        let z = 32.0 / (x * x) - 1.0;
        let mut ca = SfResult { val: 0.0, err: 0.0 };
        let mut ct = SfResult { val: 0.0, err: 0.0 };
        let mut cp = SfResult { val: 0.0, err: 0.0 };
        let stat_ca = cheb_eval_e(&BESSEL_AMP_PHASE_BM1_CS, z, &mut ca);
        let stat_ct = cheb_eval_e(&BESSEL_AMP_PHASE_BTH1_CS, z, &mut ct);
        let stat_cp = gsl_sf_bessel_cos_pi4_e(x, ct.val / x, &mut cp);
        let sqrtx = x.sqrt();
        let ampl = (0.75 + ca.val) / sqrtx;
        result.val = -ampl * cp.val;
        result.err = cp.val.abs() * ca.err / sqrtx + ampl.abs() * cp.err;
        result.err += f64::EPSILON * result.val.abs();
        if stat_ca != 0 { stat_ca } else if stat_ct != 0 { stat_ct } else { stat_cp }
    } else {
        result.val = 0.0;
        result.err = 0.0;
        libc::ERANGE
    }
}

pub fn gsl_sf_bessel_Y1(x: f64) -> f64 {
    let mut result = SfResult { val: 0.0, err: 0.0 };
    let _ = gsl_sf_bessel_Y1_e(x, &mut result);
    result.val
}