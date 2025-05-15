use num_complex::Complex32;

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

pub fn cblas_ctrmm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    transa: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    m: i32,
    n: i32,
    alpha: Complex32,
    a: &[Complex32],
    lda: i32,
    b: &mut [Complex32],
    ldb: i32,
) {
    let nonunit = diag == CBLAS_DIAG::NonUnit;
    let conj = if transa == CBLAS_TRANSPOSE::ConjTrans { -1.0 } else { 1.0 };

    let __dim = if side == CBLAS_SIDE::Left { m } else { n };

    // Parameter validation
    if !(order == CBLAS_ORDER::RowMajor || order == CBLAS_ORDER::ColMajor) {
        panic!("Invalid order parameter");
    }
    if !(side == CBLAS_SIDE::Left || side == CBLAS_SIDE::Right) {
        panic!("Invalid side parameter");
    }
    if !(uplo == CBLAS_UPLO::Upper || uplo == CBLAS_UPLO::Lower) {
        panic!("Invalid uplo parameter");
    }
    if !(transa == CBLAS_TRANSPOSE::NoTrans || 
         transa == CBLAS_TRANSPOSE::Trans || 
         transa == CBLAS_TRANSPOSE::ConjTrans) {
        panic!("Invalid transa parameter");
    }
    if !(diag == CBLAS_DIAG::NonUnit || diag == CBLAS_DIAG::Unit) {
        panic!("Invalid diag parameter");
    }
    if m < 0 {
        panic!("Invalid m parameter");
    }
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if lda < 1.max(__dim) {
        panic!("Invalid lda parameter");
    }
    if (order == CBLAS_ORDER::RowMajor && ldb < 1.max(n)) ||
       (order == CBLAS_ORDER::ColMajor && ldb < 1.max(m)) {
        panic!("Invalid ldb parameter");
    }

    let (n1, n2, side, uplo, trans) = if order == CBLAS_ORDER::RowMajor {
        (
            m,
            n,
            side as i32,
            uplo as i32,
            if transa == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans as i32
            } else {
                CBLAS_TRANSPOSE::Trans as i32
            },
        )
    } else {
        (
            n,
            m,
            if side == CBLAS_SIDE::Left {
                CBLAS_SIDE::Right as i32
            } else {
                CBLAS_SIDE::Left as i32
            },
            if uplo == CBLAS_UPLO::Upper {
                CBLAS_UPLO::Lower as i32
            } else {
                CBLAS_UPLO::Upper as i32
            },
            if transa == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans as i32
            } else {
                CBLAS_TRANSPOSE::Trans as i32
            },
        )
    };

    if side == CBLAS_SIDE::Left as i32 && uplo == CBLAS_UPLO::Upper as i32 && trans == CBLAS_TRANSPOSE::NoTrans as i32 {
        for i in 0..n1 {
            for j in 0..n2 {
                let mut temp = if nonunit {
                    let a_idx = (i * lda + i) as usize;
                    let a_val = Complex32::new(a[a_idx].re, conj * a[a_idx].im);
                    let b_idx = (i * ldb + j) as usize;
                    a_val * b[b_idx]
                } else {
                    let b_idx = (i * ldb + j) as usize;
                    b[b_idx]
                };

                for k in (i + 1)..n1 {
                    let a_idx = (i * lda + k) as usize;
                    let a_val = Complex32::new(a[a_idx].re, conj * a[a_idx].im);
                    let b_idx = (k * ldb + j) as usize;
                    temp += a_val * b[b_idx];
                }

                let b_idx = (i * ldb + j) as usize;
                b[b_idx] = alpha * temp;
            }
        }
    } else if side == CBLAS_SIDE::Left as i32 && uplo == CBLAS_UPLO::Upper as i32 && trans == CBLAS_TRANSPOSE::Trans as i32 {
        for i in (0..n1).rev() {
            for j in 0..n2 {
                let mut temp = Complex32::new(0.0, 0.0);
                
                for k in 0..i {
                    let a_idx = (k * lda + i) as usize;
                    let a_val = Complex32::new(a[a_idx].re, conj * a[a_idx].im);
                    let b_idx = (k * ldb + j) as usize;
                    temp += a_val * b[b_idx];
                }

                if nonunit {
                    let a_idx = (i * lda + i) as usize;
                    let a_val = Complex32::new(a[a_idx].re, conj * a[a_idx].im);
                    let b_idx = (i * ldb + j) as usize;
                    temp += a_val * b[b_idx];
                } else {
                    let b_idx = (i * ldb + j) as usize;
                    temp += b[b_idx];
                }

                let b_idx = (i * ldb + j) as usize;
                b[b_idx] = alpha * temp;
            }
        }
    } else {
        panic!("Unrecognized operation");
    }
}