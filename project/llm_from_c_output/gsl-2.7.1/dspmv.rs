use ndarray::{Array1, ArrayView1};
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

#[derive(Debug, Clone, Copy)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy)]
pub enum CblasUplo {
    Upper,
    Lower,
}

pub fn cblas_dspmv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f64,
    ap: ArrayView1<f64>,
    x: ArrayView1<f64>,
    inc_x: isize,
    beta: f64,
    y: &mut Array1<f64>,
    inc_y: isize,
) -> Result<(), BlasError> {
    if n == 0 {
        return Ok(());
    }

    if ap.len() != n * (n + 1) / 2 {
        return Err(BlasError::InvalidDimension(ap.len()));
    }

    if x.len() != ((n as isize - 1) * inc_x.abs() + 1) as usize {
        return Err(BlasError::InvalidDimension(x.len()));
    }

    if y.len() != ((n as isize - 1) * inc_y.abs() + 1) as usize {
        return Err(BlasError::InvalidDimension(y.len()));
    }

    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement(inc_x));
    }

    if inc_y == 0 {
        return Err(BlasError::InvalidIncrement(inc_y));
    }

    // Scale Y by beta
    if beta != 1.0 {
        if beta == 0.0 {
            y.fill(0.0);
        } else {
            *y *= beta;
        }
    }

    // Perform the matrix-vector multiplication
    match uplo {
        CblasUplo::Upper => {
            let mut ap_index = 0;
            for i in 0..n {
                let xi = x[i * inc_x as usize];
                let mut sum = 0.0;
                
                for j in 0..i {
                    let yj = j * inc_y as usize;
                    let a = ap[ap_index];
                    y[yj] += alpha * a * xi;
                    sum += a * x[j * inc_x as usize];
                    ap_index += 1;
                }
                
                let a = ap[ap_index];
                sum += a * xi;
                y[i * inc_y as usize] += alpha * sum;
                ap_index += 1;
            }
        }
        CblasUplo::Lower => {
            let mut ap_index = 0;
            for i in 0..n {
                let xi = x[i * inc_x as usize];
                let mut sum = 0.0;
                
                let a = ap[ap_index];
                sum += a * xi;
                y[i * inc_y as usize] += alpha * sum;
                ap_index += 1;
                
                for j in (i + 1)..n {
                    let yj = j * inc_y as usize;
                    let a = ap[ap_index];
                    y[yj] += alpha * a * xi;
                    sum += a * x[j * inc_x as usize];
                    ap_index += 1;
                }
            }
        }
    }

    Ok(())
}