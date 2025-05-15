use gsl::{
    blas::{CBLAS_DIAG, CBLAS_SIDE, CBLAS_TRANSPOSE, CBLAS_UPLO},
    complex::{Complex, ComplexF64},
    eigen::{herm, herm_alloc, herm_free, HermWorkspace},
    error::{Error, Result},
    linalg::complex_cholesky_decomp,
    matrix::{MatrixComplex, MatrixComplexView, MatrixComplexViewMut},
    vector::{Vector, VectorComplex, VectorComplexView, VectorComplexViewMut},
};

pub struct GenHermWorkspace {
    size: usize,
    herm_workspace: HermWorkspace,
}

impl GenHermWorkspace {
    pub fn new(n: usize) -> Result<Self> {
        if n == 0 {
            return Err(Error::new(
                "matrix dimension must be positive integer",
                "genherm.c",
                62,
                Error::EINVAL,
            ));
        }

        let herm_workspace = herm_alloc(n).map_err(|_| {
            Error::new(
                "failed to allocate space for herm workspace",
                "genherm.c",
                78,
                Error::ENOMEM,
            )
        })?;

        Ok(Self {
            size: n,
            herm_workspace,
        })
    }

    pub fn free(&mut self) {
        herm_free(&mut self.herm_workspace);
    }

    pub fn eigen_genherm(
        &mut self,
        a: &mut MatrixComplex,
        b: &mut MatrixComplex,
        eval: &mut Vector,
    ) -> Result<()> {
        let n = a.size1();
        if n != a.size2() {
            return Err(Error::new(
                "matrix must be square to compute eigenvalues",
                "genherm.c",
                127,
                Error::ENOTSQR,
            ));
        } else if n != b.size1() || n != b.size2() {
            return Err(Error::new(
                "B matrix dimensions must match A",
                "genherm.c",
                131,
                Error::EBADLEN,
            ));
        } else if eval.len() != n {
            return Err(Error::new(
                "eigenvalue vector must match matrix size",
                "genherm.c",
                135,
                Error::EBADLEN,
            ));
        } else if self.size != n {
            return Err(Error::new(
                "matrix size does not match workspace",
                "genherm.c",
                139,
                Error::EBADLEN,
            ));
        }

        complex_cholesky_decomp(b)?;
        Self::standardize(a, b)?;
        herm(a, eval, &mut self.herm_workspace)
    }

    pub fn standardize(a: &mut MatrixComplex, b: &MatrixComplex) -> Result<()> {
        standardize_l3(a, b)
    }
}

fn standardize_l2(a: &mut MatrixComplex, b: &MatrixComplex) -> Result<()> {
    let n = a.size1();
    let mut z = ComplexF64::new(0.0, 0.0);

    for i in 0..n {
        let a_ii = a.get(i, i).re();
        let b_ii = b.get(i, i).re();
        let a_new = a_ii / (b_ii * b_ii);
        a.set(i, i, ComplexF64::new(a_new, 0.0));

        if i < n - 1 {
            let mut ai = a.subcolumn_mut(i, i + 1, n - i - 1);
            let mut ma = a.submatrix_mut(i + 1, i + 1, n - i - 1, n - i - 1);
            let bi = b.subcolumn(i, i + 1, n - i - 1);
            let mb = b.submatrix(i + 1, i + 1, n - i - 1, n - i - 1);

            ai.scale(1.0 / b_ii);
            z.set(-0.5 * a_new, 0.0);
            ai.axpy(&z, &bi)?;
            ma.her2(CBLAS_UPLO::Lower, -1.0, &ai, &bi)?;
            ai.axpy(&z, &bi)?;
            ai.trsv(CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans, CBLAS_DIAG::NonUnit, &mb)?;
        }
    }

    Ok(())
}

fn standardize_l3(a: &mut MatrixComplex, b: &MatrixComplex) -> Result<()> {
    let n = a.size1();
    if n <= 24 {
        standardize_l2(a, b)
    } else {
        let n1 = if n >= 8 {
            (n + 4) / 8 * 4
        } else {
            n / 2
        };
        let n2 = n - n1;

        let mut a11 = a.submatrix_mut(0, 0, n1, n1);
        let mut a21 = a.submatrix_mut(n1, 0, n2, n1);
        let mut a22 = a.submatrix_mut(n1, n1, n2, n2);

        let b11 = b.submatrix(0, 0, n1, n1);
        let b21 = b.submatrix(n1, 0, n2, n1);
        let b22 = b.submatrix(n1, n1, n2, n2);

        let mhalf = ComplexF64::new(-0.5, 0.0);

        standardize_l3(&mut a11, &b11)?;
        a21.trsm(
            CBLAS_SIDE::Right,
            CBLAS_UPLO::Lower,
            CBLAS_TRANSPOSE::ConjTrans,
            CBLAS_DIAG::NonUnit,
            1.0,
            &b11,
        )?;
        a21.hemm(
            CBLAS_SIDE::Right,
            CBLAS_UPLO::Lower,
            mhalf,
            &a11,
            1.0,
            &b21,
        )?;
        a22.her2k(
            CBLAS_UPLO::Lower,
            CBLAS_TRANSPOSE::NoTrans,
            -1.0,
            &a21,
            &b21,
            1.0,
        )?;
        a21.hemm(
            CBLAS_SIDE::Right,
            CBLAS_UPLO::Lower,
            mhalf,
            &a11,
            1.0,
            &b21,
        )?;
        a21.trsm(
            CBLAS_SIDE::Left,
            CBLAS_UPLO::Lower,
            CBLAS_TRANSPOSE::NoTrans,
            CBLAS_DIAG::NonUnit,
            1.0,
            &b22,
        )?;
        standardize_l3(&mut a22, &b22)?;

        Ok(())
    }
}