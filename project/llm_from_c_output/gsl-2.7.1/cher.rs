use ndarray::{Array2, Array1, ShapeBuilder};
use num_complex::Complex32;
use blas::Layout;

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

pub fn cblas_cher(
    order: Layout,
    uplo: CBLAS_UPLO,
    n: usize,
    alpha: f32,
    x: &Array1<Complex32>,
    inc_x: usize,
    a: &mut Array2<Complex32>,
    lda: usize,
) -> Result<(), &'static str> {
    if x.len() < (n - 1) * inc_x + 1 {
        return Err("x array too small");
    }
    if a.shape()[0] < n || a.shape()[1] < n {
        return Err("a array too small");
    }
    if lda < n {
        return Err("lda must be >= n");
    }

    match order {
        Layout::RowMajor => {
            for i in 0..n {
                let xi = x[i * inc_x];
                let a_ii = a[(i, i)].re + alpha * xi.norm_sqr();
                a[(i, i)] = Complex32::new(a_ii, 0.0);
                
                match uplo {
                    CBLAS_UPLO::Upper => {
                        for j in (i + 1)..n {
                            a[(i, j)] += alpha * xi.conj() * x[j * inc_x];
                        }
                    }
                    CBLAS_UPLO::Lower => {
                        for j in 0..i {
                            a[(i, j)] += alpha * xi.conj() * x[j * inc_x];
                        }
                    }
                }
            }
        }
        Layout::ColumnMajor => {
            for j in 0..n {
                let xj = x[j * inc_x];
                let a_jj = a[(j, j)].re + alpha * xj.norm_sqr();
                a[(j, j)] = Complex32::new(a_jj, 0.0);
                
                match uplo {
                    CBLAS_UPLO::Upper => {
                        for i in 0..j {
                            a[(i, j)] += alpha * x[i * inc_x].conj() * xj;
                        }
                    }
                    CBLAS_UPLO::Lower => {
                        for i in (j + 1)..n {
                            a[(i, j)] += alpha * x[i * inc_x].conj() * xj;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}