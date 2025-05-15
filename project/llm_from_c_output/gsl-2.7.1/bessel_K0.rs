use std::f64::consts;
use std::f64;

// Chebyshev series structure
struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

// Result type for special functions
#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

// Constants
const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;

// Polynomial coefficients
const K0_POLY: [f64; 8] = [
    1.1593151565841244842077226e-01,
    2.7898287891460317300886539e-01,
    2.5248929932161220559969776e-02,
    8.4603509072136578707676406e-04,
    1.4914719243067801775856150e-05,
    1.6271068931224552553548933e-07,
    1.2082660336282566759313543e-09,
    6.6117104672254184399933971e-12,
];

const I0_POLY: [f64; 7] = [
    1.0000000000000000044974165e+00,
    2.4999999999999822316775454e-01,
    2.7777777777892149148858521e-02,
    1.7361111083544590676709592e-03,
    6.9444476047072424198677755e-05,
    1.9288265756466775034067979e-06,
    3.9908220583262192851839992e-08,
];

// Chebyshev expansions
const AK0_DATA: [f64; 24] = [
    -3.28737867094650101e-02,
    -4.49369057710236880e-02,
    +2.98149992004308095e-03,
    -3.03693649396187920e-04,
    +3.91085569307646836e-05,
    -5.86872422399215952e-06,
    +9.82873709937322009e-07,
    -1.78978645055651171e-07,
    +3.48332306845240957e-08,
    -7.15909210462546599e-09,
    +1.54019930048919494e-09,
    -3.44555485579194210e-10,
    +7.97356101783753023e-11,
    -1.90090968913069735e-11,
    +4.65295609304114621e-12,
    -1.16614287433470780e-12,
    +2.98554375218596891e-13,
    -7.79276979512292169e-14,
    +2.07027467168948402e-14,
    -5.58987860393825313e-15,
    +1.53202965950646914e-15,
    -4.25737536712188186e-16,
    +1.19840238501357389e-16,
    -3.41407346762502397e-17,
];

const AK0_CS: ChebSeries = ChebSeries {
    data: &AK0_DATA,
    order: 23,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

const AK02_DATA: [f64; 14] = [
    -0.1201869826307592240E-1,
    -0.9174852691025695311E-2,
    +0.1444550931775005821E-3,
    -0.4013614175435709729E-5,
    +0.1567831810852310673E-6,
    -0.7770110438521737710E-8,
    +0.4611182576179717883E-9,
    -0.3158592997860565771E-10,
    +0.2435018039365041128E-11,
    -0.2074331387398347898E-12,
    +0.1925787280589917085E-13,
    -0.1927554805838956104E-14,
    +0.2062198029197818278E-15,
    -0.2341685117579242403E-16,
];

const AK02_CS: ChebSeries = ChebSeries {
    data: &AK02_DATA,
    order: 13,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

// Error handling
#[derive(Debug)]
pub enum SfError {
    DomainError,
    OtherError,
}

// Polynomial evaluation
fn poly_eval(coeffs: &[f64], x: f64) -> f64 {
    coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
}

// Chebyshev evaluation
fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let mut d = 0.0;
    let mut dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;

    for &c in cs.data.iter().rev() {
        let temp = d;
        d = y2 * d - dd + c;
        dd = temp;
    }

    SfResult::new(y * d - dd + 0.5 * cs.data[0], 0.0) // TODO: proper error estimation
}

// Scaled modified Bessel function K0
pub fn bessel_k0_scaled_e(x: f64) -> Result<SfResult, SfError> {
    if x <= 0.0 {
        Err(SfError::DomainError)
    } else if x < 1.0 {
        let lx = x.ln();
        let ex = x.exp();
        let x2 = x * x;
        let val = ex * (poly_eval(&K0_POLY, x2) - lx * (1.0 + 0.25 * x2 * poly_eval(&I0_POLY, 0.25 * x2));
        let err = ex * (1.6 + lx.abs() * 0.6) * GSL_DBL_EPSILON;
        Ok(SfResult::new(val, err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x <= 8.0 {
        let sx = x.sqrt();
        let c = cheb_eval_e(&AK0_CS, (16.0 / x - 9.0) / 7.0);
        let val = (1.203125 + c.val) / sx; // 1.203125 = 77/64
        Ok(SfResult::new(val, c.err / sx + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        let sx = x.sqrt();
        let c = cheb_eval_e(&AK02_CS, 16.0 / x - 1.0);
        let val = (1.25 + c.val) / sx;
        Ok(SfResult::new(val, (c.err + GSL_DBL_EPSILON) / sx + 2.0 * GSL_DBL_EPSILON * val.abs()))
    }
}

// Modified Bessel function K0
pub fn bessel_k0_e(x: f64) -> Result<SfResult, SfError> {
    if x <= 0.0 {
        Err(SfError::DomainError)
    } else if x < 1.0 {
        let lx = x.ln();
        let x2 = x * x;
        let val = poly_eval(&K0_POLY, x2) - lx * (1.0 + 0.25 * x2 * poly_eval(&I0_POLY, 0.25 * x2));
        let err = (1.6 + lx.abs() * 0.6) * GSL_DBL_EPSILON;
        Ok(SfResult::new(val, err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        let k0_scaled = bessel_k0_scaled_e(x)?;
        let ex = (-x).exp();
        let val = ex * k0_scaled.val;
        let err = ex * (k0_scaled.err + GSL_DBL_EPSILON * x.abs() * k0_scaled.val.abs());
        Ok(SfResult::new(val, err))
    }
}

// Convenience functions
pub fn bessel_k0_scaled(x: f64) -> f64 {
    bessel_k0_scaled_e(x).unwrap().val
}

pub fn bessel_k0(x: f64) -> f64 {
    bessel_k0_e(x).unwrap().val
}