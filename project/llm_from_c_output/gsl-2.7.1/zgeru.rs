use ndarray::{Array2, Array1};
use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid dimensions")]
    InvalidDimensions,
    #[error("invalid increment")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

pub fn cblas_zgeru(
    order: CBLAS_ORDER,
    m: usize,
    n: usize,
    alpha: Complex64,
    x: &Array1<Complex64>,
    inc_x: usize,
    y: &Array1<Complex64>,
    inc_y: usize,
    a: &mut Array2<Complex64>,
    lda: usize,
) -> Result<(), BlasError> {
    if m == 0 || n == 0 {
        return Ok(());
    }

    if x.len() < (m - 1) * inc_x + 1 {
        return Err(BlasError::InvalidDimensions);
    }

    if y.len() < (n - 1) * inc_y + 1 {
        return Err(BlasError::InvalidDimensions);
    }

    if a.shape() != [m, n] {
        return Err(BlasError::InvalidDimensions);
    }

    if lda < m {
        return Err(BlasError::InvalidLeadingDimension);
    }

    match order {
        CBLAS_ORDER::RowMajor => {
            for i in 0..m {
                for j in 0..n {
                    a[[i, j]] += alpha * x[i * inc_x] * y[j * inc_y].conj();
                }
            }
        }
        CBLAS_ORDER::ColMajor => {
            for j in 0..n {
                for i in 0..m {
                    a[[i, j]] += alpha * x[i * inc_x] * y[j * inc_y].conj();
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_ORDER {
    RowMajor,
    ColMajor,
}