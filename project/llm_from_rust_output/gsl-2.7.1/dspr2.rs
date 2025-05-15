use std::ffi::CStr;
use std::os::raw::{c_int, c_double, c_char, c_uint};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}

extern "C" {
    fn cblas_xerbla(p: c_int, rout: *const c_char, form: *const c_char, ...);
}

pub fn cblas_dspr2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: c_int,
    alpha: c_double,
    x: &[c_double],
    inc_x: c_int,
    y: &[c_double],
    inc_y: c_int,
    ap: &mut [c_double],
) {
    let mut pos = 0;

    if order != CBLAS_ORDER::CblasRowMajor && order != CBLAS_ORDER::CblasColMajor {
        pos = 1;
    }
    if uplo != CBLAS_UPLO::CblasUpper && uplo != CBLAS_UPLO::CblasLower {
        pos = 2;
    }
    if n < 0 {
        pos = 3;
    }
    if inc_x == 0 {
        pos = 6;
    }
    if inc_y == 0 {
        pos = 8;
    }

    if pos != 0 {
        unsafe {
            cblas_xerbla(
                pos,
                b"./source_spr2.h\0".as_ptr() as *const c_char,
                b"\0".as_ptr() as *const c_char,
            );
        }
        return;
    }

    if n == 0 || alpha == 0.0 {
        return;
    }

    let x_len = x.len();
    let y_len = y.len();
    let ap_len = ap.len();

    if (order == CBLAS_ORDER::CblasRowMajor && uplo == CBLAS_UPLO::CblasUpper)
        || (order == CBLAS_ORDER::CblasColMajor && uplo == CBLAS_UPLO::CblasLower)
    {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

        for i in 0..n {
            if ix < 0 || ix as usize >= x_len || iy < 0 || iy as usize >= y_len {
                unsafe {
                    cblas_xerbla(
                        0,
                        b"./source_spr2.h\0".as_ptr() as *const c_char,
                        b"index out of bounds\0".as_ptr() as *const c_char,
                    );
                }
                return;
            }

            let tmp1 = alpha * x[ix as usize];
            let tmp2 = alpha * y[iy as usize];
            let mut jx = ix;
            let mut jy = iy;

            for j in i..n {
                if jx < 0 || jx as usize >= x_len || jy < 0 || jy as usize >= y_len {
                    unsafe {
                        cblas_xerbla(
                            0,
                            b"./source_spr2.h\0".as_ptr() as *const c_char,
                            b"index out of bounds\0".as_ptr() as *const c_char,
                        );
                    }
                    return;
                }

                let idx = ((i - 1 + 1) * (2 * n - (i - 1)) / 2 + j - i) as usize;
                if idx >= ap_len {
                    unsafe {
                        cblas_xerbla(
                            0,
                            b"./source_spr2.h\0".as_ptr() as *const c_char,
                            b"index out of bounds\0".as_ptr() as *const c_char,
                        );
                    }
                    return;
                }

                ap[idx] += tmp1 * y[jy as usize] + tmp2 * x[jx as usize];
                jx += inc_x;
                jy += inc_y;
            }

            ix += inc_x;
            iy += inc_y;
        }
    } else if (order == CBLAS_ORDER::CblasRowMajor && uplo == CBLAS_UPLO::CblasLower)
        || (order == CBLAS_ORDER::CblasColMajor && uplo == CBLAS_UPLO::CblasUpper)
    {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

        for i in 0..n {
            if ix < 0 || ix as usize >= x_len || iy < 0 || iy as usize >= y_len {
                unsafe {
                    cblas_xerbla(
                        0,
                        b"./source_spr2.h\0".as_ptr() as *const c_char,
                        b"index out of bounds\0".as_ptr() as *const c_char,
                    );
                }
                return;
            }

            let tmp1 = alpha * x[ix as usize];
            let tmp2 = alpha * y[iy as usize];
            let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

            for j in 0..=i {
                if jx < 0 || jx as usize >= x_len || jy < 0 || jy as usize >= y_len {
                    unsafe {
                        cblas_xerbla(
                            0,
                            b"./source_spr2.h\0".as_ptr() as *const c_char,
                            b"index out of bounds\0".as_ptr() as *const c_char,
                        );
                    }
                    return;
                }

                let idx = (i * (i + 1) / 2 + j) as usize;
                if idx >= ap_len {
                    unsafe {
                        cblas_xerbla(
                            0,
                            b"./source_spr2.h\0".as_ptr() as *const c_char,
                            b"index out of bounds\0".as_ptr() as *const c_char,
                        );
                    }
                    return;
                }

                ap[idx] += tmp1 * y[jy as usize] + tmp2 * x[jx as usize];
                jx += inc_x;
                jy += inc_y;
            }

            ix += inc_x;
            iy += inc_y;
        }
    } else {
        unsafe {
            cblas_xerbla(
                0,
                b"./source_spr2.h\0".as_ptr() as *const c_char,
                b"unrecognized operation\0".as_ptr() as *const c_char,
            );
        }
    }
}