use std::f64;
use std::mem;
use std::ptr;

#[derive(Debug, Clone)]
pub struct IFU {
    n_max: i32,
    n: i32,
    f: Vec<f64>,
    u: Vec<f64>,
}

impl IFU {
    pub fn new(n_max: i32) -> Self {
        let size = (n_max * n_max) as usize;
        IFU {
            n_max,
            n: 0,
            f: vec![0.0; size],
            u: vec![0.0; size],
        }
    }

    fn index(&self, i: i32, j: i32) -> usize {
        (i * self.n_max + j) as usize
    }

    pub fn expand(&mut self, c: &[f64], r: &[f64], d: f64) {
        assert!(0 <= self.n && self.n < self.n_max);
        let n = self.n as usize;

        // Initialize new column and row in F
        for i in 0..n {
            self.f[self.index(i as i32, self.n)] = 0.0;
        }
        for j in 0..n {
            self.f[self.index(self.n, j as i32)] = 0.0;
        }
        self.f[self.index(self.n, self.n)] = 1.0;

        // Update U matrix
        for i in 0..n {
            let mut t = 0.0;
            for j in 0..n {
                t += self.f[self.index(i as i32, j as i32)] * c[j];
            }
            self.u[self.index(i as i32, self.n)] = t;
        }

        for j in 0..n {
            self.u[self.index(self.n, j as i32)] = r[j];
        }
        self.u[self.index(self.n, self.n)] = d;

        self.n += 1;
    }

    pub fn bg_update(&mut self, c: &[f64], r: &[f64], d: f64) -> i32 {
        let tol = 1e-5;
        self.expand(c, r, d);
        let n = (self.n - 1) as i32;

        for k in 0..n {
            if self.u[self.index(k, k)].abs() < self.u[self.index(n, k)].abs() {
                // Swap rows k and n in U
                for j in k..=n {
                    self.u.swap(self.index(k, j), self.index(n, j));
                }
                // Swap rows k and n in F
                for j in 0..=n {
                    self.f.swap(self.index(k, j), self.index(n, j));
                }
            }

            if self.u[self.index(k, k)].abs() < tol {
                return 1;
            }

            if self.u[self.index(n, k)] != 0.0 {
                let t = self.u[self.index(n, k)] / self.u[self.index(k, k)];
                for j in (k + 1)..=n {
                    self.u[self.index(n, j)] -= t * self.u[self.index(k, j)];
                }
                for j in 0..=n {
                    self.f[self.index(n, j)] -= t * self.f[self.index(k, j)];
                }
            }
        }

        if self.u[self.index(n, n)].abs() < tol {
            return 2;
        }
        0
    }

    pub fn gr_update(&mut self, c: &[f64], r: &[f64], d: f64) -> i32 {
        let tol = 1e-5;
        self.expand(c, r, d);
        let n = (self.n - 1) as i32;

        for k in 0..n {
            if self.u[self.index(k, k)].abs() < tol && self.u[self.index(n, k)].abs() < tol {
                return 1;
            }

            if self.u[self.index(n, k)] != 0.0 {
                let (cs, sn) = Self::givens(self.u[self.index(k, k)], self.u[self.index(n, k)]);

                // Apply Givens rotation to U
                for j in k..=n {
                    let ukj = self.u[self.index(k, j)];
                    let unj = self.u[self.index(n, j)];
                    self.u[self.index(k, j)] = cs * ukj - sn * unj;
                    self.u[self.index(n, j)] = sn * ukj + cs * unj;
                }

                // Apply Givens rotation to F
                for j in 0..=n {
                    let fkj = self.f[self.index(k, j)];
                    let fnj = self.f[self.index(n, j)];
                    self.f[self.index(k, j)] = cs * fkj - sn * fnj;
                    self.f[self.index(n, j)] = sn * fkj + cs * fnj;
                }
            }
        }

        if self.u[self.index(n, n)].abs() < tol {
            return 2;
        }
        0
    }

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

    pub fn a_solve(&self, x: &mut [f64], w: &mut [f64]) {
        assert!(0 <= self.n && self.n <= self.n_max);
        let n = self.n as usize;

        w[..n].copy_from_slice(&x[..n]);

        for i in 0..n {
            let mut t = 0.0;
            for j in 0..n {
                t += self.f[self.index(i as i32, j as i32)] * w[j];
            }
            x[i] = t;
        }

        for i in (0..n).rev() {
            let mut t = x[i];
            for j in (i + 1)..n {
                t -= self.u[self.index(i as i32, j as i32)] * x[j];
            }
            x[i] = t / self.u[self.index(i as i32, i as i32)];
        }
    }

    pub fn at_solve(&self, x: &mut [f64], w: &mut [f64]) {
        assert!(0 <= self.n && self.n <= self.n_max);
        let n = self.n as usize;

        for i in 0..n {
            x[i] /= self.u[self.index(i as i32, i as i32)];
            let t = x[i];
            for j in (i + 1)..n {
                x[j] -= self.u[self.index(i as i32, j as i32)] * t;
            }
        }

        for j in 0..n {
            let mut t = 0.0;
            for i in 0..n {
                t += self.f[self.index(i as i32, j as i32)] * x[i];
            }
            w[j] = t;
        }

        x[..n].copy_from_slice(&w[..n]);
    }
}