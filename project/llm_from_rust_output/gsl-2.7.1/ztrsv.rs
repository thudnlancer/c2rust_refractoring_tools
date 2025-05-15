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

#[no_mangle]
pub fn cblas_ztrsv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    a: &[f64],
    lda: i32,
    x: &mut [f64],
    inc_x: i32,
) {
    let conj = if trans_a == CBLAS_TRANSPOSE::ConjTrans { -1 } else { 1 };
    let trans = if trans_a != CBLAS_TRANSPOSE::ConjTrans {
        trans_a
    } else {
        CBLAS_TRANSPOSE::Trans
    };
    let nonunit = (diag == CBLAS_DIAG::NonUnit) as i32;

    // Parameter validation
    if ![CBLAS_ORDER::RowMajor, CBLAS_ORDER::ColMajor].contains(&order) {
        panic!("Invalid order parameter");
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
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if lda < if 1 > n { 1 } else { n } {
        panic!("Invalid lda parameter");
    }
    if inc_x == 0 {
        panic!("Invalid inc_x parameter");
    }

    if n == 0 {
        return;
    }

    let complex_len = 2 * n as usize;
    if a.len() < complex_len || x.len() < complex_len {
        panic!("Insufficient array length");
    }

    if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Upper)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Lower)
    {
        let mut ix = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + inc_x * (n - 1);
        if nonunit != 0 {
            let a_idx = 2 * (lda * (n - 1) + (n - 1));
            let a_real = a[a_idx as usize];
            let a_imag = conj as f64 * a[(a_idx + 1) as usize];
            let x_real = x[(2 * ix) as usize];
            let x_imag = x[(2 * ix + 1) as usize];
            let s = xhypot(a_real, a_imag);
            let b_real = a_real / s;
            let b_imag = a_imag / s;
            x[(2 * ix) as usize] = (x_real * b_real + x_imag * b_imag) / s;
            x[(2 * ix + 1) as usize] = (x_imag * b_real - b_imag * x_real) / s;
        }
        ix -= inc_x;
        let mut i = n - 1;
        while i > 0 {
            let mut tmp_real = x[(2 * ix) as usize];
            let mut tmp_imag = x[(2 * ix + 1) as usize];
            let mut jx = ix + inc_x;
            let mut j = i + 1;
            while j < n {
                let a_idx = 2 * (lda * i + j);
                let aij_real = a[a_idx as usize];
                let aij_imag = conj as f64 * a[(a_idx + 1) as usize];
                let x_real = x[(2 * jx) as usize];
                let x_imag = x[(2 * jx + 1) as usize];
                tmp_real -= aij_real * x_real - aij_imag * x_imag;
                tmp_imag -= aij_real * x_imag + aij_imag * x_real;
                jx += inc_x;
                j += 1;
            }
            if nonunit != 0 {
                let a_idx = 2 * (lda * i + i);
                let a_real = a[a_idx as usize];
                let a_imag = conj as f64 * a[(a_idx + 1) as usize];
                let s = xhypot(a_real, a_imag);
                let b_real = a_real / s;
                let b_imag = a_imag / s;
                x[(2 * ix) as usize] = (tmp_real * b_real + tmp_imag * b_imag) / s;
                x[(2 * ix + 1) as usize] = (tmp_imag * b_real - tmp_real * b_imag) / s;
            } else {
                x[(2 * ix) as usize] = tmp_real;
                x[(2 * ix + 1) as usize] = tmp_imag;
            }
            ix -= inc_x;
            i -= 1;
        }
    } else {
        panic!("Unrecognized operation");
    }
}