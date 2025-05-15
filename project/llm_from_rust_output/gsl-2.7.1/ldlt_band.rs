use gsl::{
    blas::{CBLAS_DIAG, CBLAS_ORDER, CBLAS_TRANSPOSE, CBLAS_UPLO},
    error::{GSLResult, GslError},
    matrix::{Matrix, MatrixView, MatrixViewMut},
    vector::{Vector, VectorView, VectorViewMut},
};

pub fn gsl_linalg_ldlt_band_decomp(A: &mut Matrix) -> GSLResult<()> {
    let (N, ndiag) = (A.size1(), A.size2());
    if ndiag > N {
        return Err(GslError::BadLen);
    }

    let p = ndiag - 1;
    let kld = p.max(1) as i32;

    if ndiag == 1 {
        return Ok(());
    }

    let anorm = symband_norm1(A)?;
    A.set(N - 1, p, anorm)?;

    for j in 0..N - 1 {
        let ajj = A.get(j, 0)?;
        if ajj == 0.0 {
            return Err(GslError::Dom);
        }

        let lenv = p.min(N - j - 1);
        if lenv > 0 {
            let mut v = A.subrow_mut(j, 1, lenv)?;
            let mut m = A.submatrix_mut(j + 1, 0, lenv, lenv)?;
            m.set_tda(kld as usize);
            v.scale(1.0 / ajj)?;
            m.syr(CBLAS_UPLO::Upper, -ajj, &v)?;
        }
    }

    Ok(())
}

pub fn gsl_linalg_ldlt_band_solve(
    LDLT: &Matrix,
    b: &Vector,
    x: &mut Vector,
) -> GSLResult<()> {
    if LDLT.size1() != b.size() || LDLT.size1() != x.size() {
        return Err(GslError::BadLen);
    }

    gsl_linalg_ldlt_band_svx(LDLT, x)
}

pub fn gsl_linalg_ldlt_band_svx(LDLT: &Matrix, x: &mut Vector) -> GSLResult<()> {
    if LDLT.size1() != x.size() {
        return Err(GslError::BadLen);
    }

    let diag = LDLT.column(0)?;
    x.tbsv(
        CBLAS_ORDER::ColMajor,
        CBLAS_UPLO::Lower,
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_DIAG::Unit,
        (LDLT.size2() - 1) as i32,
        LDLT.data(),
        LDLT.tda() as i32,
    )?;
    x.div(&diag)?;
    x.tbsv(
        CBLAS_ORDER::ColMajor,
        CBLAS_UPLO::Lower,
        CBLAS_TRANSPOSE::Trans,
        CBLAS_DIAG::Unit,
        (LDLT.size2() - 1) as i32,
        LDLT.data(),
        LDLT.tda() as i32,
    )?;

    Ok(())
}

pub fn gsl_linalg_ldlt_band_unpack(
    LDLT: &Matrix,
    L: &mut Matrix,
    D: &mut Vector,
) -> GSLResult<()> {
    let N = LDLT.size1();
    if N != L.size1() || L.size1() != L.size2() || N != D.size() {
        return Err(GslError::BadLen);
    }

    let p = LDLT.size2() - 1;
    let diag = LDLT.column(0)?;
    D.copy(&diag)?;

    let mut diagL = L.diagonal_mut()?;
    diagL.set_all(1.0)?;

    for i in 1..=p {
        let v = LDLT.subcolumn(i, 0, N - i)?;
        let mut w = L.subdiagonal_mut(i)?;
        w.copy(&v)?;
    }

    for i in p + 1..N {
        let mut w = L.subdiagonal_mut(i)?;
        w.set_zero()?;
    }

    Ok(())
}

pub fn gsl_linalg_ldlt_band_rcond(
    LDLT: &Matrix,
    rcond: &mut f64,
    work: &mut Vector,
) -> GSLResult<()> {
    let N = LDLT.size1();
    if work.size() != 3 * N {
        return Err(GslError::BadLen);
    }

    let ndiag = LDLT.size2();
    let anorm = if ndiag == 1 {
        symband_norm1(LDLT)?
    } else {
        LDLT.get(N - 1, ndiag - 1)?
    };

    *rcond = 0.0;
    if anorm == 0.0 {
        return Ok(());
    }

    let mut ainvnorm = 0.0;
    gsl_linalg_invnorm1(
        N,
        ldlt_band_Ainv,
        LDLT as *const _ as *mut _,
        &mut ainvnorm,
        work,
    )?;

    if ainvnorm != 0.0 {
        *rcond = 1.0 / anorm / ainvnorm;
    }

    Ok(())
}

fn symband_norm1(A: &Matrix) -> GSLResult<f64> {
    let (N, ndiag) = (A.size1(), A.size2());
    if ndiag == 1 {
        let v = A.column(0)?;
        Ok(v.get(v.idamax()?)?)
    } else {
        let mut value = 0.0;
        for j in 0..N {
            let ncol = ndiag.min(N - j);
            let v = A.subrow(j, 0, ncol)?;
            let mut sum = v.asum()?;

            let mut k = j;
            let mut l = 1;
            while k > 0 && l < ndiag {
                k -= 1;
                sum += A.get(k, l)?.abs();
                l += 1;
            }

            value = value.max(sum);
        }
        Ok(value)
    }
}

fn ldlt_band_Ainv(trans: CBLAS_TRANSPOSE, x: &mut Vector, params: *mut ()) -> GSLResult<()> {
    let LDLT = unsafe { &*(params as *const Matrix) };
    let diag = LDLT.column(0)?;

    x.tbsv(
        CBLAS_ORDER::ColMajor,
        CBLAS_UPLO::Lower,
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_DIAG::Unit,
        (LDLT.size2() - 1) as i32,
        LDLT.data(),
        LDLT.tda() as i32,
    )?;
    x.div(&diag)?;
    x.tbsv(
        CBLAS_ORDER::ColMajor,
        CBLAS_UPLO::Lower,
        trans,
        CBLAS_DIAG::Unit,
        (LDLT.size2() - 1) as i32,
        LDLT.data(),
        LDLT.tda() as i32,
    )?;

    Ok(())
}