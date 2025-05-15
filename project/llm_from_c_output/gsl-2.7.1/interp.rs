use std::error::Error;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum InterpError {
    InsufficientPoints,
    AllocationFailed,
    InvalidData,
    DomainError,
    Other(String),
}

impl fmt::Display for InterpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpError::InsufficientPoints => write!(f, "insufficient number of points for interpolation type"),
            InterpError::AllocationFailed => write!(f, "failed to allocate space"),
            InterpError::InvalidData => write!(f, "invalid data"),
            InterpError::DomainError => write!(f, "interpolation error"),
            InterpError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for InterpError {}

pub type InterpResult<T> = Result<T, InterpError>;

pub trait InterpType {
    fn min_size(&self) -> usize;
    fn name(&self) -> &str;
    fn alloc(&self, size: usize) -> InterpResult<Box<dyn std::any::Any>>;
    fn init(&self, state: &mut dyn std::any::Any, x_array: &[f64], y_array: &[f64], size: usize) -> InterpResult<()>;
    fn eval(&self, state: &dyn std::any::Any, x_array: &[f64], y_array: &[f64], size: usize, x: f64, accel: Option<&dyn std::any::Any>, y: &mut f64) -> InterpResult<()>;
    fn eval_deriv(&self, state: &dyn std::any::Any, x_array: &[f64], y_array: &[f64], size: usize, x: f64, accel: Option<&dyn std::any::Any>, dydx: &mut f64) -> InterpResult<()>;
    fn eval_deriv2(&self, state: &dyn std::any::Any, x_array: &[f64], y_array: &[f64], size: usize, x: f64, accel: Option<&dyn std::any::Any>, d2: &mut f64) -> InterpResult<()>;
    fn eval_integ(&self, state: &dyn std::any::Any, x_array: &[f64], y_array: &[f64], size: usize, accel: Option<&dyn std::any::Any>, a: f64, b: f64, result: &mut f64) -> InterpResult<()>;
}

pub struct Interp {
    interp_type: Box<dyn InterpType>,
    size: usize,
    xmin: f64,
    xmax: f64,
    state: Box<dyn std::any::Any>,
}

impl Interp {
    pub fn new(interp_type: Box<dyn InterpType>, size: usize) -> InterpResult<Self> {
        if size < interp_type.min_size() {
            return Err(InterpError::InsufficientPoints);
        }

        let state = interp_type.alloc(size)?;

        Ok(Interp {
            interp_type,
            size,
            xmin: 0.0,
            xmax: 0.0,
            state,
        })
    }

    pub fn init(&mut self, x_array: &[f64], y_array: &[f64]) -> InterpResult<()> {
        if x_array.len() != self.size || y_array.len() != self.size {
            return Err(InterpError::InvalidData);
        }

        for i in 1..self.size {
            if x_array[i-1] >= x_array[i] {
                return Err(InterpError::InvalidData);
            }
        }

        self.xmin = x_array[0];
        self.xmax = x_array[self.size - 1];

        self.interp_type.init(&mut *self.state, x_array, y_array, self.size)
    }

    pub fn name(&self) -> &str {
        self.interp_type.name()
    }

    pub fn min_size(&self) -> usize {
        self.interp_type.min_size()
    }

    pub fn eval_e(&self, x_array: &[f64], y_array: &[f64], x: f64, accel: Option<&dyn std::any::Any>) -> InterpResult<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }

        let mut y = 0.0;
        self.interp_type.eval(&*self.state, x_array, y_array, self.size, x, accel, &mut y)?;
        Ok(y)
    }

    pub fn eval_deriv_e(&self, x_array: &[f64], y_array: &[f64], x: f64, accel: Option<&dyn std::any::Any>) -> InterpResult<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }

        let mut dydx = 0.0;
        self.interp_type.eval_deriv(&*self.state, x_array, y_array, self.size, x, accel, &mut dydx)?;
        Ok(dydx)
    }

    pub fn eval_deriv2_e(&self, x_array: &[f64], y_array: &[f64], x: f64, accel: Option<&dyn std::any::Any>) -> InterpResult<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }

        let mut d2 = 0.0;
        self.interp_type.eval_deriv2(&*self.state, x_array, y_array, self.size, x, accel, &mut d2)?;
        Ok(d2)
    }

    pub fn eval_integ_e(&self, x_array: &[f64], y_array: &[f64], a: f64, b: f64, accel: Option<&dyn std::any::Any>) -> InterpResult<f64> {
        if a > b || a < self.xmin || b > self.xmax {
            return Err(InterpError::DomainError);
        } else if a == b {
            return Ok(0.0);
        }

        let mut result = 0.0;
        self.interp_type.eval_integ(&*self.state, x_array, y_array, self.size, accel, a, b, &mut result)?;
        Ok(result)
    }
}