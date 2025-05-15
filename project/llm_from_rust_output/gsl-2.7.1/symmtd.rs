use gsl::{
    blas::{daxpy, ddot, dsyr2, dsymv},
    error::{Error, Result},
    linalg::{householder_left, householder_transform},
    matrix::{Matrix, MatrixView, Submatrix},
    types::{ComplexF64, Vector, VectorView},
    value::GslValue,
    CBLAS_UPLO,
};

pub fn symmtd_decomp(a: &mut Matrix, tau: &mut Vector) -> Result<()> {
    if a.size1() != a.size2() {
        return Err(Error::NotSquare);
    }
    if tau.size() + 1 != a.size1() {
        return Err(Error::BadLength);
    }

    let n = a.size1();
    for i in 0..n - 2 {
        let mut v = a.subcolumn(i, i + 1, n - i - 1)?;
        let tau_i = householder_transform(&mut v)?;

        if tau_i != 0.0 {
            let mut m = a.submatrix(i + 1, i + 1, n - i - 1, n - i - 1)?;
            let ei = v.get(0)?;
            let mut x = tau.subvector(i, n - i - 1)?;

            v.set(0, 1.0)?;
            dsymv(CBLAS_UPLO::Lower, tau_i, &m, &v, 0.0, &mut x)?;

            let xv = ddot(&x, &v)?;
            let alpha = -0.5 * tau_i * xv;
            daxpy(alpha, &v, &mut x)?;
            dsyr2(CBLAS_UPLO::Lower, -1.0, &v, &x, &mut m)?;

            v.set(0, ei)?;
        }
        tau.set(i, tau_i)?;
    }

    Ok(())
}

pub fn symmtd_unpack(
    a: &Matrix,
    tau: &Vector,
    q: &mut Matrix,
    diag: &mut Vector,
    sdiag: &mut Vector,
) -> Result<()> {
    if a.size1() != a.size2() {
        return Err(Error::NotSquare);
    }
    if tau.size() + 1 != a.size1() {
        return Err(Error::BadLength);
    }
    if q.size1() != a.size1() || q.size2() != a.size1() {
        return Err(Error::BadLength);
    }
    if diag.size() != a.size1() {
        return Err(Error::BadLength);
    }
    if sdiag.size() + 1 != a.size1() {
        return Err(Error::BadLength);
    }

    let n = a.size1();
    let d = a.diagonal();
    let sd = a.subdiagonal(1)?;

    q.set_identity();

    for i in (0..n - 2).rev() {
        let h = a.subcolumn(i, i + 1, n - i - 1)?;
        let ti = tau.get(i)?;
        let mut m = q.submatrix(i + 1, i + 1, n - i - 1, n - i - 1)?;
        let mut work = diag.subvector(0, n - i - 1)?;

        let mut tmp = h.get(0)?;
        h.set(0, 1.0)?;
        householder_left(ti, &h, &mut m, &mut work)?;
        h.set(0, tmp)?;
    }

    diag.copy_from(&d)?;
    sdiag.copy_from(&sd)?;

    Ok(())
}

pub fn symmtd_unpack_t(a: &Matrix, diag: &mut Vector, sdiag: &mut Vector) -> Result<()> {
    if a.size1() != a.size2() {
        return Err(Error::NotSquare);
    }
    if diag.size() != a.size1() {
        return Err(Error::BadLength);
    }
    if sdiag.size() + 1 != a.size1() {
        return Err(Error::BadLength);
    }

    let d = a.diagonal();
    let sd = a.subdiagonal(1)?;

    diag.copy_from(&d)?;
    sdiag.copy_from(&sd)?;

    Ok(())
}