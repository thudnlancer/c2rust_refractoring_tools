use ndarray::{ArrayView1, ArrayView2, Axis};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymvError {
    #[error("invalid matrix order")]
    InvalidOrder,
    #[error("invalid uplo parameter")]
    InvalidUplo,
    #[error("invalid matrix dimensions")]
    InvalidDimensions,
    #[error("invalid increment parameters")]
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

pub fn cblas_ssymv(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f32,
    a: ArrayView2<f32>,
    lda: usize,
    x: ArrayView1<f32>,
    inc_x: isize,
    beta: f32,
    y: &mut ArrayView1<f32>,
    inc_y: isize,
) -> Result<(), SymvError> {
    // Validate parameters
    if n == 0 {
        return Ok(());
    }

    if a.shape()[0] != n || a.shape()[1] != n {
        return Err(SymvError::InvalidDimensions);
    }

    if lda < n {
        return Err(SymvError::InvalidDimensions);
    }

    if x.len() != n || y.len() != n {
        return Err(SymvError::InvalidDimensions);
    }

    if inc_x == 0 || inc_y == 0 {
        return Err(SymvError::InvalidIncrement);
    }

    // Compute y = beta * y
    if beta != 1.0 {
        for elem in y.iter_mut() {
            *elem *= beta;
        }
    }

    // Compute y += alpha * A * x
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let mut temp = alpha * x[i];
                let mut temp2 = 0.0;
                for j in i + 1..n {
                    temp += alpha * a[[i, j]] * x[j];
                    temp2 += alpha * a[[i, j]] * x[i];
                }
                y[i] += temp;
                if i < n - 1 {
                    y[i + 1] += temp2;
                }
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let mut temp = alpha * x[i];
                let mut temp2 = 0.0;
                for j in 0..i {
                    temp += alpha * a[[i, j]] * x[j];
                    temp2 += alpha * a[[i, j]] * x[i];
                }
                y[i] += temp;
                if i > 0 {
                    y[i - 1] += temp2;
                }
            }
        }
    }

    Ok(())
}