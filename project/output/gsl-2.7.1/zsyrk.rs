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
pub unsafe extern "C" fn cblas_zsyrk(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: i32,
    K: i32,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: i32,
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
    let mut __dimA: i32 = 0 as i32;
    if Order as u32 == CblasRowMajor as i32 as u32 {
        if Trans as u32 == CblasNoTrans as i32 as u32 {
            __dimA = K;
        } else {
            __dimA = N;
        }
    } else if Trans as u32 == CblasNoTrans as i32 as u32 {
        __dimA = N;
    } else {
        __dimA = K;
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
    if lda < (if 1 as i32 > __dimA { 1 as i32 } else { __dimA }) {
        pos = 8 as i32;
    }
    if ldc < (if 1 as i32 > N { 1 as i32 } else { N }) {
        pos = 11 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_syrk_c.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as i32 as isize);
    let beta_real: libc::c_double = *(beta as *const libc::c_double)
        .offset(0 as i32 as isize);
    let beta_imag: libc::c_double = *(beta as *const libc::c_double)
        .offset(1 as i32 as isize);
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64
        && (beta_real == 1.0f64 && beta_imag == 0.0f64)
    {
        return;
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        uplo = Uplo as i32;
        trans = if Trans as u32 == CblasNoTrans as i32 as u32 {
            CblasNoTrans as i32
        } else {
            CblasTrans as i32
        };
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
    if beta_real == 0.0f64 && beta_imag == 0.0f64 {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                j = i;
                while j < N {
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = 0.0f64;
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = 0.0f64;
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
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = 0.0f64;
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = 0.0f64;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    } else if !(beta_real == 1.0f64 && beta_imag == 0.0f64) {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                j = i;
                while j < N {
                    let Cij_real: libc::c_double = *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j)) as isize);
                    let Cij_imag: libc::c_double = *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = beta_real
                        * Cij_real - beta_imag * Cij_imag;
                    *(C as *mut libc::c_double)
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
                    let Cij_real_0: libc::c_double = *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j)) as isize);
                    let Cij_imag_0: libc::c_double = *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (ldc * i + j)) as isize) = beta_real
                        * Cij_real_0 - beta_imag * Cij_imag_0;
                    *(C as *mut libc::c_double)
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
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64 {
        return;
    }
    if uplo == CblasUpper as i32 && trans == CblasNoTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = i;
            while j < N {
                let mut temp_real: libc::c_double = 0.0f64;
                let mut temp_imag: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    let Aik_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let Ajk_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    temp_real += Aik_real * Ajk_real - Aik_imag * Ajk_imag;
                    temp_imag += Aik_real * Ajk_imag + Aik_imag * Ajk_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp_real - alpha_imag * temp_imag;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag + alpha_imag * temp_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasUpper as i32 && trans == CblasTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = i;
            while j < N {
                let mut temp_real_0: libc::c_double = 0.0f64;
                let mut temp_imag_0: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    let Aki_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + i)) as isize);
                    let Aki_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + i) + 1 as i32) as isize);
                    let Akj_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + j)) as isize);
                    let Akj_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + j) + 1 as i32) as isize);
                    temp_real_0 += Aki_real * Akj_real - Aki_imag * Akj_imag;
                    temp_imag_0 += Aki_real * Akj_imag + Aki_imag * Akj_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp_real_0 - alpha_imag * temp_imag_0;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag_0 + alpha_imag * temp_real_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasLower as i32 && trans == CblasNoTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = 0 as i32;
            while j <= i {
                let mut temp_real_1: libc::c_double = 0.0f64;
                let mut temp_imag_1: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    let Aik_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let Ajk_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    temp_real_1 += Aik_real_0 * Ajk_real_0 - Aik_imag_0 * Ajk_imag_0;
                    temp_imag_1 += Aik_real_0 * Ajk_imag_0 + Aik_imag_0 * Ajk_real_0;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp_real_1 - alpha_imag * temp_imag_1;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag_1 + alpha_imag * temp_real_1;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasLower as i32 && trans == CblasTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = 0 as i32;
            while j <= i {
                let mut temp_real_2: libc::c_double = 0.0f64;
                let mut temp_imag_2: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    let Aki_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + i)) as isize);
                    let Aki_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + i) + 1 as i32) as isize);
                    let Akj_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + j)) as isize);
                    let Akj_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (k * lda + j) + 1 as i32) as isize);
                    temp_real_2 += Aki_real_0 * Akj_real_0 - Aki_imag_0 * Akj_imag_0;
                    temp_imag_2 += Aki_real_0 * Akj_imag_0 + Aki_imag_0 * Akj_real_0;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp_real_2 - alpha_imag * temp_imag_2;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag_2 + alpha_imag * temp_real_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_syrk_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}