use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
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

#[derive(Debug, Clone)]
pub struct GslBlock {
    pub size: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: usize,
    pub stride: usize,
    pub data: Vec<f64>,
    pub block: Option<GslBlock>,
    pub owner: bool,
}

impl GslVector {
    pub fn get(&self, i: usize) -> f64 {
        self.data[i * self.stride]
    }
}

pub fn gsl_multiroot_test_delta(
    dx: &GslVector,
    x: &GslVector,
    epsabs: f64,
    epsrel: f64,
) -> Result<(), GslError> {
    if epsrel < 0.0 {
        return Err(GslError::BadTol);
    }

    for i in 0..x.size {
        let xi = x.get(i);
        let dxi = dx.get(i);
        let tolerance = epsabs + epsrel * xi.abs();

        if dxi.abs() >= tolerance && dxi != 0.0 {
            return Err(GslError::Continue);
        }
    }

    Ok(())
}

pub fn gsl_multiroot_test_residual(f: &GslVector, epsabs: f64) -> Result<(), GslError> {
    if epsabs < 0.0 {
        return Err(GslError::BadTol);
    }

    let residual = f.data.iter().map(|&fi| fi.abs()).sum::<f64>();

    if residual < epsabs {
        Ok(())
    } else {
        Err(GslError::Continue)
    }
}