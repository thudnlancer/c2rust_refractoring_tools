use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::{c_int, c_uint, c_double, c_void, c_char};

pub type size_t = usize;

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

#[derive(Clone, Copy)]
pub struct GslOdeivSystem {
    pub function: Option<
        extern "C" fn(
            t: c_double,
            y: *const c_double,
            dydt: *mut c_double,
            params: *mut c_void,
        ) -> c_int,
    >,
    pub jacobian: Option<
        extern "C" fn(
            t: c_double,
            y: *const c_double,
            dfdy: *mut c_double,
            dfdt: *mut c_double,
            params: *mut c_void,
        ) -> c_int,
    >,
    pub dimension: size_t,
    pub params: *mut c_void,
}

#[derive(Clone, Copy)]
pub struct GslOdeivStepType {
    pub name: *const c_char,
    pub can_use_dydt_in: c_int,
    pub gives_exact_dydt_out: c_int,
    pub alloc: Option<extern "C" fn(size_t) -> *mut c_void>,
    pub apply: Option<
        extern "C" fn(
            *mut c_void,
            size_t,
            c_double,
            c_double,
            *mut c_double,
            *mut c_double,
            *const c_double,
            *mut c_double,
            *const GslOdeivSystem,
        ) -> c_int,
    >,
    pub reset: Option<extern "C" fn(*mut c_void, size_t) -> c_int>,
    pub order: Option<extern "C" fn(*mut c_void) -> c_uint>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

struct Gear1State {
    k: Vec<c_double>,
    y0: Vec<c_double>,
    y0_orig: Vec<c_double>,
    y_onestep: Vec<c_double>,
}

impl Gear1State {
    fn new(dim: size_t) -> Option<Box<Self>> {
        Some(Box::new(Self {
            k: vec![0.0; dim],
            y0: vec![0.0; dim],
            y0_orig: vec![0.0; dim],
            y_onestep: vec![0.0; dim],
        }))
    }

    fn reset(&mut self, dim: size_t) {
        self.k.iter_mut().take(dim).for_each(|x| *x = 0.0);
        self.y0.iter_mut().take(dim).for_each(|x| *x = 0.0);
        self.y0_orig.iter_mut().take(dim).for_each(|x| *x = 0.0);
        self.y_onestep.iter_mut().take(dim).for_each(|x| *x = 0.0);
    }
}

fn gear1_alloc(dim: size_t) -> *mut c_void {
    match Gear1State::new(dim) {
        Some(state) => Box::into_raw(state) as *mut c_void,
        None => {
            gsl_error(
                b"failed to allocate space for gear1_state\0".as_ptr() as *const c_char,
                b"gear1.c\0".as_ptr() as *const c_char,
                56,
                GslError::Nomem as c_int,
            );
            ptr::null_mut()
        }
    }
}

fn gear1_step(
    y: &mut [c_double],
    state: &mut Gear1State,
    h: c_double,
    t: c_double,
    sys: &GslOdeivSystem,
) -> Result<(), GslError> {
    const ITER_STEPS: c_int = 3;
    let dim = sys.dimension;

    for _ in 0..ITER_STEPS {
        let mut k = vec![0.0; dim];
        if let Some(func) = sys.function {
            let status = func(t + h, y.as_ptr(), k.as_mut_ptr(), sys.params);
            if status != GslError::Success as c_int {
                return Err(unsafe { mem::transmute(status) });
            }
        }

        for i in 0..dim {
            y[i] = state.y0[i] + h * k[i];
        }
    }

    Ok(())
}

fn gear1_apply(
    vstate: *mut c_void,
    dim: size_t,
    t: c_double,
    h: c_double,
    y: *mut c_double,
    yerr: *mut c_double,
    _dydt_in: *const c_double,
    dydt_out: *mut c_double,
    sys: *const GslOdeivSystem,
) -> c_int {
    let state = unsafe { &mut *(vstate as *mut Gear1State) };
    let sys = unsafe { &*sys };
    let y = unsafe { std::slice::from_raw_parts_mut(y, dim) };
    let yerr = unsafe { std::slice::from_raw_parts_mut(yerr, dim) };

    state.y0.copy_from_slice(y);
    state.y0_orig.copy_from_slice(y);
    state.y_onestep.copy_from_slice(y);

    if let Err(e) = gear1_step(&mut state.y_onestep, state, h, t, sys) {
        return e as c_int;
    }

    if let Err(e) = gear1_step(y, state, h / 2.0, t, sys) {
        y.copy_from_slice(&state.y0_orig);
        return e as c_int;
    }

    state.y0.copy_from_slice(y);

    if let Err(e) = gear1_step(y, state, h / 2.0, t + h / 2.0, sys) {
        y.copy_from_slice(&state.y0_orig);
        return e as c_int;
    }

    if !dydt_out.is_null() {
        let dydt_out = unsafe { std::slice::from_raw_parts_mut(dydt_out, dim) };
        if let Some(func) = sys.function {
            let status = func(t + h, y.as_ptr(), dydt_out.as_mut_ptr(), sys.params);
            if status != GslError::Success as c_int {
                y.copy_from_slice(&state.y0_orig);
                return status;
            }
        }
    }

    for i in 0..dim {
        yerr[i] = 4.0 * (y[i] - state.y_onestep[i]);
    }

    GslError::Success as c_int
}

fn gear1_reset(vstate: *mut c_void, dim: size_t) -> c_int {
    let state = unsafe { &mut *(vstate as *mut Gear1State) };
    state.reset(dim);
    GslError::Success as c_int
}

fn gear1_order(_vstate: *mut c_void) -> c_uint {
    1
}

fn gear1_free(vstate: *mut c_void) {
    unsafe { Box::from_raw(vstate as *mut Gear1State) };
}

fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int) {
    unsafe {
        let reason = CStr::from_ptr(reason).to_string_lossy();
        let file = CStr::from_ptr(file).to_string_lossy();
        eprintln!(
            "GSL error: {} at {}:{} (code {})",
            reason, file, line, gsl_errno
        );
    }
}

static GEAR1_TYPE: GslOdeivStepType = GslOdeivStepType {
    name: b"gear1\0".as_ptr() as *const c_char,
    can_use_dydt_in: 0,
    gives_exact_dydt_out: 1,
    alloc: Some(gear1_alloc),
    apply: Some(gear1_apply),
    reset: Some(gear1_reset),
    order: Some(gear1_order),
    free: Some(gear1_free),
};

#[no_mangle]
pub static gsl_odeiv_step_gear1: *const GslOdeivStepType = &GEAR1_TYPE;