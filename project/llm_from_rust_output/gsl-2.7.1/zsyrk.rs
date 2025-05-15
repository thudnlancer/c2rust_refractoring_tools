use num_complex::Complex64;

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

pub fn cblas_zsyrk(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    n: i32,
    k: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
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

    let dim_a = match (order, trans) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) => k,
        (CBLAS_ORDER::RowMajor, _) => n,
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => n,
        (CBLAS_ORDER::ColMajor, _) => k,
    };

    if lda < 1.max(dim_a) {
        panic!("Invalid lda parameter");
    }
    if ldc < 1.max(n) {
        panic!("Invalid ldc parameter");
    }

    // Early return if alpha is zero and beta is one
    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }

    // Handle beta scaling
    if beta == Complex64::new(0.0, 0.0) {
        match uplo {
            CBLAS_UPLO::Upper => {
                for i in 0..n {
                    for j in i..n {
                        let idx = (i * ldc + j) as usize;
                        c[idx] = Complex64::new(0.0, 0.0);
                    }
                }
            }
            CBLAS_UPLO::Lower => {
                for i in 0..n {
                    for j in 0..=i {
                        let idx = (i * ldc + j) as usize;
                        c[idx] = Complex64::new(0.0, 0.0);
                    }
                }
            }
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        match uplo {
            CBLAS_UPLO::Upper => {
                for i in 0..n {
                    for j in i..n {
                        let idx = (i * ldc + j) as usize;
                        c[idx] = beta * c[idx];
                    }
                }
            }
            CBLAS_UPLO::Lower => {
                for i in 0..n {
                    for j in 0..=i {
                        let idx = (i * ldc + j) as usize;
                        c[idx] = beta * c[idx];
                    }
                }
            }
        }
    }

    // Early return if alpha is zero
    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    // Main computation
    match (order, uplo, trans) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ik = a[(i * lda + k_val) as usize];
                        let a_jk = a[(j * lda + k_val) as usize];
                        temp += a_ik * a_jk.conj();
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ki = a[(k_val * lda + i) as usize];
                        let a_kj = a[(k_val * lda + j) as usize];
                        temp += a_ki.conj() * a_kj;
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ik = a[(i * lda + k_val) as usize];
                        let a_jk = a[(j * lda + k_val) as usize];
                        temp += a_ik * a_jk.conj();
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ki = a[(k_val * lda + i) as usize];
                        let a_kj = a[(k_val * lda + j) as usize];
                        temp += a_ki.conj() * a_kj;
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ki = a[(k_val * lda + i) as usize];
                        let a_kj = a[(k_val * lda + j) as usize];
                        temp += a_ki * a_kj.conj();
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ik = a[(i * lda + k_val) as usize];
                        let a_jk = a[(j * lda + k_val) as usize];
                        temp += a_ik.conj() * a_jk;
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ki = a[(k_val * lda + i) as usize];
                        let a_kj = a[(k_val * lda + j) as usize];
                        temp += a_ki * a_kj.conj();
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k_val in 0..k {
                        let a_ik = a[(i * lda + k_val) as usize];
                        let a_jk = a[(j * lda + k_val) as usize];
                        temp += a_ik.conj() * a_jk;
                    }
                    let idx = (i * ldc + j) as usize;
                    c[idx] += alpha * temp;
                }
            }
        }
        _ => panic!("Unrecognized operation"),
    }
}