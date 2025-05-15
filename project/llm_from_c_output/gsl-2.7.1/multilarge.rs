use std::ptr;
use std::ffi::CStr;
use libc::{c_int, c_double, size_t};
use gsl_sys::{gsl_vector, gsl_matrix, gsl_multifit_linear_workspace, gsl_multilarge_linear_workspace, gsl_multilarge_linear_type};

#[repr(C)]
pub struct gsl_multilarge_linear_workspace {
    type_: *const gsl_multilarge_linear_type,
    state: *mut c_void,
    p: size_t,
}

#[repr(C)]
pub struct gsl_multilarge_linear_type {
    name: *const c_char,
    alloc: extern "C" fn(size_t) -> *mut c_void,
    free: extern "C" fn(*mut c_void),
    reset: extern "C" fn(*mut c_void) -> c_int,
    accumulate: extern "C" fn(*const gsl_matrix, *const gsl_vector, *mut c_void) -> c_int,
    solve: extern "C" fn(c_double, *mut gsl_vector, *mut c_double, *mut c_double, *mut c_void) -> c_int,
    rcond: extern "C" fn(*mut c_double, *mut c_void) -> c_int,
    lcurve: extern "C" fn(*const gsl_vector, *const gsl_vector, *const gsl_vector, *mut c_void) -> c_int,
    matrix_ptr: extern "C" fn(*mut c_void) -> *const gsl_matrix,
    rhs_ptr: extern "C" fn(*mut c_void) -> *const gsl_vector,
}

pub unsafe fn gsl_multilarge_linear_alloc(
    T: *const gsl_multilarge_linear_type,
    p: size_t,
) -> *mut gsl_multilarge_linear_workspace {
    let w = libc::calloc(1, std::mem::size_of::<gsl_multilarge_linear_workspace>()) as *mut gsl_multilarge_linear_workspace;
    if w.is_null() {
        return ptr::null_mut();
    }

    (*w).type_ = T;
    (*w).state = ((*T).alloc)(p);
    if (*w).state.is_null() {
        gsl_multilarge_linear_free(w);
        return ptr::null_mut();
    }

    (*w).p = p;
    gsl_multilarge_linear_reset(w);

    w
}

pub unsafe fn gsl_multilarge_linear_free(w: *mut gsl_multilarge_linear_workspace) {
    if w.is_null() {
        return;
    }

    if !(*w).state.is_null() {
        ((*(*w).type_).free)((*w).state);
    }

    libc::free(w as *mut c_void);
}

pub unsafe fn gsl_multilarge_linear_name(w: *const gsl_multilarge_linear_workspace) -> *const c_char {
    (*(*w).type_).name
}

pub unsafe fn gsl_multilarge_linear_reset(w: *mut gsl_multilarge_linear_workspace) -> c_int {
    ((*(*w).type_).reset)((*w).state)
}

pub unsafe fn gsl_multilarge_linear_accumulate(
    X: *const gsl_matrix,
    y: *const gsl_vector,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    ((*(*w).type_).accumulate)(X, y, (*w).state)
}

pub unsafe fn gsl_multilarge_linear_solve(
    lambda: c_double,
    c: *mut gsl_vector,
    rnorm: *mut c_double,
    snorm: *mut c_double,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    ((*(*w).type_).solve)(lambda, c, rnorm, snorm, (*w).state)
}

pub unsafe fn gsl_multilarge_linear_rcond(
    rcond: *mut c_double,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    ((*(*w).type_).rcond)(rcond, (*w).state)
}

pub unsafe fn gsl_multilarge_linear_lcurve(
    reg_param: *const gsl_vector,
    rho: *const gsl_vector,
    eta: *const gsl_vector,
    w: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    let len = (*reg_param).size;
    if len != (*rho).size {
        return GSL_EBADLEN;
    } else if len != (*eta).size {
        return GSL_EBADLEN;
    } else {
        ((*(*w).type_).lcurve)(reg_param, rho, eta, (*w).state)
    }
}

pub unsafe fn gsl_multilarge_linear_wstdform1(
    L: *const gsl_vector,
    X: *const gsl_matrix,
    w: *const gsl_vector,
    y: *const gsl_vector,
    Xs: *mut gsl_matrix,
    ys: *mut gsl_vector,
    work: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    let n = (*X).size1;
    let p = (*X).size2;

    if !L.is_null() && p != (*L).size {
        return GSL_EBADLEN;
    } else if n != (*y).size {
        return GSL_EBADLEN;
    } else if !w.is_null() && n != (*w).size {
        return GSL_EBADLEN;
    } else if n != (*Xs).size1 || p != (*Xs).size2 {
        return GSL_EBADLEN;
    } else if n != (*ys).size {
        return GSL_EBADLEN;
    }

    let mut status = gsl_multifit_linear_applyW(X, w, y, Xs, ys);
    if status != GSL_SUCCESS {
        return status;
    }

    if !L.is_null() {
        for j in 0..p {
            let lj = gsl_vector_get(L, j);
            if lj == 0.0 {
                return GSL_EDOM;
            }
            gsl_vector_scale(gsl_matrix_column(Xs, j).vector, 1.0 / lj);
        }
    }

    GSL_SUCCESS
}

