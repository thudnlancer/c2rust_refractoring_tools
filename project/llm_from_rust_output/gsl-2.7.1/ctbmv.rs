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

pub fn cblas_ctbmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    k: i32,
    a: &[Complex32],
    lda: i32,
    x: &mut [Complex32],
    incx: i32,
) {
    // Parameter validation
    if n < 0 {
        panic!("n must be non-negative");
    }
    if k < 0 {
        panic!("k must be non-negative");
    }
    if lda < k + 1 {
        panic!("lda must be at least k+1");
    }
    if incx == 0 {
        panic!("incx cannot be zero");
    }

    let conj = if trans == CBLAS_TRANSPOSE::ConjTrans {
        -1.0
    } else {
        1.0
    };

    let trans = if trans != CBLAS_TRANSPOSE::ConjTrans {
        trans
    } else {
        CBLAS_TRANSPOSE::Trans
    };

    let nonunit = diag == CBLAS_DIAG::NonUnit;

    match (order, trans, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
            for i in 0..n {
                let mut temp = Complex32::new(0.0, 0.0);
                let j_min = i + 1;
                let j_max = (i + k + 1).min(n);
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + incx * j_min;
                
                for j in j_min..j_max {
                    let x_val = x[(jx / incx) as usize];
                    let a_idx = (lda * i + (j - i)) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }
                
                let xi = &mut x[(ix / incx) as usize];
                if nonunit {
                    let a_idx = (lda * i) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    *xi = temp + a_val * *xi;
                } else {
                    *xi += temp;
                }
                ix += incx;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx } + (n - 1) * incx;
            for i in (0..n).rev() {
                let mut temp = Complex32::new(0.0, 0.0);
                let j_min = if k > i { 0 } else { i - k };
                let j_max = i;
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;
                
                for j in j_min..j_max {
                    let x_val = x[(jx / incx) as usize];
                    let a_idx = (lda * i + (k - i + j)) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }
                
                let xi = &mut x[(ix / incx) as usize];
                if nonunit {
                    let a_idx = (lda * i + k) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    *xi = temp + a_val * *xi;
                } else {
                    *xi += temp;
                }
                ix -= incx;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx } + (n - 1) * incx;
            for i in (0..n).rev() {
                let mut temp = Complex32::new(0.0, 0.0);
                let j_min = if k > i { 0 } else { i - k };
                let j_max = i;
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;
                
                for j in j_min..j_max {
                    let x_val = x[(jx / incx) as usize];
                    let a_idx = (lda * j + (i - j)) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }
                
                let xi = &mut x[(ix / incx) as usize];
                if nonunit {
                    let a_idx = (lda * i) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    *xi = temp + a_val * *xi;
                } else {
                    *xi += temp;
                }
                ix -= incx;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) => {
            let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
            for i in 0..n {
                let mut temp = Complex32::new(0.0, 0.0);
                let j_min = i + 1;
                let j_max = (i + k + 1).min(n);
                let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;
                
                for j in j_min..j_max {
                    let x_val = x[(jx / incx) as usize];
                    let a_idx = (lda * j + (k - j + i)) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    temp += a_val * x_val;
                    jx += incx;
                }
                
                let xi = &mut x[(ix / incx) as usize];
                if nonunit {
                    let a_idx = (lda * i + k) as usize;
                    let a_val = Complex32::new(
                        a[a_idx].re,
                        conj * a[a_idx].im
                    );
                    *xi = temp + a_val * *xi;
                } else {
                    *xi += temp;
                }
                ix += incx;
            }
        }
        
        _ => panic!("unrecognized operation"),
    }
}