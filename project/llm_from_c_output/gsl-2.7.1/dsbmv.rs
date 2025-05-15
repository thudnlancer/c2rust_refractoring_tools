use ndarray::{ArrayView1, ArrayView2, s};
use num_traits::Zero;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid matrix order")]
    InvalidOrder,
    #[error("invalid uplo parameter")]
    InvalidUplo,
    #[error("invalid dimension")]
    InvalidDimension,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
    #[error("invalid increment")]
    InvalidIncrement,
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

pub fn cblas_dsbmv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    k: usize,
    alpha: f64,
    a: ArrayView2<f64>,
    lda: usize,
    x: ArrayView1<f64>,
    inc_x: isize,
    beta: f64,
    y: &mut ArrayView1<f64>,
    inc_y: isize,
) -> Result<(), BlasError> {
    // Validate parameters
    if n == 0 {
        return Ok(());
    }

    if k >= n {
        return Err(BlasError::InvalidDimension);
    }

    if lda < k + 1 {
        return Err(BlasError::InvalidLeadingDimension);
    }

    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    if inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    // Scale Y by beta
    if !beta.is_one() {
        if beta.is_zero() {
            y.fill(0.0);
        } else {
            *y *= beta;
        }
    }

    // Perform the matrix-vector multiplication
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x as usize];
                let mut temp2 = 0.0;
                let start = if k > i { 0 } else { i - k };
                let end = i.saturating_sub(1);
                
                for j in start..=end {
                    y[j * inc_y as usize] += temp1 * a[[i - j, j]];
                    temp2 += a[[i - j, j]] * x[j * inc_x as usize];
                }
                
                y[i * inc_y as usize] += temp1 * a[[0, i]] + alpha * temp2;
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x as usize];
                let mut temp2 = 0.0;
                let start = i + 1;
                let end = (i + k).min(n - 1);
                
                for j in start..=end {
                    y[j * inc_y as usize] += temp1 * a[[j - i, i]];
                    temp2 += a[[j - i, i]] * x[j * inc_x as usize];
                }
                
                y[i * inc_y as usize] += temp1 * a[[0, i]] + alpha * temp2;
            }
        }
    }

    Ok(())
}