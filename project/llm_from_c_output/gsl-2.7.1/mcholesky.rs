use std::cmp::max;
use std::f64::EPSILON as DBL_EPSILON;
use ndarray::{Array2, Array1, Axis, s, ArrayView1, ArrayViewMut1};
use ndarray_linalg::{cholesky::*, Solve, Inverse, Norm};
use ndarray_permutation::{Permutation, permute_rows_cols, permute_vector_inverse};

/// Modified Cholesky Decomposition
///
/// Factors a symmetric indefinite matrix A as:
/// P (A + E) P^T = L D L^T
///
/// Where:
/// P: permutation matrix
/// E: small, non-negative diagonal matrix
/// L: unit lower triangular matrix
/// D: strictly positive diagonal matrix
pub struct ModifiedCholesky {
    l: Array2<f64>,
    d: Array1<f64>,
    p: Permutation,
    e: Option<Array1<f64>>,
}

impl ModifiedCholesky {
    /// Perform Pivoted Modified Cholesky LDLT decomposition
    pub fn decompose(
        a: Array2<f64>,
        e: Option<Array1<f64>>,
    ) -> Result<Self, &'static str> {
        let n = a.nrows();
        if n != a.ncols() {
            return Err("LDLT decomposition requires square matrix");
        }

        let mut a = a;
        let mut p = Permutation::identity(n);
        let mut e = e;

        let delta = DBL_EPSILON;
        let mut gamma = 0.0;
        let mut xi = 0.0;

        // Save a copy of A in upper triangle (for later rcond calculation)
        let mut a_upper = a.t().to_owned();
        a_upper.diag_mut().fill(1.0);

        // Compute gamma and xi
        for i in 0..n {
            let aii = a[(i, i)].abs();
            gamma = gamma.max(aii);

            for j in 0..i {
                let aij = a[(i, j)].abs();
                xi = xi.max(aij);
            }
        }

        // Compute beta
        let beta = if n == 1 {
            gamma.max(xi).max(DBL_EPSILON).sqrt()
        } else {
            let nu = ((n * n - 1) as f64).sqrt();
            gamma.max(xi / nu).max(DBL_EPSILON).sqrt()
        };

        let mut d = Array1::zeros(n);

        for j in 0..n {
            // Find pivot
            let diag_view = a.slice(s![j.., j..j+1]);
            let (q, _) = maxabs(&diag_view);
            let q = q + j;

            // Swap rows and columns
            p.swap(q, j);
            permute_rows_cols(&mut a, q, j);

            // Compute theta_j
            let thetaj = if j < n - 1 {
                let col_view = a.slice(s![j+1.., j]);
                maxabs(&col_view).1
            } else {
                0.0
            };

            let u = thetaj / beta;
            let ajj = a[(j, j)];
            let alpha = delta.max(ajj.abs()).max(u * u);
            let alphainv = 1.0 / alpha;

            if j < n - 1 {
                let mut v = a.slice_mut(s![j+1.., j]);
                let mut m = a.slice_mut(s![j+1.., j+1..]);

                // m = m - v v^T / alpha
                let v_view = v.view();
                m -= &v_view.outer(&v_view) * alphainv;

                // v = v / alpha
                v *= alphainv;
            }

            if let Some(ref mut e_vec) = e {
                e_vec[j] = alpha - ajj;
            }

            d[j] = alpha;
            a[(j, j)] = alpha;
        }

        if let Some(ref mut e_vec) = e {
            permute_vector_inverse(&mut e_vec.view_mut(), &p);
        }

        Ok(Self {
            l: a,
            d,
            p,
            e,
        })
    }

    /// Solve the system A x = b
    pub fn solve(&self, b: &Array1<f64>) -> Result<Array1<f64>, &'static str> {
        let mut x = b.clone();
        self.solve_in_place(&mut x)?;
        Ok(x)
    }

    /// Solve the system A x = b in place
    pub fn solve_in_place(&self, x: &mut Array1<f64>) -> Result<(), &'static str> {
        // Permute x
        let mut px = x.clone();
        self.p.permute_array(&mut px);

        // Solve L D L^T px = pb
        let l = &self.l;
        let d = &self.d;

        // Forward substitution: L y = pb
        for i in 0..l.nrows() {
            let mut sum = 0.0;
            for j in 0..i {
                sum += l[(i, j)] * px[j];
            }
            px[i] -= sum;
        }

        // Diagonal scaling: D z = y
        for i in 0..px.len() {
            px[i] /= d[i];
        }

        // Backward substitution: L^T x = z
        for i in (0..l.nrows()).rev() {
            let mut sum = 0.0;
            for j in i+1..l.ncols() {
                sum += l[(j, i)] * px[j];
            }
            px[i] -= sum;
        }

        // Unpermute solution
        self.p.inv().permute_array(&mut px);
        x.assign(&px);
        Ok(())
    }

    /// Compute reciprocal condition number estimate
    pub fn rcond(&self, work: &mut Array1<f64>) -> Result<f64, &'static str> {
        // Implementation would use similar approach to GSL's pcholesky_rcond
        // but adapted for ndarray
        unimplemented!()
    }

    /// Compute inverse matrix
    pub fn invert(&self) -> Result<Array2<f64>, &'static str> {
        // Implementation would use similar approach to GSL's pcholesky_invert
        // but adapted for ndarray
        unimplemented!()
    }
}

/// Find index and value of maximum absolute value in vector
fn maxabs(v: &ArrayView1<f64>) -> (usize, f64) {
    let mut max_val = 0.0;
    let mut max_idx = 0;

    for (i, &val) in v.iter().enumerate() {
        let abs_val = val.abs();
        if abs_val > max_val {
            max_val = abs_val;
            max_idx = i;
        }
    }

    (max_idx, max_val)
}