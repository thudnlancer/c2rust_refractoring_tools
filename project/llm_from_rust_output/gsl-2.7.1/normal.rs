use gsl::{Vector, Matrix, BLAS, EigenSymm, Linalg, MultifitLinear};
use std::ffi::CString;
use std::os::raw::{c_double, c_void};
use std::ptr;

#[derive(Debug)]
struct NormalState {
    p: usize,
    ata: Matrix,
    atb: Vector,
    normb: c_double,
    work_ata: Matrix,
    workp: Vector,
    work3p: Vector,
    d: Vector,
    c: Vector,
    eigen: bool,
    eval_min: c_double,
    eval_max: c_double,
    eigen_p: EigenSymm,
}

impl NormalState {
    fn new(p: usize) -> Result<Self, &'static str> {
        if p == 0 {
            return Err("p must be a positive integer");
        }

        Ok(NormalState {
            p,
            ata: Matrix::new(p, p).map_err(|_| "failed to allocate ATA matrix")?,
            atb: Vector::new(p).map_err(|_| "failed to allocate ATb vector")?,
            normb: 0.0,
            work_ata: Matrix::new(p, p).map_err(|_| "failed to allocate temporary ATA matrix")?,
            workp: Vector::new(p).map_err(|_| "failed to allocate temporary ATb vector")?,
            work3p: Vector::new(3 * p).map_err(|_| "failed to allocate work3p")?,
            d: Vector::new(p).map_err(|_| "failed to allocate D vector")?,
            c: Vector::new(p).map_err(|_| "failed to allocate c vector")?,
            eigen: false,
            eval_min: 0.0,
            eval_max: 0.0,
            eigen_p: EigenSymm::new(p).map_err(|_| "failed to allocate eigen workspace")?,
        })
    }

    fn reset(&mut self) {
        self.ata.set_zero();
        self.atb.set_zero();
        self.normb = 0.0;
        self.eigen = false;
        self.eval_min = 0.0;
        self.eval_max = 0.0;
    }

    fn accumulate(&mut self, a: &Matrix, b: &Vector) -> Result<(), &'static str> {
        if a.size2() != self.p {
            return Err("columns of A do not match workspace");
        } else if a.size1() != b.len() {
            return Err("A and b have different numbers of rows");
        }

        BLAS::dsyrk(CblasLower, CblasTrans, 1.0, a, 1.0, &mut self.ata)?;
        BLAS::dgemv(CblasTrans, 1.0, a, b, 1.0, &mut self.atb)?;
        self.normb = gsl_hypot(self.normb, BLAS::dnrm2(b));

        Ok(())
    }

    fn solve(
        &mut self,
        lambda: c_double,
        x: &mut Vector,
        rnorm: &mut c_double,
        snorm: &mut c_double,
    ) -> Result<(), &'static str> {
        if x.len() != self.p {
            return Err("solution vector does not match workspace");
        }

        self.solve_system(lambda, x)?;
        self.calc_norms(x, rnorm, snorm);

        Ok(())
    }

    fn rcond(&mut self) -> Result<c_double, &'static str> {
        let mut rcond_ata = 0.0;
        Linalg::cholesky_rcond(&mut self.work_ata, &mut rcond_ata, &mut self.work3p)?;
        Ok(rcond_ata.sqrt())
    }

    fn lcurve(
        &mut self,
        reg_param: &mut Vector,
        rho: &mut Vector,
        eta: &mut Vector,
    ) -> Result<(), &'static str> {
        if !self.eigen {
            self.eigen()?;
        }

        if self.eval_max < 0.0 {
            return Err("matrix is not positive definite");
        }

        let smax = self.eval_max.sqrt();
        let smin = if self.eval_min > 0.0 {
            self.eval_min.sqrt()
        } else {
            0.0
        };

        MultifitLinear::lreg(smin, smax, reg_param)?;

        for i in 0..reg_param.len() {
            let lambda = reg_param.get(i);
            let mut rnorm = 0.0;
            let mut snorm = 0.0;

            self.solve_system(lambda, &mut self.c)?;
            self.calc_norms(&self.c, &mut rnorm, &mut snorm);

            rho.set(i, rnorm);
            eta.set(i, snorm);
        }

        Ok(())
    }

    fn solve_system(&mut self, lambda: c_double, x: &mut Vector) -> Result<(), &'static str> {
        let lambda_sq = lambda * lambda;
        let mut d = self.work_ata.diagonal();

        self.work_ata.tricpy(CblasLower, CblasNonUnit, &self.ata)?;
        d.add_constant(lambda_sq)?;

        self.solve_cholesky(&mut self.work_ata, &self.atb, x)
    }

    fn solve_cholesky(
        &mut self,
        ata: &mut Matrix,
        atb: &Vector,
        x: &mut Vector,
    ) -> Result<(), &'static str> {
        Linalg::cholesky_decomp2(ata, &mut self.d)?;
        Linalg::cholesky_solve2(ata, &self.d, atb, x)
    }

    fn calc_norms(&mut self, x: &Vector, rnorm: &mut c_double, snorm: &mut c_double) {
        *snorm = BLAS::dnrm2(x);
        self.workp.copy(&self.atb).unwrap();
        BLAS::dsymv(
            CblasLower,
            1.0,
            &self.ata,
            x,
            -2.0,
            &mut self.workp,
        ).unwrap();

        let mut r2 = 0.0;
        BLAS::ddot(x, &self.workp, &mut r2).unwrap();
        r2 += self.normb * self.normb;
        *rnorm = r2.sqrt();
    }

    fn eigen(&mut self) -> Result<(), &'static str> {
        self.work_ata.tricpy(CblasLower, CblasNonUnit, &self.ata)?;
        EigenSymm::symm(&mut self.work_ata, &mut self.workp, &mut self.eigen_p)?;

        let (min, max) = self.workp.minmax();
        self.eval_min = min;
        self.eval_max = max;
        self.eigen = true;

        Ok(())
    }
}

