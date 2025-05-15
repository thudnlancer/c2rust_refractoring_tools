use std::ffi::CString;
use std::os::raw::{c_double, c_int, c_void, c_char};
use std::ptr;

#[repr(C)]
pub struct gsl_block {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
pub struct gsl_vector {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
pub struct gsl_multimin_function_fdf {
    pub f: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void) -> c_double>,
    pub df: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut gsl_vector)>,
    pub fdf: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut c_double, *mut gsl_vector)>,
    pub n: usize,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct gsl_multimin_fdfminimizer_type {
    pub name: *const c_char,
    pub size: usize,
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, usize) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut c_void, *mut gsl_multimin_function_fdf, *const gsl_vector, *mut c_double, *mut gsl_vector, c_double, c_double) -> c_int>,
    pub iterate: Option<unsafe extern "C" fn(*mut c_void, *mut gsl_multimin_function_fdf, *mut gsl_vector, *mut c_double, *mut gsl_vector, *mut gsl_vector) -> c_int>,
    pub restart: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct ConjugateFRState {
    pub iter: c_int,
    pub step: c_double,
    pub max_step: c_double,
    pub tol: c_double,
    pub x1: *mut gsl_vector,
    pub dx1: *mut gsl_vector,
    pub x2: *mut gsl_vector,
    pub pnorm: c_double,
    pub p: *mut gsl_vector,
    pub g0norm: c_double,
    pub g0: *mut gsl_vector,
}

pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

fn take_step(
    x: &gsl_vector,
    p: &gsl_vector,
    step: c_double,
    lambda: c_double,
    x1: &mut gsl_vector,
    dx: &mut gsl_vector,
) {
    unsafe {
        gsl_vector_set_zero(dx);
        gsl_blas_daxpy(-step * lambda, p, dx);
        gsl_vector_memcpy(x1, x);
        gsl_blas_daxpy(1.0, dx, x1);
    }
}

fn intermediate_point(
    fdf: &mut gsl_multimin_function_fdf,
    x: &gsl_vector,
    p: &gsl_vector,
    lambda: c_double,
    pg: c_double,
    stepa: c_double,
    stepc: c_double,
    fa: c_double,
    fc: c_double,
    x1: &mut gsl_vector,
    dx: &mut gsl_vector,
    gradient: &mut gsl_vector,
    step: &mut c_double,
    f: &mut c_double,
) {
    unsafe {
        let mut stepb = 0.0;
        let mut fb = 0.0;
        loop {
            let u = fabs(pg * lambda * stepc);
            stepb = 0.5 * stepc * u / (fc - fa + u);
            take_step(x, p, stepb, lambda, x1, dx);
            if gsl_vector_equal(x, x1) != 0 {
                *step = 0.0;
                *f = fa;
                if let Some(df) = fdf.df {
                    df(x1, fdf.params, gradient);
                }
                return;
            }
            if let Some(f_func) = fdf.f {
                fb = f_func(x1, fdf.params);
            }
            if !(fb >= fa && stepb > 0.0) {
                break;
            }
            fc = fb;
            stepc = stepb;
        }
        *step = stepb;
        *f = fb;
        if let Some(df) = fdf.df {
            df(x1, fdf.params, gradient);
        }
    }
}

