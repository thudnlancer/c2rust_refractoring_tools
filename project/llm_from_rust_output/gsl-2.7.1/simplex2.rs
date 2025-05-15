use libc::{c_char, c_double, c_int, c_ulong, c_void};
use std::ptr;

type size_t = c_ulong;

pub const GSL_SUCCESS: c_int = 0;
pub const GSL_FAILURE: c_int = -1;
pub const GSL_CONTINUE: c_int = -2;
pub const GSL_EDOM: c_int = 1;
pub const GSL_ERANGE: c_int = 2;
pub const GSL_EFAULT: c_int = 3;
pub const GSL_EINVAL: c_int = 4;
pub const GSL_EFAILED: c_int = 5;
pub const GSL_EFACTOR: c_int = 6;
pub const GSL_ESANITY: c_int = 7;
pub const GSL_ENOMEM: c_int = 8;
pub const GSL_EBADFUNC: c_int = 9;
pub const GSL_ERUNAWAY: c_int = 10;
pub const GSL_EMAXITER: c_int = 11;
pub const GSL_EZERODIV: c_int = 12;
pub const GSL_EBADTOL: c_int = 13;
pub const GSL_ETOL: c_int = 14;
pub const GSL_EUNDRFLW: c_int = 15;
pub const GSL_EOVRFLW: c_int = 16;
pub const GSL_ELOSS: c_int = 17;
pub const GSL_EROUND: c_int = 18;
pub const GSL_EBADLEN: c_int = 19;
pub const GSL_ENOTSQR: c_int = 20;
pub const GSL_ESING: c_int = 21;
pub const GSL_EDIVERGE: c_int = 22;
pub const GSL_EUNSUP: c_int = 23;
pub const GSL_EUNIMPL: c_int = 24;
pub const GSL_ECACHE: c_int = 25;
pub const GSL_ETABLE: c_int = 26;
pub const GSL_ENOPROG: c_int = 27;
pub const GSL_ENOPROGJ: c_int = 28;
pub const GSL_ETOLF: c_int = 29;
pub const GSL_ETOLX: c_int = 30;
pub const GSL_ETOLG: c_int = 31;
pub const GSL_EOF: c_int = 32;

#[repr(C)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
pub struct gsl_vector_view {
    pub vector: gsl_vector,
}

#[repr(C)]
pub struct gsl_vector_const_view {
    pub vector: gsl_vector,
}

#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
pub struct gsl_matrix_view {
    pub matrix: gsl_matrix,
}

#[repr(C)]
pub struct gsl_multimin_function {
    pub f: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void) -> c_double>,
    pub n: size_t,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct gsl_multimin_fminimizer_type {
    pub name: *const c_char,
    pub size: size_t,
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t) -> c_int>,
    pub set: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut gsl_multimin_function,
            *const gsl_vector,
            *mut c_double,
            *const gsl_vector,
        ) -> c_int,
    >,
    pub iterate: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut gsl_multimin_function,
            *mut gsl_vector,
            *mut c_double,
            *mut c_double,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct nmsimplex_state_t {
    pub x1: *mut gsl_matrix,
    pub y1: *mut gsl_vector,
    pub ws1: *mut gsl_vector,
    pub ws2: *mut gsl_vector,
    pub center: *mut gsl_vector,
    pub delta: *mut gsl_vector,
    pub xmc: *mut gsl_vector,
    pub S2: c_double,
    pub count: c_ulong,
}

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_min_index(v: *const gsl_vector) -> size_t;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> gsl_vector_view;
    fn gsl_matrix_const_row(m: *const gsl_matrix, i: size_t) -> gsl_vector_const_view;
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_matrix_get_row(v: *mut gsl_vector, m: *const gsl_matrix, i: size_t) -> c_int;
    fn gsl_matrix_set_row(m: *mut gsl_matrix, i: size_t, v: *const gsl_vector) -> c_int;
    fn gsl_blas_ddot(X: *const gsl_vector, Y: *const gsl_vector, result: *mut c_double) -> c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> c_double;
    fn gsl_blas_daxpy(alpha: c_double, X: *const gsl_vector, Y: *mut gsl_vector) -> c_int;
    fn gsl_blas_drot(
        X: *mut gsl_vector,
        Y: *mut gsl_vector,
        c: c_double,
        s: c_double,
    ) -> c_int;
    fn gsl_blas_dscal(alpha: c_double, X: *mut gsl_vector);
    fn gsl_finite(x: c_double) -> c_int;
}

