use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone)]
pub enum GslError {
    NoMemory,
    Invalid,
    Failed,
    Other(i32),
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GslError::NoMemory => write!(f, "memory allocation failed"),
            GslError::Invalid => write!(f, "invalid argument"),
            GslError::Failed => write!(f, "generic failure"),
            GslError::Other(code) => write!(f, "GSL error code {}", code),
        }
    }
}

impl Error for GslError {}

pub type GslResult<T> = Result<T, GslError>;

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_ENOMEM: i32 = 8;
pub const GSL_EINVAL: i32 = 4;
pub const GSL_EFAILED: i32 = 5;

#[derive(Clone)]
pub struct GslOdeivSystem {
    pub function: Box<dyn Fn(f64, &[f64], &mut [f64]) -> GslResult<()>>,
    pub dimension: usize,
}

pub struct Rk4ImpState {
    k1nu: Vec<f64>,
    k2nu: Vec<f64>,
    ytmp1: Vec<f64>,
    ytmp2: Vec<f64>,
    y0: Vec<f64>,
    y0_orig: Vec<f64>,
    y_onestep: Vec<f64>,
}

impl Rk4ImpState {
    pub fn new(dim: usize) -> GslResult<Self> {
        Ok(Self {
            k1nu: vec![0.0; dim],
            k2nu: vec![0.0; dim],
            ytmp1: vec![0.0; dim],
            ytmp2: vec![0.0; dim],
            y0: vec![0.0; dim],
            y0_orig: vec![0.0; dim],
            y_onestep: vec![0.0; dim],
        })
    }

    fn step(
        &mut self,
        y: &mut [f64],
        h: f64,
        t: f64,
        sys: &GslOdeivSystem,
    ) -> GslResult<()> {
        let ir3 = 1.0 / 1.73205080756887729352744634151;
        let iter_steps = 3;

        for _ in 0..iter_steps {
            for i in 0..sys.dimension {
                self.ytmp1[i] = y[i] + h * (0.25 * self.k1nu[i] + 0.5 * (0.5 - ir3) * self.k2nu[i]);
                self.ytmp2[i] = y[i] + h * (0.25 * self.k2nu[i] + 0.5 * (0.5 + ir3) * self.k1nu[i]);
            }

            (sys.function)(t + 0.5 * h * (1.0 - ir3), &self.ytmp1, &mut self.k1nu)?;
            (sys.function)(t + 0.5 * h * (1.0 + ir3), &self.ytmp2, &mut self.k2nu)?;
        }

        for i in 0..sys.dimension {
            let d_i = 0.5 * (self.k1nu[i] + self.k2nu[i]);
            y[i] += h * d_i;
        }

        Ok(())
    }

    pub fn apply(
        &mut self,
        dim: usize,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &GslOdeivSystem,
    ) -> GslResult<()> {
        self.y0.copy_from_slice(y);
        self.y0_orig.copy_from_slice(y);

        if let Some(dydt) = dydt_in {
            self.k1nu.copy_from_slice(dydt);
        } else {
            (sys.function)(t, y, &mut self.k1nu)?;
        }

        self.k2nu.copy_from_slice(&self.k1nu);
        self.y_onestep.copy_from_slice(y);

        self.step(&mut self.y_onestep, h, t, sys)?;
        self.step(y, h / 2.0, t, sys)?;

        self.y0.copy_from_slice(y);
        (sys.function)(t + h / 2.0, y, &mut self.k1nu)?;
        self.k2nu.copy_from_slice(&self.k1nu);

        self.step(y, h / 2.0, t + h / 2.0, sys)?;

        if let Some(dydt_out) = dydt_out {
            (sys.function)(t + h, y, dydt_out)?;
        }

        for i in 0..dim {
            yerr[i] = 8.0 * 0.5 * (y[i] - self.y_onestep[i]) / 15.0;
        }

        Ok(())
    }

    pub fn reset(&mut self, dim: usize) {
        for v in &mut [
            &mut self.y_onestep,
            &mut self.y0_orig,
            &mut self.y0,
            &mut self.k1nu,
            &mut self.k2nu,
            &mut self.ytmp1,
            &mut self.ytmp2,
        ] {
            v.iter_mut().take(dim).for_each(|x| *x = 0.0);
        }
    }

    pub fn order(&self) -> u32 {
        4
    }
}

pub struct Rk4Imp {
    state: Rk4ImpState,
}

impl Rk4Imp {
    pub fn new(dim: usize) -> GslResult<Self> {
        Ok(Self {
            state: Rk4ImpState::new(dim)?,
        })
    }

    pub fn apply(
        &mut self,
        dim: usize,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &GslOdeivSystem,
    ) -> GslResult<()> {
        self.state.apply(dim, t, h, y, yerr, dydt_in, dydt_out, sys)
    }

    pub fn reset(&mut self, dim: usize) {
        self.state.reset(dim);
    }

    pub fn order(&self) -> u32 {
        self.state.order()
    }
}