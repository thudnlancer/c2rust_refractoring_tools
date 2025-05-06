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
#[no_mangle]
pub unsafe extern "C" fn cblas_csyr2k(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: i32,
    K: i32,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: i32,
    mut B: *const libc::c_void,
    ldb: i32,
    mut beta: *const libc::c_void,
    mut C: *mut libc::c_void,
    ldc: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut uplo: i32 = 0;
    let mut trans: i32 = 0;
    let mut pos: i32 = 0 as i32;
    let mut __dim: i32 = 0 as i32;
    if Order as u32 == CblasRowMajor as i32 as u32 {
        if Trans as u32 == CblasNoTrans as i32 as u32 {
            __dim = K;
        } else {
            __dim = N;
        }
    } else if Trans as u32 == CblasNoTrans as i32 as u32 {
        __dim = N;
    } else {
        __dim = K;
    }
    if Order as u32 != CblasRowMajor as i32 as u32
        && Order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if Uplo as u32 != CblasUpper as i32 as u32 && Uplo as u32 != CblasLower as i32 as u32
    {
        pos = 2 as i32;
    }
    if Trans as u32 != CblasNoTrans as i32 as u32
        && Trans as u32 != CblasTrans as i32 as u32
        && Trans as u32 != CblasConjTrans as i32 as u32
    {
        pos = 3 as i32;
    }
    if N < 0 as i32 {
        pos = 4 as i32;
    }
    if K < 0 as i32 {
        pos = 5 as i32;
    }
    if lda < (if 1 as i32 > __dim { 1 as i32 } else { __dim }) {
        pos = 8 as i32;
    }
    if ldb < (if 1 as i32 > __dim { 1 as i32 } else { __dim }) {
        pos = 11 as i32;
    }
    if ldc < (if 1 as i32 > N { 1 as i32 } else { N }) {
        pos = 14 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_syr2k_c.h\0" as *const u8 as *const i8,
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
    if Order as u32 == CblasRowMajor as i32 as u32 {
        uplo = Uplo as i32;
        trans = Trans as i32;
    } else {
        uplo = if Uplo as u32 == CblasUpper as i32 as u32 {
            CblasLower as i32
        } else {
            CblasUpper as i32
        };
        trans = if Trans as u32 == CblasNoTrans as i32 as u32 {
            CblasTrans as i32
        } else {
            CblasNoTrans as i32
        };
    }
    if beta_real as libc::c_double == 0.0f64 && beta_imag as libc::c_double == 0.0f64 {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                j = i;
                while j < N {
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = 0.0f64
                        as libc::c_float;
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = 0.0f64
                        as libc::c_float;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as i32;
            while i < N {
                j = 0 as i32;
                while j <= i {
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = 0.0f64
                        as libc::c_float;
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = 0.0f64
                        as libc::c_float;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    } else if !(beta_real as libc::c_double == 1.0f64
        && beta_imag as libc::c_double == 0.0f64)
    {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                j = i;
                while j < N {
                    let Cij_real: libc::c_float = *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize);
                    let Cij_imag: libc::c_float = *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = beta_real
                        * Cij_real - beta_imag * Cij_imag;
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = beta_real
                        * Cij_imag + beta_imag * Cij_real;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as i32;
            while i < N {
                j = 0 as i32;
                while j <= i {
                    let Cij_real_0: libc::c_float = *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize);
                    let Cij_imag_0: libc::c_float = *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = beta_real
                        * Cij_real_0 - beta_imag * Cij_imag_0;
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = beta_real
                        * Cij_imag_0 + beta_imag * Cij_real_0;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
        return;
    }
    if uplo == CblasUpper as i32 && trans == CblasNoTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = i;
            while j < N {
                let mut temp_real: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < K {
                    let Aik_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let Bik_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k)) as isize);
                    let Bik_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k) + 1 as i32) as isize);
                    let Ajk_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    let Bjk_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k)) as isize);
                    let Bjk_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k) + 1 as i32) as isize);
                    temp_real
                        += Aik_real * Bjk_real - Aik_imag * Bjk_imag
                            + (Bik_real * Ajk_real - Bik_imag * Ajk_imag);
                    temp_imag
                        += Aik_real * Bjk_imag + Aik_imag * Bjk_real
                            + (Bik_real * Ajk_imag + Bik_imag * Ajk_real);
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp_real - alpha_imag * temp_imag;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag + alpha_imag * temp_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasUpper as i32 && trans == CblasTrans as i32 {
        k = 0 as i32;
        while k < K {
            i = 0 as i32;
            while i < N {
                let mut Aki_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (k * lda + i)) as isize);
                let mut Aki_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (k * lda + i) + 1 as i32) as isize);
                let mut Bki_real: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (k * ldb + i)) as isize);
                let mut Bki_imag: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (k * ldb + i) + 1 as i32) as isize);
                let mut temp1_real: libc::c_float = alpha_real * Aki_real
                    - alpha_imag * Aki_imag;
                let mut temp1_imag: libc::c_float = alpha_real * Aki_imag
                    + alpha_imag * Aki_real;
                let mut temp2_real: libc::c_float = alpha_real * Bki_real
                    - alpha_imag * Bki_imag;
                let mut temp2_imag: libc::c_float = alpha_real * Bki_imag
                    + alpha_imag * Bki_real;
                j = i;
                while j < N {
                    let mut Akj_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (k * lda + j)) as isize);
                    let mut Akj_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (k * lda + j) + 1 as i32) as isize);
                    let mut Bkj_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (k * ldb + j)) as isize);
                    let mut Bkj_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (k * ldb + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (i * lda + j)) as isize)
                        += temp1_real * Bkj_real - temp1_imag * Bkj_imag
                            + (temp2_real * Akj_real - temp2_imag * Akj_imag);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (i * lda + j) + 1 as i32) as isize)
                        += temp1_real * Bkj_imag + temp1_imag * Bkj_real
                            + (temp2_real * Akj_imag + temp2_imag * Akj_real);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else if uplo == CblasLower as i32 && trans == CblasNoTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = 0 as i32;
            while j <= i {
                let mut temp_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < K {
                    let Aik_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let Bik_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k)) as isize);
                    let Bik_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k) + 1 as i32) as isize);
                    let Ajk_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    let Bjk_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k)) as isize);
                    let Bjk_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k) + 1 as i32) as isize);
                    temp_real_0
                        += Aik_real_0 * Bjk_real_0 - Aik_imag_0 * Bjk_imag_0
                            + (Bik_real_0 * Ajk_real_0 - Bik_imag_0 * Ajk_imag_0);
                    temp_imag_0
                        += Aik_real_0 * Bjk_imag_0 + Aik_imag_0 * Bjk_real_0
                            + (Bik_real_0 * Ajk_imag_0 + Bik_imag_0 * Ajk_real_0);
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp_real_0 - alpha_imag * temp_imag_0;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag_0 + alpha_imag * temp_real_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasLower as i32 && trans == CblasTrans as i32 {
        k = 0 as i32;
        while k < K {
            i = 0 as i32;
            while i < N {
                let mut Aki_real_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (k * lda + i)) as isize);
                let mut Aki_imag_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (k * lda + i) + 1 as i32) as isize);
                let mut Bki_real_0: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (k * ldb + i)) as isize);
                let mut Bki_imag_0: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (k * ldb + i) + 1 as i32) as isize);
                let mut temp1_real_0: libc::c_float = alpha_real * Aki_real_0
                    - alpha_imag * Aki_imag_0;
                let mut temp1_imag_0: libc::c_float = alpha_real * Aki_imag_0
                    + alpha_imag * Aki_real_0;
                let mut temp2_real_0: libc::c_float = alpha_real * Bki_real_0
                    - alpha_imag * Bki_imag_0;
                let mut temp2_imag_0: libc::c_float = alpha_real * Bki_imag_0
                    + alpha_imag * Bki_real_0;
                j = 0 as i32;
                while j <= i {
                    let mut Akj_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (k * lda + j)) as isize);
                    let mut Akj_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (k * lda + j) + 1 as i32) as isize);
                    let mut Bkj_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (k * ldb + j)) as isize);
                    let mut Bkj_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (k * ldb + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (i * lda + j)) as isize)
                        += temp1_real_0 * Bkj_real_0 - temp1_imag_0 * Bkj_imag_0
                            + (temp2_real_0 * Akj_real_0 - temp2_imag_0 * Akj_imag_0);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (i * lda + j) + 1 as i32) as isize)
                        += temp1_real_0 * Bkj_imag_0 + temp1_imag_0 * Bkj_real_0
                            + (temp2_real_0 * Akj_imag_0 + temp2_imag_0 * Akj_real_0);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_syr2k_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}