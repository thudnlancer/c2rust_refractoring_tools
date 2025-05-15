use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_multifit_test_delta(
        dx: *const gsl_vector,
        x: *const gsl_vector,
        epsabs: libc::c_double,
        epsrel: libc::c_double,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multifit_function = gsl_multifit_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
    >,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multifit_function,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multifit_function,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fsolver {
    pub type_0: *const gsl_multifit_fsolver_type,
    pub function: *mut gsl_multifit_function,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub state: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_alloc(
    mut T: *const gsl_multifit_fsolver_type,
    mut n: size_t,
    mut p: size_t,
) -> *mut gsl_multifit_fsolver {
    let mut status: libc::c_int = 0;
    let mut s: *mut gsl_multifit_fsolver = 0 as *mut gsl_multifit_fsolver;
    if n < p {
        gsl_error(
            b"insufficient data points, n < p\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    s = malloc(::core::mem::size_of::<gsl_multifit_fsolver>() as libc::c_ulong)
        as *mut gsl_multifit_fsolver;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for multifit solver struct\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    (*s).x = gsl_vector_calloc(p);
    if ((*s).x).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    (*s).f = gsl_vector_calloc(n);
    if ((*s).f).is_null() {
        gsl_vector_free((*s).x);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for f\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    (*s).dx = gsl_vector_calloc(p);
    if ((*s).dx).is_null() {
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dx\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    (*s).state = malloc((*T).size);
    if ((*s).state).is_null() {
        gsl_vector_free((*s).dx);
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for multifit solver state\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    (*s).type_0 = T;
    status = ((*(*s).type_0).alloc)
        .expect("non-null function pointer")((*s).state, n, p);
    if status != GSL_SUCCESS as libc::c_int {
        ((*(*s).type_0).free).expect("non-null function pointer")((*s).state);
        free((*s).state);
        gsl_vector_free((*s).dx);
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to set solver\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_multifit_fsolver;
    }
    (*s).function = 0 as *mut gsl_multifit_function;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_set(
    mut s: *mut gsl_multifit_fsolver,
    mut f: *mut gsl_multifit_function,
    mut x: *const gsl_vector,
) -> libc::c_int {
    if (*(*s).f).size != (*f).n {
        gsl_error(
            b"function size does not match solver\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if (*(*s).x).size != (*x).size {
        gsl_error(
            b"vector length does not match solver\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    (*s).function = f;
    gsl_vector_memcpy((*s).x, x);
    return ((*(*s).type_0).set)
        .expect(
            "non-null function pointer",
        )((*s).state, (*s).function, (*s).x, (*s).f, (*s).dx);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_iterate(
    mut s: *mut gsl_multifit_fsolver,
) -> libc::c_int {
    return ((*(*s).type_0).iterate)
        .expect(
            "non-null function pointer",
        )((*s).state, (*s).function, (*s).x, (*s).f, (*s).dx);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_driver(
    mut s: *mut gsl_multifit_fsolver,
    maxiter: size_t,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut iter: size_t = 0 as libc::c_int as size_t;
    loop {
        status = gsl_multifit_fsolver_iterate(s);
        if status != 0 {
            break;
        }
        status = gsl_multifit_test_delta((*s).dx, (*s).x, epsabs, epsrel);
        if !(status == GSL_CONTINUE as libc::c_int
            && {
                iter = iter.wrapping_add(1);
                iter < maxiter
            })
        {
            break;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_free(mut s: *mut gsl_multifit_fsolver) {
    if s.is_null() {
        return;
    }
    ((*(*s).type_0).free).expect("non-null function pointer")((*s).state);
    free((*s).state);
    gsl_vector_free((*s).dx);
    gsl_vector_free((*s).x);
    gsl_vector_free((*s).f);
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_name(
    mut s: *const gsl_multifit_fsolver,
) -> *const libc::c_char {
    return (*(*s).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multifit_fsolver_position(
    mut s: *const gsl_multifit_fsolver,
) -> *mut gsl_vector {
    return (*s).x;
}
