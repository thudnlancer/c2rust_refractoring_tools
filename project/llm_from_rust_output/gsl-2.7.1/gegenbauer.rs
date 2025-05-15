use std::f64::consts::PI;
use std::f64::NAN;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

impl GslSfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

pub fn gsl_sf_gegenpoly_1_e(lambda: f64, x: f64, result: &mut GslSfResult) -> GslError {
    if lambda == 0.0 {
        *result = GslSfResult::new(2.0 * x, 2.0 * f64::EPSILON * result.val.abs());
    } else {
        *result = GslSfResult::new(2.0 * lambda * x, 4.0 * f64::EPSILON * result.val.abs());
    }
    GslError::Success
}

pub fn gsl_sf_gegenpoly_2_e(lambda: f64, x: f64, result: &mut GslSfResult) -> GslError {
    if lambda == 0.0 {
        let txx = 2.0 * x * x;
        *result = GslSfResult::new(
            -1.0 + txx,
            2.0 * f64::EPSILON * (txx.abs() + result.val.abs()),
        );
    } else {
        *result = GslSfResult::new(
            lambda * (-1.0 + 2.0 * (1.0 + lambda) * x * x),
            f64::EPSILON * (2.0 * result.val.abs() + lambda.abs()),
        );
    }
    GslError::Success
}

pub fn gsl_sf_gegenpoly_3_e(lambda: f64, x: f64, result: &mut GslSfResult) -> GslError {
    if lambda == 0.0 {
        *result = GslSfResult::new(
            x * (-2.0 + 4.0 / 3.0 * x * x),
            f64::EPSILON * (2.0 * result.val.abs() + x.abs()),
        );
    } else {
        let c = 4.0 + lambda * (6.0 + 2.0 * lambda);
        *result = GslSfResult::new(
            2.0 * lambda * x * (-1.0 - lambda + c * x * x / 3.0),
            f64::EPSILON * (2.0 * result.val.abs() + (lambda * x).abs()),
        );
    }
    GslError::Success
}

pub fn gsl_sf_gegenpoly_n_e(n: i32, lambda: f64, x: f64, result: &mut GslSfResult) -> GslError {
    if lambda <= -0.5 || n < 0 {
        *result = GslSfResult::new(NAN, NAN);
        return GslError::Domain;
    }

    match n {
        0 => {
            *result = GslSfResult::new(1.0, 0.0);
            GslError::Success
        }
        1 => gsl_sf_gegenpoly_1_e(lambda, x, result),
        2 => gsl_sf_gegenpoly_2_e(lambda, x, result),
        3 => gsl_sf_gegenpoly_3_e(lambda, x, result),
        _ => {
            if lambda == 0.0 && (-1.0..=1.0).contains(&x) {
                let z = n as f64 * x.acos();
                *result = GslSfResult::new(
                    2.0 * z.cos() / n as f64,
                    2.0 * f64::EPSILON * (z * result.val).abs(),
                );
                GslError::Success
            } else {
                let mut g2 = GslSfResult::new(0.0, 0.0);
                let mut g3 = GslSfResult::new(0.0, 0.0);
                let stat_g2 = gsl_sf_gegenpoly_2_e(lambda, x, &mut g2);
                let stat_g3 = gsl_sf_gegenpoly_3_e(lambda, x, &mut g3);
                let stat_g = if stat_g2 != GslError::Success {
                    stat_g2
                } else if stat_g3 != GslError::Success {
                    stat_g3
                } else {
                    GslError::Success
                };

                let mut gkm2 = g2.val;
                let mut gkm1 = g3.val;
                let mut gk = 0.0;
                for k in 4..=n {
                    gk = (2.0 * (k as f64 + lambda - 1.0) * x * gkm1
                        - (k as f64 + 2.0 * lambda - 2.0) * gkm2)
                        / k as f64;
                    gkm2 = gkm1;
                    gkm1 = gk;
                }
                *result = GslSfResult::new(
                    gk,
                    2.0 * f64::EPSILON * 0.5 * n as f64 * gk.abs(),
                );
                stat_g
            }
        }
    }
}

pub fn gsl_sf_gegenpoly_array(nmax: i32, lambda: f64, x: f64, result_array: &mut [f64]) -> GslError {
    if lambda <= -0.5 || nmax < 0 {
        return GslError::Domain;
    }

    if nmax >= 0 {
        result_array[0] = 1.0;
    }

    if nmax == 0 {
        return GslError::Success;
    }

    if nmax >= 1 {
        result_array[1] = if lambda == 0.0 {
            2.0 * x
        } else {
            2.0 * lambda * x
        };
    }

    for k in 2..=nmax {
        let term1 = 2.0 * (k as f64 + lambda - 1.0) * x * result_array[(k - 1) as usize];
        let term2 = (k as f64 + 2.0 * lambda - 2.0) * result_array[(k - 2) as usize];
        result_array[k as usize] = (term1 - term2) / k as f64;
    }

    GslError::Success
}

pub fn gsl_sf_gegenpoly_1(lambda: f64, x: f64) -> f64 {
    let mut result = GslSfResult::new(0.0, 0.0);
    gsl_sf_gegenpoly_1_e(lambda, x, &mut result).val
}

pub fn gsl_sf_gegenpoly_2(lambda: f64, x: f64) -> f64 {
    let mut result = GslSfResult::new(0.0, 0.0);
    gsl_sf_gegenpoly_2_e(lambda, x, &mut result).val
}

pub fn gsl_sf_gegenpoly_3(lambda: f64, x: f64) -> f64 {
    let mut result = GslSfResult::new(0.0, 0.0);
    gsl_sf_gegenpoly_3_e(lambda, x, &mut result).val
}

pub fn gsl_sf_gegenpoly_n(n: i32, lambda: f64, x: f64) -> f64 {
    let mut result = GslSfResult::new(0.0, 0.0);
    gsl_sf_gegenpoly_n_e(n, lambda, x, &mut result).val
}