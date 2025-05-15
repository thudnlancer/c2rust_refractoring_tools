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

#[derive(Debug, Clone, Copy)]
pub struct GslFunctionFdf {
    pub f: Option<fn(f64, &mut c_void) -> f64>,
    pub df: Option<fn(f64, &mut c_void) -> f64>,
    pub fdf: Option<fn(f64, &mut c_void, &mut f64, &mut f64)>,
    pub params: Box<c_void>,
}

#[derive(Debug, Clone, Copy)]
pub struct SteffensonState {
    f: f64,
    df: f64,
    x: f64,
    x_1: f64,
    x_2: f64,
    count: i32,
}

pub struct SteffensonSolver {
    state: SteffensonState,
}

impl SteffensonSolver {
    pub fn new() -> Self {
        SteffensonSolver {
            state: SteffensonState {
                f: 0.0,
                df: 0.0,
                x: 0.0,
                x_1: 0.0,
                x_2: 0.0,
                count: 0,
            },
        }
    }

    pub fn init(&mut self, fdf: &GslFunctionFdf, root: &mut f64) -> Result<(), GslError> {
        let x = *root;
        self.state.f = fdf.f.ok_or(GslError::BadFunc)?(x, &mut *fdf.params);
        self.state.df = fdf.df.ok_or(GslError::BadFunc)?(x, &mut *fdf.params);
        self.state.x = x;
        self.state.x_1 = 0.0;
        self.state.x_2 = 0.0;
        self.state.count = 1;
        Ok(())
    }

    pub fn iterate(&mut self, fdf: &GslFunctionFdf, root: &mut f64) -> Result<(), GslError> {
        let x = self.state.x;
        let x_1 = self.state.x_1;

        if self.state.df == 0.0 {
            return Err(GslError::ZeroDiv);
        }

        let x_new = x - self.state.f / self.state.df;
        let (mut f_new, mut df_new) = (0.0, 0.0);
        fdf.fdf.ok_or(GslError::BadFunc)?(x_new, &mut *fdf.params, &mut f_new, &mut df_new);

        self.state.x_2 = x_1;
        self.state.x_1 = x;
        self.state.x = x_new;
        self.state.f = f_new;
        self.state.df = df_new;

        if !f_new.is_finite() {
            return Err(GslError::BadFunc);
        }

        if self.state.count < 3 {
            *root = x_new;
            self.state.count += 1;
        } else {
            let u = x - x_1;
            let v = x_new - 2.0 * x + x_1;
            *root = if v == 0.0 { x_new } else { x_1 - u * u / v };
        }

        if !df_new.is_finite() {
            return Err(GslError::BadFunc);
        }

        Ok(())
    }
}

pub struct GslRootFdfSolverType {
    pub name: &'static str,
    pub size: usize,
    pub solver: SteffensonSolver,
}

impl GslRootFdfSolverType {
    pub fn new() -> Self {
        GslRootFdfSolverType {
            name: "steffenson",
            size: std::mem::size_of::<SteffensonState>(),
            solver: SteffensonSolver::new(),
        }
    }
}

pub static GSL_ROOT_FDFSOLVER_STEFFENSON: GslRootFdfSolverType = GslRootFdfSolverType::new();