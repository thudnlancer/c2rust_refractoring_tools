use ndarray::{Array1, ArrayView1};
use blas::{Layout, Transpose, UpLo, Diag};

pub fn cblas_stpsv(
    order: Layout,
    uplo: UpLo,
    trans_a: Transpose,
    diag: Diag,
    n: usize,
    ap: ArrayView1<f32>,
    x: &mut Array1<f32>,
    inc_x: usize,
) -> Result<(), String> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 {
        return Err("incX must not be zero".to_string());
    }

    if ap.len() < n * (n + 1) / 2 {
        return Err("AP length is insufficient".to_string());
    }

    if x.len() < 1 + (n - 1) * inc_x.abs() as usize {
        return Err("X length is insufficient".to_string());
    }

    blas::stpsv(order, uplo, trans_a, diag, n, ap, x, inc_x as i32);
    Ok(())
}