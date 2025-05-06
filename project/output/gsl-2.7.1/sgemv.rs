#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn cblas_xerbla(p: i32, rout: *const i8, form: *const i8, _: ...);
}
pub type CBLAS_ORDER = u32;
pub const CblasColMajor: CBLAS_ORDER = 102;
pub const CblasRowMajor: CBLAS_ORDER = 101;
pub type CBLAS_TRANSPOSE = u32;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
#[no_mangle]
pub unsafe extern "C" fn cblas_sgemv(
    order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    M: i32,
    N: i32,
    alpha: libc::c_float,
    mut A: *const libc::c_float,
    lda: i32,
    mut X: *const libc::c_float,
    incX: i32,
    beta: libc::c_float,
    mut Y: *mut libc::c_float,
    incY: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut lenX: i32 = 0;
    let mut lenY: i32 = 0;
    let Trans: i32 = (if TransA as u32 != CblasConjTrans as i32 as u32 {
        TransA as u32
    } else {
        CblasTrans as i32 as u32
    }) as i32;
    let mut pos: i32 = 0 as i32;
    if order as u32 != CblasRowMajor as i32 as u32
        && order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if TransA as u32 != CblasNoTrans as i32 as u32
        && TransA as u32 != CblasTrans as i32 as u32
        && TransA as u32 != CblasConjTrans as i32 as u32
    {
        pos = 2 as i32;
    }
    if M < 0 as i32 {
        pos = 3 as i32;
    }
    if N < 0 as i32 {
        pos = 4 as i32;
    }
    if order as u32 == CblasRowMajor as i32 as u32 {
        if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 7 as i32;
        }
    } else if order as u32 == CblasColMajor as i32 as u32 {
        if lda < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 7 as i32;
        }
    }
    if incX == 0 as i32 {
        pos = 9 as i32;
    }
    if incY == 0 as i32 {
        pos = 12 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_gemv_r.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if M == 0 as i32 || N == 0 as i32 {
        return;
    }
    if alpha as libc::c_double == 0.0f64 && beta as libc::c_double == 1.0f64 {
        return;
    }
    if Trans == CblasNoTrans as i32 {
        lenX = N;
        lenY = M;
    } else {
        lenX = M;
        lenY = N;
    }
    if beta as libc::c_double == 0.0f64 {
        let mut iy: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            *Y.offset(iy as isize) = 0.0f64 as libc::c_float;
            iy += incY;
            i += 1;
            i;
        }
    } else if beta as libc::c_double != 1.0f64 {
        let mut iy_0: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            *Y.offset(iy_0 as isize) *= beta;
            iy_0 += incY;
            i += 1;
            i;
        }
    }
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasNoTrans as i32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasTrans as i32
    {
        let mut iy_1: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            let mut temp: libc::c_float = 0.0f64 as libc::c_float;
            let mut ix: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (lenX - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < lenX {
                temp += *X.offset(ix as isize) * *A.offset((lda * i + j) as isize);
                ix += incX;
                j += 1;
                j;
            }
            *Y.offset(iy_1 as isize) += alpha * temp;
            iy_1 += incY;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
    {
        let mut ix_0: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (lenX - 1 as i32) * -incX
        };
        j = 0 as i32;
        while j < lenX {
            let temp_0: libc::c_float = alpha * *X.offset(ix_0 as isize);
            if temp_0 as libc::c_double != 0.0f64 {
                let mut iy_2: i32 = if incY > 0 as i32 {
                    0 as i32
                } else {
                    (lenY - 1 as i32) * -incY
                };
                i = 0 as i32;
                while i < lenY {
                    *Y.offset(iy_2 as isize)
                        += temp_0 * *A.offset((lda * j + i) as isize);
                    iy_2 += incY;
                    i += 1;
                    i;
                }
            }
            ix_0 += incX;
            j += 1;
            j;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_gemv_r.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}