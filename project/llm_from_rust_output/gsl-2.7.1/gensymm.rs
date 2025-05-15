use gsl::{
    blas::{self, Layout, Transpose, Side, Diag, Uplo},
    eigen::{SymmWorkspace, SymmGenWorkspace},
    linalg::Cholesky,
    matrix::{Matrix, MatrixView, MatrixViewMut},
    vector::Vector,
    Value, types::{GslResult, GslError},
};

pub fn eigen_gensymm_alloc(n: usize) -> Result<SymmGenWorkspace, GslError> {
    if n == 0 {
        return Err(GslError::InvalidArgument("matrix dimension must be positive integer"));
    }

    let symm_workspace = SymmWorkspace::new(n)?;
    Ok(SymmGenWorkspace::new(n, symm_workspace))
}

pub fn eigen_gensymm(
    a: &mut Matrix,
    b: &mut Matrix,
    eval: &mut Vector,
    workspace: &mut SymmGenWorkspace,
) -> GslResult<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(GslError::MatrixNotSquare);
    }
    if n != b.size1() || n != b.size2() {
        return Err(GslError::InvalidDimensions("B matrix dimensions must match A"));
    }
    if eval.len() != n {
        return Err(GslError::InvalidDimensions("eigenvalue vector must match matrix size"));
    }
    if workspace.size() != n {
        return Err(GslError::InvalidDimensions("matrix size does not match workspace"));
    }

    Cholesky::decomp(b)?;
    eigen_gensymm_standardize(a, b)?;
    workspace.symm_workspace().symm(a, eval)
}

pub fn eigen_gensymm_standardize(a: &mut Matrix, b: &Matrix) -> GslResult<()> {
    gensymm_standardize_l3(a, b)
}

fn gensymm_standardize_l2(a: &mut Matrix, b: &Matrix) -> GslResult<()> {
    let n = a.size1();
    
    for i in 0..n {
        let mut a_ii = a.get(i, i)?;
        let b_ii = b.get(i, i)?;
        a_ii /= b_ii * b_ii;
        a.set(i, i, a_ii)?;

        if i < n - 1 {
            let mut ai = a.column_mut(i).subvector_mut(i + 1, n - i - 1)?;
            let mut ma = a.submatrix_mut(i + 1, i + 1, n - i - 1, n - i - 1)?;
            let bi = b.column(i).subvector(i + 1, n - i - 1)?;
            let mb = b.submatrix(i + 1, i + 1, n - i - 1, n - i - 1)?;

            blas::scal(1.0 / b_ii, &mut ai)?;
            let c = -0.5 * a_ii;
            blas::axpy(c, &bi, &mut ai)?;
            blas::syr2(Uplo::Lower, -1.0, &ai, &bi, &mut ma)?;
            blas::axpy(c, &bi, &mut ai)?;
            blas::trsv(Layout::RowMajor, Uplo::Lower, Transpose::NoTrans, Diag::NonUnit, &mb, &mut ai)?;
        }
    }

    Ok(())
}

fn gensymm_standardize_l3(a: &mut Matrix, b: &Matrix) -> GslResult<()> {
    let n = a.size1();
    if n <= 24 {
        return gensymm_standardize_l2(a, b);
    }

    let n1 = if n >= 16 {
        ((n + 8) / 16) * 8
    } else {
        n / 2
    };
    let n2 = n - n1;

    let mut a11 = a.submatrix_mut(0, 0, n1, n1)?;
    let mut a21 = a.submatrix_mut(n1, 0, n2, n1)?;
    let mut a22 = a.submatrix_mut(n1, n1, n2, n2)?;

    let b11 = b.submatrix(0, 0, n1, n1)?;
    let b21 = b.submatrix(n1, 0, n2, n1)?;
    let b22 = b.submatrix(n1, n1, n2, n2)?;

    gensymm_standardize_l3(&mut a11, &b11)?;

    blas::trsm(
        Layout::RowMajor,
        Side::Right,
        Uplo::Lower,
        Transpose::Trans,
        Diag::NonUnit,
        1.0,
        &b11,
        &mut a21,
    )?;

    blas::symm(
        Layout::RowMajor,
        Side::Right,
        Uplo::Lower,
        -0.5,
        &a11,
        &b21,
        1.0,
        &mut a21,
    )?;

    blas::syr2k(
        Layout::RowMajor,
        Uplo::Lower,
        Transpose::NoTrans,
        -1.0,
        &a21,
        &b21,
        1.0,
        &mut a22,
    )?;

    blas::symm(
        Layout::RowMajor,
        Side::Right,
        Uplo::Lower,
        -0.5,
        &a11,
        &b21,
        1.0,
        &mut a21,
    )?;

    blas::trsm(
        Layout::RowMajor,
        Side::Left,
        Uplo::Lower,
        Transpose::NoTrans,
        Diag::NonUnit,
        1.0,
        &b22,
        &mut a21,
    )?;

    gensymm_standardize_l3(&mut a22, &b22)?;

    Ok(())
}