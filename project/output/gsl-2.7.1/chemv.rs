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
pub unsafe extern "C" fn cblas_chemv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: i32,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: i32,
    mut X: *const libc::c_void,
    incX: i32,
    mut beta: *const libc::c_void,
    mut Y: *mut libc::c_void,
    incY: i32,
) {
    let conj: i32 = if order as u32 == CblasColMajor as i32 as u32 {
        -(1 as i32)
    } else {
        1 as i32
    };
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
    if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
        pos = 6 as i32;
    }
    if incX == 0 as i32 {
        pos = 8 as i32;
    }
    if incY == 0 as i32 {
        pos = 11 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_hemv.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as i32 as isize);
    let beta_real: libc::c_float = *(beta as *const libc::c_float)
        .offset(0 as i32 as isize);
    let beta_imag: libc::c_float = *(beta as *const libc::c_float)
        .offset(1 as i32 as isize);
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64
        && (beta_real as libc::c_double == 1.0f64
            && beta_imag as libc::c_double == 0.0f64)
    {
        return;
    }
    if beta_real as libc::c_double == 0.0f64 && beta_imag as libc::c_double == 0.0f64 {
        let mut iy: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            *(Y as *mut libc::c_float).offset((2 as i32 * iy) as isize) = 0.0f64
                as libc::c_float;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy + 1 as i32) as isize) = 0.0f64
                as libc::c_float;
            iy += incY;
            i += 1;
            i;
        }
    } else if !(beta_real as libc::c_double == 1.0f64
        && beta_imag as libc::c_double == 0.0f64)
    {
        let mut iy_0: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            let y_real: libc::c_float = *(Y as *mut libc::c_float)
                .offset((2 as i32 * iy_0) as isize);
            let y_imag: libc::c_float = *(Y as *mut libc::c_float)
                .offset((2 as i32 * iy_0 + 1 as i32) as isize);
            let tmpR: libc::c_float = y_real * beta_real - y_imag * beta_imag;
            let tmpI: libc::c_float = y_real * beta_imag + y_imag * beta_real;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_0) as isize) = tmpR;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_0 + 1 as i32) as isize) = tmpI;
            iy_0 += incY;
            i += 1;
            i;
        }
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
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
            let mut x_real: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * ix) as isize);
            let mut x_imag: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * ix + 1 as i32) as isize);
            let mut temp1_real: libc::c_float = alpha_real * x_real
                - alpha_imag * x_imag;
            let mut temp1_imag: libc::c_float = alpha_real * x_imag
                + alpha_imag * x_real;
            let mut temp2_real: libc::c_float = 0.0f64 as libc::c_float;
            let mut temp2_imag: libc::c_float = 0.0f64 as libc::c_float;
            let j_min: i32 = i + 1 as i32;
            let j_max: i32 = N;
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
            let mut Aii_real: libc::c_float = *(A as *const libc::c_float)
                .offset((2 as i32 * (lda * i + i)) as isize);
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_1) as isize)
                += temp1_real * Aii_real;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_1 + 1 as i32) as isize)
                += temp1_imag * Aii_real;
            j = j_min;
            while j < j_max {
                let mut Aij_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (lda * i + j)) as isize);
                let mut Aij_imag: libc::c_float = conj as libc::c_float
                    * *(A as *const libc::c_float)
                        .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize);
                *(Y as *mut libc::c_float).offset((2 as i32 * jy) as isize)
                    += temp1_real * Aij_real - temp1_imag * -Aij_imag;
                *(Y as *mut libc::c_float).offset((2 as i32 * jy + 1 as i32) as isize)
                    += temp1_real * -Aij_imag + temp1_imag * Aij_real;
                x_real = *(X as *const libc::c_float).offset((2 as i32 * jx) as isize);
                x_imag = *(X as *const libc::c_float)
                    .offset((2 as i32 * jx + 1 as i32) as isize);
                temp2_real += x_real * Aij_real - x_imag * Aij_imag;
                temp2_imag += x_real * Aij_imag + x_imag * Aij_real;
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_1) as isize)
                += alpha_real * temp2_real - alpha_imag * temp2_imag;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_1 + 1 as i32) as isize)
                += alpha_real * temp2_imag + alpha_imag * temp2_real;
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
        let mut ix_0: i32 = (if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        }) + (N - 1 as i32) * incX;
        let mut iy_2: i32 = (if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        }) + (N - 1 as i32) * incY;
        i = N;
        while i > 0 as i32
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let mut x_real_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * ix_0) as isize);
            let mut x_imag_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            let mut temp1_real_0: libc::c_float = alpha_real * x_real_0
                - alpha_imag * x_imag_0;
            let mut temp1_imag_0: libc::c_float = alpha_real * x_imag_0
                + alpha_imag * x_real_0;
            let mut temp2_real_0: libc::c_float = 0.0f64 as libc::c_float;
            let mut temp2_imag_0: libc::c_float = 0.0f64 as libc::c_float;
            let j_min_0: i32 = 0 as i32;
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
            let mut Aii_real_0: libc::c_float = *(A as *const libc::c_float)
                .offset((2 as i32 * (lda * i + i)) as isize);
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_2) as isize)
                += temp1_real_0 * Aii_real_0;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_2 + 1 as i32) as isize)
                += temp1_imag_0 * Aii_real_0;
            j = j_min_0;
            while j < j_max_0 {
                let mut Aij_real_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (lda * i + j)) as isize);
                let mut Aij_imag_0: libc::c_float = conj as libc::c_float
                    * *(A as *const libc::c_float)
                        .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize);
                *(Y as *mut libc::c_float).offset((2 as i32 * jy_0) as isize)
                    += temp1_real_0 * Aij_real_0 - temp1_imag_0 * -Aij_imag_0;
                *(Y as *mut libc::c_float).offset((2 as i32 * jy_0 + 1 as i32) as isize)
                    += temp1_real_0 * -Aij_imag_0 + temp1_imag_0 * Aij_real_0;
                x_real_0 = *(X as *const libc::c_float)
                    .offset((2 as i32 * jx_0) as isize);
                x_imag_0 = *(X as *const libc::c_float)
                    .offset((2 as i32 * jx_0 + 1 as i32) as isize);
                temp2_real_0 += x_real_0 * Aij_real_0 - x_imag_0 * Aij_imag_0;
                temp2_imag_0 += x_real_0 * Aij_imag_0 + x_imag_0 * Aij_real_0;
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_2) as isize)
                += alpha_real * temp2_real_0 - alpha_imag * temp2_imag_0;
            *(Y as *mut libc::c_float).offset((2 as i32 * iy_2 + 1 as i32) as isize)
                += alpha_real * temp2_imag_0 + alpha_imag * temp2_real_0;
            ix_0 -= incX;
            iy_2 -= incY;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_hemv.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}