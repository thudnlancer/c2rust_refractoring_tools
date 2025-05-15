use num_complex::Complex64;
use std::error::Error;
use std::result::Result;

pub fn cblas_zdotu_sub(
    n: i32,
    x: &[Complex64],
    inc_x: i32,
    y: &[Complex64],
    inc_y: i32,
    result: &mut Complex64,
) -> Result<(), Box<dyn Error>> {
    if n <= 0 {
        return Ok(());
    }

    if inc_x == 0 || inc_y == 0 {
        return Err("Increments cannot be zero".into());
    }

    if x.len() < ((n - 1) * inc_x.abs() + 1) as usize {
        return Err("X vector too short".into());
    }

    if y.len() < ((n - 1) * inc_y.abs() + 1) as usize {
        return Err("Y vector too short".into());
    }

    let mut temp = Complex64::new(0.0, 0.0);

    let x_step = if inc_x > 0 { inc_x as usize } else { (-inc_x) as usize };
    let y_step = if inc_y > 0 { inc_y as usize } else { (-inc_y) as usize };

    let x_start = if inc_x > 0 { 0 } else { (1 - n) * inc_x };
    let y_start = if inc_y > 0 { 0 } else { (1 - n) * inc_y };

    for i in 0..n as usize {
        let x_idx = (x_start + i as i32 * inc_x) as usize;
        let y_idx = (y_start + i as i32 * inc_y) as usize;
        
        temp += x[x_idx] * y[y_idx];
    }

    *result = temp;
    Ok(())
}