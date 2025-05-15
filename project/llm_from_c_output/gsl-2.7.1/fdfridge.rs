use std::ptr;
use std::mem;
use std::ffi::CString;
use libc::{c_int, c_double, size_t};
use gsl_sys::{
    gsl_blas_dgemv, gsl_error, gsl_matrix, gsl_matrix_diagonal, gsl_matrix_memcpy,
    gsl_matrix_set_zero, gsl_matrix_submatrix, gsl_matrix_view, gsl_multifit_fdfsolver,
    gsl_multifit_fdfsolver_alloc, gsl_multifit_fdfsolver_dif_df, gsl_multifit_fdfsolver_driver,
    gsl_multifit_fdfsolver_free, gsl_multifit_fdfsolver_iterate, gsl_multifit_fdfsolver_name,
    gsl_multifit_fdfsolver_position, gsl_multifit_fdfsolver_residual, gsl_multifit_fdfsolver_wset,
    gsl_multifit_function_fdf, gsl_vector, gsl_vector_alloc, gsl_vector_free, gsl_vector_memcpy,
    gsl_vector_mul, gsl_vector_scale, gsl_vector_set_all, gsl_vector_subvector, gsl_vector_view,
    CblasNoTrans, GSL_EBADLEN, GSL_ENOMEM, GSL_SUCCESS,
};

