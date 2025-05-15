use ndarray::{ArrayView1, ArrayViewMut2, Axis, Ix1, Ix2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("invalid increment")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasUplo {
    Upper,
    Lower,
}

pub fn cblas_dsyr(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f64,
    x: ArrayView1<f64>,
    inc_x: isize,
    mut a: ArrayViewMut2<f64>,
    lda: usize,
) -> Result<(), BlasError> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    if lda < n {
        return Err(BlasError::InvalidLeadingDimension);
    }

    if x.len() < ((n - 1) / inc_x.unsigned_abs() + 1) {
        return Err(BlasError::InvalidDimension("x length too small".to_string()));
    }

    if a.shape()[0] < n || a.shape()[1] < n {
        return Err(BlasError::InvalidDimension("a dimensions too small".to_string()));
    }

    match order {
        CblasOrder::RowMajor | CblasOrder::ColMajor => (),
        _ => return Err(BlasError::InvalidOrder),
    }

    match uplo {
        CblasUplo::Upper | CblasUplo::Lower => (),
        _ => return Err(BlasError::InvalidUplo),
    }

    for i in 0..n {
        let xi = if inc_x > 0 {
            x[i * inc_x as usize]
        } else {
            x[(n - i - 1) * (-inc_x) as usize]
        };

        let start_j = match uplo {
            CblasUplo::Upper => 0,
            CblasUplo::Lower => i,
        };
        let end_j = match uplo {
            CblasUplo::Upper => i + 1,
            CblasUplo::Lower => n,
        };

        for j in start_j..end_j {
            let xj = if inc_x > 0 {
                x[j * inc_x as usize]
            } else {
                x[(n - j - 1) * (-inc_x) as usize]
            };

            let (row, col) = match order {
                CblasOrder::RowMajor => (i, j),
                CblasOrder::ColMajor => (j, i),
            };

            a[[row, col]] += alpha * xi * xj;
        }
    }

    Ok(())
}