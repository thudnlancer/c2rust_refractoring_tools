use num_complex::Complex64;
use std::convert::TryFrom;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

pub fn cblas_ztbmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    k: i32,
    a: &[Complex64],
    lda: i32,
    x: &mut [Complex64],
    incx: i32,
) {
    // Parameter validation
    if n < 0 {
        panic!("Invalid n: {}", n);
    }
    if k < 0 {
        panic!("Invalid k: {}", k);
    }
    if lda < std::cmp::max(1, k + 1) {
        panic!("Invalid lda: {}", lda);
    }
    if incx == 0 {
        panic!("Invalid incx: {}", incx);
    }

    let conj = if trans == CBLAS_TRANSPOSE::ConjTrans {
        -1.0
    } else {
        1.0
    };

    let trans_simplified = if trans != CBLAS_TRANSPOSE::ConjTrans {
        trans
    } else {
        CBLAS_TRANSPOSE::Trans
    };

    let nonunit = diag == CBLAS_DIAG::NonUnit;

    match (order, trans_simplified, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
            for i in 0..n {
                let mut temp = Complex64::new(0.0, 0.0);
                let j_min = i + 1;
                let j_max = std::cmp::min(n, i + k + 1);
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + incx * j_min;

                for j in j_min..j_max {
                    let x_val = x[(jx / 2) as usize];
                    let a_idx = (lda * i + (j - i)) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }

                if nonunit {
                    let x_val = x[(ix / 2) as usize];
                    let a_idx = (lda * i) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    x[(ix / 2) as usize] = temp + a_val * x_val;
                } else {
                    x[(ix / 2) as usize] += temp;
                }
                ix += incx;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx } + (n - 1) * incx;
            for i in (0..n).rev() {
                let mut temp = Complex64::new(0.0, 0.0);
                let j_min = if k > i { 0 } else { i - k };
                let j_max = i;
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;

                for j in j_min..j_max {
                    let x_val = x[(jx / 2) as usize];
                    let a_idx = (lda * i + (k - i + j)) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }

                if nonunit {
                    let x_val = x[(ix / 2) as usize];
                    let a_idx = (lda * i + k) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    x[(ix / 2) as usize] = temp + a_val * x_val;
                } else {
                    x[(ix / 2) as usize] += temp;
                }
                ix -= incx;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx } + (n - 1) * incx;
            for i in (0..n).rev() {
                let mut temp = Complex64::new(0.0, 0.0);
                let j_min = if k > i { 0 } else { i - k };
                let j_max = i;
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;

                for j in j_min..j_max {
                    let x_val = x[(jx / 2) as usize];
                    let a_idx = (lda * j + (i - j)) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }

                if nonunit {
                    let x_val = x[(ix / 2) as usize];
                    let a_idx = (lda * i) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    x[(ix / 2) as usize] = temp + a_val * x_val;
                } else {
                    x[(ix / 2) as usize] += temp;
                }
                ix -= incx;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
            for i in 0..n {
                let mut temp = Complex64::new(0.0, 0.0);
                let j_min = i + 1;
                let j_max = std::cmp::min(n, i + k + 1);
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;

                for j in j_min..j_max {
                    let x_val = x[(jx / 2) as usize];
                    let a_idx = (lda * j + (k - j + i)) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }

                if nonunit {
                    let x_val = x[(ix / 2) as usize];
                    let a_idx = (lda * i + k) as usize;
                    let a_val = Complex64::new(
                        a[a_idx].re,
                        conj * a[a_idx].im,
                    );
                    x[(ix / 2) as usize] = temp + a_val * x_val;
                } else {
                    x[(ix / 2) as usize] += temp;
                }
                ix += incx;
            }
        }
        
        _ => panic!("Unrecognized operation combination"),
    }
}