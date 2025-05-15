use ndarray::{Array2, Array1};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid order parameter")]
    InvalidOrder,
    #[error("invalid dimensions")]
    InvalidDimensions,
    #[error("invalid increment value")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

pub fn cblas_zgerc(
    order: CblasOrder,
    m: usize,
    n: usize,
    alpha: Complex64,
    x: &Array1<Complex64>,
    inc_x: isize,
    y: &Array1<Complex64>,
    inc_y: isize,
    a: &mut Array2<Complex64>,
    lda: usize,
) -> Result<(), BlasError> {
    if m == 0 || n == 0 {
        return Ok(());
    }

    if inc_x == 0 || inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    if x.len() < ((m - 1) / inc_x.unsigned_abs() + 1) {
        return Err(BlasError::InvalidDimensions);
    }

    if y.len() < ((n - 1) / inc_y.unsigned_abs() + 1) {
        return Err(BlasError::InvalidDimensions);
    }

    if a.shape()[0] < m || a.shape()[1] < n || lda < m {
        return Err(BlasError::InvalidLeadingDimension);
    }

    match order {
        CblasOrder::RowMajor => {
            for i in 0..m {
                let xi = x[(i * inc_x.unsigned_abs())].conj();
                for j in 0..n {
                    let yj = y[(j * inc_y.unsigned_abs())];
                    a[(i, j)] += alpha * xi * yj;
                }
            }
        }
        CblasOrder::ColMajor => {
            for j in 0..n {
                let yj = y[(j * inc_y.unsigned_abs())];
                for i in 0..m {
                    let xi = x[(i * inc_x.unsigned_abs())].conj();
                    a[(i, j)] += alpha * xi * yj;
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}