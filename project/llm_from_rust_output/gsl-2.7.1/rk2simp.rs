use gsl::{
    odeiv::{Step, StepType},
    permutation::Permutation,
    vector::Vector,
    matrix::Matrix,
    Value,
};
use std::{
    ffi::CString,
    mem,
    ptr,
    slice,
};

#[derive(Debug)]
struct Rk2SimpState {
    y1: Vec<f64>,
    y0: Vec<f64>,
    y0_orig: Vec<f64>,
    ytmp: Vec<f64>,
    dfdy: Matrix,
    dfdt: Vec<f64>,
    y_onestep: Vec<f64>,
    p: Permutation,
}

impl Rk2SimpState {
    fn new(dim: usize) -> Option<Self> {
        let p = Permutation::new(dim).ok()?;
        Some(Self {
            y1: vec![0.0; dim],
            y0: vec![0.0; dim],
            y0_orig: vec![0.0; dim],
            ytmp: vec![0.0; dim],
            dfdy: Matrix::new(dim, dim).ok()?,
            dfdt: vec![0.0; dim],
            y_onestep: vec![0.0; dim],
            p,
        })
    }

    fn reset(&mut self, dim: usize) -> Result<(), Value> {
        self.y1.iter_mut().for_each(|x| *x = 0.0);
        self.y0.iter_mut().for_each(|x| *x = 0.0);
        self.y0_orig.iter_mut().for_each(|x| *x = 0.0);
        self.ytmp.iter_mut().for_each(|x| *x = 0.0);
        self.dfdy.set_zero();
        self.dfdt.iter_mut().for_each(|x| *x = 0.0);
        self.y_onestep.iter_mut().for_each(|x| *x = 0.0);
        Ok(())
    }
}

fn rk2simp_step(
    y: &mut [f64],
    state: &mut Rk2SimpState,
    h: f64,
    t: f64,
    sys: &gsl::odeiv::System,
) -> Result<(), Value> {
    let dim = sys.dimension();
    let y0 = &state.y0;
    
    // Compute Jacobian
    sys.jacobian(t, y0, &mut state.dfdy, &mut state.dfdt)?;
    
    // Scale matrix and solve system
    state.dfdy.scale(-h / 2.0);
    state.dfdy.add_diagonal(1.0)?;
    
    let mut signum = 0;
    let mut lu = state.dfdy.clone();
    lu.decomp(&mut state.p, &mut signum)?;
    
    // Compute intermediate step
    sys.function(t + 0.5 * h, y0, &mut state.ytmp)?;
    
    let ytmp_view = Vector::view(&mut state.ytmp);
    let mut y1_view = Vector::view(&mut state.y1);
    
    lu.solve(&state.p, &ytmp_view, &mut y1_view)?;
    y1_view.scale(0.5 * h);
    y1_view.add(&Vector::view(y0))?;
    
    // Compute final step
    sys.function(t + 0.5 * h, &state.y1, &mut state.ytmp)?;
    
    for i in 0..dim {
        y[i] = y0[i] + h * state.ytmp[i];
    }
    
    Ok(())
}

fn rk2simp_apply(
    state: &mut Rk2SimpState,
    t: f64,
    h: f64,
    y: &mut [f64],
    yerr: &mut [f64],
    dydt_out: Option<&mut [f64]>,
    sys: &gsl::odeiv::System,
) -> Result<(), Value> {
    let dim = sys.dimension();
    
    state.y0.copy_from_slice(y);
    state.y0_orig.copy_from_slice(y);
    state.y_onestep.copy_from_slice(y);
    
    // Compute one-step solution
    rk2simp_step(&mut state.y_onestep, state, h, t, sys)?;
    
    // Compute two half-steps
    rk2simp_step(y, state, h / 2.0, t, sys)?;
    state.y0.copy_from_slice(y);
    rk2simp_step(y, state, h / 2.0, t + h / 2.0, sys)?;
    
    // Compute derivative if needed
    if let Some(dydt) = dydt_out {
        sys.function(t + h, y, dydt)?;
    }
    
    // Compute error estimate
    for i in 0..dim {
        yerr[i] = 4.0 * (y[i] - state.y_onestep[i]) / 3.0;
    }
    
    Ok(())
}

pub static RK2SIMP: StepType = StepType {
    name: CString::new("rk2simp").unwrap(),
    can_use_dydt_in: false,
    gives_exact_dydt_out: true,
    alloc: |dim| {
        Box::new(Rk2SimpState::new(dim).ok_or(Value::NoMem).map(|s| Box::into_raw(s) as _)
    },
    apply: |state, t, h, y, yerr, dydt_in, dydt_out, sys| {
        let state = unsafe { &mut *(state as *mut Rk2SimpState) };
        rk2simp_apply(state, t, h, y, yerr, dydt_out, sys)
            .map_err(|e| e as i32)
            .unwrap_or_else(|e| e)
    },
    reset: |state, dim| {
        let state = unsafe { &mut *(state as *mut Rk2SimpState) };
        state.reset(dim).map_err(|e| e as i32).unwrap_or_else(|e| e)
    },
    order: |_| 2,
    free: |state| {
        if !state.is_null() {
            unsafe { Box::from_raw(state as *mut Rk2SimpState) };
        }
    },
};

pub fn rk2simp() -> Step {
    Step::new(&RK2SIMP).expect("Failed to create RK2SIMP step")
}