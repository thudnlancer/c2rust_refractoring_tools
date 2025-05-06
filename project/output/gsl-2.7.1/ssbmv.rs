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
pub type CBLAS_UPLO = u32;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
#[no_mangle]
pub unsafe extern "C" fn cblas_ssbmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: i32,
    K: i32,
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
    let mut pos: i32 = 0 as i32;
    if order as u32 != CblasRowMajor as i32 as u32
        && order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if Uplo as u32 != CblasUpper as i32 as u32 && Uplo as u32 != CblasLower as i32 as u32
    {
        pos = 2 as i32;
    }
    if N < 0 as i32 {
        pos = 3 as i32;
    }
    if K < 0 as i32 {
        pos = 4 as i32;
    }
    if lda < (if 1 as i32 > K + 1 as i32 { 1 as i32 } else { K + 1 as i32 }) {
        pos = 7 as i32;
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
            b"./source_sbmv.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if N == 0 as i32 {
        return;
    }
    if alpha as libc::c_double == 0.0f64 && beta as libc::c_double == 1.0f64 {
        return;
    }
    if beta as libc::c_double == 0.0f64 {
        let mut iy: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            *Y.offset(iy as isize) = 0.0f64 as libc::c_float;
            iy += incY;
            i += 1;
            i;
        }
    } else if beta as libc::c_double != 1.0f64 {
        let mut iy_0: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            *Y.offset(iy_0 as isize) *= beta;
            iy_0 += incY;
            i += 1;
            i;
        }
    }
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if order as u32 == CblasRowMajor as i32 as u32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        let mut ix: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        let mut iy_1: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            let mut tmp1: libc::c_float = alpha * *X.offset(ix as isize);
            let mut tmp2: libc::c_float = 0.0f64 as libc::c_float;
            let j_min: i32 = i + 1 as i32;
            let j_max: i32 = if N < i + K + 1 as i32 { N } else { i + K + 1 as i32 };
            let mut jx: i32 = (if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            }) + j_min * incX;
            let mut jy: i32 = (if incY > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incY
            }) + j_min * incY;
            *Y.offset(iy_1 as isize) += tmp1 * *A.offset((0 as i32 + i * lda) as isize);
            j = j_min;
            while j < j_max {
                let mut Aij: libc::c_float = *A.offset((j - i + i * lda) as isize);
                *Y.offset(jy as isize) += tmp1 * Aij;
                tmp2 += Aij * *X.offset(jx as isize);
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            *Y.offset(iy_1 as isize) += alpha * tmp2;
            ix += incX;
            iy_1 += incY;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        let mut ix_0: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        let mut iy_2: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            let mut tmp1_0: libc::c_float = alpha * *X.offset(ix_0 as isize);
            let mut tmp2_0: libc::c_float = 0.0f64 as libc::c_float;
            let j_min_0: i32 = if i > K { i - K } else { 0 as i32 };
            let j_max_0: i32 = i;
            let mut jx_0: i32 = (if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            }) + j_min_0 * incX;
            let mut jy_0: i32 = (if incY > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incY
            }) + j_min_0 * incY;
            j = j_min_0;
            while j < j_max_0 {
                let mut Aij_0: libc::c_float = *A.offset((K - i + j + i * lda) as isize);
                *Y.offset(jy_0 as isize) += tmp1_0 * Aij_0;
                tmp2_0 += Aij_0 * *X.offset(jx_0 as isize);
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            *Y.offset(iy_2 as isize)
                += tmp1_0 * *A.offset((K + i * lda) as isize) + alpha * tmp2_0;
            ix_0 += incX;
            iy_2 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_sbmv.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}