use libc::{c_double, c_int, c_ulong, c_void};
use std::ptr;

type size_t = c_ulong;

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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CBLAS_DIAG {
    CblasNonUnit = 131,
    CblasUnit = 132,
}

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
pub struct gsl_matrix_view {
    pub matrix: gsl_matrix,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_multifit_linear_workspace {
    pub nmax: size_t,
    pub pmax: size_t,
    pub n: size_t,
    pub p: size_t,
    pub A: *mut gsl_matrix,
    pub Q: *mut gsl_matrix,
    pub QSI: *mut gsl_matrix,
    pub S: *mut gsl_vector,
    pub t: *mut gsl_vector,
    pub xt: *mut gsl_vector,
    pub D: *mut gsl_vector,
    pub rcond: c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_multilarge_linear_type {
    pub name: *const libc::c_char,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub reset: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub accumulate: Option<
        unsafe extern "C" fn(*mut gsl_matrix, *mut gsl_vector, *mut c_void) -> c_int,
    >,
    pub solve: Option<
        unsafe extern "C" fn(
            c_double,
            *mut gsl_vector,
            *mut c_double,
            *mut c_double,
            *mut c_void,
        ) -> c_int,
    >,
    pub rcond: Option<unsafe extern "C" fn(*mut c_double, *mut c_void) -> c_int>,
    pub lcurve: Option<
        unsafe extern "C" fn(
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut c_void,
        ) -> c_int,
    >,
    pub matrix_ptr: Option<unsafe extern "C" fn(*const c_void) -> *const gsl_matrix>,
    pub rhs_ptr: Option<unsafe extern "C" fn(*const c_void) -> *const gsl_vector>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsqr_state_t {
    pub p: size_t,
    pub nblocks: c_int,
    pub rnorm: c_double,
    pub svd: c_int,
    pub T: *mut gsl_matrix,
    pub R: *mut gsl_matrix,
    pub QTb: *mut gsl_vector,
    pub work: *mut gsl_vector,
    pub work3: *mut gsl_vector,
    pub multifit_workspace_p: *mut gsl_multifit_linear_workspace,
}

extern "C" {
    fn gsl_hypot(x: c_double, y: c_double) -> c_double;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: c_int,
        gsl_errno: c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> gsl_matrix_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO,
        Diag: CBLAS_DIAG,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> c_int;
    fn gsl_linalg_QR_decomp_r(A: *mut gsl_matrix, T: *mut gsl_matrix) -> c_int;
    fn gsl_linalg_QR_QTvec_r(
        QR: *const gsl_matrix,
        T: *const gsl_matrix,
        b: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> c_int;
    fn gsl_linalg_QR_UR_decomp(
        S: *mut gsl_matrix,
        A: *mut gsl_matrix,
        T: *mut gsl_matrix,
    ) -> c_int;
    fn gsl_linalg_tri_rcond(
        Uplo: CBLAS_UPLO,
        A: *const gsl_matrix,
        rcond: *mut c_double,
        work: *mut gsl_vector,
    ) -> c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> c_int;
    fn gsl_blas_dtrmv(
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE,
        alpha: c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: c_double,
        Y: *mut gsl_vector,
    ) -> c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> c_double;
    fn gsl_multifit_linear_alloc(
        n: size_t,
        p: size_t,
    ) -> *mut gsl_multifit_linear_workspace;
    fn gsl_multifit_linear_free(w: *mut gsl_multifit_linear_workspace);
    fn gsl_multifit_linear_svd(
        X: *const gsl_matrix,
        work: *mut gsl_multifit_linear_workspace,
    ) -> c_int;
    fn gsl_multifit_linear_solve(
        lambda: c_double,
        X: *const gsl_matrix,
        y: *const gsl_vector,
        c: *mut gsl_vector,
        rnorm: *mut c_double,
        snorm: *mut c_double,
        work: *mut gsl_multifit_linear_workspace,
    ) -> c_int;
    fn gsl_multifit_linear_lcurve(
        y: *const gsl_vector,
        reg_param: *mut gsl_vector,
        rho: *mut gsl_vector,
        eta: *mut gsl_vector,
        work: *mut gsl_multifit_linear_workspace,
    ) -> c_int;
}

unsafe fn gsl_vector_ptr(v: *mut gsl_vector, i: size_t) -> *mut c_double {
    (*v).data.offset((i * (*v).stride) as isize)
}

unsafe fn tsqr_alloc(p: size_t) -> *mut c_void {
    if p == 0 {
        gsl_error(
            b"p must be a positive integer\0".as_ptr() as *const libc::c_char,
            b"tsqr.c\0".as_ptr() as *const libc::c_char,
            110,
            GSL_EINVAL,
        );
        return ptr::null_mut();
    }

    let state = calloc(1, std::mem::size_of::<tsqr_state_t>() as size_t) as *mut tsqr_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate tsqr state\0".as_ptr() as *const libc::c_char,
            b"tsqr.c\0".as_ptr() as *const libc::c_char,
            116,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).p = p;
    (*state).nblocks = 0;
    (*state).rnorm = 0.0;
    (*state).R = gsl_matrix_alloc(p, p);
    (*state).QTb = gsl_vector_alloc(p);
    (*state).T = gsl_matrix_alloc(p, p);
    (*state).work = gsl_vector_alloc(p);
    (*state).work3 = gsl_vector_alloc(3 * p);
    (*state).multifit_workspace_p = gsl_multifit_linear_alloc(p, p);

    if (*state).R.is_null()
        || (*state).QTb.is_null()
        || (*state).T.is_null()
        || (*state).work.is_null()
        || (*state).work3.is_null()
        || (*state).multifit_workspace_p.is_null()
    {
        tsqr_free(state as *mut c_void);
        gsl_error(
            b"failed to allocate tsqr components\0".as_ptr() as *const libc::c_char,
            b"tsqr.c\0".as_ptr() as *const libc::c_char,
            127,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    state as *mut c_void
}

unsafe fn tsqr_free(vstate: *mut c_void) {
    let state = vstate as *mut tsqr_state_t;
    if !(*state).R.is_null() {
        gsl_matrix_free((*state).R);
    }
    if !(*state).QTb.is_null() {
        gsl_vector_free((*state).QTb);
    }
    if !(*state).T.is_null() {
        gsl_matrix_free((*state).T);
    }
    if !(*state).work.is_null() {
        gsl_vector_free((*state).work);
    }
    if !(*state).work3.is_null() {
        gsl_vector_free((*state).work3);
    }
    if !(*state).multifit_workspace_p.is_null() {
        gsl_multifit_linear_free((*state).multifit_workspace_p);
    }
    free(state as *mut c_void);
}

unsafe fn tsqr_reset(vstate: *mut c_void) -> c_int {
    let state = vstate as *mut tsqr_state_t;
    gsl_matrix_set_zero((*state).R);
    gsl_vector_set_zero((*state).QTb);
    (*state).nblocks = 0;
    (*state).rnorm = 0.0;
    (*state).svd = 0;
    GSL_SUCCESS
}

unsafe fn tsqr_accumulate(
    A: *mut gsl_matrix,
    b: *mut gsl_vector,
    vstate: *mut c_void,
) -> c_int {
    let state = vstate as *mut tsqr_state_t;
    let n = (*A).size1;
    let p = (*A).size2;

    if p != (*state).p {
        gsl_error(
            b"columns of A do not match workspace\0".as_ptr() as *const libc::c_char,
            b"tsqr.c\0".as_ptr() as *const libc::c_char,
            236,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    if n != (*b).size {
        gsl_error(
            b"A and b have different numbers of rows\0".as_ptr() as *const libc::c_char,
            b"tsqr.c\0".as_ptr() as *const libc::c_char,
            240,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    if (*state).nblocks == 0 && n < p {
        gsl_error(
            b"n must be >= p\0".as_ptr() as *const libc::c_char,
            b"tsqr.c\0".as_ptr() as *const libc::c_char,
            244,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    if (*state).nblocks == 0 {
        let status = gsl_linalg_QR_decomp_r(A, (*state).T);
        if status != GSL_SUCCESS {
            return status;
        }

        let R = gsl_matrix_submatrix(A, 0, 0, p, p);
        gsl_matrix_tricpy(CblasUpper, CblasNonUnit, (*state).R, &R.matrix);

        let QTb = gsl_vector_subvector((*state).QTb, 0, p);
        let b1 = gsl_vector_subvector(b, 0, p);
        gsl_linalg_QR_QTvec_r(A, (*state).T, b, (*state).work);
        gsl_vector_memcpy(&QTb.vector, &b1.vector);

        (*state).rnorm = if n > p {
            let b2 = gsl_vector_subvector(b, p, n - p);
            gsl_blas_dnrm2(&b2.vector)
        } else {
            0.0
        };

        (*state).nblocks = 1;
        GSL_SUCCESS
    } else {
        let status = gsl_linalg_QR_UR_decomp((*state).R, A, (*state).T);
        if status != GSL_SUCCESS {
            return status;
        }

        gsl_vector_memcpy((*state).work