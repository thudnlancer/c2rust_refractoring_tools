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
pub unsafe extern "C" fn cblas_ztpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: i32,
    mut Ap: *const libc::c_void,
    mut X: *mut libc::c_void,
    incX: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
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
    if incX == 0 as i32 {
        pos = 8 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_tpmv_c.h\0" as *const u8 as *const i8,
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
            let Aii_real: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as i32
                        * ((i - 1 as i32 + 1 as i32) * (2 as i32 * N - (i - 1 as i32))
                            / 2 as i32 + i - i)) as isize,
                );
            let Aii_imag: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + i - i)
                            + 1 as i32) as isize,
                    );
            let mut temp_r: libc::c_double = 0.;
            let mut temp_i: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix) as isize);
                let mut x_imag: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix + 1 as i32) as isize);
                temp_r = Aii_real * x_real - Aii_imag * x_imag;
                temp_i = Aii_real * x_imag + Aii_imag * x_real;
            } else {
                temp_r = *(X as *mut libc::c_double).offset((2 as i32 * ix) as isize);
                temp_i = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix + 1 as i32) as isize);
            }
            let mut jx: i32 = (if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            }) + (i + 1 as i32) * incX;
            j = i + 1 as i32;
            while j < N {
                let Aij_real: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + j - i))
                            as isize,
                    );
                let Aij_imag: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as i32
                                * ((i - 1 as i32 + 1 as i32)
                                    * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + j - i)
                                + 1 as i32) as isize,
                        );
                let mut x_real_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx) as isize);
                let mut x_imag_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx + 1 as i32) as isize);
                temp_r += Aij_real * x_real_0 - Aij_imag * x_imag_0;
                temp_i += Aij_real * x_imag_0 + Aij_imag * x_real_0;
                jx += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double).offset((2 as i32 * ix) as isize) = temp_r;
            *(X as *mut libc::c_double).offset((2 as i32 * ix + 1 as i32) as isize) = temp_i;
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
        }) + incX * (N - 1 as i32);
        i = N;
        while i > 0 as i32
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let Aii_real_0: libc::c_double = *(Ap as *const libc::c_double)
                .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i)) as isize);
            let Aii_imag_0: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i) + 1 as i32)
                            as isize,
                    );
            let mut temp_r_0: libc::c_double = 0.;
            let mut temp_i_0: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real_1: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0) as isize);
                let mut x_imag_1: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize);
                temp_r_0 = Aii_real_0 * x_real_1 - Aii_imag_0 * x_imag_1;
                temp_i_0 = Aii_real_0 * x_imag_1 + Aii_imag_0 * x_real_1;
            } else {
                temp_r_0 = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0) as isize);
                temp_i_0 = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            }
            let mut jx_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < i {
                let Aij_real_0: libc::c_double = *(Ap as *const libc::c_double)
                    .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + j)) as isize);
                let Aij_imag_0: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + j) + 1 as i32)
                                as isize,
                        );
                let mut x_real_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_0) as isize);
                let mut x_imag_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_0 + 1 as i32) as isize);
                temp_r_0 += Aij_real_0 * x_real_2 - Aij_imag_0 * x_imag_2;
                temp_i_0 += Aij_real_0 * x_imag_2 + Aij_imag_0 * x_real_2;
                jx_0 += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double).offset((2 as i32 * ix_0) as isize) = temp_r_0;
            *(X as *mut libc::c_double).offset((2 as i32 * ix_0 + 1 as i32) as isize) = temp_i_0;
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
        }) + incX * (N - 1 as i32);
        i = N;
        while i > 0 as i32
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let Aii_real_1: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as i32
                        * ((i - 1 as i32 + 1 as i32) * (2 as i32 * N - (i - 1 as i32))
                            / 2 as i32 + i - i)) as isize,
                );
            let Aii_imag_1: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + i - i)
                            + 1 as i32) as isize,
                    );
            let mut temp_r_1: libc::c_double = 0.;
            let mut temp_i_1: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real_3: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1) as isize);
                let mut x_imag_3: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1 + 1 as i32) as isize);
                temp_r_1 = Aii_real_1 * x_real_3 - Aii_imag_1 * x_imag_3;
                temp_i_1 = Aii_real_1 * x_imag_3 + Aii_imag_1 * x_real_3;
            } else {
                temp_r_1 = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1) as isize);
                temp_i_1 = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_1 + 1 as i32) as isize);
            }
            let mut jx_1: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < i {
                let mut x_real_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_1) as isize);
                let mut x_imag_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_1 + 1 as i32) as isize);
                let Aji_real: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as i32
                            * ((j - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (j - 1 as i32)) / 2 as i32 + i - j))
                            as isize,
                    );
                let Aji_imag: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as i32
                                * ((j - 1 as i32 + 1 as i32)
                                    * (2 as i32 * N - (j - 1 as i32)) / 2 as i32 + i - j)
                                + 1 as i32) as isize,
                        );
                temp_r_1 += Aji_real * x_real_4 - Aji_imag * x_imag_4;
                temp_i_1 += Aji_real * x_imag_4 + Aji_imag * x_real_4;
                jx_1 += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double).offset((2 as i32 * ix_1) as isize) = temp_r_1;
            *(X as *mut libc::c_double).offset((2 as i32 * ix_1 + 1 as i32) as isize) = temp_i_1;
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
            let Aii_real_2: libc::c_double = *(Ap as *const libc::c_double)
                .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i)) as isize);
            let Aii_imag_2: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i) + 1 as i32)
                            as isize,
                    );
            let mut temp_r_2: libc::c_double = 0.;
            let mut temp_i_2: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real_5: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2) as isize);
                let mut x_imag_5: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize);
                temp_r_2 = Aii_real_2 * x_real_5 - Aii_imag_2 * x_imag_5;
                temp_i_2 = Aii_real_2 * x_imag_5 + Aii_imag_2 * x_real_5;
            } else {
                temp_r_2 = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2) as isize);
                temp_i_2 = *(X as *mut libc::c_double)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize);
            }
            let mut jx_2: i32 = (if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            }) + (i + 1 as i32) * incX;
            j = i + 1 as i32;
            while j < N {
                let mut x_real_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_2) as isize);
                let mut x_imag_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as i32 * jx_2 + 1 as i32) as isize);
                let Aji_real_0: libc::c_double = *(Ap as *const libc::c_double)
                    .offset((2 as i32 * (j * (j + 1 as i32) / 2 as i32 + i)) as isize);
                let Aji_imag_0: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as i32 * (j * (j + 1 as i32) / 2 as i32 + i) + 1 as i32)
                                as isize,
                        );
                temp_r_2 += Aji_real_0 * x_real_6 - Aji_imag_0 * x_imag_6;
                temp_i_2 += Aji_real_0 * x_imag_6 + Aji_imag_0 * x_real_6;
                jx_2 += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double).offset((2 as i32 * ix_2) as isize) = temp_r_2;
            *(X as *mut libc::c_double).offset((2 as i32 * ix_2 + 1 as i32) as isize) = temp_i_2;
            ix_2 += incX;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_tpmv_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}