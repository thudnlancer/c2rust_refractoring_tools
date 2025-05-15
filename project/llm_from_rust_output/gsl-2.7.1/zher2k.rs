use num_complex::Complex64;
use std::cmp::max;

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

pub fn cblas_zher2k(
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
    beta: f64,
    c: &mut [Complex64],
    ldc: i32,
) {
    // Input validation
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        panic!("Invalid order parameter");
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        panic!("Invalid uplo parameter");
    }
    if trans != CBLAS_TRANSPOSE::NoTrans 
        && trans != CBLAS_TRANSPOSE::Trans 
        && trans != CBLAS_TRANSPOSE::ConjTrans {
        panic!("Invalid trans parameter");
    }
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if k < 0 {
        panic!("Invalid k parameter");
    }

    let (effective_trans, effective_uplo, effective_alpha) = if order == CBLAS_ORDER::RowMajor {
        let new_uplo = match uplo {
            CBLAS_UPLO::Upper => CBLAS_UPLO::Lower,
            CBLAS_UPLO::Lower => CBLAS_UPLO::Upper,
        };
        let new_trans = match trans {
            CBLAS_TRANSPOSE::NoTrans => CBLAS_TRANSPOSE::ConjTrans,
            _ => CBLAS_TRANSPOSE::NoTrans,
        };
        (new_trans, new_uplo, Complex64::new(alpha.re, -alpha.im))
    } else {
        (trans, uplo, alpha)
    };

    let dim = if order == CBLAS_ORDER::RowMajor {
        if trans == CBLAS_TRANSPOSE::NoTrans { k } else { n }
    } else {
        if trans == CBLAS_TRANSPOSE::NoTrans { n } else { k }
    };

    if lda < max(1, dim) {
        panic!("Invalid lda parameter");
    }
    if ldb < max(1, dim) {
        panic!("Invalid ldb parameter");
    }
    if ldc < max(1, n) {
        panic!("Invalid ldc parameter");
    }

    // Early return if alpha is zero and beta is one
    if beta == 1.0 && (alpha == Complex64::new(0.0, 0.0) || k == 0) {
        return;
    }

    // Scale C by beta
    if beta == 0.0 {
        if effective_uplo == CBLAS_UPLO::Upper {
            for i in 0..n as usize {
                for j in i..n as usize {
                    c[i * ldc as usize + j] = Complex64::new(0.0, 0.0);
                }
            }
        } else {
            for i in 0..n as usize {
                for j in 0..=i {
                    c[i * ldc as usize + j] = Complex64::new(0.0, 0.0);
                }
            }
        }
    } else if beta != 1.0 {
        if effective_uplo == CBLAS_UPLO::Upper {
            for i in 0..n as usize {
                c[i * ldc as usize + i] = Complex64::new(c[i * ldc as usize + i].re * beta, 0.0);
                for j in i+1..n as usize {
                    c[i * ldc as usize + j] = c[i * ldc as usize + j].scale(beta);
                }
            }
        } else {
            for i in 0..n as usize {
                for j in 0..i {
                    c[i * ldc as usize + j] = c[i * ldc as usize + j].scale(beta);
                }
                c[i * ldc as usize + i] = Complex64::new(c[i * ldc as usize + i].re * beta, 0.0);
            }
        }
    } else {
        // Just zero the imaginary parts of diagonal
        for i in 0..n as usize {
            c[i * ldc as usize + i].im = 0.0;
        }
    }

    // Early return if alpha is zero
    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    // Main computation
    match (effective_uplo, effective_trans) {
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n as usize {
                let mut temp_real = 0.0;
                for kk in 0..k as usize {
                    let aik = a[i * lda as usize + kk];
                    let bik = b[i * ldb as usize + kk];
                    let temp1 = effective_alpha * aik;
                    temp_real += (temp1 * bik.conj()).re;
                }
                c[i * ldc as usize + i].re += 2.0 * temp_real;

                for j in i+1..n as usize {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for kk in 0..k as usize {
                        let aik = a[i * lda as usize + kk];
                        let bik = b[i * ldb as usize + kk];
                        let ajk = a[j * lda as usize + kk];
                        let bjk = b[j * ldb as usize + kk];
                        let temp1 = effective_alpha * aik;
                        let temp2 = effective_alpha * ajk;
                        temp += temp1 * bjk.conj() + bik.conj() * temp2;
                    }
                    c[i * ldc as usize + j] += temp;
                }
            }
        },
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::ConjTrans) => {
            for kk in 0..k as usize {
                for i in 0..n as usize {
                    let aki = a[kk * lda as usize + i].conj();
                    let bki = b[kk * ldb as usize + i];
                    let temp1 = effective_alpha * aki;
                    let temp2 = effective_alpha * bki.conj();
                    
                    c[i * ldc as usize + i].re += 2.0 * (temp1 * bki).re;
                    
                    for j in i+1..n as usize {
                        let akj = a[kk * lda as usize + j].conj();
                        let bkj = b[kk * ldb as usize + j];
                        c[i * ldc as usize + j] += temp1 * bkj + temp2 * akj;
                    }
                }
            }
        },
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n as usize {
                for j in 0..i {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for kk in 0..k as usize {
                        let aik = a[i * lda as usize + kk];
                        let bik = b[i * ldb as usize + kk];
                        let ajk = a[j * lda as usize + kk];
                        let bjk = b[j * ldb as usize + kk];
                        let temp1 = effective_alpha * aik;
                        let temp2 = effective_alpha * ajk;
                        temp += temp1 * bjk.conj() + bik.conj() * temp2;
                    }
                    c[i * ldc as usize + j] += temp;
                }
                
                let mut temp_real = 0.0;
                for kk in 0..k as usize {
                    let aik = a[i * lda as usize + kk];
                    let bik = b[i * ldb as usize + kk];
                    let temp1 = effective_alpha * aik;
                    temp_real += (temp1 * bik.conj()).re;
                }
                c[i * ldc as usize + i].re += 2.0 * temp_real;
            }
        },
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::ConjTrans) => {
            for kk in 0..k as usize {
                for i in 0..n as usize {
                    let aki = a[kk * lda as usize + i].conj();
                    let bki = b[kk * ldb as usize + i];
                    let temp1 = effective_alpha * aki;
                    let temp2 = effective_alpha * bki.conj();
                    
                    for j in 0..i {
                        let akj = a[kk * lda as usize + j].conj();
                        let bkj = b[kk * ldb as usize + j];
                        c[i * ldc as usize + j] += temp1 * bkj + temp2 * akj;
                    }
                    
                    c[i * ldc as usize + i].re += 2.0 * (temp1 * bki).re;
                }
            }
        },
        _ => panic!("Unrecognized operation"),
    }
}