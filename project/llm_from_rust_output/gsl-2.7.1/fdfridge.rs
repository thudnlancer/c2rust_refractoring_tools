use gsl::{
    blas::CblasTranspose,
    error::{GslError, GslResult},
    matrix::Matrix,
    multifit::{FdfSolver, FdfSolverType, FunctionFdf},
    vector::Vector,
};

pub struct FdfRidge {
    n: usize,
    p: usize,
    lambda: f64,
    l_diag: Option<Vector>,
    l: Option<Matrix>,
    f: Vector,
    wts: Vector,
    s: FdfSolver,
    fdf: Option<FunctionFdf>,
    fdftik: FunctionFdf,
}

impl FdfRidge {
    pub fn new(
        solver_type: &FdfSolverType,
        n: usize,
        p: usize,
    ) -> Result<Self, GslError> {
        let s = FdfSolver::new(solver_type, n + p, p)?;
        let wts = Vector::new(n + p)?;
        let f = Vector::new(n)?;
        
        wts.set_all(1.0);
        
        Ok(Self {
            n,
            p,
            lambda: 0.0,
            l_diag: None,
            l: None,
            f,
            wts,
            s,
            fdf: None,
            fdftik: FunctionFdf::new(n + p, p),
        })
    }

    pub fn name(&self) -> &str {
        self.s.name()
    }

    pub fn position(&self) -> &Vector {
        self.s.position()
    }

    pub fn residual(&self) -> &Vector {
        self.s.residual()
    }

    pub fn niter(&self) -> usize {
        self.s.niter()
    }

    pub fn set(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        lambda: f64,
    ) -> GslResult<()> {
        self.set_internal(fdf, x, Some(lambda), None, None)
    }

    pub fn wset(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        lambda: f64,
        wts: Option<&Vector>,
    ) -> GslResult<()> {
        self.set_internal(fdf, x, Some(lambda), None, wts)
    }

    pub fn set2(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        lambda: &Vector,
    ) -> GslResult<()> {
        self.set_internal(fdf, x, None, Some(lambda), None)
    }

    pub fn wset2(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        lambda: &Vector,
        wts: Option<&Vector>,
    ) -> GslResult<()> {
        self.set_internal(fdf, x, None, Some(lambda), wts)
    }

    pub fn set3(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        l: &Matrix,
    ) -> GslResult<()> {
        self.set_internal(fdf, x, None, None, Some(l))
    }

    pub fn wset3(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        l: &Matrix,
        wts: Option<&Vector>,
    ) -> GslResult<()> {
        self.set_internal(fdf, x, None, None, Some(l), wts)
    }

    fn set_internal(
        &mut self,
        fdf: &mut FunctionFdf,
        x: &Vector,
        lambda: Option<f64>,
        l_diag: Option<&Vector>,
        l: Option<&Matrix>,
        wts: Option<&Vector>,
    ) -> GslResult<()> {
        if self.n != fdf.n() || self.p != fdf.p() {
            return Err(GslError::BadLen);
        }
        if self.p != x.len() {
            return Err(GslError::BadLen);
        }
        if let Some(w) = wts {
            if self.n != w.len() {
                return Err(GslError::BadLen);
            }
        }

        self.fdf = Some(fdf.clone());
        self.fdftik = FunctionFdf::new(self.n + self.p, self.p)
            .with_f(Self::fdfridge_f)
            .with_df(Self::fdfridge_df);
        self.fdftik.set_params(self);

        if let Some(lambda) = lambda {
            self.lambda = lambda;
        }
        if let Some(diag) = l_diag {
            self.l_diag = Some(diag.clone());
        }
        if let Some(l) = l {
            self.l = Some(l.clone());
        }

        if let Some(w) = wts {
            self.wts.view_mut(0, self.n)?.copy_from(w)?;
        }

        self.s.wset(&self.fdftik, x, &self.wts)
    }

    pub fn iterate(&mut self) -> GslResult<()> {
        self.s.iterate()?;
        if let Some(fdf) = &mut self.fdf {
            fdf.set_nevalf(self.fdftik.nevalf());
            fdf.set_nevaldf(self.fdftik.nevaldf());
        }
        Ok(())
    }

    pub fn driver(
        &mut self,
        maxiter: usize,
        xtol: f64,
        gtol: f64,
        ftol: f64,
    ) -> GslResult<i32> {
        let mut info = 0;
        self.s.driver(maxiter, xtol, gtol, ftol, &mut info)?;
        Ok(info)
    }

    fn fdfridge_f(
        x: &Vector,
        params: &mut dyn std::any::Any,
        f: &mut Vector,
    ) -> GslResult<()> {
        let w = params.downcast_mut::<Self>().unwrap();
        let n = w.n;
        let p = w.p;

        let mut f_user = f.view_mut(0, n)?;
        let mut f_tik = f.view_mut(n, p)?;

        w.fdf.as_mut().unwrap().eval_wf(x, None, &mut f_user)?;

        if let Some(l_diag) = &w.l_diag {
            f_tik.copy_from(x)?;
            f_tik.mul(l_diag)?;
        } else if let Some(l) = &w.l {
            f_tik.gemv(CblasTranspose::NoTrans, 1.0, l, x, 0.0)?;
        } else {
            f_tik.copy_from(x)?;
            f_tik.scale(w.lambda)?;
        }

        Ok(())
    }

    fn fdfridge_df(
        x: &Vector,
        params: &mut dyn std::any::Any,
        j: &mut Matrix,
    ) -> GslResult<()> {
        let w = params.downcast_mut::<Self>().unwrap();
        let n = w.n;
        let p = w.p;

        let mut j_user = j.view_mut(0, 0, n, p)?;
        let mut j_tik = j.view_mut(n, 0, p, p)?;
        let mut diag = j_tik.diagonal()?;

        let fdf = w.fdf.as_mut().unwrap();
        if fdf.has_df() {
            fdf.eval_wdf(x, None, &mut j_user)?;
        } else {
            fdf.eval_wf(x, None, &mut w.f)?;
            fdf.dif_df(x, None, &w.f, &mut j_user)?;
        }

        if let Some(l_diag) = &w.l_diag {
            j_tik.set_zero();
            diag.copy_from(l_diag)?;
        } else if let Some(l) = &w.l {
            j_tik.copy_from(l)?;
        } else {
            j_tik.set_zero();
            diag.set_all(w.lambda)?;
        }

        Ok(())
    }
}