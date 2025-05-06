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
#[no_mangle]
pub unsafe extern "C" fn cblas_dger(
    order: CBLAS_ORDER,
    M: i32,
    N: i32,
    alpha: libc::c_double,
    mut X: *const libc::c_double,
    incX: i32,
    mut Y: *const libc::c_double,
    incY: i32,
    mut A: *mut libc::c_double,
    lda: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0 as i32;
    if order as u32 != CblasRowMajor as i32 as u32
        && order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if M < 0 as i32 {
        pos = 2 as i32;
    }
    if N < 0 as i32 {
        pos = 3 as i32;
    }
    if incX == 0 as i32 {
        pos = 6 as i32;
    }
    if incY == 0 as i32 {
        pos = 8 as i32;
    }
    if order as u32 == CblasRowMajor as i32 as u32 {
        if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 10 as i32;
        }
    } else if order as u32 == CblasColMajor as i32 as u32 {
        if lda < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 10 as i32;
        }
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_ger.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if order as u32 == CblasRowMajor as i32 as u32 {
        let mut ix: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (M - 1 as i32) * -incX
        };
        i = 0 as i32;
        while i < M {
            let tmp: libc::c_double = alpha * *X.offset(ix as isize);
            let mut jy: i32 = if incY > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incY
            };
            j = 0 as i32;
            while j < N {
                *A.offset((lda * i + j) as isize) += *Y.offset(jy as isize) * tmp;
                jy += incY;
                j += 1;
                j;
            }
            ix += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasColMajor as i32 as u32 {
        let mut jy_0: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        j = 0 as i32;
        while j < N {
            let tmp_0: libc::c_double = alpha * *Y.offset(jy_0 as isize);
            let mut ix_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (M - 1 as i32) * -incX
            };
            i = 0 as i32;
            while i < M {
                *A.offset((i + lda * j) as isize) += *X.offset(ix_0 as isize) * tmp_0;
                ix_0 += incX;
                i += 1;
                i;
            }
            jy_0 += incY;
            j += 1;
            j;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_ger.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}