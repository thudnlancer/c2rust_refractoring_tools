use std::mem;
use std::ptr;
use std::ffi::CString;
use libc::{c_int, c_double, c_void, c_char, c_uint, size_t};

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

#[repr(C)]
pub struct GslOdeivSystem {
    pub function: Option<extern "C" fn(c_double, *const c_double, *mut c_double, *mut c_void) -> c_int>,
    pub jacobian: Option<extern "C" fn(c_double, *const c_double, *mut c_double, *mut c_double, *mut c_void) -> c_int>,
    pub dimension: size_t,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct GslOdeivStepType {
    pub name: *const c_char,
    pub can_use_dydt_in: c_int,
    pub gives_exact_dydt_out: c_int,
    pub alloc: Option<extern "C" fn(size_t) -> *mut c_void>,
    pub apply: Option<extern "C" fn(*mut c_void, size_t, c_double, c_double, *mut c_double, *mut c_double, *const c_double, *mut c_double, *const GslOdeivSystem) -> c_int>,
    pub reset: Option<extern "C" fn(*mut c_void, size_t) -> c_int>,
    pub order: Option<extern "C" fn(*mut c_void) -> c_uint>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct GslOdeivStep {
    pub type_: *const GslOdeivStepType,
    pub dimension: size_t,
    pub state: *mut c_void,
}

#[repr(C)]
struct Gear2State {
    primed: c_int,
    t_primed: c_double,
    last_h: c_double,
    primer: *mut GslOdeivStep,
    yim1: *mut c_double,
    k: *mut c_double,
    y0: *mut c_double,
    y0_orig: *mut c_double,
    y_onestep: *mut c_double,
    stutter: c_int,
}

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
    static gsl_odeiv_step_rk4imp: *const GslOdeivStepType;
    fn gsl_odeiv_step_alloc(T: *const GslOdeivStepType, dim: size_t) -> *mut GslOdeivStep;
    fn gsl_odeiv_step_free(s: *mut GslOdeivStep);
    fn gsl_odeiv_step_apply(
        s: *mut GslOdeivStep,
        t: c_double,
        h: c_double,
        y: *mut c_double,
        yerr: *mut c_double,
        dydt_in: *const c_double,
        dydt_out: *mut c_double,
        dydt: *const GslOdeivSystem,
    ) -> c_int;
}

fn gear2_alloc(dim: size_t) -> *mut c_void {
    let state_size = mem::size_of::<Gear2State>();
    let state = unsafe { libc::malloc(state_size as libc::size_t) as *mut Gear2State };
    
    if state.is_null() {
        unsafe {
            gsl_error(
                b"failed to allocate space for gear2_state\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                56,
                GslError::Nomem as c_int,
            );
        }
        return ptr::null_mut();
    }

    unsafe {
        (*state).yim1 = libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double;
        if (*state).yim1.is_null() {
            libc::free(state as *mut c_void);
            gsl_error(
                b"failed to allocate space for yim1\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                64,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).k = libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double;
        if (*state).k.is_null() {
            libc::free((*state).yim1 as *mut c_void);
            libc::free(state as *mut c_void);
            gsl_error(
                b"failed to allocate space for k\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                73,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).y0 = libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double;
        if (*state).y0.is_null() {
            libc::free((*state).k as *mut c_void);
            libc::free((*state).yim1 as *mut c_void);
            libc::free(state as *mut c_void);
            gsl_error(
                b"failed to allocate space for y0\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                83,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).y0_orig = libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double;
        if (*state).y0_orig.is_null() {
            libc::free((*state).y0 as *mut c_void);
            libc::free((*state).k as *mut c_void);
            libc::free((*state).yim1 as *mut c_void);
            libc::free(state as *mut c_void);
            gsl_error(
                b"failed to allocate space for y0_orig\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                94,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).y_onestep = libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double;
        if (*state).y_onestep.is_null() {
            libc::free((*state).y0_orig as *mut c_void);
            libc::free((*state).y0 as *mut c_void);
            libc::free((*state).k as *mut c_void);
            libc::free((*state).yim1 as *mut c_void);
            libc::free(state as *mut c_void);
            gsl_error(
                b"failed to allocate space for y0_orig\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                106,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).primed = 0;
        (*state).primer = gsl_odeiv_step_alloc(gsl_odeiv_step_rk4imp, dim);
        if (*state).primer.is_null() {
            libc::free((*state).y_onestep as *mut c_void);
            libc::free((*state).y0_orig as *mut c_void);
            libc::free((*state).y0 as *mut c_void);
            libc::free((*state).k as *mut c_void);
            libc::free((*state).yim1 as *mut c_void);
            libc::free(state as *mut c_void);
            gsl_error(
                b"failed to allocate space for primer\0".as_ptr() as *const c_char,
                b"gear2.c\0".as_ptr() as *const c_char,
                120,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).last_h = 0.0;
    }

    state as *mut c_void
}

fn gear2_step(
    y: *mut c_double,
    state: *mut Gear2State,
    h: c_double,
    t: c_double,
    dim: size_t,
    sys: *const GslOdeivSystem,
) -> c_int {
    let iter_steps = 3;
    let mut nu = 0;
    
    unsafe {
        let y0 = (*state).y0;
        let yim1 = (*state).yim1;
        let k = (*state).k;

        while nu < iter_steps {
            let s = ((*sys).function.unwrap())(t + h, y, k, (*sys).params);
            if s != GslError::Success as c_int {
                return s;
            }

            for i in 0..dim {
                *y.offset(i as isize) = (4.0 * *y0.offset(i as isize) - *yim1.offset(i as isize) 
                    + 2.0 * h * *k.offset(i as isize)) / 3.0;
            }
            nu += 1;
        }
    }

    GslError::Success as c_int
}

fn gear2_apply(
    vstate: *mut c_void,
    dim: size_t,
    t: c_double,
    h: c_double,
    y: *mut c_double,
    yerr: *mut c_double,
    dydt_in: *const c_double,
    dydt_out: *mut c_double,
    sys: *const GslOdeivSystem,
) -> c_int {
    let state = vstate as *mut Gear2State;
    
    unsafe {
        (*state).stutter = 0;
        
        if (*state).primed == 0 || t == (*state).t_primed || h != (*state).last_h {
            libc::memcpy(
                (*state).yim1 as *mut c_void,
                y as *const c_void,
                dim * mem::size_of::<c_double>(),
            );
            
            let status = gsl_odeiv_step_apply(
                (*state).primer,
                t,
                h,
                y,
                yerr,
                dydt_in,
                dydt_out,
                sys,
            );
            
            (*state).primed = 1;
            (*state).t_primed = t;
            (*state).last_h = h;
            (*state).stutter = 1;
            
            return status;
        } else {
            let k = (*state).k;
            let y0 = (*state).y0;
            let y0_orig = (*state).y0_orig;
            let yim1 = (*state).yim1;
            let y_onestep = (*state).y_onestep;
            
            libc::memcpy(
                y0 as *mut c_void,
                y as *const c_void,
                dim * mem::size_of::<c_double>(),
            );
            
            libc::memcpy(
                y0_orig as *mut c_void,
                y as *const c_void,
                dim * mem::size_of::<c_double>(),
            );
            
            if !dydt_out.is_null() {
                libc::memcpy(
                    k as *mut c_void,
                    dydt_out as *const c_void,
                    dim * mem::size_of::<c_double>(),
                );
            }
            
            libc::memcpy(
                y_onestep as *mut c_void,
                y as *const c_void,
                dim * mem::size_of::<c_double>(),
            );
            
            let mut s = gear2_step(y_onestep, state, h, t, dim, sys);
            if s != GslError::Success as c_int {
                return s;
            }
            
            s = gear2_step(y, state, h / 2.0, t, dim, sys);
            if s != GslError::Success as c_int {
                libc::memcpy(
                    y as *mut c_void,
                    y0_orig as *const c_void,
                    dim * mem::size_of::<c_double>(),
                );
                return s;
            }
            
            libc::memcpy(
                y0 as *mut c_void,
                y as *const c_void,
                dim * mem::size_of::<c_double>(),
            );
            
            s = gear2_step(y, state, h / 2.0, t + h / 2.0, dim, sys);
            if s != GslError::Success as c_int {
                libc::memcpy(
                    y as *mut c_void,
                    y0_orig as *const c_void,
                    dim * mem::size_of::<c_double>(),
                );
                return s;
            }
            
            if !dydt_out.is_null() {
                s = ((*sys).function.unwrap())(t + h, y, dydt_out, (*sys).params);
                if s != GslError::Success as c_int {
                    libc::memcpy(
                        y as *mut c_void,
                        y0_orig as *const c_void,
                        dim * mem::size_of::<c_double>(),
                    );
                    return s;
                }
            }
            
            for i in 0..dim {
                *yerr.offset(i as isize) = 4.0 * (*y.offset(i as isize) - *y_onestep.offset(i as isize));
                *yim1.offset(i as isize) = *y0.offset(i as isize);
            }
            
            (*state).last_h = h;
            GslError::Success as c_int
        }
    }
}

fn gear2_reset(vstate: *mut c_void, dim: size_t) -> c_int {
    let state = vstate as *mut Gear2State;
    
    unsafe {
        libc::memset(
            (*state).yim1 as *mut c_void,
            0,
            dim * mem::size_of::<c_double>(),
        );
        
        libc::memset(
            (*state).k as *mut c_void,
            0,
            dim * mem::size_of::<c_double>(),
        );
        
        libc::memset(
            (*state).y0 as *mut c_void,
            0,
            dim * mem::size_of::<c_double>(),
        );
        
        (*state).primed = 0;
        (*state).last_h = 0.0;
    }
    
    GslError::Success as c_int
}

fn gear2_order(_vstate: *mut c_void) -> c_uint {
    3
}

fn gear2_free(vstate: *mut c_void) {
    let state = vstate as *mut Gear2State;
    
    unsafe {
        libc::free((*state).yim1 as *mut c_void);
        libc::free((*state).k as *mut c_void);
        libc::free((*state).y0 as *mut c_void);
        libc::free((*state).y0_orig as *mut c_void);
        libc::free((*state).y_onestep as *mut c_void);
        gsl_odeiv_step_free((*state).primer);
        libc::free(state as *mut c_void);
    }
}

static GEAR2_TYPE: GslOdeivStepType = GslOdeivStepType {
    name: b"gear2\0".as_ptr() as *const c_char,
    can_use_dydt_in: 1,
    gives_exact_dydt_out: 0,
    alloc: Some(gear2_alloc),
    apply: Some(gear2_apply),
    reset: Some(gear2_reset),
    order: Some(gear2_order),
    free: Some(gear2_free),
};

#[no_mangle]
pub static gsl_odeiv_step_gear2: *const GslOdeivStepType = &GEAR2_TYPE;