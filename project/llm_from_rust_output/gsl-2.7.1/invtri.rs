use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    matrix::{Matrix, MatrixView},
    vector::VectorView,
};

pub fn gsl_linalg_tri_invert(
    uplo: CblasUplo,
    diag: CblasDiag,
    t: &mut Matrix,
) -> GslResult<()> {
    let n = t.size1();
    if n != t.size2() {
        return Err(GslError::NotSquare);
    }

    triangular_singular(t)?;
    triangular_inverse_l3(uplo, diag, t)
}

fn triangular_inverse_l2(uplo: CblasUplo, diag: CblasDiag, t: &mut Matrix) -> GslResult<()> {
    let n = t.size1();
    if n != t.size2() {
        return Err(GslError::NotSquare);
    }

    if uplo == CblasUplo::Upper {
        for i in 0..n {
            let aii = if diag == CblasDiag::NonUnit {
                let tii = t.get_mut(i, i).unwrap();
                *tii = 1.0 / *tii;
                -*tii
            } else {
                -1.0
            };

            if i > 0 {
                let m = t.submatrix(0, 0, i, i);
                let mut v = t.subcolumn(i, 0, i);
                v.trmv(CblasUplo::Upper, CblasTranspose::NoTrans, diag, &m)?;
                v.scale(aii);
            }
        }
    } else {
        for i in 0..n {
            let j = n - i - 1;
            let ajj = if diag == CblasDiag::NonUnit {
                let tjj = t.get_mut(j, j).unwrap();
                *tjj = 1.0 / *tjj;
                -*tjj
            } else {
                -1.0
            };

            if j < n - 1 {
                let m = t.submatrix(j + 1, j + 1, n - j - 1, n - j - 1);
                let mut v = t.subcolumn(j, j + 1, n - j - 1);
                v.trmv(CblasUplo::Lower, CblasTranspose::NoTrans, diag, &m)?;
                v.scale(ajj);
            }
        }
    }

    Ok(())
}

fn triangular_inverse_l3(uplo: CblasUplo, diag: CblasDiag, t: &mut Matrix) -> GslResult<()> {
    let n = t.size1();
    if n != t.size2() {
        return Err(GslError::NotSquare);
    }

    if n <= 24 {
        return triangular_inverse_l2(uplo, diag, t);
    }

    let n1 = if n >= 16 {
        ((n + 8) / 16) * 8
    } else {
        n / 2
    };
    let n2 = n - n1;

    let mut t11 = t.submatrix(0, 0, n1, n1);
    let mut t12 = t.submatrix(0, n1, n1, n2);
    let mut t21 = t.submatrix(n1, 0, n2, n1);
    let mut t22 = t.submatrix(n1, n1, n2, n2);

    triangular_inverse_l3(uplo, diag, &mut t11)?;

    if uplo == CblasUplo::Lower {
        t21.trmm(
            CblasSide::Right,
            uplo,
            CblasTranspose::NoTrans,
            diag,
            -1.0,
            &t11,
        )?;
        t21.trsm(
            CblasSide::Left,
            uplo,
            CblasTranspose::NoTrans,
            diag,
            1.0,
            &t22,
        )?;
    } else {
        t12.trmm(
            CblasSide::Left,
            uplo,
            CblasTranspose::NoTrans,
            diag,
            -1.0,
            &t11,
        )?;
        t12.trsm(
            CblasSide::Right,
            uplo,
            CblasTranspose::NoTrans,
            diag,
            1.0,
            &t22,
        )?;
    }

    triangular_inverse_l3(uplo, diag, &mut t22)?;

    Ok(())
}

fn triangular_singular(t: &Matrix) -> GslResult<()> {
    for i in 0..t.size1() {
        let tii = t.get(i, i).unwrap();
        if tii == 0.0 {
            return Err(GslError::Singular);
        }
    }
    Ok(())
}

pub fn gsl_linalg_tri_upper_invert(t: &mut Matrix) -> GslResult<()> {
    triangular_singular(t)?;
    triangular_inverse_l3(CblasUplo::Upper, CblasDiag::NonUnit, t)
}

pub fn gsl_linalg_tri_lower_invert(t: &mut Matrix) -> GslResult<()> {
    triangular_singular(t)?;
    triangular_inverse_l3(CblasUplo::Lower, CblasDiag::NonUnit, t)
}

pub fn gsl_linalg_tri_upper_unit_invert(t: &mut Matrix) -> GslResult<()> {
    triangular_singular(t)?;
    triangular_inverse_l3(CblasUplo::Upper, CblasDiag::Unit, t)
}

pub fn gsl_linalg_tri_lower_unit_invert(t: &mut Matrix) -> GslResult<()> {
    triangular_singular(t)?;
    triangular_inverse_l3(CblasUplo::Lower, CblasDiag::Unit, t)
}