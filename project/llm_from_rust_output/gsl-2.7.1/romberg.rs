use std::error::Error;
use std::fmt;
use std::mem;

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
    Singularity = 21,
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

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub type GslResult<T> = Result<T, GslError>;

pub struct GslFunction {
    function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn new<F: Fn(f64) -> f64 + 'static>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

pub struct RombergWorkspace {
    n: usize,
    work1: Vec<f64>,
    work2: Vec<f64>,
}

impl RombergWorkspace {
    pub fn new(n: usize) -> GslResult<Self> {
        if n < 1 {
            return Err(GslError::Domain);
        }

        let n = if n < 30 { n } else { 30 };

        Ok(RombergWorkspace {
            n,
            work1: vec![0.0; n],
            work2: vec![0.0; n],
        })
    }
}

pub fn romberg_integration(
    f: &GslFunction,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    workspace: &mut RombergWorkspace,
) -> GslResult<(f64, usize)> {
    if epsabs < 0.0 || epsrel < 0.0 {
        return Err(GslError::Domain);
    }

    let n = workspace.n;
    let mut rp = &mut workspace.work1[..n];
    let mut rc = &mut workspace.work2[..n];
    let mut neval = 2;
    let mut h = 0.5 * (b - a);

    rp[0] = h * (f.eval(a) + f.eval(b));

    for i in 1..n {
        let mut sum = 0.0;
        let two_i = 1 << i;
        
        for j in (1..two_i).step_by(2) {
            sum += f.eval(a + j as f64 * h);
            neval += 1;
        }

        rc[0] = sum * h + 0.5 * rp[0];

        let mut four_j = 4.0;
        for j in 1..=i {
            rc[j] = (four_j * rc[j - 1] - rp[j - 1]) / (four_j - 1.0);
            four_j *= 4.0;
        }

        let err = (rc[i] - rp[i - 1]).abs();
        if err < epsabs || err < epsrel * rc[i].abs() {
            return Ok((rc[i], neval));
        }

        mem::swap(&mut rp, &mut rc);
        h *= 0.5;
    }

    Err(GslError::MaxIter)
}