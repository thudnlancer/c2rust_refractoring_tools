use std::ffi::CString;
use std::ptr;

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

pub type SizeT = usize;

pub struct OdeivSystem {
    pub function: Option<fn(f64, &[f64], &mut [f64], &mut dyn std::any::Any) -> Result<(), GslError>>,
    pub jacobian: Option<fn(f64, &[f64], &mut [f64], &mut [f64], &mut dyn std::any::Any) -> Result<(), GslError>>,
    pub dimension: SizeT,
    pub params: Box<dyn std::any::Any>,
}

pub struct OdeivStepType {
    pub name: CString,
    pub can_use_dydt_in: bool,
    pub gives_exact_dydt_out: bool,
    pub alloc: Option<fn(SizeT) -> Box<dyn std::any::Any>>,
    pub apply: Option<fn(&mut dyn std::any::Any, SizeT, f64, f64, &mut [f64], &mut [f64], &[f64], &mut [f64], &OdeivSystem) -> Result<(), GslError>>,
    pub reset: Option<fn(&mut dyn std::any::Any, SizeT) -> Result<(), GslError>>,
    pub order: Option<fn(&mut dyn std::any::Any) -> u32>,
    pub free: Option<fn(&mut dyn std::any::Any)>,
}

pub struct OdeivStep {
    pub type_: &'static OdeivStepType,
    pub dimension: SizeT,
    pub state: Box<dyn std::any::Any>,
}

pub struct OdeivControlType {
    pub name: CString,
    pub alloc: Option<fn() -> Box<dyn std::any::Any>>,
    pub init: Option<fn(&mut dyn std::any::Any, f64, f64, f64, f64) -> Result<(), GslError>>,
    pub hadjust: Option<fn(&mut dyn std::any::Any, SizeT, u32, &[f64], &[f64], &[f64], &mut f64) -> Result<(), GslError>>,
    pub free: Option<fn(&mut dyn std::any::Any)>,
}

pub struct OdeivControl {
    pub type_: &'static OdeivControlType,
    pub state: Box<dyn std::any::Any>,
}

impl OdeivControl {
    pub fn new(type_: &'static OdeivControlType) -> Result<Self, GslError> {
        let state = match type_.alloc {
            Some(alloc) => alloc(),
            None => return Err(GslError::Nomem),
        };

        Ok(Self { type_, state })
    }

    pub fn init(&mut self, eps_abs: f64, eps_rel: f64, a_y: f64, a_dydt: f64) -> Result<(), GslError> {
        match self.type_.init {
            Some(init) => init(&mut *self.state, eps_abs, eps_rel, a_y, a_dydt),
            None => Ok(()),
        }
    }

    pub fn name(&self) -> &str {
        self.type_.name.to_str().unwrap_or("")
    }

    pub fn hadjust(
        &mut self,
        step: &mut OdeivStep,
        y: &[f64],
        yerr: &[f64],
        dydt: &[f64],
        h: &mut f64,
    ) -> Result<(), GslError> {
        match (self.type_.hadjust, step.type_.order) {
            (Some(hadjust), Some(order)) => {
                let order = order(&mut *step.state);
                hadjust(&mut *self.state, step.dimension, order, y, yerr, dydt, h)
            }
            _ => Ok(()),
        }
    }
}

impl Drop for OdeivControl {
    fn drop(&mut self) {
        if let Some(free) = self.type_.free {
            free(&mut *self.state);
        }
    }
}