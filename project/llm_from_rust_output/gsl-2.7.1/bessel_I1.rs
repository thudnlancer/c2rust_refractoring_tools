use std::f64::consts::{SQRT_2, EPSILON};
use std::f64::{INFINITY, MIN_POSITIVE};

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ChebSeries {
    c: &'static [f64],
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

fn cheb_eval(cs: &ChebSeries, x: f64) -> GslSfResult {
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

    GslSfResult {
        val: d,
        err: f64::EPSILON * e + cs.c[cs.order as usize].abs(),
    }
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
    c: &BI1_DATA,
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
    c: &AI1_DATA,
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
    c: &AI12_DATA,
    order: 21,
    a: -1.0,
    b: 1.0,
    order_sp: 9,
};

pub fn gsl_sf_bessel_i1_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    let xmin = 2.0 * MIN_POSITIVE;
    let x_small = 2.0 * SQRT_2 * EPSILON;
    let y = x.abs();

    if y == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if y < xmin {
        Err(GslError::Underflow)
    } else if y < x_small {
        Ok(GslSfResult {
            val: 0.5 * x,
            err: 0.0,
        })
    } else if y <= 3.0 {
        let ey = (-y).exp();
        let c = cheb_eval(&BI1_CS, y * y / 4.5 - 1.0);
        let val = x * ey * (0.875 + c.val);
        let err = ey * c.err + y * f64::EPSILON * val.abs();
        Ok(GslSfResult {
            val,
            err: err + 2.0 * f64::EPSILON * val.abs(),
        })
    } else if y <= 8.0 {
        let sy = y.sqrt();
        let c = cheb_eval(&AI1_CS, (48.0 / y - 11.0) / 5.0);
        let b = (0.375 + c.val) / sy;
        let s = if x > 0.0 { 1.0 } else { -1.0 };
        let val = s * b;
        Ok(GslSfResult {
            val,
            err: c.err / sy + 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let sy = y.sqrt();
        let c = cheb_eval(&AI12_CS, 16.0 / y - 1.0);
        let b = (0.375 + c.val) / sy;
        let s = if x > 0.0 { 1.0 } else { -1.0 };
        let val = s * b;
        Ok(GslSfResult {
            val,
            err: c.err / sy + 2.0 * f64::EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_bessel_i1_e(x: f64) -> Result<GslSfResult, GslError> {
    let xmin = 2.0 * MIN_POSITIVE;
    let x_small = 2.0 * SQRT_2 * EPSILON;
    let y = x.abs();

    if y == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if y < xmin {
        Err(GslError::Underflow)
    } else if y < x_small {
        Ok(GslSfResult {
            val: 0.5 * x,
            err: 0.0,
        })
    } else if y <= 3.0 {
        let c = cheb_eval(&BI1_CS, y * y / 4.5 - 1.0);
        let val = x * (0.875 + c.val);
        Ok(GslSfResult {
            val,
            err: y * c.err + 2.0 * f64::EPSILON * val.abs(),
        })
    } else if y < 7.0978271289338397e+02 {
        let ey = y.exp();
        let i1_scaled = gsl_sf_bessel_i1_scaled_e(x)?;
        let val = ey * i1_scaled.val;
        Ok(GslSfResult {
            val,
            err: ey * i1_scaled.err + y * f64::EPSILON * val.abs() + 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        Err(GslError::Overflow)
    }
}

pub fn gsl_sf_bessel_i1_scaled(x: f64) -> f64 {
    gsl_sf_bessel_i1_scaled_e(x).unwrap_or_else(|e| {
        if e == GslError::Underflow {
            0.0
        } else {
            panic!("gsl_sf_bessel_i1_scaled_e error: {:?}", e)
        }
    }).val
}

pub fn gsl_sf_bessel_i1(x: f64) -> f64 {
    gsl_sf_bessel_i1_e(x).unwrap_or_else(|e| {
        if e == GslError::Underflow {
            0.0
        } else if e == GslError::Overflow {
            INFINITY
        } else {
            panic!("gsl_sf_bessel_i1_e error: {:?}", e)
        }
    }).val
}