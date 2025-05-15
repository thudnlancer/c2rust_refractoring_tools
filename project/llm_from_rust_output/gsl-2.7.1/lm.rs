use libc::{c_double, c_int, c_ulong, c_void};
use std::ptr;

pub type size_t = c_ulong;

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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CblasTranspose {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GslMultifitNlinearFdtype {
    Fwdiff = 0,
    Ctrdiff = 1,
}

pub type GslMultifitNlinearF = Option<
    unsafe extern "C" fn(
        *const GslVector,
        *mut c_void,
        *mut GslVector,
    ) -> c_int,
>;

pub type GslMultifitNlinearDf = Option<
    unsafe extern "C" fn(
        *const GslVector,
        *mut c_void,
        *mut GslMatrix,
    ) -> c_int,
>;

pub type GslMultifitNlinearFvv = Option<
    unsafe extern "C" fn(
        *const GslVector,
        *const GslVector,
        *mut c_void,
        *mut GslVector,
    ) -> c_int,
>;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMultifitNlinearFdf {
    pub f: GslMultifitNlinearF,
    pub df: GslMultifitNlinearDf,
    pub fvv: GslMultifitNlinearFvv,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
}

pub type GslMultifitNlinearAlloc = Option<
    unsafe extern "C" fn(*const c_void, size_t, size_t) -> *mut c_void,
>;

pub type GslMultifitNlinearInit = Option<
    unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int,
>;

pub type GslMultifitNlinearPreloop = Option<
    unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int,
>;

pub type GslMultifitNlinearStep = Option<
    unsafe extern "C" fn(
        *const c_void,
        c_double,
        *mut GslVector,
        *mut c_void,
    ) -> c_int,
>;

pub type GslMultifitNlinearPred = Option<
    unsafe extern "C" fn(
        *const c_void,
        *const GslVector,
        *mut c_double,
        *mut c_void,
    ) -> c_int,
>;

pub type GslMultifitNlinearFree = Option<unsafe extern "C" fn(*mut c_void)>;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMultifitNlinearTrs {
    pub name: *const libc::c_char,
    pub alloc: GslMultifitNlinearAlloc,
    pub init: GslMultifitNlinearInit,
    pub preloop: GslMultifitNlinearPreloop,
    pub step: GslMultifitNlinearStep,
    pub preduction: GslMultifitNlinearPred,
    pub free: GslMultifitNlinearFree,
}

pub type GslMultifitNlinearScaleInit = Option<
    unsafe extern "C" fn(*const GslMatrix, *mut GslVector) -> c_int,
>;

pub type GslMultifitNlinearScaleUpdate = Option<
    unsafe extern "C" fn(*const GslMatrix, *mut GslVector) -> c_int,
>;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMultifitNlinearScale {
    pub name: *const libc::c_char,
    pub init: GslMultifitNlinearScaleInit,
    pub update: GslMultifitNlinearScaleUpdate,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMultifitNlinearParameters {
    pub trs: *const GslMultifitNlinearTrs,
    pub scale: *const GslMultifitNlinearScale,
    pub solver: *const GslMultifitNlinearTrs,
    pub fdtype: GslMultifitNlinearFdtype,
    pub factor_up: c_double,
    pub factor_down: c_double,
    pub avmax: c_double,
    pub h_df: c_double,
    pub h_fvv: c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMultifitNlinearTrustState {
    pub x: *const GslVector,
    pub f: *const GslVector,
    pub g: *const GslVector,
    pub J: *const GslMatrix,
    pub diag: *const GslVector,
    pub sqrt_wts: *const GslVector,
    pub mu: *const c_double,
    pub params: *const GslMultifitNlinearParameters,
    pub solver_state: *mut c_void,
    pub fdf: *mut GslMultifitNlinearFdf,
    pub avratio: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LmState {
    pub n: size_t,
    pub p: size_t,
    pub fvv: *mut GslVector,
    pub vel: *mut GslVector,
    pub acc: *mut GslVector,
    pub workp: *mut GslVector,
    pub workn: *mut GslVector,
    pub accel: c_int,
    pub params: GslMultifitNlinearParameters,
}

extern "C" {
    pub fn sqrt(x: c_double) -> c_double;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: c_int,
        gsl_errno: c_int,
    );
    pub fn gsl_vector_alloc(n: size_t) -> *mut GslVector;
    pub fn gsl_vector_free(v: *mut GslVector);
    pub fn gsl_vector_set_zero(v: *mut GslVector);
    pub fn gsl_blas_dnrm2(X: *const GslVector) -> c_double;
    pub fn gsl_blas_dgemv(
        TransA: CblasTranspose,
        alpha: c_double,
        A: *const GslMatrix,
        X: *const GslVector,
        beta: c_double,
        Y: *mut GslVector,
    ) -> c_int;
    pub fn gsl_multifit_nlinear_eval_fvv(
        h: c_double,
        x: *const GslVector,
        v: *const GslVector,
        f: *const GslVector,
        J: *const GslMatrix,
        swts: *const GslVector,
        fdf: *mut GslMultifitNlinearFdf,
        yvv: *mut GslVector,
        work: *mut GslVector,
    ) -> c_int;
}

fn gsl_vector_get(v: &GslVector, i: size_t) -> c_double {
    unsafe { *v.data.offset((i * v.stride) as isize) }
}

fn gsl_vector_set(v: &mut GslVector, i: size_t, x: c_double) {
    unsafe {
        *v.data.offset((i * v.stride) as isize) = x;
    }
}

fn scaled_enorm(d: &GslVector, f: &GslVector) -> c_double {
    let mut e2 = 0.0;
    for i in 0..f.size {
        let fi = gsl_vector_get(f, i);
        let di = gsl_vector_get(d, i);
        let u = di * fi;
        e2 += u * u;
    }
    unsafe { sqrt(e2) }
}

fn scaled_addition(
    alpha: c_double,
    x: &GslVector,
    beta: c_double,
    y: &GslVector,
    z: &mut GslVector,
) {
    for i in 0..z.size {
        let xi = gsl_vector_get(x, i);
        let yi = gsl_vector_get(y, i);
        gsl_vector_set(z, i, alpha * xi + beta * yi);
    }
}

fn lm_alloc(
    accel: c_int,
    params: *const c_void,
    n: size_t,
    p: size_t,
) -> *mut c_void {
    let mparams = unsafe { &*(params as *const GslMultifitNlinearParameters) };
    let state = unsafe {
        calloc(1, std::mem::size_of::<LmState>() as size_t) as *mut LmState
    };

    if state.is_null() {
        unsafe {
            gsl_error(
                b"failed to allocate lm state\0".as_ptr() as *const libc::c_char,
                b"lm.c\0".as_ptr() as *const libc::c_char,
                81,
                GslError::Nomem as c_int,
            );
        }
        return ptr::null_mut();
    }

    unsafe {
        (*state).workp = gsl_vector_alloc(p);
        if (*state).workp.is_null() {
            gsl_error(
                b"failed to allocate space for workp\0".as_ptr() as *const libc::c_char,
                b"lm.c\0".as_ptr() as *const libc::c_char,
                87,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).workn = gsl_vector_alloc(n);
        if (*state).workn.is_null() {
            gsl_error(
                b"failed to allocate space for workn\0".as_ptr() as *const libc::c_char,
                b"lm.c\0".as_ptr() as *const libc::c_char,
                93,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).fvv = gsl_vector_alloc(n);
        if (*state).fvv.is_null() {
            gsl_error(
                b"failed to allocate space for fvv\0".as_ptr() as *const libc::c_char,
                b"lm.c\0".as_ptr() as *const libc::c_char,
                99,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).vel = gsl_vector_alloc(p);
        if (*state).vel.is_null() {
            gsl_error(
                b"failed to allocate space for vel\0".as_ptr() as *const libc::c_char,
                b"lm.c\0".as_ptr() as *const libc::c_char,
                105,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).acc = gsl_vector_alloc(p);
        if (*state).acc.is_null() {
            gsl_error(
                b"failed to allocate space for acc\0".as_ptr() as *const libc::c_char,
                b"lm.c\0".as_ptr() as *const libc::c_char,
                111,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }

        (*state).n = n;
        (*state).p = p;
        (*state).params = *mparams;
        (*state).accel = accel;
    }

    state as *mut c_void
}

fn lm_alloc_noaccel(params: *const c_void, n: size_t, p: size_t) -> *mut c_void {
    lm_alloc(0, params, n, p)
}

fn lm_alloc_accel(params: *const c_void, n: size_t, p: size_t) -> *mut c_void {
    lm_alloc(1, params, n, p)
}

fn lm_free(vstate: *mut c_void) {
    let state = unsafe { &mut *(vstate as *mut LmState) };

    if !state.workp.is_null() {
        unsafe { gsl_vector_free(state.workp) };
    }
    if !state.workn.is_null() {
        unsafe { gsl_vector_free(state.workn) };
    }
    if !state.fvv.is_null() {
        unsafe { gsl_vector_free(state.fvv) };
    }
    if !state.vel.is_null() {
        unsafe { gsl_vector_free(state.vel) };
    }
    if !state.acc.is_null() {
        unsafe { gsl_vector_free(state.acc) };
    }

    unsafe { free(vstate) };
}

fn lm_init(vtrust_state: *const c_void, vstate: *mut c_void) -> c_int {
    let state = unsafe { &mut *(vstate as *mut LmState) };
    unsafe {
        gsl_vector_set_zero(state.vel);
        gsl_vector_set_zero(state.acc);
    }
    let trust_state = unsafe { &*(vtrust_state as *const GslMultifitNlinearTrustState) };
    unsafe {
        *trust_state.avratio = 0.0;
    }
    GslError::Success as c_int
}

fn lm_preloop(vtrust_state: *const c_void, vstate: *mut c_void) -> c_int {
    let trust_state = unsafe { &*(vtrust_state as *const GslMultifitNlinearTrustState) };
    let params = trust_state.params;
    let solver = unsafe { &*params.solver };

    let status = unsafe {
        (solver.init.unwrap())(
            vtrust_state,
            (*trust_state).solver_state,
        )
    };

    if status != GslError::Success as c_int {
        return status;
    }

    GslError::Success as c_int
}

fn lm_step(
    vtrust_state: *const c_void,
    delta: c_double,
    dx: *mut GslVector,
    vstate: *mut c_void,
) -> c_int {
    let trust_state = unsafe { &*(vtrust_state as *const GslMultifitNlinearTrustState) };
    let state = unsafe { &mut *(vstate as *mut LmState) };
    let params = trust_state.params;
    let solver = unsafe { &*params.solver };
    let mu = unsafe { *trust_state.mu };

    let status = unsafe {
        (solver.presolve.unwrap())(mu, vtrust_state, (*trust_state).solver_state)
    };
    if status != GslError::Success as c_int {
        return status;
    }

    let status = unsafe {
        (solver.solve.unwrap())(
            (*trust_state).f,
            (*state).vel,
            vtrust_state,
            (*trust_state).solver_state,
        )
    };
    if status != GslError::Success as c_int {
        return status;
    }

    if state.accel != 0 {
        let status = unsafe {
            gsl_multifit_nlinear_eval_fvv(
                (*params).h_fvv,
                (*trust_state).x,
                (*state).vel,
                (*trust_state).f,
                (*trust_state).J,
                (*trust_state).sqrt_wts,
                (*trust_state).fdf,
                (*state).fvv,
                (*state).workp,
            )
        };
        if status != GslError::Success as c_int {
            return status;
        }

        let status = unsafe {
            (solver.solve.unwrap())(
                (*state).fvv,
                (*state).acc,
                vtrust_state,
                (*trust_state).solver_state,
            )
        };
        if status != GslError::Success as c_int {
            return status;
        }

        let anorm = unsafe { gsl_bl