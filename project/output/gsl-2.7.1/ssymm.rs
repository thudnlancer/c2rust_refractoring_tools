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
pub unsafe extern "C" fn cblas_ssymm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: i32,
    N: i32,
    alpha: libc::c_float,
    mut A: *const libc::c_float,
    lda: i32,
    mut B: *const libc::c_float,
    ldb: i32,
    beta: libc::c_float,
    mut C: *mut libc::c_float,
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
            b"./source_symm_r.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if alpha as libc::c_double == 0.0f64 && beta as libc::c_double == 1.0f64 {
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
    if beta as libc::c_double == 0.0f64 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) = 0.0f64 as libc::c_float;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if beta as libc::c_double != 1.0f64 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) *= beta;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if side == CblasLeft as i32 && uplo == CblasUpper as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let temp1: libc::c_float = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2: libc::c_float = 0.0f64 as libc::c_float;
                *C.offset((i * ldc + j) as isize)
                    += temp1 * *A.offset((i * lda + i) as isize);
                k = i + 1 as i32;
                while k < n1 {
                    let Aik: libc::c_float = *A.offset((i * lda + k) as isize);
                    *C.offset((k * ldc + j) as isize) += Aik * temp1;
                    temp2 += Aik * *B.offset((ldb * k + j) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp2;
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
                let temp1_0: libc::c_float = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < i {
                    let Aik_0: libc::c_float = *A.offset((i * lda + k) as isize);
                    *C.offset((k * ldc + j) as isize) += Aik_0 * temp1_0;
                    temp2_0 += Aik_0 * *B.offset((ldb * k + j) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize)
                    += temp1_0 * *A.offset((i * lda + i) as isize) + alpha * temp2_0;
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
                let temp1_1: libc::c_float = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2_1: libc::c_float = 0.0f64 as libc::c_float;
                *C.offset((i * ldc + j) as isize)
                    += temp1_1 * *A.offset((j * lda + j) as isize);
                k = j + 1 as i32;
                while k < n2 {
                    let Ajk: libc::c_float = *A.offset((j * lda + k) as isize);
                    *C.offset((i * ldc + k) as isize) += temp1_1 * Ajk;
                    temp2_1 += *B.offset((ldb * i + k) as isize) * Ajk;
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp2_1;
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
                let temp1_2: libc::c_float = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2_2: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < j {
                    let Ajk_0: libc::c_float = *A.offset((j * lda + k) as isize);
                    *C.offset((i * ldc + k) as isize) += temp1_2 * Ajk_0;
                    temp2_2 += *B.offset((ldb * i + k) as isize) * Ajk_0;
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize)
                    += temp1_2 * *A.offset((j * lda + j) as isize) + alpha * temp2_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_symm_r.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}