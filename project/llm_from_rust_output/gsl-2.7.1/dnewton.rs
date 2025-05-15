use gsl::{Matrix, Vector, Permutation, MultirootFunction, MultirootFsolverType, Error, Result};
use std::ffi::CString;

#[derive(Debug)]
pub struct DNewtonState {
    j: Matrix,
    lu: Matrix,
    permutation: Permutation,
}

impl DNewtonState {
    pub fn new(n: usize) -> Result<Self> {
        let lu = Matrix::new(n, n)?;
        let permutation = Permutation::new(n)?;
        let j = Matrix::new(n, n)?;
        
        Ok(Self { j, lu, permutation })
    }
}

pub fn dnewton_alloc(vstate: &mut Option<Box<DNewtonState>>, n: usize) -> Result<()> {
    *vstate = Some(Box::new(DNewtonState::new(n)?));
    Ok(())
}

pub fn dnewton_set(
    state: &mut DNewtonState,
    function: &mut MultirootFunction,
    x: &mut Vector,
    f: &mut Vector,
    dx: &mut Vector,
) -> Result<()> {
    function.eval(x, f)?;
    function.fdjacobian(x, f, 1.4901161193847656e-08, &mut state.j)?;
    
    dx.set_zero();
    Ok(())
}

pub fn dnewton_iterate(
    state: &mut DNewtonState,
    function: &mut MultirootFunction,
    x: &mut Vector,
    f: &mut Vector,
    dx: &mut Vector,
) -> Result<()> {
    state.lu.copy_from(&state.j)?;
    let mut signum = 0;
    state.lu.lu_decomp(&mut state.permutation, &mut signum)?;
    state.lu.lu_solve(&state.permutation, f, dx)?;
    
    for i in 0..x.len() {
        let e = dx.get(i);
        let y = x.get(i);
        dx.set(i, -e);
        x.set(i, y - e);
    }
    
    function.eval(x, f)?;
    function.fdjacobian(x, f, 1.4901161193847656e-08, &mut state.j)?;
    Ok(())
}

pub fn dnewton_free(state: Option<Box<DNewtonState>>) {
    // Memory is automatically freed when Box goes out of scope
}

pub static GSL_MULTIROOT_FSOLVER_DNEWTON: MultirootFsolverType = MultirootFsolverType {
    name: CString::new("dnewton").unwrap(),
    size: std::mem::size_of::<DNewtonState>(),
    alloc: dnewton_alloc,
    set: dnewton_set,
    iterate: dnewton_iterate,
    free: dnewton_free,
};