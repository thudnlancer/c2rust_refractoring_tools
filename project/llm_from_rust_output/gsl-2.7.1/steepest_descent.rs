use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::mem;

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
    Singular = 21,
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

#[derive(Debug)]
pub struct GslBlock {
    size: size_t,
    data: Vec<c_double>,
}

impl GslBlock {
    pub fn new(size: size_t) -> Self {
        GslBlock {
            size,
            data: vec![0.0; size],
        }
    }
}

#[derive(Debug)]
pub struct GslVector {
    size: size_t,
    stride: size_t,
    data: Vec<c_double>,
    block: Option<Box<GslBlock>>,
    owner: bool,
}

impl GslVector {
    pub fn new(size: size_t) -> Self {
        GslVector {
            size,
            stride: 1,
            data: vec![0.0; size],
            block: None,
            owner: true,
        }
    }

    pub fn set_zero(&mut self) {
        self.data.iter_mut().for_each(|x| *x = 0.0);
    }

    pub fn memcpy(&mut self, src: &GslVector) -> Result<(), GslError> {
        if self.size != src.size {
            return Err(GslError::BadLen);
        }
        self.data.copy_from_slice(&src.data);
        Ok(())
    }

    pub fn equal(&self, other: &GslVector) -> bool {
        self.size == other.size && self.data == other.data
    }

    pub fn dnrm2(&self) -> c_double {
        self.data.iter().map(|x| x * x).sum::<c_double>().sqrt()
    }

    pub fn daxpy(&mut self, alpha: c_double, x: &GslVector) -> Result<(), GslError> {
        if self.size != x.size {
            return Err(GslError::BadLen);
        }
        for (y, xi) in self.data.iter_mut().zip(x.data.iter()) {
            *y += alpha * xi;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct GslMultiminFunctionFdf {
    pub f: Box<dyn Fn(&GslVector, &mut c_void) -> c_double>,
    pub df: Box<dyn Fn(&GslVector, &mut c_void, &mut GslVector)>,
    pub fdf: Box<dyn Fn(&GslVector, &mut c_void, &mut c_double, &mut GslVector)>,
    pub n: size_t,
    pub params: Box<c_void>,
}

pub struct SteepestDescentState {
    step: c_double,
    max_step: c_double,
    tol: c_double,
    x1: GslVector,
    g1: GslVector,
}

impl SteepestDescentState {
    pub fn new(n: size_t) -> Result<Self, GslError> {
        Ok(SteepestDescentState {
            step: 0.0,
            max_step: 0.0,
            tol: 0.0,
            x1: GslVector::new(n),
            g1: GslVector::new(n),
        })
    }
}

pub struct SteepestDescentMinimizer {
    state: SteepestDescentState,
}

impl SteepestDescentMinimizer {
    pub fn new(n: size_t) -> Result<Self, GslError> {
        Ok(SteepestDescentMinimizer {
            state: SteepestDescentState::new(n)?,
        })
    }

    pub fn set(
        &mut self,
        fdf: &mut GslMultiminFunctionFdf,
        x: &GslVector,
        f: &mut c_double,
        gradient: &mut GslVector,
        step_size: c_double,
        tol: c_double,
    ) -> Result<(), GslError> {
        (fdf.fdf)(x, &mut *fdf.params, f, gradient);
        self.state.step = step_size;
        self.state.max_step = step_size;
        self.state.tol = tol;
        Ok(())
    }

    pub fn iterate(
        &mut self,
        fdf: &mut GslMultiminFunctionFdf,
        x: &mut GslVector,
        f: &mut c_double,
        gradient: &mut GslVector,
        dx: &mut GslVector,
    ) -> Result<(), GslError> {
        let f0 = *f;
        let mut f1 = 0.0;
        let mut step = self.state.step;
        let tol = self.state.tol;
        let mut failed = false;

        let gnorm = gradient.dnrm2();
        if gnorm == 0.0 {
            dx.set_zero();
            return Err(GslError::NoProgress);
        }

        loop {
            dx.set_zero();
            dx.daxpy(-step / gnorm, gradient)?;
            self.state.x1.memcpy(x)?;
            self.state.x1.daxpy(1.0, dx)?;

            if x.equal(&self.state.x1) {
                return Err(GslError::NoProgress);
            }

            (fdf.fdf)(&self.state.x1, &mut *fdf.params, &mut f1, &mut self.state.g1);

            if f1 <= f0 {
                break;
            }
            failed = true;
            step *= tol;
        }

        if failed {
            step *= tol;
        } else {
            step *= 2.0;
        }

        self.state.step = step;
        x.memcpy(&self.state.x1)?;
        gradient.memcpy(&self.state.g1)?;
        *f = f1;

        Ok(())
    }

    pub fn restart(&mut self) {
        self.state.step = self.state.max_step;
    }
}

pub static GSL_MULTIMIN_FDFMINIMIZER_STEEPEST_DESCENT: &str = "steepest_descent";

fn gsl_error(reason: &str, file: &str, line: c_int, gsl_errno: GslError) {
    eprintln!(
        "GSL error: {} at {}:{} (code: {:?})",
        reason, file, line, gsl_errno
    );
}