use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ffi::c_void;
use libc::{c_int, c_double, size_t};
use gsl_sys::{
    gsl_blas_dgemv, gsl_blas_dnrm2, gsl_error, gsl_linalg_QRPT_rcond, gsl_matrix, gsl_permutation,
    gsl_vector, GSL_ENOMEM, GSL_SUCCESS, GSL_FAILURE, GSL_ENOPROG, GSL_CONTINUE,
    GSL_MULTIFIT_NLINEAR_FDF, GSL_MULTIFIT_NLINEAR_PARAMETERS, GSL_MULTIFIT_NLINEAR_TRS,
    GSL_MULTIFIT_NLINEAR_TRUST_STATE, GSL_MULTIFIT_NLINEAR_SCALE, GSL_MULTIFIT_NLINEAR_SOLVER,
};

#[derive(Debug)]
struct TrustState {
    n: usize,
    p: usize,
    delta: f64,
    mu: f64,
    nu: i64,
    diag: *mut gsl_vector,
    x_trial: *mut gsl_vector,
    f_trial: *mut gsl_vector,
    workp: *mut gsl_vector,
    workn: *mut gsl_vector,
    trs_state: *mut c_void,
    solver_state: *mut c_void,
    avratio: f64,
    params: GSL_MULTIFIT_NLINEAR_PARAMETERS,
}

