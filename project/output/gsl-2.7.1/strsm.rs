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
pub type CBLAS_SIDE = u32;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
#[no_mangle]
pub unsafe extern "C" fn cblas_strsm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: i32,
    N: i32,
    alpha: libc::c_float,
    mut A: *const libc::c_float,
    lda: i32,
    mut B: *mut libc::c_float,
    ldb: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let nonunit: i32 = (Diag as u32 == CblasNonUnit as i32 as u32) as i32;
    let mut side: i32 = 0;
    let mut uplo: i32 = 0;
    let mut trans: i32 = 0;
    let mut pos: i32 = 0 as i32;
    let mut __dim: i32 = 0 as i32;
    if Side as u32 == CblasLeft as i32 as u32 {
        __dim = M;
    } else {
        __dim = N;
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
    if TransA as u32 != CblasNoTrans as i32 as u32
        && TransA as u32 != CblasTrans as i32 as u32
        && TransA as u32 != CblasConjTrans as i32 as u32
    {
        pos = 4 as i32;
    }
    if Diag as u32 != CblasNonUnit as i32 as u32
        && Diag as u32 != CblasUnit as i32 as u32
    {
        pos = 5 as i32;
    }
    if M < 0 as i32 {
        pos = 6 as i32;
    }
    if N < 0 as i32 {
        pos = 7 as i32;
    }
    if lda < (if 1 as i32 > __dim { 1 as i32 } else { __dim }) {
        pos = 10 as i32;
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        if ldb < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 12 as i32;
        }
    } else if ldb < (if 1 as i32 > M { 1 as i32 } else { M }) {
        pos = 12 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_trsm_r.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if Order as u32 == CblasRowMajor as i32 as u32 {
        n1 = M;
        n2 = N;
        side = Side as i32;
        uplo = Uplo as i32;
        trans = (if TransA as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            TransA as u32
        }) as i32;
    } else {
        n1 = N;
        n2 = M;
        side = if Side as u32 == CblasLeft as i32 as u32 {
            CblasRight as i32
        } else {
            CblasLeft as i32
        };
        uplo = if Uplo as u32 == CblasUpper as i32 as u32 {
            CblasLower as i32
        } else {
            CblasUpper as i32
        };
        trans = (if TransA as u32 == CblasConjTrans as i32 as u32 {
            CblasTrans as i32 as u32
        } else {
            TransA as u32
        }) as i32;
    }
    if side == CblasLeft as i32 && uplo == CblasUpper as i32
        && trans == CblasNoTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = n1;
        while i > 0 as i32
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            if nonunit != 0 {
                let mut Aii: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii;
                    j += 1;
                    j;
                }
            }
            k = 0 as i32;
            while k < i {
                let Aki: libc::c_float = *A.offset((k * lda + i) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aki * *B.offset((ldb * i + j) as isize);
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
        }
    } else if side == CblasLeft as i32 && uplo == CblasUpper as i32
        && trans == CblasTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n1 {
            if nonunit != 0 {
                let mut Aii_0: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii_0;
                    j += 1;
                    j;
                }
            }
            k = i + 1 as i32;
            while k < n1 {
                let Aik: libc::c_float = *A.offset((i * lda + k) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aik * *B.offset((ldb * i + j) as isize);
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as i32 && uplo == CblasLower as i32
        && trans == CblasNoTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n1 {
            if nonunit != 0 {
                let mut Aii_1: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii_1;
                    j += 1;
                    j;
                }
            }
            k = i + 1 as i32;
            while k < n1 {
                let Aki_0: libc::c_float = *A.offset((k * lda + i) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aki_0 * *B.offset((ldb * i + j) as isize);
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as i32 && uplo == CblasLower as i32
        && trans == CblasTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = n1;
        while i > 0 as i32
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            if nonunit != 0 {
                let mut Aii_2: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii_2;
                    j += 1;
                    j;
                }
            }
            k = 0 as i32;
            while k < i {
                let Aik_0: libc::c_float = *A.offset((i * lda + k) as isize);
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aik_0 * *B.offset((ldb * i + j) as isize);
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
        }
    } else if side == CblasRight as i32 && uplo == CblasUpper as i32
        && trans == CblasNoTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                if nonunit != 0 {
                    let mut Ajj: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj;
                }
                let mut Bij: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = j + 1 as i32;
                while k < n2 {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((j * lda + k) as isize) * Bij;
                    k += 1;
                    k;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as i32 && uplo == CblasUpper as i32
        && trans == CblasTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n1 {
            j = n2;
            while j > 0 as i32
                && {
                    let fresh2 = j;
                    j = j - 1;
                    fresh2 != 0
                }
            {
                if nonunit != 0 {
                    let mut Ajj_0: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj_0;
                }
                let mut Bij_0: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = 0 as i32;
                while k < j {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((k * lda + j) as isize) * Bij_0;
                    k += 1;
                    k;
                }
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as i32 && uplo == CblasLower as i32
        && trans == CblasNoTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n1 {
            j = n2;
            while j > 0 as i32
                && {
                    let fresh3 = j;
                    j = j - 1;
                    fresh3 != 0
                }
            {
                if nonunit != 0 {
                    let mut Ajj_1: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj_1;
                }
                let mut Bij_1: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = 0 as i32;
                while k < j {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((j * lda + k) as isize) * Bij_1;
                    k += 1;
                    k;
                }
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as i32 && uplo == CblasLower as i32
        && trans == CblasTrans as i32
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as i32;
            while i < n1 {
                j = 0 as i32;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                if nonunit != 0 {
                    let mut Ajj_2: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj_2;
                }
                let mut Bij_2: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = j + 1 as i32;
                while k < n2 {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((k * lda + j) as isize) * Bij_2;
                    k += 1;
                    k;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_trsm_r.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}