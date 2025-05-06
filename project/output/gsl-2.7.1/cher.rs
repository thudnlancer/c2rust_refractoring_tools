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
#[no_mangle]
pub unsafe extern "C" fn cblas_cher(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: i32,
    alpha: libc::c_float,
    mut X: *const libc::c_void,
    incX: i32,
    mut A: *mut libc::c_void,
    lda: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let conj: i32 = if order as u32 == CblasColMajor as i32 as u32 {
        -(1 as i32)
    } else {
        1 as i32
    };
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
    if N < 0 as i32 {
        pos = 3 as i32;
    }
    if incX == 0 as i32 {
        pos = 6 as i32;
    }
    if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
        pos = 8 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_her.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if order as u32 == CblasRowMajor as i32 as u32
        && Uplo as u32 == CblasUpper as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32
            && Uplo as u32 == CblasLower as i32 as u32
    {
        let mut ix: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        i = 0 as i32;
        while i < N {
            let tmp_real: libc::c_float = alpha
                * *(X as *const libc::c_float).offset((2 as i32 * ix) as isize);
            let tmp_imag: libc::c_float = alpha * conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as i32 * ix + 1 as i32) as isize);
            let mut jx: i32 = ix;
            let X_real: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * jx) as isize);
            let X_imag: libc::c_float = -conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as i32 * jx + 1 as i32) as isize);
            *(A as *mut libc::c_float).offset((2 as i32 * (lda * i + i)) as isize)
                += X_real * tmp_real - X_imag * tmp_imag;
            *(A as *mut libc::c_float)
                .offset((2 as i32 * (lda * i + i) + 1 as i32) as isize) = 0 as i32
                as libc::c_float;
            jx += incX;
            j = i + 1 as i32;
            while j < N {
                let X_real_0: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as i32 * jx) as isize);
                let X_imag_0: libc::c_float = -conj as libc::c_float
                    * *(X as *const libc::c_float)
                        .offset((2 as i32 * jx + 1 as i32) as isize);
                *(A as *mut libc::c_float).offset((2 as i32 * (lda * i + j)) as isize)
                    += X_real_0 * tmp_real - X_imag_0 * tmp_imag;
                *(A as *mut libc::c_float)
                    .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize)
                    += X_imag_0 * tmp_real + X_real_0 * tmp_imag;
                jx += incX;
                j += 1;
                j;
            }
            ix += incX;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32
        && Uplo as u32 == CblasLower as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32
            && Uplo as u32 == CblasUpper as i32 as u32
    {
        let mut ix_0: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        i = 0 as i32;
        while i < N {
            let tmp_real_0: libc::c_float = alpha
                * *(X as *const libc::c_float).offset((2 as i32 * ix_0) as isize);
            let tmp_imag_0: libc::c_float = alpha * conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            let mut jx_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < i {
                let X_real_1: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as i32 * jx_0) as isize);
                let X_imag_1: libc::c_float = -conj as libc::c_float
                    * *(X as *const libc::c_float)
                        .offset((2 as i32 * jx_0 + 1 as i32) as isize);
                *(A as *mut libc::c_float).offset((2 as i32 * (lda * i + j)) as isize)
                    += X_real_1 * tmp_real_0 - X_imag_1 * tmp_imag_0;
                *(A as *mut libc::c_float)
                    .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize)
                    += X_imag_1 * tmp_real_0 + X_real_1 * tmp_imag_0;
                jx_0 += incX;
                j += 1;
                j;
            }
            let X_real_2: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * jx_0) as isize);
            let X_imag_2: libc::c_float = -conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as i32 * jx_0 + 1 as i32) as isize);
            *(A as *mut libc::c_float).offset((2 as i32 * (lda * i + i)) as isize)
                += X_real_2 * tmp_real_0 - X_imag_2 * tmp_imag_0;
            *(A as *mut libc::c_float)
                .offset((2 as i32 * (lda * i + i) + 1 as i32) as isize) = 0 as i32
                as libc::c_float;
            jx_0 += incX;
            ix_0 += incX;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_her.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}