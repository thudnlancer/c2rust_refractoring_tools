use ndarray::{ArrayView1, ArrayViewMut1};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid uplo parameter")]
    InvalidUplo,
    #[error("Invalid dimension: {0}")]
    InvalidDimension(usize),
    #[error("Invalid increment: {0}")]
    InvalidIncrement(isize),
}

#[derive(Copy, Clone, Debug)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasUplo {
    Upper,
    Lower,
}

pub fn cblas_sspr(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f32,
    x: ArrayView1<f32>,
    inc_x: isize,
    ap: ArrayViewMut1<f32>,
) -> Result<(), BlasError> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement(inc_x));
    }

    if x.len() < ((n - 1) / inc_x.unsigned_abs() + 1) {
        return Err(BlasError::InvalidDimension(x.len()));
    }

    if ap.len() < n * (n + 1) / 2 {
        return Err(BlasError::InvalidDimension(ap.len()));
    }

    let mut ap_index = 0;
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let xi = x[i * inc_x as usize];
                let temp = alpha * xi;
                for j in i..n {
                    ap[ap_index] += temp * x[j * inc_x as usize];
                    ap_index += 1;
                }
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let xi = x[i * inc_x as usize];
                let temp = alpha * xi;
                for j in 0..=i {
                    ap[ap_index] += temp * x[j * inc_x as usize];
                    ap_index += 1;
                }
            }
        }
    }

    Ok(())
}