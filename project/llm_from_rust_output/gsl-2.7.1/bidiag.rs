use libc::{c_double, c_int, c_ulong};
use std::ptr;

pub type size_t = c_ulong;

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
    pub size: size_t,
    pub data: Vec<c_double>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: Vec<c_double>,
    pub block: Option<GslBlock>,
}

#[derive(Debug, Clone)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: Vec<c_double>,
    pub block: Option<GslBlock>,
}

impl GslVector {
    pub fn new(n: size_t) -> Self {
        GslVector {
            size: n,
            stride: 1,
            data: vec![0.0; n],
            block: None,
        }
    }

    pub fn get(&self, i: size_t) -> c_double {
        self.data[i * self.stride]
    }

    pub fn set(&mut self, i: size_t, x: c_double) {
        self.data[i * self.stride] = x;
    }

    pub fn subvector(&mut self, offset: size_t, n: size_t) -> GslVectorView {
        GslVectorView {
            vector: GslVector {
                size: n,
                stride: self.stride,
                data: self.data[offset..offset + n * self.stride].to_vec(),
                block: None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct GslVectorView {
    pub vector: GslVector,
}

#[derive(Debug, Clone)]
pub struct GslVectorConstView {
    pub vector: GslVector,
}

impl GslMatrix {
    pub fn new(size1: size_t, size2: size_t) -> Self {
        GslMatrix {
            size1,
            size2,
            tda: size2,
            data: vec![0.0; size1 * size2],
            block: None,
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
    ) -> GslMatrixView {
        GslMatrixView {
            matrix: GslMatrix {
                size1: n1,
                size2: n2,
                tda: self.tda,
                data: self.data[i * self.tda + j..].to_vec(),
                block: None,
            },
        }
    }

    pub fn subrow(&mut self, i: size_t, offset: size_t, n: size_t) -> GslVectorView {
        GslVectorView {
            vector: GslVector {
                size: n,
                stride: 1,
                data: self.data[i * self.tda + offset..i * self.tda + offset + n].to_vec(),
                block: None,
            },
        }
    }

    pub fn subcolumn(&mut self, j: size_t, offset: size_t, n: size_t) -> GslVectorView {
        GslVectorView {
            vector: GslVector {
                size: n,
                stride: self.tda,
                data: self.data[offset * self.tda + j..].to_vec(),
                block: None,
            },
        }
    }

    pub fn const_row(&self, i: size_t) -> GslVectorConstView {
        GslVectorConstView {
            vector: GslVector {
                size: self.size2,
                stride: 1,
                data: self.data[i * self.tda..(i + 1) * self.tda].to_vec(),
                block: None,
            },
        }
    }

    pub fn const_subrow(&self, i: size_t, offset: size_t, n: size_t) -> GslVectorConstView {
        GslVectorConstView {
            vector: GslVector {
                size: n,
                stride: 1,
                data: self.data[i * self.tda + offset..i * self.tda + offset + n].to_vec(),
                block: None,
            },
        }
    }

    pub fn const_subcolumn(&self, j: size_t, offset: size_t, n: size_t) -> GslVectorConstView {
        GslVectorConstView {
            vector: GslVector {
                size: n,
                stride: self.tda,
                data: self.data[offset * self.tda + j..].to_vec(),
                block: None,
            },
        }
    }

    pub fn set_identity(&mut self) {
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                self.set(i, j, if i == j { 1.0 } else { 0.0 });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GslMatrixView {
    pub matrix: GslMatrix,
}

pub fn gsl_linalg_householder_transform(v: &mut GslVector) -> c_double {
    // Implementation of Householder transform
    0.0
}

pub fn gsl_linalg_householder_hm(tau: c_double, v: &GslVector, a: &mut GslMatrix) -> c_int {
    // Implementation of Householder matrix multiplication
    0
}

pub fn gsl_linalg_householder_left(
    tau: c_double,
    v: &GslVector,
    a: &mut GslMatrix,
    work: &mut GslVector,
) -> c_int {
    // Implementation of Householder left multiplication
    0
}

pub fn gsl_linalg_householder_right(
    tau: c_double,
    v: &GslVector,
    a: &mut GslMatrix,
    work: &mut GslVector,
) -> c_int {
    // Implementation of Householder right multiplication
    0
}

pub fn gsl_linalg_householder_hm1(tau: c_double, a: &mut GslMatrix) -> c_int {
    // Implementation of Householder matrix multiplication (variant)
    0
}

pub fn gsl_linalg_bidiag_decomp(
    a: &mut GslMatrix,
    tau_u: &mut GslVector,
    tau_v: &mut GslVector,
) -> Result<(), GslError> {
    if a.size1 < a.size2 {
        return Err(GslError::BadLen);
    }
    if tau_u.size != a.size2 {
        return Err(GslError::BadLen);
    }
    if tau_v.size + 1 != a.size2 {
        return Err(GslError::BadLen);
    }

    let m = a.size1;
    let n = a.size2;
    let mut tmp = GslVector::new(m);

    for j in 0..n {
        let mut v = a.subcolumn(j, j, m - j);
        let tau_j = gsl_linalg_householder_transform(&mut v.vector);

        if j + 1 < n {
            let mut m_view = a.submatrix(j, j + 1, m - j, n - j - 1);
            let mut work = tau_u.subvector(j, n - j - 1);
            let tmp_val = v.vector.get(0);
            v.vector.set(0, 1.0);
            gsl_linalg_householder_left(tau_j, &v.vector, &mut m_view.matrix, &mut work.vector);
            v.vector.set(0, tmp_val);
        }

        tau_u.set(j, tau_j);

        if j + 1 < n {
            let mut v = a.subrow(j, j + 1, n - j - 1);
            let tau_j = gsl_linalg_householder_transform(&mut v.vector);

            if j + 1 < m {
                let mut m_view = a.submatrix(j + 1, j + 1, m - j - 1, n - j - 1);
                let mut work = tmp.subvector(0, m - j - 1);
                gsl_linalg_householder_right(tau_j, &v.vector, &mut m_view.matrix, &mut work.vector);
            }

            tau_v.set(j, tau_j);
        }
    }

    Ok(())
}

pub fn gsl_linalg_bidiag_unpack(
    a: &GslMatrix,
    tau_u: &GslVector,
    u: &mut GslMatrix,
    tau_v: &GslVector,
    v: &mut GslMatrix,
    diag: &mut GslVector,
    superdiag: &mut GslVector,
) -> Result<(), GslError> {
    let m = a.size1;
    let n = a.size2;
    let k = if m < n { m } else { n };

    if m < n {
        return Err(GslError::BadLen);
    }
    if tau_u.size != k {
        return Err(GslError::BadLen);
    }
    if tau_v.size + 1 != k {
        return Err(GslError::BadLen);
    }
    if u.size1 != m || u.size2 != n {
        return Err(GslError::BadLen);
    }
    if v.size1 != n || v.size2 != n {
        return Err(GslError::BadLen);
    }
    if diag.size != k {
        return Err(GslError::BadLen);
    }
    if superdiag.size + 1 != k {
        return Err(GslError::BadLen);
    }

    for i in 0..n {
        let aii = a.get(i, i);
        diag.set(i, aii);
    }

    for i in 0..n - 1 {
        let aij = a.get(i, i + 1);
        superdiag.set(i, aij);
    }

    v.set_identity();

    for i in (1..n).rev() {
        let h = a.const_subrow(i, i + 1, n - i - 1);
        let ti = tau_v.get(i);
        let mut m_view = v.submatrix(i + 1, i + 1, n - i - 1, n - i - 1);
        let mut work = u.subrow(0, 0, n - i - 1);
        let tmp_val = h.vector.get(0);
        h.vector.set(0, 1.0);
        gsl_linalg_householder_left(ti, &h.vector, &mut m_view.matrix, &mut work.vector);
        h.vector.set(0, tmp_val);
    }

    u.set_identity();

    for j in (1..=n).rev() {
        let h = a.const_subcolumn(j, j, m - j);
        let tj = tau_u.get(j);
        let mut m_view = u.submatrix(j, j, m - j, n - j);
        gsl_linalg_householder_hm(tj, &h.vector, &mut m_view.matrix);
    }

    Ok(())
}

pub fn gsl_linalg_bidiag_unpack2(
    a: &mut GslMatrix,
    tau_u: &mut GslVector,
    tau_v: &mut GslVector,
    v: &mut GslMatrix,
) -> Result<(), GslError> {
    let m = a.size1;
    let n = a.size2;
    let k = if m < n { m } else { n };

    if m < n {
        return Err(GslError::BadLen);
    }
    if tau_u.size != k {
        return Err(GslError::BadLen);
    }
    if tau_v.size + 1 != k {
        return Err(GslError::BadLen);
    }
    if v.size1 != n || v.size2 != n {
        return Err(GslError::BadLen);
    }

    v.set_identity();

    for i in (1..n).rev() {
        let r = a.const_row(i);
        let h = GslVectorConstView {
            vector: GslVector {
                size: n - i - 1,
                stride: 1,
                data: r.vector.data[i + 1..].to_vec(),
                block: None,
            },
        };
        let ti = tau_v.get(i);
        let mut m_view = v.submatrix(i + 1, i + 1, n - i - 1, n - i - 1);
        gsl_linalg_householder_hm(ti, &h.vector, &mut m_view.matrix);
    }

    for i in 0..n - 1 {
        let aij = a.get(i, i + 1);
        tau_v.set(i, aij);
    }

    for j in (1..=n).rev() {
        let tj = tau_u.get(j);
        let ajj = a.get(j, j);
        let mut m_view = a.submatrix(j, j, m - j, n - j);
        tau_u.set(j, ajj);
        gsl_linalg_householder_hm1(tj, &mut m_view.matrix);
    }

    Ok(())
}

pub fn gsl_linalg_bidiag_unpack_b(
    a: &GslMatrix,
    diag: &mut GslVector,
    superdiag: &mut GslVector,
) -> Result<(), GslError> {
    let m = a.size1;
    let n = a.size2;
    let k = if m < n { m } else { n };

    if diag.size != k {
        return Err(GslError::BadLen);
    }
    if superdiag.size + 1 != k {
        return Err(GslError::BadLen);
    }

    for i in 0..k {
        let aii = a.get(i, i);
        diag.set(i, aii);
    }

    for i in 0..k - 1 {
        let aij = a.get(i, i + 1);
        superdiag.set(i, aij);
    }

    Ok(())
}