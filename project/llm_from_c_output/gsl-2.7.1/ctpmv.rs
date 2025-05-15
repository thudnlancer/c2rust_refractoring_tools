use blas_sys::cblas::{CBLAS_DIAG, CBLAS_ORDER, CBLAS_TRANSPOSE, CBLAS_UPLO};
use num_complex::Complex32;

pub fn cblas_ctpmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    ap: &[Complex32],
    x: &mut [Complex32],
    inc_x: i32,
) -> Result<(), String> {
    if ap.len() < (n * (n + 1) / 2) as usize {
        return Err("Invalid ap length".to_string());
    }
    
    if x.len() < ((n - 1) * inc_x.abs() + 1) as usize {
        return Err("Invalid x length".to_string());
    }

    unsafe {
        blas_sys::cblas_ctpmv(
            order,
            uplo,
            trans_a,
            diag,
            n,
            ap.as_ptr() as *const _,
            x.as_mut_ptr() as *mut _,
            inc_x,
        );
    }

    Ok(())
}