use libc::{c_double, c_int, c_uint, c_ulong, c_void};
use std::ffi::CStr;
use std::mem;
use std::ptr;

type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_vector_view {
    pub vector: gsl_vector,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
pub enum gsl_multifit_nlinear_fdtype {
    FwDiff = 0,
    CtDiff = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trs {
    pub name: *const i8,
    pub alloc: Option<unsafe extern "C" fn(*const c_void, size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub preloop: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub step: Option<unsafe extern "C" fn(*const c_void, c_double, *mut gsl_vector, *mut c_void) -> c_int>,
    pub preduction: Option<unsafe extern "C" fn(*const c_void, *const gsl_vector, *mut c_double, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_multifit_nlinear_scale {
    pub name: *const i8,
    pub init: Option<unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> c_int>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_multifit_nlinear_solver {
    pub name: *const i8,
    pub alloc: Option<unsafe extern "C" fn(size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub presolve: Option<unsafe extern "C" fn(c_double, *const c_void, *mut c_void) -> c_int>,
    pub solve: Option<unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector, *const c_void, *mut c_void) -> c_int>,
    pub rcond: Option<unsafe extern "C" fn(*mut c_double, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_multifit_nlinear_type {
    pub name: *const i8,
    pub alloc: Option<unsafe extern "C" fn(*const gsl_multifit_nlinear_parameters, size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*mut c_void, *const gsl_vector, *mut gsl_multifit_nlinear_fdf, *const gsl_vector, *mut gsl_vector, *mut gsl_matrix, *mut gsl_vector) -> c_int>,
    pub iterate: Option<unsafe extern "C" fn(*mut c_void, *const gsl_vector, *mut gsl_multifit_nlinear_fdf, *mut gsl_vector, *mut gsl_vector, *mut gsl_matrix, *mut gsl_vector, *mut gsl_vector) -> c_int>,
    pub rcond: Option<unsafe extern "C" fn(*mut c_double, *mut c_void) -> c_int>,
    pub avratio: Option<unsafe extern "C" fn(*mut c_void) -> c_double>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_multifit_nlinear_workspace {
    pub type_: *const gsl_multifit_nlinear_type,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub g: *mut gsl_vector,
    pub J: *mut gsl_matrix,
    pub sqrt_wts_work: *mut gsl_vector,
    pub sqrt_wts: *mut gsl_vector,
    pub niter: size_t,
    pub params: gsl_multifit_nlinear_parameters,
    pub state: *mut c_void,
}

#[no_mangle]
pub extern "C" fn gsl_multifit_nlinear_default_parameters() -> gsl_multifit_nlinear_parameters {
    gsl_multifit_nlinear_parameters {
        trs: ptr::null(),
        scale: ptr::null(),
        solver: ptr::null(),
        fdtype: gsl_multifit_nlinear_fdtype::FwDiff,
        factor_up: 3.0,
        factor_down: 2.0,
        avmax: 0.75,
        h_df: 1.4901161193847656e-08,
        h_fvv: 0.02,
    }
}

#[no_mangle]
pub extern "C" fn gsl_multifit_nlinear_alloc(
    T: *const gsl_multifit_nlinear_type,
    params: *const gsl_multifit_nlinear_parameters,
    n: size_t,
    p: size_t,
) -> *mut gsl_multifit_nlinear_workspace {
    unsafe {
        if n < p {
            gsl_error(
                b"insufficient data points, n < p\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                37,
                GSL_EINVAL,
            );
            return ptr::null_mut();
        }

        let w = libc::calloc(1, mem::size_of::<gsl_multifit_nlinear_workspace>() as size_t)
            as *mut gsl_multifit_nlinear_workspace;
        if w.is_null() {
            gsl_error(
                b"failed to allocate space for multifit workspace\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                44,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).x = gsl_vector_calloc(p);
        if (*w).x.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for x\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                51,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).f = gsl_vector_calloc(n);
        if (*w).f.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for f\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                58,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).dx = gsl_vector_calloc(p);
        if (*w).dx.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for dx\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                65,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).g = gsl_vector_alloc(p);
        if (*w).g.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for g\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                72,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).J = gsl_matrix_alloc(n, p);
        if (*w).J.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for Jacobian\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                79,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).sqrt_wts_work = gsl_vector_calloc(n);
        if (*w).sqrt_wts_work.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for weights\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                86,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).state = ((*T).alloc.unwrap())(params, n, p);
        if (*w).state.is_null() {
            gsl_multifit_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for multifit state\0".as_ptr() as *const i8,
                b"fdf.c\0".as_ptr() as *const i8,
                93,
                GSL_ENOMEM,
            );
            return ptr::null_mut();
        }

        (*w).type_ = T;
        (*w).fdf = ptr::null_mut();
        (*w).niter = 0;
        (*w).params = *params;
        w
    }
}

#[no_mangle]
pub extern "C" fn gsl_multifit_nlinear_free(w: *mut gsl_multifit_nlinear_workspace) {
    if w.is_null() {
        return;
    }

    unsafe {
        if !(*w).state.is_null() {
            ((*(*w).type_).free.unwrap())((*w).state);
        }

        if !(*w).dx.is_null() {
            gsl_vector_free((*w).dx);
        }

        if !(*w).x.is_null() {
            gsl_vector_free((*w).x);
        }

        if !(*w).f.is_null() {
            gsl_vector_free((*w).f);
        }

        if !(*w).sqrt_wts_work.is_null() {
            gsl_vector_free((*w).sqrt_wts_work);
        }

        if !(*w).g.is_null() {
            gsl_vector_free((*w).g);
        }

        if !(*w).J.is_null() {
            gsl_matrix_free((*w).J);
        }

        libc::free(w as *mut c_void);
    }
}

const GSL_SUCCESS: c_int = 0;
const GSL_FAILURE: c_int = -1;
const GSL_CONTINUE: c_int = -2;
const GSL_EDOM: c_int = 1;
const GSL_ERANGE: c_int = 2;
const GSL_EFAULT: c_int = 3;
const GSL_EINVAL: c_int = 4;
const GSL_EFAILED: c_int = 5;
const GSL_EFACTOR: c_int = 6;
const GSL_ESANITY: c_int = 7;
const GSL_ENOMEM: c_int = 8;
const GSL_EBADFUNC: c_int = 9;
const GSL_ERUNAWAY: c_int = 10;
const GSL_EMAXITER: c_int = 11;
const GSL_EZERODIV: c_int = 12;
const GSL_EBADTOL: c_int = 13;
const GSL_ETOL: c_int = 14;
const GSL_EUNDRFLW: c_int = 15;
const GSL_EOVRFLW: c_int = 16;
const GSL_ELOSS: c_int = 17;
const GSL_EROUND: c_int = 18;
const GSL_EBADLEN: c_int = 19;
const GSL_ENOTSQR: c_int = 20;
const GSL_ESING: c_int = 21;
const GSL_EDIVERGE: c_int = 22;
const GSL_EUNSUP: c_int = 23;
const GSL_EUNIMPL: c_int = 24;
const GSL_ECACHE: c_int = 25;
const GSL_ETABLE: c_int = 26;
const GSL_ENOPROG: c_int = 27;
const GSL_ENOPROGJ: c_int = 28;
const GSL_ETOLF: c_int = 29;
const GSL_ETOLX: c_int = 30;
const GSL_ETOLG: c_int = 31;
const GSL_EOF: c_int = 32;

extern "C" {
    fn gsl_error(reason: *const i8, file: *const i8, line: c_int, gsl_errno: c_int);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> gsl_vector_view;
    static gsl_multifit_nlinear_solver_qr: *const gsl_multifit_nlinear_solver;
    static gsl_multifit_nlinear_scale_more: *const gsl_multifit_nlinear_scale;
    static gsl_multifit_nlinear_trs_lm: *const gsl_multifit_nlinear_trs;
    fn gsl_multifit_nlinear_test(
        xtol: c_double,
        gtol: c_double,
        ftol: c_double,
        info: *mut c_int,
        w: *const gsl_multifit_nlinear_workspace,
    ) -> c_int;
    fn gsl_multifit_nlinear_fdfvv(
        h: c_double,
        x: *const gsl_vector,
        v: *const gsl_vector,
        f: *const gsl_vector,
        J: *const gsl_matrix,
        swts: *const gsl_vector,
        fdf: *mut gsl_multifit_nlinear_fdf,
        fvv: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> c_int;
    fn gsl_multifit_nlinear_df(
        h: c_double,
        fdtype: gsl_multifit_nlinear_fdtype,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        fdf: *mut gsl_multifit_nlinear_fdf,
        f: *const gsl_vector,
        J: *mut gsl_matrix,
        work: *mut gsl_vector