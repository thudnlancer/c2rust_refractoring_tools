use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct InvalidInputError;

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input parameters for scnrm2 calculation")
    }
}

impl Error for InvalidInputError {}

pub fn cblas_scnrm2(n: i32, x: &[f32], inc_x: i32) -> Result<f32, InvalidInputError> {
    if n <= 0 || inc_x <= 0 || x.is_empty() {
        return Err(InvalidInputError);
    }

    let mut scale = 0.0;
    let mut ssq = 1.0;

    let actual_length = 1 + (n - 1) * inc_x;
    if actual_length as usize > x.len() {
        return Err(InvalidInputError);
    }

    for i in (0..n as usize).step_by(inc_x as usize) {
        if x[i] != 0.0 {
            let abs_xi = x[i].abs();
            if scale < abs_xi {
                ssq = 1.0 + ssq * (scale / abs_xi).powi(2);
                scale = abs_xi;
            } else {
                ssq = ssq + (abs_xi / scale).powi(2);
            }
        }
    }

    Ok(scale * ssq.sqrt())
}