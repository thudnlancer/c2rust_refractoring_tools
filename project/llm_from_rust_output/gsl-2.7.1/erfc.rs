use std::f64::consts::{PI, SQRT_2};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
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
const GSL_EUNDRFLW: i32 = 15;

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut SfResult) -> i32 {
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut e = 0.0;
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.c[j as usize];
        e += f64::abs(y2 * temp) + f64::abs(dd) + f64::abs(cs.c[j as usize]);
        dd = temp;
    }
    
    let temp = d;
    d = y * d - dd + 0.5 * cs.c[0];
    e += f64::abs(y * temp) + f64::abs(dd) + 0.5 * f64::abs(cs.c[0]);
    
    result.val = d;
    result.err = 2.2204460492503131e-16 * e + f64::abs(cs.c[cs.order as usize]);
    
    GSL_SUCCESS
}

fn erfc8_sum(x: f64) -> f64 {
    const P: [f64; 6] = [
        2.97886562639399288862,
        7.409740605964741794425,
        6.1602098531096305440906,
        5.019049726784267463450058,
        1.275366644729965952479585264,
        0.5641895835477550741253201704,
    ];
    
    const Q: [f64; 7] = [
        3.3690752069827527677,
        9.608965327192787870698,
        17.08144074746600431571095,
        12.0489519278551290360340491,
        9.396034016235054150430579648,
        2.260528520767326969591866945,
        1.0,
    ];
    
    let num = P[5] + x * (P[4] + x * (P[3] + x * (P[2] + x * (P[1] + x * P[0]))); 
    let den = Q[6] + x * (Q[5] + x * (Q[4] + x * (Q[3] + x * (Q[2] + x * (Q[1] + x * Q[0])))));
    
    num / den
}

fn erfc8(x: f64) -> f64 {
    let e = erfc8_sum(x);
    e * f64::exp(-x * x)
}

fn log_erfc8(x: f64) -> f64 {
    let e = erfc8_sum(x);
    f64::ln(e) - x * x
}

fn erfseries(x: f64, result: &mut SfResult) -> i32 {
    let mut coef = x;
    let mut e = coef;
    
    for k in 1..30 {
        coef *= -x * x / k as f64;
        let del = coef / (2.0 * k as f64 + 1.0);
        e += del;
    }
    
    result.val = 2.0 / PI.sqrt() * e;
    result.err = 2.0 / PI.sqrt() * (f64::abs(e) + 2.2204460492503131e-16);
    
    GSL_SUCCESS
}

const ERFC_XLT1_DATA: [f64; 20] = [
    // ... (same as original)
];

const ERFC_X15_DATA: [f64; 25] = [
    // ... (same as original)
];

const ERFC_X510_DATA: [f64; 20] = [
    // ... (same as original)
];

const ERFC_XLT1_CS: ChebSeries = ChebSeries {
    c: &ERFC_XLT1_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

const ERFC_X15_CS: ChebSeries = ChebSeries {
    c: &ERFC_X15_DATA,
    order: 24,
    a: -1.0,
    b: 1.0,
    order_sp: 16,
};

const ERFC_X510_CS: ChebSeries = ChebSeries {
    c: &ERFC_X510_DATA,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

pub fn gsl_sf_erfc_e(x: f64, result: &mut SfResult) -> i32 {
    let ax = f64::abs(x);
    let (e_val, e_err) = if ax <= 1.0 {
        let t = 2.0 * ax - 1.0;
        let mut c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&ERFC_XLT1_CS, t, &mut c);
        (c.val, c.err)
    } else if ax <= 5.0 {
        let ex2 = f64::exp(-x * x);
        let t = 0.5 * (ax - 3.0);
        let mut c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&ERFC_X15_CS, t, &mut c);
        (ex2 * c.val, ex2 * (c.err + 2.0 * f64::abs(x) * 2.2204460492503131e-16))
    } else if ax < 10.0 {
        let exterm = f64::exp(-x * x) / ax;
        let t = (2.0 * ax - 15.0) / 5.0;
        let mut c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&ERFC_X510_CS, t, &mut c);
        (exterm * c.val, exterm * (c.err + 2.0 * f64::abs(x) * 2.2204460492503131e-16 + 2.2204460492503131e-16))
    } else {
        let val = erfc8(ax);
        (val, (x * x + 1.0) * 2.2204460492503131e-16 * f64::abs(val))
    };
    
    if x < 0.0 {
        result.val = 2.0 - e_val;
        result.err = e_err + 2.0 * 2.2204460492503131e-16 * f64::abs(result.val);
    } else {
        result.val = e_val;
        result.err = e_err + 2.0 * 2.2204460492503131e-16 * f64::abs(result.val);
    }
    
    GSL_SUCCESS
}

// ... (remaining functions following the same pattern of conversion)