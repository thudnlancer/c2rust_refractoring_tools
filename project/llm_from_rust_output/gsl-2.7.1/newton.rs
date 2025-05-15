use gsl::{
    enums::GslResult,
    matrix::Matrix,
    permutation::Permutation,
    vector::Vector,
    multiroot::{MultirootFunctionFdf, MultirootFdfsolverType},
};

struct NewtonState {
    lu: Matrix,
    permutation: Permutation,
}

fn newton_alloc(n: usize) -> Result<NewtonState, GslResult> {
    let lu = Matrix::new(n, n).map_err(|_| GslResult::NoMem)?;
    let permutation = Permutation::new(n).map_err(|_| GslResult::NoMem)?;
    Ok(NewtonState { lu, permutation })
}

fn newton_set(
    state: &mut NewtonState,
    fdf: &mut MultirootFunctionFdf,
    x: &mut Vector,
    f: &mut Vector,
    j: &mut Matrix,
    dx: &mut Vector,
) -> GslResult {
    fdf.fdf(x, f, j)?;
    dx.set_zero();
    GslResult::Success
}

fn newton_iterate(
    state: &mut NewtonState,
    fdf: &mut MultirootFunctionFdf,
    x: &mut Vector,
    f: &mut Vector,
    j: &mut Matrix,
    dx: &mut Vector,
) -> GslResult {
    state.lu.copy_from(j)?;
    state.lu.lu_decomp(&mut state.permutation)?;
    
    state.lu.lu_solve(&state.permutation, f, dx)?;
    
    for i in 0..x.len() {
        let e = dx.get(i);
        let y = x.get(i);
        dx.set(i, -e);
        x.set(i, y - e);
    }
    
    fdf.fdf(x, f, j)?;
    GslResult::Success
}

static NEWTON_TYPE: MultirootFdfsolverType = MultirootFdfsolverType {
    name: "newton",
    size: std::mem::size_of::<NewtonState>(),
    alloc: newton_alloc,
    set: newton_set,
    iterate: newton_iterate,
    free: |state| {
        // Resources will be automatically dropped
        GslResult::Success
    },
};

pub static GSL_MULTIROOT_FDFSOLVER_NEWTON: &MultirootFdfsolverType = &NEWTON_TYPE;