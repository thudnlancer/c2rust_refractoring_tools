use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub type GslResult<T> = Result<T, GslError>;

pub struct OdeivSystem {
    pub function: Box<dyn Fn(f64, &[f64], &mut [f64]) -> GslResult<()>>,
    pub jacobian: Option<Box<dyn Fn(f64, &[f64], &mut [f64], &mut [f64]) -> GslResult<()>>>,
    pub dimension: usize,
}

pub struct Rk4State {
    k: Vec<f64>,
    k1: Vec<f64>,
    y0: Vec<f64>,
    ytmp: Vec<f64>,
    y_onestep: Vec<f64>,
}

impl Rk4State {
    pub fn new(dim: usize) -> GslResult<Self> {
        Ok(Self {
            k: vec![0.0; dim],
            k1: vec![0.0; dim],
            y0: vec![0.0; dim],
            ytmp: vec![0.0; dim],
            y_onestep: vec![0.0; dim],
        })
    }

    pub fn reset(&mut self) {
        self.k.fill(0.0);
        self.k1.fill(0.0);
        self.y0.fill(0.0);
        self.ytmp.fill(0.0);
        self.y_onestep.fill(0.0);
    }

    pub fn order() -> u32 {
        4
    }
}

pub struct Rk4Step {
    state: Rk4State,
}

impl Rk4Step {
    pub fn new(dim: usize) -> GslResult<Self> {
        Ok(Self {
            state: Rk4State::new(dim)?,
        })
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &OdeivSystem,
    ) -> GslResult<()> {
        let dim = sys.dimension;
        self.state.y0.copy_from_slice(y);

        if let Some(dydt_in) = dydt_in {
            self.state.k.copy_from_slice(dydt_in);
        } else {
            (sys.function)(t, &self.state.y0, &mut self.state.k)?;
        }

        self.state.k1.copy_from_slice(&self.state.k);
        self.state.y_onestep.copy_from_slice(y);

        self.step(&mut self.state.y_onestep, h, t, sys)?;

        self.state.k.copy_from_slice(&self.state.k1);
        self.step(y, h / 2.0, t, sys)?;

        (sys.function)(t + h / 2.0, y, &mut self.state.k)?;

        let y0_copy = self.state.y0.clone();
        self.state.y0.copy_from_slice(y);
        self.step(y, h / 2.0, t + h / 2.0, sys)?;

        if let Some(dydt_out) = dydt_out {
            (sys.function)(t + h, y, dydt_out)?;
        }

        for i in 0..dim {
            yerr[i] = 4.0 * (y[i] - self.state.y_onestep[i]) / 15.0;
        }

        Ok(())
    }

    fn step(
        &mut self,
        y: &mut [f64],
        h: f64,
        t: f64,
        sys: &OdeivSystem,
    ) -> GslResult<()> {
        let dim = sys.dimension;

        for i in 0..dim {
            y[i] += h / 6.0 * self.state.k[i];
            self.state.ytmp[i] = self.state.y0[i] + 0.5 * h * self.state.k[i];
        }

        (sys.function)(t + 0.5 * h, &self.state.ytmp, &mut self.state.k)?;

        for i in 0..dim {
            y[i] += h / 3.0 * self.state.k[i];
            self.state.ytmp[i] = self.state.y0[i] + 0.5 * h * self.state.k[i];
        }

        (sys.function)(t + 0.5 * h, &self.state.ytmp, &mut self.state.k)?;

        for i in 0..dim {
            y[i] += h / 3.0 * self.state.k[i];
            self.state.ytmp[i] = self.state.y0[i] + h * self.state.k[i];
        }

        (sys.function)(t + h, &self.state.ytmp, &mut self.state.k)?;

        for i in 0..dim {
            y[i] += h / 6.0 * self.state.k[i];
        }

        Ok(())
    }
}

pub struct Rk4 {
    step: Rk4Step,
}

impl Rk4 {
    pub fn new(dim: usize) -> GslResult<Self> {
        Ok(Self {
            step: Rk4Step::new(dim)?,
        })
    }

    pub fn reset(&mut self) {
        self.step.reset();
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &OdeivSystem,
    ) -> GslResult<()> {
        self.step.apply(t, h, y, yerr, dydt_in, dydt_out, sys)
    }

    pub fn order() -> u32 {
        Rk4State::order()
    }
}