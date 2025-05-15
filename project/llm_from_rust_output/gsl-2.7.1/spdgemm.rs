use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Domain,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProgress,
    NoProgressJ,
    TolF,
    TolX,
    TolG,
    Eof,
    BadLenMatrix,
    InvalidFormat,
    InvalidStorage,
    Other(i32),
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GslError::Domain => write!(f, "input domain error"),
            GslError::Range => write!(f, "output range error"),
            GslError::Fault => write!(f, "invalid pointer"),
            GslError::Invalid => write!(f, "invalid argument"),
            GslError::Failed => write!(f, "generic failure"),
            GslError::Factor => write!(f, "factorization failed"),
            GslError::Sanity => write!(f, "sanity check failed"),
            GslError::NoMem => write!(f, "malloc failed"),
            GslError::BadFunc => write!(f, "problem with user-supplied function"),
            GslError::Runaway => write!(f, "iterative process is out of control"),
            GslError::MaxIter => write!(f, "exceeded max number of iterations"),
            GslError::ZeroDiv => write!(f, "tried to divide by zero"),
            GslError::BadTol => write!(f, "invalid tolerance"),
            GslError::Tol => write!(f, "failed to reach tolerance"),
            GslError::Underflow => write!(f, "underflow"),
            GslError::Overflow => write!(f, "overflow"),
            GslError::Loss => write!(f, "loss of accuracy"),
            GslError::Round => write!(f, "failed because of roundoff error"),
            GslError::BadLen => write!(f, "matrix/vector sizes are not conformant"),
            GslError::NotSquare => write!(f, "matrix must be square"),
            GslError::Singular => write!(f, "apparent singularity detected"),
            GslError::Diverge => write!(f, "integral or series is divergent"),
            GslError::Unsupported => write!(f, "requested feature is not supported"),
            GslError::Unimplemented => write!(f, "requested feature not implemented"),
            GslError::Cache => write!(f, "cache limit exceeded"),
            GslError::Table => write!(f, "table limit exceeded"),
            GslError::NoProgress => write!(f, "iteration is not making progress"),
            GslError::NoProgressJ => write!(f, "jacobian evaluations not improving"),
            GslError::TolF => write!(f, "cannot reach tolerance in F"),
            GslError::TolX => write!(f, "cannot reach tolerance in X"),
            GslError::TolG => write!(f, "cannot reach tolerance in gradient"),
            GslError::Eof => write!(f, "end of file"),
            GslError::BadLenMatrix => write!(f, "matrix dimensions do not match"),
            GslError::InvalidFormat => write!(f, "matrix storage formats do not match"),
            GslError::InvalidStorage => write!(f, "compressed column format required"),
            GslError::Other(code) => write!(f, "other error (code {})", code),
        }
    }
}

impl Error for GslError {}

#[derive(Debug, Clone, Copy)]
pub enum SparseMatrixType {
    COO,
    CSC,
    CSR,
}

#[derive(Debug)]
pub struct SparseMatrix {
    size1: usize,
    size2: usize,
    i: Vec<i32>,
    data: Vec<f64>,
    p: Vec<i32>,
    nzmax: usize,
    nz: usize,
    sptype: SparseMatrixType,
}

impl SparseMatrix {
    pub fn new(size1: usize, size2: usize, sptype: SparseMatrixType) -> Self {
        SparseMatrix {
            size1,
            size2,
            i: Vec::new(),
            data: Vec::new(),
            p: Vec::new(),
            nzmax: 0,
            nz: 0,
            sptype,
        }
    }

    pub fn realloc(&mut self, nzmax: usize) -> Result<(), GslError> {
        if nzmax < self.nz {
            return Err(GslError::Invalid);
        }
        self.i.resize(nzmax, 0);
        self.data.resize(nzmax, 0.0);
        self.nzmax = nzmax;
        Ok(())
    }

    pub fn scale(&mut self, x: f64) -> Result<(), GslError> {
        for val in &mut self.data {
            *val *= x;
        }
        Ok(())
    }
}

pub fn spblas_dgemv(
    alpha: f64,
    a: &SparseMatrix,
    x: &[f64],
    beta: f64,
    y: &mut [f64],
) -> Result<(), GslError> {
    if a.size2 != x.len() || a.size1 != y.len() {
        return Err(GslError::BadLenMatrix);
    }

    if !matches!(a.sptype, SparseMatrixType::CSC) {
        return Err(GslError::InvalidStorage);
    }

    // Scale y by beta
    for val in y.iter_mut() {
        *val *= beta;
    }

    // Compute y = alpha*A*x + y
    for j in 0..a.size2 {
        for p in a.p[j]..a.p[j + 1] {
            let i = a.i[p as usize] as usize;
            y[i] += alpha * a.data[p as usize] * x[j];
        }
    }

    Ok(())
}

pub fn spblas_dgemm(
    alpha: f64,
    a: &SparseMatrix,
    b: &SparseMatrix,
    c: &mut SparseMatrix,
) -> Result<(), GslError> {
    if a.size2 != b.size1 || a.size1 != c.size1 || b.size2 != c.size2 {
        return Err(GslError::BadLenMatrix);
    }

    if a.sptype != b.sptype || a.sptype != c.sptype {
        return Err(GslError::InvalidFormat);
    }

    if !matches!(a.sptype, SparseMatrixType::CSC) {
        return Err(GslError::InvalidStorage);
    }

    let m = a.size1;
    let n = b.size2;
    let mut w = vec![0; m];
    let mut x = vec![0.0; m];
    let mut nz = 0;

    if c.nzmax < a.nz + b.nz {
        c.realloc(a.nz + b.nz)?;
    }

    c.p.resize(n + 1, 0);

    for j in 0..n {
        c.p[j] = nz as i32;
        
        for p in b.p[j]..b.p[j + 1] {
            let k = b.i[p as usize] as usize;
            nz = spblas_scatter(a, k, b.data[p as usize], &mut w, &mut x, (j + 1) as i32, c, nz)?;
        }

        for p in c.p[j]..nz as i32 {
            c.data[p as usize] = x[c.i[p as usize] as usize];
        }
    }

    c.p[n] = nz as i32;
    c.nz = nz;
    c.scale(alpha)?;

    Ok(())
}

fn spblas_scatter(
    a: &SparseMatrix,
    j: usize,
    alpha: f64,
    w: &mut [i32],
    x: &mut [f64],
    mark: i32,
    c: &mut SparseMatrix,
    mut nz: usize,
) -> Result<usize, GslError> {
    for p in a.p[j]..a.p[j + 1] {
        let i = a.i[p as usize] as usize;
        if w[i] < mark {
            w[i] = mark;
            if nz >= c.nzmax {
                c.realloc(2 * c.nzmax + a.size1)?;
            }
            c.i[nz] = i as i32;
            x[i] = alpha * a.data[p as usize];
            nz += 1;
        } else {
            x[i] += alpha * a.data[p as usize];
        }
    }
    Ok(nz)
}