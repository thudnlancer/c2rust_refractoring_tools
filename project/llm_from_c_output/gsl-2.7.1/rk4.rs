use std::error::Error;
use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct OdeError {
    message: String,
}

impl OdeError {
    fn new(msg: &str) -> Self {
        OdeError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for OdeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for OdeError {}

type OdeResult<T> = Result<T, OdeError>;

pub trait OdeSystem {
    fn eval(&self, t: f64, y: &[f64], dydt: &mut [f64]) -> OdeResult<()>;
}

pub struct Rk4State {
    k: Vec<f64>,
    k1: Vec<f64>,
    y0: Vec<f64>,
    ytmp: Vec<f64>,
    y_onestep: Vec<f64>,
}

impl Rk4State {
    pub fn new(dim: usize) -> OdeResult<Self> {
        Ok(Rk4State {
            k: vec![0.0; dim],
            k1: vec![0.0; dim],
            y0: vec![0.0; dim],
            ytmp: vec![0.0; dim],
            y_onestep: vec![0.0; dim],
        })
    }

    fn step(
        &mut self,
        y: &mut [f64],
        h: f64,
        t: f64,
        sys: &dyn OdeSystem,
    ) -> OdeResult<()> {
        let y0 = &self.y0;
        let ytmp = &mut self.ytmp;
        let k = &mut self.k;

        // k1 step
        for i in 0..y.len() {
            y[i] += h / 6.0 * k[i];
            ytmp[i] = y0[i] + 0.5 * h * k[i];
        }

        // k2 step
        sys.eval(t + 0.5 * h, ytmp, k)?;
        for i in 0..y.len() {
            y[i] += h / 3.0 * k[i];
            ytmp[i] = y0[i] + 0.5 * h * k[i];
        }

        // k3 step
        sys.eval(t + 0.5 * h, ytmp, k)?;
        for i in 0..y.len() {
            y[i] += h / 3.0 * k[i];
            ytmp[i] = y0[i] + h * k[i];
        }

        // k4 step
        sys.eval(t + h, ytmp, k)?;
        for i in 0..y.len() {
            y[i] += h / 6.0 * k[i];
        }

        Ok(())
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &dyn OdeSystem,
    ) -> OdeResult<()> {
        self.y0.copy_from_slice(y);

        if let Some(dydt) = dydt_in {
            self.k.copy_from_slice(dydt);
        } else {
            sys.eval(t, &self.y0, &mut self.k)?;
        }

        // Error estimation by step doubling
        self.k1.copy_from_slice(&self.k);
        self.y_onestep.copy_from_slice(y);

        self.step(&mut self.y_onestep, h, t, sys)?;

        // Two half steps
        self.k.copy_from_slice(&self.k1);
        self.step(y, h / 2.0, t, sys)?;

        // Update before second step
        sys.eval(t + h / 2.0, y, &mut self.k)?;

        // Save original y0 for possible failures
        let y0_backup = self.y0.clone();

        // Update y0 for second step
        self.y0.copy_from_slice(y);
        self.step(y, h / 2.0, t + h / 2.0, sys)?;

        // Derivatives at output
        if let Some(dydt) = dydt_out {
            sys.eval(t + h, y, dydt)?;
        }

        // Error estimation
        for i in 0..y.len() {
            yerr[i] = 0.5 * (y[i] - self.y_onestep[i]) / 15.0;
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        for v in &mut self.k {
            *v = 0.0;
        }
        for v in &mut self.k1 {
            *v = 0.0;
        }
        for v in &mut self.y0 {
            *v = 0.0;
        }
        for v in &mut self.ytmp {
            *v = 0.0;
        }
        for v in &mut self.y_onestep {
            *v = 0.0;
        }
    }

    pub fn order(&self) -> usize {
        4
    }
}

pub struct Rk4Stepper {
    state: Rk4State,
}

impl Rk4Stepper {
    pub fn new(dim: usize) -> OdeResult<Self> {
        Ok(Rk4Stepper {
            state: Rk4State::new(dim)?,
        })
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &dyn OdeSystem,
    ) -> OdeResult<()> {
        self.state.apply(t, h, y, yerr, dydt_in, dydt_out, sys)
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }

    pub fn order(&self) -> usize {
        self.state.order()
    }
}