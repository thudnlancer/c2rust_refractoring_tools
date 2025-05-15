use blas_sys::{cblas_sstrmv, CBLAS_DIAG, CBLAS_ORDER, CBLAS_TRANSPOSE, CBLAS_UPLO};

pub fn cblas_strmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    a: &[f32],
    lda: i32,
    x: &mut [f32],
    inc_x: i32,
) -> Result<(), &'static str> {
    if a.len() < (n * lda) as usize {
        return Err("Matrix A is too small for the specified dimensions");
    }
    if x.len() < ((n - 1) * inc_x.abs() + 1) as usize {
        return Err("Vector X is too small for the specified dimensions");
    }

    unsafe {
        cblas_sstrmv(
            order,
            uplo,
            trans_a,
            diag,
            n,
            a.as_ptr(),
            lda,
            x.as_mut_ptr(),
            inc_x,
        );
    }

    Ok(())
}