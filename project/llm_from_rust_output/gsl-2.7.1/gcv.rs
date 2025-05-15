use gsl::{
    blas::CblasTranspose,
    error::{GslError, GslResult},
    matrix::Matrix,
    minimization::{Minimizer, MinimizerType},
    vector::Vector,
    GslContext,
};

pub struct GcvParams<'a> {
    s: &'a Vector,
    uty: &'a Vector,
    delta0: f64,
    np: usize,
    workp: &'a mut Vector,
}

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
    pub fn gcv_init(
        &mut self,
        y: &Vector,
        reg_param: &mut Vector,
        uty: &mut Vector,
        delta0: &mut f64,
    ) -> GslResult<()> {
        if y.len() != self.n {
            return Err(GslError::BadLen("y vector does not match workspace"));
        }
        if uty.len() != self.p {
            return Err(GslError::BadLen("UTy vector does not match workspace"));
        }

        let p = self.p;
        let u = self.a.submatrix(0, 0, self.n, p)?;
        let s = self.s.subvector(0, p)?;

        let smax = s.get(0);
        let smin = s.get(p - 1);
        let normy = y.norm2();
        let normuty = {
            uty.gemv(CblasTranspose::Trans, 1.0, &u, y, 0.0)?;
            uty.norm2()
        };

        let dr = (normy + normuty) * (normy - normuty);
        *delta0 = if self.n > p && dr > 0.0 { dr } else { 0.0 };

        self.linear_lreg(smin, smax, reg_param)
    }

    pub fn gcv_curve(
        &mut self,
        reg_param: &Vector,
        uty: &Vector,
        delta0: f64,
        g: &mut Vector,
    ) -> GslResult<()> {
        let p = self.p;
        let n = reg_param.len();

        if uty.len() != p {
            return Err(GslError::BadLen("UTy vector does not match workspace"));
        }
        if g.len() != n {
            return Err(GslError::BadLen(
                "size of reg_param and G vectors do not match",
            ));
        }

        let s = self.s.subvector(0, p)?;
        let workp = self.qsi.subcolumn(0, 0, p)?;

        let params = GcvParams {
            s: &s,
            uty,
            delta0,
            np: self.n - p,
            workp,
        };

        for i in 0..n {
            let lambdai = reg_param.get(i);
            let gi = gcv_func(lambdai, &params)?;
            g.set(i, gi);
        }

        Ok(())
    }

    pub fn gcv_min(
        &self,
        reg_param: &Vector,
        uty: &Vector,
        g: &Vector,
        delta0: f64,
        lambda: &mut f64,
    ) -> GslResult<()> {
        let p = self.p;
        let npts = reg_param.len();

        if uty.len() != p {
            return Err(GslError::BadLen("UTy vector does not match workspace"));
        }
        if g.len() != npts {
            return Err(GslError::BadLen(
                "size of reg_param and G vectors do not match",
            ));
        }

        let max_iter = 500;
        let tol = 1.0e-4;
        let s = self.s.subvector(0, p)?;
        let mut workp = self.qsi.subcolumn(0, 0, p)?;

        let idx_g = g.min_index()?;
        let npts_i = npts as i32;

        if idx_g == 0 || idx_g == npts_i - 1 {
            *lambda = reg_param.get(idx_g as usize);
            return Ok(());
        }

        let a = reg_param.get(if idx_g + 1 < npts_i - 1 {
            (idx_g + 1) as usize
        } else {
            npts - 1
        });
        let b = reg_param.get(if idx_g - 1 > 0 {
            (idx_g - 1) as usize
        } else {
            0
        });
        let m = reg_param.get(idx_g as usize);

        let params = GcvParams {
            s: &s,
            uty,
            delta0,
            np: self.n - p,
            workp: &mut workp,
        };

        let mut minimizer = Minimizer::new(MinimizerType::Brent)?;
        minimizer.set(gcv_func, m, a, b, &params)?;

        let mut iter = 0;
        let mut status = GslResult::Continue;
        while status == GslResult::Continue && iter < max_iter {
            iter += 1;
            status = minimizer.iterate()?;
            let a = minimizer.x_lower();
            let b = minimizer.x_upper();
            status = minimizer.test_interval(a, b, 0.0, tol)?;
        }

        if status == GslResult::Success {
            *lambda = minimizer.minimum();
            Ok(())
        } else {
            Err(GslError::MaxIteration)
        }
    }

    pub fn gcv_calc(
        &mut self,
        lambda: f64,
        uty: &Vector,
        delta0: f64,
    ) -> GslResult<f64> {
        let p = self.p;
        if uty.len() != p {
            return Err(GslError::BadLen("UTy vector does not match workspace"));
        }

        let s = self.s.subvector(0, p)?;
        let mut workp = self.qsi.subcolumn(0, 0, p)?;

        let params = GcvParams {
            s: &s,
            uty,
            delta0,
            np: self.n - p,
            workp: &mut workp,
        };

        gcv_func(lambda, &params)
    }

    pub fn gcv(
        &mut self,
        y: &Vector,
        reg_param: &mut Vector,
        g: &mut Vector,
        lambda: &mut f64,
        g_lambda: &mut f64,
    ) -> GslResult<()> {
        let n = y.len();
        let n_reg = g.len();

        if n != self.n {
            return Err(GslError::BadLen("y vector does not match workspace"));
        }
        if reg_param.len() != n_reg {
            return Err(GslError::BadLen(
                "size of reg_param and G vectors do not match",
            ));
        }

        let p = self.p;
        let mut uty = self.xt.subvector(0, p)?;
        let mut delta0 = 0.0;

        self.gcv_init(y, reg_param, &mut uty, &mut delta0)?;
        self.gcv_curve(reg_param, &uty, delta0, g)?;
        self.gcv_min(reg_param, &uty, g, delta0, lambda)?;
        *g_lambda = self.gcv_calc(*lambda, &uty, delta0)?;

        Ok(())
    }

    fn linear_lreg(&self, smin: f64, smax: f64, reg_param: &mut Vector) -> GslResult<()> {
        // Implementation of gsl_multifit_linear_lreg
        Ok(())
    }
}

fn gcv_func(lambda: f64, params: &GcvParams) -> GslResult<f64> {
    let lambda_sq = lambda * lambda;
    let mut sumf = 0.0;

    for i in 0..params.s.len() {
        let si = params.s.get(i);
        let fi = lambda_sq / (si * si + lambda_sq);
        params.workp.set(i, fi)?;
        sumf += fi;
    }

    let d = params.np as f64 + sumf;
    params.workp.mul(params.uty)?;
    let norm = params.workp.norm2();
    let g = (norm * norm + params.delta0) / (d * d);

    Ok(g)
}