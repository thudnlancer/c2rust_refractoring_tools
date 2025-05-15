use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ChebSeries {
    c: &'static [f64],
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

const GSL_SUCCESS: i32 = 0;
const GSL_EDOM: i32 = 1;
const GSL_EOVRFLW: i32 = 16;
const GSL_EUNDRFLW: i32 = 15;

const BY1_DATA: [f64; 14] = [
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

static BY1_CS: ChebSeries = ChebSeries {
    c: &BY1_DATA,
    order: 13,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut GslSfResult) -> i32 {
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.c[j as usize];
        e += (y2 * temp).abs() + dd.abs() + cs.c[j as usize].abs();
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.c[0];
    e += (y * temp).abs() + dd.abs() + 0.5 * cs.c[0].abs();
    
    result.val = d;
    result.err = 2.2204460492503131e-16 * e + cs.c[cs.order as usize].abs();
    
    GSL_SUCCESS
}

pub fn gsl_sf_bessel_Y1_e(x: f64, result: &mut GslSfResult) -> i32 {
    let two_over_pi = 2.0 / PI;
    let xmin = 1.571 * f64::MIN_POSITIVE;
    let x_small = 2.0 * f64::EPSILON;
    let xmax = 1.0 / f64::EPSILON;
    
    if x <= 0.0 {
        result.val = NAN;
        result.err = NAN;
        return GSL_EDOM;
    } else if x < xmin {
        result.val = INFINITY;
        result.err = INFINITY;
        return GSL_EOVRFLW;
    } else if x < x_small {
        let lnterm = (0.5 * x).ln();
        let mut J1 = GslSfResult { val: 0.0, err: 0.0 };
        let mut c = GslSfResult { val: 0.0, err: 0.0 };
        
        // Assume gsl_sf_bessel_J1_e is implemented safely elsewhere
        let status = gsl_sf_bessel_J1_e(x, &mut J1);
        cheb_eval_e(&BY1_CS, -1.0, &mut c);
        
        result.val = two_over_pi * lnterm * J1.val + (0.5 + c.val) / x;
        result.err = lnterm.abs() * (2.2204460492503131e-16 * J1.val.abs() + J1.err) + c.err / x;
        
        status
    } else if x < 4.0 {
        let lnterm = (0.5 * x).ln();
        let mut J1 = GslSfResult { val: 0.0, err: 0.0 };
        let mut c = GslSfResult { val: 0.0, err: 0.0 };
        
        cheb_eval_e(&BY1_CS, 0.125 * x * x - 1.0, &mut c);
        let status = gsl_sf_bessel_J1_e(x, &mut J1);
        
        result.val = two_over_pi * lnterm * J1.val + (0.5 + c.val) / x;
        result.err = lnterm.abs() * (2.2204460492503131e-16 * J1.val.abs() + J1.err) + c.err / x;
        
        status
    } else if x < xmax {
        let z = 32.0 / (x * x) - 1.0;
        let mut ca = GslSfResult { val: 0.0, err: 0.0 };
        let mut ct = GslSfResult { val: 0.0, err: 0.0 };
        let mut cp = GslSfResult { val: 0.0, err: 0.0 };
        
        // Assume these functions are implemented safely elsewhere
        let stat_ca = cheb_eval_e(&_gsl_sf_bessel_amp_phase_bm1_cs, z, &mut ca);
        let stat_ct = cheb_eval_e(&_gsl_sf_bessel_amp_phase_bth1_cs, z, &mut ct);
        let stat_cp = gsl_sf_bessel_cos_pi4_e(x, ct.val / x, &mut cp);
        
        let sqrtx = x.sqrt();
        let ampl = (0.75 + ca.val) / sqrtx;
        
        result.val = -ampl * cp.val;
        result.err = cp.val.abs() * ca.err / sqrtx + ampl.abs() * cp.err;
        result.err += 2.2204460492503131e-16 * result.val.abs();
        
        if stat_ca != GSL_SUCCESS {
            stat_ca
        } else if stat_ct != GSL_SUCCESS {
            stat_ct
        } else if stat_cp != GSL_SUCCESS {
            stat_cp
        } else {
            GSL_SUCCESS
        }
    } else {
        result.val = 0.0;
        result.err = f64::MIN_POSITIVE;
        GSL_EUNDRFLW
    }
}

pub fn gsl_sf_bessel_Y1(x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_bessel_Y1_e(x, &mut result);
    if status != GSL_SUCCESS {
        // Handle error appropriately
        result.val
    } else {
        result.val
    }
}

// These would need to be defined elsewhere in your codebase
static _gsl_sf_bessel_amp_phase_bm1_cs: ChebSeries = ChebSeries { /* ... */ };
static _gsl_sf_bessel_amp_phase_bth1_cs: ChebSeries = ChebSeries { /* ... */ };

// These functions would need to be implemented safely elsewhere
fn gsl_sf_bessel_J1_e(x: f64, result: &mut GslSfResult) -> i32 { unimplemented!() }
fn gsl_sf_bessel_cos_pi4_e(y: f64, eps: f64, result: &mut GslSfResult) -> i32 { unimplemented!() }