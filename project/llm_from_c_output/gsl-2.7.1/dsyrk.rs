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

pub fn cblas_dsyrk(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    n: usize,
    k: usize,
    alpha: f64,
    a: ArrayView2<f64>,
    lda: usize,
    beta: f64,
    c: &mut Array2<f64>,
    ldc: usize,
) -> Result<(), String> {
    // Validate dimensions
    if a.shape()[0] < lda || a.shape()[1] < if trans == CblasTranspose::NoTrans { k } else { n } {
        return Err("Invalid dimensions for matrix A".to_string());
    }
    if c.shape()[0] < ldc || c.shape()[1] < n {
        return Err("Invalid dimensions for matrix C".to_string());
    }

    // Perform the symmetric rank-k update
    match (order, uplo, trans) {
        (CblasOrder::RowMajor, uplo, trans) => {
            let a = if trans == CblasTranspose::NoTrans {
                a.slice(s![..n, ..k])
            } else {
                a.slice(s![..k, ..n])
            };
            
            let mut c_slice = c.slice_mut(s![..n, ..n]);
            
            if beta.is_zero() {
                c_slice.fill(0.0);
            } else if beta != 1.0 {
                c_slice.mapv_inplace(|x| x * beta);
            }
            
            if alpha != 0.0 {
                match uplo {
                    CblasUplo::Upper => {
                        for i in 0..n {
                            for j in i..n {
                                let mut sum = 0.0;
                                for l in 0..k {
                                    sum += a[[i, l]] * a[[j, l]];
                                }
                                c_slice[[i, j]] += alpha * sum;
                            }
                        }
                    }
                    CblasUplo::Lower => {
                        for j in 0..n {
                            for i in j..n {
                                let mut sum = 0.0;
                                for l in 0..k {
                                    sum += a[[i, l]] * a[[j, l]];
                                }
                                c_slice[[i, j]] += alpha * sum;
                            }
                        }
                    }
                }
            }
        }
        (CblasOrder::ColMajor, uplo, trans) => {
            let a = if trans == CblasTranspose::NoTrans {
                a.slice(s![..k, ..n])
            } else {
                a.slice(s![..n, ..k])
            };
            
            let mut c_slice = c.slice_mut(s![..n, ..n]);
            
            if beta.is_zero() {
                c_slice.fill(0.0);
            } else if beta != 1.0 {
                c_slice.mapv_inplace(|x| x * beta);
            }
            
            if alpha != 0.0 {
                match uplo {
                    CblasUplo::Upper => {
                        for i in 0..n {
                            for j in i..n {
                                let mut sum = 0.0;
                                for l in 0..k {
                                    sum += a[[l, i]] * a[[l, j]];
                                }
                                c_slice[[i, j]] += alpha * sum;
                            }
                        }
                    }
                    CblasUplo::Lower => {
                        for j in 0..n {
                            for i in j..n {
                                let mut sum = 0.0;
                                for l in 0..k {
                                    sum += a[[l, i]] * a[[l, j]];
                                }
                                c_slice[[i, j]] += alpha * sum;
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}