fn minimize(
    fdf: &mut gsl_multimin_function_fdf,
    x: &gsl_vector,
    p: &gsl_vector,
    lambda: c_double,
    stepa: c_double,
    stepb: c_double,
    stepc: c_double,
    fa: c_double,
    fb: c_double,
    fc: c_double,
    tol: c_double,
    x1: &mut gsl_vector,
    dx1: &mut gsl_vector,
    x2: &mut gsl_vector,
    dx2: &mut gsl_vector,
    gradient: &mut gsl_vector,
    step: &mut c_double,
    f: &mut c_double,
    gnorm: &mut c_double,
) {
    unsafe {
        let mut u = stepb;
        let mut v = stepa;
        let mut w = stepc;
        let mut fu = fb;
        let mut fv = fa;
        let mut fw = fc;
        let mut old2 = fabs(w - v);
        let mut old1 = fabs(v - u);
        let mut stepm = 0.0;
        let mut fm = 0.0;
        let mut pg = 0.0;
        let mut gnorm1 = 0.0;
        let mut iter = 0;

        gsl_vector_memcpy(x2, x1);
        gsl_vector_memcpy(dx2, dx1);
        *f = fb;
        *step = stepb;
        *gnorm = gsl_blas_dnrm2(gradient);

        loop {
            iter += 1;
            if iter > 10 {
                return;
            }

            let dw = w - u;
            let dv = v - u;
            let du = if (fv - fu) * dw + (fu - fw) * dv != 0.0 {
                ((fv - fu) * dw * dw + (fu - fw) * dv * dv) / (2.0 * ((fv - fu) * dw + (fu - fw) * dv))
            } else {
                0.0
            };

            if du > 0.0 && du < stepc - stepb && fabs(du) < 0.5 * old2 {
                stepm = u + du;
            } else if du < 0.0 && du > stepa - stepb && fabs(du) < 0.5 * old2 {
                stepm = u + du;
            } else if stepc - stepb > stepb - stepa {
                stepm = 0.38 * (stepc - stepb) + stepb;
            } else {
                stepm = stepb - 0.38 * (stepb - stepa);
            }

            take_step(x, p, stepm, lambda, x1, dx1);
            if let Some(f_func) = fdf.f {
                fm = f_func(x1, fdf.params);
            }

            if fm > fb {
                if fm < fv {
                    w = v;
                    v = stepm;
                    fw = fv;
                    fv = fm;
                } else if fm < fw {
                    w = stepm;
                    fw = fm;
                }

                if stepm < stepb {
                    stepa = stepm;
                    fa = fm;
                } else {
                    stepc = stepm;
                    fc = fm;
                }
            } else {
                if fm > fb {
                    break;
                }

                old2 = old1;
                old1 = fabs(u - stepm);
                w = v;
                v = u;
                u = stepm;
                fw = fv;
                fv = fu;
                fu = fm;

                gsl_vector_memcpy(x2, x1);
                gsl_vector_memcpy(dx2, dx1);

                if let Some(df) = fdf.df {
                    df(x1, fdf.params, gradient);
                }

                gsl_blas_ddot(p, gradient, &mut pg);
                gnorm1 = gsl_blas_dnrm2(gradient);
                *f = fm;
                *step = stepm;
                *gnorm = gnorm1;

                if fabs(pg * lambda / gnorm1) < tol {
                    return;
                }

                if stepm < stepb {
                    stepc = stepb;
                    fc = fb;
                    stepb = stepm;
                    fb = fm;
                } else {
                    stepa = stepb;
                    fa = fb;
                    stepb = stepm;
                    fb = fm;
                }
            }
        }
    }
}

fn conjugate_fr_alloc(vstate: *mut c_void, n: usize) -> c_int {
    unsafe {
        let state = vstate as *mut ConjugateFRState;
        (*state).x1 = gsl_vector_calloc(n);
        if (*state).x1.is_null() {
            gsl_error(
                CString::new("failed to allocate space for x1").unwrap().as_ptr(),
                CString::new("conjugate_fr.c").unwrap().as_ptr(),
                55,
                GslError::NoMem as c_int,
            );
            return GslError::NoMem as c_int;
        }

        (*state).dx1 = gsl_vector_calloc(n);
        if (*state).dx1.is_null() {
            gsl_vector_free((*state).x1);
            gsl_error(
                CString::new("failed to allocate space for dx1").unwrap().as_ptr(),
                CString::new("conjugate_fr.c").unwrap().as_ptr(),
                63,
                GslError::NoMem as c_int,
            );
            return GslError::NoMem as c_int;
        }

        (*state).x2 = gsl_vector_calloc(n);
        if (*state).x2.is_null() {
            gsl_vector_free((*state).dx1);
            gsl_vector_free((*state).x1);
            gsl_error(
                CString::new("failed to allocate space for x2").unwrap().as_ptr(),
                CString::new("conjugate_fr.c").unwrap().as_ptr(),
                72,
                GslError::NoMem as c_int,
            );
            return GslError::NoMem as c_int;
        }

        (*state).p = gsl_vector_calloc(n);
        if (*state).p.is_null() {
            gsl_vector_free((*state).x2);
            gsl_vector_free((*state).dx1);
            gsl_vector_free((*state).x1);
            gsl_error(
                CString::new("failed to allocate space for p").unwrap().as_ptr(),
                CString::new("conjugate_fr.c").unwrap().as_ptr(),
                82,
                GslError::NoMem as c_int,
            );
            return GslError::NoMem as c_int;
        }

        (*state).g0 = gsl_vector_calloc(n);
        if (*state).g0.is_null() {
            gsl_vector_free((*state).p);
            gsl_vector_free((*state).x2);
            gsl_vector_free((*state).dx1);
            gsl_vector_free((*state).x1);
            gsl_error(
                CString::new("failed to allocate space for g0").unwrap().as_ptr(),
                CString::new("conjugate_fr.c").unwrap().as_ptr(),
                93,
                GslError::NoMem as c_int,
            );
            return GslError::NoMem as c_int;
        }

        GslError::Success as c_int
    }
}

