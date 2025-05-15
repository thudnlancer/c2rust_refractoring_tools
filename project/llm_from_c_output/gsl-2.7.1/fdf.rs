use std::fmt;
use std::error::Error;
use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};
use libc::{c_int, c_double, c_void};
use gsl_sys;

#[derive(Debug)]
pub enum NLinearError {
    InsufficientData,
    AllocationFailed,
    BadLength,
    NoProgress,
    MaxIterations,
    Tolerance,
    Other(i32),
}

impl fmt::Display for NLinearError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NLinearError::InsufficientData => write!(f, "insufficient data points, n < p"),
            NLinearError::AllocationFailed => write!(f, "failed to allocate space"),
            NLinearError::BadLength => write!(f, "vector/matrix length mismatch"),
            NLinearError::NoProgress => write!(f, "no progress on first iteration"),
            NLinearError::MaxIterations => write!(f, "maximum iterations reached"),
            NLinearError::Tolerance => write!(f, "tolerance reached machine precision"),
            NLinearError::Other(code) => write!(f, "GSL error code {}", code),
        }
    }
}

impl Error for NLinearError {}

pub type NLinearResult<T> = Result<T, NLinearError>;

pub struct NLinearWorkspace {
    x: Vec<f64>,
    f: Vec<f64>,
    dx: Vec<f64>,
    g: Vec<f64>,
    J: Vec<Vec<f64>>,
    sqrt_wts_work: Vec<f64>,
    sqrt_wts: Option<Vec<f64>>,
    niter: usize,
    params: NLinearParameters,
    fdf: Option<NLinearFdf>,
    state: Box<dyn NLinearState>,
}

pub struct NLinearParameters {
    pub trs: NLinearTrsType,
    pub scale: NLinearScaleType,
    pub solver: NLinearSolverType,
    pub fdtype: NLinearFdType,
    pub factor_up: f64,
    pub factor_down: f64,
    pub avmax: f64,
    pub h_df: f64,
    pub h_fvv: f64,
}

pub struct NLinearFdf {
    pub f: Option<fn(&[f64], &mut [f64]) -> i32>,
    pub df: Option<fn(&[f64], &mut [f64]) -> i32>,
    pub fvv: Option<fn(&[f64], &[f64], &mut [f64]) -> i32>,
    pub n: usize,
    pub p: usize,
    pub params: Box<dyn Any>,
    pub nevalf: usize,
    pub nevaldf: usize,
    pub nevalfvv: usize,
}

pub trait NLinearState {
    fn alloc(params: &NLinearParameters, n: usize, p: usize) -> Box<dyn NLinearState>;
    fn free(&mut self);
    fn init(&mut self, swts: Option<&[f64]>, fdf: &NLinearFdf, x: &[f64], f: &mut [f64], 
            J: &mut [Vec<f64>], g: &mut [f64]) -> i32;
    fn iterate(&mut self, swts: Option<&[f64]>, fdf: &NLinearFdf, x: &mut [f64], 
               f: &mut [f64], J: &mut [Vec<f64>], g: &mut [f64], dx: &mut [f64]) -> i32;
    fn avratio(&self) -> f64;
    fn rcond(&self, rcond: &mut f64) -> i32;
    fn name(&self) -> &'static str;
}

impl NLinearWorkspace {
    pub fn new(T: &dyn NLinearState, params: &NLinearParameters, n: usize, p: usize) -> NLinearResult<Self> {
        if n < p {
            return Err(NLinearError::InsufficientData);
        }

        let x = vec![0.0; p];
        let f = vec![0.0; n];
        let dx = vec![0.0; p];
        let g = vec![0.0; p];
        let J = vec![vec![0.0; p]; n];
        let sqrt_wts_work = vec![0.0; n];

        let state = T.alloc(params, n, p);

        Ok(NLinearWorkspace {
            x,
            f,
            dx,
            g,
            J,
            sqrt_wts_work,
            sqrt_wts: None,
            niter: 0,
            params: params.clone(),
            fdf: None,
            state,
        })
    }

    pub fn free(&mut self) {
        self.state.free();
    }

    pub fn default_parameters() -> NLinearParameters {
        NLinearParameters {
            trs: NLinearTrsType::Lm,
            scale: NLinearScaleType::More,
            solver: NLinearSolverType::Qr,
            fdtype: NLinearFdType::FwdDiff,
            factor_up: 3.0,
            factor_down: 2.0,
            avmax: 0.75,
            h_df: f64::EPSILON.sqrt(),
            h_fvv: 0.02,
        }
    }

    pub fn init(&mut self, x: &[f64], fdf: NLinearFdf) -> NLinearResult<()> {
        self.winit(x, None, fdf)
    }

