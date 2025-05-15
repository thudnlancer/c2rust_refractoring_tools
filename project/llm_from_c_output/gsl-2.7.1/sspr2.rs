use blas_sys::cblas_spr2;
use blas_sys::{CBLAS_ORDER, CBLAS_UPLO};

pub fn cblas_sspr2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: f32,
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
    ap: &mut [f32],
) {
    unsafe {
        cblas_spr2(
            order,
            uplo,
            n,
            alpha,
            x.as_ptr(),
            inc_x,
            y.as_ptr(),
            inc_y,
            ap.as_mut_ptr(),
        );
    }
}