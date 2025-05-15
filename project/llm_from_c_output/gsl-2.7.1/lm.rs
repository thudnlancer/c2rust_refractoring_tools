use std::ptr;
use std::mem;
use std::ffi::CString;
use libc::{c_int, c_double, c_void, size_t};
use gsl_sys::{
    gsl_blas_dgemv, gsl_blas_dnrm2, gsl_error, gsl_linalg_QRPT_decomp, gsl_linalg_QRPT_solve,
    gsl_matrix, gsl_multifit_nlinear_parameters, gsl_multifit_nlinear_trust_state,
    gsl_multifit_nlinear_trs, gsl_vector, GSL_ENOMEM, GSL_SUCCESS,
};

#[derive(Debug)]
struct LmState {
    n: usize,
    p: usize,
    fvv: *mut gsl_vector,
    vel: *mut gsl_vector,
    acc: *mut gsl_vector,
    workp: *mut gsl_vector,
    workn: *mut gsl_vector,
    accel: bool,
    params: gsl_multifit_nlinear_parameters,
}

unsafe extern "C" fn lm_alloc(
    accel: c_int,
    params: *const c_void,
    n: size_t,
    p: size_t,
) -> *mut c_void {
    let mparams = params as *const gsl_multifit_nlinear_parameters;
    let state = Box::into_raw(Box::new(LmState {
        n: 0,
        p: 0,
        fvv: ptr::null_mut(),
        vel: ptr::null_mut(),
        acc: ptr::null_mut(),
        workp: ptr::null_mut(),
        workn: ptr::null_mut(),
        accel: false,
        params: mem::zeroed(),
    }));

    (*state).workp = gsl_vector_alloc(p);
    if (*state).workp.is_null() {
        gsl_error(b"failed to allocate space for workp\0".as_ptr() as *const _, b"\0".as_ptr() as *const _, 0, GSL_ENOMEM);
        lm_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).workn = gsl_vector_alloc(n);
    if (*state).workn.is_null() {
        gsl_error(b"failed to allocate space for workn\0".as_ptr() as *const _, b"\0".as_ptr() as *const _, 0, GSL_ENOMEM);
        lm_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).fvv = gsl_vector_alloc(n);
    if (*state).fvv.is_null() {
        gsl_error(b"failed to allocate space for fvv\0".as_ptr() as *const _, b"\0".as_ptr() as *const _, 0, GSL_ENOMEM);
        lm_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).vel = gsl_vector_alloc(p);
    if (*state).vel.is_null() {
        gsl_error(b"failed to allocate space for vel\0".as_ptr() as *const _, b"\0".as_ptr() as *const _, 0, GSL_ENOMEM);
        lm_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).acc = gsl_vector_alloc(p);
    if (*state).acc.is_null() {
        gsl_error(b"failed to allocate space for acc\0".as_ptr() as *const _, b"\0".as_ptr() as *const _, 0, GSL_ENOMEM);
        lm_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).n = n;
    (*state).p = p;
    (*state).params = *mparams;
    (*state).accel = accel != 0;

    state as *mut c_void
}

unsafe extern "C" fn lm_alloc_noaccel(params: *const c_void, n: size_t, p: size_t) -> *mut c_void {
    lm_alloc(0, params, n, p)
}

unsafe extern "C" fn lm_alloc_accel(params: *const c_void, n: size_t, p: size_t) -> *mut c_void {
    lm_alloc(1, params, n, p)
}

unsafe extern "C" fn lm_free(vstate: *mut c_void) {
    let state = vstate as *mut LmState;

    if !(*state).workp.is_null() {
        gsl_vector_free((*state).workp);
    }

    if !(*state).workn.is_null() {
        gsl_vector_free((*state).workn);
    }

    if !(*state).fvv.is_null() {
        gsl_vector_free((*state).fvv);
    }

    if !(*state).vel.is_null() {
        gsl_vector_free((*state).vel);
    }

    if !(*state).acc.is_null() {
        gsl_vector_free((*state).acc);
    }

    Box::from_raw(state);
}

