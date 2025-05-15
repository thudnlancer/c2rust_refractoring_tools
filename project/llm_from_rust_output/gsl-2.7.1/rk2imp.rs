use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void, c_double, c_uint};
use std::slice;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GslOdeivSystem {
    pub function: Option<
        extern "C" fn(
            c_double,
            *const c_double,
            *mut c_double,
            *mut c_void,
        ) -> c_int,
    >,
    pub jacobian: Option<
        extern "C" fn(
            c_double,
            *const c_double,
            *mut c_double,
            *mut c_double,
            *mut c_void,
        ) -> c_int,
    >,
    pub dimension: usize,
    pub params: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GslOdeivStepType {
    pub name: *const c_char,
    pub can_use_dydt_in: c_int,
    pub gives_exact_dydt_out: c_int,
    pub alloc: Option<extern "C" fn(usize) -> *mut c_void>,
    pub apply: Option<
        extern "C" fn(
            *mut c_void,
            usize,
            c_double,
            c_double,
            *mut c_double,
            *mut c_double,
            *const c_double,
            *mut c_double,
            *const GslOdeivSystem,
        ) -> c_int,
    >,
    pub reset: Option<extern "C" fn(*mut c_void, usize) -> c_int>,
    pub order: Option<extern "C" fn(*mut c_void) -> c_uint>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Rk2ImpState {
    y1: Vec<c_double>,
    y0: Vec<c_double>,
    ytmp: Vec<c_double>,
    y_onestep: Vec<c_double>,
    y0_orig: Vec<c_double>,
}

impl Rk2ImpState {
    fn new(dim: usize) -> Option<Box<Self>> {
        Some(Box::new(Rk2ImpState {
            y1: vec![0.0; dim],
            y0: vec![0.0; dim],
            ytmp: vec![0.0; dim],
            y_onestep: vec![0.0; dim],
            y0_orig: vec![0.0; dim],
        }))
    }

    fn reset(&mut self) {
        self.y1.iter_mut().for_each(|x| *x = 0.0);
        self.ytmp.iter_mut().for_each(|x| *x = 0.0);
        self.y0.iter_mut().for_each(|x| *x = 0.0);
        self.y_onestep.iter_mut().for_each(|x| *x = 0.0);
        self.y0_orig.iter_mut().for_each(|x| *x = 0.0);
    }
}

fn rk2imp_alloc(dim: usize) -> *mut c_void {
    match Rk2ImpState::new(dim) {
        Some(state) => Box::into_raw(state) as *mut c_void,
        None => {
            gsl_error(
                CString::new("failed to allocate space for rk2imp_state").unwrap().as_ptr(),
                CString::new("rk2imp.c").unwrap().as_ptr(),
                58,
                GslError::Nomem as c_int,
            );
            ptr::null_mut()
        }
    }
}

fn rk2imp_reset(vstate: *mut c_void, dim: usize) -> c_int {
    if vstate.is_null() {
        return GslError::Fault as c_int;
    }
    unsafe {
        let state = &mut *(vstate as *mut Rk2ImpState);
        state.reset();
    }
    GslError::Success as c_int
}

fn rk2imp_free(vstate: *mut c_void) {
    if !vstate.is_null() {
        unsafe {
            Box::from_raw(vstate as *mut Rk2ImpState);
        }
    }
}

fn rk2imp_order(_vstate: *mut c_void) -> c_uint {
    2
}

fn rk2imp_step(
    y: &mut [c_double],
    state: &mut Rk2ImpState,
    h: c_double,
    t: c_double,
    sys: &GslOdeivSystem,
) -> c_int {
    let max_iter = 3;
    let mut nu = 0;

    while nu < max_iter {
        for i in 0..sys.dimension {
            state.ytmp[i] = state.y0[i] + 0.5 * h * state.y1[i];
        }

        let s = match sys.function {
            Some(func) => func(
                t + 0.5 * h,
                state.ytmp.as_ptr(),
                state.y1.as_mut_ptr(),
                sys.params,
            ),
            None => return GslError::Badfunc as c_int,
        };

        if s != GslError::Success as c_int {
            return s;
        }
        nu += 1;
    }

    for i in 0..sys.dimension {
        y[i] = state.y0[i] + h * state.y1[i];
    }

    GslError::Success as c_int
}

fn rk2imp_apply(
    vstate: *mut c_void,
    dim: usize,
    t: c_double,
    h: c_double,
    y: *mut c_double,
    yerr: *mut c_double,
    dydt_in: *const c_double,
    dydt_out: *mut c_double,
    sys: *const GslOdeivSystem,
) -> c_int {
    if vstate.is_null() || sys.is_null() {
        return GslError::Fault as c_int;
    }

    let state = unsafe { &mut *(vstate as *mut Rk2ImpState) };
    let sys = unsafe { &*sys };
    let y_slice = unsafe { slice::from_raw_parts_mut(y, dim) };
    let yerr_slice = unsafe { slice::from_raw_parts_mut(yerr, dim) };

    state.y0.copy_from_slice(y_slice);
    state.y0_orig.copy_from_slice(y_slice);

    if !dydt_in.is_null() {
        let dydt_in_slice = unsafe { slice::from_raw_parts(dydt_in, dim) };
        state.y1.copy_from_slice(dydt_in_slice);
    } else {
        let s = match sys.function {
            Some(func) => func(t, y, state.y1.as_mut_ptr(), sys.params),
            None => return GslError::Badfunc as c_int,
        };
        if s != GslError::Success as c_int {
            return s;
        }
    }

    state.y_onestep.copy_from_slice(y_slice);
    let s = rk2imp_step(&mut state.y_onestep, state, h, t, sys);
    if s != GslError::Success as c_int {
        return s;
    }

    let s = rk2imp_step(y_slice, state, h / 2.0, t, sys);
    if s != GslError::Success as c_int {
        y_slice.copy_from_slice(&state.y0_orig);
        return s;
    }

    state.y0.copy_from_slice(y_slice);
    let s = match sys.function {
        Some(func) => func(t + h / 2.0, y, state.y1.as_mut_ptr(), sys.params),
        None => return GslError::Badfunc as c_int,
    };
    if s != GslError::Success as c_int {
        y_slice.copy_from_slice(&state.y0_orig);
        return s;
    }

    let s = rk2imp_step(y_slice, state, h / 2.0, t + h / 2.0, sys);
    if s != GslError::Success as c_int {
        y_slice.copy_from_slice(&state.y0_orig);
        return s;
    }

    if !dydt_out.is_null() {
        let dydt_out_slice = unsafe { slice::from_raw_parts_mut(dydt_out, dim) };
        let s = match sys.function {
            Some(func) => func(t + h, y, dydt_out_slice.as_mut_ptr(), sys.params),
            None => return GslError::Badfunc as c_int,
        };
        if s != GslError::Success as c_int {
            y_slice.copy_from_slice(&state.y0_orig);
            return s;
        }
    }

    for i in 0..dim {
        yerr_slice[i] = 4.0 * (y_slice[i] - state.y_onestep[i]) / 3.0;
    }

    GslError::Success as c_int
}

static RK2IMP_TYPE: GslOdeivStepType = GslOdeivStepType {
    name: b"rk2imp\0" as *const u8 as *const c_char,
    can_use_dydt_in: 1,
    gives_exact_dydt_out: 1,
    alloc: Some(rk2imp_alloc),
    apply: Some(rk2imp_apply),
    reset: Some(rk2imp_reset),
    order: Some(rk2imp_order),
    free: Some(rk2imp_free),
};

#[no_mangle]
pub static gsl_odeiv_step_rk2imp: *const GslOdeivStepType = &RK2IMP_TYPE;

extern "C" {
    fn gsl_error(
        reason: *const c_char,
        file: *const c_char,
        line: c_int,
        gsl_errno: c_int,
    );
}