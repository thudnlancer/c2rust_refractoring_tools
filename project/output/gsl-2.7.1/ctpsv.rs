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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn xhypot(x: libc::c_double, y: libc::c_double) -> libc::c_double {
    let mut xabs: libc::c_double = fabs(x);
    let mut yabs: libc::c_double = fabs(y);
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    if xabs < yabs {
        min = xabs;
        max = yabs;
    } else {
        min = yabs;
        max = xabs;
    }
    if min == 0 as i32 as libc::c_double {
        return max;
    }
    let mut u: libc::c_double = min / max;
    return max * sqrt(1 as i32 as libc::c_double + u * u);
}
#[no_mangle]
pub unsafe extern "C" fn cblas_ctpsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: i32,
    mut Ap: *const libc::c_void,
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
    if incX == 0 as i32 {
        pos = 8 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_tpsv_c.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if N == 0 as i32 {
        return;
    }
    if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasNoTrans as i32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasTrans as i32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        let mut ix: i32 = (if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        }) + incX * (N - 1 as i32);
        if nonunit != 0 {
            let a_real: libc::c_float = *(Ap as *const libc::c_float)
                .offset(
                    (2 as i32
                        * ((N - 1 as i32 - 1 as i32 + 1 as i32)
                            * (2 as i32 * N - (N - 1 as i32 - 1 as i32)) / 2 as i32
                            + (N - 1 as i32) - (N - 1 as i32))) as isize,
                );
            let a_imag: libc::c_float = conj as libc::c_float
                * *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((N - 1 as i32 - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (N - 1 as i32 - 1 as i32)) / 2 as i32
                                + (N - 1 as i32) - (N - 1 as i32)) + 1 as i32) as isize,
                    );
            let x_real: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix) as isize);
            let x_imag: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix + 1 as i32) as isize);
            let s: libc::c_float = xhypot(
                a_real as libc::c_double,
                a_imag as libc::c_double,
            ) as libc::c_float;
            let b_real: libc::c_float = a_real / s;
            let b_imag: libc::c_float = a_imag / s;
            *(X as *mut libc::c_float).offset((2 as i32 * ix) as isize) = (x_real
                * b_real + x_imag * b_imag) / s;
            *(X as *mut libc::c_float).offset((2 as i32 * ix + 1 as i32) as isize) = (x_imag
                * b_real - b_imag * x_real) / s;
        }
        ix -= incX;
        i = N - 1 as i32;
        while i > 0 as i32
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let mut tmp_real: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix) as isize);
            let mut tmp_imag: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix + 1 as i32) as isize);
            let mut jx: i32 = ix + incX;
            j = i + 1 as i32;
            while j < N {
                let Aij_real: libc::c_float = *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + j - i))
                            as isize,
                    );
                let Aij_imag: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32
                                * ((i - 1 as i32 + 1 as i32)
                                    * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + j - i)
                                + 1 as i32) as isize,
                        );
                let x_real_0: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx) as isize);
                let x_imag_0: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx + 1 as i32) as isize);
                tmp_real -= Aij_real * x_real_0 - Aij_imag * x_imag_0;
                tmp_imag -= Aij_real * x_imag_0 + Aij_imag * x_real_0;
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_0: libc::c_float = *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + i - i))
                            as isize,
                    );
                let a_imag_0: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32
                                * ((i - 1 as i32 + 1 as i32)
                                    * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + i - i)
                                + 1 as i32) as isize,
                        );
                let s_0: libc::c_float = xhypot(
                    a_real_0 as libc::c_double,
                    a_imag_0 as libc::c_double,
                ) as libc::c_float;
                let b_real_0: libc::c_float = a_real_0 / s_0;
                let b_imag_0: libc::c_float = a_imag_0 / s_0;
                *(X as *mut libc::c_float).offset((2 as i32 * ix) as isize) = (tmp_real
                    * b_real_0 + tmp_imag * b_imag_0) / s_0;
                *(X as *mut libc::c_float).offset((2 as i32 * ix + 1 as i32) as isize) = (tmp_imag
                    * b_real_0 - tmp_real * b_imag_0) / s_0;
            } else {
                *(X as *mut libc::c_float).offset((2 as i32 * ix) as isize) = tmp_real;
                *(X as *mut libc::c_float).offset((2 as i32 * ix + 1 as i32) as isize) = tmp_imag;
            }
            ix -= incX;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasNoTrans as i32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasTrans as i32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        let mut ix_0: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        if nonunit != 0 {
            let a_real_1: libc::c_float = *(Ap as *const libc::c_float)
                .offset(
                    (2 as i32 * (0 as i32 * (0 as i32 + 1 as i32) / 2 as i32 + 0 as i32))
                        as isize,
                );
            let a_imag_1: libc::c_float = conj as libc::c_float
                * *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * (0 as i32 * (0 as i32 + 1 as i32) / 2 as i32 + 0 as i32)
                            + 1 as i32) as isize,
                    );
            let x_real_1: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_0) as isize);
            let x_imag_1: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            let s_1: libc::c_float = xhypot(
                a_real_1 as libc::c_double,
                a_imag_1 as libc::c_double,
            ) as libc::c_float;
            let b_real_1: libc::c_float = a_real_1 / s_1;
            let b_imag_1: libc::c_float = a_imag_1 / s_1;
            *(X as *mut libc::c_float).offset((2 as i32 * ix_0) as isize) = (x_real_1
                * b_real_1 + x_imag_1 * b_imag_1) / s_1;
            *(X as *mut libc::c_float).offset((2 as i32 * ix_0 + 1 as i32) as isize) = (x_imag_1
                * b_real_1 - b_imag_1 * x_real_1) / s_1;
        }
        ix_0 += incX;
        i = 1 as i32;
        while i < N {
            let mut tmp_real_0: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_0) as isize);
            let mut tmp_imag_0: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            let mut jx_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < i {
                let Aij_real_0: libc::c_float = *(Ap as *const libc::c_float)
                    .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + j)) as isize);
                let Aij_imag_0: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + j) + 1 as i32)
                                as isize,
                        );
                let x_real_2: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx_0) as isize);
                let x_imag_2: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx_0 + 1 as i32) as isize);
                tmp_real_0 -= Aij_real_0 * x_real_2 - Aij_imag_0 * x_imag_2;
                tmp_imag_0 -= Aij_real_0 * x_imag_2 + Aij_imag_0 * x_real_2;
                jx_0 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_2: libc::c_float = *(Ap as *const libc::c_float)
                    .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i)) as isize);
                let a_imag_2: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i) + 1 as i32)
                                as isize,
                        );
                let s_2: libc::c_float = xhypot(
                    a_real_2 as libc::c_double,
                    a_imag_2 as libc::c_double,
                ) as libc::c_float;
                let b_real_2: libc::c_float = a_real_2 / s_2;
                let b_imag_2: libc::c_float = a_imag_2 / s_2;
                *(X as *mut libc::c_float).offset((2 as i32 * ix_0) as isize) = (tmp_real_0
                    * b_real_2 + tmp_imag_0 * b_imag_2) / s_2;
                *(X as *mut libc::c_float)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize) = (tmp_imag_0
                    * b_real_2 - tmp_real_0 * b_imag_2) / s_2;
            } else {
                *(X as *mut libc::c_float).offset((2 as i32 * ix_0) as isize) = tmp_real_0;
                *(X as *mut libc::c_float)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize) = tmp_imag_0;
            }
            ix_0 += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        let mut ix_1: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        if nonunit != 0 {
            let a_real_3: libc::c_float = *(Ap as *const libc::c_float)
                .offset(
                    (2 as i32
                        * ((0 as i32 - 1 as i32 + 1 as i32)
                            * (2 as i32 * N - (0 as i32 - 1 as i32)) / 2 as i32
                            + 0 as i32 - 0 as i32)) as isize,
                );
            let a_imag_3: libc::c_float = conj as libc::c_float
                * *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((0 as i32 - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (0 as i32 - 1 as i32)) / 2 as i32
                                + 0 as i32 - 0 as i32) + 1 as i32) as isize,
                    );
            let x_real_3: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_1) as isize);
            let x_imag_3: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_1 + 1 as i32) as isize);
            let s_3: libc::c_float = xhypot(
                a_real_3 as libc::c_double,
                a_imag_3 as libc::c_double,
            ) as libc::c_float;
            let b_real_3: libc::c_float = a_real_3 / s_3;
            let b_imag_3: libc::c_float = a_imag_3 / s_3;
            *(X as *mut libc::c_float).offset((2 as i32 * ix_1) as isize) = (x_real_3
                * b_real_3 + x_imag_3 * b_imag_3) / s_3;
            *(X as *mut libc::c_float).offset((2 as i32 * ix_1 + 1 as i32) as isize) = (x_imag_3
                * b_real_3 - b_imag_3 * x_real_3) / s_3;
        }
        ix_1 += incX;
        i = 1 as i32;
        while i < N {
            let mut tmp_real_1: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_1) as isize);
            let mut tmp_imag_1: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_1 + 1 as i32) as isize);
            let mut jx_1: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < i {
                let Aij_real_1: libc::c_float = *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((j - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (j - 1 as i32)) / 2 as i32 + i - j))
                            as isize,
                    );
                let Aij_imag_1: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32
                                * ((j - 1 as i32 + 1 as i32)
                                    * (2 as i32 * N - (j - 1 as i32)) / 2 as i32 + i - j)
                                + 1 as i32) as isize,
                        );
                let x_real_4: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx_1) as isize);
                let x_imag_4: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx_1 + 1 as i32) as isize);
                tmp_real_1 -= Aij_real_1 * x_real_4 - Aij_imag_1 * x_imag_4;
                tmp_imag_1 -= Aij_real_1 * x_imag_4 + Aij_imag_1 * x_real_4;
                jx_1 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_4: libc::c_float = *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + i - i))
                            as isize,
                    );
                let a_imag_4: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32
                                * ((i - 1 as i32 + 1 as i32)
                                    * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + i - i)
                                + 1 as i32) as isize,
                        );
                let s_4: libc::c_float = xhypot(
                    a_real_4 as libc::c_double,
                    a_imag_4 as libc::c_double,
                ) as libc::c_float;
                let b_real_4: libc::c_float = a_real_4 / s_4;
                let b_imag_4: libc::c_float = a_imag_4 / s_4;
                *(X as *mut libc::c_float).offset((2 as i32 * ix_1) as isize) = (tmp_real_1
                    * b_real_4 + tmp_imag_1 * b_imag_4) / s_4;
                *(X as *mut libc::c_float)
                    .offset((2 as i32 * ix_1 + 1 as i32) as isize) = (tmp_imag_1
                    * b_real_4 - tmp_real_1 * b_imag_4) / s_4;
            } else {
                *(X as *mut libc::c_float).offset((2 as i32 * ix_1) as isize) = tmp_real_1;
                *(X as *mut libc::c_float)
                    .offset((2 as i32 * ix_1 + 1 as i32) as isize) = tmp_imag_1;
            }
            ix_1 += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        let mut ix_2: i32 = (if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        }) + incX * (N - 1 as i32);
        if nonunit != 0 {
            let a_real_5: libc::c_float = *(Ap as *const libc::c_float)
                .offset(
                    (2 as i32
                        * ((N - 1 as i32) * (N - 1 as i32 + 1 as i32) / 2 as i32
                            + (N - 1 as i32))) as isize,
                );
            let a_imag_5: libc::c_float = conj as libc::c_float
                * *(Ap as *const libc::c_float)
                    .offset(
                        (2 as i32
                            * ((N - 1 as i32) * (N - 1 as i32 + 1 as i32) / 2 as i32
                                + (N - 1 as i32)) + 1 as i32) as isize,
                    );
            let x_real_5: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_2) as isize);
            let x_imag_5: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_2 + 1 as i32) as isize);
            let s_5: libc::c_float = xhypot(
                a_real_5 as libc::c_double,
                a_imag_5 as libc::c_double,
            ) as libc::c_float;
            let b_real_5: libc::c_float = a_real_5 / s_5;
            let b_imag_5: libc::c_float = a_imag_5 / s_5;
            *(X as *mut libc::c_float).offset((2 as i32 * ix_2) as isize) = (x_real_5
                * b_real_5 + x_imag_5 * b_imag_5) / s_5;
            *(X as *mut libc::c_float).offset((2 as i32 * ix_2 + 1 as i32) as isize) = (x_imag_5
                * b_real_5 - b_imag_5 * x_real_5) / s_5;
        }
        ix_2 -= incX;
        i = N - 1 as i32;
        while i > 0 as i32
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let mut tmp_real_2: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_2) as isize);
            let mut tmp_imag_2: libc::c_float = *(X as *mut libc::c_float)
                .offset((2 as i32 * ix_2 + 1 as i32) as isize);
            let mut jx_2: i32 = ix_2 + incX;
            j = i + 1 as i32;
            while j < N {
                let Aij_real_2: libc::c_float = *(Ap as *const libc::c_float)
                    .offset((2 as i32 * (j * (j + 1 as i32) / 2 as i32 + i)) as isize);
                let Aij_imag_2: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32 * (j * (j + 1 as i32) / 2 as i32 + i) + 1 as i32)
                                as isize,
                        );
                let x_real_6: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx_2) as isize);
                let x_imag_6: libc::c_float = *(X as *mut libc::c_float)
                    .offset((2 as i32 * jx_2 + 1 as i32) as isize);
                tmp_real_2 -= Aij_real_2 * x_real_6 - Aij_imag_2 * x_imag_6;
                tmp_imag_2 -= Aij_real_2 * x_imag_6 + Aij_imag_2 * x_real_6;
                jx_2 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_6: libc::c_float = *(Ap as *const libc::c_float)
                    .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i)) as isize);
                let a_imag_6: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i) + 1 as i32)
                                as isize,
                        );
                let s_6: libc::c_float = xhypot(
                    a_real_6 as libc::c_double,
                    a_imag_6 as libc::c_double,
                ) as libc::c_float;
                let b_real_6: libc::c_float = a_real_6 / s_6;
                let b_imag_6: libc::c_float = a_imag_6 / s_6;
                *(X as *mut libc::c_float).offset((2 as i32 * ix_2) as isize) = (tmp_real_2
                    * b_real_6 + tmp_imag_2 * b_imag_6) / s_6;
                *(X as *mut libc::c_float)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize) = (tmp_imag_2
                    * b_real_6 - tmp_real_2 * b_imag_6) / s_6;
            } else {
                *(X as *mut libc::c_float).offset((2 as i32 * ix_2) as isize) = tmp_real_2;
                *(X as *mut libc::c_float)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize) = tmp_imag_2;
            }
            ix_2 -= incX;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_tpsv_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}