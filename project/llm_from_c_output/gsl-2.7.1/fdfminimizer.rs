use std::error::Error;
use std::fmt;
use std::ptr::NonNull;

#[derive(Debug)]
pub enum MinimizerError {
    AllocationFailed,
    InitializationFailed,
    IncompatibleSize,
}

impl fmt::Display for MinimizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MinimizerError::AllocationFailed => write!(f, "failed to allocate space"),
            MinimizerError::InitializationFailed => write!(f, "failed to initialize minimizer state"),
            MinimizerError::IncompatibleSize => write!(f, "incompatible sizes"),
        }
    }
}

impl Error for MinimizerError {}

pub struct FdfMinimizer<'a, T: MinimizerType> {
    type_: &'a T,
    x: Vec<f64>,
    gradient: Vec<f64>,
    dx: Vec<f64>,
    state: T::State,
    f: f64,
    fdf: Option<Box<dyn FdfFunction>>,
}

pub trait MinimizerType {
    type State;
    fn alloc(&self, n: usize) -> Result<Self::State, MinimizerError>;
    fn set(
        &self,
        state: &mut Self::State,
        fdf: &dyn FdfFunction,
        x: &[f64],
        f: &mut f64,
        gradient: &mut [f64],
        step_size: f64,
        tol: f64,
    ) -> Result<(), MinimizerError>;
    fn iterate(
        &self,
        state: &mut Self::State,
        fdf: &dyn FdfFunction,
        x: &mut [f64],
        f: &mut f64,
        gradient: &mut [f64],
        dx: &mut [f64],
    ) -> Result<(), MinimizerError>;
    fn restart(&self, state: &mut Self::State) -> Result<(), MinimizerError>;
    fn free(&self, state: Self::State);
    fn name(&self) -> &'static str;
}

pub trait FdfFunction {
    fn n(&self) -> usize;
    fn value(&self, x: &[f64]) -> f64;
    fn gradient(&self, x: &[f64], grad: &mut [f64]);
    fn value_and_gradient(&self, x: &[f64], grad: &mut [f64]) -> f64;
}

impl<'a, T: MinimizerType> FdfMinimizer<'a, T> {
    pub fn new(type_: &'a T, n: usize) -> Result<Self, MinimizerError> {
        let state = type_.alloc(n)?;
        
        Ok(Self {
            type_,
            x: vec![0.0; n],
            gradient: vec![0.0; n],
            dx: vec![0.0; n],
            state,
            f: 0.0,
            fdf: None,
        })
    }

    pub fn set(
        &mut self,
        fdf: Box<dyn FdfFunction>,
        x: &[f64],
        step_size: f64,
        tol: f64,
    ) -> Result<(), MinimizerError> {
        if self.x.len() != fdf.n() {
            return Err(MinimizerError::IncompatibleSize);
        }
        
        if x.len() != fdf.n() {
            return Err(MinimizerError::IncompatibleSize);
        }

        self.fdf = Some(fdf);
        self.x.copy_from_slice(x);
        self.dx.fill(0.0);

        self.type_.set(
            &mut self.state,
            self.fdf.as_ref().unwrap(),
            &self.x,
            &mut self.f,
            &mut self.gradient,
            step_size,
            tol,
        )
    }

    pub fn iterate(&mut self) -> Result<(), MinimizerError> {
        self.type_.iterate(
            &mut self.state,
            self.fdf.as_ref().unwrap(),
            &mut self.x,
            &mut self.f,
            &mut self.gradient,
            &mut self.dx,
        )
    }

    pub fn restart(&mut self) -> Result<(), MinimizerError> {
        self.type_.restart(&mut self.state)
    }

    pub fn name(&self) -> &'static str {
        self.type_.name()
    }

    pub fn x(&self) -> &[f64] {
        &self.x
    }

    pub fn dx(&self) -> &[f64] {
        &self.dx
    }

    pub fn gradient(&self) -> &[f64] {
        &self.gradient
    }

    pub fn minimum(&self) -> f64 {
        self.f
    }
}

impl<'a, T: MinimizerType> Drop for FdfMinimizer<'a, T> {
    fn drop(&mut self) {
        self.type_.free(std::mem::take(&mut self.state));
    }
}