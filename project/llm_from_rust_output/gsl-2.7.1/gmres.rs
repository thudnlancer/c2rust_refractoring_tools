use libc::{c_double, c_int, c_ulong, c_void};
use std::ffi::CString;
use std::ptr;

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
pub struct gsl_spmatrix {
    pub size1: usize,
    pub size2: usize,
    pub i: *mut c_int,
    pub data: *mut c_double,
    pub p: *mut c_int,
    pub nzmax: usize,
    pub nz: usize,
    pub tree: *mut c_void,
    pub pool: *mut c_void,
    pub node_size: usize,
    pub work: *mut c_void,
    pub sptype: c_int,
    pub spflags: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmres_state_t {
    pub n: usize,
    pub m: usize,
    pub r: *mut gsl_vector,
    pub H: *mut gsl_matrix,
    pub tau: *mut gsl_vector,
    pub y: *mut gsl_vector,
    pub c: *mut c_double,
    pub s: *mut c_double,
    pub normr: c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_splinalg_itersolve_type {
    pub name: *const libc::c_char,
    pub alloc: Option<unsafe extern "C" fn(usize, usize) -> *mut c_void>,
    pub iterate: Option<
        unsafe extern "C" fn(
            *const gsl_spmatrix,
            *const gsl_vector,
            c_double,
            *mut gsl_vector,
            *mut c_void,
        ) -> c_int,
    >,
    pub normr: Option<unsafe extern "C" fn(*const c_void) -> c_double>,
    pub free: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
}

const GSL_SUCCESS: c_int = 0;
const GSL_CONTINUE: c_int = -2;
const GSL_EINVAL: c_int = 1;
const GSL_ENOMEM: c_int = 8;
const GSL_ENOTSQR: c_int = 20;
const GSL_EBADLEN: c_int = 19;

const CblasNoTrans: u32 = 111;
const CblasUpper: u32 = 121;
const CblasNonUnit: u32 = 131;

extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: c_int,
        gsl_errno: c_int,
    );
    fn gsl_vector_alloc(n: usize) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(v: *mut gsl_vector, i: usize, n: usize) -> gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_add(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_matrix_alloc(n1: usize, n2: usize) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: usize,
        j: usize,
        n1: usize,
        n2: usize,
    ) -> gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: usize) -> gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: usize,
        offset: usize,
        n: usize,
    ) -> gsl_vector_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> c_double;
    fn gsl_linalg_householder_hv(tau: c_double, v: *const gsl_vector, w: *mut gsl_vector) -> c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> c_double;
    fn gsl_blas_dtrsv(
        Uplo: u32,
        TransA: u32,
        Diag: u32,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> c_int;
    fn gsl_spblas_dgemv(
        TransA: u32,
        alpha: c_double,
        A: *const gsl_spmatrix,
        x: *const gsl_vector,
        beta: c_double,
        y: *mut gsl_vector,
    ) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sqrt(x: c_double) -> c_double;
    fn fabs(x: c_double) -> c_double;
}

unsafe fn gsl_vector_get(v: *const gsl_vector, i: usize) -> c_double {
    *(*v).data.add(i * (*v).stride)
}

unsafe fn gsl_vector_set(v: *mut gsl_vector, i: usize, x: c_double) {
    *(*v).data.add(i * (*v).stride) = x;
}

unsafe fn gsl_linalg_givens_gv(
    v: *mut gsl_vector,
    i: usize,
    j: usize,
    c: c_double,
    s: c_double,
) {
    let vi = gsl_vector_get(v, i);
    let vj = gsl_vector_get(v, j);
    gsl_vector_set(v, i, c * vi - s * vj);
    gsl_vector_set(v, j, s * vi + c * vj);
}

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