pub struct NormalType {
    name: CString,
}

impl NormalType {
    pub fn new() -> Self {
        NormalType {
            name: CString::new("normal").unwrap(),
        }
    }

    pub fn alloc(&self, p: usize) -> *mut c_void {
        match NormalState::new(p) {
            Ok(state) => Box::into_raw(Box::new(state)) as *mut c_void,
            Err(_) => ptr::null_mut(),
        }
    }

    pub fn reset(&self, vstate: *mut c_void) -> i32 {
        if let Some(state) = unsafe { (vstate as *mut NormalState).as_mut() } {
            state.reset();
            gsl::SUCCESS
        } else {
            gsl::FAILURE
        }
    }

    pub fn accumulate(&self, a: *mut Matrix, b: *mut Vector, vstate: *mut c_void) -> i32 {
        if let Some(state) = unsafe { (vstate as *mut NormalState).as_mut() } {
            match unsafe { a.as_ref() }
                .and_then(|a| unsafe { b.as_ref() }.map(|b| state.accumulate(a, b)))
            {
                Ok(_) => gsl::SUCCESS,
                Err(_) => gsl::FAILURE,
            }
        } else {
            gsl::FAILURE
        }
    }

    pub fn solve(
        &self,
        lambda: c_double,
        x: *mut Vector,
        rnorm: *mut c_double,
        snorm: *mut c_double,
        vstate: *mut c_void,
    ) -> i32 {
        if let Some(state) = unsafe { (vstate as *mut NormalState).as_mut() } {
            match unsafe { x.as_mut() }
                .and_then(|x| {
                    unsafe { rnorm.as_mut() }
                        .and_then(|rnorm| unsafe { snorm.as_mut() }.map(|snorm| (x, rnorm, snorm)))
                })
                .map(|(x, rnorm, snorm)| state.solve(lambda, x, rnorm, snorm))
            {
                Ok(_) => gsl::SUCCESS,
                Err(_) => gsl::FAILURE,
            }
        } else {
            gsl::FAILURE
        }
    }

    pub fn rcond(&self, rcond: *mut c_double, vstate: *mut c_void) -> i32 {
        if let Some(state) = unsafe { (vstate as *mut NormalState).as_mut() } {
            match unsafe { rcond.as_mut() }
                .and_then(|rcond| state.rcond().map(|val| *rcond = val).ok())
            {
                Some(_) => gsl::SUCCESS,
                None => gsl::FAILURE,
            }
        } else {
            gsl::FAILURE
        }
    }

    pub fn lcurve(
        &self,
        reg_param: *mut Vector,
        rho: *mut Vector,
        eta: *mut Vector,
        vstate: *mut c_void,
    ) -> i32 {
        if let Some(state) = unsafe { (vstate as *mut NormalState).as_mut() } {
            match unsafe { reg_param.as_mut() }
                .and_then(|reg_param| {
                    unsafe { rho.as_mut() }
                        .and_then(|rho| unsafe { eta.as_mut() }.map(|eta| (reg_param, rho, eta)))
                })
                .map(|(reg_param, rho, eta)| state.lcurve(reg_param, rho, eta))
            {
                Ok(_) => gsl::SUCCESS,
                Err(_) => gsl::FAILURE,
            }
        } else {
            gsl::FAILURE
        }
    }

    pub fn ata_ptr(&self, vstate: *const c_void) -> *const Matrix {
        unsafe { (vstate as *const NormalState).as_ref() }
            .map(|state| &state.ata as *const Matrix)
            .unwrap_or(ptr::null())
    }

    pub fn atb_ptr(&self, vstate: *const c_void) -> *const Vector {
        unsafe { (vstate as *const NormalState).as_ref() }
            .map(|state| &state.atb as *const Vector)
            .unwrap_or(ptr::null())
    }

    pub fn free(&self, vstate: *mut c_void) {
        if !vstate.is_null() {
            unsafe { Box::from_raw(vstate as *mut NormalState) };
        }
    }
}

pub static GSL_MULTILARGE_LINEAR_NORMAL: NormalType = NormalType::new();