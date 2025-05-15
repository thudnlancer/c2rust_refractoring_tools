use std::ptr;
use std::mem;
use std::os::raw::c_void;
use libc::{free, calloc};
use gsl_sys::{
    gsl_blas_ddot, gsl_blas_dgemv, gsl_blas_dnrm2, gsl_blas_dsyrk, gsl_blas_dsymv,
    gsl_error, gsl_linalg_QR_decomp, gsl_linalg_QR_lssolve, gsl_linalg_QR_matQ,
    gsl_linalg_QR_QTvec, gsl_linalg_QR_Qvec, gsl_linalg_QRPT_decomp, gsl_linalg_QRPT_rank,
    gsl_linalg_QRPT_rcond, gsl_linalg_QRPT_solve, gsl_linalg_QRPT_svx, gsl_linalg_QR_solve,
    gsl_linalg_QR_svx, gsl_linalg_cholesky_decomp, gsl_linalg_cholesky_solve,
    gsl_linalg_cholesky_svx, gsl_linalg_mcholesky_decomp, gsl_linalg_mcholesky_solve,
    gsl_linalg_pcholesky_decomp, gsl_linalg_pcholesky_solve, gsl_linalg_pcholesky_svx,
    gsl_matrix, gsl_matrix_alloc, gsl_matrix_column, gsl_matrix_free, gsl_matrix_get,
    gsl_matrix_memcpy, gsl_matrix_set, gsl_matrix_submatrix, gsl_matrix_view,
    gsl_multifit_nlinear_parameters, gsl_multifit_nlinear_trs, gsl_multifit_nlinear_trust_state,
    gsl_permutation, gsl_permutation_alloc, gsl_permutation_free, gsl_poly_complex_solve,
    gsl_poly_complex_workspace, gsl_poly_complex_workspace_alloc, gsl_poly_complex_workspace_free,
    gsl_vector, gsl_vector_alloc, gsl_vector_div, gsl_vector_free, gsl_vector_get,
    gsl_vector_memcpy, gsl_vector_scale, gsl_vector_set, gsl_vector_set_zero, gsl_vector_view,
    CblasLower, CblasNoTrans, CblasTrans, GSL_ENOMEM, GSL_SUCCESS,
};

struct Subspace2DState {
    n: usize,
    p: usize,
    dx_gn: *mut gsl_vector,
    dx_sd: *mut gsl_vector,
    norm_Dgn: f64,
    norm_Dsd: f64,
    workp: *mut gsl_vector,
    workn: *mut gsl_vector,
    W: *mut gsl_matrix,
    JQ: *mut gsl_matrix,
    tau: *mut gsl_vector,
    subg: *mut gsl_vector,
    subB: *mut gsl_matrix,
    perm: *mut gsl_permutation,
    trB: f64,
    detB: f64,
    normg: f64,
    term0: f64,
    term1: f64,
    rank: usize,
    poly_p: *mut gsl_poly_complex_workspace,
    params: gsl_multifit_nlinear_parameters,
}

