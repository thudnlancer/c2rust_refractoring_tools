use std::f64::consts::PI;
use std::f64::{NAN, EPSILON};

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ChebSeries {
    pub c: &'static [f64],
    pub order: i32,
    pub a: f64,
    pub b: f64,
    pub order_sp: i32,
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

impl GslError {
    fn from_i32(code: i32) -> Self {
        match code {
            0 => GslError::Success,
            -1 => GslError::Failure,
            -2 => GslError::Continue,
            1 => GslError::Domain,
            2 => GslError::Range,
            3 => GslError::Fault,
            4 => GslError::Invalid,
            5 => GslError::Failed,
            6 => GslError::Factor,
            7 => GslError::Sanity,
            8 => GslError::NoMem,
            9 => GslError::BadFunc,
            10 => GslError::Runaway,
            11 => GslError::MaxIter,
            12 => GslError::ZeroDiv,
            13 => GslError::BadTol,
            14 => GslError::Tol,
            15 => GslError::Underflow,
            16 => GslError::Overflow,
            17 => GslError::Loss,
            18 => GslError::Round,
            19 => GslError::BadLen,
            20 => GslError::NotSquare,
            21 => GslError::Singular,
            22 => GslError::Diverge,
            23 => GslError::Unsupported,
            24 => GslError::Unimplemented,
            25 => GslError::Cache,
            26 => GslError::Table,
            27 => GslError::NoProgress,
            28 => GslError::NoProgressJ,
            29 => GslError::TolF,
            30 => GslError::TolX,
            31 => GslError::TolG,
            32 => GslError::Eof,
            _ => GslError::Invalid,
        }
    }
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> Result<GslSfResult, GslError> {
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
    
    Ok(GslSfResult {
        val: d,
        err: 2.2204460492503131e-16 * e + cs.c[cs.order as usize].abs(),
    })
}

static BY0_DATA: [f64; 13] = [
    -0.011277839392865573,
    -0.128345237560420350,
    -0.104378847997942490,
    0.023662749183969695,
    -0.002090391647700486,
    0.000103975453939057,
    -0.000003369747162423,
    0.000000077293842676,
    -0.000000001324976772,
    0.000000000017648232,
    -0.000000000000188105,
    0.000000000000001641,
    -0.000000000000000011,
];

static BY0_CS: ChebSeries = ChebSeries {
    c: &BY0_DATA,
    order: 12,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

pub fn gsl_sf_bessel_y0_e(x: f64) -> Result<GslSfResult, GslError> {
    let two_over_pi = 2.0 / PI;
    let xmax = 1.0 / EPSILON;
    
    if x <= 0.0 {
        Err(GslError::Domain)
    } else if x < 4.0 {
        let j0 = gsl_sf_bessel_j0_e(x)?;
        let c = cheb_eval_e(&BY0_CS, 0.125 * x * x - 1.0)?;
        
        let val = two_over_pi * (-0.69314718055994530942 + x.ln()) * j0.val + 0.375 + c.val;
        let err = 2.0 * 2.2204460492503131e-16 * val.abs() + c.err;
        
        Ok(GslSfResult { val, err })
    } else if x < xmax {
        let z = 32.0 / (x * x) - 1.0;
        
        let c1 = cheb_eval_e(&_GSL_SF_BESSEL_AMP_PHASE_BM0_CS, z)?;
        let c2 = cheb_eval_e(&_GSL_SF_BESSEL_AMP_PHASE_BTH0_CS, z)?;
        let sp = gsl_sf_bessel_sin_pi4_e(x, c2.val / x)?;
        
        let sqrtx = x.sqrt();
        let ampl = (0.75 + c1.val) / sqrtx;
        
        let val = ampl * sp.val;
        let err = sp.val.abs() * c1.err / sqrtx + ampl.abs() * sp.err;
        let err = err + 2.0 * 2.2204460492503131e-16 * val.abs();
        
        Ok(GslSfResult { val, err })
    } else {
        Err(GslError::Underflow)
    }
}

pub fn gsl_sf_bessel_y0(x: f64) -> f64 {
    match gsl_sf_bessel_y0_e(x) {
        Ok(result) => result.val,
        Err(_) => NAN,
    }
}

// Placeholder for actual implementations of these functions
fn gsl_sf_bessel_j0_e(x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_sin_pi4_e(x: f64, eps: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

static _GSL_SF_BESSEL_AMP_PHASE_BM0_CS: ChebSeries = ChebSeries {
    c: &[],
    order: 0,
    a: 0.0,
    b: 0.0,
    order_sp: 0,
};

static _GSL_SF_BESSEL_AMP_PHASE_BTH0_CS: ChebSeries = ChebSeries {
    c: &[],
    order: 0,
    a: 0.0,
    b: 0.0,
    order_sp: 0,
};