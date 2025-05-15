/* ode-initval2/rkf45.rs

   Translated from C to Rust while maintaining functionality and safety.
*/

use std::error::Error;
use std::fmt;

// Constants for Runge-Kutta-Fehlberg coefficients
const AH: [f64; 5] = [1.0 / 4.0, 3.0 / 8.0, 12.0 / 13.0, 1.0, 1.0 / 2.0];
const B3: [f64; 2] = [3.0 / 32.0, 9.0 / 32.0];
const B4: [f64; 3] = [1932.0 / 2197.0, -7200.0 / 2197.0, 7296.0 / 2197.0];
const B5: [f64; 4] = [8341.0 / 4104.0, -32832.0 / 4104.0, 29440.0 / 4104.0, -845.0 / 4104.0];
const B6: [f64; 5] = [
    -6080.0 / 20520.0,
    41040.0 / 20520.0,
    -28352.0 / 20520.0,
    9295.0 / 20520.0,
    -5643.0 / 20520.0,
];

const C1: f64 = 902880.0 / 7618050.0;
const C3: f64 = 3953664.0 / 7618050.0;
const C4: f64 = 3855735.0 / 7618050.0;
const C5: f64 = -1371249.0 / 7618050.0;
const C6: f64 = 277020.0 / 7618050.0;

// Error coefficients for difference between 5th and 4th order
const EC: [f64; 7] = [
    0.0,
    1.0 / 360.0,
    0.0,
    -128.0 / 4275.0,
    -2197.0 / 75240.0,
    1.0 / 50.0,
    2.0 / 55.0,
];

// Error type for ODE solver
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
        write!(f, "ODE Error: {}", self.message)
    }
}

impl Error for OdeError {}

// ODE system trait
pub trait OdeSystem {
    fn eval(&self, t: f64, y: &[f64], dydt: &mut [f64]) -> Result<(), OdeError>;
}

// RKF45 state
struct Rkf45State {
    k1: Vec<f64>,
    k2: Vec<f64>,
    k3: Vec<f64>,
    k4: Vec<f64>,
    k5: Vec<f64>,
    k6: Vec<f64>,
    y0: Vec<f64>,
    ytmp: Vec<f64>,
}

impl Rkf45State {
    fn new(dim: usize) -> Result<Self, OdeError> {
        Ok(Rkf45State {
            k1: vec![0.0; dim],
            k2: vec![0.0; dim],
            k3: vec![0.0; dim],
            k4: vec![0.0; dim],
            k5: vec![0.0; dim],
            k6: vec![0.0; dim],
            y0: vec![0.0; dim],
            ytmp: vec![0.0; dim],
        })
    }

    fn reset(&mut self) {
        self.k1.iter_mut().for_each(|x| *x = 0.0);
        self.k2.iter_mut().for_each(|x| *x = 0.0);
        self.k3.iter_mut().for_each(|x| *x = 0.0);
        self.k4.iter_mut().for_each(|x| *x = 0.0);
        self.k5.iter_mut().for_each(|x| *x = 0.0);
        self.k6.iter_mut().for_each(|x| *x = 0.0);
        self.ytmp.iter_mut().for_each(|x| *x = 0.0);
        self.y0.iter_mut().for_each(|x| *x = 0.0);
    }
}

pub struct Rkf45Stepper {
    state: Rkf45State,
}

impl Rkf45Stepper {
    pub fn new(dim: usize) -> Result<Self, OdeError> {
        Ok(Rkf45Stepper {
            state: Rkf45State::new(dim)?,
        })
    }

    pub fn apply<S: OdeSystem>(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &S,
    ) -> Result<(), OdeError> {
        let dim = y.len();
        self.state.y0.copy_from_slice(y);

        // k1 step
        if let Some(dydt) = dydt_in {
            self.state.k1.copy_from_slice(dydt);
        } else {
            sys.eval(t, y, &mut self.state.k1)?;
        }

        // Prepare ytmp for k2 step
        for i in 0..dim {
            self.state.ytmp[i] = y[i] + AH[0] * h * self.state.k1[i];
        }

        // k2 step
        sys.eval(t + AH[0] * h, &self.state.ytmp, &mut self.state.k2)?;

        // Prepare ytmp for k3 step
        for i in 0..dim {
            self.state.ytmp[i] = y[i] + h * (B3[0] * self.state.k1[i] + B3[1] * self.state.k2[i]);
        }

        // k3 step
        sys.eval(t + AH[1] * h, &self.state.ytmp, &mut self.state.k3)?;

        // Prepare ytmp for k4 step
        for i in 0..dim {
            self.state.ytmp[i] = y[i]
                + h * (B4[0] * self.state.k1[i] + B4[1] * self.state.k2[i] + B4[2] * self.state.k3[i]);
        }

        // k4 step
        sys.eval(t + AH[2] * h, &self.state.ytmp, &mut self.state.k4)?;

        // Prepare ytmp for k5 step
        for i in 0..dim {
            self.state.ytmp[i] = y[i]
                + h * (B5[0] * self.state.k1[i]
                    + B5[1] * self.state.k2[i]
                    + B5[2] * self.state.k3[i]
                    + B5[3] * self.state.k4[i]);
        }

        // k5 step
        sys.eval(t + AH[3] * h, &self.state.ytmp, &mut self.state.k5)?;

        // Prepare ytmp for k6 step
        for i in 0..dim {
            self.state.ytmp[i] = y[i]
                + h * (B6[0] * self.state.k1[i]
                    + B6[1] * self.state.k2[i]
                    + B6[2] * self.state.k3[i]
                    + B6[3] * self.state.k4[i]
                    + B6[4] * self.state.k5[i]);
        }

        // k6 step
        sys.eval(t + AH[4] * h, &self.state.ytmp, &mut self.state.k6)?;

        // Final sum
        for i in 0..dim {
            let d_i = C1 * self.state.k1[i]
                + C3 * self.state.k3[i]
                + C4 * self.state.k4[i]
                + C5 * self.state.k5[i]
                + C6 * self.state.k6[i];
            y[i] += h * d_i;
        }

        // Derivatives at output
        if let Some(dydt) = dydt_out {
            sys.eval(t + h, y, dydt)?;
        }

        // Error estimation
        for i in 0..dim {
            yerr[i] = h
                * (EC[1] * self.state.k1[i]
                    + EC[3] * self.state.k3[i]
                    + EC[4] * self.state.k4[i]
                    + EC[5] * self.state.k5[i]
                    + EC[6] * self.state.k6[i]);
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }

    pub fn order(&self) -> u32 {
        5
    }
}