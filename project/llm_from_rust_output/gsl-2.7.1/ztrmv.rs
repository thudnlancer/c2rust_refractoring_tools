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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

pub fn cblas_ztrmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    a: &[Complex64],
    lda: i32,
    x: &mut [Complex64],
    inc_x: i32,
) {
    // Parameter validation
    if !(order == CBLAS_ORDER::RowMajor || order == CBLAS_ORDER::ColMajor) {
        panic!("Invalid order parameter");
    }
    if !(uplo == CBLAS_UPLO::Upper || uplo == CBLAS_UPLO::Lower) {
        panic!("Invalid uplo parameter");
    }
    if !matches!(trans_a, CBLAS_TRANSPOSE::NoTrans | CBLAS_TRANSPOSE::Trans | CBLAS_TRANSPOSE::ConjTrans) {
        panic!("Invalid trans_a parameter");
    }
    if !(diag == CBLAS_DIAG::NonUnit || diag == CBLAS_DIAG::Unit) {
        panic!("Invalid diag parameter");
    }
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if lda < 1.max(n) {
        panic!("Invalid lda parameter");
    }
    if inc_x == 0 {
        panic!("Invalid inc_x parameter");
    }

    let conj = if trans_a == CBLAS_TRANSPOSE::ConjTrans { -1.0 } else { 1.0 };
    let trans = if trans_a != CBLAS_TRANSPOSE::ConjTrans {
        trans_a
    } else {
        CBLAS_TRANSPOSE::Trans
    };
    let nonunit = diag == CBLAS_DIAG::NonUnit;

    if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Upper)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Lower)
    {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
        for i in 0..n {
            let mut temp = Complex64::new(0.0, 0.0);
            let j_min = i + 1;
            let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + inc_x * j_min;
            
            for j in j_min..n {
                let x_val = x[(jx / inc_x) as usize];
                let a_val = if order == CBLAS_ORDER::RowMajor {
                    a[(i * lda + j) as usize]
                } else {
                    a[(j * lda + i) as usize]
                };
                let a_val_conj = Complex64::new(a_val.re, conj * a_val.im);
                temp += a_val_conj * x_val;
                jx += inc_x;
            }

            let diag_val = if nonunit {
                let a_val = a[(i * lda + i) as usize];
                Complex64::new(a_val.re, conj * a_val.im)
            } else {
                Complex64::new(1.0, 0.0)
            };

            x[(ix / inc_x) as usize] = temp + diag_val * x[(ix / inc_x) as usize];
            ix += inc_x;
        }
    } else if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Lower)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Upper)
    {
        let mut ix = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + (n - 1) * inc_x;
        for i in (0..n).rev() {
            let mut temp = Complex64::new(0.0, 0.0);
            let j_max = i;
            let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            
            for j in 0..j_max {
                let x_val = x[(jx / inc_x) as usize];
                let a_val = if order == CBLAS_ORDER::RowMajor {
                    a[(i * lda + j) as usize]
                } else {
                    a[(j * lda + i) as usize]
                };
                let a_val_conj = Complex64::new(a_val.re, conj * a_val.im);
                temp += a_val_conj * x_val;
                jx += inc_x;
            }

            let diag_val = if nonunit {
                let a_val = a[(i * lda + i) as usize];
                Complex64::new(a_val.re, conj * a_val.im)
            } else {
                Complex64::new(1.0, 0.0)
            };

            x[(ix / inc_x) as usize] = temp + diag_val * x[(ix / inc_x) as usize];
            ix -= inc_x;
        }
    } else if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Upper)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Lower)
    {
        let mut ix = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + (n - 1) * inc_x;
        for i in (0..n).rev() {
            let mut temp = Complex64::new(0.0, 0.0);
            let j_max = i;
            let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            
            for j in 0..j_max {
                let x_val = x[(jx / inc_x) as usize];
                let a_val = if order == CBLAS_ORDER::RowMajor {
                    a[(j * lda + i) as usize]
                } else {
                    a[(i * lda + j) as usize]
                };
                let a_val_conj = Complex64::new(a_val.re, conj * a_val.im);
                temp += a_val_conj * x_val;
                jx += inc_x;
            }

            let diag_val = if nonunit {
                let a_val = a[(i * lda + i) as usize];
                Complex64::new(a_val.re, conj * a_val.im)
            } else {
                Complex64::new(1.0, 0.0)
            };

            x[(ix / inc_x) as usize] = temp + diag_val * x[(ix / inc_x) as usize];
            ix -= inc_x;
        }
    } else if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Lower)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Upper)
    {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
        for i in 0..n {
            let mut temp = Complex64::new(0.0, 0.0);
            let j_min = i + 1;
            let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + j_min * inc_x;
            
            for j in j_min..n {
                let x_val = x[(jx / inc_x) as usize];
                let a_val = if order == CBLAS_ORDER::RowMajor {
                    a[(j * lda + i) as usize]
                } else {
                    a[(i * lda + j) as usize]
                };
                let a_val_conj = Complex64::new(a_val.re, conj * a_val.im);
                temp += a_val_conj * x_val;
                jx += inc_x;
            }

            let diag_val = if nonunit {
                let a_val = a[(i * lda + i) as usize];
                Complex64::new(a_val.re, conj * a_val.im)
            } else {
                Complex64::new(1.0, 0.0)
            };

            x[(ix / inc_x) as usize] = temp + diag_val * x[(ix / inc_x) as usize];
            ix += inc_x;
        }
    } else {
        panic!("Unrecognized operation");
    }
}