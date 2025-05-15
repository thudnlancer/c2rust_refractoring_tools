use blas_sys::c::{CBLAS_ORDER, CBLAS_SIDE, CBLAS_UPLO, CBLAS_TRANSPOSE, CBLAS_DIAG};

pub fn cblas_strmm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    transa: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    m: i32,
    n: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> Result<(), &'static str> {
    // Validate matrix dimensions and strides
    if m < 0 || n < 0 {
        return Err("Invalid matrix dimensions");
    }
    if lda < 1.max(if side == CBLAS_SIDE::Left { m } else { n }) {
        return Err("Invalid leading dimension for A");
    }
    if ldb < 1.max(n) {
        return Err("Invalid leading dimension for B");
    }

    // Validate array lengths
    let a_len = match side {
        CBLAS_SIDE::Left => (m as usize) * (lda as usize),
        CBLAS_SIDE::Right => (n as usize) * (lda as usize),
    };
    if a.len() < a_len {
        return Err("A array too small");
    }

    let b_len = (m as usize) * (ldb as usize);
    if b.len() < b_len {
        return Err("B array too small");
    }

    // Call the BLAS implementation
    unsafe {
        blas_sys::c::cblas_strmm(
            order,
            side,
            uplo,
            transa,
            diag,
            m,
            n,
            alpha,
            a.as_ptr(),
            lda,
            b.as_mut_ptr(),
            ldb,
        );
    }

    Ok(())
}