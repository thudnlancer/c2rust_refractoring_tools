use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CBlasError {
    message: String,
}

impl fmt::Display for CBlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CBLAS error: {}", self.message)
    }
}

impl Error for CBlasError {}

fn cblas_ccopy(
    n: i32,
    x: &[f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
) -> Result<(), Box<dyn Error>> {
    if n <= 0 {
        return Ok(());
    }

    if inc_x == 0 || inc_y == 0 {
        return Err(Box::new(CBlasError {
            message: "Increment cannot be zero".to_string(),
        }));
    }

    if x.len() < ((n - 1) * inc_x.abs() + 1) as usize {
        return Err(Box::new(CBlasError {
            message: "X vector too short".to_string(),
        }));
    }

    if y.len() < ((n - 1) * inc_y.abs() + 1) as usize {
        return Err(Box::new(CBlasError {
            message: "Y vector too short".to_string(),
        }));
    }

    let x_step = if inc_x > 0 { inc_x as usize } else { (-inc_x) as usize };
    let y_step = if inc_y > 0 { inc_y as usize } else { (-inc_y) as usize };

    let x_start = if inc_x > 0 { 0 } else { (1 - n) * inc_x as usize };
    let y_start = if inc_y > 0 { 0 } else { (1 - n) * inc_y as usize };

    for i in 0..n as usize {
        let x_idx = x_start + i * x_step;
        let y_idx = y_start + i * y_step;
        
        if x_idx >= x.len() || y_idx >= y.len() {
            return Err(Box::new(CBlasError {
                message: "Index out of bounds".to_string(),
            }));
        }
        
        y[y_idx] = x[x_idx];
    }

    Ok(())
}