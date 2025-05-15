use ndarray::{Array1, ArrayView1};
use num_traits::Zero;

pub enum CBLAS_ORDER {
    RowMajor,
    ColMajor,
}

pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

pub fn cblas_sspmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: usize,
    alpha: f32,
    ap: &[f32],
    x: ArrayView1<f32>,
    inc_x: usize,
    beta: f32,
    y: &mut Array1<f32>,
    inc_y: usize,
) -> Result<(), String> {
    if n == 0 {
        return Ok(());
    }

    if ap.len() != n * (n + 1) / 2 {
        return Err("Invalid packed matrix size".to_string());
    }

    if x.len() != ((n - 1) * inc_x + 1) {
        return Err("Invalid x vector size".to_string());
    }

    if y.len() != ((n - 1) * inc_y + 1) {
        return Err("Invalid y vector size".to_string());
    }

    // Compute y = beta * y
    if beta.is_zero() {
        y.fill(0.0);
    } else if beta != 1.0 {
        for elem in y.iter_mut() {
            *elem *= beta;
        }
    }

    // Compute y += alpha * A * x
    match uplo {
        CBLAS_UPLO::Upper => {
            let mut idx = 0;
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = 0.0;
                for j in i..n {
                    let y_idx = j * inc_y;
                    y[y_idx] += alpha * x[i * inc_x] * ap[idx + j - i];
                    if j != i {
                        temp2 += ap[idx + j - i] * x[j * inc_x];
                    }
                }
                y[i * inc_y] += alpha * temp2;
                idx += n - i;
            }
        }
        CBLAS_UPLO::Lower => {
            let mut idx = 0;
            for i in 0..n {
                let mut temp1 = alpha * x[i * inc_x];
                let mut temp2 = 0.0;
                for j in 0..=i {
                    let y_idx = j * inc_y;
                    y[y_idx] += alpha * x[i * inc_x] * ap[idx + i - j];
                    if j != i {
                        temp2 += ap[idx + i - j] * x[j * inc_x];
                    }
                }
                y[i * inc_y] += alpha * temp2;
                idx += i + 1;
            }
        }
    }

    Ok(())
}