use ndarray::{Array1, Array2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("Invalid order parameter")]
    InvalidOrder,
    #[error("Invalid uplo parameter")]
    InvalidUplo,
    #[error("Invalid dimension")]
    InvalidDimension,
    #[error("Invalid increment")]
    InvalidIncrement,
    #[error("Invalid leading dimension")]
    InvalidLeadingDimension,
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

pub fn cblas_ssbmv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    k: usize,
    alpha: f32,
    a: &Array2<f32>,
    lda: usize,
    x: &Array1<f32>,
    inc_x: usize,
    beta: f32,
    y: &mut Array1<f32>,
    inc_y: usize,
) -> Result<(), BlasError> {
    // Validate parameters
    if n == 0 {
        return Ok(());
    }
    
    if k >= n {
        return Err(BlasError::InvalidDimension);
    }
    
    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement);
    }
    
    if inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }
    
    let expected_a_rows = match uplo {
        CblasUplo::Upper | CblasUplo::Lower => k + 1,
    };
    
    if a.nrows() != expected_a_rows || a.ncols() != n {
        return Err(BlasError::InvalidDimension);
    }
    
    if lda < k + 1 {
        return Err(BlasError::InvalidLeadingDimension);
    }
    
    if x.len() < 1 + (n - 1) * inc_x {
        return Err(BlasError::InvalidDimension);
    }
    
    if y.len() < 1 + (n - 1) * inc_y {
        return Err(BlasError::InvalidDimension);
    }

    // Scale Y by beta
    if beta != 1.0 {
        if beta == 0.0 {
            for i in 0..n {
                y[i * inc_y] = 0.0;
            }
        } else {
            for i in 0..n {
                y[i * inc_y] *= beta;
            }
        }
    }

    // Perform matrix-vector multiplication
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = 0.0;
                let min_k = n - i - 1;
                let actual_k = k.min(min_k);
                
                for j in 0..actual_k {
                    let a_index = match order {
                        CblasOrder::RowMajor => (j, i),
                        CblasOrder::ColMajor => (i, j),
                    };
                    
                    y[(i + j + 1) * inc_y] += temp1 * a[a_index];
                    temp2 += a[a_index] * x[(i + j + 1) * inc_x];
                }
                
                y[i * inc_y] += temp1 * a[(0, i)] + alpha * temp2;
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = 0.0;
                let actual_k = k.min(i);
                
                for j in 0..actual_k {
                    let a_index = match order {
                        CblasOrder::RowMajor => (k - j, i - j - 1),
                        CblasOrder::ColMajor => (i - j - 1, k - j),
                    };
                    
                    y[(i - j - 1) * inc_y] += temp1 * a[a_index];
                    temp2 += a[a_index] * x[(i - j - 1) * inc_x];
                }
                
                y[i * inc_y] += temp1 * a[(k, i)] + alpha * temp2;
            }
        }
    }

    Ok(())
}