extern "C" fn subspace2D_alloc(
    params: *const c_void,
    n: usize,
    p: usize,
) -> *mut c_void {
    let par = params as *const gsl_multifit_nlinear_parameters;
    let state = unsafe { calloc(1, mem::size_of::<Subspace2DState>()) } as *mut Subspace2DState;

    if state.is_null() {
        unsafe {
            gsl_error(
                b"failed to allocate subspace2D state\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                42,
                GSL_ENOMEM,
            );
        }
        return ptr::null_mut();
    }

    unsafe {
        (*state).dx_gn = gsl_vector_alloc(p);
        if (*state).dx_gn.is_null() {
            gsl_error(
                b"failed to allocate space for dx_gn\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                47,
                GSL_ENOMEM,
            );
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).dx_sd = gsl_vector_alloc(p);
        if (*state).dx_sd.is_null() {
            gsl_error(
                b"failed to allocate space for dx_sd\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                53,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).workp = gsl_vector_alloc(p);
        if (*state).workp.is_null() {
            gsl_error(
                b"failed to allocate space for workp\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                59,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).workn = gsl_vector_alloc(n);
        if (*state).workn.is_null() {
            gsl_error(
                b"failed to allocate space for workn\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                65,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).W = gsl_matrix_alloc(p, 2);
        if (*state).W.is_null() {
            gsl_error(
                b"failed to allocate space for W\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                71,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).JQ = gsl_matrix_alloc(n, p);
        if (*state).JQ.is_null() {
            gsl_error(
                b"failed to allocate space for JQ\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                77,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            gsl_matrix_free((*state).W);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).tau = gsl_vector_alloc(2);
        if (*state).tau.is_null() {
            gsl_error(
                b"failed to allocate space for tau\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                83,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            gsl_matrix_free((*state).W);
            gsl_matrix_free((*state).JQ);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).subg = gsl_vector_alloc(2);
        if (*state).subg.is_null() {
            gsl_error(
                b"failed to allocate space for subg\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                89,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            gsl_matrix_free((*state).W);
            gsl_matrix_free((*state).JQ);
            gsl_vector_free((*state).tau);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).subB = gsl_matrix_alloc(2, 2);
        if (*state).subB.is_null() {
            gsl_error(
                b"failed to allocate space for subB\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                95,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            gsl_matrix_free((*state).W);
            gsl_matrix_free((*state).JQ);
            gsl_vector_free((*state).tau);
            gsl_vector_free((*state).subg);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).perm = gsl_permutation_alloc(2);
        if (*state).perm.is_null() {
            gsl_error(
                b"failed to allocate space for perm\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                101,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            gsl_matrix_free((*state).W);
            gsl_matrix_free((*state).JQ);
            gsl_vector_free((*state).tau);
            gsl_vector_free((*state).subg);
            gsl_matrix_free((*state).subB);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).poly_p = gsl_poly_complex_workspace_alloc(5);
        if (*state).poly_p.is_null() {
            gsl_error(
                b"failed to allocate space for poly workspace\0".as_ptr() as *const _,
                b"subspace2D.c\0".as_ptr() as *const _,
                107,
                GSL_ENOMEM,
            );
            gsl_vector_free((*state).dx_gn);
            gsl_vector_free((*state).dx_sd);
            gsl_vector_free((*state).workp);
            gsl_vector_free((*state).workn);
            gsl_matrix_free((*state).W);
            gsl_matrix_free((*state).JQ);
            gsl_vector_free((*state).tau);
            gsl_vector_free((*state).subg);
            gsl_matrix_free((*state).subB);
            gsl_permutation_free((*state).perm);
            free(state as *mut _);
            return ptr::null_mut();
        }

        (*state).n = n;
        (*state).p = p;
        (*state).rank = 0;
        (*state).params = *par;
    }

    state as *mut c_void
}

extern "C" fn subspace2D_free(vstate: *mut c_void) {
    let state = vstate as *mut Subspace2DState;

    unsafe {
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

        if !(*state).W.is_null() {
            gsl_matrix_free((*state).W);
        }

        if !(*state).JQ.is_null() {
            gsl_matrix_free((*state).JQ);
        }

        if !(*state).tau.is_null() {
            gsl_vector_free((*state).tau);
        }

        if !(*state).subg.is_null() {
            gsl_vector_free((*state).subg);
        }

        if !(*state).subB.is_null() {
            gsl_matrix_free((*state).subB);
        }

        if !(*state).perm.is_null() {
            gsl_permutation_free((*state).perm);
        }

        if !(*state).poly_p.is_null() {
            gsl_poly_complex_workspace_free((*state).poly_p);
        }

        free(state as *mut _);
    }
}

extern "C" fn subspace2D_init(
    _vtrust_state: *const c_void,
    _vstate: *mut c_void,
) -> i32 {
    GSL_SUCCESS
}

extern "C" fn subspace2D_preloop(
    vtrust_state: *const c_void,
    vstate: *mut c_void,
) -> i32 {
    let trust_state = vtrust_state as *const gsl_multifit_nlinear_trust_state;
    let state = vstate as *mut Subspace2DState;
    let mut status = GSL_SUCCESS;
    let mut signum = 0;

    unsafe {
        status = subspace2D_calc_gn(trust_state, (*state).dx_gn);
        if status != GSL_SUCCESS {
            return status;
        }

        status = subspace2D_calc_sd(trust_state, (*state).dx_sd, state);
        if status != GSL_SUCCESS {
            return status;
        }

        (*state).norm_Dgn = scaled_enorm((*trust_state).diag, (*state).dx_gn);
        (*state).norm_Dsd = scaled_enorm((*trust_state).diag, (*state).dx_sd);

        let mut v = gsl_matrix_column((*state).W, 0);
        gsl_vector_memcpy(&mut v.vector, (*state).dx_sd);
        gsl_vector_mul(&mut v.vector, (*trust_state).diag);
        if (*state).norm_Dsd != 0.0 {
            gsl_vector_scale(&mut v.vector, 1.0 / (*state).norm_Dsd);
        }

        v = gsl_matrix_column((*state).W, 1);
        gsl_vector_memcpy(&mut v.vector, (*state).dx_gn);
        gsl_vector_mul(&mut v.vector, (*trust_state).diag);
        if (*state).norm_Dgn != 0.0 {
            gsl_vector_scale(&mut v.vector, 1.0 / (*state).norm_Dgn);
        }

        let mut work_data = [0.0; 2];
        let mut work = gsl_vector_view_array(&mut work_data, 2);

        gsl_linalg_QRPT_decomp(
            (*state).W,
            (*state).tau,
            (*state).perm,
            &mut signum,
            &mut work.vector,
        );

        (*state).rank = gsl_linalg_QRPT_rank((*state).W, -1.0);

        if (*state).rank == 2 {
            let p = (*state).p;
            let mut JQ = gsl_matrix_submatrix(
                (*state).JQ,
                0,
                0,
                (*state).n,
                if 2 < p { 2 } else { p },
            );

            gsl_vector_memcpy((*state).workp, (*trust_state).g);
            gsl_vector_div((*state).workp, (*trust_state).diag);
            gsl_linalg_QR_QTvec((*state).W, (*state).tau, (*state).workp);

            let g0 = gsl_vector_get((*state).workp, 0);
            let g1 = gsl_vector_get((*state).workp, 1);

            gsl_vector_set((*state).subg, 0, g0);
            gsl_vector_set((*state).subg, 1, g1);

            gsl_matrix_memcpy((*state).JQ, (*trust_state).J);

            for i in 0..p {
                let mut c = gsl_matrix_column((*state).JQ, i);
                let di = gsl_vector_get((*trust_state).diag, i);
                gsl_vector_scale(&mut c.vector, 1.0 / di);
            }

            gsl_linalg_QR_matQ((*state).W, (*state).tau, (*state).JQ);

            gsl_blas_dsyrk(
                CblasLower,
                CblasTrans,
                1.0,
                &JQ.matrix,
                0.0,
                (*state).subB,
            );

            let B00 = gsl_matrix_get((*state).subB, 0, 0);
            let B10 = gsl_matrix_get((*state).subB, 1, 0);
            let B11 = gsl_matrix_get((*state).subB, 1, 1);

            (*state).trB