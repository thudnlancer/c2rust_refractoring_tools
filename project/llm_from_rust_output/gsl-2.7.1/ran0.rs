use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Eof,
    Etolg,
    Etolx,
    Etolf,
    Enoprogj,
    Enoprog,
    Etable,
    Ecache,
    Eunimpl,
    Eunsup,
    Ediverge,
    Esing,
    Enotsqr,
    Ebadlen,
    Eround,
    Eloss,
    Eovrflw,
    Eundrflw,
    Etol,
    Ebadtol,
    Ezerodiv,
    Emaxiter,
    Erunaway,
    Ebadfunc,
    Enomem,
    Esanity,
    Efactor,
    Efailed,
    Einval,
    Efault,
    Erange,
    Edom,
    Continue,
    Failure,
    Success,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub type SizeT = usize;

#[derive(Debug, Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: SizeT,
    pub set: fn(&mut Ran0State, u64) -> Result<(), GslError>,
    pub get: fn(&mut Ran0State) -> u64,
    pub get_double: fn(&mut Ran0State) -> f64,
}

#[derive(Debug, Clone)]
pub struct Ran0State {
    x: u64,
}

const M: i64 = 2147483647;
const A: i64 = 16807;
const Q: i64 = 127773;
const R: i64 = 2836;
const MASK: u64 = 123459876;

fn ran0_get(state: &mut Ran0State) -> u64 {
    let x = state.x;
    let h = (x / Q as u64) as i64;
    let t = (A as u64)
        .wrapping_mul(x.wrapping_sub((h * Q) as u64))
        .wrapping_sub((h * R) as u64) as i64;
    
    state.x = if t < 0 { (t + M) as u64 } else { t as u64 };
    state.x
}

fn ran0_get_double(state: &mut Ran0State) -> f64 {
    ran0_get(state) as f64 / 2147483647.0
}

fn ran0_set(state: &mut Ran0State, s: u64) -> Result<(), GslError> {
    if s == MASK {
        return Err(GslError::Einval);
    }
    state.x = s ^ MASK;
    Ok(())
}

pub static GSL_RNG_RAN0: GslRngType = GslRngType {
    name: "ran0",
    max: 2147483646,
    min: 1,
    size: std::mem::size_of::<Ran0State>(),
    set: ran0_set,
    get: ran0_get,
    get_double: ran0_get_double,
};