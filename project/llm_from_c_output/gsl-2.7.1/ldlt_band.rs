use std::cmp::{max, min};
use std::f64;

pub struct LDLTBand {
    matrix: Vec<Vec<f64>>,
    size1: usize,
    size2: usize,
}

impl LDLTBand {
    pub fn new(matrix: Vec<Vec<f64>>) -> Result<Self, &'static str> {
        let size1 = matrix.len();
        if size1 == 0 {
            return Err("matrix must have at least one row");
        }
        let size2 = matrix[0].len();
        for row in &matrix {
            if row.len() != size2 {
                return Err("all matrix rows must have the same length");
            }
        }
        Ok(Self { matrix, size1, size2 })
    }

    pub fn decomp(&mut self) -> Result<(), &'static str> {
        let n = self.size1;
        let ndiag = self.size2;

        if ndiag > n {
            return Err("invalid matrix dimensions");
        }

        if ndiag == 1 {
            return Ok(());
        }

        let p = ndiag - 1;
        let kld = max(1, p) as usize;
        let anorm = self.symband_norm1()?;
        self.matrix[n - 1][p] = anorm;

        for j in 0..n - 1 {
            let ajj = self.matrix[j][0];
            if ajj == 0.0 {
                return Err("matrix is singular");
            }

            let lenv = min(p, n - j - 1);
            if lenv > 0 {
                // Scale v by 1/ajj
                for i in 1..=lenv {
                    self.matrix[j][i] /= ajj;
                }

                // Perform DSYR operation: m -= ajj * v * v^T
                for k in 0..lenv {
                    for l in 0..lenv {
                        self.matrix[j + 1 + k][l] -= ajj * self.matrix[j][k + 1] * self.matrix[j][l + 1];
                    }
                }
            }
        }

        Ok(())
    }

    pub fn solve(&self, b: &[f64], x: &mut [f64]) -> Result<(), &'static str> {
        if self.size1 != b.len() {
            return Err("matrix size must match b size");
        }
        if self.size1 != x.len() {
            return Err("matrix size must match solution size");
        }

        x.copy_from_slice(b);
        self.svx(x)
    }

    pub fn svx(&self, x: &mut [f64]) -> Result<(), &'static str> {
        if self.size1 != x.len() {
            return Err("matrix size must match solution size");
        }

        let n = self.size1;
        let p = self.size2 - 1;

        // Forward substitution: L z = x
        for i in 0..n {
            let start = if i > p { i - p } else { 0 };
            for j in start..i {
                x[i] -= self.matrix[j][i - j] * x[j];
            }
        }

        // Diagonal scaling: D y = z
        for i in 0..n {
            x[i] /= self.matrix[i][0];
        }

        // Back substitution: L^T x = y
        for i in (0..n).rev() {
            let end = min(i + p + 1, n);
            for j in i + 1..end {
                x[i] -= self.matrix[i][j - i] * x[j];
            }
        }

        Ok(())
    }

    pub fn unpack(&self, l: &mut [Vec<f64>], d: &mut [f64]) -> Result<(), &'static str> {
        let n = self.size1;
        if n != l.len() {
            return Err("L matrix does not match LDLT dimensions");
        }
        if l.iter().any(|row| row.len() != n) {
            return Err("L matrix is not square");
        }
        if n != d.len() {
            return Err("D vector does not match LDLT dimensions");
        }

        let p = self.size2 - 1;

        // Copy diagonal entries
        for i in 0..n {
            d[i] = self.matrix[i][0];
        }

        // Initialize L as identity matrix
        for i in 0..n {
            for j in 0..n {
                l[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }

        // Copy subdiagonals into L
        for i in 1..=p {
            for j in 0..n - i {
                l[j + i][j] = self.matrix[j][i];
            }
        }

        Ok(())
    }

    pub fn rcond(&self, work: &mut [f64]) -> Result<f64, &'static str> {
        let n = self.size1;
        if work.len() != 3 * n {
            return Err("work vector must have length 3*N");
        }

        let ndiag = self.size2;
        let anorm = if ndiag == 1 {
            self.symband_norm1()?
        } else {
            self.matrix[n - 1][ndiag - 1]
        };

        if anorm == 0.0 {
            return Ok(0.0);
        }

        let ainvnorm = self.invnorm1(work)?;
        if ainvnorm == 0.0 {
            Ok(0.0)
        } else {
            Ok((1.0 / anorm) / ainvnorm)
        }
    }

    fn symband_norm1(&self) -> Result<f64, &'static str> {
        let n = self.size1;
        let ndiag = self.size2;

        if ndiag == 1 {
            // Diagonal matrix
            let mut max_val = 0.0;
            for i in 0..n {
                max_val = max_val.max(self.matrix[i][0].abs());
            }
            Ok(max_val)
        } else {
            let mut value = 0.0;
            for j in 0..n {
                let ncol = min(ndiag, n - j);
                let mut sum = 0.0;
                for i in 0..ncol {
                    sum += self.matrix[j][i].abs();
                }

                let mut k = j;
                let mut l = 1;
                while k > 0 && l < ndiag {
                    k -= 1;
                    sum += self.matrix[k][l].abs();
                    l += 1;
                }

                value = value.max(sum);
            }
            Ok(value)
        }
    }

    fn invnorm1(&self, work: &mut [f64]) -> Result<f64, &'static str> {
        let n = self.size1;
        let mut ainvnorm = 0.0;

        for k in 0..n {
            let (x, e) = work.split_at_mut(n);
            let (e, work) = e.split_at_mut(n);

            x.fill(0.0);
            x[k] = 1.0;

            self.svx(x)?;

            let mut sum = 0.0;
            for i in 0..n {
                sum += x[i].abs();
            }

            if sum > ainvnorm {
                ainvnorm = sum;
            }
        }

        Ok(ainvnorm)
    }
}