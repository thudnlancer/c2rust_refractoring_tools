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
pub type CBLAS_SIDE = u32;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
#[no_mangle]
pub unsafe extern "C" fn cblas_zsymm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: i32,
    N: i32,
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
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut uplo: i32 = 0;
    let mut side: i32 = 0;
    let mut pos: i32 = 0 as i32;
    let mut __dimA: i32 = 0 as i32;
    if Side as u32 == CblasLeft as i32 as u32 {
        __dimA = M;
    } else {
        __dimA = N;
    }
    if Order as u32 != CblasRowMajor as i32 as u32
        && Order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if Side as u32 != CblasLeft as i32 as u32 && Side as u32 != CblasRight as i32 as u32
    {
        pos = 2 as i32;
    }
    if Uplo as u32 != CblasUpper as i32 as u32 && Uplo as u32 != CblasLower as i32 as u32
    {
        pos = 3 as i32;
    }
    if M < 0 as i32 {
        pos = 4 as i32;
    }
    if N < 0 as i32 {
        pos = 5 as i32;
    }
    if lda < (if 1 as i32 > __dimA { 1 as i32 } else { __dimA }) {
        pos = 8 as i32;
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        if ldb < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 10 as i32;
        }
        if ldc < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 13 as i32;
        }
    } else if Order as u32 == CblasColMajor as i32 as u32 {
        if ldb < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 10 as i32;
        }
        if ldc < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 13 as i32;
        }
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_symm_c.h\0" as *const u8 as *const i8,
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
        n1 = M;
        n2 = N;
        uplo = Uplo as i32;
        side = Side as i32;
    } else {
        n1 = N;
        n2 = M;
        uplo = if Uplo as u32 == CblasUpper as i32 as u32 {
            CblasLower as i32
        } else {
            CblasUpper as i32
        };
        side = if Side as u32 == CblasLeft as i32 as u32 {
            CblasRight as i32
        } else {
            CblasLeft as i32
        };
    }
    if beta_real == 0.0f64 && beta_imag == 0.0f64 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
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
    } else if !(beta_real == 1.0f64 && beta_imag == 0.0f64) {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let Cij_real: libc::c_double = *(C as *mut libc::c_double)
                    .offset((2 as i32 * (ldc * i + j)) as isize);
                let Cij_imag: libc::c_double = *(C as *mut libc::c_double)
                    .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize);
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (ldc * i + j)) as isize) = beta_real * Cij_real
                    - beta_imag * Cij_imag;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize) = beta_real
                    * Cij_imag + beta_imag * Cij_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64 {
        return;
    }
    if side == CblasLeft as i32 && uplo == CblasUpper as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let Bij_real: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j)) as isize);
                let Bij_imag: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j) + 1 as i32) as isize);
                let temp1_real: libc::c_double = alpha_real * Bij_real
                    - alpha_imag * Bij_imag;
                let temp1_imag: libc::c_double = alpha_real * Bij_imag
                    + alpha_imag * Bij_real;
                let mut temp2_real: libc::c_double = 0.0f64;
                let mut temp2_imag: libc::c_double = 0.0f64;
                let Aii_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (i * lda + i)) as isize);
                let Aii_imag: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (i * lda + i) + 1 as i32) as isize);
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += temp1_real * Aii_real - temp1_imag * Aii_imag;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += temp1_real * Aii_imag + temp1_imag * Aii_real;
                k = i + 1 as i32;
                while k < n1 {
                    let Aik_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let Bkj_real: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * k + j)) as isize);
                    let Bkj_imag: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * k + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (k * ldc + j)) as isize)
                        += Aik_real * temp1_real - Aik_imag * temp1_imag;
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (k * ldc + j) + 1 as i32) as isize)
                        += Aik_real * temp1_imag + Aik_imag * temp1_real;
                    temp2_real += Aik_real * Bkj_real - Aik_imag * Bkj_imag;
                    temp2_imag += Aik_real * Bkj_imag + Aik_imag * Bkj_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real - alpha_imag * temp2_imag;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp2_imag + alpha_imag * temp2_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as i32 && uplo == CblasLower as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let Bij_real_0: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j)) as isize);
                let Bij_imag_0: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j) + 1 as i32) as isize);
                let temp1_real_0: libc::c_double = alpha_real * Bij_real_0
                    - alpha_imag * Bij_imag_0;
                let temp1_imag_0: libc::c_double = alpha_real * Bij_imag_0
                    + alpha_imag * Bij_real_0;
                let mut temp2_real_0: libc::c_double = 0.0f64;
                let mut temp2_imag_0: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < i {
                    let Aik_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (i * lda + k) + 1 as i32) as isize);
                    let Bkj_real_0: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * k + j)) as isize);
                    let Bkj_imag_0: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * k + j) + 1 as i32) as isize);
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (k * ldc + j)) as isize)
                        += Aik_real_0 * temp1_real_0 - Aik_imag_0 * temp1_imag_0;
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (k * ldc + j) + 1 as i32) as isize)
                        += Aik_real_0 * temp1_imag_0 + Aik_imag_0 * temp1_real_0;
                    temp2_real_0 += Aik_real_0 * Bkj_real_0 - Aik_imag_0 * Bkj_imag_0;
                    temp2_imag_0 += Aik_real_0 * Bkj_imag_0 + Aik_imag_0 * Bkj_real_0;
                    k += 1;
                    k;
                }
                let Aii_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (i * lda + i)) as isize);
                let Aii_imag_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (i * lda + i) + 1 as i32) as isize);
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += temp1_real_0 * Aii_real_0 - temp1_imag_0 * Aii_imag_0;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += temp1_real_0 * Aii_imag_0 + temp1_imag_0 * Aii_real_0;
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real_0 - alpha_imag * temp2_imag_0;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp2_imag_0 + alpha_imag * temp2_real_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as i32 && uplo == CblasUpper as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let Bij_real_1: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j)) as isize);
                let Bij_imag_1: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j) + 1 as i32) as isize);
                let temp1_real_1: libc::c_double = alpha_real * Bij_real_1
                    - alpha_imag * Bij_imag_1;
                let temp1_imag_1: libc::c_double = alpha_real * Bij_imag_1
                    + alpha_imag * Bij_real_1;
                let mut temp2_real_1: libc::c_double = 0.0f64;
                let mut temp2_imag_1: libc::c_double = 0.0f64;
                let Ajj_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (j * lda + j)) as isize);
                let Ajj_imag: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (j * lda + j) + 1 as i32) as isize);
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += temp1_real_1 * Ajj_real - temp1_imag_1 * Ajj_imag;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += temp1_real_1 * Ajj_imag + temp1_imag_1 * Ajj_real;
                k = j + 1 as i32;
                while k < n2 {
                    let Ajk_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    let Bik_real: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * i + k)) as isize);
                    let Bik_imag: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * i + k) + 1 as i32) as isize);
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (i * ldc + k)) as isize)
                        += temp1_real_1 * Ajk_real - temp1_imag_1 * Ajk_imag;
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (i * ldc + k) + 1 as i32) as isize)
                        += temp1_real_1 * Ajk_imag + temp1_imag_1 * Ajk_real;
                    temp2_real_1 += Bik_real * Ajk_real - Bik_imag * Ajk_imag;
                    temp2_imag_1 += Bik_real * Ajk_imag + Bik_imag * Ajk_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real_1 - alpha_imag * temp2_imag_1;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp2_imag_1 + alpha_imag * temp2_real_1;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as i32 && uplo == CblasLower as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let Bij_real_2: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j)) as isize);
                let Bij_imag_2: libc::c_double = *(B as *const libc::c_double)
                    .offset((2 as i32 * (ldb * i + j) + 1 as i32) as isize);
                let temp1_real_2: libc::c_double = alpha_real * Bij_real_2
                    - alpha_imag * Bij_imag_2;
                let temp1_imag_2: libc::c_double = alpha_real * Bij_imag_2
                    + alpha_imag * Bij_real_2;
                let mut temp2_real_2: libc::c_double = 0.0f64;
                let mut temp2_imag_2: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < j {
                    let Ajk_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as i32 * (j * lda + k) + 1 as i32) as isize);
                    let Bik_real_0: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * i + k)) as isize);
                    let Bik_imag_0: libc::c_double = *(B as *const libc::c_double)
                        .offset((2 as i32 * (ldb * i + k) + 1 as i32) as isize);
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (i * ldc + k)) as isize)
                        += temp1_real_2 * Ajk_real_0 - temp1_imag_2 * Ajk_imag_0;
                    *(C as *mut libc::c_double)
                        .offset((2 as i32 * (i * ldc + k) + 1 as i32) as isize)
                        += temp1_real_2 * Ajk_imag_0 + temp1_imag_2 * Ajk_real_0;
                    temp2_real_2 += Bik_real_0 * Ajk_real_0 - Bik_imag_0 * Ajk_imag_0;
                    temp2_imag_2 += Bik_real_0 * Ajk_imag_0 + Bik_imag_0 * Ajk_real_0;
                    k += 1;
                    k;
                }
                let Ajj_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (j * lda + j)) as isize);
                let Ajj_imag_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (j * lda + j) + 1 as i32) as isize);
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += temp1_real_2 * Ajj_real_0 - temp1_imag_2 * Ajj_imag_0;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += temp1_real_2 * Ajj_imag_0 + temp1_imag_2 * Ajj_real_0;
                *(C as *mut libc::c_double).offset((2 as i32 * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real_2 - alpha_imag * temp2_imag_2;
                *(C as *mut libc::c_double)
                    .offset((2 as i32 * (i * ldc + j) + 1 as i32) as isize)
                    += alpha_real * temp2_imag_2 + alpha_imag * temp2_real_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_symm_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}