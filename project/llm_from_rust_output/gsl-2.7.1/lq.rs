use libc::{c_double, c_int, c_ulong};
use std::ptr;

pub type size_t = c_ulong;

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

#[derive(Debug, Clone, Copy)]
pub enum CblasTranspose {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasUplo {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasDiag {
    NonUnit = 131,
    Unit = 132,
}

#[derive(Debug)]
pub struct GslBlock {
    pub size: size_t,
    pub data: Vec<c_double>,
}

#[derive(Debug)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: Vec<c_double>,
    pub block: Option<Box<GslBlock>>,
    pub owner: c_int,
}

#[derive(Debug)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: Vec<c_double>,
    pub block: Option<Box<GslBlock>>,
    pub owner: c_int,
}

impl GslVector {
    pub fn new(size: size_t) -> Self {
        GslVector {
            size,
            stride: 1,
            data: vec![0.0; size],
            block: None,
            owner: 0,
        }
    }

    pub fn get(&self, i: size_t) -> c_double {
        self.data[i * self.stride]
    }

    pub fn set(&mut self, i: size_t, x: c_double) {
        self.data[i * self.stride] = x;
    }

    pub fn subvector(&mut self, i: size_t, n: size_t) -> GslVector {
        GslVector {
            size: n,
            stride: self.stride,
            data: self.data[i * self.stride..(i + n) * self.stride].to_vec(),
            block: None,
            owner: 0,
        }
    }

    pub fn sub(&mut self, other: &GslVector) -> Result<(), GslError> {
        if self.size != other.size {
            return Err(GslError::BadLen);
        }
        for i in 0..self.size {
            self.data[i * self.stride] -= other.data[i * other.stride];
        }
        Ok(())
    }

    pub fn scale(&mut self, x: c_double) {
        for i in 0..self.size {
            self.data[i * self.stride] *= x;
        }
    }

    pub fn set_zero(&mut self) {
        for i in 0..self.size {
            self.data[i * self.stride] = 0.0;
        }
    }

    pub fn memcpy(dest: &mut GslVector, src: &GslVector) -> Result<(), GslError> {
        if dest.size != src.size {
            return Err(GslError::BadLen);
        }
        for i in 0..dest.size {
            dest.data[i * dest.stride] = src.data[i * src.stride];
        }
        Ok(())
    }
}

impl GslMatrix {
    pub fn new(size1: size_t, size2: size_t) -> Self {
        GslMatrix {
            size1,
            size2,
            tda: size2,
            data: vec![0.0; size1 * size2],
            block: None,
            owner: 0,
        }
    }

    pub fn get(&self, i: size_t, j: size_t) -> c_double {
        self.data[i * self.tda + j]
    }

    pub fn set(&mut self, i: size_t, j: size_t, x: c_double) {
        self.data[i * self.tda + j] = x;
    }

