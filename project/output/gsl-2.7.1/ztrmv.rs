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
pub type CBLAS_UPLO = u32;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = u32;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
#[no_mangle]
pub unsafe extern "C" fn cblas_ztrmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: i32,
    mut A: *const libc::c_void,
    lda: i32,
    mut X: *mut libc::c_void,
    incX: i32,
) {
    let conj: i32 = if TransA as u32 == CblasConjTrans as i32 as u32 {
        -(1 as i32)
    } else {
        1 as i32
    };
    let Trans: i32 = (if TransA as u32 != CblasConjTrans as i32 as u32 {
        TransA as u32
    } else {
        CblasTrans as i32 as u32
    }) as i32;
    let nonunit: i32 = (Diag as u32 == CblasNonUnit as i32 as u32) as i32;
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
    if TransA as u32 != CblasNoTrans as i32 as u32
        && TransA as u32 != CblasTrans as i32 as u32
        && TransA as u32 != CblasConjTrans as i32 as u32
    {
        pos = 3 as i32;
    }
    if Diag as u32 != CblasNonUnit as i32 as u32
        && Diag as u32 != CblasUnit as i32 as u32
    {
        pos = 4 as i32;
    }
    if N < 0 as i32 {
        pos = 5 as i32;
    }
    if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
        pos = 7 as i32;
    }
    if incX == 0 as i32 {
        pos = 9 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_trmv_c.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasNoTrans as i32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasTrans as i32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        let mut ix: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        i = 0 as i32;
        while i < N {
            let mut temp_r: libc::c_double = 0.0f64;
            let mut temp_i: libc::c_double = 0.0f64;
            let j_min: i32 = i + 1 as i32;
            let mut jx: i32 = (if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            }) + incX * j_min;
            j = j_min;
            while j < N {
                let x_real: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx) as isize);
                let x_imag: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx + 1 as i32) as isize);
                let A_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + j)) as isize);
                let A_imag: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize);
                temp_r += A_real * x_real - A_imag * x_imag;
                temp_i += A_real * x_imag + A_imag * x_real;
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let x_real_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix) as isize);
                let x_imag_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix + 1 as i32) as isize);
                let A_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + i)) as isize);
                let A_imag_0: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * i + i) + 1 as i32) as isize);
                *(X as *mut libc::c_double).offset((2 as i32 * ix) as isize) = temp_r
                    + (A_real_0 * x_real_0 - A_imag_0 * x_imag_0);
                *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix + 1 as i32) as isize) = temp_i
                    + (A_real_0 * x_imag_0 + A_imag_0 * x_real_0);
            } else {
                *(X as *mut libc::c_double).offset((2 as i32 * ix) as isize) += temp_r;
                *(X as *mut libc::c_double).offset((2 as i32 * ix + 1 as i32) as isize)
                    += temp_i;
            }
            ix += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasNoTrans as i32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasTrans as i32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        let mut ix_0: i32 = (if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        }) + (N - 1 as i32) * incX;
        i = N;
        while i > 0 as i32
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let mut temp_r_0: libc::c_double = 0.0f64;
            let mut temp_i_0: libc::c_double = 0.0f64;
            let j_max: i32 = i;
            let mut jx_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < j_max {
                let x_real_1: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_0) as isize);
                let x_imag_1: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_0 + 1 as i32) as isize);
                let A_real_1: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + j)) as isize);
                let A_imag_1: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize);
                temp_r_0 += A_real_1 * x_real_1 - A_imag_1 * x_imag_1;
                temp_i_0 += A_real_1 * x_imag_1 + A_imag_1 * x_real_1;
                jx_0 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let x_real_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0) as isize);
                let x_imag_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize);
                let A_real_2: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + i)) as isize);
                let A_imag_2: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * i + i) + 1 as i32) as isize);
                *(X as *mut libc::c_double).offset((2 as i32 * ix_0) as isize) = temp_r_0
                    + (A_real_2 * x_real_2 - A_imag_2 * x_imag_2);
                *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize) = temp_i_0
                    + (A_real_2 * x_imag_2 + A_imag_2 * x_real_2);
            } else {
                *(X as *mut libc::c_double).offset((2 as i32 * ix_0) as isize)
                    += temp_r_0;
                *(X as *mut libc::c_double).offset((2 as i32 * ix_0 + 1 as i32) as isize)
                    += temp_i_0;
            }
            ix_0 -= incX;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        let mut ix_1: i32 = (if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        }) + (N - 1 as i32) * incX;
        i = N;
        while i > 0 as i32
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let mut temp_r_1: libc::c_double = 0.0f64;
            let mut temp_i_1: libc::c_double = 0.0f64;
            let j_max_0: i32 = i;
            let mut jx_1: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < j_max_0 {
                let x_real_3: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_1) as isize);
                let x_imag_3: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_1 + 1 as i32) as isize);
                let A_real_3: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * j + i)) as isize);
                let A_imag_3: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * j + i) + 1 as i32) as isize);
                temp_r_1 += A_real_3 * x_real_3 - A_imag_3 * x_imag_3;
                temp_i_1 += A_real_3 * x_imag_3 + A_imag_3 * x_real_3;
                jx_1 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let x_real_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1) as isize);
                let x_imag_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1 + 1 as i32) as isize);
                let A_real_4: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + i)) as isize);
                let A_imag_4: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * i + i) + 1 as i32) as isize);
                *(X as *mut libc::c_double).offset((2 as i32 * ix_1) as isize) = temp_r_1
                    + (A_real_4 * x_real_4 - A_imag_4 * x_imag_4);
                *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1 + 1 as i32) as isize) = temp_i_1
                    + (A_real_4 * x_imag_4 + A_imag_4 * x_real_4);
            } else {
                *(X as *mut libc::c_double).offset((2 as i32 * ix_1) as isize)
                    += temp_r_1;
                *(X as *mut libc::c_double).offset((2 as i32 * ix_1 + 1 as i32) as isize)
                    += temp_i_1;
            }
            ix_1 -= incX;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        let mut ix_2: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        i = 0 as i32;
        while i < N {
            let mut temp_r_2: libc::c_double = 0.0f64;
            let mut temp_i_2: libc::c_double = 0.0f64;
            let j_min_0: i32 = i + 1 as i32;
            let mut jx_2: i32 = (if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            }) + j_min_0 * incX;
            j = j_min_0;
            while j < N {
                let x_real_5: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_2) as isize);
                let x_imag_5: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_2 + 1 as i32) as isize);
                let A_real_5: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * j + i)) as isize);
                let A_imag_5: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * j + i) + 1 as i32) as isize);
                temp_r_2 += A_real_5 * x_real_5 - A_imag_5 * x_imag_5;
                temp_i_2 += A_real_5 * x_imag_5 + A_imag_5 * x_real_5;
                jx_2 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let x_real_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2) as isize);
                let x_imag_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize);
                let A_real_6: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + i)) as isize);
                let A_imag_6: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset((2 as i32 * (lda * i + i) + 1 as i32) as isize);
                *(X as *mut libc::c_double).offset((2 as i32 * ix_2) as isize) = temp_r_2
                    + (A_real_6 * x_real_6 - A_imag_6 * x_imag_6);
                *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize) = temp_i_2
                    + (A_real_6 * x_imag_6 + A_imag_6 * x_real_6);
            } else {
                *(X as *mut libc::c_double).offset((2 as i32 * ix_2) as isize)
                    += temp_r_2;
                *(X as *mut libc::c_double).offset((2 as i32 * ix_2 + 1 as i32) as isize)
                    += temp_i_2;
            }
            ix_2 += incX;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_trmv_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}