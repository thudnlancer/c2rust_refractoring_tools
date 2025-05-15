use ndarray::{Array2, Array1};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid transpose")]
    InvalidTranspose,
    #[error("invalid dimensions")]
    InvalidDimensions,
    #[error("invalid increment")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

pub fn cblas_cgemv(
    order: CblasOrder,
    trans_a: CblasTranspose,
    m: i32,
    n: i32,
    alpha: Complex32,
    a: &Array2<Complex32>,
    lda: i32,
    x: &Array1<Complex32>,
    inc_x: i32,
    beta: Complex32,
    y: &mut Array1<Complex32>,
    inc_y: i32,
) -> Result<(), BlasError> {
    if m <= 0 || n <= 0 {
        return Err(BlasError::InvalidDimensions);
    }
    if inc_x == 0 || inc_y == 0 {
        return Err(BlasError::InvalidIncrement);
    }
    if lda < std::cmp::max(1, if trans_a == CblasTranspose::NoTrans { m } else { n }) {
        return Err(BlasError::InvalidLeadingDimension);
    }

    let (rows, cols) = match trans_a {
        CblasTranspose::NoTrans => (m as usize, n as usize),
        _ => (n as usize, m as usize),
    };

    if a.shape() != [rows, cols] {
        return Err(BlasError::InvalidDimensions);
    }
    if x.len() != cols * inc_x as usize {
        return Err(BlasError::InvalidDimensions);
    }
    if y.len() != rows * inc_y as usize {
        return Err(BlasError::InvalidDimensions);
    }

    // Scale Y by beta
    if beta != Complex32::new(0.0, 0.0) {
        for elem in y.iter_mut() {
            *elem *= beta;
        }
    } else {
        y.fill(Complex32::new(0.0, 0.0));
    }

    // Perform matrix-vector multiplication
    match trans_a {
        CblasTranspose::NoTrans => {
            for i in 0..rows {
                let mut sum = Complex32::new(0.0, 0.0);
                for j in 0..cols {
                    sum += a[[i, j]] * x[j * inc_x as usize];
                }
                y[i * inc_y as usize] += alpha * sum;
            }
        }
        CblasTranspose::Trans => {
            for j in 0..cols {
                let x_val = x[j * inc_x as usize];
                for i in 0..rows {
                    y[i * inc_y as usize] += alpha * a[[i, j]] * x_val;
                }
            }
        }
        CblasTranspose::ConjTrans => {
            for j in 0..cols {
                let x_val = x[j * inc_x as usize];
                for i in 0..rows {
                    y[i * inc_y as usize] += alpha * a[[i, j]].conj() * x_val;
                }
            }
        }
    }

    Ok(())
}