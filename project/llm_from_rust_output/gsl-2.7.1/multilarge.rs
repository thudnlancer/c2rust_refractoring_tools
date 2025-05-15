use libc::{c_char, c_double, c_int, c_ulong, c_void};
use std::ptr;

type size_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_vector_view {
    pub vector: gsl_vector,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_matrix_const_view {
    pub matrix: gsl_matrix,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_multilarge_linear_type {
    pub name: *const c_char,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub reset: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub accumulate: Option<unsafe extern "C" fn(*mut gsl_matrix, *mut gsl_vector, *mut c_void) -> c_int>,
    pub solve: Option<unsafe extern "C" fn(c_double, *mut gsl_vector, *mut c_double, *mut c_double, *mut c_void) -> c_int>,
    pub rcond: Option<unsafe extern "C" fn(*mut c_double, *mut c_void) -> c_int>,
    pub lcurve: Option<unsafe extern "C" fn(*mut gsl_vector, *mut gsl_vector, *mut gsl_vector, *mut c_void) -> c_int>,
    pub matrix_ptr: Option<unsafe extern "C" fn(*const c_void) -> *const gsl_matrix>,
    pub rhs_ptr: Option<unsafe extern "C" fn(*const c_void) -> *const gsl_vector>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_multilarge_linear_workspace {
    pub type_: *const gsl_multilarge_linear_type,
    pub state: *mut c_void,
    pub p: size_t,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

extern "C" {
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> gsl_vector_view;
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> gsl_matrix_const_view;
    fn gsl_multifit_linear_applyW(
        X: *const gsl_matrix,
        w: *const gsl_vector,
        y: *const gsl_vector,
        WX: *mut gsl_matrix,
        Wy: *mut gsl_vector,
    ) -> c_int;
    fn gsl_multifit_linear_L_decomp(L: *mut gsl_matrix, tau: *mut gsl_vector) -> c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> c_int;
}

#[inline]
unsafe fn gsl_vector_get(v: *const gsl_vector, i: size_t) -> c_double {
    *(*v).data.offset((i * (*v).stride) as isize)
}

pub unsafe fn gsl_multilarge_linear_alloc(
    T: *const gsl_multilarge_linear_type,
    p: size_t,
) -> *mut gsl_multilarge_linear_workspace {
    let w = calloc(1, std::mem::size_of::<gsl_multilarge_linear_workspace>() as size_t) as *mut gsl_multilarge_linear_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0".as_ptr() as *const c_char,
            b"multilarge.c\0".as_ptr() as *const c_char,
            39,
            GslError::ENOMEM as c_int,
        );
        return ptr::null_mut();
    }
    (*w).type_ = T;
    (*w).state = ((*(*w).type_).alloc.unwrap())(p);
    if (*w).state.is_null() {
        gsl_multilarge_linear_free(w);
        gsl_error(
            b"failed to allocate space for multilarge state\0".as_ptr() as *const c_char,
            b"multilarge.c\0".as_ptr() as *const c_char,
            49,
            GslError::ENOMEM as c_int,
        );
        return ptr::null_mut();
    }
    (*w).p = p;
    gsl_multilarge_linear_reset(w);
    w
}

pub unsafe fn gsl_multilarge_linear_free(w: *mut gsl_multilarge_linear_workspace) {
    if w.is_null() {
        return;
    }
    if !(*w).state.is_null() {
        ((*(*w).type_).free.unwrap())((*w).state);
    }
    free(w as *mut c_void);
}

pub unsafe fn gsl_multilarge_linear_name(w: *const gsl_multilarge_linear_workspace) -> *const c_char {
    (*(*w).type_).name
}

pub unsafe fn gsl_multilarge_linear_reset(w: *mut gsl_multilarge_linear_workspace) -> c_int {
    ((*(*w).type_).reset.unwrap())((*w).state)
}

pub unsafe fn gsl_multilarge_linear_accumulate(
    X: *mut gsl_matrix,
    y: *mut gsl_vector,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    ((*(*w).type_).accumulate.unwrap())(X, y, (*w).state)
}

pub unsafe fn gsl_multilarge_linear_solve(
    lambda: c_double,
    c: *mut gsl_vector,
    rnorm: *mut c_double,
    snorm: *mut c_double,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    ((*(*w).type_).solve.unwrap())(lambda, c, rnorm, snorm, (*w).state)
}

pub unsafe fn gsl_multilarge_linear_rcond(
    rcond: *mut c_double,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    ((*(*w).type_).rcond.unwrap())(rcond, (*w).state)
}

pub unsafe fn gsl_multilarge_linear_lcurve(
    reg_param: *mut gsl_vector,
    rho: *mut gsl_vector,
    eta: *mut gsl_vector,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    let len = (*reg_param).size;
    if len != (*rho).size {
        gsl_error(
            b"reg_param and rho have different sizes\0".as_ptr() as *const c_char,
            b"multilarge.c\0".as_ptr() as *const c_char,
            117,
            GslError::EBADLEN as c_int,
        );
        return GslError::EBADLEN as c_int;
    } else if len != (*eta).size {
        gsl_error(
            b"reg_param and eta have different sizes\0".as_ptr() as *const c_char,
            b"multilarge.c\0".as_ptr() as *const c_char,
            121,
            GslError::EBADLEN as c_int,
        );
        return GslError::EBADLEN as c_int;
    } else {
        ((*(*w).type_).lcurve.unwrap())(reg_param, rho, eta, (*w).state)
    }
}

// Additional functions follow the same pattern of safe wrapping around unsafe FFI calls