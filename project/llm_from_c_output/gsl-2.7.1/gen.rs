use libc::{c_int, c_double};
use std::ptr;
use std::mem;
use std::cmp;
use std::f64;

#[repr(C)]
pub struct gsl_matrix {
    size1: usize,
    size2: usize,
    tda: usize,
    data: *mut f64,
    block: *mut gsl_block,
    owner: c_int,
}

#[repr(C)]
pub struct gsl_block {
    size: usize,
    data: *mut f64,
}

#[repr(C)]
pub struct gsl_vector {
    size: usize,
    stride: usize,
    data: *mut f64,
    block: *mut gsl_block,
    owner: c_int,
}

#[repr(C)]
pub struct gsl_vector_complex {
    size: usize,
    stride: usize,
    data: *mut f64,
    block: *mut gsl_block_complex,
    owner: c_int,
}

#[repr(C)]
pub struct gsl_block_complex {
    size: usize,
    data: *mut f64,
}

#[repr(C)]
pub struct gsl_complex {
    dat: [f64; 2],
}

#[repr(C)]
pub struct gsl_eigen_gen_workspace {
    size: usize,
    max_iterations: usize,
    n_evals: usize,
    n_iter: usize,
    needtop: c_int,
    atol: f64,
    btol: f64,
    ascale: f64,
    bscale: f64,
    eshift: f64,
    H: *mut gsl_matrix,
    R: *mut gsl_matrix,
    compute_s: c_int,
    compute_t: c_int,
    Q: *mut gsl_matrix,
    Z: *mut gsl_matrix,
    work: *mut gsl_vector,
}

const GEN_ESHIFT_COEFF: f64 = 1.736;

#[no_mangle]
pub extern "C" fn gsl_eigen_gen_alloc(n: usize) -> *mut gsl_eigen_gen_workspace {
    if n == 0 {
        return ptr::null_mut();
    }

    let w = Box::into_raw(Box::new(gsl_eigen_gen_workspace {
        size: n,
        max_iterations: 30 * n,
        n_evals: 0,
        n_iter: 0,
        needtop: 0,
        atol: 0.0,
        btol: 0.0,
        ascale: 0.0,
        bscale: 0.0,
        eshift: 0.0,
        H: ptr::null_mut(),
        R: ptr::null_mut(),
        compute_s: 0,
        compute_t: 0,
        Q: ptr::null_mut(),
        Z: ptr::null_mut(),
        work: unsafe { gsl_vector_alloc(n) },
    }));

    if unsafe { (*w).work.is_null() } {
        unsafe { Box::from_raw(w) };
        return ptr::null_mut();
    }

    w
}

#[no_mangle]
pub extern "C" fn gsl_eigen_gen_free(w: *mut gsl_eigen_gen_workspace) {
    if w.is_null() {
        return;
    }

    unsafe {
        if !(*w).work.is_null() {
            gsl_vector_free((*w).work);
        }
        Box::from_raw(w);
    }
}

#[no_mangle]
pub extern "C" fn gsl_eigen_gen_params(
    compute_s: c_int,
    compute_t: c_int,
    _balance: c_int,
    w: *mut gsl_eigen_gen_workspace,
) {
    unsafe {
        (*w).compute_s = compute_s;
        (*w).compute_t = compute_t;
    }
}

