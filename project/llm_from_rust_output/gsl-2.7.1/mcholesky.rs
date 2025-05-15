use gsl::{
    blas::CBLAS_UPLO,
    error::{GSLResult, GslError},
    linalg::{Cholesky, Matrix, Permutation, Vector},
    types::{MatrixF64, VectorF64},
};

const DELTA: f64 = 2.2204460492503131e-16;

pub fn mcholesky_decomp(
    a: &mut MatrixF64,
    p: &mut Permutation,
    e: Option<&mut VectorF64>,
) -> GSLResult<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(GslError::NotSquareMatrix);
    }
    if p.size() != n {
        return Err(GslError::InvalidLength);
    }

    let mut gamma = 0.0;
    let mut xi = 0.0;

    // Compute gamma and xi
    for i in 0..n {
        let aii = a.get(i, i).abs();
        gamma = gamma.max(aii);

        for j in 0..i {
            let aij = a.get(i, j).abs();
            xi = xi.max(aij);
        }
    }

    let beta = if n == 1 {
        gamma.max(xi).max(DELTA)
    } else {
        let nu = ((n * n - 1) as f64).sqrt();
        gamma.max(xi / nu).max(DELTA)
    }
    .sqrt();

    p.init();

    for j in 0..n {
        // Find maximum element in subcolumn
        let (q, thetaj) = {
            let mut max_abs = 0.0;
            let mut idx = j;
            
            for i in j..n {
                let abs_val = a.get(i, j).abs();
                if abs_val > max_abs {
                    max_abs = abs_val;
                    idx = i;
                }
            }

            if j < n - 1 {
                let mut theta = 0.0;
                for i in j + 1..n {
                    theta = theta.max(a.get(i, j).abs());
                }
                (idx, theta)
            } else {
                (idx, 0.0)
            }
        };

        p.swap(q, j)?;
        a.swap_rows_cols(q, j)?;

        let u = thetaj / beta;
        let ajj = a.get(j, j);
        let alpha = delta.max(ajj.abs()).max(u * u);
        let alphainv = 1.0 / alpha;

        if j < n - 1 {
            // Update submatrix
            for k in j + 1..n {
                for l in j + 1..=k {
                    let val = a.get(k, l) - alphainv * a.get(k, j) * a.get(l, j);
                    a.set(k, l, val);
                }
            }

            // Scale subcolumn
            for i in j + 1..n {
                let val = a.get(i, j) * alphainv;
                a.set(i, j, val);
            }
        }

        if let Some(e) = e {
            e.set(j, alpha - ajj)?;
        }

        a.set(j, j, alpha);
    }

    if let Some(e) = e {
        p.inv_permute(e)?;
    }

    Ok(())
}

pub fn mcholesky_solve(
    ldlt: &MatrixF64,
    p: &Permutation,
    b: &VectorF64,
    x: &mut VectorF64,
) -> GSLResult<()> {
    Cholesky::pcholesky_solve(ldlt, p, b, x)
}

pub fn mcholesky_svx(ldlt: &MatrixF64, p: &Permutation, x: &mut VectorF64) -> GSLResult<()> {
    Cholesky::pcholesky_svx(ldlt, p, x)
}

pub fn mcholesky_rcond(
    ldlt: &MatrixF64,
    p: &Permutation,
    rcond: &mut f64,
    work: &mut VectorF64,
) -> GSLResult<()> {
    Cholesky::pcholesky_rcond(ldlt, p, rcond, work)
}

pub fn mcholesky_invert(
    ldlt: &MatrixF64,
    p: &Permutation,
    ainv: &mut MatrixF64,
) -> GSLResult<()> {
    Cholesky::pcholesky_invert(ldlt, p, ainv)
}

// Helper functions
trait MatrixExt {
    fn swap_rows_cols(&mut self, i: usize, j: usize) -> GSLResult<()>;
}

impl MatrixExt for MatrixF64 {
    fn swap_rows_cols(&mut self, i: usize, j: usize) -> GSLResult<()> {
        if i != j {
            let n = self.size1();
            
            // Swap elements in rows i and j below diagonal
            for k in 0..i.min(j) {
                let tmp = self.get(i, k);
                self.set(i, k, self.get(j, k))?;
                self.set(j, k, tmp)?;
            }

            // Swap elements between i and j
            for k in i + 1..j {
                let tmp = self.get(k, i);
                self.set(k, i, self.get(j, k))?;
                self.set(j, k, tmp)?;
            }

            // Swap elements above j
            for k in j + 1..n {
                let tmp = self.get(k, i);
                self.set(k, i, self.get(k, j))?;
                self.set(k, j, tmp)?;
            }

            // Swap diagonal elements
            let tmp = self.get(i, i);
            self.set(i, i, self.get(j, j))?;
            self.set(j, j, tmp)?;
        }
        Ok(())
    }
}