pub unsafe fn gsl_multilarge_linear_stdform1(
    L: *const gsl_vector,
    X: *const gsl_matrix,
    y: *const gsl_vector,
    Xs: *mut gsl_matrix,
    ys: *mut gsl_vector,
    work: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    gsl_multilarge_linear_wstdform1(L, X, ptr::null(), y, Xs, ys, work)
}

pub unsafe fn gsl_multilarge_linear_L_decomp(
    L: *mut gsl_matrix,
    tau: *mut gsl_vector,
) -> c_int {
    let m = (*L).size1;
    let p = (*L).size2;

    if m < p {
        GSL_EBADLEN
    } else {
        gsl_multifit_linear_L_decomp(L, tau)
    }
}

pub unsafe fn gsl_multilarge_linear_wstdform2(
    LQR: *const gsl_matrix,
    Ltau: *const gsl_vector,
    X: *const gsl_matrix,
    w: *const gsl_vector,
    y: *const gsl_vector,
    Xs: *mut gsl_matrix,
    ys: *mut gsl_vector,
    work: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    let m = (*LQR).size1;
    let n = (*X).size1;
    let p = (*X).size2;

    if p != (*work).p {
        return GSL_EBADLEN;
    } else if p != (*LQR).size2 {
        return GSL_EBADLEN;
    } else if n != (*y).size {
        return GSL_EBADLEN;
    } else if !w.is_null() && n != (*w).size {
        return GSL_EBADLEN;
    } else if m < p {
        return GSL_EBADLEN;
    } else if n != (*Xs).size1 || p != (*Xs).size2 {
        return GSL_EBADLEN;
    } else if n != (*ys).size {
        return GSL_EBADLEN;
    }

    let mut status = gsl_multifit_linear_applyW(X, w, y, Xs, ys);
    if status != GSL_SUCCESS {
        return status;
    }

    let R = gsl_matrix_const_submatrix(LQR, 0, 0, p, p);
    for i in 0..n {
        let mut v = gsl_matrix_row(Xs, i);
        gsl_blas_dtrsv(CblasUpper, CblasTrans, CblasNonUnit, &R.matrix, &mut v.vector);
    }

    GSL_SUCCESS
}

pub unsafe fn gsl_multilarge_linear_stdform2(
    LQR: *const gsl_matrix,
    Ltau: *const gsl_vector,
    X: *const gsl_matrix,
    y: *const gsl_vector,
    Xs: *mut gsl_matrix,
    ys: *mut gsl_vector,
    work: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    gsl_multilarge_linear_wstdform2(LQR, Ltau, X, ptr::null(), y, Xs, ys, work)
}

pub unsafe fn gsl_multilarge_linear_genform1(
    L: *const gsl_vector,
    cs: *const gsl_vector,
    c: *mut gsl_vector,
    work: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    if (*L).size != (*work).p {
        return GSL_EBADLEN;
    } else if (*L).size != (*cs).size {
        return GSL_EBADLEN;
    } else if (*L).size != (*c).size {
        return GSL_EBADLEN;
    }

    gsl_vector_memcpy(c, cs);
    gsl_vector_div(c, L);
    GSL_SUCCESS
}

pub unsafe fn gsl_multilarge_linear_genform2(
    LQR: *const gsl_matrix,
    Ltau: *const gsl_vector,
    cs: *const gsl_vector,
    c: *mut gsl_vector,
    work: *mut gsl_multilarge_linear_workspace,
) -> c_int {
    let m = (*LQR).size1;
    let p = (*LQR).size2;

    if p != (*c).size {
        return GSL_EBADLEN;
    } else if m < p {
        return GSL_EBADLEN;
    } else if p != (*cs).size {
        return GSL_EBADLEN;
    }

    let R = gsl_matrix_const_submatrix(LQR, 0, 0, p, p);
    gsl_vector_memcpy(c, cs);
    gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, &R.matrix, c)
}

pub unsafe fn gsl_multilarge_linear_matrix_ptr(
    work: *const gsl_multilarge_linear_workspace,
) -> *const gsl_matrix {
    ((*(*work).type_).matrix_ptr)((*work).state)
}

pub unsafe fn gsl_multilarge_linear_rhs_ptr(
    work: *const gsl_multilarge_linear_workspace,
) -> *const gsl_vector {
    ((*(*work).type_).rhs_ptr)((*work).state)
}