use ndarray::{Array2, Array1};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid dimensions: M={0}, N={1}")]
    InvalidDimensions(i32, i32),
    #[error("Invalid increment value: incX={0}, incY={1}")]
    InvalidIncrement(i32, i32),
    #[error("Invalid leading dimension: lda={0} (must be >= max(1, {1}))")]
    InvalidLeadingDimension(i32, i32),
}

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

pub fn cblas_dger(
    order: CblasOrder,
    m: i32,
    n: i32,
    alpha: f64,
    x: &Array1<f64>,
    inc_x: i32,
    y: &Array1<f64>,
    inc_y: i32,
    a: &mut Array2<f64>,
) -> Result<(), BlasError> {
    // Validate parameters
    if m <= 0 || n <= 0 {
        return Err(BlasError::InvalidDimensions(m, n));
    }
    if inc_x == 0 || inc_y == 0 {
        return Err(BlasError::InvalidIncrement(inc_x, inc_y));
    }
    let lda = match order {
        CblasOrder::RowMajor => a.ncols(),
        CblasOrder::ColMajor => a.nrows(),
    } as i32;
    if lda < m.max(1) {
        return Err(BlasError::InvalidLeadingDimension(lda, m));
    }

    // Perform the rank-1 update A := alpha*x*y' + A
    match order {
        CblasOrder::RowMajor => {
            for i in 0..m as usize {
                let xi = if inc_x > 0 {
                    x[i * inc_x as usize]
                } else {
                    x[(m as usize - 1 - i) * (-inc_x) as usize]
                };
                for j in 0..n as usize {
                    let yj = if inc_y > 0 {
                        y[j * inc_y as usize]
                    } else {
                        y[(n as usize - 1 - j) * (-inc_y) as usize]
                    };
                    a[[i, j]] += alpha * xi * yj;
                }
            }
        }
        CblasOrder::ColMajor => {
            for j in 0..n as usize {
                let yj = if inc_y > 0 {
                    y[j * inc_y as usize]
                } else {
                    y[(n as usize - 1 - j) * (-inc_y) as usize]
                };
                for i in 0..m as usize {
                    let xi = if inc_x > 0 {
                        x[i * inc_x as usize]
                    } else {
                        x[(m as usize - 1 - i) * (-inc_x) as usize]
                    };
                    a[[i, j]] += alpha * xi * yj;
                }
            }
        }
    }

    Ok(())
}