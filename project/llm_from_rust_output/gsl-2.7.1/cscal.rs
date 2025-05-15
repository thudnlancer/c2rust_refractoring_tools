use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_control_type {
    pub name: *const c_char,
    pub alloc: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*mut c_void, c_double, c_double, c_double, c_double) -> c_int>,
    pub hadjust: Option<unsafe extern "C" fn(*mut c_void, usize, c_uint, *const c_double, *const c_double, *const c_double, *mut c_double) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_control {
    pub type_: *const gsl_odeiv_control_type,
    pub state: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
struct ScControlState {
    eps_abs: c_double,
    eps_rel: c_double,
    a_y: c_double,
    a_dydt: c_double,
    scale_abs: Vec<c_double>,
}

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
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

fn gsl_error(reason: &str, file: &str, line: c_int, gsl_errno: GslError) {
    let reason = CString::new(reason).unwrap();
    let file = CString::new(file).unwrap();
    unsafe {
        // Assuming gsl_error is defined elsewhere
        gsl_error(
            reason.as_ptr(),
            file.as_ptr(),
            line,
            gsl_errno as c_int,
        );
    }
}

fn sc_control_alloc() -> *mut c_void {
    Box::into_raw(Box::new(ScControlState {
        eps_abs: 0.0,
        eps_rel: 0.0,
        a_y: 0.0,
        a_dydt: 0.0,
        scale_abs: Vec::new(),
    })) as *mut c_void
}

fn sc_control_init(
    vstate: *mut c_void,
    eps_abs: c_double,
    eps_rel: c_double,
    a_y: c_double,
    a_dydt: c_double,
) -> c_int {
    if eps_abs < 0.0 {
        gsl_error("eps_abs is negative", "cscal.c", 61, GslError::Inval);
        return GslError::Inval as c_int;
    }
    if eps_rel < 0.0 {
        gsl_error("eps_rel is negative", "cscal.c", 65, GslError::Inval);
        return GslError::Inval as c_int;
    }
    if a_y < 0.0 {
        gsl_error("a_y is negative", "cscal.c", 69, GslError::Inval);
        return GslError::Inval as c_int;
    }
    if a_dydt < 0.0 {
        gsl_error("a_dydt is negative", "cscal.c", 73, GslError::Inval);
        return GslError::Inval as c_int;
    }

    let state = unsafe { &mut *(vstate as *mut ScControlState) };
    state.eps_abs = eps_abs;
    state.eps_rel = eps_rel;
    state.a_y = a_y;
    state.a_dydt = a_dydt;

    GslError::Success as c_int
}

fn sc_control_hadjust(
    vstate: *mut c_void,
    dim: usize,
    ord: c_uint,
    y: *const c_double,
    yerr: *const c_double,
    yp: *const c_double,
    h: *mut c_double,
) -> c_int {
    let state = unsafe { &mut *(vstate as *mut ScControlState) };
    let eps_abs = state.eps_abs;
    let eps_rel = state.eps_rel;
    let a_y = state.a_y;
    let a_dydt = state.a_dydt;
    let scale_abs = &state.scale_abs;

    let s = 0.9;
    let h_old = unsafe { *h };
    let mut rmax = std::f64::MIN_POSITIVE;

    for i in 0..dim {
        let y_i = unsafe { *y.add(i) };
        let yp_i = unsafe { *yp.add(i) };
        let yerr_i = unsafe { *yerr.add(i) };
        let scale_abs_i = scale_abs[i];

        let d0 = eps_rel * (a_y * y_i.abs() + a_dydt * (h_old * yp_i).abs()) + eps_abs * scale_abs_i;
        let r = yerr_i.abs() / d0.abs();
        rmax = r.max(rmax);
    }

    if rmax > 1.1 {
        let mut r = s / rmax.powf(1.0 / ord as f64);
        if r < 0.2 {
            r = 0.2;
        }
        unsafe { *h = r * h_old };
        -1
    } else if rmax < 0.5 {
        let mut r = s / rmax.powf(1.0 / (ord as f64 + 1.0));
        if r > 5.0 {
            r = 5.0;
        }
        if r < 1.0 {
            r = 1.0;
        }
        unsafe { *h = r * h_old };
        1
    } else {
        0
    }
}

fn sc_control_free(vstate: *mut c_void) {
    unsafe {
        Box::from_raw(vstate as *mut ScControlState);
    }
}

static SC_CONTROL_TYPE: gsl_odeiv_control_type = gsl_odeiv_control_type {
    name: b"scaled\0".as_ptr() as *const c_char,
    alloc: Some(sc_control_alloc),
    init: Some(sc_control_init),
    hadjust: Some(sc_control_hadjust),
    free: Some(sc_control_free),
};

pub static GSL_ODEIV_CONTROL_SCALED: *const gsl_odeiv_control_type = &SC_CONTROL_TYPE;

pub fn gsl_odeiv_control_scaled_new(
    eps_abs: c_double,
    eps_rel: c_double,
    a_y: c_double,
    a_dydt: c_double,
    scale_abs: &[c_double],
    dim: usize,
) -> *mut gsl_odeiv_control {
    unsafe {
        let c = Box::into_raw(Box::new(gsl_odeiv_control {
            type_: GSL_ODEIV_CONTROL_SCALED,
            state: ptr::null_mut(),
        }));

        let status = sc_control_init((*c).state, eps_abs, eps_rel, a_y, a_dydt);
        if status != GslError::Success as c_int {
            Box::from_raw(c);
            gsl_error(
                "error trying to initialize control",
                "cscal.c",
                174,
                GslError::from(status),
            );
            return ptr::null_mut();
        }

        let s = &mut *((*c).state as *mut ScControlState);
        s.scale_abs = scale_abs.to_vec();

        c
    }
}