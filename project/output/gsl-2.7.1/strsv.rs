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
#[no_mangle]
pub unsafe extern "C" fn cblas_strsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: i32,
    mut A: *const libc::c_float,
    lda: i32,
    mut X: *mut libc::c_float,
    incX: i32,
) {
    let nonunit: i32 = (Diag as u32 == CblasNonUnit as i32 as u32) as i32;
    let mut ix: i32 = 0;
    let mut jx: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let Trans: i32 = (if TransA as u32 != CblasConjTrans as i32 as u32 {
        TransA as u32
    } else {
        CblasTrans as i32 as u32
    }) as i32;
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
    if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
        pos = 7 as i32;
    }
    if incX == 0 as i32 {
        pos = 9 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_trsv_r.h\0" as *const u8 as *const i8,
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
        ix = (if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX })
            + incX * (N - 1 as i32);
        if nonunit != 0 {
            *X.offset(ix as isize) = *X.offset(ix as isize)
                / *A.offset((lda * (N - 1 as i32) + (N - 1 as i32)) as isize);
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
            let mut tmp: libc::c_float = *X.offset(ix as isize);
            jx = ix + incX;
            j = i + 1 as i32;
            while j < N {
                let Aij: libc::c_float = *A.offset((lda * i + j) as isize);
                tmp -= Aij * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp;
            }
            ix -= incX;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasNoTrans as i32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasTrans as i32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        ix = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
        if nonunit != 0 {
            *X.offset(ix as isize) = *X.offset(ix as isize)
                / *A.offset((lda * 0 as i32 + 0 as i32) as isize);
        }
        ix += incX;
        i = 1 as i32;
        while i < N {
            let mut tmp_0: libc::c_float = *X.offset(ix as isize);
            jx = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
            j = 0 as i32;
            while j < i {
                let Aij_0: libc::c_float = *A.offset((lda * i + j) as isize);
                tmp_0 -= Aij_0 * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp_0 / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp_0;
            }
            ix += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        ix = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
        if nonunit != 0 {
            *X.offset(ix as isize) = *X.offset(ix as isize)
                / *A.offset((lda * 0 as i32 + 0 as i32) as isize);
        }
        ix += incX;
        i = 1 as i32;
        while i < N {
            let mut tmp_1: libc::c_float = *X.offset(ix as isize);
            jx = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
            j = 0 as i32;
            while j < i {
                let Aji: libc::c_float = *A.offset((lda * j + i) as isize);
                tmp_1 -= Aji * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp_1 / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp_1;
            }
            ix += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32 && Trans == CblasTrans as i32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32 && Trans == CblasNoTrans as i32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        ix = (if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX })
            + (N - 1 as i32) * incX;
        if nonunit != 0 {
            *X.offset(ix as isize) = *X.offset(ix as isize)
                / *A.offset((lda * (N - 1 as i32) + (N - 1 as i32)) as isize);
        }
        ix -= incX;
        i = N - 1 as i32;
        while i > 0 as i32
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let mut tmp_2: libc::c_float = *X.offset(ix as isize);
            jx = ix + incX;
            j = i + 1 as i32;
            while j < N {
                let Aji_0: libc::c_float = *A.offset((lda * j + i) as isize);
                tmp_2 -= Aji_0 * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp_2 / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp_2;
            }
            ix -= incX;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_trsv_r.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}