    pub fn winit(&mut self, x: &[f64], wts: Option<&[f64]>, fdf: NLinearFdf) -> NLinearResult<()> {
        let n = self.f.len();
        
        if n != fdf.n {
            return Err(NLinearError::BadLength);
        } else if self.x.len() != x.len() {
            return Err(NLinearError::BadLength);
        } else if let Some(wts) = wts {
            if n != wts.len() {
                return Err(NLinearError::BadLength);
            }
        }

        fdf.nevalf = 0;
        fdf.nevaldf = 0;
        fdf.nevalfvv = 0;

        self.fdf = Some(fdf);
        self.x.copy_from_slice(x);
        self.niter = 0;

        if let Some(wts) = wts {
            self.sqrt_wts = Some(self.sqrt_wts_work.clone());
            for (i, &wi) in wts.iter().enumerate() {
                self.sqrt_wts_work[i] = wi.sqrt();
            }
        } else {
            self.sqrt_wts = None;
        }

        let status = self.state.init(
            self.sqrt_wts.as_deref(),
            self.fdf.as_ref().unwrap(),
            &self.x,
            &mut self.f,
            &mut self.J,
            &mut self.g,
        );

        if status != 0 {
            Err(NLinearError::Other(status))
        } else {
            Ok(())
        }
    }

    pub fn iterate(&mut self) -> NLinearResult<()> {
        let status = self.state.iterate(
            self.sqrt_wts.as_deref(),
            self.fdf.as_ref().unwrap(),
            &mut self.x,
            &mut self.f,
            &mut self.J,
            &mut self.g,
            &mut self.dx,
        );

        self.niter += 1;

        if status != 0 {
            Err(NLinearError::Other(status))
        } else {
            Ok(())
        }
    }

    pub fn avratio(&self) -> f64 {
        self.state.avratio()
    }

    pub fn driver(
        &mut self,
        maxiter: usize,
        xtol: f64,
        gtol: f64,
        ftol: f64,
        callback: Option<fn(usize, &NLinearWorkspace)>,
    ) -> NLinearResult<usize> {
        let mut info = 0;

        if let Some(cb) = callback {
            cb(0, self);
        }

        for iter in 1..=maxiter {
            let status = self.iterate();
            
            if let Err(NLinearError::Other(GSL_ENOPROG)) = status {
                if iter == 1 {
                    return Err(NLinearError::NoProgress);
                }
            } else if status.is_err() {
                return status.map(|_| unreachable!());
            }

            if let Some(cb) = callback {
                cb(iter, self);
            }

            let test_status = self.test(xtol, gtol, ftol, &mut info);
            
            match test_status {
                Ok(()) => return Ok(info),
                Err(NLinearError::Tolerance) => return Ok(info),
                _ => (),
            }
        }

        Err(NLinearError::MaxIterations)
    }

    pub fn jac(&self) -> &[Vec<f64>] {
        &self.J
    }

    pub fn name(&self) -> &'static str {
        self.state.name()
    }

    pub fn position(&self) -> &[f64] {
        &self.x
    }

    pub fn residual(&self) -> &[f64] {
        &self.f
    }

    pub fn niter(&self) -> usize {
        self.niter
    }

    pub fn rcond(&self) -> NLinearResult<f64> {
        let mut rcond = 0.0;
        let status = self.state.rcond(&mut rcond);
        if status != 0 {
            Err(NLinearError::Other(status))
        } else {
            Ok(rcond)
        }
    }

    pub fn trs_name(&self) -> &'static str {
        self.params.trs.name()
    }

    fn test(&self, xtol: f64, gtol: f64, ftol: f64, info: &mut i32) -> NLinearResult<()> {
        // Implementation of convergence tests
        unimplemented!()
    }
}

pub fn eval_f(
    fdf: &NLinearFdf,
    x: &[f64],
    swts: Option<&[f64]>,
    y: &mut [f64],
) -> NLinearResult<()> {
    if let Some(f) = fdf.f {
        let status = f(x, y);
        if status != 0 {
            return Err(NLinearError::Other(status));
        }
    }

    fdf.nevalf += 1;

    if let Some(swts) = swts {
        for (yi, &swi) in y.iter_mut().zip(swts) {
            *yi *= swi;
        }
    }

    Ok(())
}

pub fn eval_df(
    x: &[f64],
    f: &[f64],
    swts: Option<&[f64]>,
    h: f64,
    fdtype: NLinearFdType,
    fdf: &NLinearFdf,
    df: &mut [Vec<f64>],
    work: &mut [f64],
) -> NLinearResult<()> {
    if let Some(df_func) = fdf.df {
        let status = df_func(x, work);
        if status != 0 {
            return Err(NLinearError::Other(status));
        }

        fdf.nevaldf += 1;

        if let Some(swts) = swts {
            for (i, &swi) in swts.iter().enumerate() {
                for j in 0..df[i].len() {
                    df[i][j] *= swi;
                }
            }
        }
    } else {
        // Implement finite difference
        unimplemented!()
    }

    Ok(())
}

pub fn eval_fvv(
    h: f64,
    x: &[f64],
    v: &[f64],
    f: &[f64],
    J: &[Vec<f64>],
    swts: Option<&[f64]>,
    fdf: &NLinearFdf,
    yvv: &mut [f64],
    work: &mut [f64],
) -> NLinearResult<()> {
    if let Some(fvv) = fdf.fvv {
        let status = fvv(x, v, yvv);
        if status != 0 {
            return Err(NLinearError::Other(status));
        }

        fdf.nevalfvv += 1;

        if let Some(swts) = swts {
            for (yi, &swi) in yvv.iter_mut().zip(swts) {
                *yi *= swi;
            }
        }
    } else {
        // Implement finite difference
        unimplemented!()
    }

    Ok(())
}