use blas_sys::cblas::{CBLAS_ORDER, CBLAS_SIDE, CBLAS_UPLO};
use num_complex::Complex64;

pub fn cblas_zhemm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    m: i32,
    n: i32,
    alpha: &Complex64,
    a: &[Complex64],
    lda: i32,
    b: &[Complex64],
    ldb: i32,
    beta: &Complex64,
    c: &mut [Complex64],
    ldc: i32,
) -> Result<(), &'static str> {
    // Validate matrix dimensions and strides
    if m < 0 || n < 0 {
        return Err("Invalid matrix dimensions");
    }
    if lda < m.max(1) || ldb < m.max(1) || ldc < m.max(1) {
        return Err("Invalid leading dimension");
    }
    if a.len() < (lda * (side == CBLAS_SIDE::Left ? m : n)) as usize {
        return Err("Matrix A too small");
    }
    if b.len() < (ldb * n) as usize {
        return Err("Matrix B too small");
    }
    if c.len() < (ldc * n) as usize {
        return Err("Matrix C too small");
    }

    // Call the BLAS implementation
    unsafe {
        blas_sys::cblas_zhemm(
            order,
            side,
            uplo,
            m,
            n,
            alpha as *const _ as *const _,
            a.as_ptr() as *const _,
            lda,
            b.as_ptr() as *const _,
            ldb,
            beta as *const _ as *const _,
            c.as_mut_ptr() as *mut _,
            ldc,
        );
    }

    Ok(())
}