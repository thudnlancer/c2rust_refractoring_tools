use blas_sys::{cblas_chpr2, CBLAS_ORDER, CBLAS_UPLO};
use std::ffi::c_void;

pub fn cblas_chpr2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: &[f32; 2],
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
    ap: &mut [f32],
) {
    unsafe {
        cblas_chpr2(
            order,
            uplo,
            n,
            alpha as *const _ as *const c_void,
            x.as_ptr() as *const c_void,
            inc_x,
            y.as_ptr() as *const c_void,
            inc_y,
            ap.as_mut_ptr() as *mut c_void,
        );
    }
}