unsafe extern "C" fn lm_init(vtrust_state: *const c_void, vstate: *mut c_void) -> c_int {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let state = vstate as *mut LmState;

    gsl_vector_set_zero((*state).vel);
    gsl_vector_set_zero((*state).acc);

    *(*trust_state).avratio = 0.0;

    GSL_SUCCESS
}

unsafe extern "C" fn lm_preloop(vtrust_state: *const c_void, vstate: *mut c_void) -> c_int {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let params = (*trust_state).params;
    let status = ((*params).solver.init)(trust_state as *const c_void, (*trust_state).solver_state);

    if status != GSL_SUCCESS {
        return status;
    }

    GSL_SUCCESS
}

unsafe extern "C" fn lm_step(
    vtrust_state: *const c_void,
    delta: c_double,
    dx: *mut gsl_vector,
    vstate: *mut c_void,
) -> c_int {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let state = vstate as *mut LmState;
    let params = (*trust_state).params;
    let mu = *(*trust_state).mu;
    let mut status = ((*params).solver.presolve)(mu, trust_state as *const c_void, (*trust_state).solver_state);

    if status != GSL_SUCCESS {
        return status;
    }

    status = ((*params).solver.solve)(
        (*trust_state).f,
        (*state).vel,
        trust_state as *const c_void,
        (*trust_state).solver_state,
    );

    if status != GSL_SUCCESS {
        return status;
    }

    if (*state).accel {
        let mut anorm = gsl_blas_dnrm2((*state).acc);
        let vnorm = gsl_blas_dnrm2((*state).vel);

        *(*trust_state).avratio = anorm / vnorm;
    }

    scaled_addition(1.0, (*state).vel, 0.5, (*state).acc, dx);

    GSL_SUCCESS
}

unsafe extern "C" fn lm_preduction(
    vtrust_state: *const c_void,
    dx: *const gsl_vector,
    pred: *mut c_double,
    vstate: *mut c_void,
) -> c_int {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let state = vstate as *mut LmState;
    let diag = (*trust_state).diag;
    let p = (*state).vel;
    let norm_Dp = scaled_enorm(diag, p);
    let normf = gsl_blas_dnrm2((*trust_state).f);
    let mu = *(*trust_state).mu;
    let mut norm_Jp = 0.0;
    let mut u = 0.0;
    let mut v = 0.0;

    gsl_blas_dgemv(
        CblasNoTrans,
        1.0,
        (*trust_state).J,
        p,
        0.0,
        (*state).workn,
    );

    norm_Jp = gsl_blas_dnrm2((*state).workn);

    u = norm_Jp / normf;
    v = norm_Dp / normf;

    *pred = u * u + 2.0 * mu * v * v;

    GSL_SUCCESS
}

static LM_TYPE: gsl_multifit_nlinear_trs = gsl_multifit_nlinear_trs {
    name: b"levenberg-marquardt\0".as_ptr() as *const _,
    alloc: Some(lm_alloc_noaccel),
    init: Some(lm_init),
    preloop: Some(lm_preloop),
    step: Some(lm_step),
    preduction: Some(lm_preduction),
    free: Some(lm_free),
};

#[no_mangle]
pub static gsl_multifit_nlinear_trs_lm: *const gsl_multifit_nlinear_trs = &LM_TYPE;

static LMACCEL_TYPE: gsl_multifit_nlinear_trs = gsl_multifit_nlinear_trs {
    name: b"levenberg-marquardt+accel\0".as_ptr() as *const _,
    alloc: Some(lm_alloc_accel),
    init: Some(lm_init),
    preloop: Some(lm_preloop),
    step: Some(lm_step),
    preduction: Some(lm_preduction),
    free: Some(lm_free),
};

#[no_mangle]
pub static gsl_multifit_nlinear_trs_lmaccel: *const gsl_multifit_nlinear_trs = &LMACCEL_TYPE;