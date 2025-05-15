use gsl::{
    blas::CBLAS_DIAG,
    blas::CBLAS_UPLO,
    error::{GSL_EBADLEN, GSL_ENOTSQR, GSL_SUCCESS},
    matrix::{Matrix, MatrixView, MatrixViewMut},
    vector::{Vector, VectorView, VectorViewMut},
};

pub fn ql_decomp(a: &mut Matrix, tau: &mut Vector) -> Result<(), i32> {
    let m = a.size1();
    let n = a.size2();

    if tau.size() != n {
        return Err(GSL_EBADLEN);
    }

    let k = std::cmp::min(m, n);

    for i in 0..k {
        let col = n - i - 1;
        let mut c = a.subcolumn(col, 0, m - i)?;
        let alpha = a.get_mut(m - i - 1, col).ok_or(GSL_EBADLEN)?;

        let tau_j = gsl::linalg::householder_transform2(alpha, &mut c)?;

        if i + 1 < n {
            let mut work = tau.subvector(0, n - i - 1)?;
            let mut m_sub = a.submatrix(0, 0, m - i, n - i - 1)?;
            let tmp = *alpha;
            *alpha = 1.0;
            gsl::linalg::householder_left(tau_j, &c, &mut m_sub, &mut work)?;
            *alpha = tmp;
        }

        tau.set(n - i - 1, tau_j)?;
    }

    Ok(())
}

pub fn ql_unpack(
    ql: &Matrix,
    tau: &Vector,
    q: &mut Matrix,
    l: &mut Matrix,
) -> Result<(), i32> {
    let m = ql.size1();
    let n = ql.size2();

    if q.size1() != m || q.size2() != m {
        return Err(GSL_ENOTSQR);
    }
    if l.size1() != m || l.size2() != n {
        return Err(GSL_ENOTSQR);
    }
    if tau.size() != n {
        return Err(GSL_EBADLEN);
    }

    let k = std::cmp::min(m, n);

    q.set_identity();

    for i in 0..k {
        let col = n - k + i;
        let h = ql.subcolumn(col, 0, m - k + i + 1)?;
        let mut m_sub = q.submatrix(0, 0, m - k + i + 1, m - k + i + 1)?;
        let mut work = l.subcolumn(0, 0, m - k + i + 1)?;
        let ti = tau.get(col)?;
        let ptr = ql.get(m - k + i, col).ok_or(GSL_EBADLEN)?;
        let tmp = *ptr;
        *ptr = 1.0;
        gsl::linalg::householder_left(ti, &h, &mut m_sub, &mut work)?;
        *ptr = tmp;
    }

    l.set_zero();

    if m >= n {
        let src = ql.submatrix(m - n, 0, n, n)?;
        let mut dest = l.submatrix(m - n, 0, n, n)?;
        gsl::matrix::tricpy(CBLAS_UPLO::Lower, CBLAS_DIAG::NonUnit, &mut dest, &src)?;
    } else {
        let src1 = ql.submatrix(0, 0, m, n - m)?;
        let mut dest1 = l.submatrix(0, 0, m, n - m)?;
        let src2 = ql.submatrix(0, n - m, m, m)?;
        let mut dest2 = l.submatrix(0, n - m, m, m)?;
        gsl::matrix::memcpy(&mut dest1, &src1)?;
        gsl::matrix::tricpy(CBLAS_UPLO::Lower, CBLAS_DIAG::NonUnit, &mut dest2, &src2)?;
    }

    Ok(())
}