use num_complex::Complex64;
use std::fmt::Debug;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_SIDE {
    Left = 141,
    Right = 142,
}

pub fn cblas_ztrmm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    m: i32,
    n: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    b: &mut [Complex64],
    ldb: i32,
) {
    // Parameter validation
    if ![CBLAS_ORDER::RowMajor, CBLAS_ORDER::ColMajor].contains(&order) {
        panic!("Invalid order parameter");
    }
    if ![CBLAS_SIDE::Left, CBLAS_SIDE::Right].contains(&side) {
        panic!("Invalid side parameter");
    }
    if ![CBLAS_UPLO::Upper, CBLAS_UPLO::Lower].contains(&uplo) {
        panic!("Invalid uplo parameter");
    }
    if ![CBLAS_TRANSPOSE::NoTrans, CBLAS_TRANSPOSE::Trans, CBLAS_TRANSPOSE::ConjTrans].contains(&trans_a) {
        panic!("Invalid trans_a parameter");
    }
    if ![CBLAS_DIAG::NonUnit, CBLAS_DIAG::Unit].contains(&diag) {
        panic!("Invalid diag parameter");
    }
    if m < 0 || n < 0 {
        panic!("Invalid dimensions");
    }

    let dim = if side == CBLAS_SIDE::Left { m } else { n };
    if lda < dim.max(1) {
        panic!("Invalid lda");
    }

    let min_ldb = if order == CBLAS_ORDER::RowMajor { n.max(1) } else { m.max(1) };
    if ldb < min_ldb {
        panic!("Invalid ldb");
    }

    let nonunit = diag == CBLAS_DIAG::NonUnit;
    let conj = if trans_a == CBLAS_TRANSPOSE::ConjTrans { -1.0 } else { 1.0 };

    let (n1, n2, side, uplo, trans) = if order == CBLAS_ORDER::RowMajor {
        (
            m,
            n,
            side,
            uplo,
            if trans_a == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
        )
    } else {
        (
            n,
            m,
            if side == CBLAS_SIDE::Left {
                CBLAS_SIDE::Right
            } else {
                CBLAS_SIDE::Left
            },
            if uplo == CBLAS_UPLO::Upper {
                CBLAS_UPLO::Lower
            } else {
                CBLAS_UPLO::Upper
            },
            if trans_a == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
        )
    };

    match (side, uplo, trans) {
        (CBLAS_SIDE::Left, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let mut temp = if nonunit {
                        let a_idx = i * lda + i;
                        let a_ii = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        a_ii * b_ij
                    } else {
                        let b_idx = i * ldb + j;
                        b[b_idx as usize]
                    };

                    for k in (i + 1)..n1 {
                        let a_idx = i * lda + k;
                        let a_ik = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = k * ldb + j;
                        let b_kj = b[b_idx as usize];
                        temp += a_ik * b_kj;
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Left, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::Trans) => {
            for i in (0..n1).rev() {
                for j in 0..n2 {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k in 0..i {
                        let a_idx = k * lda + i;
                        let a_ki = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = k * ldb + j;
                        let b_kj = b[b_idx as usize];
                        temp += a_ki * b_kj;
                    }

                    if nonunit {
                        let a_idx = i * lda + i;
                        let a_ii = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        temp += a_ii * b_ij;
                    } else {
                        let b_idx = i * ldb + j;
                        temp += b[b_idx as usize];
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Left, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in (0..n1).rev() {
                for j in 0..n2 {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k in 0..i {
                        let a_idx = i * lda + k;
                        let a_ik = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = k * ldb + j;
                        let b_kj = b[b_idx as usize];
                        temp += a_ik * b_kj;
                    }

                    if nonunit {
                        let a_idx = i * lda + i;
                        let a_ii = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        temp += a_ii * b_ij;
                    } else {
                        let b_idx = i * ldb + j;
                        temp += b[b_idx as usize];
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Left, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let mut temp = if nonunit {
                        let a_idx = i * lda + i;
                        let a_ii = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        a_ii * b_ij
                    } else {
                        let b_idx = i * ldb + j;
                        b[b_idx as usize]
                    };

                    for k in (i + 1)..n1 {
                        let a_idx = k * lda + i;
                        let a_ki = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = k * ldb + j;
                        let b_kj = b[b_idx as usize];
                        temp += a_ki * b_kj;
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n1 {
                for j in (0..n2).rev() {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k in 0..j {
                        let a_idx = k * lda + j;
                        let a_kj = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + k;
                        let b_ik = b[b_idx as usize];
                        temp += a_kj * b_ik;
                    }

                    if nonunit {
                        let a_idx = j * lda + j;
                        let a_jj = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        temp += a_jj * b_ij;
                    } else {
                        let b_idx = i * ldb + j;
                        temp += b[b_idx as usize];
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let mut temp = if nonunit {
                        let a_idx = j * lda + j;
                        let a_jj = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        a_jj * b_ij
                    } else {
                        let b_idx = i * ldb + j;
                        b[b_idx as usize]
                    };

                    for k in (j + 1)..n2 {
                        let a_idx = j * lda + k;
                        let a_jk = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + k;
                        let b_ik = b[b_idx as usize];
                        temp += a_jk * b_ik;
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n1 {
                for j in 0..n2 {
                    let mut temp = if nonunit {
                        let a_idx = j * lda + j;
                        let a_jj = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        a_jj * b_ij
                    } else {
                        let b_idx = i * ldb + j;
                        b[b_idx as usize]
                    };

                    for k in (j + 1)..n2 {
                        let a_idx = k * lda + j;
                        let a_kj = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + k;
                        let b_ik = b[b_idx as usize];
                        temp += a_kj * b_ik;
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        (CBLAS_SIDE::Right, CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 {
                for j in (0..n2).rev() {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for k in 0..j {
                        let a_idx = j * lda + k;
                        let a_jk = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + k;
                        let b_ik = b[b_idx as usize];
                        temp += a_jk * b_ik;
                    }

                    if nonunit {
                        let a_idx = j * lda + j;
                        let a_jj = Complex64::new(a[a_idx as usize].re, conj * a[a_idx as usize].im);
                        let b_idx = i * ldb + j;
                        let b_ij = b[b_idx as usize];
                        temp += a_jj * b_ij;
                    } else {
                        let b_idx = i * ldb + j;
                        temp += b[b_idx as usize];
                    }

                    let b_idx = i * ldb + j;
                    b[b_idx as usize] = alpha * temp;
                }
            }
        }
        _ => panic!("Unrecognized operation"),
    }
}