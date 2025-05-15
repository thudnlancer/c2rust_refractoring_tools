/* roots/secant.rs

   The secant algorithm is a variant of the Newton algorithm with the
   derivative term replaced by a numerical estimate from the last two
   function evaluations.

   x[i+1] = x[i] - f(x[i]) / f'_est

   where f'_est = (f(x[i]) - f(x[i-1])) / (x[i] - x[i-1])

   The exact derivative is used for the initial value of f'_est.
*/

use std::f64;

#[derive(Debug)]
struct SecantState {
    f: f64,
    df: f64,
}

#[derive(Debug)]
pub struct GslFunctionFdf {
    pub f: Box<dyn Fn(f64) -> f64>,
    pub df: Box<dyn Fn(f64) -> f64>,
    pub fdf: Box<dyn Fn(f64) -> (f64, f64)>,
}

#[derive(Debug)]
pub struct GslRootFdfsolverType {
    pub name: &'static str,
    pub size: usize,
    pub init: fn(&mut SecantState, &GslFunctionFdf, &mut f64) -> Result<(), GslError>,
    pub iterate: fn(&mut SecantState, &GslFunctionFdf, &mut f64) -> Result<(), GslError>,
}

#[derive(Debug, PartialEq)]
pub enum GslError {
    ZeroDiv,
    BadFunc,
    Success,
}

fn secant_init(state: &mut SecantState, fdf: &GslFunctionFdf, root: &mut f64) -> Result<(), GslError> {
    let x = *root;
    let (f, df) = (fdf.fdf)(x);
    state.f = f;
    state.df = df;
    Ok(())
}

fn secant_iterate(state: &mut SecantState, fdf: &GslFunctionFdf, root: &mut f64) -> Result<(), GslError> {
    let x = *root;
    let f = state.f;
    let df = state.df;

    if f == 0.0 {
        return Ok(());
    }

    if df == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    let x_new = x - (f / df);
    let f_new = (fdf.f)(x_new);
    let df_new = df * ((f - f_new) / f);

    *root = x_new;
    state.f = f_new;
    state.df = df_new;

    if !f_new.is_finite() {
        return Err(GslError::BadFunc);
    }

    if !df_new.is_finite() {
        return Err(GslError::BadFunc);
    }

    Ok(())
}

pub static SECANT_TYPE: GslRootFdfsolverType = GslRootFdfsolverType {
    name: "secant",
    size: std::mem::size_of::<SecantState>(),
    init: secant_init,
    iterate: secant_iterate,
};

pub static GSL_ROOT_FDFSOLVER_SECANT: &'static GslRootFdfsolverType = &SECANT_TYPE;