unsafe fn gmres_alloc(n: usize, m: usize) -> *mut c_void {
    if n == 0 {
        gsl_error(
            b"matrix dimension n must be a positive integer\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            82,
            GSL_EINVAL,
        );
        return ptr::null_mut();
    }

    let state = calloc(1, std::mem::size_of::<gmres_state_t>()) as *mut gmres_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate gmres state\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            88,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).n = n;
    (*state).m = if m == 0 {
        if n < 10 { n } else { 10 }
    } else {
        if n < m { n } else { m }
    };

    (*state).r = gsl_vector_alloc(n);
    if (*state).r.is_null() {
        gmres_free(state as *mut _);
        gsl_error(
            b"failed to allocate r vector\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            103,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).H = gsl_matrix_alloc(n, (*state).m + 1);
    if (*state).H.is_null() {
        gmres_free(state as *mut _);
        gsl_error(
            b"failed to allocate H matrix\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            110,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).tau = gsl_vector_alloc((*state).m + 1);
    if (*state).tau.is_null() {
        gmres_free(state as *mut _);
        gsl_error(
            b"failed to allocate tau vector\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            117,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).y = gsl_vector_alloc((*state).m + 1);
    if (*state).y.is_null() {
        gmres_free(state as *mut _);
        gsl_error(
            b"failed to allocate y vector\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            124,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).c = malloc((*state).m * std::mem::size_of::<c_double>()) as *mut c_double;
    (*state).s = malloc((*state).m * std::mem::size_of::<c_double>()) as *mut c_double;
    if (*state).c.is_null() || (*state).s.is_null() {
        gmres_free(state as *mut _);
        gsl_error(
            b"failed to allocate Givens vectors\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            132,
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*state).normr = 0.0;
    state as *mut c_void
}

unsafe fn gmres_free(vstate: *mut c_void) {
    let state = vstate as *mut gmres_state_t;
    if !(*state).r.is_null() {
        gsl_vector_free((*state).r);
    }
    if !(*state).H.is_null() {
        gsl_matrix_free((*state).H);
    }
    if !(*state).tau.is_null() {
        gsl_vector_free((*state).tau);
    }
    if !(*state).y.is_null() {
        gsl_vector_free((*state).y);
    }
    if !(*state).c.is_null() {
        free((*state).c as *mut _);
    }
    if !(*state).s.is_null() {
        free((*state).s as *mut _);
    }
    free(state as *mut _);
}

unsafe fn gmres_iterate(
    A: *const gsl_spmatrix,
    b: *const gsl_vector,
    tol: c_double,
    x: *mut gsl_vector,
    vstate: *mut c_void,
) -> c_int {
    let N = (*A).size1;
    let state = vstate as *mut gmres_state_t;

    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            204,
            GSL_ENOTSQR,
        );
        return GSL_ENOTSQR;
    } else if N != (*b).size {
        gsl_error(
            b"matrix does not match right hand side\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            208,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if N != (*x).size {
        gsl_error(
            b"matrix does not match solution vector\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            212,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if N != (*state).n {
        gsl_error(
            b"matrix does not match workspace\0".as_ptr() as *const _,
            b"gmres.c\0".as_ptr() as *const _,
            216,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    let mut status = GSL_SUCCESS;
    let maxit = (*state).m;
    let normb = gsl_blas_dnrm2(b);
    let reltol = tol * normb;
    let mut normr = 0.0;
    let mut m = 0;
    let mut k = 0;
    let mut tau = 0.0;

    let H = (*state).H;
    let r = (*state).r;
    let w = (*state).y;

    let mut h0 = gsl_matrix_column(H, 0);

    gsl_matrix_set_zero(H);
    gsl_vector_memcpy(r, b);
    gsl_spblas_dgemv(CblasNoTrans, -1.0, A, x, 1.0, r);
    gsl_vector_memcpy(&mut h0.vector, r);
    tau = gsl_linalg_householder_transform(&mut h0.vector);
    gsl_vector_set((*state).tau, 0, tau);
    gsl_vector_set_zero(w);
    gsl_vector_set(w, 0, gsl_vector_get(&mut h0.vector, 0));

    m = 1;
    while m <= maxit {
        let j = m - 1;
        let mut c = 0.0;
        let mut s = 0.0;

        let mut vm = gsl_matrix_column(H, m);
        let mut vv = gsl_vector_subvector(&mut vm.vector, j, N - j);
        let mut um = gsl_matrix_subcolumn(H, j, j, N - j);

        gsl_vector_set_zero(&mut vm.vector);
        gsl_vector_memcpy(&mut vv.vector, &mut um.vector);
        tau = gsl_vector_get((*state).tau, j);
        gsl_vector_scale(&mut vv.vector, -tau);
        gsl_vector_set(&mut vv.vector, 0, 1.0 - tau);

        k = j;
        while k > 0 {
            k -= 1;
            let mut uk = gsl_matrix_subcolumn(H, k, k, N - k);
            let mut vk = gsl_vector_subvector(&mut vm.vector, k, N - k);
            tau = gsl_vector_get((*state).tau, k);
            gsl_linalg_householder_hv(tau, &mut uk.vector, &mut vk.vector);
        }

        gsl_spblas_dgemv(CblasNoTrans, 1.0, A, &mut vm.vector, 0.0, r);
        gsl_vector_memcpy(&mut vm.vector, r);

        k = 0;
        while k <= j {
            let mut uk = gsl_matrix_subcolumn(H, k, k, N - k);
            let mut vk = gsl_vector_subvector(&mut vm.vector, k, N - k);
            tau = gsl_vector_get((*state).tau, k);
            gsl_linalg_householder_hv(tau, &mut uk.vector, &mut vk.vector);
            k += 1;
        }

        if m < N {
            let mut ump1 = gsl_matrix_subcolumn(H, m, m, N - m);
            tau = gsl_linalg_householder_transform(&mut ump1.vector);
            gsl_vector_set((*state).tau, j + 1, tau);
        }

        k = 0;
        while k < j {
            gsl_linalg_givens_gv(
                &mut vm.vector,
                k,
                k + 1,
                *(*state).c.add(k),
                *(*state).s.add(k),
            );
            k += 1;
        }

        if m < N {
            gsl_linalg_givens(
                gsl_vector_get(&mut vm.vector, j),
                gsl_vector_get(&mut vm.vector, j + 1),
                &mut c,
                &mut s,
            );
            *(*state).c.add(j) = c