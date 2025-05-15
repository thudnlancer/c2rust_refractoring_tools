use ndarray::{Array2, ArrayView2};
use num_traits::Zero;

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
}

pub fn cblas_ssyrk(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    n: usize,
    k: usize,
    alpha: f32,
    a: ArrayView2<f32>,
    lda: usize,
    beta: f32,
    c: &mut Array2<f32>,
    ldc: usize,
) -> Result<(), String> {
    // Validate dimensions
    if a.shape()[0] < lda || a.shape()[1] < if trans == CblasTranspose::NoTrans { k } else { n } {
        return Err("Invalid dimensions for matrix A".to_string());
    }
    if c.shape()[0] < ldc || c.shape()[1] < n {
        return Err("Invalid dimensions for matrix C".to_string());
    }

    // Perform SYRK operation
    match (order, uplo, trans) {
        (CblasOrder::RowMajor, uplo, trans) => {
            // Row major implementation
            let a = if trans == CblasTranspose::NoTrans {
                a.slice(s![..n, ..k])
            } else {
                a.slice(s![..k, ..n]).t()
            };

            for i in 0..n {
                for j in 0..=i {
                    let mut sum = Zero::zero();
                    for l in 0..k {
                        sum += a[[i, l]] * a[[j, l]];
                    }
                    c[[i, j]] = alpha * sum + beta * c[[i, j]];
                }
            }

            // Fill the symmetric part
            if uplo == CblasUplo::Upper {
                for i in 0..n {
                    for j in i+1..n {
                        c[[i, j]] = c[[j, i]];
                    }
                }
            }
        },
        (CblasOrder::ColMajor, uplo, trans) => {
            // Column major implementation
            let a = if trans == CblasTranspose::NoTrans {
                a.slice(s![..n, ..k])
            } else {
                a.slice(s![..k, ..n]).t()
            };

            for j in 0..n {
                for i in j..n {
                    let mut sum = Zero::zero();
                    for l in 0..k {
                        sum += a[[i, l]] * a[[j, l]];
                    }
                    c[[i, j]] = alpha * sum + beta * c[[i, j]];
                }
            }

            // Fill the symmetric part
            if uplo == CblasUplo::Lower {
                for j in 0..n {
                    for i in 0..j {
                        c[[i, j]] = c[[j, i]];
                    }
                }
            }
        },
    }

    Ok(())
}