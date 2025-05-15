use std::ffi::CString;
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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

pub type GslResult = Result<(), GslError>;

#[derive(Debug)]
pub struct GslOdeivSystem {
    pub function: Box<dyn Fn(f64, &[f64], &mut [f64]) -> GslResult>,
    pub jacobian: Option<Box<dyn Fn(f64, &[f64], &mut [f64], &mut [f64]) -> GslResult>>,
    pub dimension: usize,
}

pub struct Rk2State {
    k1: Vec<f64>,
    k2: Vec<f64>,
    k3: Vec<f64>,
    ytmp: Vec<f64>,
}

impl Rk2State {
    pub fn new(dim: usize) -> Option<Self> {
        Some(Self {
            k1: vec![0.0; dim],
            k2: vec![0.0; dim],
            k3: vec![0.0; dim],
            ytmp: vec![0.0; dim],
        })
    }

    pub fn reset(&mut self) {
        self.k1.iter_mut().for_each(|x| *x = 0.0);
        self.k2.iter_mut().for_each(|x| *x = 0.0);
        self.k3.iter_mut().for_each(|x| *x = 0.0);
        self.ytmp.iter_mut().for_each(|x| *x = 0.0);
    }

    pub fn order() -> u32 {
        2
    }
}

pub struct Rk2Step {
    state: Rk2State,
}

impl Rk2Step {
    pub fn new(dim: usize) -> Option<Self> {
        Rk2State::new(dim).map(|state| Self { state })
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
    ) -> GslResult {
        let dim = sys.dimension;
        let k1 = &mut self.state.k1;
        let k2 = &mut self.state.k2;
        let k3 = &mut self.state.k3;
        let ytmp = &mut self.state.ytmp;

        if let Some(dydt_in) = dydt_in {
            k1.copy_from_slice(dydt_in);
        } else {
            (sys.function)(t, y, k1)?;
        }

        for i in 0..dim {
            ytmp[i] = y[i] + 0.5 * h * k1[i];
        }

        (sys.function)(t + 0.5 * h, ytmp, k2)?;

        for i in 0..dim {
            ytmp[i] = y[i] + h * (-k1[i] + 2.0 * k2[i]);
        }

        (sys.function)(t + h, ytmp, k3)?;

        for i in 0..dim {
            ytmp[i] = y[i];
            let ksum3 = (k1[i] + 4.0 * k2[i] + k3[i]) / 6.0;
            y[i] += h * ksum3;
        }

        if let Some(dydt_out) = dydt_out {
            if let Err(e) = (sys.function)(t + h, y, dydt_out) {
                y.copy_from_slice(ytmp);
                return Err(e);
            }
        }

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
        Rk2State::order()
    }
}

pub fn gsl_odeiv_step_rk2(dim: usize) -> Option<Rk2Step> {
    Rk2Step::new(dim)
}