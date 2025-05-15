use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_void};

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

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn new<F: Fn(f64) -> f64 + 'static>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FalsePosState {
    f_lower: f64,
    f_upper: f64,
}

pub struct FalsePosSolver {
    state: FalsePosState,
    solver_type: FalsePosSolverType,
}

impl FalsePosSolver {
    pub fn new() -> Self {
        FalsePosSolver {
            state: FalsePosState {
                f_lower: 0.0,
                f_upper: 0.0,
            },
            solver_type: FalsePosSolverType::new(),
        }
    }

    pub fn init(
        &mut self,
        f: &GslFunction,
        x_lower: f64,
        x_upper: f64,
    ) -> Result<f64, GslError> {
        let root = 0.5 * (x_lower + x_upper);
        let f_lower = (f.function)(x_lower);
        if !f_lower.is_finite() {
            return Err(GslError::BadFunc);
        }

        let f_upper = (f.function)(x_upper);
        if !f_upper.is_finite() {
            return Err(GslError::BadFunc);
        }

        self.state.f_lower = f_lower;
        self.state.f_upper = f_upper;

        if (f_lower < 0.0 && f_upper < 0.0) || (f_lower > 0.0 && f_upper > 0.0) {
            return Err(GslError::Invalid);
        }

        Ok(root)
    }

    pub fn iterate(
        &mut self,
        f: &GslFunction,
        x_lower: &mut f64,
        x_upper: &mut f64,
    ) -> Result<f64, GslError> {
        let mut x_left = *x_lower;
        let mut x_right = *x_upper;
        let mut f_lower = self.state.f_lower;
        let mut f_upper = self.state.f_upper;

        if f_lower == 0.0 {
            *x_upper = x_left;
            return Ok(x_left);
        }

        if f_upper == 0.0 {
            *x_lower = x_right;
            return Ok(x_right);
        }

        let x_linear = x_right - f_upper * (x_left - x_right) / (f_lower - f_upper);
        let f_linear = (f.function)(x_linear);
        if !f_linear.is_finite() {
            return Err(GslError::BadFunc);
        }

        if f_linear == 0.0 {
            *x_lower = x_linear;
            *x_upper = x_linear;
            return Ok(x_linear);
        }

        let root;
        if (f_lower > 0.0 && f_linear < 0.0) || (f_lower < 0.0 && f_linear > 0.0) {
            root = x_linear;
            *x_upper = x_linear;
            self.state.f_upper = f_linear;
        } else {
            root = x_linear;
            *x_lower = x_linear;
            self.state.f_lower = f_linear;
        }

        let w = if *x_upper == x_linear {
            x_linear - x_left
        } else {
            x_right - x_linear
        };

        if w < 0.5 * (x_right - x_left) {
            return Ok(root);
        }

        let x_bisect = 0.5 * (x_left + x_right);
        let f_bisect = (f.function)(x_bisect);
        if !f_bisect.is_finite() {
            return Err(GslError::BadFunc);
        }

        if (f_lower > 0.0 && f_bisect < 0.0) || (f_lower < 0.0 && f_bisect > 0.0) {
            *x_upper = x_bisect;
            self.state.f_upper = f_bisect;
            if root > x_bisect {
                return Ok(0.5 * (x_left + x_bisect));
            }
        } else {
            *x_lower = x_bisect;
            self.state.f_lower = f_bisect;
            if root < x_bisect {
                return Ok(0.5 * (x_bisect + x_right));
            }
        }

        Ok(root)
    }
}

#[derive(Debug, Clone, Copy)]
struct FalsePosSolverType {
    name: &'static str,
    size: size_t,
}

impl FalsePosSolverType {
    fn new() -> Self {
        FalsePosSolverType {
            name: "falsepos",
            size: std::mem::size_of::<FalsePosState>(),
        }
    }
}

pub static GSL_ROOT_FSOLVER_FALSEPOS: FalsePosSolverType = FalsePosSolverType {
    name: "falsepos",
    size: std::mem::size_of::<FalsePosState>(),
};