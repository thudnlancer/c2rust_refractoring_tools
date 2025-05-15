use gsl::{
    enums::GslResult,
    matrix::Matrix,
    permutation::Permutation,
    vector::Vector,
    multiroot::{MultirootFunction, MultirootFsolverType},
};

pub struct BroydenState {
    h: Matrix,
    lu: Matrix,
    permutation: Permutation,
    v: Vector,
    w: Vector,
    y: Vector,
    p: Vector,
    fnew: Vector,
    x_trial: Vector,
    phi: f64,
}

impl BroydenState {
    pub fn new(n: usize) -> Result<Self, GslResult> {
        Ok(Self {
            h: Matrix::new(n, n)?,
            lu: Matrix::new(n, n)?,
            permutation: Permutation::new(n)?,
            v: Vector::new(n)?,
            w: Vector::new(n)?,
            y: Vector::new(n)?,
            p: Vector::new(n)?,
            fnew: Vector::new(n)?,
            x_trial: Vector::new(n)?,
            phi: 0.0,
        })
    }
}

pub fn broyden_alloc(vstate: &mut Option<Box<BroydenState>>, n: usize) -> GslResult {
    *vstate = Some(Box::new(BroydenState::new(n)?));
    GslResult::Success
}

pub fn broyden_set(
    state: &mut BroydenState,
    function: &mut MultirootFunction,
    x: &mut Vector,
    f: &mut Vector,
    dx: &mut Vector,
) -> GslResult {
    function.f(x, f)?;
    
    function.fdjacobian(x, f, 1.4901161193847656e-08, &mut state.lu)?;
    let mut signum = 0;
    state.lu.lu_decomp(&mut state.permutation, &mut signum)?;
    state.lu.lu_invert(&state.permutation, &mut state.h)?;
    
    for i in 0..function.n {
        for j in 0..function.n {
            state.h.set(i, j, -state.h.get(i, j));
        }
    }
    
    dx.set_zero();
    state.phi = enorm(f);
    GslResult::Success
}

pub fn broyden_iterate(
    state: &mut BroydenState,
    function: &mut MultirootFunction,
    x: &mut Vector,
    f: &mut Vector,
    dx: &mut Vector,
) -> GslResult {
    let n = function.n;
    let mut p = Vector::new(n)?;
    
    // Calculate p = -H*f
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            sum += state.h.get(i, j) * f.get(j);
        }
        p.set(i, sum);
    }
    
    let mut t = 1.0;
    let mut iter = 0;
    let phi0 = state.phi;
    let mut phi1;
    
    loop {
        // x_trial = x + t*p
        for i in 0..n {
            state.x_trial.set(i, x.get(i) + t * p.get(i));
        }
        
        function.f(&state.x_trial, &mut state.fnew)?;
        phi1 = enorm(&state.fnew);
        iter += 1;
        
        if !(phi1 > phi0 && iter < 10 && t > 0.1) {
            break;
        }
        
        let theta = phi1 / phi0;
        t *= (f64::sqrt(1.0 + 6.0 * theta) - 1.0) / (3.0 * theta);
    }
    
    if phi1 > phi0 {
        // Recompute Jacobian
        let mut signum = 0;
        function.fdjacobian(x, f, 1.4901161193847656e-08, &mut state.lu)?;
        
        for i in 0..n {
            for j in 0..n {
                state.lu.set(i, j, -state.lu.get(i, j));
            }
        }
        
        state.lu.lu_decomp(&mut state.permutation, &mut signum)?;
        state.lu.lu_invert(&state.permutation, &mut state.h)?;
        state.h.lu_solve(&state.permutation, f, &mut p)?;
        
        t = 1.0;
        for i in 0..n {
            state.x_trial.set(i, x.get(i) + t * p.get(i));
        }
        
        function.f(&state.x_trial, &mut state.fnew)?;
        phi1 = enorm(&state.fnew);
    }
    
    // Update y = fnew - f
    for i in 0..n {
        state.y.set(i, state.fnew.get(i) - f.get(i));
    }
    
    // Update H using Broyden's formula
    let mut v = Vector::new(n)?;
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            sum += state.h.get(i, j) * state.y.get(j);
        }
        v.set(i, sum);
    }
    
    let mut lambda = 0.0;
    for i in 0..n {
        lambda += p.get(i) * v.get(i);
    }
    
    if lambda == 0.0 {
        return GslResult::Failure;
    }
    
    for i in 0..n {
        v.set(i, v.get(i) + t * p.get(i));
    }
    
    let mut w = Vector::new(n)?;
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            sum += state.h.get(j, i) * p.get(j);
        }
        w.set(i, sum);
    }
    
    for i in 0..n {
        let vi = v.get(i);
        for j in 0..n {
            let wj = w.get(j);
            let hij = state.h.get(i, j) - vi * wj / lambda;
            state.h.set(i, j, hij);
        }
    }
    
    f.copy_from(&state.fnew)?;
    x.copy_from(&state.x_trial)?;
    
    for i in 0..n {
        dx.set(i, t * p.get(i));
    }
    
    state.phi = phi1;
    GslResult::Success
}

pub fn broyden_free(state: &mut Option<Box<BroydenState>>) {
    *state = None;
}

fn enorm(f: &Vector) -> f64 {
    let mut e2 = 0.0;
    for i in 0..f.len() {
        let fi = f.get(i);
        e2 += fi * fi;
    }
    e2.sqrt()
}

pub static BROYDEN_TYPE: MultirootFsolverType = MultirootFsolverType {
    name: "broyden",
    size: std::mem::size_of::<BroydenState>(),
    alloc: broyden_alloc,
    set: broyden_set,
    iterate: broyden_iterate,
    free: broyden_free,
};