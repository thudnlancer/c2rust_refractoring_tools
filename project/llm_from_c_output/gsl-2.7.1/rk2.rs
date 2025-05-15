use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub struct OdeError {
    message: String,
    code: i32,
}

impl OdeError {
    fn new(message: &str, code: i32) -> Self {
        OdeError {
            message: message.to_string(),
            code,
        }
    }
}

impl fmt::Display for OdeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (code: {})", self.message, self.code)
    }
}

impl Error for OdeError {}

const GSL_SUCCESS: i32 = 0;
const GSL_ENOMEM: i32 = 1;

pub struct Odeiv2System {
    pub dimension: usize,
    pub params: Vec<f64>,
    pub function: fn(f64, &[f64], &mut [f64], &[f64]) -> Result<(), OdeError>,
}

pub struct Rk2State {
    k1: Vec<f64>,
    k2: Vec<f64>,
    k3: Vec<f64>,
    ytmp: Vec<f64>,
}

impl Rk2State {
    fn new(dim: usize) -> Result<Self, OdeError> {
        Ok(Rk2State {
            k1: vec![0.0; dim],
            k2: vec![0.0; dim],
            k3: vec![0.0; dim],
            ytmp: vec![0.0; dim],
        })
    }

    fn reset(&mut self) {
        self.k1.iter_mut().for_each(|x| *x = 0.0);
        self.k2.iter_mut().for_each(|x| *x = 0.0);
        self.k3.iter_mut().for_each(|x| *x = 0.0);
        self.ytmp.iter_mut().for_each(|x| *x = 0.0);
    }
}

pub struct Rk2Stepper {
    state: Rk2State,
}

impl Rk2Stepper {
    pub fn new(dim: usize) -> Result<Self, OdeError> {
        Ok(Rk2Stepper {
            state: Rk2State::new(dim)?,
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
        sys: &Odeiv2System,
    ) -> Result<(), OdeError> {
        let dim = sys.dimension;
        let k1 = &mut self.state.k1;
        let k2 = &mut self.state.k2;
        let k3 = &mut self.state.k3;
        let ytmp = &mut self.state.ytmp;

        // k1 step
        if let Some(dydt_in) = dydt_in {
            k1.copy_from_slice(dydt_in);
        } else {
            (sys.function)(t, y, k1, &sys.params)?;
        }

        // k2 step
        for i in 0..dim {
            ytmp[i] = y[i] + 0.5 * h * k1[i];
        }
        (sys.function)(t + 0.5 * h, ytmp, k2, &sys.params)?;

        // k3 step
        for i in 0..dim {
            ytmp[i] = y[i] + h * (-k1[i] + 2.0 * k2[i]);
        }
        (sys.function)(t + h, ytmp, k3, &sys.params)?;

        // Save original values
        ytmp.copy_from_slice(y);

        // Final sum
        for i in 0..dim {
            let ksum3 = (k1[i] + 4.0 * k2[i] + k3[i]) / 6.0;
            y[i] += h * ksum3;
        }

        // Derivatives at output
        if let Some(dydt_out) = dydt_out {
            if let Err(e) = (sys.function)(t + h, y, dydt_out, &sys.params) {
                // Restore original values
                y.copy_from_slice(ytmp);
                return Err(e);
            }
        }

        // Error estimation
        for i in 0..dim {
            let ksum3 = (k1[i] + 4.0 * k2[i] + k3[i]) / 6.0;
            yerr[i] = h * (k2[i] - ksum3);
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }

    pub fn order(&self) -> u32 {
        2
    }
}