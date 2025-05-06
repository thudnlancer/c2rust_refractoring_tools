#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_eigen_francis_Z(
        H: *mut gsl_matrix,
        eval: *mut gsl_vector_complex,
        Z: *mut gsl_matrix,
        w: *mut gsl_eigen_francis_workspace,
    ) -> i32;
    fn gsl_eigen_francis(
        H: *mut gsl_matrix,
        eval: *mut gsl_vector_complex,
        w: *mut gsl_eigen_francis_workspace,
    ) -> i32;
    fn gsl_eigen_francis_T(compute_t: i32, w: *mut gsl_eigen_francis_workspace);
    fn gsl_eigen_francis_free(w: *mut gsl_eigen_francis_workspace);
    fn gsl_eigen_francis_alloc() -> *mut gsl_eigen_francis_workspace;
    fn gsl_linalg_hessenberg_unpack(
        H: *mut gsl_matrix,
        tau: *mut gsl_vector,
        U: *mut gsl_matrix,
    ) -> i32;
    fn gsl_linalg_hessenberg_decomp(A: *mut gsl_matrix, tau: *mut gsl_vector) -> i32;
    fn gsl_linalg_balance_matrix(A: *mut gsl_matrix, D: *mut gsl_vector) -> i32;
    fn gsl_linalg_balance_accum(A: *mut gsl_matrix, D: *mut gsl_vector) -> i32;
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_francis_workspace {
    pub size: size_t,
    pub max_iterations: size_t,
    pub n_iter: size_t,
    pub n_evals: size_t,
    pub compute_t: i32,
    pub H: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_nonsymm_workspace {
    pub size: size_t,
    pub diag: *mut gsl_vector,
    pub tau: *mut gsl_vector,
    pub Z: *mut gsl_matrix,
    pub do_balance: i32,
    pub n_evals: size_t,
    pub francis_workspace_p: *mut gsl_eigen_francis_workspace,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_eigen_nonsymm_alloc(
    n: size_t,
) -> *mut gsl_eigen_nonsymm_workspace {
    let mut w: *mut gsl_eigen_nonsymm_workspace = 0 as *mut gsl_eigen_nonsymm_workspace;
    if n == 0 as i32 as u64 {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            61 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_eigen_nonsymm_workspace;
    }
    w = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<gsl_eigen_nonsymm_workspace>() as u64,
    ) as *mut gsl_eigen_nonsymm_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            69 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_eigen_nonsymm_workspace;
    }
    (*w).size = n;
    (*w).Z = 0 as *mut gsl_matrix;
    (*w).do_balance = 0 as i32;
    (*w).diag = gsl_vector_alloc(n);
    if ((*w).diag).is_null() {
        gsl_eigen_nonsymm_free(w);
        gsl_error(
            b"failed to allocate space for balancing vector\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            81 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_eigen_nonsymm_workspace;
    }
    (*w).tau = gsl_vector_alloc(n);
    if ((*w).tau).is_null() {
        gsl_eigen_nonsymm_free(w);
        gsl_error(
            b"failed to allocate space for hessenberg coefficients\0" as *const u8
                as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            89 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_eigen_nonsymm_workspace;
    }
    (*w).francis_workspace_p = gsl_eigen_francis_alloc();
    if ((*w).francis_workspace_p).is_null() {
        gsl_eigen_nonsymm_free(w);
        gsl_error(
            b"failed to allocate space for francis workspace\0" as *const u8
                as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            97 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_eigen_nonsymm_workspace;
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_eigen_nonsymm_free(
    mut w: *mut gsl_eigen_nonsymm_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).tau).is_null() {
        gsl_vector_free((*w).tau);
    }
    if !((*w).diag).is_null() {
        gsl_vector_free((*w).diag);
    }
    if !((*w).francis_workspace_p).is_null() {
        gsl_eigen_francis_free((*w).francis_workspace_p);
    }
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_eigen_nonsymm_params(
    compute_t: i32,
    balance: i32,
    mut w: *mut gsl_eigen_nonsymm_workspace,
) {
    gsl_eigen_francis_T(compute_t, (*w).francis_workspace_p);
    (*w).do_balance = balance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_eigen_nonsymm(
    mut A: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut w: *mut gsl_eigen_nonsymm_workspace,
) -> i32 {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*eval).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            185 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut s: i32 = 0;
        if (*w).do_balance != 0 {
            gsl_linalg_balance_matrix(A, (*w).diag);
        }
        gsl_linalg_hessenberg_decomp(A, (*w).tau);
        if !((*w).Z).is_null() {
            gsl_linalg_hessenberg_unpack(A, (*w).tau, (*w).Z);
            s = gsl_eigen_francis_Z(A, eval, (*w).Z, (*w).francis_workspace_p);
            if (*w).do_balance != 0 {
                gsl_linalg_balance_accum((*w).Z, (*w).diag);
            }
        } else {
            s = gsl_eigen_francis(A, eval, (*w).francis_workspace_p);
        }
        (*w).n_evals = (*(*w).francis_workspace_p).n_evals;
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_eigen_nonsymm_Z(
    mut A: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut Z: *mut gsl_matrix,
    mut w: *mut gsl_eigen_nonsymm_workspace,
) -> i32 {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            271 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*eval).size != (*A).size1 {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            275 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*Z).size1 != (*Z).size2 || (*Z).size1 != (*A).size1 {
        gsl_error(
            b"Z matrix has wrong dimensions\0" as *const u8 as *const i8,
            b"nonsymm.c\0" as *const u8 as *const i8,
            279 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut s: i32 = 0;
        (*w).Z = Z;
        s = gsl_eigen_nonsymm(A, eval, w);
        (*w).Z = 0 as *mut gsl_matrix;
        return s;
    };
}