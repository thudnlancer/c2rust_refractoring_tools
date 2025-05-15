use std::cmp::min;
use std::f64;
use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, Axis};
use ndarray_linalg::{Solve, Determinant, LU, Scalar};
use num_traits::{Zero, One, Signed};

const CROSSOVER_LU: usize = 32;

pub struct LUDecomposition {
    pub lu: Array2<f64>,
    pub pivots: Array1<usize>,
    pub signum: i32,
}

impl LUDecomposition {
    pub fn new(a: Array2<f64>) -> Result<Self, &'static str> {
        let (m, n) = a.dim();
        if m != n {
            return Err("Matrix must be square");
        }

        let mut lu = a.clone();
        let mut pivots = Array1::zeros(m);
        let mut signum = 1;

        // Perform LU decomposition with partial pivoting
        for j in 0..m {
            // Find pivot
            let mut max_idx = j;
            let mut max_val = lu[(j, j)].abs();
            for i in j + 1..m {
                let val = lu[(i, j)].abs();
                if val > max_val {
                    max_val = val;
                    max_idx = i;
                }
            }

            pivots[j] = max_idx;

            // Swap rows if necessary
            if max_idx != j {
                lu.swap_rows(j, max_idx);
                signum *= -1;
            }

            // Check for singularity
            if lu[(j, j)] == 0.0 {
                return Err("Matrix is singular");
            }

            // Compute elements of L and U
            for i in j + 1..m {
                lu[(i, j)] /= lu[(j, j)];
                for k in j + 1..m {
                    lu[(i, k)] -= lu[(i, j)] * lu[(j, k)];
                }
            }
        }

        Ok(Self { lu, pivots, signum })
    }

    pub fn solve(&self, b: &Array1<f64>) -> Result<Array1<f64>, &'static str> {
        let n = self.lu.dim().0;
        if b.len() != n {
            return Err("Vector size must match matrix size");
        }

        let mut x = b.clone();

        // Apply permutations
        for i in 0..n {
            x.swap(i, self.pivots[i]);
        }

        // Forward substitution (L y = P b)
        for i in 0..n {
            for k in 0..i {
                x[i] -= self.lu[(i, k)] * x[k];
            }
        }

        // Back substitution (U x = y)
        for i in (0..n).rev() {
            for k in i + 1..n {
                x[i] -= self.lu[(i, k)] * x[k];
            }
            x[i] /= self.lu[(i, i)];
        }

        Ok(x)
    }

    pub fn det(&self) -> f64 {
        let n = self.lu.dim().0;
        let mut det = self.signum as f64;
        for i in 0..n {
            det *= self.lu[(i, i)];
        }
        det
    }

    pub fn inv(&self) -> Result<Array2<f64>, &'static str> {
        let n = self.lu.dim().0;
        let mut inv = Array2::zeros((n, n));

        for i in 0..n {
            let mut e = Array1::zeros(n);
            e[i] = 1.0;
            let col = self.solve(&e)?;
            inv.column_mut(i).assign(&col);
        }

        Ok(inv)
    }
}

pub fn lu_decomp(a: Array2<f64>) -> Result<LUDecomposition, &'static str> {
    LUDecomposition::new(a)
}

pub fn lu_solve(lu: &Array2<f64>, pivots: &Array1<usize>, b: &Array1<f64>) -> Result<Array1<f64>, &'static str> {
    let decomp = LUDecomposition {
        lu: lu.clone(),
        pivots: pivots.clone(),
        signum: 1, // Not used for solving
    };
    decomp.solve(b)
}

pub fn lu_det(lu: &Array2<f64>, signum: i32) -> f64 {
    let n = lu.dim().0;
    let mut det = signum as f64;
    for i in 0..n {
        det *= lu[(i, i)];
    }
    det
}

pub fn lu_invert(lu: &Array2<f64>, pivots: &Array1<usize>) -> Result<Array2<f64>, &'static str> {
    let decomp = LUDecomposition {
        lu: lu.clone(),
        pivots: pivots.clone(),
        signum: 1, // Not used for inversion
    };
    decomp.inv()
}

fn apply_pivots(a: &mut Array2<f64>, ipiv: &Array1<usize>) -> Result<(), &'static str> {
    if a.dim().0 < ipiv.len() {
        return Err("Matrix does not match pivot vector");
    }

    for (i, &pi) in ipiv.iter().enumerate() {
        if i != pi {
            a.swap_rows(i, pi);
        }
    }

    Ok(())
}

fn singular(lu: &Array2<f64>) -> bool {
    let n = lu.dim().0;
    for i in 0..n {
        if lu[(i, i)] == 0.0 {
            return true;
        }
    }
    false
}