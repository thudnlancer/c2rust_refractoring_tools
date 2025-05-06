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
#[no_mangle]
pub unsafe extern "C" fn cblas_dgemm(
    Order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: i32,
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
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut ldf: i32 = 0;
    let mut ldg: i32 = 0;
    let mut TransF: i32 = 0;
    let mut TransG: i32 = 0;
    let mut F: *const libc::c_double = 0 as *const libc::c_double;
    let mut G: *const libc::c_double = 0 as *const libc::c_double;
    let mut pos: i32 = 0 as i32;
    let mut __transF: CBLAS_TRANSPOSE = CblasNoTrans;
    let mut __transG: CBLAS_TRANSPOSE = CblasNoTrans;
    if Order as u32 == CblasRowMajor as i32 as u32 {
        __transF = (if TransA as u32 != CblasConjTrans as i32 as u32 {
            TransA as u32
        } else {
            CblasTrans as i32 as u32
        }) as CBLAS_TRANSPOSE;
        __transG = (if TransB as u32 != CblasConjTrans as i32 as u32 {
            TransB as u32
        } else {
            CblasTrans as i32 as u32
        }) as CBLAS_TRANSPOSE;
    } else {
        __transF = (if TransB as u32 != CblasConjTrans as i32 as u32 {
            TransB as u32
        } else {
            CblasTrans as i32 as u32
        }) as CBLAS_TRANSPOSE;
        __transG = (if TransA as u32 != CblasConjTrans as i32 as u32 {
            TransA as u32
        } else {
            CblasTrans as i32 as u32
        }) as CBLAS_TRANSPOSE;
    }
    if Order as u32 != CblasRowMajor as i32 as u32
        && Order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if TransA as u32 != CblasNoTrans as i32 as u32
        && TransA as u32 != CblasTrans as i32 as u32
        && TransA as u32 != CblasConjTrans as i32 as u32
    {
        pos = 2 as i32;
    }
    if TransB as u32 != CblasNoTrans as i32 as u32
        && TransB as u32 != CblasTrans as i32 as u32
        && TransB as u32 != CblasConjTrans as i32 as u32
    {
        pos = 3 as i32;
    }
    if M < 0 as i32 {
        pos = 4 as i32;
    }
    if N < 0 as i32 {
        pos = 5 as i32;
    }
    if K < 0 as i32 {
        pos = 6 as i32;
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        if __transF as u32 == CblasNoTrans as i32 as u32 {
            if lda < (if 1 as i32 > K { 1 as i32 } else { K }) {
                pos = 9 as i32;
            }
        } else if lda < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 9 as i32;
        }
        if __transG as u32 == CblasNoTrans as i32 as u32 {
            if ldb < (if 1 as i32 > N { 1 as i32 } else { N }) {
                pos = 11 as i32;
            }
        } else if ldb < (if 1 as i32 > K { 1 as i32 } else { K }) {
            pos = 11 as i32;
        }
        if ldc < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 14 as i32;
        }
    } else if Order as u32 == CblasColMajor as i32 as u32 {
        if __transF as u32 == CblasNoTrans as i32 as u32 {
            if ldb < (if 1 as i32 > K { 1 as i32 } else { K }) {
                pos = 11 as i32;
            }
        } else if ldb < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 11 as i32;
        }
        if __transG as u32 == CblasNoTrans as i32 as u32 {
            if lda < (if 1 as i32 > M { 1 as i32 } else { M }) {
                pos = 9 as i32;
            }
        } else if lda < (if 1 as i32 > K { 1 as i32 } else { K }) {
            pos = 9 as i32;
        }
        if ldc < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 14 as i32;
        }
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_gemm_r.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
        return;
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        n1 = M;
        n2 = N;
        F = A;
        ldf = lda;
        TransF = (if TransA as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            TransA as u32
        }) as i32;
        G = B;
        ldg = ldb;
        TransG = (if TransB as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            TransB as u32
        }) as i32;
    } else {
        n1 = N;
        n2 = M;
        F = B;
        ldf = ldb;
        TransF = (if TransB as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            TransB as u32
        }) as i32;
        G = A;
        ldg = lda;
        TransG = (if TransA as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            TransA as u32
        }) as i32;
    }
    if beta == 0.0f64 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) = 0.0f64;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if beta != 1.0f64 {
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
    if alpha == 0.0f64 {
        return;
    }
    if TransF == CblasNoTrans as i32 && TransG == CblasNoTrans as i32 {
        k = 0 as i32;
        while k < K {
            i = 0 as i32;
            while i < n1 {
                let temp: libc::c_double = alpha * *F.offset((ldf * i + k) as isize);
                if temp != 0.0f64 {
                    j = 0 as i32;
                    while j < n2 {
                        *C.offset((ldc * i + j) as isize)
                            += temp * *G.offset((ldg * k + j) as isize);
                        j += 1;
                        j;
                    }
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else if TransF == CblasNoTrans as i32 && TransG == CblasTrans as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let mut temp_0: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    temp_0
                        += *F.offset((ldf * i + k) as isize)
                            * *G.offset((ldg * j + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((ldc * i + j) as isize) += alpha * temp_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if TransF == CblasTrans as i32 && TransG == CblasNoTrans as i32 {
        k = 0 as i32;
        while k < K {
            i = 0 as i32;
            while i < n1 {
                let temp_1: libc::c_double = alpha * *F.offset((ldf * k + i) as isize);
                if temp_1 != 0.0f64 {
                    j = 0 as i32;
                    while j < n2 {
                        *C.offset((ldc * i + j) as isize)
                            += temp_1 * *G.offset((ldg * k + j) as isize);
                        j += 1;
                        j;
                    }
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else if TransF == CblasTrans as i32 && TransG == CblasTrans as i32 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let mut temp_2: libc::c_double = 0.0f64;
                k = 0 as i32;
                while k < K {
                    temp_2
                        += *F.offset((ldf * k + i) as isize)
                            * *G.offset((ldg * j + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((ldc * i + j) as isize) += alpha * temp_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_gemm_r.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}