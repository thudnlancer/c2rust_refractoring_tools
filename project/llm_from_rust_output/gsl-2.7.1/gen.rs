use libc::{c_double, c_int, c_ulong, c_void};
use std::f64;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_complex {
    pub dat: [c_double; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_view {
    pub vector: gsl_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_complex {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_complex {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_view {
    pub matrix: gsl_matrix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_eigen_gen_workspace {
    pub size: usize,
    pub work: *mut gsl_vector,
    pub n_evals: usize,
    pub max_iterations: usize,
    pub n_iter: usize,
    pub eshift: c_double,
    pub needtop: c_int,
    pub atol: c_double,
    pub btol: c_double,
    pub ascale: c_double,
    pub bscale: c_double,
    pub H: *mut gsl_matrix,
    pub R: *mut gsl_matrix,
    pub compute_s: c_int,
    pub compute_t: c_int,
    pub Q: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
}

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

extern "C" {
    fn calloc(n: c_ulong, size: c_ulong) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sqrt(x: c_double) -> c_double;
    fn hypot(x: c_double, y: c_double) -> c_double;
    fn fabs(x: c_double) -> c_double;
    fn floor(x: c_double) -> c_double;
    fn gsl_error(reason: *const libc::c_char, file: *const libc::c_char, line: c_int, gsl_errno: c_int);
    fn gsl_vector_alloc(n: usize) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_view_array(v: *mut c_double, n: usize) -> gsl_vector_view;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_schur_gen_eigvals(
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        wr1: *mut c_double,
        wr2: *mut c_double,
        wi: *mut c_double,
        scale1: *mut c_double,
        scale2: *mut c_double,
    ) -> c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: usize,
        j: usize,
        n1: usize,
        n2: usize,
    ) -> gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: usize) -> gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: usize) -> gsl_vector_view;
    fn gsl_matrix_subrow(m: *mut gsl_matrix, i: usize, offset: usize, n: usize) -> gsl_vector_view;
    fn gsl_matrix_subcolumn(m: *mut gsl_matrix, j: usize, offset: usize, n: usize) -> gsl_vector_view;
    fn gsl_matrix_view_array(base: *mut c_double, n1: usize, n2: usize) -> gsl_matrix_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> c_int;
    fn gsl_blas_drot(X: *mut gsl_vector, Y: *mut gsl_vector, c: c_double, s: c_double) -> c_int;
    fn gsl_linalg_hesstri_decomp(
        A: *mut gsl_matrix,
        B: *mut gsl_matrix,
        U: *mut gsl_matrix,
        V: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> c_int;
    fn gsl_linalg_householder_hm(tau: c_double, v: *const gsl_vector, A: *mut gsl_matrix) -> c_int;
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> c_double;
    fn gsl_linalg_householder_mh(tau: c_double, v: *const gsl_vector, A: *mut gsl_matrix) -> c_int;
    fn gsl_linalg_SV_decomp(
        A: *mut gsl_matrix,
        V: *mut gsl_matrix,
        S: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> c_int;
    fn gsl_hypot3(x: c_double, y: c_double, z: c_double) -> c_double;
}

#[inline]
unsafe fn gsl_vector_get(v: *const gsl_vector, i: usize) -> c_double {
    *(*v).data.add(i * (*v).stride)
}

#[inline]
unsafe fn gsl_vector_set(v: *mut gsl_vector, i: usize, x: c_double) {
    *(*v).data.add(i * (*v).stride) = x;
}

#[inline]
unsafe fn gsl_vector_complex_set(v: *mut gsl_vector_complex, i: usize, z: gsl_complex) {
    let ptr = (*(*v).data.add(2 * i * (*v).stride)) as *mut gsl_complex;
    *ptr = z;
}

#[inline]
unsafe fn gsl_matrix_get(m: *const gsl_matrix, i: usize, j: usize) -> c_double {
    *(*m).data.add(i * (*m).tda + j)
}

#[inline]
unsafe fn gsl_matrix_set(m: *mut gsl_matrix, i: usize, j: usize, x: c_double) {
    *(*m).data.add(i * (*m).tda + j) = x;
}

#[inline]
unsafe fn gsl_linalg_givens(a: c_double, b: c_double, c: *mut c_double, s: *mut c_double) {
    if b == 0.0 {
        *c = 1.0;
        *s = 0.0;
    } else if fabs(b) > fabs(a) {
        let t = -a / b;
        let s1 = 1.0 / sqrt(1.0 + t * t);
        *s = s1;
        *c = s1 * t;
    } else {
        let t = -b / a;
        let c1 = 1.0 / sqrt(1.0 + t * t);
        *c = c1;
        *s = c1 * t;
    }
}

pub unsafe fn gsl_eigen_gen_alloc(n: usize) -> *mut gsl_eigen_gen_workspace {
    if n == 0 {
        gsl_error(
            b"matrix dimension must be positive integer\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            117,
            GSL_EINVAL,
        );
        return ptr::null_mut();
    }

    let w = calloc(1, std::mem::size_of::<gsl_eigen_gen_workspace>() as c_ulong) as *mut gsl_eigen_gen_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            124,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*w).size = n;
    (*w).max_iterations = 30 * n;
    (*w).n_evals = 0;
    (*w).n_iter = 0;
    (*w).needtop = 0;
    (*w).atol = 0.0;
    (*w).btol = 0.0;
    (*w).ascale = 0.0;
    (*w).bscale = 0.0;
    (*w).eshift = 0.0;
    (*w).H = ptr::null_mut();
    (*w).R = ptr::null_mut();
    (*w).compute_s = 0;
    (*w).compute_t = 0;
    (*w).Q = ptr::null_mut();
    (*w).Z = ptr::null_mut();
    (*w).work = gsl_vector_alloc(n);

    if (*w).work.is_null() {
        gsl_eigen_gen_free(w);
        gsl_error(
            b"failed to allocate space for additional workspace\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            149,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    w
}

pub unsafe fn gsl_eigen_gen_free(w: *mut gsl_eigen_gen_workspace) {
    if w.is_null() {
        return;
    }
    if !(*w).work.is_null() {
        gsl_vector_free((*w).work);
    }
    free(w as *mut _);
}

pub unsafe fn gsl_eigen_gen_params(
    compute_s: c_int,
    compute_t: c_int,
    balance: c_int,
    w: *mut gsl_eigen_gen_workspace,
) {
    (*w).compute_s = compute_s;
    (*w).compute_t = compute_t;
}

pub unsafe fn gsl_eigen_gen(
    A: *mut gsl_matrix,
    B: *mut gsl_matrix,
    alpha: *mut gsl_vector_complex,
    beta: *mut gsl_vector,
    w: *mut gsl_eigen_gen_workspace,
) -> c_int {
    let N = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            219,
            GSL_ENOTSQR,
        );
        return GSL_ENOTSQR;
    } else if N != (*B).size1 || N != (*B).size2 {
        gsl_error(
            b"B matrix dimensions must match A\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            223,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if (*alpha).size != N || (*beta).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            227,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if (*w).size != N {
        gsl_error(
            b"matrix size does not match workspace\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            231,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else {
        let mut anorm = 0.0;
        let mut bnorm = 0.0;
        gsl_linalg_hesstri_decomp(A, B, (*w).Q, (*w).Z, (*w).work);
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
        anorm = normF(A);
        bnorm = normF(B);
        (*w).atol = f64::max(2.2250738585072014e-308, 2.2204460492503131e-16 * anorm);
        (*w).btol = f64::max(2.2250738585072014e-308, 2.2204460492503131e-16 * bnorm);
        (*w).ascale = 1.0 / f64::max(2.2250738585072014e-308, anorm);
        (*w).bscale = 1.0 / f64::max(2.2250738585072014e-308, bnorm);
        gen_schur_decomp(A, B, alpha, beta, w);
        if (*w).n_evals != N {
            return GSL_EMAXITER;
        }
        GSL_SUCCESS
    }
}

pub unsafe fn gsl_eigen_gen_QZ(
    A: *mut gsl_matrix,
    B: *mut gsl_matrix,
    alpha: *mut gsl_vector_complex,
    beta: *mut gsl_vector,
    Q: *mut gsl_matrix,
    Z: *mut gsl_matrix,
    w: *mut gsl_eigen_gen_workspace,
) -> c_int {
    if !Q.is_null() && ((*A).size1 != (*Q).size1 || (*A).size1 != (*Q).size2) {
        gsl_error(
            b"Q matrix has wrong dimensions\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            305,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if !Z.is_null() && ((*A).size1 != (*Z).size1 || (*A).size1 != (*Z).size2) {
        gsl_error(
            b"Z matrix has wrong dimensions\0".as_ptr() as *const _,
            b"gen.c\0".as_ptr() as *const _,
            309,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else {
        (*w).Q = Q;
        (*w).Z = Z;
        let s = gsl_eigen_gen(A, B, alpha, beta, w);
        (*w).Q = ptr::null_mut();
        (*w).Z = ptr::null_mut();
        s
    }
}

unsafe fn gen_schur_decomp(
    H: *mut gsl_matrix,
    R: *mut