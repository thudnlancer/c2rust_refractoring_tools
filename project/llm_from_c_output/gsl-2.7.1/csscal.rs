pub fn cblas_csscal(n: usize, alpha: f32, x: &mut [f32], inc_x: usize) -> Result<(), &'static str> {
    if inc_x == 0 {
        return Err("Increment must be non-zero");
    }
    if n == 0 {
        return Ok(());
    }
    
    let len = x.len();
    let required_len = (n - 1) * inc_x + 1;
    if len < required_len {
        return Err("Input array too short for given n and inc_x");
    }

    for i in (0..n).map(|i| i * inc_x) {
        x[i] *= alpha;
    }

    Ok(())
}