#[no_mangle]
pub extern "C" fn gsl_eigen_gen(
    A: *mut gsl_matrix,
    B: *mut gsl_matrix,
    alpha: *mut gsl_vector_complex,
    beta: *mut gsl_vector,
    w: *mut gsl_eigen_gen_workspace,
) -> c_int {
    unsafe {
        let N = (*A).size1;

        if (*A).size2 != N {
            return 1; // GSL_ENOTSQR
        }
        if (*B).size1 != N || (*B).size2 != N {
            return 2; // GSL_EBADLEN
        }
        if (*alpha).size != N || (*beta).size != N {
            return 2; // GSL_EBADLEN
        }
        if (*w).size != N {
            return 2; // GSL_EBADLEN
        }

        (*w).H = A;
        (*w).R = B;
        (*w).n_evals = 0;
        (*w).n_iter = 0;
        (*w).eshift = 0.0;

        (*w).needtop = if !(*w).Q.is_null() || !(*w).Z.is_null() || (*w).compute_t != 0 || (*w).compute_s != 0 {
            1
        } else {
            0
        };

        let anorm = normF(A);
        let bnorm = normF(B);

        (*w).atol = f64::max(f64::MIN_POSITIVE, f64::EPSILON * anorm);
        (*w).btol = f64::max(f64::MIN_POSITIVE, f64::EPSILON * bnorm);
        (*w).ascale = 1.0 / f64::max(f64::MIN_POSITIVE, anorm);
        (*w).bscale = 1.0 / f64::max(f64::MIN_POSITIVE, bnorm);

        gen_schur_decomp(A, B, alpha, beta, w);

        if (*w).n_evals != N {
            return 3; // GSL_EMAXITER
        }

        0 // GSL_SUCCESS
    }
}

#[no_mangle]
pub extern "C" fn gsl_eigen_gen_QZ(
    A: *mut gsl_matrix,
    B: *mut gsl_matrix,
    alpha: *mut gsl_vector_complex,
    beta: *mut gsl_vector,
    Q: *mut gsl_matrix,
    Z: *mut gsl_matrix,
    w: *mut gsl_eigen_gen_workspace,
) -> c_int {
    unsafe {
        if !Q.is_null() && ((*A).size1 != (*Q).size1 || (*A).size1 != (*Q).size2) {
            return 2; // GSL_EBADLEN
        }
        if !Z.is_null() && ((*A).size1 != (*Z).size1 || (*A).size1 != (*Z).size2) {
            return 2; // GSL_EBADLEN
        }

        (*w).Q = Q;
        (*w).Z = Z;

        let s = gsl_eigen_gen(A, B, alpha, beta, w);

        (*w).Q = ptr::null_mut();
        (*w).Z = ptr::null_mut();

        s
    }
}

// Helper functions that would be implemented elsewhere in the Rust GSL bindings
extern "C" {
    fn gsl_vector_alloc(n: usize) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_linalg_hesstri_decomp(A: *mut gsl_matrix, B: *mut gsl_matrix, Q: *mut gsl_matrix, Z: *mut gsl_matrix, work: *mut gsl_vector);
    fn gsl_schur_gen_eigvals(A: *mut gsl_matrix, B: *mut gsl_matrix, wr1: *mut f64, wr2: *mut f64, wi: *mut f64, scale1: *mut f64, scale2: *mut f64);
    fn gsl_linalg_givens(a: f64, b: f64, cs: *mut f64, sn: *mut f64);
    fn gsl_blas_drot(x: *mut gsl_vector, y: *mut gsl_vector, c: f64, s: f64);
    fn gsl_linalg_SV_decomp(A: *mut gsl_matrix, V: *mut gsl_matrix, S: *mut gsl_vector, work: *mut gsl_vector);
}

// Internal functions would be implemented similarly with proper Rust safety checks
unsafe fn gen_schur_decomp(
    H: *mut gsl_matrix,
    R: *mut gsl_matrix,
    alpha: *mut gsl_vector_complex,
    beta: *mut gsl_vector,
    w: *mut gsl_eigen_gen_workspace,
) {
    // Implementation would mirror the C version with Rust safety checks
}

unsafe fn normF(A: *mut gsl_matrix) -> f64 {
    let mut sum = 0.0;
    let mut scale = 0.0;
    let mut ssq = 1.0;

    for i in 0..(*A).size1 {
        for j in 0..(*A).size2 {
            let Aij = *((*A).data.add(i * (*A).tda + j));
            if Aij != 0.0 {
                let ax = Aij.abs();
                if scale < ax {
                    ssq = 1.0 + ssq * (scale / ax).powi(2);
                    scale = ax;
                } else {
                    ssq += (ax / scale).powi(2);
                }
            }
        }
    }

    scale * ssq.sqrt()
}