unsafe extern "C" fn trust_alloc(
    params: *const GSL_MULTIFIT_NLINEAR_PARAMETERS,
    n: size_t,
    p: size_t,
) -> *mut c_void {
    let state = Box::into_raw(Box::new(TrustState {
        n: n as usize,
        p: p as usize,
        delta: 0.0,
        mu: 0.0,
        nu: 0,
        diag: ptr::null_mut(),
        x_trial: ptr::null_mut(),
        f_trial: ptr::null_mut(),
        workp: ptr::null_mut(),
        workn: ptr::null_mut(),
        trs_state: ptr::null_mut(),
        solver_state: ptr::null_mut(),
        avratio: 0.0,
        params: *params,
    }));

    (*state).diag = gsl_vector_alloc(p);
    if (*state).diag.is_null() {
        gsl_error("failed to allocate space for diag", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).workp = gsl_vector_alloc(p);
    if (*state).workp.is_null() {
        gsl_error("failed to allocate space for workp", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).workn = gsl_vector_alloc(n);
    if (*state).workn.is_null() {
        gsl_error("failed to allocate space for workn", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).x_trial = gsl_vector_alloc(p);
    if (*state).x_trial.is_null() {
        gsl_error("failed to allocate space for x_trial", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    (*state).f_trial = gsl_vector_alloc(n);
    if (*state).f_trial.is_null() {
        gsl_error("failed to allocate space for f_trial", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    let trs_alloc = (*(*params).trs).alloc;
    (*state).trs_state = trs_alloc(params, n, p);
    if (*state).trs_state.is_null() {
        gsl_error("failed to allocate space for trs state", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    let solver_alloc = (*(*params).solver).alloc;
    (*state).solver_state = solver_alloc(n, p);
    if (*state).solver_state.is_null() {
        gsl_error("failed to allocate space for solver state", GSL_ENOMEM);
        trust_free(state as *mut c_void);
        return ptr::null_mut();
    }

    state as *mut c_void
}

unsafe extern "C" fn trust_free(vstate: *mut c_void) {
    let state = vstate as *mut TrustState;
    if state.is_null() {
        return;
    }

    let params = &(*state).params;

    if !(*state).diag.is_null() {
        gsl_vector_free((*state).diag);
    }

    if !(*state).workp.is_null() {
        gsl_vector_free((*state).workp);
    }

    if !(*state).workn.is_null() {
        gsl_vector_free((*state).workn);
    }

    if !(*state).x_trial.is_null() {
        gsl_vector_free((*state).x_trial);
    }

    if !(*state).f_trial.is_null() {
        gsl_vector_free((*state).f_trial);
    }

    if !(*state).trs_state.is_null() {
        let trs_free = (*params.trs).free;
        trs_free((*state).trs_state);
    }

    if !(*state).solver_state.is_null() {
        let solver_free = (*params.solver).free;
        solver_free((*state).solver_state);
    }

    Box::from_raw(state);
}

unsafe extern "C" fn trust_init(
    vstate: *mut c_void,
    swts: *const gsl_vector,
    fdf: *mut GSL_MULTIFIT_NLINEAR_FDF,
    x: *const gsl_vector,
    f: *mut gsl_vector,
    J: *mut gsl_matrix,
    g: *mut gsl_vector,
) -> c_int {
    let state = vstate as *mut TrustState;
    let params = &(*state).params;
    let mut status: c_int;

    status = gsl_multifit_nlinear_eval_f(fdf, x, swts, f);
    if status != GSL_SUCCESS {
        return status;
    }

    status = gsl_multifit_nlinear_eval_df(
        x,
        f,
        swts,
        params.h_df,
        params.fdtype,
        fdf,
        J,
        (*state).workn,
    );
    if status != GSL_SUCCESS {
        return status;
    }

    gsl_blas_dgemv(CblasTrans, 1.0, J, f, 0.0, g);

    let scale_init = (*params.scale).init;
    scale_init(J, (*state).diag);

    let Dx = trust_scaled_norm((*state).diag, x);
    (*state).delta = 0.3 * if 1.0 > Dx { 1.0 } else { Dx };

    status = nielsen_init(J, (*state).diag, &mut (*state).mu, &mut (*state).nu);
    if status != GSL_SUCCESS {
        return status;
    }

    let mut trust_state = GSL_MULTIFIT_NLINEAR_TRUST_STATE {
        x,
        f,
        g,
        J,
        diag: (*state).diag,
        sqrt_wts: swts,
        mu: &mut (*state).mu,
        params,
        solver_state: (*state).solver_state,
        fdf,
        avratio: &mut (*state).avratio,
    };

    let trs_init = (*params.trs).init;
    status = trs_init(&mut trust_state, (*state).trs_state);
    if status != GSL_SUCCESS {
        return status;
    }

    (*state).avratio = 0.0;

    GSL_SUCCESS
}

unsafe extern "C" fn trust_iterate(
    vstate: *mut c_void,
    swts: *const gsl_vector,
    fdf: *mut GSL_MULTIFIT_NLINEAR_FDF,
    x: *mut gsl_vector,
    f: *mut gsl_vector,
    J: *mut gsl_matrix,
    g: *mut gsl_vector,
    dx: *mut gsl_vector,
) -> c_int {
    let state = vstate as *mut TrustState;
    let params = &(*state).params;
    let trs = params.trs;
    let mut trust_state = GSL_MULTIFIT_NLINEAR_TRUST_STATE {
        x,
        f,
        g,
        J,
        diag: (*state).diag,
        sqrt_wts: swts,
        mu: &mut (*state).mu,
        params,
        solver_state: (*state).solver_state,
        fdf,
        avratio: &mut (*state).avratio,
    };
    let x_trial = (*state).x_trial;
    let f_trial = (*state).f_trial;
    let diag = (*state).diag;
    let mut rho: f64;
    let mut foundstep = 0;
    let mut bad_steps = 0;
    let mut status: c_int;

    let trs_preloop = (*trs).preloop;
    status = trs_preloop(&mut trust_state, (*state).trs_state);
    if status != GSL_SUCCESS {
        return status;
    }

    while foundstep == 0 {
        let trs_step = (*trs).step;
        status = trs_step(&mut trust_state, (*state).delta, dx, (*state).trs_state);

        if status == GSL_SUCCESS {
            trust_trial_step(x, dx, x_trial);

            status = gsl_multifit_nlinear_eval_f(fdf, x_trial, swts, f_trial);
            if status != GSL_SUCCESS {
                return status;
            }

            status = trust_eval_step(f, f_trial, g, J, dx, &mut rho, state);
            if status == GSL_SUCCESS {
                foundstep = 1;
            }
        } else {
            rho = -1.0;
        }

        if rho > 0.75 {
            (*state).delta *= params.factor_up;
        } else if rho < 0.25 {
            (*state).delta /= params.factor_down;
        }

        if foundstep != 0 {
            status = gsl_multifit_nlinear_eval_df(
                x_trial,
                f_trial,
                swts,
                params.h_df,
                params.fdtype,
                fdf,
                J,
                (*state).workn,
            );
            if status != GSL_SUCCESS {
                return status;
            }

            gsl_vector_memcpy(x, x_trial);
            gsl_vector_memcpy(f, f_trial);
            gsl_blas_dgemv(CblasTrans, 1.0, J, f, 0.0, g);

            let scale_update = (*params.scale).update;
            scale_update(J, diag);

            status = nielsen_accept(rho, &mut (*state).mu, &mut (*state).nu);
            if status != GSL_SUCCESS {
                return status;
            }

            bad_steps = 0;
        } else {
            status = nielsen_reject(&mut (*state).mu, &mut (*state).nu);
            if status != GSL_SUCCESS {
                return status;
            }

            bad_steps += 1;
            if bad_steps > 15 {
                return GSL_ENOPROG;
            }
        }
    }

    GSL_SUCCESS
}

unsafe extern "C" fn trust_rcond(rcond: *mut f64, vstate: *mut c_void) -> c_int {
    let state = vstate as *mut TrustState;
    let params = &(*state).params;
    let solver_rcond = (*params.solver).rcond;
    solver_rcond(rcond, (*state).solver_state)
}

unsafe extern "C" fn trust_avratio(vstate: *mut c_void) -> f64 {
    let state = vstate as *mut TrustState;
    (*state).avratio
}

unsafe extern "C" fn trust_trial_step(
    x: *const gsl_vector,
    dx: *const gsl_vector,
    x_trial: *mut gsl_vector,
) {
    let n = (*x).size;
    for i in 0..n {
        let dxi = gsl_vector_get(dx, i);
        let xi = gsl_vector_get(x, i);
        gsl_vector_set(x_trial, i, xi + dxi);
    }
}

unsafe extern "C" fn trust_calc_rho(
    f: *const gsl_vector,
    f_trial: *const gsl_vector,
    g: *const gsl_vector,
    J: *const gsl_matrix,
    dx: *const gsl_vector,
    state: *mut TrustState,
) -> f64 {
    let params = &(*state).params;
    let trs = params.trs;
    let normf = gsl_blas_dnrm2(f);
    let normf_trial = gsl_blas_dnrm2(f_trial);
    let mut trust_state = GSL_MULTIFIT_NLINEAR_TRUST_STATE {
        x: ptr::null_mut(),
        f,
        g,
        J,
        diag: (*state).diag,
        sqrt_wts: ptr::null_mut(),
        mu: &mut (*state).mu,
        params,
        solver_state: (*state).solver_state,
        fdf: ptr::null_mut(),
        avratio: &mut (*state).avratio,
    };
    let mut rho: f64;
    let mut actual_reduction: f64;
    let mut pred_reduction: f64;
    let u: f64;

    if normf_trial >= normf {
        return -1.0;
    }

    u = normf_trial / normf;
    actual_reduction = 1.0 - u * u;

    let trs_preduction = (*trs).preduction;
    let status = trs_preduction(&mut trust_state, dx, &mut pred_reduction, (*state).trs_state);
    if status != GSL_SUCCESS {
        return -1.0;
    }

    if pred_reduction > 0.0 {
        rho = actual_reduction / pred_reduction;
    } else {
        rho = -1.0;
    }

    rho
}

unsafe extern "C" fn trust_eval_step(
    f: *const gsl_vector,
    f_trial: *const gsl_vector,
    g: *const gsl_vector,
    J: *const gsl_matrix,
    dx: *const gsl_vector,
    rho: *mut f64,
    state: *mut TrustState,
) -> c_int {
    let params = &(*state).params;
    let mut status = GSL_SUCCESS;

    if params.trs == gsl_multifit_nlinear_trs_lmaccel {
        if (*state).avratio > params.avmax {
            status = GSL_FAILURE;
        }
    }

    *rho = trust_calc_rho(f, f_trial, g, J, dx, state);
    if *rho <= 0.0 {
        status = GSL_FAILURE;
    }

    status
}

unsafe extern "C" fn trust_scaled_norm(D: *const gsl_vector, a: *const gsl_vector) -> f64 {
    let n = (*a).size;
    let mut e2 = 0.0;

    for i in 0..n {
        let Di = gsl_vector_get(D, i);
        let ai = gsl_vector_get(a, i);
        let u = Di * ai;
        e2 += u * u;
    }

    e2.sqrt()
}

static TRUST_TYPE: GSL_MULTIFIT_NLINEAR_TYPE = GSL_MULTIFIT_NLINEAR_TYPE {
    name: b"trust-region\0".as_ptr() as *const i8,
    alloc: Some(trust_alloc),
    init: Some(trust_init),
    iterate: Some(trust_iterate),
    rcond: Some(trust_rcond),
    avratio: Some(trust_avratio),
    free: Some(trust_free),
};

#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_nlinear_trust() -> *const GSL_MULTIFIT_NLINEAR_TYPE {
    &TRUST_TYPE
}