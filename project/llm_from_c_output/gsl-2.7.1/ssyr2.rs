use ndarray::{ArrayView1, ArrayViewMut2, Axis};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlasError {
    #[error("invalid order")]
    InvalidOrder,
    #[error("invalid uplo")]
    InvalidUplo,
    #[error("invalid dimension")]
    InvalidDimension,
    #[error("invalid increment")]
    InvalidIncrement,
    #[error("invalid leading dimension")]
    InvalidLeadingDimension,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasOrder {
    RowMajor,
    ColMajor,
}

#[derive(Copy, Clone, Debug)]
pub enum CblasUplo {
    Upper,
    Lower,
}

pub fn cblas_ssyr2(
    order: CblasOrder,
    uplo: CblasUplo,
    n: usize,
    alpha: f32,
    x: ArrayView1<f32>,
    incx: isize,
    y: ArrayView1<f32>,
    incy: isize,
    a: &mut ArrayViewMut2<f32>,
) -> Result<(), BlasError> {
    if n == 0 {
        return Ok(());
    }

    if incx == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    if incy == 0 {
        return Err(BlasError::InvalidIncrement);
    }

    let lda = match order {
        CblasOrder::RowMajor => a.len_of(Axis(1)),
        CblasOrder::ColMajor => a.len_of(Axis(0)),
    };

    if lda < n {
        return Err(BlasError::InvalidLeadingDimension);
    }

    let x_len = ((n as isize - 1) * incx.abs() + 1) as usize;
    let y_len = ((n as isize - 1) * incy.abs() + 1) as usize;

    if x.len() < x_len || y.len() < y_len {
        return Err(BlasError::InvalidDimension);
    }

    match (order, uplo) {
        (CblasOrder::RowMajor, CblasUplo::Upper) => {
            for i in 0..n {
                for j in i..n {
                    let xi = x[(i as isize * incx) as usize];
                    let yi = y[(i as isize * incy) as usize];
                    let xj = x[(j as isize * incx) as usize];
                    let yj = y[(j as isize * incy) as usize];
                    a[[i, j]] += alpha * (xi * yj + yi * xj);
                }
            }
        }
        (CblasOrder::RowMajor, CblasUplo::Lower) => {
            for i in 0..n {
                for j in 0..=i {
                    let xi = x[(i as isize * incx) as usize];
                    let yi = y[(i as isize * incy) as usize];
                    let xj = x[(j as isize * incx) as usize];
                    let yj = y[(j as isize * incy) as usize];
                    a[[i, j]] += alpha * (xi * yj + yi * xj);
                }
            }
        }
        (CblasOrder::ColMajor, CblasUplo::Upper) => {
            for j in 0..n {
                for i in 0..=j {
                    let xi = x[(i as isize * incx) as usize];
                    let yi = y[(i as isize * incy) as usize];
                    let xj = x[(j as isize * incx) as usize];
                    let yj = y[(j as isize * incy) as usize];
                    a[[i, j]] += alpha * (xi * yj + yi * xj);
                }
            }
        }
        (CblasOrder::ColMajor, CblasUplo::Lower) => {
            for j in 0..n {
                for i in j..n {
                    let xi = x[(i as isize * incx) as usize];
                    let yi = y[(i as isize * incy) as usize];
                    let xj = x[(j as isize * incx) as usize];
                    let yj = y[(j as isize * incy) as usize];
                    a[[i, j]] += alpha * (xi * yj + yi * xj);
                }
            }
        }
    }

    Ok(())
}