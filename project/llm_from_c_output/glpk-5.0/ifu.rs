/* ifu.rs (dense updatable IFU-factorization) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2012-2013 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::f64;
use std::mem;

/// IFU-factorization structure
pub struct IFU {
    n_max: usize,
    n: usize,
    f: Vec<f64>,
    u: Vec<f64>,
}

impl IFU {
    /// Creates a new IFU factorization with given maximum order
    pub fn new(n_max: usize) -> Self {
        assert!(n_max >= 1);
        IFU {
            n_max,
            n: 0,
            f: vec![0.0; n_max * n_max],
            u: vec![0.0; n_max * n_max],
        }
    }

    /// Expands the IFU-factorization
    pub fn expand(&mut self, c: &[f64], r: &[f64], d: f64) {
        assert!(self.n < self.n_max);
        let n = self.n;
        let n_max = self.n_max;

        // Set new zero column of matrix F
        for i in 0..n {
            self.f[i * n_max + n] = 0.0;
        }

        // Set new zero row of matrix F
        for j in 0..n {
            self.f[n * n_max + j] = 0.0;
        }

        // Set new unity diagonal element of matrix F
        self.f[n * n_max + n] = 1.0;

        // Set new column of matrix U to vector (old F) * c
        for i in 0..n {
            let mut t = 0.0;
            for j in 0..n {
                t += self.f[i * n_max + j] * c[j];
            }
            self.u[i * n_max + n] = t;
        }

        // Set new row of matrix U to vector r
        for j in 0..n {
            self.u[n * n_max + j] = r[j];
        }

        // Set new diagonal element of matrix U to scalar d
        self.u[n * n_max + n] = d;

        // Increase factorization order
        self.n += 1;
    }

    /// Updates IFU-factorization using Bartels-Golub method
    pub fn bg_update(&mut self, c: &[f64], r: &[f64], d: f64) -> Result<(), &'static str> {
        const TOL: f64 = 1e-5;
        let n_max = self.n_max;
        let n = self.n;

        // Expand factorization
        self.expand(c, r, d);

        // Eliminate spike in last row of matrix U
        for k in 0..n {
            // Check if pivot is eligible
            if self.u[k * n_max + k].abs() < self.u[n * n_max + k].abs() {
                // Interchange rows k and n of matrix U
                for j in k..=n {
                    let tmp = self.u[k * n_max + j];
                    self.u[k * n_max + j] = self.u[n * n_max + j];
                    self.u[n * n_max + j] = tmp;
                }

                // Interchange rows k and n of matrix F
                for j in 0..=n {
                    let tmp = self.f[k * n_max + j];
                    self.f[k * n_max + j] = self.f[n * n_max + j];
                    self.f[n * n_max + j] = tmp;
                }
            }

            // Check if diagonal element is too small
            if self.u[k * n_max + k].abs() < TOL {
                return Err("Pivot too small in magnitude");
            }

            // Skip elimination if element is zero
            if self.u[n * n_max + k] == 0.0 {
                continue;
            }

            // Compute Gaussian multiplier
            let t = self.u[n * n_max + k] / self.u[k * n_max + k];

            // Apply Gaussian transformation to U
            for j in (k + 1)..=n {
                self.u[n * n_max + j] -= t * self.u[k * n_max + j];
            }

            // Apply same transformation to F
            for j in 0..=n {
                self.f[n * n_max + j] -= t * self.f[k * n_max + j];
            }
        }

        // Check final diagonal element
        if self.u[n * n_max + n].abs() < TOL {
            Err("Final diagonal element too small")
        } else {
            Ok(())
        }
    }

    /// Updates IFU-factorization using Givens rotations
    pub fn gr_update(&mut self, c: &[f64], r: &[f64], d: f64) -> Result<(), &'static str> {
        const TOL: f64 = 1e-5;
        let n_max = self.n_max;
        let n = self.n;

        // Expand factorization
        self.expand(c, r, d);

        // Eliminate spike in last row of matrix U
        for k in 0..n {
            // Check if elements are eligible
            if self.u[k * n_max + k].abs() < TOL && self.u[n * n_max + k].abs() < TOL {
                return Err("Both elements too small");
            }

            // Skip elimination if element is zero
            if self.u[n * n_max + k] == 0.0 {
                continue;
            }

            // Compute Givens rotation parameters
            let (cs, sn) = Self::givens(self.u[k * n_max + k], self.u[n * n_max + k]);

            // Apply rotation to U
            for j in k..=n {
                let ukj = self.u[k * n_max + j];
                let unj = self.u[n * n_max + j];
                self.u[k * n_max + j] = cs * ukj - sn * unj;
                self.u[n * n_max + j] = sn * ukj + cs * unj;
            }

            // Apply same rotation to F
            for j in 0..=n {
                let fkj = self.f[k * n_max + j];
                let fnj = self.f[n * n_max + j];
                self.f[k * n_max + j] = cs * fkj - sn * fnj;
                self.f[n * n_max + j] = sn * fkj + cs * fnj;
            }
        }

        // Check final diagonal element
        if self.u[n * n_max + n].abs() < TOL {
            Err("Final diagonal element too small")
        } else {
            Ok(())
        }
    }

    /// Computes Givens rotation parameters
    fn givens(a: f64, b: f64) -> (f64, f64) {
        if b == 0.0 {
            (1.0, 0.0)
        } else if a.abs() <= b.abs() {
            let t = -a / b;
            let s = 1.0 / (1.0 + t * t).sqrt();
            (s * t, s)
        } else {
            let t = -b / a;
            let c = 1.0 / (1.0 + t * t).sqrt();
            (c, c * t)
        }
    }

    /// Solves system A * x = b
    pub fn a_solve(&self, x: &mut [f64], w: &mut [f64]) {
        let n = self.n;
        let n_max = self.n_max;

        // y := F * b
        w[1..=n].copy_from_slice(&x[1..=n]);
        for i in 0..n {
            let mut t = 0.0;
            for j in 0..n {
                t += self.f[i * n_max + j] * w[j + 1];
            }
            x[i + 1] = t;
        }

        // x := inv(U) * y
        for i in (0..n).rev() {
            let mut t = x[i + 1];
            for j in (i + 1)..n {
                t -= self.u[i * n_max + j] * x[j + 1];
            }
            x[i + 1] = t / self.u[i * n_max + i];
        }
    }

    /// Solves system A' * x = b
    pub fn at_solve(&self, x: &mut [f64], w: &mut [f64]) {
        let n = self.n;
        let n_max = self.n_max;

        // y := inv(U') * b
        for i in 0..n {
            let t = x[i + 1] / self.u[i * n_max + i];
            x[i + 1] = t;
            for j in (i + 1)..n {
                x[j + 1] -= self.u[i * n_max + j] * t;
            }
        }

        // x := F' * y
        for j in 0..n {
            let mut t = 0.0;
            for i in 0..n {
                t += self.f[i * n_max + j] * x[i + 1];
            }
            w[j + 1] = t;
        }

        // Copy result back to x
        x[1..=n].copy_from_slice(&w[1..=n]);
    }
}