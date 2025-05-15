use std::f64::consts::E;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfError {
    Domain,
    MaxIteration,
    Success,
}

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const MAX_ITERS_W0: usize = 10;
const MAX_ITERS_WM1: usize = 32;

fn halley_iteration(x: f64, w_initial: f64, max_iters: usize) -> Result<SfResult, SfError> {
    let mut w = w_initial;

    for _ in 0..max_iters {
        let e = w.exp();
        let p = w + 1.0;
        let mut t = w * e - x;

        if w > 0.0 {
            t = (t / p) / e; // Newton iteration
        } else {
            t /= e * p - 0.5 * (p + 1.0) * t / p; // Halley iteration
        }

        w -= t;

        let tol = 10.0 * GSL_DBL_EPSILON * f64::max(w.abs(), 1.0 / (p.abs() * e));

        if t.abs() < tol {
            return Ok(SfResult {
                val: w,
                err: 2.0 * tol,
            });
        }
    }

    Err(SfError::MaxIteration)
}

fn series_eval(r: f64) -> f64 {
    const C: [f64; 12] = [
        -1.0,
        2.331643981597124203363536062168,
        -1.812187885639363490240191647568,
        1.936631114492359755363277457668,
        -2.353551201881614516821543561516,
        3.066858901050631912893148922704,
        -4.175335600258177138854984177460,
        5.858023729874774148815053846119,
        -8.401032217523977370984161688514,
        12.250753501314460424,
        -18.100697012472442755,
        27.029044799010561650,
    ];

    let t_8 = C[8] + r * (C[9] + r * (C[10] + r * C[11]));
    let t_5 = C[5] + r * (C[6] + r * (C[7] + r * t_8));
    let t_1 = C[1] + r * (C[2] + r * (C[3] + r * (C[4] + r * t_5)));
    C[0] + r * t_1
}

pub fn lambert_w0_e(x: f64) -> Result<SfResult, SfError> {
    let one_over_e = 1.0 / E;
    let q = x + one_over_e;

    if x == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else if q < 0.0 {
        Err(SfError::Domain)
    } else if q == 0.0 {
        Ok(SfResult {
            val: -1.0,
            err: GSL_DBL_EPSILON,
        })
    } else if q < 1.0e-03 {
        let r = q.sqrt();
        let val = series_eval(r);
        Ok(SfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        let w = if x < 1.0 {
            let p = (2.0 * E * q).sqrt();
            -1.0 + p * (1.0 + p * (-1.0 / 3.0 + p * 11.0 / 72.0))
        } else {
            let mut w = x.ln();
            if x > 3.0 {
                w -= w.ln();
            }
            w
        };

        halley_iteration(x, w, MAX_ITERS_W0)
    }
}

pub fn lambert_wm1_e(x: f64) -> Result<SfResult, SfError> {
    if x > 0.0 {
        lambert_w0_e(x)
    } else if x == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else {
        let one_over_e = 1.0 / E;
        let q = x + one_over_e;

        if q < 0.0 {
            Err(SfError::Domain)
        } else if x < -1.0e-6 {
            let r = (-q).sqrt();
            let w = series_eval(r);
            if q < 3.0e-3 {
                Ok(SfResult {
                    val: w,
                    err: 5.0 * GSL_DBL_EPSILON * w.abs(),
                })
            } else {
                halley_iteration(x, w, MAX_ITERS_WM1)
            }
        } else {
            let l1 = (-x).ln();
            let l2 = (-l1).ln();
            let w = l1 - l2 + l2 / l1;
            halley_iteration(x, w, MAX_ITERS_WM1)
        }
    }
}

pub fn lambert_w0(x: f64) -> f64 {
    match lambert_w0_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

pub fn lambert_wm1(x: f64) -> f64 {
    match lambert_wm1_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}