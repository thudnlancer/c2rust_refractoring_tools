use ndarray::{Array2, ArrayView2, ShapeBuilder};
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

pub fn cblas_dsyr2k(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    n: usize,
    k: usize,
    alpha: f64,
    a: ArrayView2<f64>,
    lda: usize,
    b: ArrayView2<f64>,
    ldb: usize,
    beta: f64,
    c: &mut Array2<f64>,
    ldc: usize,
) -> Result<(), String> {
    // Validate dimensions
    if n == 0 {
        return Ok(());
    }

    let (nrow_a, ncol_a) = match trans {
        CblasTranspose::NoTrans => (n, k),
        CblasTranspose::Trans => (k, n),
    };

    if a.shape()[0] < nrow_a || a.shape()[1] < ncol_a {
        return Err("Invalid dimensions for matrix A".to_string());
    }
    if b.shape()[0] < nrow_a || b.shape()[1] < ncol_a {
        return Err("Invalid dimensions for matrix B".to_string());
    }
    if c.shape()[0] < n || c.shape()[1] < n {
        return Err("Invalid dimensions for matrix C".to_string());
    }

    // Perform the symmetric rank 2k update: C = alpha*(A*B' + B*A') + beta*C
    match order {
        CblasOrder::RowMajor => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp1 = Zero::zero();
                    let mut temp2 = Zero::zero();

                    for l in 0..k {
                        let a_il = match trans {
                            CblasTranspose::NoTrans => a[[i, l]],
                            CblasTranspose::Trans => a[[l, i]],
                        };
                        let b_jl = match trans {
                            CblasTranspose::NoTrans => b[[j, l]],
                            CblasTranspose::Trans => b[[l, j]],
                        };
                        temp1 += a_il * b_jl;

                        let b_il = match trans {
                            CblasTranspose::NoTrans => b[[i, l]],
                            CblasTranspose::Trans => b[[l, i]],
                        };
                        let a_jl = match trans {
                            CblasTranspose::NoTrans => a[[j, l]],
                            CblasTranspose::Trans => a[[l, j]],
                        };
                        temp2 += b_il * a_jl;
                    }

                    c[[i, j]] = alpha * (temp1 + temp2) + beta * c[[i, j]];
                    if i != j {
                        c[[j, i]] = c[[i, j]];
                    }
                }
            }
        }
        CblasOrder::ColMajor => {
            for j in 0..n {
                for i in 0..=j {
                    let mut temp1 = Zero::zero();
                    let mut temp2 = Zero::zero();

                    for l in 0..k {
                        let a_il = match trans {
                            CblasTranspose::NoTrans => a[[i, l]],
                            CblasTranspose::Trans => a[[l, i]],
                        };
                        let b_jl = match trans {
                            CblasTranspose::NoTrans => b[[j, l]],
                            CblasTranspose::Trans => b[[l, j]],
                        };
                        temp1 += a_il * b_jl;

                        let b_il = match trans {
                            CblasTranspose::NoTrans => b[[i, l]],
                            CblasTranspose::Trans => b[[l, i]],
                        };
                        let a_jl = match trans {
                            CblasTranspose::NoTrans => a[[j, l]],
                            CblasTranspose::Trans => a[[l, j]],
                        };
                        temp2 += b_il * a_jl;
                    }

                    c[[i, j]] = alpha * (temp1 + temp2) + beta * c[[i, j]];
                    if i != j {
                        c[[j, i]] = c[[i, j]];
                    }
                }
            }
        }
    }

    Ok(())
}