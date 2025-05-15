use num_complex::Complex64;
use std::error::Error;
use std::result::Result;

pub fn cblas_zscal(
    n: usize,
    alpha: &Complex64,
    x: &mut [Complex64],
    inc_x: usize,
) -> Result<(), Box<dyn Error>> {
    if inc_x == 0 {
        return Err("inc_x must be positive".into());
    }
    
    if n == 0 {
        return Ok(());
    }
    
    let len = x.len();
    let required_len = (n - 1) * inc_x + 1;
    if len < required_len {
        return Err(format!(
            "x length {} is less than required length {}",
            len, required_len
        ).into());
    }
    
    for i in (0..n).map(|i| i * inc_x) {
        x[i] *= alpha;
    }
    
    Ok(())
}