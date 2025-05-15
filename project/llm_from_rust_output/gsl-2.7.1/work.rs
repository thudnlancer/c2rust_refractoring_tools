use std::ptr::NonNull;
use std::mem::size_of;
use std::ffi::CString;

pub type size_t = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
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

#[derive(Debug)]
pub struct GslBlock {
    size: size_t,
    data: Vec<f64>,
}

#[derive(Debug)]
pub struct GslVector {
    size: size_t,
    stride: size_t,
    data: Vec<f64>,
    block: GslBlock,
    owner: bool,
}

#[derive(Debug)]
pub struct GslMatrix {
    size1: size_t,
    size2: size_t,
    tda: size_t,
    data: Vec<f64>,
    block: GslBlock,
    owner: bool,
}

#[derive(Debug)]
pub struct GslMultifitLinearWorkspace {
    nmax: size_t,
    pmax: size_t,
    n: size_t,
    p: size_t,
    a: GslMatrix,
    q: GslMatrix,
    qsi: GslMatrix,
    s: GslVector,
    t: GslVector,
    xt: GslVector,
    d: GslVector,
    rcond: f64,
}

impl GslMultifitLinearWorkspace {
    pub fn new(nmax: size_t, pmax: size_t) -> Result<Self, GslError> {
        if nmax == 0 || pmax == 0 {
            return Err(GslError::Invalid);
        }

        let a = GslMatrix::new(nmax, pmax).map_err(|_| GslError::NoMem)?;
        let q = GslMatrix::new(pmax, pmax).map_err(|_| GslError::NoMem)?;
        let qsi = GslMatrix::new(pmax, pmax).map_err(|_| GslError::NoMem)?;
        let s = GslVector::new(pmax).map_err(|_| GslError::NoMem)?;
        let t = GslVector::new(nmax).map_err(|_| GslError::NoMem)?;
        let xt = GslVector::with_capacity(pmax).map_err(|_| GslError::NoMem)?;
        let d = GslVector::with_capacity(pmax).map_err(|_| GslError::NoMem)?;

        Ok(Self {
            nmax,
            pmax,
            n: 0,
            p: 0,
            a,
            q,
            qsi,
            s,
            t,
            xt,
            d,
            rcond: 0.0,
        })
    }
}

impl GslMatrix {
    pub fn new(size1: size_t, size2: size_t) -> Result<Self, GslError> {
        if size1 == 0 || size2 == 0 {
            return Err(GslError::Invalid);
        }

        let tda = size2;
        let total_size = size1 * size2;
        let block = GslBlock::new(total_size)?;
        
        Ok(Self {
            size1,
            size2,
            tda,
            data: vec![0.0; total_size],
            block,
            owner: true,
        })
    }
}

impl GslVector {
    pub fn new(size: size_t) -> Result<Self, GslError> {
        if size == 0 {
            return Err(GslError::Invalid);
        }

        let block = GslBlock::new(size)?;
        
        Ok(Self {
            size,
            stride: 1,
            data: vec![0.0; size],
            block,
            owner: true,
        })
    }

    pub fn with_capacity(size: size_t) -> Result<Self, GslError> {
        if size == 0 {
            return Err(GslError::Invalid);
        }

        let block = GslBlock::new(size)?;
        
        Ok(Self {
            size,
            stride: 1,
            data: Vec::with_capacity(size),
            block,
            owner: true,
        })
    }
}

impl GslBlock {
    pub fn new(size: size_t) -> Result<Self, GslError> {
        if size == 0 {
            return Err(GslError::Invalid);
        }

        Ok(Self {
            size,
            data: vec![0.0; size],
        })
    }
}

fn gsl_error(reason: &str, file: &str, line: i32, gsl_errno: GslError) {
    eprintln!("GSL error: {} at {}:{} (code {})", reason, file, line, gsl_errno as i32);
}