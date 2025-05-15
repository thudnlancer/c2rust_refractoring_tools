use num_complex::Complex64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

pub fn cblas_zsyr2k(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    n: i32,
    k: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    b: &[Complex64],
    ldb: i32,
    beta: Complex64,
    c: &mut [Complex64],
    ldc: i32,
) {
    // Parameter validation
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        panic!("Invalid order parameter");
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        panic!("Invalid uplo parameter");
    }
    if trans != CBLAS_TRANSPOSE::NoTrans
        && trans != CBLAS_TRANSPOSE::Trans
        && trans != CBLAS_TRANSPOSE::ConjTrans
    {
        panic!("Invalid trans parameter");
    }
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if k < 0 {
        panic!("Invalid k parameter");
    }

    let dim = match (order, trans) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) => k,
        (CBLAS_ORDER::RowMajor, _) => n,
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => n,
        (CBLAS_ORDER::ColMajor, _) => k,
    };

    if lda < 1.max(dim) {
        panic!("Invalid lda parameter");
    }
    if ldb < 1.max(dim) {
        panic!("Invalid ldb parameter");
    }
    if ldc < 1.max(n) {
        panic!("Invalid ldc parameter");
    }

    // Early return if alpha is zero and beta is one
    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }

    // Adjust parameters for row-major order
    let (uplo, trans) = match order {
        CBLAS_ORDER::RowMajor => (
            match uplo {
                CBLAS_UPLO::Upper => CBLAS_UPLO::Lower,
                CBLAS_UPLO::Lower => CBLAS_UPLO::Upper,
            },
            match trans {
                CBLAS_TRANSPOSE::NoTrans => CBLAS_TRANSPOSE::Trans,
                _ => CBLAS_TRANSPOSE::NoTrans,
            },
        ),
        CBLAS_ORDER::ColMajor => (uplo, trans),
    };

    // Handle beta scaling
    if beta == Complex64::new(0.0, 0.0) {
        if uplo == CBLAS_UPLO::Upper {
            for i in 0..n {
                for j in i..n {
                    let idx = (i * ldc + j) as usize;
                    c[idx] = Complex64::new(0.0, 0.0);
                }
            }
        } else {
            for i in 0..n {
                for j in 0..=i {
                    let idx = (i * ldc + j) as usize;
                    c[idx] = Complex64::new(0.0, 0.0);
                }
            }
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        if uplo == CBLAS_UPLO::Upper {
            for i in 0..n {
                for j in i..n {
                    let idx = (i * ldc + j) as usize;
                    c[idx] = beta * c[idx];
                }
            }
        } else {
            for i in 0..n {
                for j in 0..=i {
                    let idx = (i * ldc + j) as usize;
                    c[idx] = beta * c[idx];
                }
            }
        }
    }

    // Early return if alpha is zero
    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    // Perform the main computation
    match (uplo, trans) {
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k in 0..k {
                        let a_ik = a[(i * lda + k) as usize];
                        let b_ik = b[(i * ldb + k) as usize];
                        let a_jk = a[(j * lda + k) as usize];
                        let b_jk = b[(j * ldb + k) as usize];
                        temp += a_ik * b_jk.conj() + b_ik * a_jk.conj();
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::Trans) => {
            for k in 0..k {
                for i in 0..n {
                    let a_ki = a[(k * lda + i) as usize];
                    let b_ki = b[(k * ldb + i) as usize];
                    let temp1 = alpha * a_ki;
                    let temp2 = alpha * b_ki;
                    for j in i..n {
                        let a_kj = a[(k * lda + j) as usize];
                        let b_kj = b[(k * ldb + j) as usize];
                        let idx = (i * ldc + j) as usize;
                        c[idx] += temp1 * b_kj.conj() + temp2 * a_kj.conj();
                    }
                }
            }
        }
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k in 0..k {
                        let a_ik = a[(i * lda + k) as usize];
                        let b_ik = b[(i * ldb + k) as usize];
                        let a_jk = a[(j * lda + k) as usize];
                        let b_jk = b[(j * ldb + k) as usize];
                        temp += a_ik * b_jk.conj() + b_ik * a_jk.conj();
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::Trans) => {
            for k in 0..k {
                for i in 0..n {
                    let a_ki = a[(k * lda + i) as usize];
                    let b_ki = b[(k * ldb + i) as usize];
                    let temp1 = alpha * a_ki;
                    let temp2 = alpha * b_ki;
                    for j in 0..=i {
                        let a_kj = a[(k * lda + j) as usize];
                        let b_kj = b[(k * ldb + j) as usize];
                        let idx = (i * ldc + j) as usize;
                        c[idx] += temp1 * b_kj.conj() + temp2 * a_kj.conj();
                    }
                }
            }
        }
        _ => panic!("Unrecognized operation"),
    }
}