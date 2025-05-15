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

impl GslError {
    fn from_code(code: c_int) -> Option<Self> {
        match code {
            0 => Some(GslError::Success),
            -1 => Some(GslError::Failure),
            -2 => Some(GslError::Continue),
            1 => Some(GslError::Domain),
            2 => Some(GslError::Range),
            3 => Some(GslError::Fault),
            4 => Some(GslError::Invalid),
            5 => Some(GslError::Failed),
            6 => Some(GslError::Factor),
            7 => Some(GslError::Sanity),
            8 => Some(GslError::NoMem),
            9 => Some(GslError::BadFunc),
            10 => Some(GslError::Runaway),
            11 => Some(GslError::MaxIter),
            12 => Some(GslError::ZeroDiv),
            13 => Some(GslError::BadTol),
            14 => Some(GslError::Tol),
            15 => Some(GslError::Underflow),
            16 => Some(GslError::Overflow),
            17 => Some(GslError::Loss),
            18 => Some(GslError::Round),
            19 => Some(GslError::BadLen),
            20 => Some(GslError::NotSquare),
            21 => Some(GslError::Singularity),
            22 => Some(GslError::Diverge),
            23 => Some(GslError::Unsupported),
            24 => Some(GslError::Unimplemented),
            25 => Some(GslError::Cache),
            26 => Some(GslError::Table),
            27 => Some(GslError::NoProgress),
            28 => Some(GslError::NoProgressJ),
            29 => Some(GslError::TolF),
            30 => Some(GslError::TolX),
            31 => Some(GslError::TolG),
            32 => Some(GslError::Eof),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GslFunction {
    function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn new<F: Fn(f64) -> f64 + 'static>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }

    fn evaluate(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

#[derive(Debug, Clone)]
struct BisectionState {
    f_lower: f64,
    f_upper: f64,
}

#[derive(Debug, Clone)]
pub struct BisectionSolver {
    state: BisectionState,
    solver_type: BisectionSolverType,
}

#[derive(Debug, Clone)]
struct BisectionSolverType {
    name: &'static str,
    size: usize,
}

impl BisectionSolver {
    pub fn new() -> Self {
        BisectionSolver {
            state: BisectionState {
                f_lower: 0.0,
                f_upper: 0.0,
            },
            solver_type: BisectionSolverType {
                name: "bisection",
                size: std::mem::size_of::<BisectionState>(),
            },
        }
    }

    pub fn init(
        &mut self,
        f: &GslFunction,
        x_lower: f64,
        x_upper: f64,
    ) -> Result<(f64, f64, f64), GslError> {
        let root = 0.5 * (x_lower + x_upper);
        let f_lower = f.evaluate(x_lower);
        let f_upper = f.evaluate(x_upper);

        if !f_lower.is_finite() || !f_upper.is_finite() {
            return Err(GslError::BadFunc);
        }

        self.state.f_lower = f_lower;
        self.state.f_upper = f_upper;

        if (f_lower < 0.0 && f_upper < 0.0) || (f_lower > 0.0 && f_upper > 0.0) {
            return Err(GslError::Invalid);
        }

        Ok((root, x_lower, x_upper))
    }

    pub fn iterate(
        &mut self,
        f: &GslFunction,
        x_lower: f64,
        x_upper: f64,
    ) -> Result<(f64, f64, f64), GslError> {
        let f_lower = self.state.f_lower;
        let f_upper = self.state.f_upper;

        if f_lower == 0.0 {
            return Ok((x_lower, x_lower, x_lower));
        }
        if f_upper == 0.0 {
            return Ok((x_upper, x_upper, x_upper));
        }

        let x_bisect = (x_lower + x_upper) / 2.0;
        let f_bisect = f.evaluate(x_bisect);

        if !f_bisect.is_finite() {
            return Err(GslError::BadFunc);
        }

        if f_bisect == 0.0 {
            return Ok((x_bisect, x_bisect, x_bisect));
        }

        let (new_root, new_x_lower, new_x_upper) = if (f_lower > 0.0 && f_bisect < 0.0)
            || (f_lower < 0.0 && f_bisect > 0.0)
        {
            let root = 0.5 * (x_lower + x_bisect);
            self.state.f_upper = f_bisect;
            (root, x_lower, x_bisect)
        } else {
            let root = 0.5 * (x_bisect + x_upper);
            self.state.f_lower = f_bisect;
            (root, x_bisect, x_upper)
        };

        Ok((new_root, new_x_lower, new_x_upper))
    }
}

pub static GSL_ROOT_FSOLVER_BISECTION: BisectionSolverType = BisectionSolverType {
    name: "bisection",
    size: std::mem::size_of::<BisectionState>(),
};