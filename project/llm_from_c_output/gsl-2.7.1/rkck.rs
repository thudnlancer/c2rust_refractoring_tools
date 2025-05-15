/* Runge-Kutta 4(5), Cash-Karp */

/* Reference: Cash, J.R., Karp, A.H., ACM Transactions of Mathematical
   Software, vol. 16 (1990) 201-222. */

use std::error::Error;
use std::fmt;

// Constants
const AH: [f64; 5] = [1.0 / 5.0, 0.3, 3.0 / 5.0, 1.0, 7.0 / 8.0];
const B21: f64 = 1.0 / 5.0;
const B3: [f64; 2] = [3.0 / 40.0, 9.0 / 40.0];
const B4: [f64; 3] = [0.3, -0.9, 1.2];
const B5: [f64; 4] = [-11.0 / 54.0, 2.5, -70.0 / 27.0, 35.0 / 27.0];
const B6: [f64; 5] = [
    1631.0 / 55296.0,
    175.0 / 512.0,
    575.0 / 13824.0,
    44275.0 / 110592.0,
    253.0 / 4096.0,
];
const C1: f64 = 37.0 / 378.0;
const C3: f64 = 250.0 / 621.0;
const C4: f64 = 125.0 / 594.0;
const C6: f64 = 512.0 / 1771.0;
const EC: [f64; 7] = [
    0.0,
    37.0 / 378.0 - 2825.0 / 27648.0,
    0.0,
    250.0 / 621.0 - 18575.0 / 48384.0,
    125.0 / 594.0 - 13525.0 / 55296.0,
    -277.0 / 14336.0,
    512.0 / 1771.0 - 0.25,
];

#[derive(Debug)]
struct OdeError {
    message: String,
}

impl fmt::Display for OdeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ODE error: {}", self.message)
    }
}

impl Error for OdeError {}

impl OdeError {
    fn new(msg: &str) -> Self {
        OdeError {
            message: msg.to_string(),
        }
    }
}

type OdeResult<T> = Result<T, OdeError>;

struct RkckState {
    k1: Vec<f64>,
    k2: Vec<f64>,
    k3: Vec<f64>,
    k4: Vec<f64>,
    k5: Vec<f64>,
    k6: Vec<f64>,
    y0: Vec<f64>,
    ytmp: Vec<f64>,
}

impl RkckState {
    fn new(dim: usize) -> OdeResult<Self> {
        Ok(RkckState {
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

struct OdeSystem {
    dimension: usize,
    params: Vec<f64>,
    func: Box<dyn Fn(f64, &[f64], &mut [f64]) -> OdeResult<()>>,
}

impl OdeSystem {
    fn eval(&self, t: f64, y: &[f64], dydt: &mut [f64]) -> OdeResult<()> {
        (self.func)(t, y, dydt)
    }
}

fn rkck_apply(
    state: &mut RkckState,
    dim: usize,
    t: f64,
    h: f64,
    y: &mut [f64],
    yerr: &mut [f64],
    dydt_in: Option<&[f64]>,
    dydt_out: Option<&mut [f64]>,
    sys: &OdeSystem,
) -> OdeResult<()> {
    state.y0.copy_from_slice(y);

    // k1 step
    if let Some(dydt) = dydt_in {
        state.k1.copy_from_slice(dydt);
    } else {
        sys.eval(t, y, &mut state.k1)?;
    }

    for i in 0..dim {
        state.ytmp[i] = y[i] + B21 * h * state.k1[i];
    }

    // k2 step
    sys.eval(t + AH[0] * h, &state.ytmp, &mut state.k2)?;

    for i in 0..dim {
        state.ytmp[i] = y[i] + h * (B3[0] * state.k1[i] + B3[1] * state.k2[i]);
    }

    // k3 step
    sys.eval(t + AH[1] * h, &state.ytmp, &mut state.k3)?;

    for i in 0..dim {
        state.ytmp[i] = y[i] + h * (B4[0] * state.k1[i] + B4[1] * state.k2[i] + B4[2] * state.k3[i]);
    }

    // k4 step
    sys.eval(t + AH[2] * h, &state.ytmp, &mut state.k4)?;

    for i in 0..dim {
        state.ytmp[i] = y[i]
            + h * (B5[0] * state.k1[i]
                + B5[1] * state.k2[i]
                + B5[2] * state.k3[i]
                + B5[3] * state.k4[i]);
    }

    // k5 step
    sys.eval(t + AH[3] * h, &state.ytmp, &mut state.k5)?;

    for i in 0..dim {
        state.ytmp[i] = y[i]
            + h * (B6[0] * state.k1[i]
                + B6[1] * state.k2[i]
                + B6[2] * state.k3[i]
                + B6[3] * state.k4[i]
                + B6[4] * state.k5[i]);
    }

    // k6 step and final sum
    sys.eval(t + AH[4] * h, &state.ytmp, &mut state.k6)?;

    for i in 0..dim {
        let d_i = C1 * state.k1[i] + C3 * state.k3[i] + C4 * state.k4[i] + C6 * state.k6[i];
        y[i] += h * d_i;
    }

    // Evaluate dydt_out
    if let Some(dydt) = dydt_out {
        sys.eval(t + h, y, dydt)?;
    }

    // difference between 4th and 5th order
    for i in 0..dim {
        yerr[i] = h * (EC[1] * state.k1[i]
            + EC[3] * state.k3[i]
            + EC[4] * state.k4[i]
            + EC[5] * state.k5[i]
            + EC[6] * state.k6[i]);
    }

    Ok(())
}

struct RkckStepper {
    state: RkckState,
    dim: usize,
}

impl RkckStepper {
    fn new(dim: usize) -> OdeResult<Self> {
        Ok(RkckStepper {
            state: RkckState::new(dim)?,
            dim,
        })
    }

    fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &OdeSystem,
    ) -> OdeResult<()> {
        rkck_apply(
            &mut self.state,
            self.dim,
            t,
            h,
            y,
            yerr,
            dydt_in,
            dydt_out,
            sys,
        )
    }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn order(&self) -> u32 {
        5
    }
}