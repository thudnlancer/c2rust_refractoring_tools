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
pub unsafe extern "C" fn cblas_cgerc(
    order: CBLAS_ORDER,
    M: i32,
    N: i32,
    mut alpha: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: i32,
    mut Y: *const libc::c_void,
    incY: i32,
    mut A: *mut libc::c_void,
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
            b"./source_gerc.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as i32 as isize);
    if order as u32 == CblasRowMajor as i32 as u32 {
        let mut ix: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (M - 1 as i32) * -incX
        };
        i = 0 as i32;
        while i < M {
            let X_real: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * ix) as isize);
            let X_imag: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as i32 * ix + 1 as i32) as isize);
            let tmp_real: libc::c_float = alpha_real * X_real - alpha_imag * X_imag;
            let tmp_imag: libc::c_float = alpha_imag * X_real + alpha_real * X_imag;
            let mut jy: i32 = if incY > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incY
            };
            j = 0 as i32;
            while j < N {
                let Y_real: libc::c_float = *(Y as *const libc::c_float)
                    .offset((2 as i32 * jy) as isize);
                let Y_imag: libc::c_float = -*(Y as *const libc::c_float)
                    .offset((2 as i32 * jy + 1 as i32) as isize);
                *(A as *mut libc::c_float).offset((2 as i32 * (lda * i + j)) as isize)
                    += Y_real * tmp_real - Y_imag * tmp_imag;
                *(A as *mut libc::c_float)
                    .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize)
                    += Y_imag * tmp_real + Y_real * tmp_imag;
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
            let Y_real_0: libc::c_float = *(Y as *const libc::c_float)
                .offset((2 as i32 * jy_0) as isize);
            let Y_imag_0: libc::c_float = -*(Y as *const libc::c_float)
                .offset((2 as i32 * jy_0 + 1 as i32) as isize);
            let tmp_real_0: libc::c_float = alpha_real * Y_real_0
                - alpha_imag * Y_imag_0;
            let tmp_imag_0: libc::c_float = alpha_imag * Y_real_0
                + alpha_real * Y_imag_0;
            let mut ix_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (M - 1 as i32) * -incX
            };
            i = 0 as i32;
            while i < M {
                let X_real_0: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as i32 * ix_0) as isize);
                let X_imag_0: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as i32 * ix_0 + 1 as i32) as isize);
                *(A as *mut libc::c_float).offset((2 as i32 * (i + lda * j)) as isize)
                    += X_real_0 * tmp_real_0 - X_imag_0 * tmp_imag_0;
                *(A as *mut libc::c_float)
                    .offset((2 as i32 * (i + lda * j) + 1 as i32) as isize)
                    += X_imag_0 * tmp_real_0 + X_real_0 * tmp_imag_0;
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
            b"./source_gerc.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}