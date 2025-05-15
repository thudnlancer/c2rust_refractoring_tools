use ndarray::{ArrayView1, ArrayView2, Axis};
use num_traits::Zero;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasUplo {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasTranspose {
    NoTrans,
    Trans,
    ConjTrans,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CblasDiag {
    NonUnit,
    Unit,
}

pub fn cblas_dtbmv(
    order: CblasOrder,
    uplo: CblasUplo,
    trans: CblasTranspose,
    diag: CblasDiag,
    n: usize,
    k: usize,
    a: ArrayView2<f64>,
    lda: usize,
    x: ArrayView1<f64>,
    inc_x: usize,
) -> Result<Vec<f64>, String> {
    if n == 0 {
        return Ok(vec![]);
    }

    if k >= n {
        return Err("k must be less than n".to_string());
    }

    if lda < k + 1 {
        return Err("lda must be at least k+1".to_string());
    }

    if inc_x == 0 {
        return Err("inc_x must not be zero".to_string());
    }

    let mut result = x.to_vec();

    match (order, uplo, trans, diag) {
        (CblasOrder::RowMajor, CblasUplo::Upper, CblasTranspose::NoTrans, CblasDiag::NonUnit) => {
            for i in (0..n).rev() {
                let mut temp = result[i * inc_x];
                let start = if i + 1 > k { i + 1 - k } else { 0 };
                for j in start..i {
                    temp += a[[j, k - i + j]] * result[j * inc_x];
                }
                result[i * inc_x] = temp;
            }
        }
        (CblasOrder::RowMajor, CblasUplo::Upper, CblasTranspose::NoTrans, CblasDiag::Unit) => {
            for i in (0..n).rev() {
                let mut temp = result[i * inc_x];
                let start = if i + 1 > k { i + 1 - k } else { 0 };
                for j in start..i {
                    temp += a[[j, k - i + j]] * result[j * inc_x];
                }
                result[i * inc_x] = temp + result[i * inc_x];
            }
        }
        (CblasOrder::RowMajor, CblasUplo::Lower, CblasTranspose::NoTrans, CblasDiag::NonUnit) => {
            for i in 0..n {
                let mut temp = result[i * inc_x];
                let end = if i + k < n { i + k } else { n - 1 };
                for j in i + 1..=end {
                    temp += a[[j, i - j + k]] * result[j * inc_x];
                }
                result[i * inc_x] = temp;
            }
        }
        (CblasOrder::RowMajor, CblasUplo::Lower, CblasTranspose::NoTrans, CblasDiag::Unit) => {
            for i in 0..n {
                let mut temp = result[i * inc_x];
                let end = if i + k < n { i + k } else { n - 1 };
                for j in i + 1..=end {
                    temp += a[[j, i - j + k]] * result[j * inc_x];
                }
                result[i * inc_x] = temp + result[i * inc_x];
            }
        }
        _ => return Err("Unsupported operation combination".to_string()),
    }

    Ok(result)
}