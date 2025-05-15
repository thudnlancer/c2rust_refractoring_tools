use std::mem;
use std::ptr;
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
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

#[derive(Clone)]
pub struct GslOdeivSystem {
    pub function: Box<dyn Fn(f64, &[f64], &mut [f64], &mut dyn std::any::Any) -> Result<(), GslError>>,
    pub jacobian: Option<Box<dyn Fn(f64, &[f64], &mut [f64], &mut [f64], &mut dyn std::any::Any) -> Result<(), GslError>>>,
    pub dimension: usize,
    pub params: Box<dyn std::any::Any>,
}

pub struct Rkf45State {
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
    pub fn new(dim: usize) -> Option<Self> {
        Some(Self {
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
        self.k1.fill(0.0);
        self.k2.fill(0.0);
        self.k3.fill(0.0);
        self.k4.fill(0.0);
        self.k5.fill(0.0);
        self.k6.fill(0.0);
        self.ytmp.fill(0.0);
        self.y0.fill(0.0);
    }
}

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
const EC: [f64; 7] = [
    0.0,
    1.0 / 360.0,
    0.0,
    -128.0 / 4275.0,
    -2197.0 / 75240.0,
    1.0 / 50.0,
    2.0 / 55.0,
];

pub fn rkf45_apply(
    state: &mut Rkf45State,
    dim: usize,
    t: f64,
    h: f64,
    y: &mut [f64],
    yerr: &mut [f64],
    dydt_in: Option<&[f64]>,
    dydt_out: Option<&mut [f64]>,
    sys: &GslOdeivSystem,
) -> Result<(), GslError> {
    state.y0.copy_from_slice(y);

    if let Some(dydt) = dydt_in {
        state.k1.copy_from_slice(dydt);
    } else {
        (sys.function)(t, y, &mut state.k1, &mut *sys.params)?;
    }

    for i in 0..dim {
        state.ytmp[i] = y[i] + AH[0] * h * state.k1[i];
    }

    (sys.function)(t + AH[0] * h, &state.ytmp, &mut state.k2, &mut *sys.params)?;

    for i in 0..dim {
        state.ytmp[i] = y[i] + h * (B3[0] * state.k1[i] + B3[1] * state.k2[i]);
    }

    (sys.function)(t + AH[1] * h, &state.ytmp, &mut state.k3, &mut *sys.params)?;

    for i in 0..dim {
        state.ytmp[i] = y[i] + h * (B4[0] * state.k1[i] + B4[1] * state.k2[i] + B4[2] * state.k3[i]);
    }

    (sys.function)(t + AH[2] * h, &state.ytmp, &mut state.k4, &mut *sys.params)?;

    for i in 0..dim {
        state.ytmp[i] = y[i] + h * (B5[0] * state.k1[i] + B5[1] * state.k2[i] + B5[2] * state.k3[i] + B5[3] * state.k4[i]);
    }

    (sys.function)(t + AH[3] * h, &state.ytmp, &mut state.k5, &mut *sys.params)?;

    for i in 0..dim {
        state.ytmp[i] = y[i] + h * (B6[0] * state.k1[i] + B6[1] * state.k2[i] + B6[2] * state.k3[i] + B6[3] * state.k4[i] + B6[4] * state.k5[i]);
    }

    (sys.function)(t + AH[4] * h, &state.ytmp, &mut state.k6, &mut *sys.params)?;

    for i in 0..dim {
        let d_i = C1 * state.k1[i] + C3 * state.k3[i] + C4 * state.k4[i] + C5 * state.k5[i] + C6 * state.k6[i];
        y[i] += h * d_i;
    }

    if let Some(dydt) = dydt_out {
        if let Err(e) = (sys.function)(t + h, y, dydt, &mut *sys.params) {
            y.copy_from_slice(&state.y0);
            return Err(e);
        }
    }

    for i in 0..dim {
        yerr[i] = h * (EC[1] * state.k1[i] + EC[3] * state.k3[i] + EC[4] * state.k4[i] + EC[5] * state.k5[i] + EC[6] * state.k6[i]);
    }

    Ok(())
}

pub struct GslOdeivStepRkf45;

impl GslOdeivStepRkf45 {
    pub fn new() -> Self {
        Self
    }

    pub fn alloc(&self, dim: usize) -> Option<Rkf45State> {
        Rkf45State::new(dim)
    }

    pub fn apply(
        &self,
        state: &mut Rkf45State,
        dim: usize,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &GslOdeivSystem,
    ) -> Result<(), GslError> {
        rkf45_apply(state, dim, t, h, y, yerr, dydt_in, dydt_out, sys)
    }

    pub fn reset(&self, state: &mut Rkf45State, dim: usize) -> Result<(), GslError> {
        state.reset();
        Ok(())
    }

    pub fn order(&self, _state: &Rkf45State) -> u32 {
        5
    }
}