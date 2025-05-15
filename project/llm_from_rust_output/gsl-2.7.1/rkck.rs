use std::mem;
use std::ptr;
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslOdeivSystem {
    pub function: Box<dyn Fn(f64, &[f64], &mut [f64]) -> Result<(), GslError>>,
    pub jacobian: Option<Box<dyn Fn(f64, &[f64], &mut [f64], &mut [f64]) -> Result<(), GslError>>>,
    pub dimension: usize,
}

#[derive(Debug, Clone)]
pub struct RkckState {
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
    pub fn new(dim: usize) -> Result<Self, GslError> {
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

    pub fn reset(&mut self) {
        for v in &mut self.k1 { *v = 0.0; }
        for v in &mut self.k2 { *v = 0.0; }
        for v in &mut self.k3 { *v = 0.0; }
        for v in &mut self.k4 { *v = 0.0; }
        for v in &mut self.k5 { *v = 0.0; }
        for v in &mut self.k6 { *v = 0.0; }
        for v in &mut self.ytmp { *v = 0.0; }
        for v in &mut self.y0 { *v = 0.0; }
    }
}

pub struct RkckStepper {
    state: RkckState,
    ah: [f64; 5],
    b21: f64,
    b3: [f64; 2],
    b4: [f64; 3],
    b5: [f64; 4],
    b6: [f64; 5],
    c1: f64,
    c3: f64,
    c4: f64,
    c6: f64,
    ec: [f64; 7],
}

impl RkckStepper {
    pub fn new(dim: usize) -> Result<Self, GslError> {
        Ok(RkckStepper {
            state: RkckState::new(dim)?,
            ah: [1.0/5.0, 0.3, 3.0/5.0, 1.0, 7.0/8.0],
            b21: 1.0/5.0,
            b3: [3.0/40.0, 9.0/40.0],
            b4: [0.3, -0.9, 1.2],
            b5: [-11.0/54.0, 2.5, -70.0/27.0, 35.0/27.0],
            b6: [1631.0/55296.0, 175.0/512.0, 575.0/13824.0, 44275.0/110592.0, 253.0/4096.0],
            c1: 37.0/378.0,
            c3: 250.0/621.0,
            c4: 125.0/594.0,
            c6: 512.0/1771.0,
            ec: [
                0.0,
                37.0/378.0 - 2825.0/27648.0,
                0.0,
                250.0/621.0 - 18575.0/48384.0,
                125.0/594.0 - 13525.0/55296.0,
                -277.0/14336.0,
                512.0/1771.0 - 0.25,
            ],
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
        sys: &GslOdeivSystem,
    ) -> Result<(), GslError> {
        let dim = sys.dimension;
        self.state.y0.copy_from_slice(y);

        if let Some(dydt_in) = dydt_in {
            self.state.k1.copy_from_slice(dydt_in);
        } else {
            (sys.function)(t, y, &mut self.state.k1)?;
        }

        for i in 0..dim {
            self.state.ytmp[i] = y[i] + self.b21 * h * self.state.k1[i];
        }

        (sys.function)(t + self.ah[0] * h, &self.state.ytmp, &mut self.state.k2)?;

        for i in 0..dim {
            self.state.ytmp[i] = y[i] + h * (self.b3[0] * self.state.k1[i] + self.b3[1] * self.state.k2[i]);
        }

        (sys.function)(t + self.ah[1] * h, &self.state.ytmp, &mut self.state.k3)?;

        for i in 0..dim {
            self.state.ytmp[i] = y[i] + h * (self.b4[0] * self.state.k1[i] + 
                self.b4[1] * self.state.k2[i] + self.b4[2] * self.state.k3[i]);
        }

        (sys.function)(t + self.ah[2] * h, &self.state.ytmp, &mut self.state.k4)?;

        for i in 0..dim {
            self.state.ytmp[i] = y[i] + h * (self.b5[0] * self.state.k1[i] + 
                self.b5[1] * self.state.k2[i] + self.b5[2] * self.state.k3[i] + 
                self.b5[3] * self.state.k4[i]);
        }

        (sys.function)(t + self.ah[3] * h, &self.state.ytmp, &mut self.state.k5)?;

        for i in 0..dim {
            self.state.ytmp[i] = y[i] + h * (self.b6[0] * self.state.k1[i] + 
                self.b6[1] * self.state.k2[i] + self.b6[2] * self.state.k3[i] + 
                self.b6[3] * self.state.k4[i] + self.b6[4] * self.state.k5[i]);
        }

        (sys.function)(t + self.ah[4] * h, &self.state.ytmp, &mut self.state.k6)?;

        for i in 0..dim {
            let di = self.c1 * self.state.k1[i] + self.c3 * self.state.k3[i] + 
                self.c4 * self.state.k4[i] + self.c6 * self.state.k6[i];
            y[i] += h * di;
        }

        if let Some(dydt_out) = dydt_out {
            if let Err(e) = (sys.function)(t + h, y, dydt_out) {
                y.copy_from_slice(&self.state.y0);
                return Err(e);
            }
        }

        for i in 0..dim {
            yerr[i] = h * (self.ec[1] * self.state.k1[i] + 
                self.ec[3] * self.state.k3[i] + self.ec[4] * self.state.k4[i] + 
                self.ec[5] * self.state.k5[i] + self.ec[6] * self.state.k6[i]);
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