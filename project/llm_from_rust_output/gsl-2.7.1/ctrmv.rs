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

pub fn cblas_ctrmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    a: &[Complex32],
    lda: i32,
    x: &mut [Complex32],
    incx: i32,
) {
    // Parameter validation
    if n < 0 {
        panic!("Invalid n: {}", n);
    }
    if lda < 1.max(n) {
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

    let trans_adj = if trans != CBLAS_TRANSPOSE::ConjTrans {
        trans
    } else {
        CBLAS_TRANSPOSE::Trans
    };

    let nonunit = diag == CBLAS_DIAG::NonUnit;

    if order == CBLAS_ORDER::RowMajor && trans_adj == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Upper
        || order == CBLAS_ORDER::ColMajor && trans_adj == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Lower
    {
        let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
        for i in 0..n {
            let mut temp = Complex32::new(0.0, 0.0);
            let j_min = i + 1;
            let mut jx = (if incx > 0 { 0 } else { (n - 1) * -incx }) + incx * j_min;
            for j in j_min..n {
                let x_val = x[(jx / incx) as usize];
                let a_val = Complex32::new(
                    a[(2 * (lda * i + j)) as usize].re,
                    conj * a[(2 * (lda * i + j) + 1) as usize].im,
                );
                temp += a_val * x_val;
                jx += incx;
            }
            if nonunit {
                let x_val = x[(ix / incx) as usize];
                let a_val = Complex32::new(
                    a[(2 * (lda * i + i)) as usize].re,
                    conj * a[(2 * (lda * i + i) + 1) as usize].im,
                );
                x[(ix / incx) as usize] = temp + a_val * x_val;
            } else {
                x[(ix / incx) as usize] += temp;
            }
            ix += incx;
        }
    } else if order == CBLAS_ORDER::RowMajor && trans_adj == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Lower
        || order == CBLAS_ORDER::ColMajor && trans_adj == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Upper
    {
        let mut ix = (if incx > 0 { 0 } else { (n - 1) * -incx }) + (n - 1) * incx;
        for i in (0..n).rev() {
            let mut temp = Complex32::new(0.0, 0.0);
            let j_max = i;
            let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx };
            for j in 0..j_max {
                let x_val = x[(jx / incx) as usize];
                let a_val = Complex32::new(
                    a[(2 * (lda * i + j)) as usize].re,
                    conj * a[(2 * (lda * i + j) + 1) as usize].im,
                );
                temp += a_val * x_val;
                jx += incx;
            }
            if nonunit {
                let x_val = x[(ix / incx) as usize];
                let a_val = Complex32::new(
                    a[(2 * (lda * i + i)) as usize].re,
                    conj * a[(2 * (lda * i + i) + 1) as usize].im,
                );
                x[(ix / incx) as usize] = temp + a_val * x_val;
            } else {
                x[(ix / incx) as usize] += temp;
            }
            ix -= incx;
        }
    } else {
        panic!("Unrecognized operation combination");
    }
}