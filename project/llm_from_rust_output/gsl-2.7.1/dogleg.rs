use libc::{c_double, c_int, c_ulong, c_void};
use std::ffi::CString;
use std::ptr;

type size_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum gsl_multifit_nlinear_fdtype {
    GSL_MULTIFIT_NLINEAR_FWDIFF = 0,
    GSL_MULTIFIT_NLINEAR_CTRDIFF = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_multifit_nlinear_fdf {
    pub f: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut gsl_vector) -> c_int>,
    pub df: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut gsl_matrix) -> c_int>,
    pub fvv: Option<unsafe extern "C" fn(*const gsl_vector, *const gsl_vector, *mut c_void, *mut gsl_vector) -> c_int>,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_multifit_nlinear_trs {
    pub name: *const libc::c_char,
    pub alloc: Option<unsafe extern "C" fn(*const c_void, size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub preloop: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub step: Option<unsafe extern "C" fn(*const c_void, c_double, *mut gsl_vector, *mut c_void) -> c_int>,
    pub preduction: Option<unsafe extern "C" fn(*const c_void, *const gsl_vector, *mut c_double, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_multifit_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_multifit_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option<unsafe extern "C" fn(size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub presolve: Option<unsafe extern "C" fn(c_double, *const c_void, *mut c_void) -> c_int>,
    pub solve: Option<unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector, *const c_void, *mut c_void) -> c_int>,
    pub rcond: Option<unsafe extern "C" fn(*mut c_double, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_multifit_nlinear_parameters {
    pub trs: *const gsl_multifit_nlinear_trs,
    pub scale: *const gsl_multifit_nlinear_scale,
    pub solver: *const gsl_multifit_nlinear_solver,
    pub fdtype: gsl_multifit_nlinear_fdtype,
    pub factor_up: c_double,
    pub factor_down: c_double,
    pub avmax: c_double,
    pub h_df: c_double,
    pub h_fvv: c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_multifit_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub J: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const c_double,
    pub params: *const gsl_multifit_nlinear_parameters,
    pub solver_state: *mut c_void,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub avratio: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dogleg_state_t {
    pub n: size_t,
    pub p: size_t,
    pub dx_gn: *mut gsl_vector,
    pub dx_sd: *mut gsl_vector,
    pub norm_Dgn: c_double,
    pub norm_Dsd: c_double,
    pub norm_Dinvg: c_double,
    pub norm_JDinv2g: c_double,
    pub workp: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub params: gsl_multifit_nlinear_parameters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum GSL_Error {
    GSL_SUCCESS = 0,
    GSL_FAILURE = -1,
    GSL_CONTINUE = -2,
    GSL_EDOM = 1,
    GSL_ERANGE = 2,
    GSL_EFAULT = 3,
    GSL_EINVAL = 4,
    GSL_EFAILED = 5,
    GSL_EFACTOR = 6,
    GSL_ESANITY = 7,
    GSL_ENOMEM = 8,
    GSL_EBADFUNC = 9,
    GSL_ERUNAWAY = 10,
    GSL_EMAXITER = 11,
    GSL_EZERODIV = 12,
    GSL_EBADTOL = 13,
    GSL_ETOL = 14,
    GSL_EUNDRFLW = 15,
    GSL_EOVRFLW = 16,
    GSL_ELOSS = 17,
    GSL_EROUND = 18,
    GSL_EBADLEN = 19,
    GSL_ENOTSQR = 20,
    GSL_ESING = 21,
    GSL_EDIVERGE = 22,
    GSL_EUNSUP = 23,
    GSL_EUNIMPL = 24,
    GSL_ECACHE = 25,
    GSL_ETABLE = 26,
    GSL_ENOPROG = 27,
    GSL_ENOPROGJ = 28,
    GSL_ETOLF = 29,
    GSL_ETOLX = 30,
    GSL_ETOLG = 31,
    GSL_EOF = 32,
}

extern "C" {
    fn sqrt(_: c_double) -> c_double;
    fn fabs(_: c_double) -> c_double;
    fn calloc(_: size_t, _: size_t) -> *mut c_void;
    fn free(__ptr: *mut c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: c_int,
        gsl_errno: c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> c_double;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut c_double,
    ) -> c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE,
        alpha: c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: c_double,
        Y: *mut gsl_vector,
    ) -> c_int;
}

unsafe fn gsl_vector_get(v: *const gsl_vector, i: size_t) -> c_double {
    *(*v).data.offset((i * (*v).stride) as isize)
}

unsafe fn gsl_vector_set(v: *mut gsl_vector, i: size_t, x: c_double) {
    *(*v).data.offset((i * (*v).stride) as isize) = x;
}

unsafe fn scaled_enorm(d: *const gsl_vector, f: *const gsl_vector) -> c_double {
    let mut e2 = 0.0;
    let n = (*f).size;
    for i in 0..n {
        let fi = gsl_vector_get(f, i);
        let di = gsl_vector_get(d, i);
        let u = di * fi;
        e2 += u * u;
    }
    sqrt(e2)
}

unsafe fn scaled_addition(
    alpha: c_double,
    x: *const gsl_vector,
    beta: c_double,
    y: *const gsl_vector,
    z: *mut gsl_vector,
) {
    let N = (*z).size;
    for i in 0..N {
        let xi = gsl_vector_get(x, i);
        let yi = gsl_vector_get(y, i);
        gsl_vector_set(z, i, alpha * xi + beta * yi);
    }
}

unsafe fn quadratic_preduction(
    f: *const gsl_vector,
    J: *const gsl_matrix,
    dx: *const gsl_vector,
    work: *mut gsl_vector,
) -> c_double {
    let n = (*f).size;
    let normf = gsl_blas_dnrm2(f);
    let mut pred_reduction = 0.0;
    let mut norm_beta = 0.0;

    gsl_blas_dgemv(CBLAS_TRANSPOSE::CblasNoTrans, 1.0 / normf, J, dx, 0.0, work);
    norm_beta = gsl_blas_dnrm2(work);
    pred_reduction = -norm_beta * norm_beta;

    for i in 0..n {
        let fi = gsl_vector_get(f, i);
        let betai = gsl_vector_get(work, i);
        pred_reduction -= 2.0 * (fi / normf) * betai;
    }

    pred_reduction
}

unsafe fn dogleg_alloc(
    params: *const c_void,
    n: size_t,
    p: size_t,
) -> *mut c_void {
    let mparams = params as *const gsl_multifit_nlinear_parameters;
    let state = calloc(1, std::mem::size_of::<dogleg_state_t>() as size_t) as *mut dogleg_state_t;

    if state.is_null() {
        gsl_error(
            b"failed to allocate dogleg state\0".as_ptr() as *const _,
            b"dogleg.c\0".as_ptr() as *const _,
            85,
            GSL_Error::GSL_ENOMEM as c_int,
        );
        return ptr::null_mut();
    }

    (*state).dx_gn = gsl_vector_alloc(p);
    if (*state).dx_gn.is_null() {
        gsl_error(
            b"failed to allocate space for dx_gn\0".as_ptr() as *const _,
            b"dogleg.c\0".as_ptr() as *const _,
            91,
            GSL_Error::GSL_ENOMEM as c_int,
        );
        return ptr::null_mut();
    }

    (*state).dx_sd = gsl_vector_alloc(p);
    if (*state).dx_sd.is_null() {
        gsl_error(
            b"failed to allocate space for dx_sd\0".as_ptr() as *const _,
            b"dogleg.c\0".as_ptr() as *const _,
            97,
            GSL_Error::GSL_ENOMEM as c_int,
        );
        return ptr::null_mut();
    }

    (*state).workp = gsl_vector_alloc(p);
    if (*state).workp.is_null() {
        gsl_error(
            b"failed to allocate space for workp\0".as_ptr() as *const _,
            b"dogleg.c\0".as_ptr() as *const _,
            103,
            GSL_Error::GSL_ENOMEM as c_int,
        );
        return ptr::null_mut();
    }

    (*state).workn = gsl_vector_alloc(n);
    if (*state).workn.is_null() {
        gsl_error(
            b"failed to allocate space for workn\0".as_ptr() as *const _,
            b"dogleg.c\0".as_ptr() as *const _,
            109,
            GSL_Error::GSL_ENOMEM as c_int,
        );
        return ptr::null_mut();
    }

    (*state).n = n;
    (*state).p = p;
    (*state).params = *mparams;
    state as *mut c_void
}

unsafe fn dogleg_free(vstate: *mut c_void) {
    let state = vstate as *mut dogleg_state_t;
    if !(*state).dx_gn.is_null() {
        gsl_vector_free((*state).dx_gn);
    }
    if !(*state).dx_sd.is_null() {
        gsl_vector_free((*state).dx_sd);
    }
    if !(*state).workp.is_null() {
        gsl_vector_free((*state).workp);
    }
    if !(*state).workn.is_null() {
        gsl_vector_free((*state).workn);
    }
    free(state as *mut c_void);
}

unsafe fn dogleg_init(_vtrust_state: *const c_void, _vstate: *mut c_void) -> c_int {
    GSL_Error::GSL_SUCCESS as c_int
}

unsafe fn dogleg_preloop(vtrust_state: *const c_void, vstate: *mut c_void) -> c_int {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let state = vstate as *mut dogleg_state_t;

    gsl_vector_memcpy((*state).workp, (*trust_state).g);
    gsl_vector_div((*state).workp, (*trust_state).diag);
    (*state).norm_Dinvg = gsl_blas_dnrm2((*state).workp);

    gsl_vector_div((*state).workp, (*trust_state).diag);
    gsl_blas_dgemv(
        CBLAS_TRANSPOSE::CblasNoTrans,
        1.0,
        (*trust_state).J,
        (*state).workp,
        0.0,
        (*state).workn,
    );
    (*state).norm_JDinv2g = gsl_blas_dnrm2((*state).workn);

    let u = (*state).norm_Dinvg / (*state).norm_JDinv2g;
    let alpha = u * u;

    gsl_vector_memcpy((*state).dx_sd, (*state).workp);
    gsl_vector_scale((*state).dx_sd, -alpha);
    (*state).norm_Dsd = scaled_enorm((*trust_state).diag, (*state).dx_sd);
    (*state).norm_Dgn = -1.0;

    GSL_Error::GSL_SUCCESS as c_int
}

unsafe fn dogleg_step(
    vtrust_state: *const c_void,
    delta: c_double,
    dx: *mut gsl_vector,
    vstate: *mut c_void,
) -> c_int {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let state = vstate as *mut dogleg_state_t;

    if (*state).norm_Dsd >= delta {
        gsl_vector_memcpy(dx, (*state).dx_sd);
        gsl_vector_scale(dx, delta / (*state).norm_Dsd);
    } else {
        if (*state).norm_Dgn < 0.