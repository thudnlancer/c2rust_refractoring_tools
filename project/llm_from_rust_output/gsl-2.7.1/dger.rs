use std::ffi::CString;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

#[no_mangle]
pub extern "C" fn cblas_dger(
    order: CBLAS_ORDER,
    m: i32,
    n: i32,
    alpha: f64,
    x: *const f64,
    inc_x: i32,
    y: *const f64,
    inc_y: i32,
    a: *mut f64,
    lda: i32,
) {
    // Validate inputs
    let mut pos = 0;
    if order != CBLAS_ORDER::CblasRowMajor && order != CBLAS_ORDER::CblasColMajor {
        pos = 1;
    } else if m < 0 {
        pos = 2;
    } else if n < 0 {
        pos = 3;
    } else if inc_x == 0 {
        pos = 6;
    } else if inc_y == 0 {
        pos = 8;
    } else {
        let min_lda = match order {
            CBLAS_ORDER::CblasRowMajor => n.max(1),
            CBLAS_ORDER::CblasColMajor => m.max(1),
        };
        if lda < min_lda {
            pos = 10;
        }
    }

    if pos != 0 {
        unsafe {
            cblas_xerbla(
                pos,
                CString::new("./source_ger.h").unwrap().as_ptr(),
                CString::new("").unwrap().as_ptr(),
            );
        }
        return;
    }

    unsafe {
        match order {
            CBLAS_ORDER::CblasRowMajor => {
                let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
                for i in 0..m {
                    let tmp = alpha * *x.offset(ix as isize);
                    let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
                    for j in 0..n {
                        *a.offset((lda * i + j) as isize) += *y.offset(jy as isize) * tmp;
                        jy += inc_y;
                    }
                    ix += inc_x;
                }
            }
            CBLAS_ORDER::CblasColMajor => {
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
                for j in 0..n {
                    let tmp = alpha * *y.offset(jy as isize);
                    let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
                    for i in 0..m {
                        *a.offset((i + lda * j) as isize) += *x.offset(ix as isize) * tmp;
                        ix += inc_x;
                    }
                    jy += inc_y;
                }
            }
        }
    }
}

extern "C" {
    fn cblas_xerbla(p: i32, rout: *const i8, form: *const i8, ...);
}