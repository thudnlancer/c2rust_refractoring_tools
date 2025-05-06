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
pub unsafe extern "C" fn cblas_dsyr2k(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: i32,
    K: i32,
    alpha: libc::c_double,
    mut A: *const libc::c_double,
    lda: i32,
    mut B: *const libc::c_double,
    ldb: i32,
    beta: libc::c_double,
    mut C: *mut libc::c_double,
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
            b"./source_syr2k_r.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
        return;
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        uplo = Uplo as i32;
        trans = (if Trans as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            Trans as u32
        }) as i32;
    } else {
        uplo = if Uplo as u32 == CblasUpper as i32 as u32 {
            CblasLower as i32
        } else {
            CblasUpper as i32
        };
        if Trans as u32 == CblasTrans as i32 as u32
            || Trans as u32 == CblasConjTrans as i32 as u32
        {
            trans = CblasNoTrans as i32;
        } else {
            trans = CblasTrans as i32;
        }
    }
    if beta == 0.0f64 {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                j = i;
                while j < N {
                    *C.offset((ldc * i + j) as isize) = 0.0f64;
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
                    *C.offset((ldc * i + j) as isize) = 0.0f64;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    } else if beta != 1.0f64 {
        if uplo == CblasUpper as i32 {
            i = 0 as i32;
            while i < N {
                j = i;
                while j < N {
                    *C.offset((ldc * i + j) as isize) *= beta;
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
                    *C.offset((ldc * i + j) as isize) *= beta;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    }
    if alpha == 0.0f64 {
        return;
    }
    if uplo == CblasUpper as i32 && trans == CblasNoTrans as i32 {
        i = 0 as i32;
        while i < N {
            j = i;
            while j < N {
                let mut temp: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    temp
                        += *A.offset((i * lda + k) as isize)
                            * *B.offset((j * ldb + k) as isize)
                            + *B.offset((i * ldb + k) as isize)
                                * *A.offset((j * lda + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp;
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
                let mut temp1: libc::c_double = alpha
                    * *A.offset((k * lda + i) as isize);
                let mut temp2: libc::c_double = alpha
                    * *B.offset((k * ldb + i) as isize);
                j = i;
                while j < N {
                    *C.offset((i * lda + j) as isize)
                        += temp1 * *B.offset((k * ldb + j) as isize)
                            + temp2 * *A.offset((k * lda + j) as isize);
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
                let mut temp_0: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    temp_0
                        += *A.offset((i * lda + k) as isize)
                            * *B.offset((j * ldb + k) as isize)
                            + *B.offset((i * ldb + k) as isize)
                                * *A.offset((j * lda + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp_0;
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
                let mut temp1_0: libc::c_double = alpha
                    * *A.offset((k * lda + i) as isize);
                let mut temp2_0: libc::c_double = alpha
                    * *B.offset((k * ldb + i) as isize);
                j = 0 as i32;
                while j <= i {
                    *C.offset((i * lda + j) as isize)
                        += temp1_0 * *B.offset((k * ldb + j) as isize)
                            + temp2_0 * *A.offset((k * lda + j) as isize);
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
            b"./source_syr2k_r.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}