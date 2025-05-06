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
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
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
pub struct gsl_bspline_workspace {
    pub k: size_t,
    pub km1: size_t,
    pub l: size_t,
    pub nbreak: size_t,
    pub n: size_t,
    pub knots: *mut gsl_vector,
    pub deltal: *mut gsl_vector,
    pub deltar: *mut gsl_vector,
    pub B: *mut gsl_vector,
    pub A: *mut gsl_matrix,
    pub dB: *mut gsl_matrix,
}
#[inline]
unsafe extern "C" fn GSL_MAX_INT(mut a: i32, mut b: i32) -> i32 {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MIN_INT(mut a: i32, mut b: i32) -> i32 {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_alloc(
    k: size_t,
    nbreak: size_t,
) -> *mut gsl_bspline_workspace {
    if k == 0 as i32 as u64 {
        gsl_error(
            b"k must be at least 1\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_bspline_workspace;
    } else if nbreak < 2 as i32 as u64 {
        gsl_error(
            b"nbreak must be at least 2\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            61 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_bspline_workspace;
    } else {
        let mut w: *mut gsl_bspline_workspace = 0 as *mut gsl_bspline_workspace;
        w = calloc(
            1 as i32 as u64,
            ::core::mem::size_of::<gsl_bspline_workspace>() as u64,
        ) as *mut gsl_bspline_workspace;
        if w.is_null() {
            gsl_error(
                b"failed to allocate space for workspace\0" as *const u8 as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                72 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        (*w).k = k;
        (*w).km1 = k.wrapping_sub(1 as i32 as u64);
        (*w).nbreak = nbreak;
        (*w).l = nbreak.wrapping_sub(1 as i32 as u64);
        (*w).n = ((*w).l).wrapping_add(k).wrapping_sub(1 as i32 as u64);
        (*w).knots = gsl_vector_alloc(((*w).n).wrapping_add(k));
        if ((*w).knots).is_null() {
            gsl_bspline_free(w);
            gsl_error(
                b"failed to allocate space for knots vector\0" as *const u8 as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                86 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        (*w).deltal = gsl_vector_alloc(k);
        if ((*w).deltal).is_null() {
            gsl_bspline_free(w);
            gsl_error(
                b"failed to allocate space for deltal vector\0" as *const u8
                    as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                94 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        (*w).deltar = gsl_vector_alloc(k);
        if ((*w).deltar).is_null() {
            gsl_bspline_free(w);
            gsl_error(
                b"failed to allocate space for deltar vector\0" as *const u8
                    as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                102 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        (*w).B = gsl_vector_alloc(k);
        if ((*w).B).is_null() {
            gsl_bspline_free(w);
            gsl_error(
                b"failed to allocate space for temporary spline vector\0" as *const u8
                    as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                111 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        (*w).A = gsl_matrix_alloc(k, k);
        if ((*w).A).is_null() {
            gsl_bspline_free(w);
            gsl_error(
                b"failed to allocate space for derivative work matrix\0" as *const u8
                    as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                120 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        (*w).dB = gsl_matrix_alloc(k, k.wrapping_add(1 as i32 as u64));
        if ((*w).dB).is_null() {
            gsl_bspline_free(w);
            gsl_error(
                b"failed to allocate space for temporary derivative matrix\0"
                    as *const u8 as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                129 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_bspline_workspace;
        }
        return w;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_free(mut w: *mut gsl_bspline_workspace) {
    if w.is_null() {
        return;
    }
    if !((*w).knots).is_null() {
        gsl_vector_free((*w).knots);
    }
    if !((*w).deltal).is_null() {
        gsl_vector_free((*w).deltal);
    }
    if !((*w).deltar).is_null() {
        gsl_vector_free((*w).deltar);
    }
    if !((*w).B).is_null() {
        gsl_vector_free((*w).B);
    }
    if !((*w).A).is_null() {
        gsl_matrix_free((*w).A);
    }
    if !((*w).dB).is_null() {
        gsl_matrix_free((*w).dB);
    }
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_ncoeffs(
    mut w: *mut gsl_bspline_workspace,
) -> size_t {
    return (*w).n;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_order(mut w: *mut gsl_bspline_workspace) -> size_t {
    return (*w).k;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_nbreak(
    mut w: *mut gsl_bspline_workspace,
) -> size_t {
    return (*w).nbreak;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_breakpoint(
    mut i: size_t,
    mut w: *mut gsl_bspline_workspace,
) -> libc::c_double {
    let mut j: size_t = i.wrapping_add((*w).k).wrapping_sub(1 as i32 as u64);
    return gsl_vector_get((*w).knots, j);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_knots(
    mut breakpts: *const gsl_vector,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    if (*breakpts).size != (*w).nbreak {
        gsl_error(
            b"breakpts vector has wrong size\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            228 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        i = 0 as i32 as size_t;
        while i < (*w).k {
            gsl_vector_set((*w).knots, i, gsl_vector_get(breakpts, 0 as i32 as size_t));
            i = i.wrapping_add(1);
            i;
        }
        i = 1 as i32 as size_t;
        while i < (*w).l {
            gsl_vector_set(
                (*w).knots,
                ((*w).k).wrapping_sub(1 as i32 as u64).wrapping_add(i),
                gsl_vector_get(breakpts, i),
            );
            i = i.wrapping_add(1);
            i;
        }
        i = (*w).n;
        while i < ((*w).n).wrapping_add((*w).k) {
            gsl_vector_set((*w).knots, i, gsl_vector_get(breakpts, (*w).l));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_knots_uniform(
    a: libc::c_double,
    b: libc::c_double,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    let mut i: size_t = 0;
    let mut delta: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    delta = (b - a) / (*w).l as libc::c_double;
    i = 0 as i32 as size_t;
    while i < (*w).k {
        gsl_vector_set((*w).knots, i, a);
        i = i.wrapping_add(1);
        i;
    }
    x = a + delta;
    i = 0 as i32 as size_t;
    while i < ((*w).l).wrapping_sub(1 as i32 as u64) {
        gsl_vector_set((*w).knots, ((*w).k).wrapping_add(i), x);
        x += delta;
        i = i.wrapping_add(1);
        i;
    }
    i = (*w).n;
    while i < ((*w).n).wrapping_add((*w).k) {
        gsl_vector_set((*w).knots, i, b);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_eval(
    x: libc::c_double,
    mut B: *mut gsl_vector,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    if (*B).size != (*w).n {
        gsl_error(
            b"vector B not of length n\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            322 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        let mut istart: size_t = 0;
        let mut iend: size_t = 0;
        let mut error: i32 = 0;
        error = gsl_bspline_eval_nonzero(x, (*w).B, &mut istart, &mut iend, w);
        if error != 0 {
            return error;
        }
        i = 0 as i32 as size_t;
        while i < istart {
            gsl_vector_set(B, i, 0.0f64);
            i = i.wrapping_add(1);
            i;
        }
        i = istart;
        while i <= iend {
            gsl_vector_set(B, i, gsl_vector_get((*w).B, i.wrapping_sub(istart)));
            i = i.wrapping_add(1);
            i;
        }
        i = iend.wrapping_add(1 as i32 as u64);
        while i < (*w).n {
            gsl_vector_set(B, i, 0.0f64);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_eval_nonzero(
    x: libc::c_double,
    mut Bk: *mut gsl_vector,
    mut istart: *mut size_t,
    mut iend: *mut size_t,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    if (*Bk).size != (*w).k {
        gsl_error(
            b"Bk vector length does not match order k\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            384 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut flag: i32 = 0 as i32;
        let mut error: i32 = 0 as i32;
        i = bspline_find_interval(x, &mut flag, w);
        error = bspline_process_interval_for_eval(x, &mut i, flag, w);
        if error != 0 {
            return error;
        }
        *istart = i.wrapping_sub((*w).k).wrapping_add(1 as i32 as u64);
        *iend = i;
        bspline_pppack_bsplvb(
            (*w).knots,
            (*w).k,
            1 as i32 as size_t,
            x,
            *iend,
            &mut j,
            (*w).deltal,
            (*w).deltar,
            Bk,
        );
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_deriv_eval(
    x: libc::c_double,
    nderiv: size_t,
    mut dB: *mut gsl_matrix,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    if (*dB).size1 != (*w).n {
        gsl_error(
            b"dB matrix first dimension not of length n\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            436 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*dB).size2 < nderiv.wrapping_add(1 as i32 as u64) {
        gsl_error(
            b"dB matrix second dimension must be at least length nderiv+1\0" as *const u8
                as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            442 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut istart: size_t = 0;
        let mut iend: size_t = 0;
        let mut error: i32 = 0;
        error = gsl_bspline_deriv_eval_nonzero(
            x,
            nderiv,
            (*w).dB,
            &mut istart,
            &mut iend,
            w,
        );
        if error != 0 {
            return error;
        }
        j = 0 as i32 as size_t;
        while j <= nderiv {
            i = 0 as i32 as size_t;
            while i < istart {
                gsl_matrix_set(dB, i, j, 0.0f64);
                i = i.wrapping_add(1);
                i;
            }
            i = istart;
            while i <= iend {
                gsl_matrix_set(
                    dB,
                    i,
                    j,
                    gsl_matrix_get((*w).dB, i.wrapping_sub(istart), j),
                );
                i = i.wrapping_add(1);
                i;
            }
            i = iend.wrapping_add(1 as i32 as u64);
            while i < (*w).n {
                gsl_matrix_set(dB, i, j, 0.0f64);
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_bspline_deriv_eval_nonzero(
    x: libc::c_double,
    nderiv: size_t,
    mut dB: *mut gsl_matrix,
    mut istart: *mut size_t,
    mut iend: *mut size_t,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    if (*dB).size1 != (*w).k {
        gsl_error(
            b"dB matrix first dimension not of length k\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            522 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*dB).size2 < nderiv.wrapping_add(1 as i32 as u64) {
        gsl_error(
            b"dB matrix second dimension must be at least length nderiv+1\0" as *const u8
                as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            528 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut flag: i32 = 0 as i32;
        let mut error: i32 = 0 as i32;
        let mut min_nderivk: size_t = 0;
        i = bspline_find_interval(x, &mut flag, w);
        error = bspline_process_interval_for_eval(x, &mut i, flag, w);
        if error != 0 {
            return error;
        }
        *istart = i.wrapping_sub((*w).k).wrapping_add(1 as i32 as u64);
        *iend = i;
        bspline_pppack_bsplvd(
            (*w).knots,
            (*w).k,
            x,
            *iend,
            (*w).deltal,
            (*w).deltar,
            (*w).A,
            dB,
            nderiv,
        );
        min_nderivk = GSL_MIN_INT(
            nderiv as i32,
            ((*w).k).wrapping_sub(1 as i32 as u64) as i32,
        ) as size_t;
        j = min_nderivk.wrapping_add(1 as i32 as u64);
        while j <= nderiv {
            i = 0 as i32 as size_t;
            while i < (*w).k {
                gsl_matrix_set(dB, i, j, 0.0f64);
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as i32;
    };
}
#[inline]
unsafe extern "C" fn bspline_find_interval(
    x: libc::c_double,
    mut flag: *mut i32,
    mut w: *mut gsl_bspline_workspace,
) -> size_t {
    let mut i: size_t = 0;
    if x < gsl_vector_get((*w).knots, 0 as i32 as size_t) {
        *flag = -(1 as i32);
        return 0 as i32 as size_t;
    }
    i = ((*w).k).wrapping_sub(1 as i32 as u64);
    while i < ((*w).k).wrapping_add((*w).l).wrapping_sub(1 as i32 as u64) {
        let ti: libc::c_double = gsl_vector_get((*w).knots, i);
        let tip1: libc::c_double = gsl_vector_get(
            (*w).knots,
            i.wrapping_add(1 as i32 as u64),
        );
        if tip1 < ti {
            gsl_error(
                b"knots vector is not increasing\0" as *const u8 as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                606 as i32,
                GSL_EINVAL as i32,
            );
            return GSL_EINVAL as i32 as size_t;
        }
        if ti <= x && x < tip1 {
            break;
        }
        if ti < x && x == tip1
            && tip1
                == gsl_vector_get(
                    (*w).knots,
                    ((*w).k).wrapping_add((*w).l).wrapping_sub(1 as i32 as u64),
                )
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == ((*w).k).wrapping_add((*w).l).wrapping_sub(1 as i32 as u64) {
        *flag = 1 as i32;
    } else {
        *flag = 0 as i32;
    }
    return i;
}
#[inline]
unsafe extern "C" fn bspline_process_interval_for_eval(
    x: libc::c_double,
    mut i: *mut size_t,
    flag: i32,
    mut w: *mut gsl_bspline_workspace,
) -> i32 {
    if flag == -(1 as i32) {
        gsl_error(
            b"x outside of knot interval\0" as *const u8 as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            638 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else if flag == 1 as i32 {
        if x <= gsl_vector_get((*w).knots, *i) + 2.2204460492503131e-16f64 {
            *i = (*i as u64).wrapping_sub(1 as i32 as u64) as size_t as size_t;
        } else {
            gsl_error(
                b"x outside of knot interval\0" as *const u8 as *const i8,
                b"bspline.c\0" as *const u8 as *const i8,
                648 as i32,
                GSL_EINVAL as i32,
            );
            return GSL_EINVAL as i32;
        }
    }
    if gsl_vector_get((*w).knots, *i)
        == gsl_vector_get((*w).knots, (*i).wrapping_add(1 as i32 as u64))
    {
        gsl_error(
            b"knot(i) = knot(i+1) will result in division by zero\0" as *const u8
                as *const i8,
            b"bspline.c\0" as *const u8 as *const i8,
            654 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn bspline_pppack_bsplvb(
    mut t: *const gsl_vector,
    jhigh: size_t,
    index: size_t,
    x: libc::c_double,
    left: size_t,
    mut j: *mut size_t,
    mut deltal: *mut gsl_vector,
    mut deltar: *mut gsl_vector,
    mut biatx: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut saved: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    if index == 1 as i32 as u64 {
        *j = 0 as i32 as size_t;
        gsl_vector_set(biatx, 0 as i32 as size_t, 1.0f64);
    }
    while *j < jhigh.wrapping_sub(1 as i32 as u64) {
        gsl_vector_set(
            deltar,
            *j,
            gsl_vector_get(t, left.wrapping_add(*j).wrapping_add(1 as i32 as u64)) - x,
        );
        gsl_vector_set(deltal, *j, x - gsl_vector_get(t, left.wrapping_sub(*j)));
        saved = 0.0f64;
        i = 0 as i32 as size_t;
        while i <= *j {
            term = gsl_vector_get(biatx, i)
                / (gsl_vector_get(deltar, i)
                    + gsl_vector_get(deltal, (*j).wrapping_sub(i)));
            gsl_vector_set(biatx, i, saved + gsl_vector_get(deltar, i) * term);
            saved = gsl_vector_get(deltal, (*j).wrapping_sub(i)) * term;
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(biatx, (*j).wrapping_add(1 as i32 as u64), saved);
        *j = (*j as u64).wrapping_add(1 as i32 as u64) as size_t as size_t;
    }
}
unsafe extern "C" fn bspline_pppack_bsplvd(
    mut t: *const gsl_vector,
    k: size_t,
    x: libc::c_double,
    left: size_t,
    mut deltal: *mut gsl_vector,
    mut deltar: *mut gsl_vector,
    mut a: *mut gsl_matrix,
    mut dbiatx: *mut gsl_matrix,
    nderiv: size_t,
) {
    let mut i: i32 = 0;
    let mut ideriv: i32 = 0;
    let mut il: i32 = 0;
    let mut j: i32 = 0;
    let mut jlow: i32 = 0;
    let mut jp1mid: i32 = 0;
    let mut kmm: i32 = 0;
    let mut ldummy: i32 = 0;
    let mut m: i32 = 0;
    let mut mhigh: i32 = 0;
    let mut factor: libc::c_double = 0.;
    let mut fkmm: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut bsplvb_j: size_t = 0;
    let mut dbcol: gsl_vector_view = gsl_matrix_column(dbiatx, 0 as i32 as size_t);
    mhigh = GSL_MIN_INT(nderiv as i32, k.wrapping_sub(1 as i32 as u64) as i32);
    bspline_pppack_bsplvb(
        t,
        k.wrapping_sub(mhigh as u64),
        1 as i32 as size_t,
        x,
        left,
        &mut bsplvb_j,
        deltal,
        deltar,
        &mut dbcol.vector,
    );
    if mhigh > 0 as i32 {
        ideriv = mhigh;
        m = 1 as i32;
        while m <= mhigh {
            j = ideriv;
            jp1mid = 0 as i32;
            while j < k as i32 {
                gsl_matrix_set(
                    dbiatx,
                    j as size_t,
                    ideriv as size_t,
                    gsl_matrix_get(dbiatx, jp1mid as size_t, 0 as i32 as size_t),
                );
                j += 1;
                j;
                jp1mid += 1;
                jp1mid;
            }
            ideriv -= 1;
            ideriv;
            bspline_pppack_bsplvb(
                t,
                k.wrapping_sub(ideriv as u64),
                2 as i32 as size_t,
                x,
                left,
                &mut bsplvb_j,
                deltal,
                deltar,
                &mut dbcol.vector,
            );
            m += 1;
            m;
        }
        jlow = 0 as i32;
        i = 0 as i32;
        while i < k as i32 {
            j = jlow;
            while j < k as i32 {
                gsl_matrix_set(a, j as size_t, i as size_t, 0.0f64);
                j += 1;
                j;
            }
            jlow = i;
            gsl_matrix_set(a, i as size_t, i as size_t, 1.0f64);
            i += 1;
            i;
        }
        m = 1 as i32;
        while m <= mhigh {
            kmm = k.wrapping_sub(m as u64) as i32;
            fkmm = kmm as libc::c_float as libc::c_double;
            il = left as i32;
            i = k.wrapping_sub(1 as i32 as u64) as i32;
            ldummy = 0 as i32;
            while ldummy < kmm {
                factor = fkmm
                    / (gsl_vector_get(t, (il + kmm) as size_t)
                        - gsl_vector_get(t, il as size_t));
                j = 0 as i32;
                while j <= i {
                    gsl_matrix_set(
                        a,
                        i as size_t,
                        j as size_t,
                        factor
                            * (gsl_matrix_get(a, i as size_t, j as size_t)
                                - gsl_matrix_get(a, (i - 1 as i32) as size_t, j as size_t)),
                    );
                    j += 1;
                    j;
                }
                il -= 1;
                il;
                i -= 1;
                i;
                ldummy += 1;
                ldummy;
            }
            i = 0 as i32;
            while i < k as i32 {
                sum = 0 as i32 as libc::c_double;
                jlow = GSL_MAX_INT(i, m);
                j = jlow;
                while j < k as i32 {
                    sum
                        += gsl_matrix_get(a, j as size_t, i as size_t)
                            * gsl_matrix_get(dbiatx, j as size_t, m as size_t);
                    j += 1;
                    j;
                }
                gsl_matrix_set(dbiatx, i as size_t, m as size_t, sum);
                i += 1;
                i;
            }
            m += 1;
            m;
        }
    }
}