use ndarray::{ArrayView1, ArrayViewMut2, Axis, Ix1, Ix2};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("invalid increment: {0}")]
    InvalidIncrement(String),
    #[error("invalid leading dimension: {0}")]
    InvalidLeadingDimension(String),
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

pub fn cblas_ssyr(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f32,
    x: ArrayView1<f32>,
    incx: usize,
    a: &mut ArrayViewMut2<f32>,
) -> Result<(), BlasError> {
    if n == 0 {
        return Ok(());
    }

    if incx == 0 {
        return Err(BlasError::InvalidIncrement("incx must not be zero".to_string()));
    }

    if x.len() < 1 + (n - 1) * incx {
        return Err(BlasError::InvalidDimension(
            "x length is insufficient".to_string(),
        ));
    }

    if a.shape()[0] != n || a.shape()[1] != n {
        return Err(BlasError::InvalidDimension(
            "a must be square matrix with size n x n".to_string(),
        ));
    }

    match uplo {
        CblasUplo::Upper | CblasUplo::Lower => {}
        _ => return Err(BlasError::InvalidUplo),
    }

    let x_slice = x.as_slice().unwrap();

    for j in 0..n {
        let x_j = x_slice[j * incx];
        let temp = alpha * x_j;

        match uplo {
            CblasUplo::Upper => {
                for i in 0..=j {
                    a[[i, j]] += x_slice[i * incx] * temp;
                }
            }
            CblasUplo::Lower => {
                for i in j..n {
                    a[[i, j]] += x_slice[i * incx] * temp;
                }
            }
        }
    }

    Ok(())
}