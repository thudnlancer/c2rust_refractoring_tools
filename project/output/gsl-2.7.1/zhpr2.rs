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
pub unsafe extern "C" fn cblas_zhpr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: i32,
    mut alpha: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: i32,
    mut Y: *const libc::c_void,
    incY: i32,
    mut Ap: *mut libc::c_void,
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
    if incY == 0 as i32 {
        pos = 8 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_hpr2.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as i32 as isize);
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64 {
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
        let mut iy: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            let Xi_real: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix) as isize);
            let Xi_imag: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix + 1 as i32) as isize);
            let tmp1_real: libc::c_double = alpha_real * Xi_real - alpha_imag * Xi_imag;
            let tmp1_imag: libc::c_double = alpha_imag * Xi_real + alpha_real * Xi_imag;
            let Yi_real: libc::c_double = *(Y as *const libc::c_double)
                .offset((2 as i32 * iy) as isize);
            let Yi_imag: libc::c_double = *(Y as *const libc::c_double)
                .offset((2 as i32 * iy + 1 as i32) as isize);
            let tmp2_real: libc::c_double = alpha_real * Yi_real + alpha_imag * Yi_imag;
            let tmp2_imag: libc::c_double = -alpha_imag * Yi_real + alpha_real * Yi_imag;
            let mut jx: i32 = ix + incX;
            let mut jy: i32 = iy + incY;
            *(Ap as *mut libc::c_double)
                .offset(
                    (2 as i32
                        * ((i - 1 as i32 + 1 as i32) * (2 as i32 * N - (i - 1 as i32))
                            / 2 as i32 + i - i)) as isize,
                )
                += 2 as i32 as libc::c_double
                    * (tmp1_real * Yi_real + tmp1_imag * Yi_imag);
            *(Ap as *mut libc::c_double)
                .offset(
                    (2 as i32
                        * ((i - 1 as i32 + 1 as i32) * (2 as i32 * N - (i - 1 as i32))
                            / 2 as i32 + i - i) + 1 as i32) as isize,
                ) = 0 as i32 as libc::c_double;
            j = i + 1 as i32;
            while j < N {
                let Xj_real: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * jx) as isize);
                let Xj_imag: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * jx + 1 as i32) as isize);
                let Yj_real: libc::c_double = *(Y as *const libc::c_double)
                    .offset((2 as i32 * jy) as isize);
                let Yj_imag: libc::c_double = *(Y as *const libc::c_double)
                    .offset((2 as i32 * jy + 1 as i32) as isize);
                *(Ap as *mut libc::c_double)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + j - i))
                            as isize,
                    )
                    += tmp1_real * Yj_real + tmp1_imag * Yj_imag
                        + (tmp2_real * Xj_real + tmp2_imag * Xj_imag);
                *(Ap as *mut libc::c_double)
                    .offset(
                        (2 as i32
                            * ((i - 1 as i32 + 1 as i32)
                                * (2 as i32 * N - (i - 1 as i32)) / 2 as i32 + j - i)
                            + 1 as i32) as isize,
                    )
                    += conj as libc::c_double
                        * (tmp1_imag * Yj_real - tmp1_real * Yj_imag
                            + (tmp2_imag * Xj_real - tmp2_real * Xj_imag));
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            ix += incX;
            iy += incY;
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
        let mut iy_0: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            let Xi_real_0: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix_0) as isize);
            let Xi_imag_0: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            let tmp1_real_0: libc::c_double = alpha_real * Xi_real_0
                - alpha_imag * Xi_imag_0;
            let tmp1_imag_0: libc::c_double = alpha_imag * Xi_real_0
                + alpha_real * Xi_imag_0;
            let Yi_real_0: libc::c_double = *(Y as *const libc::c_double)
                .offset((2 as i32 * iy_0) as isize);
            let Yi_imag_0: libc::c_double = *(Y as *const libc::c_double)
                .offset((2 as i32 * iy_0 + 1 as i32) as isize);
            let tmp2_real_0: libc::c_double = alpha_real * Yi_real_0
                + alpha_imag * Yi_imag_0;
            let tmp2_imag_0: libc::c_double = -alpha_imag * Yi_real_0
                + alpha_real * Yi_imag_0;
            let mut jx_0: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incX
            };
            let mut jy_0: i32 = if incY > 0 as i32 {
                0 as i32
            } else {
                (N - 1 as i32) * -incY
            };
            j = 0 as i32;
            while j < i {
                let Xj_real_0: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * jx_0) as isize);
                let Xj_imag_0: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * jx_0 + 1 as i32) as isize);
                let Yj_real_0: libc::c_double = *(Y as *const libc::c_double)
                    .offset((2 as i32 * jy_0) as isize);
                let Yj_imag_0: libc::c_double = *(Y as *const libc::c_double)
                    .offset((2 as i32 * jy_0 + 1 as i32) as isize);
                *(Ap as *mut libc::c_double)
                    .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + j)) as isize)
                    += tmp1_real_0 * Yj_real_0 + tmp1_imag_0 * Yj_imag_0
                        + (tmp2_real_0 * Xj_real_0 + tmp2_imag_0 * Xj_imag_0);
                *(Ap as *mut libc::c_double)
                    .offset(
                        (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + j) + 1 as i32)
                            as isize,
                    )
                    += conj as libc::c_double
                        * (tmp1_imag_0 * Yj_real_0 - tmp1_real_0 * Yj_imag_0
                            + (tmp2_imag_0 * Xj_real_0 - tmp2_real_0 * Xj_imag_0));
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            *(Ap as *mut libc::c_double)
                .offset((2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i)) as isize)
                += 2 as i32 as libc::c_double
                    * (tmp1_real_0 * Yi_real_0 + tmp1_imag_0 * Yi_imag_0);
            *(Ap as *mut libc::c_double)
                .offset(
                    (2 as i32 * (i * (i + 1 as i32) / 2 as i32 + i) + 1 as i32) as isize,
                ) = 0 as i32 as libc::c_double;
            ix_0 += incX;
            iy_0 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_hpr2.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}