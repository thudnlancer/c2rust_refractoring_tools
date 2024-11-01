#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_mcholesky_decomp(
        A: *mut gsl_matrix,
        p: *mut gsl_permutation,
        E: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_mcholesky_solve(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_mcholesky_rcond(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        rcond: *mut libc::c_double,
        work: *mut gsl_vector,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
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
    pub owner: libc::c_int,
}
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
pub type gsl_multifit_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTIFIT_NLINEAR_CTRDIFF: gsl_multifit_nlinear_fdtype = 1;
pub const GSL_MULTIFIT_NLINEAR_FWDIFF: gsl_multifit_nlinear_fdtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_fdf {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub fvv: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trs {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(*const libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub preloop: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub step: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub preduction: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const gsl_vector,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
    pub update: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub presolve: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub solve: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut gsl_vector,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_parameters {
    pub trs: *const gsl_multifit_nlinear_trs,
    pub scale: *const gsl_multifit_nlinear_scale,
    pub solver: *const gsl_multifit_nlinear_solver,
    pub fdtype: gsl_multifit_nlinear_fdtype,
    pub factor_up: libc::c_double,
    pub factor_down: libc::c_double,
    pub avmax: libc::c_double,
    pub h_df: libc::c_double,
    pub h_fvv: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub J: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const libc::c_double,
    pub params: *const gsl_multifit_nlinear_parameters,
    pub solver_state: *mut libc::c_void,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub avratio: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcholesky_state_t {
    pub JTJ: *mut gsl_matrix,
    pub work_JTJ: *mut gsl_matrix,
    pub rhs: *mut gsl_vector,
    pub perm: *mut gsl_permutation,
    pub work3p: *mut gsl_vector,
    pub mu: libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
unsafe extern "C" fn mcholesky_alloc(n: size_t, p: size_t) -> *mut libc::c_void {
    let mut state: *mut mcholesky_state_t = 0 as *mut mcholesky_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<mcholesky_state_t>() as libc::c_ulong,
    ) as *mut mcholesky_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate mcholesky state\0" as *const u8 as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).JTJ = gsl_matrix_alloc(p, p);
    if ((*state).JTJ).is_null() {
        gsl_error(
            b"failed to allocate space for JTJ\0" as *const u8 as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).work_JTJ = gsl_matrix_alloc(p, p);
    if ((*state).work_JTJ).is_null() {
        gsl_error(
            b"failed to allocate space for JTJ workspace\0" as *const u8
                as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).rhs = gsl_vector_alloc(p);
    if ((*state).rhs).is_null() {
        gsl_error(
            b"failed to allocate space for rhs\0" as *const u8 as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).perm = gsl_permutation_alloc(p);
    if ((*state).perm).is_null() {
        gsl_error(
            b"failed to allocate space for perm\0" as *const u8 as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .work3p = gsl_vector_alloc((3 as libc::c_int as libc::c_ulong).wrapping_mul(p));
    if ((*state).work3p).is_null() {
        gsl_error(
            b"failed to allocate space for work3p\0" as *const u8 as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).mu = -1.0f64;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn mcholesky_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut mcholesky_state_t = vstate as *mut mcholesky_state_t;
    if !((*state).JTJ).is_null() {
        gsl_matrix_free((*state).JTJ);
    }
    if !((*state).work_JTJ).is_null() {
        gsl_matrix_free((*state).work_JTJ);
    }
    if !((*state).rhs).is_null() {
        gsl_vector_free((*state).rhs);
    }
    if !((*state).perm).is_null() {
        gsl_permutation_free((*state).perm);
    }
    if !((*state).work3p).is_null() {
        gsl_vector_free((*state).work3p);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn mcholesky_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut mcholesky_state_t = vstate as *mut mcholesky_state_t;
    gsl_blas_dsyrk(
        CblasLower,
        CblasTrans,
        1.0f64,
        (*trust_state).J,
        0.0f64,
        (*state).JTJ,
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn mcholesky_presolve(
    mu: libc::c_double,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut mcholesky_state_t = vstate as *mut mcholesky_state_t;
    let mut JTJ: *mut gsl_matrix = (*state).work_JTJ;
    let mut diag: *const gsl_vector = (*trust_state).diag;
    let mut status: libc::c_int = 0;
    gsl_matrix_tricpy(CblasLower, CblasNonUnit, JTJ, (*state).JTJ);
    status = mcholesky_regularize(mu, diag, JTJ, state);
    if status != 0 {
        return status;
    }
    status = gsl_linalg_mcholesky_decomp(JTJ, (*state).perm, 0 as *mut gsl_vector);
    if status != 0 {
        return status;
    }
    (*state).mu = mu;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn mcholesky_solve(
    mut f: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut mcholesky_state_t = vstate as *mut mcholesky_state_t;
    let mut status: libc::c_int = 0;
    gsl_blas_dgemv(CblasTrans, -1.0f64, (*trust_state).J, f, 0.0f64, (*state).rhs);
    status = mcholesky_solve_rhs((*state).rhs, x, state);
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn mcholesky_rcond(
    mut rcond: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut mcholesky_state_t = vstate as *mut mcholesky_state_t;
    let mut rcond_JTJ: libc::c_double = 0.;
    if (*state).mu != 0 as libc::c_int as libc::c_double {
        gsl_matrix_tricpy(CblasLower, CblasNonUnit, (*state).work_JTJ, (*state).JTJ);
        status = gsl_linalg_mcholesky_decomp(
            (*state).work_JTJ,
            (*state).perm,
            0 as *mut gsl_vector,
        );
        if status != 0 {
            return status;
        }
    }
    status = gsl_linalg_mcholesky_rcond(
        (*state).work_JTJ,
        (*state).perm,
        &mut rcond_JTJ,
        (*state).work3p,
    );
    if status == GSL_SUCCESS as libc::c_int {
        *rcond = sqrt(rcond_JTJ);
    }
    return status;
}
unsafe extern "C" fn mcholesky_solve_rhs(
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut state: *mut mcholesky_state_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut JTJ: *mut gsl_matrix = (*state).work_JTJ;
    status = gsl_linalg_mcholesky_solve(JTJ, (*state).perm, b, x);
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn mcholesky_regularize(
    mu: libc::c_double,
    mut diag: *const gsl_vector,
    mut A: *mut gsl_matrix,
    mut state: *mut mcholesky_state_t,
) -> libc::c_int {
    if mu != 0.0f64 {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*diag).size {
            let mut di: libc::c_double = gsl_vector_get(diag, i);
            let mut Aii: *mut libc::c_double = gsl_matrix_ptr(A, i, i);
            *Aii += mu * di * di;
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut mcholesky_type: gsl_multifit_nlinear_solver = unsafe {
    {
        let mut init = gsl_multifit_nlinear_solver {
            name: b"mcholesky\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                mcholesky_alloc
                    as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
            ),
            init: Some(
                mcholesky_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            presolve: Some(
                mcholesky_presolve
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            solve: Some(
                mcholesky_solve
                    as unsafe extern "C" fn(
                        *const gsl_vector,
                        *mut gsl_vector,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                mcholesky_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(mcholesky_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_multifit_nlinear_solver_mcholesky: *const gsl_multifit_nlinear_solver = unsafe {
    &mcholesky_type as *const gsl_multifit_nlinear_solver
};
