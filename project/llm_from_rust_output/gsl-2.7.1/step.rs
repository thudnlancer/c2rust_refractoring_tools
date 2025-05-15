use std::ffi::CString;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Clone)]
pub struct GslOdeivSystem {
    pub function: Box<dyn Fn(f64, &[f64], &mut [f64], &mut dyn std::any::Any) -> Result<(), GslError>>,
    pub jacobian: Option<Box<dyn Fn(f64, &[f64], &mut [f64], &mut [f64], &mut dyn std::any::Any) -> Result<(), GslError>>>,
    pub dimension: usize,
    pub params: Box<dyn std::any::Any>,
}

pub struct GslOdeivStepType {
    pub name: CString,
    pub can_use_dydt_in: bool,
    pub gives_exact_dydt_out: bool,
    pub alloc: Box<dyn Fn(usize) -> Result<Box<dyn std::any::Any>, GslError>>,
    pub apply: Box<dyn Fn(&mut dyn std::any::Any, usize, f64, f64, &mut [f64], &mut [f64], Option<&[f64]>, &mut [f64], &GslOdeivSystem) -> Result<(), GslError>>,
    pub reset: Box<dyn Fn(&mut dyn std::any::Any, usize) -> Result<(), GslError>>,
    pub order: Box<dyn Fn(&dyn std::any::Any) -> Result<u32, GslError>>,
}

pub struct GslOdeivStep {
    pub step_type: &'static GslOdeivStepType,
    pub dimension: usize,
    pub state: Box<dyn std::any::Any>,
}

impl GslOdeivStep {
    pub fn new(step_type: &'static GslOdeivStepType, dim: usize) -> Result<Self, GslError> {
        let state = (step_type.alloc)(dim)?;
        Ok(Self {
            step_type,
            dimension: dim,
            state,
        })
    }

    pub fn name(&self) -> &str {
        self.step_type.name.to_str().unwrap()
    }

    pub fn order(&self) -> Result<u32, GslError> {
        (self.step_type.order)(&*self.state)
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: &mut [f64],
        dydt: &GslOdeivSystem,
    ) -> Result<(), GslError> {
        (self.step_type.apply)(&mut *self.state, self.dimension, t, h, y, yerr, dydt_in, dydt_out, dydt)
    }

    pub fn reset(&mut self) -> Result<(), GslError> {
        (self.step_type.reset)(&mut *self.state, self.dimension)
    }
}

impl Drop for GslOdeivStep {
    fn drop(&mut self) {
        // The free function is handled by the Box's Drop implementation
    }
}