#[repr(C)]
pub struct gsl_multifit_fdfridge {
    s: *mut gsl_multifit_fdfsolver,
    wts: *mut gsl_vector,
    f: *mut gsl_vector,
    n: size_t,
    p: size_t,
    lambda: c_double,
    L_diag: *mut gsl_vector,
    L: *mut gsl_matrix,
    fdf: *mut gsl_multifit_function_fdf,
    fdftik: gsl_multifit_function_fdf,
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_alloc(
    T: *const gsl_multifit_fdfsolver,
    n: size_t,
    p: size_t,
) -> *mut gsl_multifit_fdfridge {
    let work = libc::calloc(1, mem::size_of::<gsl_multifit_fdfridge>()) as *mut gsl_multifit_fdfridge;
    if work.is_null() {
        gsl_error(
            b"failed to allocate workspace\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*work).s = gsl_multifit_fdfsolver_alloc(T, n + p, p);
    if (*work).s.is_null() {
        gsl_multifit_fdfridge_free(work);
        gsl_error(
            b"failed to allocate space for fdfsolver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*work).wts = gsl_vector_alloc(n + p);
    if (*work).wts.is_null() {
        gsl_multifit_fdfridge_free(work);
        gsl_error(
            b"failed to allocate space for weight vector\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*work).f = gsl_vector_alloc(n);
    if (*work).f.is_null() {
        gsl_multifit_fdfridge_free(work);
        gsl_error(
            b"failed to allocate space for f vector\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_ENOMEM,
        );
        return ptr::null_mut();
    }

    (*work).n = n;
    (*work).p = p;
    (*work).lambda = 0.0;
    (*work).L_diag = ptr::null_mut();
    (*work).L = ptr::null_mut();
    (*work).fdf = ptr::null_mut();

    gsl_vector_set_all((*work).wts, 1.0);

    work
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_free(work: *mut gsl_multifit_fdfridge) {
    if !(*work).s.is_null() {
        gsl_multifit_fdfsolver_free((*work).s);
    }
    if !(*work).wts.is_null() {
        gsl_vector_free((*work).wts);
    }
    if !(*work).f.is_null() {
        gsl_vector_free((*work).f);
    }
    libc::free(work as *mut _);
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_name(
    w: *const gsl_multifit_fdfridge,
) -> *const libc::c_char {
    gsl_multifit_fdfsolver_name((*w).s)
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_position(
    w: *const gsl_multifit_fdfridge,
) -> *mut gsl_vector {
    gsl_multifit_fdfsolver_position((*w).s)
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_residual(
    w: *const gsl_multifit_fdfridge,
) -> *mut gsl_vector {
    gsl_multifit_fdfsolver_residual((*w).s)
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_niter(w: *const gsl_multifit_fdfridge) -> size_t {
    (*(*w).s).niter
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_set(
    w: *mut gsl_multifit_fdfridge,
    f: *mut gsl_multifit_function_fdf,
    x: *const gsl_vector,
    lambda: c_double,
) -> c_int {
    gsl_multifit_fdfridge_wset(w, f, x, lambda, ptr::null())
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_wset(
    w: *mut gsl_multifit_fdfridge,
    f: *mut gsl_multifit_function_fdf,
    x: *const gsl_vector,
    lambda: c_double,
    wts: *const gsl_vector,
) -> c_int {
    let n = (*w).n;
    let p = (*w).p;

    if n != (*f).n || p != (*f).p {
        gsl_error(
            b"function size does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if p != (*x).size {
        gsl_error(
            b"vector length does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else {
        let mut status;
        let wv = gsl_vector_subvector((*w).wts, 0, n);

        (*w).fdf = f;
        (*w).fdftik.f = Some(fdfridge_f);
        (*w).fdftik.df = Some(fdfridge_df);
        (*w).fdftik.n = n + p;
        (*w).fdftik.p = p;
        (*w).fdftik.params = w as *mut _;
        (*w).lambda = lambda;
        (*w).L = ptr::null_mut();

        if !wts.is_null() {
            gsl_vector_memcpy(&wv.vector as *const _ as *mut _, wts);
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &(*w).fdftik,
                x,
                (*w).wts,
            );
        } else {
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &(*w).fdftik,
                x,
                ptr::null(),
            );
        }

        (*f).nevalf = (*w).fdftik.nevalf;
        (*f).nevaldf = (*w).fdftik.nevaldf;

        status
    }
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_set2(
    w: *mut gsl_multifit_fdfridge,
    f: *mut gsl_multifit_function_fdf,
    x: *const gsl_vector,
    lambda: *const gsl_vector,
) -> c_int {
    gsl_multifit_fdfridge_wset2(w, f, x, lambda, ptr::null())
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_wset2(
    w: *mut gsl_multifit_fdfridge,
    f: *mut gsl_multifit_function_fdf,
    x: *const gsl_vector,
    lambda: *const gsl_vector,
    wts: *const gsl_vector,
) -> c_int {
    let n = (*w).n;
    let p = (*w).p;

    if n != (*f).n || p != (*f).p {
        gsl_error(
            b"function size does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if p != (*x).size {
        gsl_error(
            b"vector length does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if (*lambda).size != p {
        gsl_error(
            b"lambda vector size does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else {
        let mut status;
        let wv = gsl_vector_subvector((*w).wts, 0, n);

        (*w).fdf = f;
        (*f).nevalf = 0;
        (*f).nevaldf = 0;
        (*w).fdftik.f = Some(fdfridge_f);
        (*w).fdftik.df = Some(fdfridge_df);
        (*w).fdftik.n = n + p;
        (*w).fdftik.p = p;
        (*w).fdftik.params = w as *mut _;
        (*w).lambda = 0.0;
        (*w).L_diag = lambda as *mut _;
        (*w).L = ptr::null_mut();

        if !wts.is_null() {
            gsl_vector_memcpy(&wv.vector as *const _ as *mut _, wts);
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &(*w).fdftik,
                x,
                (*w).wts,
            );
        } else {
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &(*w).fdftik,
                x,
                ptr::null(),
            );
        }

        (*f).nevalf = (*w).fdftik.nevalf;
        (*f).nevaldf = (*w).fdftik.nevaldf;

        status
    }
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_set3(
    w: *mut gsl_multifit_fdfridge,
    f: *mut gsl_multifit_function_fdf,
    x: *const gsl_vector,
    L: *const gsl_matrix,
) -> c_int {
    gsl_multifit_fdfridge_wset3(w, f, x, L, ptr::null())
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_wset3(
    w: *mut gsl_multifit_fdfridge,
    f: *mut gsl_multifit_function_fdf,
    x: *const gsl_vector,
    L: *const gsl_matrix,
    wts: *const gsl_vector,
) -> c_int {
    let n = (*w).n;
    let p = (*w).p;

    if n != (*f).n || p != (*f).p {
        gsl_error(
            b"function size does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if p != (*x).size {
        gsl_error(
            b"vector length does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if (*L).size2 != p {
        gsl_error(
            b"L matrix size2 does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0".as_ptr() as *const _,
            __FILE!(),
            __LINE!(),
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else {
        let mut status;
        let wv = gsl_vector_subvector((*w).wts, 0, n);

        (*w).fdf = f;
        (*f).nevalf = 0;
        (*f).nevaldf = 0;
        (*w).fdftik.f = Some(fdfridge_f);
        (*w).fdftik.df = Some(fdfridge_df);
        (*w).fdftik.n = n + p;
        (*w).fdftik.p = p;
        (*w).fdftik.params = w as *mut _;
        (*w).lambda = 0.0;
        (*w).L_diag = ptr::null_mut();
        (*w).L = L as *mut _;

        if !wts.is_null() {
            gsl_vector_memcpy(&wv.vector as *const _ as *mut _, wts);
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &(*w).fdftik,
                x,
                (*w).wts,
            );
        } else {
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &(*w).fdftik,
                x,
                ptr::null(),
            );
        }

        (*f).nevalf = (*w).fdftik.nevalf;
        (*f).nevaldf = (*w).fdftik.nevaldf;

        status
    }
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_iterate(w: *mut gsl_multifit_fdfridge) -> c_int {
    let status = gsl_multifit_fdfsolver_iterate((*w).s);
    (*w).fdf.nevalf = (*w).fdftik.nevalf;
    (*w).fdf.nevaldf = (*w).fdftik.nevaldf;
    status
}

pub unsafe extern "C" fn gsl_multifit_fdfridge_driver(
    w: *mut gsl_multifit_fdfridge,
    maxiter: size_t,
    xtol: c_double,
    gtol: c_double,
    ftol: c_double,
    info: *mut c_int,
) -> c_int {
    gsl_multifit_fdfsolver_driver((*w).s, maxiter, xtol, gtol, ftol, info)
}

pub unsafe extern "C" fn fdfridge_f(
    x: *const gsl_vector,
    params: *mut libc::c_void,
    f: *mut gsl_vector,
) -> c_int {
    let mut status;
    let w = params as *mut gsl_multifit_fdfridge;
    let n = (*w).n;
    let p = (*w).p;
    let f_user = gsl_vector_subvector(f, 0, n);
    let f_tik = gsl_vector_subvector(f, n, p);

    status = gsl_multifit_eval_wf((*w).fdf, x, ptr::null(), &f_user.vector as *const _ as *mut _);
    if status != GSL_SUCCESS {
        return status;
    }

    if !(*w).L_diag.is_null() {
        gsl_vector_memcpy(&f_tik.vector as *const _ as *mut _, x);
        gsl_vector_mul(&f_tik.vector as *const _ as *mut _, (*w).L_diag);
    } else if !(*w).L.is_null() {
        gsl_blas_dgemv(
            CblasNoTrans,
            1.0,
            (*w).L,
            x,
            0.0,
            &f_tik.vector as *const _ as *mut _,
        );
   