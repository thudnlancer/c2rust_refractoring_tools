use gsl::{
    blas::{dgemv, ddot, dnrm2, CblasTranspose},
    linalg::{balance_columns, SV_decomp_mod},
    matrix::{Matrix, MatrixView},
    vector::{Vector, VectorView},
    types::{GslResult, GslError},
};

pub struct MultifitLinearWorkspace {
    nmax: usize,
    pmax: usize,
    n: usize,
    p: usize,
    a: Matrix,
    q: Matrix,
    qsi: Matrix,
    s: Vector,
    t: Vector,
    xt: Vector,
    d: Vector,
    rcond: f64,
}

impl MultifitLinearWorkspace {
    pub fn new(nmax: usize, pmax: usize) -> Self {
        Self {
            nmax,
            pmax,
            n: 0,
            p: 0,
            a: Matrix::new(nmax, pmax),
            q: Matrix::new(pmax, pmax),
            qsi: Matrix::new(pmax, pmax),
            s: Vector::new(pmax),
            t: Vector::new(nmax),
            xt: Vector::new(pmax),
            d: Vector::new(pmax),
            rcond: 0.0,
        }
    }

    pub fn multifit_linear(
        &mut self,
        x: &Matrix,
        y: &Vector,
        c: &mut Vector,
        cov: &mut Matrix,
        chisq: &mut f64,
    ) -> GslResult<()> {
        let mut rank = 0;
        self.multifit_linear_tsvd(x, y, 2.2204460492503131e-16, c, cov, chisq, &mut rank)
    }

    pub fn multifit_linear_tsvd(
        &mut self,
        x: &Matrix,
        y: &Vector,
        tol: f64,
        c: &mut Vector,
        cov: &mut Matrix,
        chisq: &mut f64,
        rank: &mut usize,
    ) -> GslResult<()> {
        let n = x.size1();
        let p = x.size2();

        if y.size() != n {
            return Err(GslError::BadLen);
        }
        if p != c.size() {
            return Err(GslError::BadLen);
        }
        if tol <= 0.0 {
            return Err(GslError::Invalid);
        }

        self.multifit_linear_bsvd(x)?;

        let (rnorm, snorm) = self.solve(x, y, tol, -1.0, rank, c)?;
        *chisq = rnorm * rnorm;

        let r2 = rnorm * rnorm;
        let s2 = r2 / (n - *rank) as f64;

        let qsi_view = self.qsi.submatrix(0, 0, p, p);
        let d_view = self.d.subvector(0, p);

        for i in 0..p {
            let row_i = qsi_view.row(i);
            let d_i = d_view.get(i);

            for j in i..p {
                let row_j = qsi_view.row(j);
                let d_j = d_view.get(j);
                let mut s = 0.0;
                ddot(&row_i, &row_j, &mut s)?;
                let val = s * s2 / (d_i * d_j);
                cov.set(i, j, val);
                cov.set(j, i, val);
            }
        }

        Ok(())
    }

    pub fn multifit_linear_svd(&mut self, x: &Matrix) -> GslResult<()> {
        self.svd(x, false)
    }

    pub fn multifit_linear_bsvd(&mut self, x: &Matrix) -> GslResult<()> {
        self.svd(x, true)
    }

    pub fn multifit_linear_rank(&self, tol: f64) -> usize {
        let s0 = self.s.get(0);
        let mut rank = 0;

        for j in 0..self.p {
            let sj = self.s.get(j);
            if sj > tol * s0 {
                rank += 1;
            }
        }

        rank
    }

    pub fn multifit_linear_est(
        x: &Vector,
        c: &Vector,
        cov: &Matrix,
        y: &mut f64,
        y_err: &mut f64,
    ) -> GslResult<()> {
        if x.size() != c.size() {
            return Err(GslError::BadLen);
        }
        if cov.size1() != cov.size2() {
            return Err(GslError::NotSquare);
        }
        if c.size() != cov.size1() {
            return Err(GslError::BadLen);
        }

        ddot(x, c, y)?;

        let mut var = 0.0;
        for i in 0..x.size() {
            let xi = x.get(i);
            var += xi * xi * cov.get(i, i);

            for j in 0..i {
                let xj = x.get(j);
                var += 2.0 * xi * xj * cov.get(i, j);
            }
        }

        *y_err = var.sqrt();
        Ok(())
    }

    pub fn multifit_linear_rcond(&self) -> f64 {
        self.rcond
    }

