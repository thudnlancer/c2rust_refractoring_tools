use gsl::{
    blas::{CBLAS_DIAG, CBLAS_SIDE, CBLAS_TRANSPOSE, CBLAS_UPLO},
    complex::{Complex, ComplexF64},
    error::{GslError, GslResult},
    matrix_complex::{MatrixComplex, MatrixComplexView, MatrixComplexConstView},
    vector_complex::{VectorComplex, VectorComplexView},
};

pub fn gsl_linalg_complex_QR_decomp_r(
    A: &mut MatrixComplex,
    T: &mut MatrixComplex,
) -> GslResult<()> {
    let M = A.size1();
    let N = A.size2();

    if M < N {
        return Err(GslError::BadLen);
    } else if T.size1() != T.size2() {
        return Err(GslError::NotSquare);
    } else if T.size1() != N {
        return Err(GslError::BadLen);
    } else if N == 1 {
        let T00 = T.get_mut(0, 0)?;
        let mut v = A.column(0)?;
        *T00 = gsl_linalg_complex_householder_transform(&mut v)?;
        return Ok(());
    }

    let N1 = N / 2;
    let N2 = N - N1;
    let M2 = M - N1;

    let mut A11 = A.submatrix(0, 0, N1, N1)?;
    let mut A12 = A.submatrix(0, N1, N1, N2)?;
    let mut A21 = A.submatrix(N1, 0, M2, N1)?;
    let mut A22 = A.submatrix(N1, N1, M2, N2)?;

    let mut T11 = T.submatrix(0, 0, N1, N1)?;
    let mut T12 = T.submatrix(0, N1, N1, N2)?;
    let mut T22 = T.submatrix(N1, N1, N2, N2)?;

    let mut m = A.submatrix(0, 0, M, N1)?;
    gsl_linalg_complex_QR_decomp_r(&mut m, &mut T11)?;

    T12.copy_from(&A12)?;
    T12.trmm(
        CBLAS_SIDE::Left,
        CBLAS_UPLO::Lower,
        CBLAS_TRANSPOSE::ConjTrans,
        CBLAS_DIAG::Unit,
        ComplexF64::new(1.0, 0.0),
        &A11,
    )?;

    T12.gemm(
        CBLAS_TRANSPOSE::ConjTrans,
        CBLAS_TRANSPOSE::NoTrans,
        ComplexF64::new(1.0, 0.0),
        &A21,
        &A22,
        ComplexF64::new(1.0, 0.0),
    )?;

    T12.trmm(
        CBLAS_SIDE::Left,
        CBLAS_UPLO::Upper,
        CBLAS_TRANSPOSE::ConjTrans,
        CBLAS_DIAG::NonUnit,
        ComplexF64::new(1.0, 0.0),
        &T11,
    )?;

    A22.gemm(
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_TRANSPOSE::NoTrans,
        ComplexF64::new(-1.0, 0.0),
        &A21,
        &T12,
        ComplexF64::new(1.0, 0.0),
    )?;

    T12.trmm(
        CBLAS_SIDE::Left,
        CBLAS_UPLO::Lower,
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_DIAG::Unit,
        ComplexF64::new(1.0, 0.0),
        &A11,
    )?;

    A12.sub(&T12)?;
    gsl_linalg_complex_QR_decomp_r(&mut A22, &mut T22)?;

    let mut m = A21.submatrix(0, 0, N2, N1)?;
    T12.conjtrans_memcpy(&m)?;

    let m = A.submatrix(N1, N1, N2, N2)?;
    T12.trmm(
        CBLAS_SIDE::Right,
        CBLAS_UPLO::Lower,
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_DIAG::Unit,
        ComplexF64::new(1.0, 0.0),
        &m,
    )?;

    if M > N {
        let V31 = A.submatrix(N, 0, M - N, N1)?;
        let V32 = A.submatrix(N, N1, M - N, N2)?;
        T12.gemm(
            CBLAS_TRANSPOSE::ConjTrans,
            CBLAS_TRANSPOSE::NoTrans,
            ComplexF64::new(1.0, 0.0),
            &V31,
            &V32,
            ComplexF64::new(1.0, 0.0),
        )?;
    }

    T12.trmm(
        CBLAS_SIDE::Left,
        CBLAS_UPLO::Upper,
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_DIAG::NonUnit,
        ComplexF64::new(-1.0, 0.0),
        &T11,
    )?;

    T12.trmm(
        CBLAS_SIDE::Right,
        CBLAS_UPLO::Upper,
        CBLAS_TRANSPOSE::NoTrans,
        CBLAS_DIAG::NonUnit,
        ComplexF64::new(1.0, 0.0),
        &T22,
    )?;

    Ok(())
}

// 其他函数类似地进行转换...