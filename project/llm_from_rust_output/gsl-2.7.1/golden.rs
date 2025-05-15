use std::ffi::CStr;
use std::os::raw::{c_double, c_int, c_void};

pub type size_t = usize;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
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
    Continue = -2,
    Failure = -1,
    Success = 0,
}

pub trait GslFunction {
    fn evaluate(&self, x: f64) -> f64;
}

#[derive(Clone)]
pub struct GslMinFminimizerType {
    name: &'static str,
    size: size_t,
    set: fn(&mut dyn std::any::Any, &dyn GslFunction, f64, f64, f64, f64, f64, f64) -> Result<(), GslError>,
    iterate: fn(&mut dyn std::any::Any, &dyn GslFunction, &mut f64, &mut f64, &mut f64, &mut f64, &mut f64, &mut f64) -> Result<(), GslError>,
}

#[derive(Default, Clone)]
struct GoldensectionState {
    dummy: f64,
}

fn goldensection_init(
    state: &mut dyn std::any::Any,
    _f: &dyn GslFunction,
    _x_minimum: f64,
    _f_minimum: f64,
    _x_lower: f64,
    _f_lower: f64,
    _x_upper: f64,
    _f_upper: f64,
) -> Result<(), GslError> {
    let _ = state.downcast_mut::<GoldensectionState>().unwrap();
    Ok(())
}

fn goldensection_iterate(
    state: &mut dyn std::any::Any,
    f: &dyn GslFunction,
    x_minimum: &mut f64,
    f_minimum: &mut f64,
    x_lower: &mut f64,
    f_lower: &mut f64,
    x_upper: &mut f64,
    f_upper: &mut f64,
) -> Result<(), GslError> {
    let _state = state.downcast_mut::<GoldensectionState>().unwrap();
    let x_center = *x_minimum;
    let x_left = *x_lower;
    let x_right = *x_upper;
    let f_min = *f_minimum;
    let golden = 0.3819660f64;
    
    let w_lower = x_center - x_left;
    let w_upper = x_right - x_center;
    
    let x_new = x_center + golden * if w_upper > w_lower { w_upper } else { -w_lower };
    let f_new = f.evaluate(x_new);
    
    if !f_new.is_finite() {
        return Err(GslError::Badfunc);
    }
    
    if f_new < f_min {
        *x_minimum = x_new;
        *f_minimum = f_new;
        Ok(())
    } else if x_new < x_center && f_new > f_min {
        *x_lower = x_new;
        *f_lower = f_new;
        Ok(())
    } else if x_new > x_center && f_new > f_min {
        *x_upper = x_new;
        *f_upper = f_new;
        Ok(())
    } else {
        Err(GslError::Failure)
    }
}

pub static GSL_MIN_FMINIMIZER_GOLDENSECTION: GslMinFminimizerType = GslMinFminimizerType {
    name: "goldensection",
    size: std::mem::size_of::<GoldensectionState>(),
    set: goldensection_init,
    iterate: goldensection_iterate,
};