    pub fn multifit_linear_residuals(
        x: &Matrix,
        y: &Vector,
        c: &Vector,
        r: &mut Vector,
    ) -> GslResult<()> {
        if x.size1() != y.size() {
            return Err(GslError::BadLen);
        }
        if x.size2() != c.size() {
            return Err(GslError::BadLen);
        }
        if y.size() != r.size() {
            return Err(GslError::BadLen);
        }

        r.copy_from(y)?;
        dgemv(
            CblasTranspose::NoTrans,
            -1.0,
            x,
            c,
            1.0,
            r,
        )?;

        Ok(())
    }

    fn solve(
        &mut self,
        x: &Matrix,
        y: &Vector,
        tol: f64,
        lambda: f64,
        rank: &mut usize,
        c: &mut Vector,
    ) -> GslResult<(f64, f64)> {
        let n = x.size1();
        let p = x.size2();

        if n != self.n || p != self.p {
            return Err(GslError::BadLen);
        }
        if y.size() != n {
            return Err(GslError::BadLen);
        }
        if p != c.size() {
            return Err(GslError::BadLen);
        }
        if tol <= 0.0 {
            return Err(GslError::Invalid);
        }

        let lambda_sq = lambda * lambda;
        let mut rho_ls = 0.0;

        let a_view = self.a.submatrix(0, 0, n, p);
        let q_view = self.q.submatrix(0, 0, p, p);
        let s_view = self.s.subvector(0, p);
        let qsi_view = self.qsi.submatrix(0, 0, p, p);
        let xt_view = self.xt.subvector(0, p);
        let d_view = self.d.subvector(0, p);
        let t_view = self.t.subvector(0, n);

        dgemv(
            CblasTranspose::Trans,
            1.0,
            &a_view,
            y,
            0.0,
            &mut xt_view,
        )?;

        if n > p {
            t_view.copy_from(y)?;
            dgemv(
                CblasTranspose::NoTrans,
                -1.0,
                &a_view,
                &xt_view,
                1.0,
                &mut t_view,
            )?;
            rho_ls = dnrm2(&t_view);
        }

        if lambda > 0.0 {
            for j in 0..p {
                let sj = s_view.get(j);
                let f = sj * sj / (sj * sj + lambda_sq);
                let ptr = xt_view.get_mut(j);
                d_view.set(j, (1.0 - f) * *ptr);
                *ptr *= sj / (sj * sj + lambda_sq);
            }

            dgemv(
                CblasTranspose::NoTrans,
                1.0,
                &q_view,
                &xt_view,
                0.0,
                c,
            )?;

            let snorm = dnrm2(c);
            let rnorm = dnrm2(&d_view);

            let rnorm = if n > p {
                (rnorm * rnorm + rho_ls * rho_ls).sqrt()
            } else {
                rnorm
            };

            d_view.set_all(1.0);
            Ok((rnorm, snorm))
        } else {
            qsi_view.copy_from(&q_view)?;

            let s0 = s_view.get(0);
            *rank = 0;

            for j in 0..p {
                let column = qsi_view.column(j);
                let sj = s_view.get(j);
                let alpha = if sj <= tol * s0 {
                    0.0
                } else {
                    *rank += 1;
                    1.0 / sj
                };
                column.scale(alpha)?;
            }

            dgemv(
                CblasTranspose::NoTrans,
                1.0,
                &qsi_view,
                &xt_view,
                0.0,
                c,
            )?;

            c.div_elements(&d_view)?;
            let snorm = dnrm2(c);
            Ok((rho_ls, snorm))
        }
    }

    fn svd(&mut self, x: &Matrix, balance: bool) -> GslResult<()> {
        let n = x.size1();
        let p = x.size2();

        if n > self.nmax || p > self.pmax {
            return Err(GslError::BadLen);
        }

        let a_view = self.a.submatrix(0, 0, n, p);
        let q_view = self.q.submatrix(0, 0, p, p);
        let qsi_view = self.qsi.submatrix(0, 0, p, p);
        let s_view = self.s.subvector(0, p);
        let xt_view = self.xt.subvector(0, p);
        let d_view = self.d.subvector(0, p);

        a_view.copy_from(x)?;

        if balance {
            balance_columns(&mut a_view, &mut d_view)?;
        } else {
            d_view.set_all(1.0);
        }

        SV_decomp_mod(
            &mut a_view,
            &mut qsi_view,
            &mut q_view,
            &mut s_view,
            &mut xt_view,
        )?;

        let (smin, smax) = s_view.minmax();
        self.rcond = smin / smax;
        self.n = n;
        self.p = p;

        Ok(())
    }
}