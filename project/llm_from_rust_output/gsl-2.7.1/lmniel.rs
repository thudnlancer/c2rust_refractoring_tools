use gsl::{
    blas::{CblasDiag, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    matrix::Matrix,
    multifit::{FunctionFdf, MultifitFdfsolverType},
    vector::Vector,
};

pub struct LmnielState {
    a: Matrix,
    a_copy: Matrix,
    j: Matrix,
    diag: Vector,
    rhs: Vector,
    x_trial: Vector,
    f_trial: Vector,
    work: Vector,
    nu: i64,
    mu: f64,
    tau: f64,
}

impl LmnielState {
    fn calc_dx(&mut self, mu: f64, a: &Matrix, rhs: &Vector, dx: &mut Vector) -> GslResult<()> {
        let mut diag = a.diagonal();
        self.a_copy.copy(a)?;
        diag.add_constant(mu)?;
        self.a_copy.qr_decomp(&mut self.work)?;
        self.a_copy.qr_solve(&self.work, rhs, dx)
    }

    fn trial_step(x: &Vector, dx: &Vector, x_trial: &mut Vector) {
        for i in 0..x.len() {
            let dxi = dx.get(i);
            let xi = x.get(i);
            x_trial.set(i, xi + dxi);
        }
    }

    fn calc_df(f: &Vector, f_new: &Vector) -> f64 {
        let mut df = 0.0;
        for i in 0..f.len() {
            let fi = f.get(i);
            let fnewi = f_new.get(i);
            df += (fi - fnewi) * (fi + fnewi);
        }
        df * 0.5
    }

    fn calc_dl(mu: f64, diag: &Vector, dx: &Vector, mg: &Vector) -> f64 {
        let mut dl = 0.0;
        for i in 0..dx.len() {
            let dxi = dx.get(i);
            let di = diag.get(i);
            let mgi = mg.get(i);
            dl += dxi * (mu * di * di * dxi + mgi);
        }
        dl * 0.5
    }
}

impl MultifitFdfsolverType for LmnielState {
    fn new(n: usize, p: usize) -> Result<Self, GslError> {
        Ok(Self {
            a: Matrix::new(p, p)?,
            j: Matrix::new(n, p)?,
            diag: Vector::new(p)?,
            rhs: Vector::new(p)?,
            work: Vector::new(p)?,
            a_copy: Matrix::new(p, p)?,
            x_trial: Vector::new(p)?,
            f_trial: Vector::new(n)?,
            nu: 2,
            mu: -1.0,
            tau: 1.0e-3,
        })
    }

    fn set(
        &mut self,
        swts: &Vector,
        fdf: &mut FunctionFdf,
        x: &mut Vector,
        f: &mut Vector,
        dx: &mut Vector,
    ) -> GslResult<()> {
        fdf.nevalf = 0;
        fdf.nevaldf = 0;
        fdf.eval_wf(x, swts, f)?;

        if fdf.has_df() {
            fdf.eval_wdf(x, swts, &mut self.j)?;
        } else {
            fdf.fdfsolver_dif_df(x, swts, f, &mut self.j)?;
        }

        self.j.gemv(CblasTranspose::Trans, -1.0, f, 0.0, &mut self.rhs)?;
        self.diag.set_all(1.0);

        self.nu = 2;
        self.mu = -1.0;

        for i in 0..x.len() {
            let c = self.j.column(i);
            let result = c.dot(&c)?;
            self.mu = self.mu.max(result);
        }

        self.mu *= self.tau;
        Ok(())
    }

    fn iterate(
        &mut self,
        swts: &Vector,
        fdf: &mut FunctionFdf,
        x: &mut Vector,
        f: &mut Vector,
        dx: &mut Vector,
    ) -> GslResult<()> {
        self.j.syrk(CblasUplo::Lower, CblasTranspose::Trans, 1.0, 0.0, &mut self.a)?;
        self.a.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit)?;

        let mut foundstep = false;
        while !foundstep {
            self.calc_dx(self.mu, &self.a, &self.rhs, dx)?;
            Self::trial_step(x, dx, &mut self.x_trial);
            fdf.eval_wf(&self.x_trial, swts, &mut self.f_trial)?;

            let df = Self::calc_df(f, &self.f_trial);
            let dl = Self::calc_dl(self.mu, &self.diag, dx, &self.rhs);

            if dl > 0.0 && df >= 0.0 {
                let tmp = (2.0 * (df / dl) - 1.0).max(0.333333333333333).min(1.0);
                self.mu *= tmp;
                self.nu = 2;

                if fdf.has_df() {
                    fdf.eval_wdf(&self.x_trial, swts, &mut self.j)?;
                } else {
                    fdf.fdfsolver_dif_df(&self.x_trial, swts, &self.f_trial, &mut self.j)?;
                }

                x.copy(&self.x_trial)?;
                f.copy(&self.f_trial)?;
                self.j.gemv(CblasTranspose::Trans, -1.0, f, 0.0, &mut self.rhs)?;
                foundstep = true;
            } else {
                let nu2 = self.nu << 1;
                self.mu *= self.nu as f64;
                if nu2 <= self.nu {
                    let d = self.a.diagonal();
                    self.nu = 2;
                    self.mu = self.tau * d.max();
                    break;
                } else {
                    self.nu = nu2;
                }
            }
        }
        Ok(())
    }

    fn gradient(&self, g: &mut Vector) -> GslResult<()> {
        g.copy(&self.rhs)?;
        g.scale(-1.0)
    }

    fn jacobian(&self, j: &mut Matrix) -> GslResult<()> {
        j.copy(&self.j)
    }
}

pub static GSL_MULTIFIT_FDFSOLVER_LMNIEL: &dyn MultifitFdfsolverType = &LmnielState;