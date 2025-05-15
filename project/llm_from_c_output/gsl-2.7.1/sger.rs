use ndarray::{Array2, Array1};
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

#[derive(Copy, Clone, Debug)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

pub fn cblas_sger(
    order: CblasOrder,
    m: usize,
    n: usize,
    alpha: f32,
    x: &Array1<f32>,
    inc_x: usize,
    y: &Array1<f32>,
    inc_y: usize,
    a: &mut Array2<f32>,
    lda: usize,
) -> Result<(), BlasError> {
    if m == 0 || n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement);
    }
    if inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    let expected_x_len = if m > 0 { 1 + (m - 1) * inc_x } else { 0 };
    let expected_y_len = if n > 0 { 1 + (n - 1) * inc_y } else { 0 };

    if x.len() < expected_x_len {
        return Err(BlasError::InvalidDimensions);
    }
    if y.len() < expected_y_len {
        return Err(BlasError::InvalidDimensions);
    }

    match order {
        CblasOrder::RowMajor => {
            if lda < n {
                return Err(BlasError::InvalidLeadingDimension);
            }
            if a.shape()[0] < m || a.shape()[1] < n {
                return Err(BlasError::InvalidDimensions);
            }
        }
        CblasOrder::ColMajor => {
            if lda < m {
                return Err(BlasError::InvalidLeadingDimension);
            }
            if a.shape()[0] < n || a.shape()[1] < m {
                return Err(BlasError::InvalidDimensions);
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            let xi = x[i * inc_x];
            let yj = y[j * inc_y];
            match order {
                CblasOrder::RowMajor => a[[i, j]] += alpha * xi * yj,
                CblasOrder::ColMajor => a[[j, i]] += alpha * xi * yj,
            }
        }
    }

    Ok(())
}