use std::f64::consts::{SQRT_2, EPSILON, MIN};
use std::f64;

const ROOT_EIGHT: f64 = 2.0 * SQRT_2;

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

static BI1_DATA: [f64; 11] = [
    -0.001971713261099859,
    0.407348876675464810,
    0.034838994299959456,
    0.001545394556300123,
    0.000041888521098377,
    0.000000764902676483,
    0.000000010042493924,
    0.000000000099322077,
    0.000000000000766380,
    0.000000000000004741,
    0.000000000000000024,
];

static BI1_CS: ChebSeries = ChebSeries {
    data: &BI1_DATA,
    order: 10,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static AI1_DATA: [f64; 21] = [
    -0.02846744181881479,
    -0.01922953231443221,
    -0.00061151858579437,
    -0.00002069971253350,
    0.00000858561914581,
    0.00000104949824671,
    -0.00000029183389184,
    -0.00000001559378146,
    0.00000001318012367,
    -0.00000000144842341,
    -0.00000000029085122,
    0.00000000012663889,
    -0.00000000001664947,
    -0.00000000000166665,
    0.00000000000124260,
    -0.00000000000027315,
    0.00000000000002023,
    0.00000000000000730,
    -0.00000000000000333,
    0.00000000000000071,
    -0.00000000000000006,
];

static AI1_CS: ChebSeries = ChebSeries {
    data: &AI1_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 11,
};

static AI12_DATA: [f64; 22] = [
    0.02857623501828014,
    -0.00976109749136147,
    -0.00011058893876263,
    -0.00000388256480887,
    -0.00000025122362377,
    -0.00000002631468847,
    -0.00000000383538039,
    -0.00000000055897433,
    -0.00000000001897495,
    0.00000000003252602,
    0.00000000001412580,
    0.00000000000203564,
    -0.00000000000071985,
    -0.00000000000040836,
    -0.00000000000002101,
    0.00000000000004273,
    0.00000000000001041,
    -0.00000000000000382,
    -0.00000000000000186,
    0.00000000000000033,
    0.00000000000000028,
    -0.00000000000000003,
];

static AI12_CS: ChebSeries = ChebSeries {
    data: &AI12_DATA,
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

pub enum SfError {
    Underflow,
    Overflow,
    Domain,
    Other,
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let d = 0.0;
    let dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;
    
    let mut val = 0.0;
    for &c in cs.data.iter().rev() {
        let temp = d;
        d = y2 * d - dd + c;
        dd = temp;
        e += c.abs();
    }
    
    val = y * d - dd + 0.5 * cs.data[0];
    e = (e + val.abs()) * EPSILON;
    
    SfResult { val, err: e }
}

pub fn bessel_i1_scaled_e(x: f64) -> Result<SfResult, SfError> {
    let xmin = 2.0 * MIN;
    let x_small = ROOT_EIGHT * f64::sqrt(EPSILON);
    let y = x.abs();

    if y == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else if y < xmin {
        Err(SfError::Underflow)
    } else if y < x_small {
        Ok(SfResult { val: 0.5 * x, err: 0.0 })
    } else if y <= 3.0 {
        let ey = f64::exp(-y);
        let c = cheb_eval_e(&BI1_CS, y * y / 4.5 - 1.0);
        let val = x * ey * (0.875 + c.val);
        let err = ey * c.err + y * EPSILON * val.abs();
        let err = err + 2.0 * EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if y <= 8.0 {
        let sy = f64::sqrt(y);
        let c = cheb_eval_e(&AI1_CS, (48.0 / y - 11.0) / 5.0);
        let b = (0.375 + c.val) / sy;
        let s = if x > 0.0 { 1.0 } else { -1.0 };
        let val = s * b;
        let err = c.err / sy + 2.0 * EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else {
        let sy = f64::sqrt(y);
        let c = cheb_eval_e(&AI12_CS, 16.0 / y - 1.0);
        let b = (0.375 + c.val) / sy;
        let s = if x > 0.0 { 1.0 } else { -1.0 };
        let val = s * b;
        let err = c.err / sy + 2.0 * EPSILON * val.abs();
        Ok(SfResult { val, err })
    }
}

pub fn bessel_i1_e(x: f64) -> Result<SfResult, SfError> {
    let xmin = 2.0 * MIN;
    let x_small = ROOT_EIGHT * f64::sqrt(EPSILON);
    let y = x.abs();

    if y == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else if y < xmin {
        Err(SfError::Underflow)
    } else if y < x_small {
        Ok(SfResult { val: 0.5 * x, err: 0.0 })
    } else if y <= 3.0 {
        let c = cheb_eval_e(&BI1_CS, y * y / 4.5 - 1.0);
        let val = x * (0.875 + c.val);
        let err = y * c.err + 2.0 * EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if y < f64::MAX.log2() {
        let ey = f64::exp(y);
        let i1_scaled = bessel_i1_scaled_e(x)?;
        let val = ey * i1_scaled.val;
        let err = ey * i1_scaled.err + y * EPSILON * val.abs();
        let err = err + 2.0 * EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else {
        Err(SfError::Overflow)
    }
}

pub fn bessel_i1_scaled(x: f64) -> f64 {
    bessel_i1_scaled_e(x).unwrap().val
}

pub fn bessel_i1(x: f64) -> f64 {
    bessel_i1_e(x).unwrap().val
}