use ndarray::{Array2, Array1};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CblasError {
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

pub fn cblas_cgerc(
    order: CblasOrder,
    m: usize,
    n: usize,
    alpha: Complex32,
    x: &Array1<Complex32>,
    inc_x: usize,
    y: &Array1<Complex32>,
    inc_y: usize,
    a: &mut Array2<Complex32>,
) -> Result<(), CblasError> {
    if m == 0 || n == 0 {
        return Ok(());
    }

    if x.len() < (m - 1) * inc_x + 1 {
        return Err(CblasError::InvalidDimensions);
    }

    if y.len() < (n - 1) * inc_y + 1 {
        return Err(CblasError::InvalidDimensions);
    }

    if a.shape() != [m, n] {
        return Err(CblasError::InvalidDimensions);
    }

    if inc_x == 0 {
        return Err(CblasError::InvalidIncrement);
    }

    if inc_y == 0 {
        return Err(CblasError::InvalidIncrement);
    }

    match order {
        CblasOrder::RowMajor => {
            for i in 0..m {
                for j in 0..n {
                    let xi = x[i * inc_x];
                    let yj = y[j * inc_y].conj();
                    a[(i, j)] += alpha * xi * yj;
                }
            }
        }
        CblasOrder::ColMajor => {
            for j in 0..n {
                for i in 0..m {
                    let xi = x[i * inc_x];
                    let yj = y[j * inc_y].conj();
                    a[(i, j)] += alpha * xi * yj;
                }
            }
        }
    }

    Ok(())
}