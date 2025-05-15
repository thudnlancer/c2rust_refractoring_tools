use ndarray::{Array1, Array2};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CBlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimension")]
    InvalidDimension,
    #[error("invalid increment")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

#[derive(Copy, Clone, Debug)]
pub enum CBLAS_ORDER {
    RowMajor,
    ColMajor,
}

#[derive(Copy, Clone, Debug)]
pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

pub fn cblas_chemv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: usize,
    alpha: Complex32,
    a: &Array2<Complex32>,
    lda: usize,
    x: &Array1<Complex32>,
    inc_x: usize,
    beta: Complex32,
    y: &mut Array1<Complex32>,
    inc_y: usize,
) -> Result<(), CBlasError> {
    if n == 0 {
        return Ok(());
    }

    if a.nrows() != n || a.ncols() != n {
        return Err(CBlasError::InvalidDimension);
    }

    if lda < n {
        return Err(CBlasError::InvalidLeadingDimension);
    }

    if x.len() != n * inc_x {
        return Err(CBlasError::InvalidDimension);
    }

    if y.len() != n * inc_y {
        return Err(CBlasError::InvalidDimension);
    }

    if inc_x == 0 {
        return Err(CBlasError::InvalidIncrement);
    }

    if inc_y == 0 {
        return Err(CBlasError::InvalidIncrement);
    }

    match uplo {
        CBLAS_UPLO::Upper | CBLAS_UPLO::Lower => (),
        _ => return Err(CBlasError::InvalidUplo),
    }

    // Perform the Hermitian matrix-vector multiplication
    for i in 0..n {
        let mut temp = Complex32::new(0.0, 0.0);
        for j in 0..n {
            let a_ij = match uplo {
                CBLAS_UPLO::Upper if j >= i => a[(i, j)],
                CBLAS_UPLO::Lower if j <= i => a[(i, j)],
                _ => Complex32::new(0.0, 0.0),
            };
            temp += a_ij * x[j * inc_x];
        }
        y[i * inc_y] = alpha * temp + beta * y[i * inc_y];
    }

    Ok(())
}