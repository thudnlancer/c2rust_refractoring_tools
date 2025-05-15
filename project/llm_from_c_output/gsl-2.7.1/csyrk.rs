use ndarray::{Array2, ArrayView2};
use num_complex::Complex32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid transpose")]
    InvalidTranspose,
    #[error("invalid dimension")]
    InvalidDimension,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

pub fn cblas_csyrk(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    n: usize,
    k: usize,
    alpha: Complex32,
    a: ArrayView2<Complex32>,
    lda: usize,
    beta: Complex32,
    c: &mut Array2<Complex32>,
    ldc: usize,
) -> Result<(), BlasError> {
    // Validate dimensions
    if n == 0 {
        return Ok(());
    }

    if k == 0 {
        return Ok(());
    }

    if a.is_empty() {
        return Err(BlasError::InvalidDimension);
    }

    if c.is_empty() {
        return Err(BlasError::InvalidDimension);
    }

    // Validate leading dimensions
    let (expected_a_rows, expected_a_cols) = match trans {
        CblasTranspose::NoTrans => (n, k),
        _ => (k, n),
    };

    if lda < expected_a_rows {
        return Err(BlasError::InvalidDimension);
    }

    if ldc < n {
        return Err(BlasError::InvalidDimension);
    }

    // Perform SYRK operation
    match trans {
        CblasTranspose::NoTrans => {
            let a_slice = a.slice(s![..n, ..k]);
            let c_slice = c.slice_mut(s![..n, ..n]);
            
            // C = alpha * A * A^T + beta * C
            for i in 0..n {
                for j in 0..=i {
                    let mut sum = Complex32::new(0.0, 0.0);
                    for l in 0..k {
                        sum += a_slice[[i, l]] * a_slice[[j, l]].conj();
                    }
                    c_slice[[i, j]] = alpha * sum + beta * c_slice[[i, j]];
                    if i != j {
                        c_slice[[j, i]] = c_slice[[i, j]].conj();
                    }
                }
            }
        }
        CblasTranspose::Trans => {
            let a_slice = a.slice(s![..k, ..n]);
            let c_slice = c.slice_mut(s![..n, ..n]);
            
            // C = alpha * A^T * A + beta * C
            for i in 0..n {
                for j in 0..=i {
                    let mut sum = Complex32::new(0.0, 0.0);
                    for l in 0..k {
                        sum += a_slice[[l, i]].conj() * a_slice[[l, j]];
                    }
                    c_slice[[i, j]] = alpha * sum + beta * c_slice[[i, j]];
                    if i != j {
                        c_slice[[j, i]] = c_slice[[i, j]].conj();
                    }
                }
            }
        }
        CblasTranspose::ConjTrans => {
            let a_slice = a.slice(s![..k, ..n]);
            let c_slice = c.slice_mut(s![..n, ..n]);
            
            // C = alpha * A^H * A + beta * C
            for i in 0..n {
                for j in 0..=i {
                    let mut sum = Complex32::new(0.0, 0.0);
                    for l in 0..k {
                        sum += a_slice[[l, i]].conj() * a_slice[[l, j]];
                    }
                    c_slice[[i, j]] = alpha * sum + beta * c_slice[[i, j]];
                    if i != j {
                        c_slice[[j, i]] = c_slice[[i, j]].conj();
                    }
                }
            }
        }
    }

    Ok(())
}