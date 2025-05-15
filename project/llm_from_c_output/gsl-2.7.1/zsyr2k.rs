use ndarray::{Array2, ArrayView2};
use num_complex::Complex64;
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

pub fn cblas_zsyr2k(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    n: usize,
    k: usize,
    alpha: Complex64,
    a: ArrayView2<Complex64>,
    lda: usize,
    b: ArrayView2<Complex64>,
    ldb: usize,
    beta: Complex64,
    c: &mut Array2<Complex64>,
    ldc: usize,
) -> Result<(), BlasError> {
    // Validate dimensions
    if n == 0 {
        return Ok(());
    }

    let (nrow_a, ncol_a) = match trans {
        CblasTranspose::NoTrans => (n, k),
        _ => (k, n),
    };

    if a.shape()[0] < nrow_a || a.shape()[1] < ncol_a {
        return Err(BlasError::InvalidDimension);
    }
    if b.shape()[0] < nrow_a || b.shape()[1] < ncol_a {
        return Err(BlasError::InvalidDimension);
    }
    if c.shape()[0] < n || c.shape()[1] < n {
        return Err(BlasError::InvalidDimension);
    }

    // Perform the operation
    match (order, uplo) {
        (CblasOrder::RowMajor, CblasUplo::Upper) | (CblasOrder::ColMajor, CblasUplo::Lower) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp1 = Complex64::new(0.0, 0.0);
                    let mut temp2 = Complex64::new(0.0, 0.0);
                    
                    for l in 0..k {
                        let a_il = if trans == CblasTranspose::NoTrans {
                            a[[i, l]]
                        } else {
                            a[[l, i]].conj()
                        };
                        
                        let b_jl = if trans == CblasTranspose::NoTrans {
                            b[[j, l]]
                        } else {
                            b[[l, j]].conj()
                        };
                        
                        let a_jl = if trans == CblasTranspose::NoTrans {
                            a[[j, l]]
                        } else {
                            a[[l, j]].conj()
                        };
                        
                        let b_il = if trans == CblasTranspose::NoTrans {
                            b[[i, l]]
                        } else {
                            b[[l, i]].conj()
                        };
                        
                        temp1 += a_il * b_jl;
                        temp2 += a_jl * b_il;
                    }
                    
                    c[[i, j]] = alpha * (temp1 + temp2) + beta * c[[i, j]];
                }
            }
        }
        (CblasOrder::RowMajor, CblasUplo::Lower) | (CblasOrder::ColMajor, CblasUplo::Upper) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp1 = Complex64::new(0.0, 0.0);
                    let mut temp2 = Complex64::new(0.0, 0.0);
                    
                    for l in 0..k {
                        let a_il = if trans == CblasTranspose::NoTrans {
                            a[[i, l]]
                        } else {
                            a[[l, i]].conj()
                        };
                        
                        let b_jl = if trans == CblasTranspose::NoTrans {
                            b[[j, l]]
                        } else {
                            b[[l, j]].conj()
                        };
                        
                        let a_jl = if trans == CblasTranspose::NoTrans {
                            a[[j, l]]
                        } else {
                            a[[l, j]].conj()
                        };
                        
                        let b_il = if trans == CblasTranspose::NoTrans {
                            b[[i, l]]
                        } else {
                            b[[l, i]].conj()
                        };
                        
                        temp1 += a_il * b_jl;
                        temp2 += a_jl * b_il;
                    }
                    
                    c[[i, j]] = alpha * (temp1 + temp2) + beta * c[[i, j]];
                }
            }
        }
    }

    Ok(())
}