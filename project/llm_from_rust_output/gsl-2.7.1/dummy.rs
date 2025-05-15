use std::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
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

#[derive(Debug, Clone)]
pub struct GslBlock {
    pub size: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: usize,
    pub stride: usize,
    pub data: Vec<f64>,
    pub block: Option<Box<GslBlock>>,
    pub owner: bool,
}

#[derive(Debug, Clone)]
pub struct GslMatrix {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: Vec<f64>,
    pub block: Option<Box<GslBlock>>,
    pub owner: bool,
}

pub trait MultilargeNlinearSolver {
    fn name(&self) -> &str;
    fn alloc(&self, n: usize, p: usize) -> Option<Box<dyn std::any::Any>>;
    fn init(&self, trust_state: &dyn std::any::Any, state: &mut dyn std::any::Any) -> GslError;
    fn presolve(&self, mu: f64, trust_state: &dyn std::any::Any, state: &mut dyn std::any::Any) -> GslError;
    fn solve(&self, g: &GslVector, x: &mut GslVector, trust_state: &dyn std::any::Any, state: &mut dyn std::any::Any) -> GslError;
    fn rcond(&self, rcond: &mut f64, jtj: &GslMatrix, state: &mut dyn std::any::Any) -> GslError;
    fn covar(&self, jtj: &GslMatrix, covar: &mut GslMatrix, state: &mut dyn std::any::Any) -> GslError;
}

pub struct DummySolver;

impl MultilargeNlinearSolver for DummySolver {
    fn name(&self) -> &str {
        "dummy"
    }

    fn alloc(&self, _n: usize, _p: usize) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn init(&self, _trust_state: &dyn std::any::Any, _state: &mut dyn std::any::Any) -> GslError {
        GslError::Success
    }

    fn presolve(&self, _mu: f64, _trust_state: &dyn std::any::Any, _state: &mut dyn std::any::Any) -> GslError {
        GslError::Success
    }

    fn solve(&self, _g: &GslVector, _x: &mut GslVector, _trust_state: &dyn std::any::Any, _state: &mut dyn std::any::Any) -> GslError {
        GslError::Success
    }

    fn rcond(&self, rcond: &mut f64, _jtj: &GslMatrix, _state: &mut dyn std::any::Any) -> GslError {
        *rcond = 0.0;
        GslError::Success
    }

    fn covar(&self, _jtj: &GslMatrix, covar: &mut GslMatrix, _state: &mut dyn std::any::Any) -> GslError {
        covar.data.iter_mut().for_each(|x| *x = 0.0);
        GslError::Success
    }
}

pub static GSL_MULTILARGE_NLINEAR_SOLVER_NONE: DummySolver = DummySolver;