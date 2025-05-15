use num_traits::Zero;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

fn cblas_xerbla(p: i32, rout: &str, form: &str) {
    eprintln!("Parameter {} was incorrect in {}: {}", p, rout, form);
}

pub fn cblas_ssyr2k(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    beta: f32,
    c: &mut [f32],
    ldc: i32,
) {
    let mut pos = 0;
    let dim = match (order, trans) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) => k,
        (CBLAS_ORDER::RowMajor, _) => n,
        (_, CBLAS_TRANSPOSE::NoTrans) => n,
        _ => k,
    };

    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        pos = 1;
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        pos = 2;
    }
    if trans != CBLAS_TRANSPOSE::NoTrans 
        && trans != CBLAS_TRANSPOSE::Trans 
        && trans != CBLAS_TRANSPOSE::ConjTrans {
        pos = 3;
    }
    if n < 0 {
        pos = 4;
    }
    if k < 0 {
        pos = 5;
    }
    if lda < max(1, dim) {
        pos = 8;
    }
    if ldb < max(1, dim) {
        pos = 11;
    }
    if ldc < max(1, n) {
        pos = 14;
    }

    if pos != 0 {
        cblas_xerbla(pos, "cblas_ssyr2k", "");
        return;
    }

    if alpha.is_zero() && beta == 1.0 {
        return;
    }

    let (effective_uplo, effective_trans) = match order {
        CBLAS_ORDER::RowMajor => {
            let t = if trans == CBLAS_TRANSPOSE::ConjTrans {
                CBLAS_TRANSPOSE::Trans
            } else {
                trans
            };
            (uplo, t)
        }
        CBLAS_ORDER::ColMajor => {
            let u = if uplo == CBLAS_UPLO::Upper {
                CBLAS_UPLO::Lower
            } else {
                CBLAS_UPLO::Upper
            };
            let t = if trans == CBLAS_TRANSPOSE::Trans || trans == CBLAS_TRANSPOSE::ConjTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            };
            (u, t)
        }
    };

    if beta.is_zero() {
        match effective_uplo {
            CBLAS_UPLO::Upper => {
                for i in 0..n {
                    for j in i..n {
                        c[(i * ldc + j) as usize] = 0.0;
                    }
                }
            }
            CBLAS_UPLO::Lower => {
                for i in 0..n {
                    for j in 0..=i {
                        c[(i * ldc + j) as usize] = 0.0;
                    }
                }
            }
        }
    } else if beta != 1.0 {
        match effective_uplo {
            CBLAS_UPLO::Upper => {
                for i in 0..n {
                    for j in i..n {
                        c[(i * ldc + j) as usize] *= beta;
                    }
                }
            }
            CBLAS_UPLO::Lower => {
                for i in 0..n {
                    for j in 0..=i {
                        c[(i * ldc + j) as usize] *= beta;
                    }
                }
            }
        }
    }

    if alpha.is_zero() {
        return;
    }

    match (effective_uplo, effective_trans) {
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = 0.0;
                    for k_ in 0..k {
                        let a_ik = a[(i * lda + k_) as usize];
                        let b_jk = b[(j * ldb + k_) as usize];
                        let b_ik = b[(i * ldb + k_) as usize];
                        let a_jk = a[(j * lda + k_) as usize];
                        temp += a_ik * b_jk + b_ik * a_jk;
                    }
                    c[(i * ldc + j) as usize] += alpha * temp;
                }
            }
        }
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::Trans) => {
            for k_ in 0..k {
                for i in 0..n {
                    let temp1 = alpha * a[(k_ * lda + i) as usize];
                    let temp2 = alpha * b[(k_ * ldb + i) as usize];
                    for j in i..n {
                        c[(i * ldc + j) as usize] += 
                            temp1 * b[(k_ * ldb + j) as usize] + 
                            temp2 * a[(k_ * lda + j) as usize];
                    }
                }
            }
        }
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = 0.0;
                    for k_ in 0..k {
                        let a_ik = a[(i * lda + k_) as usize];
                        let b_jk = b[(j * ldb + k_) as usize];
                        let b_ik = b[(i * ldb + k_) as usize];
                        let a_jk = a[(j * lda + k_) as usize];
                        temp += a_ik * b_jk + b_ik * a_jk;
                    }
                    c[(i * ldc + j) as usize] += alpha * temp;
                }
            }
        }
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::Trans) => {
            for k_ in 0..k {
                for i in 0..n {
                    let temp1 = alpha * a[(k_ * lda + i) as usize];
                    let temp2 = alpha * b[(k_ * ldb + i) as usize];
                    for j in 0..=i {
                        c[(i * ldc + j) as usize] += 
                            temp1 * b[(k_ * ldb + j) as usize] + 
                            temp2 * a[(k_ * lda + j) as usize];
                    }
                }
            }
        }
        _ => {
            cblas_xerbla(0, "cblas_ssyr2k", "unrecognized operation");
        }
    }
}