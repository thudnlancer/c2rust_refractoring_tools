use std::f64::consts::{PI, E};
use std::f64::{NAN, INFINITY};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

const BESSEL_J_SMALL: f64 = f64::MIN_POSITIVE / f64::EPSILON;

pub fn gsl_sf_bessel_j0_e(x: f64) -> Result<GslSfResult, GslError> {
    let ax = x.abs();
    if ax < 0.5 {
        let y = x * x;
        let c1 = -1.0 / 6.0;
        let c2 = 1.0 / 120.0;
        let c3 = -1.0 / 5040.0;
        let c4 = 1.0 / 362880.0;
        let c5 = -1.0 / 39916800.0;
        let c6 = 1.0 / 6227020800.0;
        let val = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * c6))));
        Ok(GslSfResult {
            val,
            err: 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let val = x.sin() / x;
        Ok(GslSfResult {
            val,
            err: 2.0 * f64::EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_bessel_j1_e(x: f64) -> Result<GslSfResult, GslError> {
    let ax = x.abs();
    if x == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if ax < 3.1 * f64::MIN_POSITIVE {
        Err(GslError::Underflow)
    } else if ax < 0.25 {
        let y = x * x;
        let c1 = -1.0 / 10.0;
        let c2 = 1.0 / 280.0;
        let c3 = -1.0 / 15120.0;
        let c4 = 1.0 / 1330560.0;
        let c5 = -1.0 / 172972800.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        let val = x / 3.0 * sum;
        Ok(GslSfResult {
            val,
            err: 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let cos_x = x.cos();
        let sin_x = x.sin();
        let val = (sin_x / x - cos_x) / x;
        let err = 2.0 * f64::EPSILON * ((sin_x / (x * x)).abs() + (cos_x / x).abs());
        Ok(GslSfResult {
            val,
            err: err + 2.0 * f64::EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_bessel_j2_e(x: f64) -> Result<GslSfResult, GslError> {
    let ax = x.abs();
    if x == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if ax < 4.0 * 1.4916681462400413e-154 {
        Err(GslError::Underflow)
    } else if ax < 1.3 {
        let y = x * x;
        let c1 = -1.0 / 14.0;
        let c2 = 1.0 / 504.0;
        let c3 = -1.0 / 33264.0;
        let c4 = 1.0 / 3459456.0;
        let c5 = -1.0 / 518918400.0;
        let c6 = 1.0 / 105859353600.0;
        let c7 = -1.0 / 28158588057600.0;
        let c8 = 1.0 / 9461285587353600.0;
        let c9 = -1.0 / 3916972233164390400.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * (c7 + y * (c8 + y * c9))))))));
        let val = y / 15.0 * sum;
        Ok(GslSfResult {
            val,
            err: 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let cos_x = x.cos();
        let sin_x = x.sin();
        let f = 3.0 / (x * x) - 1.0;
        let val = (f * sin_x - 3.0 * cos_x / x) / x;
        let err = 2.0 * f64::EPSILON * ((f * sin_x / x).abs() + 3.0 * (cos_x / (x * x)).abs());
        Ok(GslSfResult {
            val,
            err: err + 2.0 * f64::EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_bessel_jl_e(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    if l < 0 || x < 0.0 {
        Err(GslError::Domain)
    } else if x == 0.0 {
        Ok(GslSfResult {
            val: if l > 0 { 0.0 } else { 1.0 },
            err: 0.0,
        })
    } else {
        match l {
            0 => gsl_sf_bessel_j0_e(x),
            1 => gsl_sf_bessel_j1_e(x),
            2 => gsl_sf_bessel_j2_e(x),
            _ => {
                let nu = l as f64 + 0.5;
                if x * x < 10.0 * (l as f64 + 0.5) / E {
                    let mut b = GslSfResult { val: 0.0, err: 0.0 };
                    let status = gsl_sf_bessel_IJ_taylor_e(nu, x, -1, 50, f64::EPSILON, &mut b)?;
                    let pre = (0.5 * PI / x).sqrt();
                    Ok(GslSfResult {
                        val: pre * b.val,
                        err: pre * b.err + 2.0 * f64::EPSILON * (pre * b.val).abs(),
                    })
                } else if 1.220703125e-4 * x > (l * l + l) as f64 + 1.0 {
                    let mut b = GslSfResult { val: 0.0, err: 0.0 };
                    let status = gsl_sf_bessel_Jnu_asympx_e(nu, x, &mut b)?;
                    let pre = (0.5 * PI / x).sqrt();
                    Ok(GslSfResult {
                        val: pre * b.val,
                        err: 2.0 * f64::EPSILON * (pre * b.val).abs() + pre * b.err,
                    })
                } else if nu > 1.0 / 2.4607833005759251e-3 {
                    let mut b = GslSfResult { val: 0.0, err: 0.0 };
                    let status = gsl_sf_bessel_Jnu_asymp_Olver_e(nu, x, &mut b)?;
                    let pre = (0.5 * PI / x).sqrt();
                    Ok(GslSfResult {
                        val: pre * b.val,
                        err: 2.0 * f64::EPSILON * (pre * b.val).abs() + pre * b.err,
                    })
                } else if x > 1000.0 && x > (l * l) as f64 {
                    let mut b = GslSfResult { val: 0.0, err: 0.0 };
                    let status = gsl_sf_bessel_Jnu_asympx_e(nu, x, &mut b)?;
                    let pre = (0.5 * PI / x).sqrt();
                    Ok(GslSfResult {
                        val: pre * b.val,
                        err: 2.0 * f64::EPSILON * (pre * b.val).abs() + pre * b.err,
                    })
                } else {
                    let (ratio, sgn) = gsl_sf_bessel_J_CF1(nu, x)?;
                    let mut jellp1 = BESSEL_J_SMALL * ratio;
                    let mut jell = BESSEL_J_SMALL;
                    let mut jellm1;
                    let mut ell = l;
                    while ell > 0 {
                        jellm1 = -jellp1 + (2 * ell + 1) as f64 / x * jell;
                        jellp1 = jell;
                        jell = jellm1;
                        ell -= 1;
                    }
                    if jell.abs() > jellp1.abs() {
                        let j0_result = gsl_sf_bessel_j0_e(x)?;
                        let pre = BESSEL_J_SMALL / jell;
                        Ok(GslSfResult {
                            val: j0_result.val * pre,
                            err: j0_result.err * pre.abs() + 4.0 * f64::EPSILON * (0.5 * l as f64 + 1.0) * (j0_result.val * pre).abs(),
                        })
                    } else {
                        let j1_result = gsl_sf_bessel_j1_e(x)?;
                        let pre = BESSEL_J_SMALL / jellp1;
                        Ok(GslSfResult {
                            val: j1_result.val * pre,
                            err: j1_result.err * pre.abs() + 4.0 * f64::EPSILON * (0.5 * l as f64 + 1.0) * (j1_result.val * pre).abs(),
                        })
                    }
                }
            }
        }
    }
}

// Helper functions would need to be implemented similarly
fn gsl_sf_bessel_IJ_taylor_e(nu: f64, x: f64, sign: i32, kmax: i32, threshold: f64, result: &mut GslSfResult) -> Result<(), GslError> {
    // Implementation would go here
    Ok(())
}

fn gsl_sf_bessel_Jnu_asympx_e(nu: f64, x: f64, result: &mut GslSfResult) -> Result<(), GslError> {
    // Implementation would go here
    Ok(())
}

fn gsl_sf_bessel_J_CF1(nu: f64, x: f64) -> Result<(f64, f64), GslError> {
    // Implementation would go here
    Ok((0.0, 0.0))
}

fn gsl_sf_bessel_Jnu_asymp_Olver_e(nu: f64, x: f64, result: &mut GslSfResult) -> Result<(), GslError> {
    // Implementation would go here
    Ok(())
}