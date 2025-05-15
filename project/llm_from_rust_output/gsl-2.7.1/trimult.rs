use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    matrix::Matrix,
    vector::Vector,
    error::{Error, Result},
};

pub fn gsl_linalg_tri_LTL(L: &mut Matrix) -> Result<()> {
    triangular_multsymm_L3(CblasUplo::Lower, L)
}

pub fn gsl_linalg_tri_UL(LU: &mut Matrix) -> Result<()> {
    triangular_mult_L3(CblasUplo::Upper, LU)
}

fn triangular_multsymm_L2(uplo: CblasUplo, T: &mut Matrix) -> Result<()> {
    let n = T.size1();
    if n != T.size2() {
        return Err(Error::new("matrix must be square", "trimult.c", 71, GSL_ENOTSQR));
    }

    if uplo == CblasUplo::Lower {
        for i in 0..n {
            let t_ii = T.get(i, i)?;
            
            if i < n - 1 {
                let mut v1 = T.subcolumn(i, i, n - i)?;
                let tmp = v1.dot(&v1)?;
                T.set(i, i, tmp)?;

                if i > 0 {
                    let mut m = T.submatrix(i + 1, 0, n - i - 1, i)?;
                    let mut v1 = T.subcolumn(i, i + 1, n - i - 1)?;
                    let mut v2 = T.subrow(i, 0, i)?;
                    m.gemv(CblasTranspose::Trans, 1.0, &v1, t_ii, &mut v2)?;
                }
            } else {
                let mut v = T.row(n - 1)?;
                v.scale(t_ii)?;
            }
        }
    }

    Ok(())
}

fn triangular_multsymm_L3(uplo: CblasUplo, T: &mut Matrix) -> Result<()> {
    let n = T.size1();
    if n != T.size2() {
        return Err(Error::new("matrix must be square", "trimult.c", 138, GSL_ENOTSQR));
    }

    if n <= 24 {
        return triangular_multsymm_L2(uplo, T);
    }

    let n1 = if n >= 16 {
        ((n + 8) / 16) * 8
    } else {
        n / 2
    };
    let n2 = n - n1;

    let mut t11 = T.submatrix(0, 0, n1, n1)?;
    let mut t12 = T.submatrix(0, n1, n1, n2)?;
    let mut t21 = T.submatrix(n1, 0, n2, n1)?;
    let mut t22 = T.submatrix(n1, n1, n2, n2)?;

    triangular_multsymm_L3(uplo, &mut t11)?;

    if uplo == CblasUplo::Lower {
        t11.syrk(uplo, CblasTranspose::Trans, 1.0, &t21, 1.0)?;
        t22.trmm(CblasSide::Left, uplo, CblasTranspose::Trans, CblasDiag::NonUnit, 1.0, &mut t21)?;
    } else {
        t11.syrk(uplo, CblasTranspose::NoTrans, 1.0, &t12, 1.0)?;
        t22.trmm(CblasSide::Right, uplo, CblasTranspose::Trans, CblasDiag::NonUnit, 1.0, &mut t12)?;
    }

    triangular_multsymm_L3(uplo, &mut t22)?;

    Ok(())
}

fn triangular_mult_L2(uplo: CblasUplo, A: &mut Matrix) -> Result<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(Error::new("matrix must be square", "trimult.c", 210, GSL_ENOTSQR));
    }

    if n == 1 {
        return Ok(());
    }

    if uplo == CblasUplo::Upper {
        for i in 0..n {
            let a_ii = A.get(i, i)?;
            
            if i < n - 1 {
                let lb = A.subcolumn(i, i + 1, n - i - 1)?;
                let ur = A.subrow(i, i + 1, n - i - 1)?;
                let tmp = lb.dot(&ur)?;
                A.set(i, i, a_ii + tmp)?;

                if i > 0 {
                    let u_tr = A.submatrix(0, i + 1, i, n - i - 1)?;
                    let l_bl = A.submatrix(i + 1, 0, n - i - 1, i)?;
                    let mut ut = A.subcolumn(i, 0, i)?;
                    let mut ll = A.subrow(i, 0, i)?;

                    l_bl.gemv(CblasTranspose::Trans, 1.0, &ur, a_ii, &mut ll)?;
                    u_tr.gemv(CblasTranspose::NoTrans, 1.0, &lb, 1.0, &mut ut)?;
                }
            } else {
                let mut v = A.subrow(n - 1, 0, n - 1)?;
                v.scale(a_ii)?;
            }
        }
    }

    Ok(())
}

fn triangular_mult_L3(uplo: CblasUplo, A: &mut Matrix) -> Result<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(Error::new("matrix must be square", "trimult.c", 282, GSL_ENOTSQR));
    }

    if n <= 24 {
        return triangular_mult_L2(uplo, A);
    }

    let n1 = if n >= 16 {
        ((n + 8) / 16) * 8
    } else {
        n / 2
    };
    let n2 = n - n1;

    let mut a11 = A.submatrix(0, 0, n1, n1)?;
    let mut a12 = A.submatrix(0, n1, n1, n2)?;
    let mut a21 = A.submatrix(n1, 0, n2, n1)?;
    let mut a22 = A.submatrix(n1, n1, n2, n2)?;

    triangular_mult_L3(uplo, &mut a11)?;

    if uplo != CblasUplo::Lower {
        a11.gemm(CblasTranspose::NoTrans, CblasTranspose::NoTrans, 1.0, &a12, &a21, 1.0)?;
        a22.trmm(CblasSide::Right, CblasUplo::Lower, CblasTranspose::NoTrans, CblasDiag::Unit, 1.0, &mut a12)?;
        a22.trmm(CblasSide::Left, CblasUplo::Upper, CblasTranspose::NoTrans, CblasDiag::NonUnit, 1.0, &mut a21)?;
    }

    triangular_mult_L3(uplo, &mut a22)?;

    Ok(())
}