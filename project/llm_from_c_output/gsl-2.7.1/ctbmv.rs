use blas_sys::{cblas_ctbmv, CBLAS_DIAG, CBLAS_ORDER, CBLAS_TRANSPOSE, CBLAS_UPLO};

pub fn cblas_ctbmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    x: &mut [f32],
    inc_x: i32,
) {
    unsafe {
        cblas_ctbmv(
            order,
            uplo,
            trans_a,
            diag,
            n,
            k,
            a.as_ptr() as *const _,
            lda,
            x.as_mut_ptr() as *mut _,
            inc_x,
        );
    }
}