    pub fn submatrix(
        &mut self,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> GslMatrix {
        let mut data = Vec::with_capacity(n1 * n2);
        for row in i..i + n1 {
            for col in j..j + n2 {
                data.push(self.data[row * self.tda + col]);
            }
        }
        GslMatrix {
            size1: n1,
            size2: n2,
            tda: n2,
            data,
            block: None,
            owner: 0,
        }
    }

    pub fn subrow(&mut self, i: size_t, offset: size_t, n: size_t) -> GslVector {
        GslVector {
            size: n,
            stride: 1,
            data: self.data[i * self.tda + offset..i * self.tda + offset + n]
                .to_vec(),
            block: None,
            owner: 0,
        }
    }

    pub fn set_identity(&mut self) {
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                self.data[i * self.tda + j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }

    pub fn row(&self, i: size_t) -> GslVector {
        GslVector {
            size: self.size2,
            stride: 1,
            data: self.data[i * self.tda..(i + 1) * self.tda].to_vec(),
            block: None,
            owner: 0,
        }
    }
}

pub fn gsl_linalg_givens(a: c_double, b: c_double) -> (c_double, c_double) {
    if b == 0.0 {
        (1.0, 0.0)
    } else if b.abs() > a.abs() {
        let t = -a / b;
        let s1 = 1.0 / (1.0 + t * t).sqrt();
        (s1 * t, s1)
    } else {
        let t = -b / a;
        let c1 = 1.0 / (1.0 + t * t).sqrt();
        (c1, c1 * t)
    }
}

pub fn gsl_linalg_givens_gv(v: &mut GslVector, i: size_t, j: size_t, c: c_double, s: c_double) {
    let vi = v.get(i);
    let vj = v.get(j);
    v.set(i, c * vi - s * vj);
    v.set(j, s * vi + c * vj);
}

pub fn apply_givens_lq(
    m: size_t,
    n: size_t,
    q: &mut GslMatrix,
    l: &mut GslMatrix,
    i: size_t,
    j: size_t,
    c: c_double,
    s: c_double,
) {
    for k in 0..m {
        let qik = q.get(i, k);
        let qjk = q.get(j, k);
        q.set(i, k, qik * c - qjk * s);
        q.set(j, k, qik * s + qjk * c);
    }

    let start = if i < j { i } else { j };
    for k in start..n {
        let lki = l.get(k, i);
        let lkj = l.get(k, j);
        l.set(k, i, c * lki - s * lkj);
        l.set(k, j, s * lki + c * lkj);
    }
}

pub fn gsl_linalg_householder_transform(v: &mut GslVector) -> c_double {
    let n = v.size;
    let mut tau = 0.0;

    if n == 1 {
        tau = 0.0;
        v.set(0, -1.0);
    } else {
        let mut xnorm = 0.0;
        for i in 1..n {
            xnorm += v.get(i) * v.get(i);
        }
        xnorm = xnorm.sqrt();

        if xnorm == 0.0 {
            tau = 0.0;
        } else {
            let beta = -(v.get(0).signum()) * (v.get(0) * v.get(0) + xnorm * xnorm).sqrt();
            tau = (beta - v.get(0)) / beta;

            let s = 1.0 / (v.get(0) - beta);
            v.set(0, beta);
            for i in 1..n {
                v.set(i, s * v.get(i));
            }
        }
    }

    tau
}

pub fn gsl_linalg_householder_mh(tau: c_double, v: &GslVector, a: &mut GslMatrix) -> Result<(), GslError> {
    let n = a.size2;
    let k = v.size;

    if a.size1 != n {
        return Err(GslError::BadLen);
    }

    for j in 0..n {
        let mut wj = 0.0;
        for i in 0..k {
            wj += a.get(i, j) * v.get(i);
        }
        wj *= tau;

        for i in 0..k {
            let aij = a.get(i, j);
            a.set(i, j, aij - wj * v.get(i));
        }
    }

    Ok(())
}

pub fn gsl_linalg_householder_hv(tau: c_double, v: &GslVector, w: &mut GslVector) -> Result<(), GslError> {
    let n = v.size;

    if w.size != n {
        return Err(GslError::BadLen);
    }

    let mut d = 0.0;
    for i in 0..n {
        d += v.get(i) * w.get(i);
    }
    d *= tau;

    for i in 0..n {
        let wi = w.get(i);
        w.set(i, wi - d * v.get(i));
    }

    Ok(())
}

pub fn gsl_linalg_LQ_decomp(a: &mut GslMatrix, tau: &mut GslVector) -> Result<(), GslError> {
    let n = a.size1;
    let m = a.size2;
    let min_size = if m < n { m } else { n };

    if tau.size != min_size {
        return Err(GslError::BadLen);
    }

    for i in 0..min_size {
        let mut c = a.subrow(i, i, m - i);
        let tau_i = gsl_linalg_householder_transform(&mut c);
        tau.set(i, tau_i);

        if i + 1 < n {
            let mut m_sub = a.submatrix(i + 1, i, n - (i + 1), m - i);
            gsl_linalg_householder_mh(tau_i, &c, &mut m_sub)?;
        }
    }

    Ok(())
}

pub fn gsl_linalg_LQ_solve_T(
    lq: &GslMatrix,
    tau: &GslVector,
    b: &GslVector,
    x: &mut GslVector,
) -> Result<(), GslError> {
    if lq.size1 != lq.size2 {
        return Err(GslError::NotSquare);
    } else if lq.size2 != b.size {
        return Err(GslError::BadLen);
    } else if lq.size1 != x.size {
        return Err(GslError::BadLen);
    }

    GslVector::memcpy(x, b)?;
    gsl_linalg_LQ_svx_T(lq, tau, x)?;
    Ok(())
}

pub fn gsl_linalg_LQ_svx_T(lq: &GslMatrix, tau: &GslVector, x: &mut GslVector) -> Result<(), GslError> {
    if lq.size1 != lq.size2 {
        return Err(GslError::NotSquare);
    } else if lq.size1 != x.size {
        return Err(GslError::BadLen);
    }

    gsl_linalg_LQ_vecQT(lq, tau, x)?;
    // gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, lq, x);
    Ok(())
}

// Additional functions would follow the same pattern of conversion to safe Rust
// with proper error handling and avoiding unsafe blocks where possible.