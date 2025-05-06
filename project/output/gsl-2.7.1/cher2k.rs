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
pub unsafe extern "C" fn cblas_cher2k(
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
    beta: libc::c_float,
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
            b"./source_her2k.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as i32 as isize);
    let mut alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as i32 as isize);
    if beta as libc::c_double == 1.0f64
        && (alpha_real as libc::c_double == 0.0f64
            && alpha_imag as libc::c_double == 0.0f64 || K == 0 as i32)
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
            CblasConjTrans as i32
        } else {
            CblasNoTrans as i32
        };
        alpha_imag *= -(1 as i32) as libc::c_float;
    }
    if beta as libc::c_double == 0.0f64 {
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
    } else if beta as libc::c_double != 1.0f64 {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                *(C as *mut libc::c_float).offset((2 as i32 * (ldc * i + i)) as isize)
                    *= beta;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (ldc * i + i) + 1 as i32) as isize) = 0.0f64
                    as libc::c_float;
                j = i + 1 as i32;
                while j < N {
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize) *= beta;
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) *= beta;
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
                while j < i {
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j)) as isize) *= beta;
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) *= beta;
                    j += 1;
                    j;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (ldc * i + i)) as isize)
                    *= beta;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (ldc * i + i) + 1 as i32) as isize) = 0.0f64
                    as libc::c_float;
                i += 1;
                i;
            }
        }
    } else {
        i = 0 as i32;
        while i < N {
            *(C as *mut libc::c_float)
                .offset((2 as i32 * (ldc * i + i) + 1 as i32) as isize) = 0.0f64
                as libc::c_float;
            i += 1;
            i;
        }
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
        return;
    }
    if uplo == CblasUpper as i32 && trans == CblasNoTrans as i32 {
        i = 0 as i32;
        while i < N {
            let mut temp_real: libc::c_float = 0.0f64 as libc::c_float;
            k = 0 as i32;
            while k < K {
                let Aik_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (i * lda + k)) as isize);
                let Aik_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                let temp1_real: libc::c_float = alpha_real * Aik_real
                    - alpha_imag * Aik_imag;
                let temp1_imag: libc::c_float = alpha_real * Aik_imag
                    + alpha_imag * Aik_real;
                let Bik_real: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (i * ldb + k)) as isize);
                let Bik_imag: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (i * ldb + k) + 1 as i32) as isize);
                temp_real += temp1_real * Bik_real + temp1_imag * Bik_imag;
                k += 1;
                k;
            }
            *(C as *mut libc::c_float).offset((2 as i32 * (i * ldc + i)) as isize)
                += 2 as i32 as libc::c_float * temp_real;
            *(C as *mut libc::c_float)
                .offset((2 as i32 * (i * ldc + i) + 1 as i32) as isize) = 0.0f64
                as libc::c_float;
            j = i + 1 as i32;
            while j < N {
                let mut temp_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < K {
                    let Aik_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let temp1_real_0: libc::c_float = alpha_real * Aik_real_0
                        - alpha_imag * Aik_imag_0;
                    let temp1_imag_0: libc::c_float = alpha_real * Aik_imag_0
                        + alpha_imag * Aik_real_0;
                    let Bik_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k)) as isize);
                    let Bik_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k) + 1 as i32) as isize);
                    let Ajk_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    let temp2_real: libc::c_float = alpha_real * Ajk_real
                        - alpha_imag * Ajk_imag;
                    let temp2_imag: libc::c_float = alpha_real * Ajk_imag
                        + alpha_imag * Ajk_real;
                    let Bjk_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k)) as isize);
                    let Bjk_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k) + 1 as i32) as isize);
                    temp_real_0
                        += temp1_real_0 * Bjk_real + temp1_imag_0 * Bjk_imag
                            + (Bik_real_0 * temp2_real + Bik_imag_0 * temp2_imag);
                    temp_imag
                        += temp1_real_0 * -Bjk_imag + temp1_imag_0 * Bjk_real
                            + (Bik_real_0 * -temp2_imag + Bik_imag_0 * temp2_real);
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (i * ldc + j)) as isize)
                    += temp_real_0;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize) += temp_imag;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasUpper as i32 && trans == CblasConjTrans as i32 {
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
                let mut temp1_real_1: libc::c_float = alpha_real * Aki_real
                    - alpha_imag * -Aki_imag;
                let mut temp1_imag_1: libc::c_float = alpha_real * -Aki_imag
                    + alpha_imag * Aki_real;
                let mut temp2_real_0: libc::c_float = alpha_real * Bki_real
                    - alpha_imag * Bki_imag;
                let mut temp2_imag_0: libc::c_float = -(alpha_real * Bki_imag
                    + alpha_imag * Bki_real);
                *(C as *mut libc::c_float).offset((2 as i32 * (i * lda + i)) as isize)
                    += 2 as i32 as libc::c_float
                        * (temp1_real_1 * Bki_real - temp1_imag_1 * Bki_imag);
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (i * lda + i) + 1 as i32) as isize) = 0.0f64
                    as libc::c_float;
                j = i + 1 as i32;
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
                        += temp1_real_1 * Bkj_real - temp1_imag_1 * Bkj_imag
                            + (temp2_real_0 * Akj_real - temp2_imag_0 * Akj_imag);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (i * lda + j) + 1 as i32) as isize)
                        += temp1_real_1 * Bkj_imag + temp1_imag_1 * Bkj_real
                            + (temp2_real_0 * Akj_imag + temp2_imag_0 * Akj_real);
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
            while j < i {
                let mut temp_real_1: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < K {
                    let Aik_real_1: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag_1: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let temp1_real_2: libc::c_float = alpha_real * Aik_real_1
                        - alpha_imag * Aik_imag_1;
                    let temp1_imag_2: libc::c_float = alpha_real * Aik_imag_1
                        + alpha_imag * Aik_real_1;
                    let Bik_real_1: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k)) as isize);
                    let Bik_imag_1: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (i * ldb + k) + 1 as i32) as isize);
                    let Ajk_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    let temp2_real_1: libc::c_float = alpha_real * Ajk_real_0
                        - alpha_imag * Ajk_imag_0;
                    let temp2_imag_1: libc::c_float = alpha_real * Ajk_imag_0
                        + alpha_imag * Ajk_real_0;
                    let Bjk_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k)) as isize);
                    let Bjk_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as i32 * (j * ldb + k) + 1 as i32) as isize);
                    temp_real_1
                        += temp1_real_2 * Bjk_real_0 + temp1_imag_2 * Bjk_imag_0
                            + (Bik_real_1 * temp2_real_1 + Bik_imag_1 * temp2_imag_1);
                    temp_imag_0
                        += temp1_real_2 * -Bjk_imag_0 + temp1_imag_2 * Bjk_real_0
                            + (Bik_real_1 * -temp2_imag_1 + Bik_imag_1 * temp2_real_1);
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (i * ldc + j)) as isize)
                    += temp_real_1;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += temp_imag_0;
                j += 1;
                j;
            }
            let mut temp_real_2: libc::c_float = 0.0f64 as libc::c_float;
            k = 0 as i32;
            while k < K {
                let Aik_real_2: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (i * lda + k)) as isize);
                let Aik_imag_2: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                let temp1_real_3: libc::c_float = alpha_real * Aik_real_2
                    - alpha_imag * Aik_imag_2;
                let temp1_imag_3: libc::c_float = alpha_real * Aik_imag_2
                    + alpha_imag * Aik_real_2;
                let Bik_real_2: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (i * ldb + k)) as isize);
                let Bik_imag_2: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as i32 * (i * ldb + k) + 1 as i32) as isize);
                temp_real_2 += temp1_real_3 * Bik_real_2 + temp1_imag_3 * Bik_imag_2;
                k += 1;
                k;
            }
            *(C as *mut libc::c_float).offset((2 as i32 * (i * ldc + i)) as isize)
                += 2 as i32 as libc::c_float * temp_real_2;
            *(C as *mut libc::c_float)
                .offset((2 as i32 * (i * ldc + i) + 1 as i32) as isize) = 0.0f64
                as libc::c_float;
            i += 1;
            i;
        }
    } else if uplo == CblasLower as i32 && trans == CblasConjTrans as i32 {
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
                let mut temp1_real_4: libc::c_float = alpha_real * Aki_real_0
                    - alpha_imag * -Aki_imag_0;
                let mut temp1_imag_4: libc::c_float = alpha_real * -Aki_imag_0
                    + alpha_imag * Aki_real_0;
                let mut temp2_real_2: libc::c_float = alpha_real * Bki_real_0
                    - alpha_imag * Bki_imag_0;
                let mut temp2_imag_2: libc::c_float = -(alpha_real * Bki_imag_0
                    + alpha_imag * Bki_real_0);
                j = 0 as i32;
                while j < i {
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
                        += temp1_real_4 * Bkj_real_0 - temp1_imag_4 * Bkj_imag_0
                            + (temp2_real_2 * Akj_real_0 - temp2_imag_2 * Akj_imag_0);
                    *(C as *mut libc::c_float)
                        .offset((2 as i32 * (i * lda + j) + 1 as i32) as isize)
                        += temp1_real_4 * Bkj_imag_0 + temp1_imag_4 * Bkj_real_0
                            + (temp2_real_2 * Akj_imag_0 + temp2_imag_2 * Akj_real_0);
                    j += 1;
                    j;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (i * lda + i)) as isize)
                    += 2 as i32 as libc::c_float
                        * (temp1_real_4 * Bki_real_0 - temp1_imag_4 * Bki_imag_0);
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (i * lda + i) + 1 as i32) as isize) = 0.0f64
                    as libc::c_float;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_her2k.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}