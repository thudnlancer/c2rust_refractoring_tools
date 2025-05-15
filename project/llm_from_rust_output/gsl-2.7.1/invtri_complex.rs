use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    complex::{Complex, ComplexF64},
    matrix_complex::MatrixComplexF64,
    vector_complex::VectorComplexF64,
    types::{MatrixComplexView, VectorComplexView},
    error::{Result, GslError, GslResult},
};

pub fn gsl_linalg_complex_tri_invert(
    uplo: CblasUplo,
    diag: CblasDiag,
    t: &mut MatrixComplexF64,
) -> Result<()> {
    let n = t.size1();
    if n != t.size2() {
        return Err(GslError::MatrixNotSquare);
    }

    triangular_singular(t)?;
    complex_tri_invert_l3(uplo, diag, t)
}

fn complex_tri_invert_l2(
    uplo: CblasUplo,
    diag: CblasDiag,
    t: &mut MatrixComplexF64,
) -> Result<()> {
    let n = t.size1();
    if n != t.size2() {
        return Err(GslError::MatrixNotSquare);
    }

    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut tii = t.get_mut(i, i)?;
                let aii = if diag == CblasDiag::NonUnit {
                    *tii = tii.inverse();
                    -(*tii)
                } else {
                    Complex::new(-1.0, 0.0)
                };

                if i > 0 {
                    let m = t.submatrix(0, 0, i, i);
                    let mut v = t.subcolumn(i, 0, i);
                    gsl::blas::ztrmv(CblasUplo::Upper, CblasTranspose::NoTrans, diag, &m, &mut v)?;
                    gsl::blas::zscal(aii, &mut v)?;
                }
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let j = n - i - 1;
                let mut tjj = t.get_mut(j, j)?;
                let ajj = if diag == CblasDiag::NonUnit {
                    *tjj = tjj.inverse();
                    -(*tjj)
                } else {
                    Complex::new(-1.0, 0.0)
                };

                if j < n - 1 {
                    let m = t.submatrix(j + 1, j + 1, n - j - 1, n - j - 1);
                    let mut v = t.subcolumn(j, j + 1, n - j - 1);
                    gsl::blas::ztrmv(CblasUplo::Lower, CblasTranspose::NoTrans, diag, &m, &mut v)?;
                    gsl::blas::zscal(ajj, &mut v)?;
                }
            }
        }
    }

    Ok(())
}

fn complex_tri_invert_l3(
    uplo: CblasUplo,
    diag: CblasDiag,
    t: &mut MatrixComplexF64,
) -> Result<()> {
    let n = t.size1();
    if n != t.size2() {
        return Err(GslError::MatrixNotSquare);
    }

    if n <= 24 {
        return complex_tri_invert_l2(uplo, diag, t);
    }

    let n1 = if n >= 8 {
        ((n + 4) / 8) * 4
    } else {
        n / 2
    };
    let n2 = n - n1;

    let mut t11 = t.submatrix(0, 0, n1, n1);
    let mut t12 = t.submatrix(0, n1, n1, n2);
    let mut t21 = t.submatrix(n1, 0, n2, n1);
    let mut t22 = t.submatrix(n1, n1, n2, n2);

    complex_tri_invert_l3(uplo, diag, &mut t11)?;

    match uplo {
        CblasUplo::Lower => {
            gsl::blas::ztrmm(
                CblasSide::Right,
                uplo,
                CblasTranspose::NoTrans,
                diag,
                Complex::new(-1.0, 0.0),
                &t11,
                &mut t21,
            )?;
            gsl::blas::ztrsm(
                CblasSide::Left,
                uplo,
                CblasTranspose::NoTrans,
                diag,
                Complex::new(1.0, 0.0),
                &t22,
                &mut t21,
            )?;
        }
        CblasUplo::Upper => {
            gsl::blas::ztrmm(
                CblasSide::Left,
                uplo,
                CblasTranspose::NoTrans,
                diag,
                Complex::new(-1.0, 0.0),
                &t11,
                &mut t12,
            )?;
            gsl::blas::ztrsm(
                CblasSide::Right,
                uplo,
                CblasTranspose::NoTrans,
                diag,
                Complex::new(1.0, 0.0),
                &t22,
                &mut t12,
            )?;
        }
    }

    complex_tri_invert_l3(uplo, diag, &mut t22)?;

    Ok(())
}

fn triangular_singular(t: &MatrixComplexF64) -> Result<()> {
    for i in 0..t.size1() {
        let z = t.get(i, i)?;
        if z.re() == 0.0 && z.im() == 0.0 {
            return Err(GslError::SingularMatrix);
        }
    }
    Ok(())
}