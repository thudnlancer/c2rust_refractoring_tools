use ndarray::{Array2, Array1};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeruError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid dimensions")]
    InvalidDimensions,
    #[error("invalid increment")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

pub fn cblas_cgeru(
    order: CblasOrder,
    m: usize,
    n: usize,
    alpha: Complex32,
    x: &Array1<Complex32>,
    inc_x: usize,
    y: &Array1<Complex32>,
    inc_y: usize,
    a: &mut Array2<Complex32>,
) -> Result<(), GeruError> {
    if m == 0 || n == 0 {
        return Ok(());
    }

    if x.len() < (m - 1) * inc_x + 1 {
        return Err(GeruError::InvalidDimensions);
    }

    if y.len() < (n - 1) * inc_y + 1 {
        return Err(GeruError::InvalidDimensions);
    }

    let lda = match order {
        CblasOrder::RowMajor => a.ncols(),
        CblasOrder::ColMajor => a.nrows(),
    };

    if lda < m {
        return Err(GeruError::InvalidLeadingDimension);
    }

    if inc_x <= 0 {
        return Err(GeruError::InvalidIncrement);
    }

    if inc_y <= 0 {
        return Err(GeruError::InvalidIncrement);
    }

    match order {
        CblasOrder::RowMajor => {
            for i in 0..m {
                for j in 0..n {
                    a[[i, j]] += alpha * x[i * inc_x] * y[j * inc_y].conj();
                }
            }
        }
        CblasOrder::ColMajor => {
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
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}