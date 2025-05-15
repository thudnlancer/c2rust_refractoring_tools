use num_complex::Complex32;
use std::f32::consts;
use std::f64;

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

fn xhypot(x: f64, y: f64) -> f64 {
    let xabs = x.abs();
    let yabs = y.abs();
    let (min, max) = if xabs < yabs {
        (xabs, yabs)
    } else {
        (yabs, xabs)
    };

    if min == 0.0 {
        return max;
    }

    let u = min / max;
    max * (1.0 + u * u).sqrt()
}

pub fn cblas_ctrsv(
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

    // Parameter validation
    if n == 0 {
        return;
    }

    // Main logic
    if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Upper)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Lower)
    {
        let mut ix = if incx > 0 {
            (n - 1) * incx
        } else {
            (1 - n) * incx
        };

        if nonunit {
            let a_idx = (lda * (n - 1) + (n - 1)) as usize;
            let a_val = a[a_idx];
            let x_val = x[ix as usize];
            let s = xhypot(a_val.re as f64, (a_val.im * conj) as f64) as f32;
            let b = Complex32::new(a_val.re / s, (a_val.im * conj) / s);
            x[ix as usize] = Complex32::new(
                (x_val.re * b.re + x_val.im * b.im) / s,
                (x_val.im * b.re - b.im * x_val.re) / s,
            );
        }

        ix -= incx;
        for i in (1..n).rev() {
            let mut tmp = x[ix as usize];
            let mut jx = ix + incx;
            for j in (i + 1)..n {
                let a_idx = (lda * i + j) as usize;
                let a_val = Complex32::new(
                    a[a_idx].re,
                    a[a_idx].im * conj,
                );
                let x_val = x[jx as usize];
                tmp -= a_val * x_val;
                jx += incx;
            }

            if nonunit {
                let a_idx = (lda * i + i) as usize;
                let a_val = Complex32::new(
                    a[a_idx].re,
                    a[a_idx].im * conj,
                );
                let s = xhypot(a_val.re as f64, a_val.im as f64) as f32;
                let b = Complex32::new(a_val.re / s, a_val.im / s);
                x[ix as usize] = Complex32::new(
                    (tmp.re * b.re + tmp.im * b.im) / s,
                    (tmp.im * b.re - tmp.re * b.im) / s,
                );
            } else {
                x[ix as usize] = tmp;
            }
            ix -= incx;
        }
    } else if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Lower)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Upper)
    {
        let mut ix = if incx > 0 { 0 } else { (1 - n) * incx };

        if nonunit {
            let a_val = a[0];
            let x_val = x[ix as usize];
            let s = xhypot(a_val.re as f64, (a_val.im * conj) as f64) as f32;
            let b = Complex32::new(a_val.re / s, (a_val.im * conj) / s);
            x[ix as usize] = Complex32::new(
                (x_val.re * b.re + x_val.im * b.im) / s,
                (x_val.im * b.re - b.im * x_val.re) / s,
            );
        }

        ix += incx;
        for i in 1..n {
            let mut tmp = x[ix as usize];
            let mut jx = if incx > 0 { 0 } else { (1 - n) * incx };
            for j in 0..i {
                let a_idx = (lda * i + j) as usize;
                let a_val = Complex32::new(
                    a[a_idx].re,
                    a[a_idx].im * conj,
                );
                let x_val = x[jx as usize];
                tmp -= a_val * x_val;
                jx += incx;
            }

            if nonunit {
                let a_idx = (lda * i + i) as usize;
                let a_val = Complex32::new(
                    a[a_idx].re,
                    a[a_idx].im * conj,
                );
                let s = xhypot(a_val.re as f64, a_val.im as f64) as f32;
                let b = Complex32::new(a_val.re / s, a_val.im / s);
                x[ix as usize] = Complex32::new(
                    (tmp.re * b.re + tmp.im * b.im) / s,
                    (tmp.im * b.re - tmp.re * b.im) / s,
                );
            } else {
                x[ix as usize] = tmp;
            }
            ix += incx;
        }
    } else {
        panic!("Unrecognized operation");
    }
}