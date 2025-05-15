use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone)]
pub enum GslError {
    NoMemory,
    InvalidInput,
    NoMinimum,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GslError::NoMemory => write!(f, "failed to allocate space"),
            GslError::InvalidInput => write!(f, "invalid input parameters"),
            GslError::NoMinimum => write!(f, "endpoints do not enclose a minimum"),
        }
    }
}

impl Error for GslError {}

pub type GslResult<T> = Result<T, GslError>;

pub trait GslFunction {
    fn evaluate(&self, x: f64) -> f64;
}

pub struct GslMinFminimizerType {
    name: &'static str,
    size: usize,
    set: fn(state: &mut Vec<u8>, f: &dyn GslFunction, x_min: f64, f_min: f64, x_lower: f64, f_lower: f64, x_upper: f64, f_upper: f64) -> GslResult<()>,
    iterate: fn(state: &mut Vec<u8>, f: &dyn GslFunction, x_min: &mut f64, f_min: &mut f64, x_lower: &mut f64, f_lower: &mut f64, x_upper: &mut f64, f_upper: &mut f64) -> GslResult<()>,
}

pub struct GslMinFminimizer {
    state: Vec<u8>,
    function: Option<Box<dyn GslFunction>>,
    x_minimum: f64,
    x_lower: f64,
    x_upper: f64,
    f_minimum: f64,
    f_lower: f64,
    f_upper: f64,
    type_: &'static GslMinFminimizerType,
}

fn compute_f_values(
    f: &dyn GslFunction,
    x_minimum: f64,
    f_minimum: &mut f64,
    x_lower: f64,
    f_lower: &mut f64,
    x_upper: f64,
    f_upper: &mut f64,
) -> GslResult<()> {
    *f_lower = f.evaluate(x_lower);
    *f_upper = f.evaluate(x_upper);
    *f_minimum = f.evaluate(x_minimum);
    Ok(())
}

impl GslMinFminimizer {
    pub fn new(t: &'static GslMinFminimizerType) -> GslResult<Self> {
        let state = vec![0u8; t.size];
        Ok(Self {
            state,
            function: None,
            x_minimum: 0.0,
            x_lower: 0.0,
            x_upper: 0.0,
            f_minimum: 0.0,
            f_lower: 0.0,
            f_upper: 0.0,
            type_: t,
        })
    }

    pub fn set(
        &mut self,
        f: Box<dyn GslFunction>,
        x_minimum: f64,
        x_lower: f64,
        x_upper: f64,
    ) -> GslResult<()> {
        let mut f_minimum = 0.0;
        let mut f_lower = 0.0;
        let mut f_upper = 0.0;

        compute_f_values(&*f, x_minimum, &mut f_minimum, x_lower, &mut f_lower, x_upper, &mut f_upper)?;

        self.set_with_values(f, x_minimum, f_minimum, x_lower, f_lower, x_upper, f_upper)
    }

    pub fn set_with_values(
        &mut self,
        f: Box<dyn GslFunction>,
        x_minimum: f64,
        f_minimum: f64,
        x_lower: f64,
        f_lower: f64,
        x_upper: f64,
        f_upper: f64,
    ) -> GslResult<()> {
        self.function = Some(f);
        self.x_minimum = x_minimum;
        self.x_lower = x_lower;
        self.x_upper = x_upper;

        if x_lower > x_upper {
            return Err(GslError::InvalidInput);
        }

        if x_minimum >= x_upper || x_minimum <= x_lower {
            return Err(GslError::InvalidInput);
        }

        self.f_lower = f_lower;
        self.f_upper = f_upper;
        self.f_minimum = f_minimum;

        if f_minimum >= f_lower || f_minimum >= f_upper {
            return Err(GslError::NoMinimum);
        }

        (self.type_.set)(
            &mut self.state,
            self.function.as_ref().unwrap(),
            x_minimum,
            f_minimum,
            x_lower,
            f_lower,
            x_upper,
            f_upper,
        )
    }

    pub fn iterate(&mut self) -> GslResult<()> {
        (self.type_.iterate)(
            &mut self.state,
            self.function.as_ref().unwrap(),
            &mut self.x_minimum,
            &mut self.f_minimum,
            &mut self.x_lower,
            &mut self.f_lower,
            &mut self.x_upper,
            &mut self.f_upper,
        )
    }

    pub fn name(&self) -> &'static str {
        self.type_.name
    }

    pub fn x_minimum(&self) -> f64 {
        self.x_minimum
    }

    pub fn x_lower(&self) -> f64 {
        self.x_lower
    }

    pub fn x_upper(&self) -> f64 {
        self.x_upper
    }

    pub fn f_minimum(&self) -> f64 {
        self.f_minimum
    }

    pub fn f_lower(&self) -> f64 {
        self.f_lower
    }

    pub fn f_upper(&self) -> f64 {
        self.f_upper
    }
}