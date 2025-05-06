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
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> i32;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> i32;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> i32;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_multifit_linear_applyW(
        X: *const gsl_matrix,
        w: *const gsl_vector,
        y: *const gsl_vector,
        WX: *mut gsl_matrix,
        Wy: *mut gsl_vector,
    ) -> i32;
    fn gsl_multifit_linear_L_decomp(L: *mut gsl_matrix, tau: *mut gsl_vector) -> i32;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> i32;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
pub type CBLAS_TRANSPOSE = u32;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = u32;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = u32;
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
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_const_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_const_view = _gsl_matrix_const_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_linear_type {
    pub name: *const i8,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub reset: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub accumulate: Option<
        unsafe extern "C" fn(*mut gsl_matrix, *mut gsl_vector, *mut libc::c_void) -> i32,
    >,
    pub solve: Option<
        unsafe extern "C" fn(
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub rcond: Option<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> i32,
    >,
    pub lcurve: Option<
        unsafe extern "C" fn(
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub matrix_ptr: Option<
        unsafe extern "C" fn(*const libc::c_void) -> *const gsl_matrix,
    >,
    pub rhs_ptr: Option<unsafe extern "C" fn(*const libc::c_void) -> *const gsl_vector>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_linear_workspace {
    pub type_0: *const gsl_multilarge_linear_type,
    pub state: *mut libc::c_void,
    pub p: size_t,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_alloc(
    mut T: *const gsl_multilarge_linear_type,
    p: size_t,
) -> *mut gsl_multilarge_linear_workspace {
    let mut w: *mut gsl_multilarge_linear_workspace = 0
        as *mut gsl_multilarge_linear_workspace;
    w = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<gsl_multilarge_linear_workspace>() as u64,
    ) as *mut gsl_multilarge_linear_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            39 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multilarge_linear_workspace;
    }
    (*w).type_0 = T;
    (*w).state = ((*(*w).type_0).alloc).expect("non-null function pointer")(p);
    if ((*w).state).is_null() {
        gsl_multilarge_linear_free(w);
        gsl_error(
            b"failed to allocate space for multilarge state\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            49 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multilarge_linear_workspace;
    }
    (*w).p = p;
    gsl_multilarge_linear_reset(w);
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_free(
    mut w: *mut gsl_multilarge_linear_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).state).is_null() {
        ((*(*w).type_0).free).expect("non-null function pointer")((*w).state);
    }
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_name(
    mut w: *const gsl_multilarge_linear_workspace,
) -> *const i8 {
    return (*(*w).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_reset(
    mut w: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let mut status: i32 = ((*(*w).type_0).reset)
        .expect("non-null function pointer")((*w).state);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_accumulate(
    mut X: *mut gsl_matrix,
    mut y: *mut gsl_vector,
    mut w: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let mut status: i32 = ((*(*w).type_0).accumulate)
        .expect("non-null function pointer")(X, y, (*w).state);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_solve(
    lambda: libc::c_double,
    mut c: *mut gsl_vector,
    mut rnorm: *mut libc::c_double,
    mut snorm: *mut libc::c_double,
    mut w: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let mut status: i32 = ((*(*w).type_0).solve)
        .expect("non-null function pointer")(lambda, c, rnorm, snorm, (*w).state);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_rcond(
    mut rcond: *mut libc::c_double,
    mut w: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let mut status: i32 = ((*(*w).type_0).rcond)
        .expect("non-null function pointer")(rcond, (*w).state);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_lcurve(
    mut reg_param: *mut gsl_vector,
    mut rho: *mut gsl_vector,
    mut eta: *mut gsl_vector,
    mut w: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let len: size_t = (*reg_param).size;
    if len != (*rho).size {
        gsl_error(
            b"reg_param and rho have different sizes\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            117 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if len != (*eta).size {
        gsl_error(
            b"reg_param and eta have different sizes\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            121 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut status: i32 = ((*(*w).type_0).lcurve)
            .expect("non-null function pointer")(reg_param, rho, eta, (*w).state);
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_wstdform1(
    mut L: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut work: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if !L.is_null() && p != (*L).size {
        gsl_error(
            b"L vector does not match X\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            170 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if n != (*y).size {
        gsl_error(
            b"y vector does not match X\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            174 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if !w.is_null() && n != (*w).size {
        gsl_error(
            b"weight vector does not match X\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            178 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if n != (*Xs).size1 || p != (*Xs).size2 {
        gsl_error(
            b"Xs matrix dimensions do not match X\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            182 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if n != (*ys).size {
        gsl_error(
            b"ys vector must be length n\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            186 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut status: i32 = GSL_SUCCESS as i32;
        status = gsl_multifit_linear_applyW(X, w, y, Xs, ys);
        if status != 0 {
            return status;
        }
        if !L.is_null() {
            let mut j: size_t = 0;
            j = 0 as i32 as size_t;
            while j < p {
                let mut Xj: gsl_vector_view = gsl_matrix_column(Xs, j);
                let mut lj: libc::c_double = gsl_vector_get(L, j);
                if lj == 0.0f64 {
                    gsl_error(
                        b"L matrix is singular\0" as *const u8 as *const i8,
                        b"multilarge.c\0" as *const u8 as *const i8,
                        209 as i32,
                        GSL_EDOM as i32,
                    );
                    return GSL_EDOM as i32;
                }
                gsl_vector_scale(&mut Xj.vector, 1.0f64 / lj);
                j = j.wrapping_add(1);
                j;
            }
        }
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_stdform1(
    mut L: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut work: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let mut status: i32 = 0;
    status = gsl_multilarge_linear_wstdform1(
        L,
        X,
        0 as *const gsl_vector,
        y,
        Xs,
        ys,
        work,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_L_decomp(
    mut L: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> i32 {
    let m: size_t = (*L).size1;
    let p: size_t = (*L).size2;
    if m < p {
        gsl_error(
            b"m < p not yet supported\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            243 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut status: i32 = 0;
        status = gsl_multifit_linear_L_decomp(L, tau);
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_wstdform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut work: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let m: size_t = (*LQR).size1;
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if p != (*work).p {
        gsl_error(
            b"X has wrong number of columns\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            273 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if p != (*LQR).size2 {
        gsl_error(
            b"LQR and X matrices have different numbers of columns\0" as *const u8
                as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            277 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if n != (*y).size {
        gsl_error(
            b"y vector does not match X\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            281 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if !w.is_null() && n != (*w).size {
        gsl_error(
            b"weights vector must be length n\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            285 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if m < p {
        gsl_error(
            b"m < p not yet supported\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            289 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if n != (*Xs).size1 || p != (*Xs).size2 {
        gsl_error(
            b"Xs matrix must be n-by-p\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            293 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if n != (*ys).size {
        gsl_error(
            b"ys vector must have length n\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            297 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut status: i32 = 0;
        let mut i: size_t = 0;
        let R: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            LQR,
            0 as i32 as size_t,
            0 as i32 as size_t,
            p,
            p,
        );
        status = gsl_multifit_linear_applyW(X, w, y, Xs, ys);
        if status != 0 {
            return status;
        }
        i = 0 as i32 as size_t;
        while i < n {
            let mut v: gsl_vector_view = gsl_matrix_row(Xs, i);
            gsl_blas_dtrsv(
                CblasUpper,
                CblasTrans,
                CblasNonUnit,
                &R.matrix,
                &mut v.vector,
            );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_stdform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut work: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let mut status: i32 = 0;
    status = gsl_multilarge_linear_wstdform2(
        LQR,
        Ltau,
        X,
        0 as *const gsl_vector,
        y,
        Xs,
        ys,
        work,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_genform1(
    mut L: *const gsl_vector,
    mut cs: *const gsl_vector,
    mut c: *mut gsl_vector,
    mut work: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    if (*L).size != (*work).p {
        gsl_error(
            b"L vector does not match workspace\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            353 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*L).size != (*cs).size {
        gsl_error(
            b"cs vector does not match L\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            357 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*L).size != (*c).size {
        gsl_error(
            b"c vector does not match L\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            361 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        gsl_vector_memcpy(c, cs);
        gsl_vector_div(c, L);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_genform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut cs: *const gsl_vector,
    mut c: *mut gsl_vector,
    mut work: *mut gsl_multilarge_linear_workspace,
) -> i32 {
    let m: size_t = (*LQR).size1;
    let p: size_t = (*LQR).size2;
    if p != (*c).size {
        gsl_error(
            b"c vector does not match LQR\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            388 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if m < p {
        gsl_error(
            b"m < p not yet supported\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            392 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if p != (*cs).size {
        gsl_error(
            b"cs vector size does not match c\0" as *const u8 as *const i8,
            b"multilarge.c\0" as *const u8 as *const i8,
            396 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut s: i32 = 0;
        let R: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            LQR,
            0 as i32 as size_t,
            0 as i32 as size_t,
            p,
            p,
        );
        gsl_vector_memcpy(c, cs);
        s = gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, &R.matrix, c);
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_matrix_ptr(
    mut work: *const gsl_multilarge_linear_workspace,
) -> *const gsl_matrix {
    return ((*(*work).type_0).matrix_ptr)
        .expect("non-null function pointer")((*work).state);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multilarge_linear_rhs_ptr(
    mut work: *const gsl_multilarge_linear_workspace,
) -> *const gsl_vector {
    return ((*(*work).type_0).rhs_ptr)
        .expect("non-null function pointer")((*work).state);
}