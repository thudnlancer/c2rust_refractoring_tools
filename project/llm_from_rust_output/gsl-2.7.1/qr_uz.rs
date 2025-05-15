use gsl::{
    blas::{
        CblasDiag, CblasSide, CblasTranspose, CblasUplo, dnrm2, dscal, dgemm, dtrmm,
    },
    linalg::QR_UU_decomp,
    matrix::{Matrix, MatrixView, Submatrix, Vector, VectorView},
    types::{GslError, GslResult},
    value::GslValue,
};

pub fn gsl_linalg_QR_UZ_decomp(
    S: &mut Matrix,
    A: &mut Matrix,
    T: &mut Matrix,
) -> GslResult<()> {
    let M = A.size1();
    let N = S.size1();

    if M < N {
        return Err(GslError::BadLen("M must be >= N"));
    } else if N != S.size2() {
        return Err(GslError::NotSquare("S matrix must be square"));
    } else if N != A.size2() {
        return Err(GslError::BadLen("S and A must have same number of columns"));
    } else if T.size1() != N || T.size2() != N {
        return Err(GslError::BadLen("T matrix has wrong dimensions"));
    } else if M == N {
        return QR_UU_decomp(S, A, T);
    } else if N == 1 {
        let T00 = T.get_mut(0, 0)?;
        let S00 = S.get_mut(0, 0)?;
        let mut v = A.column(0)?;
        *T00 = qrtz_householder_transform(S00, &mut v)?;
        Ok(())
    } else {
        let N1 = N / 2;
        let N2 = N - N1;

        let mut S11 = S.submatrix(0, 0, N1, N1)?;
        let mut S12 = S.submatrix(0, N1, N1, N2)?;
        let mut S22 = S.submatrix(N1, N1, N2, N2)?;

        let mut A1 = A.submatrix(0, 0, M - N, N1)?;
        let mut A2 = A.submatrix(0, N1, M - N, N2)?;

        let mut U11 = A.submatrix(M - N, 0, N1, N1)?;
        let mut U12 = A.submatrix(M - N, N1, N1, N2)?;

        let mut T11 = T.submatrix(0, 0, N1, N1)?;
        let mut T12 = T.submatrix(0, N1, N1, N2)?;
        let mut T22 = T.submatrix(N1, N1, N2, N2)?;

        let mut m = A.submatrix(0, 0, M - N2, N1)?;
        gsl_linalg_QR_UZ_decomp(&mut S11, &mut m, &mut T11)?;

        T12.copy_from(&U12)?;
        dtrmm(
            CblasSide::Left,
            CblasUplo::Upper,
            CblasTranspose::Trans,
            CblasDiag::NonUnit,
            1.0,
            &U11,
            &mut T12,
        )?;
        T12.add(&S12)?;
        dgemm(
            CblasTranspose::Trans,
            CblasTranspose::NoTrans,
            1.0,
            &A1,
            &A2,
            1.0,
            &mut T12,
        )?;
        dtrmm(
            CblasSide::Left,
            CblasUplo::Upper,
            CblasTranspose::Trans,
            CblasDiag::NonUnit,
            1.0,
            &T11,
            &mut T12,
        )?;
        dgemm(
            CblasTranspose::NoTrans,
            CblasTranspose::NoTrans,
            -1.0,
            &A1,
            &T12,
            1.0,
            &mut A2,
        )?;
        dgemm(
            CblasTranspose::NoTrans,
            CblasTranspose::NoTrans,
            -1.0,
            &U11,
            &T12,
            1.0,
            &mut U12,
        )?;
        S12.sub(&T12)?;

        let mut m = A.submatrix(0, N1, M, N2)?;
        gsl_linalg_QR_UZ_decomp(&mut S22, &mut m, &mut T22)?;

        T12.copy_from(&U12)?;
        dtrmm(
            CblasSide::Left,
            CblasUplo::Upper,
            CblasTranspose::Trans,
            CblasDiag::NonUnit,
            1.0,
            &U11,
            &mut T12,
        )?;
        dgemm(
            CblasTranspose::Trans,
            CblasTranspose::NoTrans,
            1.0,
            &A1,
            &A2,
            1.0,
            &mut T12,
        )?;
        dtrmm(
            CblasSide::Left,
            CblasUplo::Upper,
            CblasTranspose::NoTrans,
            CblasDiag::NonUnit,
            -1.0,
            &T11,
            &mut T12,
        )?;
        dtrmm(
            CblasSide::Right,
            CblasUplo::Upper,
            CblasTranspose::NoTrans,
            CblasDiag::NonUnit,
            1.0,
            &T22,
            &mut T12,
        )?;

        Ok(())
    }
}

fn qrtz_householder_transform(
    v0: &mut f64,
    v: &mut Vector,
) -> Result<f64, GslError> {
    let xnorm = dnrm2(v)?;
    if xnorm == 0.0 {
        return Ok(0.0);
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
    let tau = (beta - alpha) / beta;
    let s = alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        dscal(1.0 / s, v)?;
    } else {
        dscal(f64::EPSILON / s, v)?;
        dscal(1.0 / f64::EPSILON, v)?;
    }
    *v0 = beta;

    Ok(tau)
}