fn conjugate_fr_set(
    vstate: *mut c_void,
    fdf: *mut gsl_multimin_function_fdf,
    x: *const gsl_vector,
    f: *mut c_double,
    gradient: *mut gsl_vector,
    step_size: c_double,
    tol: c_double,
) -> c_int {
    unsafe {
        let state = vstate as *mut ConjugateFRState;
        (*state).iter = 0;
        (*state).step = step_size;
        (*state).max_step = step_size;
        (*state).tol = tol;

        if let Some(fdf_func) = (*fdf).fdf {
            fdf_func(x, (*fdf).params, f, gradient);
        }

        gsl_vector_memcpy((*state).p, gradient);
        gsl_vector_memcpy((*state).g0, gradient);
        let gnorm = gsl_blas_dnrm2(gradient);
        (*state).pnorm = gnorm;
        (*state).g0norm = gnorm;

        GslError::Success as c_int
    }
}

fn conjugate_fr_free(vstate: *mut c_void) {
    unsafe {
        let state = vstate as *mut ConjugateFRState;
        gsl_vector_free((*state).g0);
        gsl_vector_free((*state).p);
        gsl_vector_free((*state).x2);
        gsl_vector_free((*state).dx1);
        gsl_vector_free((*state).x1);
    }
}

fn conjugate_fr_restart(vstate: *mut c_void) -> c_int {
    unsafe {
        let state = vstate as *mut ConjugateFRState;
        (*state).iter = 0;
        GslError::Success as c_int
    }
}

fn conjugate_fr_iterate(
    vstate: *mut c_void,
    fdf: *mut gsl_multimin_function_fdf,
    x: *mut gsl_vector,
    f: *mut c_double,
    gradient: *mut gsl_vector,
    dx: *mut gsl_vector,
) -> c_int {
    unsafe {
        let state = vstate as *mut ConjugateFRState;
        let x1 = (*state).x1;
        let dx1 = (*state).dx1;
        let x2 = (*state).x2;
        let p = (*state).p;
        let g0 = (*state).g0;
        let pnorm = (*state).pnorm;
        let g0norm = (*state).g0norm;
        let fa = *f;
        let mut fb = 0.0;
        let mut fc = 0.0;
        let mut dir = 0.0;
        let mut stepa = 0.0;
        let mut stepb = 0.0;
        let stepc = (*state).step;
        let tol = (*state).tol;
        let mut g1norm = 0.0;
        let mut pg = 0.0;

        if pnorm == 0.0 || g0norm == 0.0 {
            gsl_vector_set_zero(dx);
            return GslError::NoProgress as c_int;
        }

        gsl_blas_ddot(p, gradient, &mut pg);
        dir = if pg >= 0.0 { 1.0 } else { -1.0 };

        take_step(x, p, stepc, dir / pnorm, &mut *x1, &mut *dx1);
        if let Some(f_func) = (*fdf).f {
            fc = f_func(x1, (*fdf).params);
        }

        if fc < fa {
            (*state).step = stepc * 2.0;
            *f = fc;
            gsl_vector_memcpy(x, x1);
            if let Some(df) = (*fdf).df {
                df(x1, (*fdf).params, gradient);
            }
            return GslError::Success as c_int;
        }

        intermediate_point(
            &mut *fdf,
            x,
            p,
            dir / pnorm,
            pg,
            stepa,
            stepc,
            fa,
            fc,
            &mut *x1,
            &mut *dx1,
            gradient,
            &mut stepb,
            &mut fb,
        );

        if stepb == 0.0 {
            return GslError::NoProgress as c_int;
        }

        minimize(
            &mut *fdf,
            x,
            p,
            dir / pnorm,
            stepa,
            stepb,
            stepc,
            fa,
            fb,
            fc,
            tol,
            &mut *x1,
            &mut *dx1,
            &mut *x2,
            dx,
            gradient,
            &mut (*state).step,
            f,
            &mut g1norm,
        );

        gsl_vector_memcpy(x, x2);
        (*state).iter = ((*state).iter + 1) % (*(*x).size as c_int);

        if (*state).iter == 0 {
            gsl_vector_memcpy(p,