#[inline]
unsafe fn gsl_vector_get(v: *const gsl_vector, i: size_t) -> c_double {
    *(*v).data.offset(i.wrapping_mul((*v).stride) as isize)
}

#[inline]
unsafe fn gsl_vector_set(v: *mut gsl_vector, i: size_t, x: c_double) {
    *(*v).data.offset(i.wrapping_mul((*v).stride) as isize = x;
}

#[inline]
unsafe fn gsl_matrix_get(m: *const gsl_matrix, i: size_t, j: size_t) -> c_double {
    *(*m).data.offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
}

#[inline]
unsafe fn gsl_matrix_set(m: *mut gsl_matrix, i: size_t, j: size_t, x: c_double) {
    *(*m).data.offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize = x;
}

unsafe fn try_corner_move(
    coeff: c_double,
    state: *const nmsimplex_state_t,
    corner: size_t,
    xc: *mut gsl_vector,
    f: *const gsl_multimin_function,
) -> c_double {
    let x1 = (*state).x1;
    let P = (*x1).size1;
    let alpha = (1.0 - coeff) * P as c_double / (P as c_double - 1.0);
    let beta = (P as c_double * coeff - 1.0) / (P as c_double - 1.0);
    let row = gsl_matrix_const_row(x1, corner);
    gsl_vector_memcpy(xc, (*state).center);
    gsl_blas_dscal(alpha, xc);
    gsl_blas_daxpy(beta, &row.vector, xc);
    ((*f).f.unwrap())(xc, (*f).params)
}

unsafe fn update_point(
    state: *mut nmsimplex_state_t,
    i: size_t,
    x: *const gsl_vector,
    val: c_double,
) {
    let x_orig = gsl_matrix_const_row((*state).x1, i);
    let P = (*(*state).x1).size1;
    gsl_vector_memcpy((*state).delta, x);
    gsl_blas_daxpy(-1.0, &x_orig.vector, (*state).delta);
    gsl_vector_memcpy((*state).xmc, &x_orig.vector);
    gsl_blas_daxpy(-1.0, (*state).center, (*state).xmc);
    let d = gsl_blas_dnrm2((*state).delta);
    let mut xmcd = 0.0;
    gsl_blas_ddot((*state).xmc, (*state).delta, &mut xmcd);
    (*state).S2 += 2.0 / P as c_double * xmcd
        + (P as c_double - 1.0) / P as c_double * (d * d / P as c_double);
    let alpha = 1.0 / P as c_double;
    gsl_blas_daxpy(-alpha, &x_orig.vector, (*state).center);
    gsl_blas_daxpy(alpha, x, (*state).center);
    gsl_matrix_set_row((*state).x1, i, x);
    gsl_vector_set((*state).y1, i, val);
}

unsafe fn contract_by_best(
    state: *mut nmsimplex_state_t,
    best: size_t,
    xc: *mut gsl_vector,
    f: *mut gsl_multimin_function,
) -> c_int {
    let x1 = (*state).x1;
    let y1 = (*state).y1;
    let mut status = GSL_SUCCESS;
    for i in 0..(*x1).size1 {
        if i != best {
            for j in 0..(*x1).size2 {
                let newval = 0.5 * (gsl_matrix_get(x1, i, j) + gsl_matrix_get(x1, best, j));
                gsl_matrix_set(x1, i, j, newval);
            }
            gsl_matrix_get_row(xc, x1, i);
            let newval = ((*f).f.unwrap())(xc, (*f).params);
            gsl_vector_set(y1, i, newval);
            if gsl_finite(newval) == 0 {
                status = GSL_EBADFUNC;
            }
        }
    }
    compute_center(state, (*state).center);
    compute_size(state, (*state).center);
    status
}

unsafe fn compute_center(state: *const nmsimplex_state_t, center: *mut gsl_vector) -> c_int {
    let x1 = (*state).x1;
    let P = (*x1).size1;
    gsl_vector_set_zero(center);
    for i in 0..P {
        let row = gsl_matrix_const_row(x1, i);
        gsl_blas_daxpy(1.0, &row.vector, center);
    }
    let alpha = 1.0 / P as c_double;
    gsl_blas_dscal(alpha, center);
    GSL_SUCCESS
}

unsafe fn compute_size(state: *mut nmsimplex_state_t, center: *const gsl_vector) -> c_double {
    let s = (*state).ws1;
    let x1 = (*state).x1;
    let P = (*x1).size1;
    let mut ss = 0.0;
    for i in 0..P {
        gsl_matrix_get_row(s, x1, i);
        gsl_blas_daxpy(-1.0, center, s);
        let t = gsl_blas_dnrm2(s);
        ss += t * t;
    }
    (*state).S2 = ss / P as c_double;
    (ss / P as c_double).sqrt()
}

unsafe fn nmsimplex_alloc(vstate: *mut c_void, n: size_t) -> c_int {
    let state = vstate as *mut nmsimplex_state_t;
    if n == 0 {
        gsl_error(
            b"invalid number of parameters specified\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            251,
            GSL_EINVAL,
        );
        return GSL_EINVAL;
    }
    (*state).x1 = gsl_matrix_alloc(n + 1, n);
    if (*state).x1.is_null() {
        gsl_error(
            b"failed to allocate space for x1\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            258,
            GSL_ENOMEM,
        );
        return GSL_ENOMEM;
    }
    (*state).y1 = gsl_vector_alloc(n + 1);
    if (*state).y1.is_null() {
        gsl_matrix_free((*state).x1);
        gsl_error(
            b"failed to allocate space for y\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            266,
            GSL_ENOMEM,
        );
        return GSL_ENOMEM;
    }
    (*state).ws1 = gsl_vector_alloc(n);
    if (*state).ws1.is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_error(
            b"failed to allocate space for ws1\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            275,
            GSL_ENOMEM,
        );
        return GSL_ENOMEM;
    }
    (*state).ws2 = gsl_vector_alloc(n);
    if (*state).ws2.is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_vector_free((*state).ws1);
        gsl_error(
            b"failed to allocate space for ws2\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            285,
            GSL_ENOMEM,
        );
        return GSL_ENOMEM;
    }
    (*state).center = gsl_vector_alloc(n);
    if (*state).center.is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_vector_free((*state).ws1);
        gsl_vector_free((*state).ws2);
        gsl_error(
            b"failed to allocate space for center\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            296,
            GSL_ENOMEM,
        );
        return GSL_ENOMEM;
    }
    (*state).delta = gsl_vector_alloc(n);
    if (*state).delta.is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_vector_free((*state).ws1);
        gsl_vector_free((*state).ws2);
        gsl_vector_free((*state).center);
        gsl_error(
            b"failed to allocate space for delta\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            308,
            GSL_ENOMEM,
        );
        return GSL_ENOMEM;
    }
    (*state).xmc = gsl_vector_alloc(n);
    if (*state).xmc.is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_vector_free((*state).ws1);
        gsl_vector_free((*state).ws2);
        gsl_vector_free((*state).center);
        gsl_vector_free((*state).delta);
        gsl_error(
            b"failed to allocate space for xmc\0".as_ptr() as *const c_char,
            b"simplex2.c\0".as_ptr() as *const c_char,
            321,
            GSL_ENOMEM,
        );
        return GSL