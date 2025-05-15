use std::f64::consts::E;

#[derive(Debug, Copy, Clone)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
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

fn max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn halley_iteration(x: f64, w_initial: f64, max_iters: u32) -> Result<SfResult, GslError> {
    let mut w = w_initial;
    
    for _ in 0..max_iters {
        let e = w.exp();
        let p = w + 1.0;
        let mut t = w * e - x;
        
        if w > 0.0 {
            t /= p * e;
        } else {
            t /= e * p - 0.5 * (p + 1.0) * t / p;
        }
        
        w -= t;
        let tol = 10.0 * f64::EPSILON * max_dbl(w.abs(), 1.0 / (p.abs() * e));
        
        if t.abs() < tol {
            return Ok(SfResult {
                val: w,
                err: 2.0 * tol,
            });
        }
    }
    
    Err(GslError::MaxIter)
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

pub fn lambert_w0_e(x: f64) -> Result<SfResult, GslError> {
    let one_over_e = 1.0 / E;
    let q = x + one_over_e;
    
    if x == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else if q < 0.0 {
        Err(GslError::Domain)
    } else if q == 0.0 {
        Ok(SfResult {
            val: -1.0,
            err: f64::EPSILON,
        })
    } else if q < 1.0e-03 {
        let r = q.sqrt();
        let val = series_eval(r);
        Ok(SfResult {
            val,
            err: 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        const MAX_ITERS: u32 = 10;
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
        
        halley_iteration(x, w, MAX_ITERS)
    }
}

pub fn lambert_wm1_e(x: f64) -> Result<SfResult, GslError> {
    if x > 0.0 {
        lambert_w0_e(x)
    } else if x == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else {
        const MAX_ITERS: u32 = 32;
        let one_over_e = 1.0 / E;
        let q = x + one_over_e;
        
        if q < 0.0 {
            return Err(GslError::Domain);
        }
        
        let w = if x < -1.0e-6 {
            let r = (-q).sqrt();
            let w = series_eval(r);
            if q < 3.0e-3 {
                return Ok(SfResult {
                    val: w,
                    err: 5.0 * f64::EPSILON * w.abs(),
                });
            }
            w
        } else {
            let l1 = (-x).ln();
            let l2 = (-l1).ln();
            l1 - l2 + l2 / l1
        };
        
        halley_iteration(x, w, MAX_ITERS)
    }
}

pub fn lambert_w0(x: f64) -> f64 {
    match lambert_w0_e(x) {
        Ok(result) => result.val,
        Err(e) => {
            eprintln!("Error in lambert_w0_e: {:?}", e);
            f64::NAN
        }
    }
}

pub fn lambert_wm1(x: f64) -> f64 {
    match lambert_wm1_e(x) {
        Ok(result) => result.val,
        Err(e) => {
            eprintln!("Error in lambert_wm1_e: {:?}", e);
            f64::NAN
        }
    }
}