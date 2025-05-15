use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    linalg::{cholesky_decomp, cholesky_decomp1, cholesky_invert, cholesky_solve, cholesky_svx},
    matrix::{Matrix, MatrixView},
    vector::Vector,
};

pub fn gsl_linalg_cholesky_decomp(A: &mut Matrix) -> GslResult<()> {
    cholesky_decomp1(A)?;
    A.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit, A)
}

pub fn gsl_linalg_cholesky_decomp1(A: &mut Matrix) -> GslResult<()> {
    if A.size1() != A.size2() {
        return Err(GslError::NotSquare);
    }
    A.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit, A)?;
    cholesky_decomp_L3(A)
}

pub fn gsl_linalg_cholesky_solve(
    LLT: &Matrix,
    b: &Vector,
    x: &mut Vector,
) -> GslResult<()> {
    if LLT.size1() != LLT.size2() {
        return Err(GslError::NotSquare);
    }
    if LLT.size1() != b.len() {
        return Err(GslError::BadLength);
    }
    if LLT.size2() != x.len() {
        return Err(GslError::BadLength);
    }

    x.copy_from(b)?;
    gsl_linalg_cholesky_svx(LLT, x)
}

pub fn gsl_linalg_cholesky_svx(LLT: &Matrix, x: &mut Vector) -> GslResult<()> {
    if LLT.size1() != LLT.size2() {
        return Err(GslError::NotSquare);
    }
    if LLT.size2() != x.len() {
        return Err(GslError::BadLength);
    }

    x.trsv(CblasUplo::Lower, CblasTranspose::NoTrans, CblasDiag::NonUnit, LLT)?;
    x.trsv(CblasUplo::Lower, CblasTranspose::Trans, CblasDiag::NonUnit, LLT)?;
    Ok(())
}

pub fn gsl_linalg_cholesky_invert(LLT: &mut Matrix) -> GslResult<()> {
    if LLT.size1() != LLT.size2() {
        return Err(GslError::NotSquare);
    }

    LLT.tri_invert(CblasUplo::Lower, CblasDiag::NonUnit)?;
    LLT.tri_LTL()?;
    LLT.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit, LLT)
}

fn cholesky_decomp_L3(A: &mut Matrix) -> GslResult<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(GslError::NotSquare);
    }

    if n <= 24 {
        return cholesky_decomp_L2(A);
    }

    let n1 = if n >= 16 {
        (n + 8) / 16 * 8
    } else {
        n / 2
    };
    let n2 = n - n1;

    let (a11, a21, a22) = A.split_at(n1, n1);
    cholesky_decomp_L3(a11)?;
    a21.trsm(
        CblasSide::Right,
        CblasUplo::Lower,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        a11,
    )?;
    a22.syrk(
        CblasUplo::Lower,
        CblasTranspose::NoTrans,
        -1.0,
        a21,
        1.0,
    )?;
    cholesky_decomp_L3(a22)
}

fn cholesky_decomp_L2(A: &mut Matrix) -> GslResult<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(GslError::NotSquare);
    }

    for j in 0..n {
        let mut v = A.subcolumn(j, j, n - j);
        if j > 0 {
            let w = A.subrow(j, 0, j);
            let m = A.submatrix(j, 0, n - j, j);
            v.gemv(CblasTranspose::NoTrans, -1.0, &m, &w, 1.0)?;
        }

        let ajj = A.get(j, j);
        if ajj <= 0.0 {
            return Err(GslError::Domain);
        }

        let ajj = ajj.sqrt();
        v.scale(1.0 / ajj)?;
        A.set(j, j, ajj);
    }

    Ok(())
}