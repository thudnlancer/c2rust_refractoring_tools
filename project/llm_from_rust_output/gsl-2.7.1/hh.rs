use std::f64;
use std::ptr;

#[derive(Debug, Clone, Copy)]
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
    pub block: Option<Box<GslBlock>>,
    pub owner: i32,
}

impl GslVector {
    pub fn get(&self, i: usize) -> f64 {
        self.data[i * self.stride]
    }

    pub fn set(&mut self, i: usize, x: f64) {
        self.data[i * self.stride] = x;
    }
}

#[derive(Debug, Clone)]
pub struct GslMatrix {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: Vec<f64>,
    pub block: Option<Box<GslBlock>>,
    pub owner: i32,
}

impl GslMatrix {
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.tda + j]
    }

    pub fn set(&mut self, i: usize, j: usize, x: f64) {
        self.data[i * self.tda + j] = x;
    }
}

pub fn gsl_linalg_hh_solve(a: &mut GslMatrix, b: &GslVector, x: &mut GslVector) -> Result<(), GslError> {
    if a.size1 > a.size2 {
        return Err(GslError::Invalid);
    } else if a.size2 != x.size {
        return Err(GslError::BadLen);
    }

    x.data.copy_from_slice(&b.data);
    gsl_linalg_hh_svx(a, x)
}

pub fn gsl_linalg_hh_svx(a: &mut GslMatrix, x: &mut GslVector) -> Result<(), GslError> {
    if a.size1 > a.size2 {
        return Err(GslError::Invalid);
    } else if a.size2 != x.size {
        return Err(GslError::BadLen);
    }

    let n = a.size1;
    let m = a.size2;
    let mut d = vec![0.0; n];

    for i in 0..n {
        let aii = a.get(i, i);
        let mut alpha = 0.0;
        let mut f = 0.0;
        let mut ak = 0.0;
        let mut max_norm = 0.0;
        let mut r = 0.0;

        for k in i..m {
            let aki = a.get(k, i);
            r += aki * aki;
        }

        if r == 0.0 {
            return Err(GslError::Singular);
        }

        alpha = r.sqrt() * if aii >= 0.0 { 1.0 } else { -1.0 };
        ak = 1.0 / (r + alpha * aii);
        a.set(i, i, aii + alpha);
        d[i] = -alpha;

        for k in (i + 1)..n {
            let mut norm = 0.0;
            f = 0.0;

            for j in i..m {
                let ajk = a.get(j, k);
                let aji = a.get(j, i);
                norm += ajk * ajk;
                f += ajk * aji;
            }

            max_norm = max_norm.max(norm);
            f *= ak;

            for j in i..m {
                let ajk = a.get(j, k);
                let aji = a.get(j, i);
                a.set(j, k, ajk - f * aji);
            }
        }

        if alpha.abs() < 2.0 * f64::EPSILON * max_norm.sqrt() {
            return Err(GslError::Singular);
        }

        f = 0.0;
        for j in i..m {
            f += x.get(j) * a.get(j, i);
        }
        f *= ak;

        for j in i..m {
            let xj = x.get(j);
            let aji = a.get(j, i);
            x.set(j, xj - f * aji);
        }
    }

    for i in (0..n).rev() {
        let xi = x.get(i);
        let mut sum = 0.0;

        for k in (i + 1)..n {
            sum += a.get(i, k) * x.get(k);
        }

        x.set(i, (xi - sum) / d[i]);
    }

    Ok(())
}