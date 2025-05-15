use std::f64;
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub struct GslRngType {
    name: &'static str,
    max: u64,
    min: u64,
    size: usize,
    set: Option<fn(&mut [u8], u64)>,
    get: Option<fn(&mut [u8]) -> u64>,
    get_double: Option<fn(&mut [u8]) -> f64>,
}

pub struct GslRng {
    type_: &'static GslRngType,
    state: Vec<u8>,
}

pub struct GslMonteFunction {
    f: fn(&[f64], usize, &mut dyn std::any::Any) -> f64,
    dim: usize,
    params: Box<dyn std::any::Any>,
}

pub struct GslMontePlainState {
    dim: usize,
    x: Vec<f64>,
}

impl GslMontePlainState {
    pub fn new(dim: usize) -> Result<Self, GslError> {
        if dim == 0 {
            return Err(GslError::Invalid);
        }
        Ok(Self {
            dim,
            x: vec![0.0; dim],
        })
    }

    pub fn init(&mut self) -> Result<(), GslError> {
        for x in &mut self.x {
            *x = 0.0;
        }
        Ok(())
    }
}

pub fn gsl_monte_plain_integrate(
    f: &GslMonteFunction,
    xl: &[f64],
    xu: &[f64],
    calls: usize,
    rng: &mut GslRng,
    state: &mut GslMontePlainState,
) -> Result<(f64, f64), GslError> {
    if state.dim != f.dim {
        return Err(GslError::Invalid);
    }

    if xl.len() != state.dim || xu.len() != state.dim {
        return Err(GslError::Invalid);
    }

    for i in 0..state.dim {
        if xu[i] <= xl[i] {
            return Err(GslError::Invalid);
        }
        if xu[i] - xl[i] > f64::MAX {
            return Err(GslError::Invalid);
        }
    }

    let vol = xl.iter()
        .zip(xu.iter())
        .fold(1.0, |acc, (&l, &u)| acc * (u - l));

    let mut m = 0.0;
    let mut q = 0.0;

    for _ in 0..calls {
        let x: Vec<f64> = (0..state.dim)
            .map(|i| xl[i] + rng.uniform_pos() * (xu[i] - xl[i]))
            .collect();

        let fval = (f.f)(&x, f.dim, &mut *f.params);
        let d = fval - m;
        m += d / (calls as f64);
        q += d * d * ((calls - 1) as f64 / calls as f64);
    }

    let result = vol * m;
    let abserr = if calls < 2 {
        f64::INFINITY
    } else {
        vol * (q / (calls as f64 * (calls as f64 - 1.0))).sqrt()
    };

    Ok((result, abserr))
}

impl GslRng {
    pub fn uniform_pos(&mut self) -> f64 {
        loop {
            let x = (self.type_.get_double.unwrap())(&mut self.state);
            if x != 0.0 {
                return x;
            }
        }
    }
}