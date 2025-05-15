use ndarray::{Array1, ArrayView1};
use thiserror::Error;

#[derive(Error, Debug)]
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

pub fn cblas_dspr2(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f64,
    x: ArrayView1<f64>,
    inc_x: isize,
    y: ArrayView1<f64>,
    inc_y: isize,
    ap: &mut Array1<f64>,
) -> Result<(), BlasError> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err(BlasError::InvalidIncrement(inc_x));
    }

    if inc_y == 0 {
        return Err(BlasError::InvalidIncrement(inc_y));
    }

    let len_x = ((n as isize - 1) * inc_x.abs() + 1) as usize;
    let len_y = ((n as isize - 1) * inc_y.abs() + 1) as usize;

    if x.len() < len_x {
        return Err(BlasError::InvalidDimension(x.len()));
    }

    if y.len() < len_y {
        return Err(BlasError::InvalidDimension(y.len()));
    }

    let ap_len = n * (n + 1) / 2;
    if ap.len() < ap_len {
        return Err(BlasError::InvalidDimension(ap.len()));
    }

    let mut ap_index = 0;
    match uplo {
        CblasUplo::Upper => {
            for i in 0..n {
                let xi = x[i * inc_x as usize];
                let yi = y[i * inc_y as usize];
                for j in i..n {
                    let xj = x[j * inc_x as usize];
                    let yj = y[j * inc_y as usize];
                    ap[ap_index] += alpha * (xi * yj + xj * yi);
                    ap_index += 1;
                }
            }
        }
        CblasUplo::Lower => {
            for i in 0..n {
                let xi = x[i * inc_x as usize];
                let yi = y[i * inc_y as usize];
                for j in 0..=i {
                    let xj = x[j * inc_x as usize];
                    let yj = y[j * inc_y as usize];
                    ap[ap_index] += alpha * (xi * yj + xj * yi);
                    ap_index += 1;
                }
            }
        }
    }

    Ok(())
}