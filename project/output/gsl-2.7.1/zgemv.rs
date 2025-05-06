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
pub unsafe extern "C" fn cblas_zgemv(
    order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    M: i32,
    N: i32,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: i32,
    mut X: *const libc::c_void,
    incX: i32,
    mut beta: *const libc::c_void,
    mut Y: *mut libc::c_void,
    incY: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut lenX: i32 = 0;
    let mut lenY: i32 = 0;
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as i32 as isize);
    let beta_real: libc::c_double = *(beta as *const libc::c_double)
        .offset(0 as i32 as isize);
    let beta_imag: libc::c_double = *(beta as *const libc::c_double)
        .offset(1 as i32 as isize);
    let mut pos: i32 = 0 as i32;
    if order as u32 != CblasRowMajor as i32 as u32
        && order as u32 != CblasColMajor as i32 as u32
    {
        pos = 1 as i32;
    }
    if TransA as u32 != CblasNoTrans as i32 as u32
        && TransA as u32 != CblasTrans as i32 as u32
        && TransA as u32 != CblasConjTrans as i32 as u32
    {
        pos = 2 as i32;
    }
    if M < 0 as i32 {
        pos = 3 as i32;
    }
    if N < 0 as i32 {
        pos = 4 as i32;
    }
    if order as u32 == CblasRowMajor as i32 as u32 {
        if lda < (if 1 as i32 > N { 1 as i32 } else { N }) {
            pos = 7 as i32;
        }
    } else if order as u32 == CblasColMajor as i32 as u32 {
        if lda < (if 1 as i32 > M { 1 as i32 } else { M }) {
            pos = 7 as i32;
        }
    }
    if incX == 0 as i32 {
        pos = 9 as i32;
    }
    if incY == 0 as i32 {
        pos = 12 as i32;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_gemv_c.h\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    if M == 0 as i32 || N == 0 as i32 {
        return;
    }
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64
        && (beta_real == 1.0f64 && beta_imag == 0.0f64)
    {
        return;
    }
    if TransA as u32 == CblasNoTrans as i32 as u32 {
        lenX = N;
        lenY = M;
    } else {
        lenX = M;
        lenY = N;
    }
    if beta_real == 0.0f64 && beta_imag == 0.0f64 {
        let mut iy: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            *(Y as *mut libc::c_double).offset((2 as i32 * iy) as isize) = 0.0f64;
            *(Y as *mut libc::c_double).offset((2 as i32 * iy + 1 as i32) as isize) = 0.0f64;
            iy += incY;
            i += 1;
            i;
        }
    } else if !(beta_real == 1.0f64 && beta_imag == 0.0f64) {
        let mut iy_0: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            let y_real: libc::c_double = *(Y as *mut libc::c_double)
                .offset((2 as i32 * iy_0) as isize);
            let y_imag: libc::c_double = *(Y as *mut libc::c_double)
                .offset((2 as i32 * iy_0 + 1 as i32) as isize);
            let tmpR: libc::c_double = y_real * beta_real - y_imag * beta_imag;
            let tmpI: libc::c_double = y_real * beta_imag + y_imag * beta_real;
            *(Y as *mut libc::c_double).offset((2 as i32 * iy_0) as isize) = tmpR;
            *(Y as *mut libc::c_double).offset((2 as i32 * iy_0 + 1 as i32) as isize) = tmpI;
            iy_0 += incY;
            i += 1;
            i;
        }
    }
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64 {
        return;
    }
    if order as u32 == CblasRowMajor as i32 as u32
        && TransA as u32 == CblasNoTrans as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32
            && TransA as u32 == CblasTrans as i32 as u32
    {
        let mut iy_1: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            let mut dotR: libc::c_double = 0.0f64;
            let mut dotI: libc::c_double = 0.0f64;
            let mut ix: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (lenX - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < lenX {
                let x_real: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * ix) as isize);
                let x_imag: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * ix + 1 as i32) as isize);
                let A_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + j)) as isize);
                let A_imag: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize);
                dotR += A_real * x_real - A_imag * x_imag;
                dotI += A_real * x_imag + A_imag * x_real;
                ix += incX;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_double).offset((2 as i32 * iy_1) as isize)
                += alpha_real * dotR - alpha_imag * dotI;
            *(Y as *mut libc::c_double).offset((2 as i32 * iy_1 + 1 as i32) as isize)
                += alpha_real * dotI + alpha_imag * dotR;
            iy_1 += incY;
            i += 1;
            i;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32
        && TransA as u32 == CblasTrans as i32 as u32
        || order as u32 == CblasColMajor as i32 as u32
            && TransA as u32 == CblasNoTrans as i32 as u32
    {
        let mut ix_0: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (lenX - 1 as i32) * -incX
        };
        j = 0 as i32;
        while j < lenX {
            let mut x_real_0: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix_0) as isize);
            let mut x_imag_0: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix_0 + 1 as i32) as isize);
            let mut tmpR_0: libc::c_double = alpha_real * x_real_0
                - alpha_imag * x_imag_0;
            let mut tmpI_0: libc::c_double = alpha_real * x_imag_0
                + alpha_imag * x_real_0;
            let mut iy_2: i32 = if incY > 0 as i32 {
                0 as i32
            } else {
                (lenY - 1 as i32) * -incY
            };
            i = 0 as i32;
            while i < lenY {
                let A_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * j + i)) as isize);
                let A_imag_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * j + i) + 1 as i32) as isize);
                *(Y as *mut libc::c_double).offset((2 as i32 * iy_2) as isize)
                    += A_real_0 * tmpR_0 - A_imag_0 * tmpI_0;
                *(Y as *mut libc::c_double).offset((2 as i32 * iy_2 + 1 as i32) as isize)
                    += A_real_0 * tmpI_0 + A_imag_0 * tmpR_0;
                iy_2 += incY;
                i += 1;
                i;
            }
            ix_0 += incX;
            j += 1;
            j;
        }
    } else if order as u32 == CblasRowMajor as i32 as u32
        && TransA as u32 == CblasConjTrans as i32 as u32
    {
        let mut ix_1: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (lenX - 1 as i32) * -incX
        };
        j = 0 as i32;
        while j < lenX {
            let mut x_real_1: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix_1) as isize);
            let mut x_imag_1: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as i32 * ix_1 + 1 as i32) as isize);
            let mut tmpR_1: libc::c_double = alpha_real * x_real_1
                - alpha_imag * x_imag_1;
            let mut tmpI_1: libc::c_double = alpha_real * x_imag_1
                + alpha_imag * x_real_1;
            let mut iy_3: i32 = if incY > 0 as i32 {
                0 as i32
            } else {
                (lenY - 1 as i32) * -incY
            };
            i = 0 as i32;
            while i < lenY {
                let A_real_1: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * j + i)) as isize);
                let A_imag_1: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * j + i) + 1 as i32) as isize);
                *(Y as *mut libc::c_double).offset((2 as i32 * iy_3) as isize)
                    += A_real_1 * tmpR_1 - -A_imag_1 * tmpI_1;
                *(Y as *mut libc::c_double).offset((2 as i32 * iy_3 + 1 as i32) as isize)
                    += A_real_1 * tmpI_1 + -A_imag_1 * tmpR_1;
                iy_3 += incY;
                i += 1;
                i;
            }
            ix_1 += incX;
            j += 1;
            j;
        }
    } else if order as u32 == CblasColMajor as i32 as u32
        && TransA as u32 == CblasConjTrans as i32 as u32
    {
        let mut iy_4: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (lenY - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < lenY {
            let mut dotR_0: libc::c_double = 0.0f64;
            let mut dotI_0: libc::c_double = 0.0f64;
            let mut ix_2: i32 = if incX > 0 as i32 {
                0 as i32
            } else {
                (lenX - 1 as i32) * -incX
            };
            j = 0 as i32;
            while j < lenX {
                let x_real_2: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * ix_2) as isize);
                let x_imag_2: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as i32 * ix_2 + 1 as i32) as isize);
                let A_real_2: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + j)) as isize);
                let A_imag_2: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as i32 * (lda * i + j) + 1 as i32) as isize);
                dotR_0 += A_real_2 * x_real_2 - -A_imag_2 * x_imag_2;
                dotI_0 += A_real_2 * x_imag_2 + -A_imag_2 * x_real_2;
                ix_2 += incX;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_double).offset((2 as i32 * iy_4) as isize)
                += alpha_real * dotR_0 - alpha_imag * dotI_0;
            *(Y as *mut libc::c_double).offset((2 as i32 * iy_4 + 1 as i32) as isize)
                += alpha_real * dotI_0 + alpha_imag * dotR_0;
            iy_4 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_gemv_c.h\0" as *const u8 as *const i8,
            b"unrecognized operation\0" as *const u8 as *const i8,
        );
    };
}