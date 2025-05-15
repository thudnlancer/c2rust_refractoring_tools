use std::ffi::CStr;
use std::os::raw::{c_double, c_int, c_void};

pub type size_t = usize;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singularity = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Clone)]
pub struct GslFunctionFdf {
    pub f: Box<dyn Fn(f64) -> f64>,
    pub df: Box<dyn Fn(f64) -> f64>,
    pub fdf: Box<dyn Fn(f64) -> (f64, f64)>,
}

#[derive(Debug, Clone)]
struct SecantState {
    f: f64,
    df: f64,
}

pub struct SecantSolver {
    state: SecantState,
    solver_type: SecantSolverType,
}

#[derive(Debug, Clone)]
struct SecantSolverType {
    name: &'static str,
    size: usize,
}

impl SecantSolver {
    pub fn new() -> Self {
        SecantSolver {
            state: SecantState { f: 0.0, df: 0.0 },
            solver_type: SecantSolverType {
                name: "secant",
                size: std::mem::size_of::<SecantState>(),
            },
        }
    }

    pub fn init(&mut self, fdf: &GslFunctionFdf, root: &mut f64) -> Result<(), GslError> {
        let x = *root;
        let (f, df) = (fdf.fdf)(x);
        self.state.f = f;
        self.state.df = df;
        Ok(())
    }

    pub fn iterate(&mut self, fdf: &GslFunctionFdf, root: &mut f64) -> Result<(), GslError> {
        let x = *root;
        let f = self.state.f;
        let df = self.state.df;

        if f == 0.0 {
            return Ok(());
        }

        if df == 0.0 {
            return Err(GslError::ZeroDiv);
        }

        let x_new = x - f / df;
        let f_new = (fdf.f)(x_new);
        let df_new = df * ((f - f_new) / f);

        *root = x_new;
        self.state.f = f_new;
        self.state.df = df_new;

        if !f_new.is_finite() {
            return Err(GslError::BadFunc);
        }

        if !df_new.is_finite() {
            return Err(GslError::BadFunc);
        }

        Ok(())
    }
}

pub static GSL_ROOT_FDFSOLVER_SECANT: SecantSolverType = SecantSolverType {
    name: "secant",
    size: std::mem::size_of::<SecantState>(),
};