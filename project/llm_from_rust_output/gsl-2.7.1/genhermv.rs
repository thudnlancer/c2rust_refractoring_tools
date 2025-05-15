use gsl::{
    blas::{CblasConjTrans, CblasDiag, CblasLeft, CblasLower, CblasNonUnit, CblasSide, CblasTranspose, CblasUpLo},
    eigen::{hermv_alloc, hermv_free, hermv_workspace, hermv},
    error::{Error, Result},
    linalg::complex_cholesky_decomp,
    matrix::{complex::MatrixComplex, VectorComplexView},
    types::{complex::Complex, Vector, VectorComplex},
    Value,
};

pub struct GenHermVWorkspace {
    size: usize,
    hermv_workspace: hermv_workspace,
}

impl GenHermVWorkspace {
    pub fn new(n: usize) -> Result<Self> {
        if n == 0 {
            return Err(Error::new(Value::Invalid, "matrix dimension must be positive integer"));
        }

        let hermv_workspace = hermv_alloc(n)?;
        Ok(Self {
            size: n,
            hermv_workspace,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for GenHermVWorkspace {
    fn drop(&mut self) {
        hermv_free(&mut self.hermv_workspace);
    }
}

pub fn genhermv(
    a: &mut MatrixComplex,
    b: &mut MatrixComplex,
    eval: &mut Vector,
    evec: &mut MatrixComplex,
    w: &mut GenHermVWorkspace,
) -> Result<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(Error::new(Value::MatrixNotSquare, "matrix must be square to compute eigenvalues"));
    }
    if n != b.size1() || n != b.size2() {
        return Err(Error::new(Value::BadLength, "B matrix dimensions must match A"));
    }
    if eval.len() != n {
        return Err(Error::new(Value::BadLength, "eigenvalue vector must match matrix size"));
    }
    if evec.size1() != evec.size2() {
        return Err(Error::new(Value::MatrixNotSquare, "eigenvector matrix must be square"));
    }
    if evec.size1() != n {
        return Err(Error::new(Value::BadLength, "eigenvector matrix has wrong size"));
    }
    if w.size() != n {
        return Err(Error::new(Value::BadLength, "matrix size does not match workspace"));
    }

    complex_cholesky_decomp(b)?;
    genherm_standardize(a, b)?;
    hermv(a, eval, evec, &mut w.hermv_workspace)?;

    let alpha = Complex::new(1.0, 0.0);
    evec.ztrsm(
        CblasSide::Left,
        CblasUpLo::Lower,
        CblasTranspose::ConjTrans,
        CblasDiag::NonUnit,
        &alpha,
        b,
    )?;

    normalize_eigenvectors(evec);
    Ok(())
}

fn genherm_standardize(a: &mut MatrixComplex, b: &MatrixComplex) -> Result<()> {
    // Implementation of gsl_eigen_genherm_standardize
    // This function is not directly available in the gsl crate,
    // so we would need to implement it using available operations
    unimplemented!("gsl_eigen_genherm_standardize needs to be implemented")
}

fn normalize_eigenvectors(evec: &mut MatrixComplex) {
    let n = evec.size1();
    for i in 0..n {
        let mut vi = evec.column(i);
        let scale = 1.0 / vi.dznrm2();
        vi.zdscal(scale);
    }
}