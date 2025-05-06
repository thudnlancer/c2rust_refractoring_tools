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
pub unsafe extern "C" fn cblas_cgemm(
    Order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: i32,
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
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut ldf: i32 = 0;
    let mut ldg: i32 = 0;
    let mut conjF: i32 = 0;
    let mut conjG: i32 = 0;
    let mut TransF: i32 = 0;
    let mut TransG: i32 = 0;
    let mut F: *const libc::c_float = 0 as *const libc::c_float;
    let mut G: *const libc::c_float = 0 as *const libc::c_float;
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
            b"./source_gemm_c.h\0" as *const u8 as *const i8,
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
        n1 = M;
        n2 = N;
        F = A as *const libc::c_float;
        ldf = lda;
        conjF = if TransA as u32 == CblasConjTrans as i32 as u32 {
            -(1 as i32)
        } else {
            1 as i32
        };
        TransF = if TransA as u32 == CblasNoTrans as i32 as u32 {
            CblasNoTrans as i32
        } else {
            CblasTrans as i32
        };
        G = B as *const libc::c_float;
        ldg = ldb;
        conjG = if TransB as u32 == CblasConjTrans as i32 as u32 {
            -(1 as i32)
        } else {
            1 as i32
        };
        TransG = if TransB as u32 == CblasNoTrans as i32 as u32 {
            CblasNoTrans as i32
        } else {
            CblasTrans as i32
        };
    } else {
        n1 = N;
        n2 = M;
        F = B as *const libc::c_float;
        ldf = ldb;
        conjF = if TransB as u32 == CblasConjTrans as i32 as u32 {
            -(1 as i32)
        } else {
            1 as i32
        };
        TransF = if TransB as u32 == CblasNoTrans as i32 as u32 {
            CblasNoTrans as i32
        } else {
            CblasTrans as i32
        };
        G = A as *const libc::c_float;
        ldg = lda;
        conjG = if TransA as u32 == CblasConjTrans as i32 as u32 {
            -(1 as i32)
        } else {
            1 as i32
        };
        TransG = if TransA as u32 == CblasNoTrans as i32 as u32 {
            CblasNoTrans as i32
        } else {
            CblasTrans as i32
        };
    }
    if beta_real as libc::c_double == 0.0f64 && beta_imag as libc::c_double == 0.0f64 {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                *(C as *mut libc::c_float).offset((2 as i32 * (ldc * i + j)) as isize) = 0.0f64
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
    } else if !(beta_real as libc::c_double == 1.0f64
        && beta_imag as libc::c_double == 0.0f64)
    {
        i = 0 as i32;
        while i < n1 {
            j = 0 as i32;
            while j < n2 {
                let Cij_real: libc::c_float = *(C as *mut libc::c_float)
                    .offset((2 as i32 * (ldc * i + j)) as isize);
                let Cij_imag: libc::c_float = *(C as *mut libc::c_float)
                    .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize);
                *(C as *mut libc::c_float).offset((2 as i32 * (ldc * i + j)) as isize) = beta_real
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
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
        return;
    }
    if TransF == CblasNoTrans as i32 && TransG == CblasNoTrans as i32 {
        k = 0 as i32;
        while k < K {
            i = 0 as i32;
            while i < n1 {
                let Fik_real: libc::c_float = *F
                    .offset((2 as i32 * (ldf * i + k)) as isize);
                let Fik_imag: libc::c_float = conjF as libc::c_float
                    * *F.offset((2 as i32 * (ldf * i + k) + 1 as i32) as isize);
                let temp_real: libc::c_float = alpha_real * Fik_real
                    - alpha_imag * Fik_imag;
                let temp_imag: libc::c_float = alpha_real * Fik_imag
                    + alpha_imag * Fik_real;
                if !(temp_real as libc::c_double == 0.0f64
                    && temp_imag as libc::c_double == 0.0f64)
                {
                    j = 0 as i32;
                    while j < n2 {
                        let Gkj_real: libc::c_float = *G
                            .offset((2 as i32 * (ldg * k + j)) as isize);
                        let Gkj_imag: libc::c_float = conjG as libc::c_float
                            * *G.offset((2 as i32 * (ldg * k + j) + 1 as i32) as isize);
                        *(C as *mut libc::c_float)
                            .offset((2 as i32 * (ldc * i + j)) as isize)
                            += temp_real * Gkj_real - temp_imag * Gkj_imag;
                        *(C as *mut libc::c_float)
                            .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize)
                            += temp_real * Gkj_imag + temp_imag * Gkj_real;
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
                let mut temp_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < K {
                    let Fik_real_0: libc::c_float = *F
                        .offset((2 as i32 * (ldf * i + k)) as isize);
                    let Fik_imag_0: libc::c_float = conjF as libc::c_float
                        * *F.offset((2 as i32 * (ldf * i + k) + 1 as i32) as isize);
                    let Gjk_real: libc::c_float = *G
                        .offset((2 as i32 * (ldg * j + k)) as isize);
                    let Gjk_imag: libc::c_float = conjG as libc::c_float
                        * *G.offset((2 as i32 * (ldg * j + k) + 1 as i32) as isize);
                    temp_real_0 += Fik_real_0 * Gjk_real - Fik_imag_0 * Gjk_imag;
                    temp_imag_0 += Fik_real_0 * Gjk_imag + Fik_imag_0 * Gjk_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (ldc * i + j)) as isize)
                    += alpha_real * temp_real_0 - alpha_imag * temp_imag_0;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize)
                    += alpha_real * temp_imag_0 + alpha_imag * temp_real_0;
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
                let Fki_real: libc::c_float = *F
                    .offset((2 as i32 * (ldf * k + i)) as isize);
                let Fki_imag: libc::c_float = conjF as libc::c_float
                    * *F.offset((2 as i32 * (ldf * k + i) + 1 as i32) as isize);
                let temp_real_1: libc::c_float = alpha_real * Fki_real
                    - alpha_imag * Fki_imag;
                let temp_imag_1: libc::c_float = alpha_real * Fki_imag
                    + alpha_imag * Fki_real;
                if !(temp_real_1 as libc::c_double == 0.0f64
                    && temp_imag_1 as libc::c_double == 0.0f64)
                {
                    j = 0 as i32;
                    while j < n2 {
                        let Gkj_real_0: libc::c_float = *G
                            .offset((2 as i32 * (ldg * k + j)) as isize);
                        let Gkj_imag_0: libc::c_float = conjG as libc::c_float
                            * *G.offset((2 as i32 * (ldg * k + j) + 1 as i32) as isize);
                        *(C as *mut libc::c_float)
                            .offset((2 as i32 * (ldc * i + j)) as isize)
                            += temp_real_1 * Gkj_real_0 - temp_imag_1 * Gkj_imag_0;
                        *(C as *mut libc::c_float)
                            .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize)
                            += temp_real_1 * Gkj_imag_0 + temp_imag_1 * Gkj_real_0;
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
                let mut temp_real_2: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_2: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as i32;
                while k < K {
                    let Fki_real_0: libc::c_float = *F
                        .offset((2 as i32 * (ldf * k + i)) as isize);
                    let Fki_imag_0: libc::c_float = conjF as libc::c_float
                        * *F.offset((2 as i32 * (ldf * k + i) + 1 as i32) as isize);
                    let Gjk_real_0: libc::c_float = *G
                        .offset((2 as i32 * (ldg * j + k)) as isize);
                    let Gjk_imag_0: libc::c_float = conjG as libc::c_float
                        * *G.offset((2 as i32 * (ldg * j + k) + 1 as i32) as isize);
                    temp_real_2 += Fki_real_0 * Gjk_real_0 - Fki_imag_0 * Gjk_imag_0;
                    temp_imag_2 += Fki_real_0 * Gjk_imag_0 + Fki_imag_0 * Gjk_real_0;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float).offset((2 as i32 * (ldc * i + j)) as isize)
                    += alpha_real * temp_real_2 - alpha_imag * temp_imag_2;
                *(C as *mut libc::c_float)
                    .offset((2 as i32 * (ldc * i + j) + 1 as i32) as isize)
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
            b"./source_gemm_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}