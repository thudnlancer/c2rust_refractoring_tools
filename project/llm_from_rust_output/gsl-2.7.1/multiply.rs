use std::error::Error;
use std::fmt;

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

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug, Clone)]
pub struct GslMatrix {
    size1: usize,
    size2: usize,
    tda: usize,
    data: Vec<f64>,
}

impl GslMatrix {
    pub fn new(size1: usize, size2: usize) -> Self {
        let tda = size2;
        let data = vec![0.0; size1 * size2];
        GslMatrix { size1, size2, tda, data }
    }

    pub fn get(&self, i: usize, j: usize) -> Result<f64, GslError> {
        if i >= self.size1 || j >= self.size2 {
            return Err(GslError::Invalid);
        }
        Ok(self.data[i * self.tda + j])
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) -> Result<(), GslError> {
        if i >= self.size1 || j >= self.size2 {
            return Err(GslError::Invalid);
        }
        self.data[i * self.tda + j] = value;
        Ok(())
    }

    pub fn size1(&self) -> usize {
        self.size1
    }

    pub fn size2(&self) -> usize {
        self.size2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixMod {
    None = 0,
    Transpose = 1,
    Conjugate = 2,
}

pub fn matmult(a: &GslMatrix, b: &GslMatrix, c: &mut GslMatrix) -> Result<(), GslError> {
    if a.size2() != b.size1() || a.size1() != c.size1() || b.size2() != c.size2() {
        return Err(GslError::BadLen);
    }

    for i in 0..c.size1() {
        for j in 0..c.size2() {
            let mut temp = a.get(i, 0)? * b.get(0, j)?;
            for k in 1..a.size2() {
                temp += a.get(i, k)? * b.get(k, j)?;
            }
            c.set(i, j, temp)?;
        }
    }

    Ok(())
}

pub fn matmult_mod(
    a: &GslMatrix,
    mod_a: MatrixMod,
    b: &GslMatrix,
    mod_b: MatrixMod,
    c: &mut GslMatrix,
) -> Result<(), GslError> {
    if mod_a == MatrixMod::None && mod_b == MatrixMod::None {
        return matmult(a, b, c);
    }

    let (dim1_a, dim2_a) = if mod_a == MatrixMod::Transpose {
        (a.size2(), a.size1())
    } else {
        (a.size1(), a.size2())
    };

    let (dim1_b, dim2_b) = if mod_b == MatrixMod::Transpose {
        (b.size2(), b.size1())
    } else {
        (b.size1(), b.size2())
    };

    if dim2_a != dim1_b || dim1_a != c.size1() || dim2_b != c.size2() {
        return Err(GslError::BadLen);
    }

    for i in 0..c.size1() {
        for j in 0..c.size2() {
            let (mut a1, mut a2) = (i, 0);
            let (mut b1, mut b2) = (0, j);

            if mod_a == MatrixMod::Transpose {
                std::mem::swap(&mut a1, &mut a2);
            }
            if mod_b == MatrixMod::Transpose {
                std::mem::swap(&mut b1, &mut b2);
            }

            let mut temp = a.get(a1, a2)? * b.get(b1, b2)?;

            for k in 1..dim2_a {
                let (mut a1, mut a2) = (i, k);
                let (mut b1, mut b2) = (k, j);

                if mod_a == MatrixMod::Transpose {
                    std::mem::swap(&mut a1, &mut a2);
                }
                if mod_b == MatrixMod::Transpose {
                    std::mem::swap(&mut b1, &mut b2);
                }

                temp += a.get(a1, a2)? * b.get(b1, b2)?;
            }

            c.set(i, j, temp)?;
        }
    }

    Ok(())
}