use std::ffi::CString;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

#[no_mangle]
pub extern "C" fn cblas_dtrmv(
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
    // Validate parameters
    if n < 0 {
        cblas_xerbla(5, "./source_trmv_r.h", "");
        return;
    }
    if lda < std::cmp::max(1, n) {
        cblas_xerbla(7, "./source_trmv_r.h", "");
        return;
    }
    if inc_x == 0 {
        cblas_xerbla(9, "./source_trmv_r.h", "");
        return;
    }

    let nonunit = diag == CBLAS_DIAG::NonUnit;
    let trans = match trans_a {
        CBLAS_TRANSPOSE::ConjTrans => CBLAS_TRANSPOSE::Trans,
        _ => trans_a,
    };

    match (order, trans, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            for i in 0..n {
                let mut temp = 0.0;
                let j_min = i + 1;
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + j_min * inc_x;
                for j in j_min..n {
                    temp += x[(jx as usize)] * a[(lda * i + j) as usize];
                    jx += inc_x;
                }
                if nonunit {
                    x[ix as usize] = temp + x[ix as usize] * a[(lda * i + i) as usize];
                } else {
                    x[ix as usize] += temp;
                }
                ix += inc_x;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + (n - 1) * inc_x;
            for i in (0..n).rev() {
                let mut temp = 0.0;
                let j_min = 0;
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + j_min * inc_x;
                for j in j_min..i {
                    temp += x[(jx as usize)] * a[(lda * i + j) as usize];
                    jx += inc_x;
                }
                if nonunit {
                    x[ix as usize] = temp + x[ix as usize] * a[(lda * i + i) as usize];
                } else {
                    x[ix as usize] += temp;
                }
                ix -= inc_x;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Upper)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + (n - 1) * inc_x;
            for i in (0..n).rev() {
                let mut temp = 0.0;
                let j_min = 0;
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + j_min * inc_x;
                for j in j_min..i {
                    temp += x[(jx as usize)] * a[(lda * j + i) as usize];
                    jx += inc_x;
                }
                if nonunit {
                    x[ix as usize] = temp + x[ix as usize] * a[(lda * i + i) as usize];
                } else {
                    x[ix as usize] += temp;
                }
                ix -= inc_x;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans, CBLAS_UPLO::Lower)
        | (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            for i in 0..n {
                let mut temp = 0.0;
                let j_min = i + 1;
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + (i + 1) * inc_x;
                for j in j_min..n {
                    temp += x[(jx as usize)] * a[(lda * j + i) as usize];
                    jx += inc_x;
                }
                if nonunit {
                    x[ix as usize] = temp + x[ix as usize] * a[(lda * i + i) as usize];
                } else {
                    x[ix as usize] += temp;
                }
                ix += inc_x;
            }
        }
        _ => {
            cblas_xerbla(0, "./source_trmv_r.h", "unrecognized operation");
        }
    }
}

fn cblas_xerbla(p: i32, rout: &str, form: &str) {
    unsafe {
        let rout_c = CString::new(rout).unwrap();
        let form_c = CString::new(form).unwrap();
        libc::cblas_xerbla(p, rout_c.as_ptr(), form_c.as_ptr());
    }
}

extern "C" {
    fn cblas_xerbla(p: i32, rout: *const libc::c_char, form: *const libc::c_char, ...);
}