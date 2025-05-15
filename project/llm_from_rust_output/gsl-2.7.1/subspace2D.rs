use libc::{c_double, c_int, c_ulong, c_void};
use std::ptr;

type size_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block {
    size: size_t,
    data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector {
    size: size_t,
    stride: size_t,
    data: *mut c_double,
    block: *mut gsl_block,
    owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_view {
    vector: gsl_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix {
    size1: size_t,
    size2: size_t,
    tda: size_t,
    data: *mut c_double,
    block: *mut gsl_block,
    owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_view {
    matrix: gsl_matrix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_permutation {
    size: size_t,
    data: *mut size_t,
}

#[repr(C)]
pub struct gsl_multifit_nlinear_fdf {
    f: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut gsl_vector) -> c_int>,
    df: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut gsl_matrix) -> c_int>,
    fvv: Option<
        unsafe extern "C" fn(
            *const gsl_vector,
            *const gsl_vector,
            *mut c_void,
            *mut gsl_vector,
        ) -> c_int,
    >,
    n: size_t,
    p: size_t,
    params: *mut c_void,
    nevalf: size_t,
    nevaldf: size_t,
    nevalfvv: size_t,
}

#[repr(C)]
pub struct gsl_multifit_nlinear_trs {
    name: *const libc::c_char,
    alloc: Option<unsafe extern "C" fn(*const c_void, size_t, size_t) -> *mut c_void>,
    init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    preloop: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    step: Option<
        unsafe extern "C" fn(*const c_void, c_double, *mut gsl_vector, *mut c_void) -> c_int,
    >,
    preduction: Option<
        unsafe extern "C" fn(*const c_void, *const gsl_vector, *mut c_double, *mut c_void) -> c_int,
    >,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct gsl_multifit_nlinear_parameters {
    trs: *const gsl_multifit_nlinear_trs,
    scale: *const gsl_multifit_nlinear_scale,
    solver: *const gsl_multifit_nlinear_solver,
    fdtype: gsl_multifit_nlinear_fdtype,
    factor_up: c_double,
    factor_down: c_double,
    avmax: c_double,
    h_df: c_double,
    h_fvv: c_double,
}

#[repr(C)]
pub struct gsl_multifit_nlinear_trust_state {
    x: *const gsl_vector,
    f: *const gsl_vector,
    g: *const gsl_vector,
    J: *const gsl_matrix,
    diag: *const gsl_vector,
    sqrt_wts: *const gsl_vector,
    mu: *const c_double,
    params: *const gsl_multifit_nlinear_parameters,
    solver_state: *mut c_void,
    fdf: *mut gsl_multifit_nlinear_fdf,
    avratio: *mut c_double,
}

#[repr(C)]
pub struct subspace2D_state_t {
    n: size_t,
    p: size_t,
    dx_gn: *mut gsl_vector,
    dx_sd: *mut gsl_vector,
    norm_Dgn: c_double,
    norm_Dsd: c_double,
    workp: *mut gsl_vector,
    workn: *mut gsl_vector,
    W: *mut gsl_matrix,
    JQ: *mut gsl_matrix,
    tau: *mut gsl_vector,
    subg: *mut gsl_vector,
    subB: *mut gsl_matrix,
    perm: *mut gsl_permutation,
    trB: c_double,
    detB: c_double,
    normg: c_double,
    term0: c_double,
    term1: c_double,
    rank: size_t,
    poly_p: *mut gsl_poly_complex_workspace,
    params: gsl_multifit_nlinear_parameters,
}

#[repr(C)]
pub struct gsl_poly_complex_workspace {
    nc: size_t,
    matrix: *mut c_double,
}

#[repr(u32)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

#[repr(u32)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}

const GSL_SUCCESS: c_int = 0;
const GSL_FAILURE: c_int = -1;
const GSL_ENOMEM: c_int = 8;

extern "C" {
    fn sqrt(x: c_double) -> c_double;
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
    fn gsl_vector_view_array(v: *mut c_double, n: size_t) -> gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> gsl_vector_view;
    fn gsl_matrix_view_array(base: *mut c_double, n1: size_t, n2: size_t) -> gsl_matrix_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> c_int;
    fn gsl_linalg_QR_QTvec(QR: *const gsl_matrix, tau: *const gsl_vector, v: *mut gsl_vector) -> c_int;
    fn gsl_linalg_QR_Qvec(QR: *const gsl_matrix, tau: *const gsl_vector, v: *mut gsl_vector) -> c_int;
    fn gsl_linalg_QR_matQ(QR: *const gsl_matrix, tau: *const gsl_vector, A: *mut gsl_matrix) -> c_int;
    fn gsl_linalg_QRPT_decomp(
        A: *mut gsl_matrix,
        tau: *mut gsl_vector,
        p: *mut gsl_permutation,
        signum: *mut c_int,
        norm: *mut gsl_vector,
    ) -> c_int;
    fn gsl_linalg_QRPT_rank(QR: *const gsl_matrix, tol: c_double) -> size_t;
    fn gsl_blas_ddot(X: *const gsl_vector, Y: *const gsl_vector, result: *mut c_double) -> c_int;
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        alpha: c_double,
        A: *const gsl_matrix,
        beta: c_double,
        C: *mut gsl_matrix,
    ) -> c_int;
    fn gsl_blas_dsymv(
        Uplo: CBLAS_UPLO,
        alpha: c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: c_double,
        Y: *mut gsl_vector,
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
    fn gsl_linalg_mcholesky_decomp(
        A: *mut gsl_matrix,
        p: *mut gsl_permutation,
        E: *mut gsl_vector,
    ) -> c_int;
    fn gsl_linalg_mcholesky_solve(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> c_int;
    fn gsl_poly_complex_workspace_alloc(n: size_t) -> *mut gsl_poly_complex_workspace;
    fn gsl_poly_complex_workspace_free(w: *mut gsl_poly_complex_workspace);
    fn gsl_poly_complex_solve(
        a: *const c_double,
        n: size_t,
        w: *mut gsl_poly_complex_workspace,
        z: *mut c_double,
    ) -> c_int;
}

unsafe fn gsl_vector_get(v: *const gsl_vector, i: size_t) -> c_double {
    *(*v).data.offset((i * (*v).stride) as isize)
}

unsafe fn gsl_vector_set(v: *mut gsl_vector, i: size_t, x: c_double) {
    *(*v).data.offset((i * (*v).stride) as isize) = x;
}

unsafe fn gsl_matrix_get(m: *const gsl_matrix, i: size_t, j: size_t) -> c_double {
    *(*m).data.offset((i * (*m).tda + j) as isize)
}

unsafe fn gsl_matrix_set(m: *mut gsl_matrix, i: size_t, j: size_t, x: c_double) {
    *(*m).data.offset((i * (*m).tda + j) as isize) = x;
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

    gsl_blas_dgemv(CblasNoTrans, 1.0 / normf, J, dx, 0.0, work);
    norm_beta = gsl_blas_dnrm2(work);
    pred_reduction = -norm_beta * norm_beta;

    for i in 0..n {
        let fi = gsl_vector_get(f, i);
        let betai = gsl_vector_get(work, i);
        pred_reduction -= 2.0 * (fi / normf) * betai;
    }

    pred_reduction
}

unsafe fn subspace2D_alloc(
    params: *const c_void,
    n: size_t,
    p: size_t,
) -> *mut c_void {
    let par = params as *const gsl_multifit_nlinear_parameters;
    let state = calloc(1, std::mem::size_of::<subspace2D_state_t>()) as *mut subspace2D_state_t;

    if state.is_null() {
        gsl_error(
            b"failed to allocate subspace2D state\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            165,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).dx_gn = gsl_vector_alloc(p);
    if (*state).dx_gn.is_null() {
        gsl_error(
            b"failed to allocate space for dx_gn\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            171,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).dx_sd = gsl_vector_alloc(p);
    if (*state).dx_sd.is_null() {
        gsl_error(
            b"failed to allocate space for dx_sd\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            177,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).workp = gsl_vector_alloc(p);
    if (*state).workp.is_null() {
        gsl_error(
            b"failed to allocate space for workp\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            183,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).workn = gsl_vector_alloc(n);
    if (*state).workn.is_null() {
        gsl_error(
            b"failed to allocate space for workn\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            189,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).W = gsl_matrix_alloc(p, 2);
    if (*state).W.is_null() {
        gsl_error(
            b"failed to allocate space for W\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            195,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).JQ = gsl_matrix_alloc(n, p);
    if (*state).JQ.is_null() {
        gsl_error(
            b"failed to allocate space for JQ\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            201,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).tau = gsl_vector_alloc(2);
    if (*state).tau.is_null() {
        gsl_error(
            b"failed to allocate space for tau\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            207,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).subg = gsl_vector_alloc(2);
    if (*state).subg.is_null() {
        gsl_error(
            b"failed to allocate space for subg\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            213,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).subB = gsl_matrix_alloc(2, 2);
    if (*state).subB.is_null() {
        gsl_error(
            b"failed to allocate space for subB\0".as_ptr() as *const _,
            b"subspace2D.c\0".as_ptr() as *const _,
            219,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).perm = gsl_permutation_alloc(2);
    if (*state).perm.is_null() {
        gsl_error(
            b"failed to allocate space for perm\0".as_ptr() as *const _,
