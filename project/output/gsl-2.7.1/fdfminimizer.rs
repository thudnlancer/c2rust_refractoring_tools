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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> i32;
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
pub struct gsl_multimin_function_fdf_struct {
    pub f: Option<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void) -> libc::c_double,
    >,
    pub df: Option<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void, *mut gsl_vector) -> (),
    >,
    pub fdf: Option<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut libc::c_double,
            *mut gsl_vector,
        ) -> (),
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multimin_function_fdf = gsl_multimin_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multimin_fdfminimizer_type {
    pub name: *const i8,
    pub size: size_t,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> i32>,
    pub set: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function_fdf,
            *const gsl_vector,
            *mut libc::c_double,
            *mut gsl_vector,
            libc::c_double,
            libc::c_double,
        ) -> i32,
    >,
    pub iterate: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function_fdf,
            *mut gsl_vector,
            *mut libc::c_double,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> i32,
    >,
    pub restart: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multimin_fdfminimizer {
    pub type_0: *const gsl_multimin_fdfminimizer_type,
    pub fdf: *mut gsl_multimin_function_fdf,
    pub f: libc::c_double,
    pub x: *mut gsl_vector,
    pub gradient: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub state: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_alloc(
    mut T: *const gsl_multimin_fdfminimizer_type,
    mut n: size_t,
) -> *mut gsl_multimin_fdfminimizer {
    let mut status: i32 = 0;
    let mut s: *mut gsl_multimin_fdfminimizer = malloc(
        ::core::mem::size_of::<gsl_multimin_fdfminimizer>() as u64,
    ) as *mut gsl_multimin_fdfminimizer;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for minimizer struct\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            36 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multimin_fdfminimizer;
    }
    (*s).type_0 = T;
    (*s).x = gsl_vector_calloc(n);
    if ((*s).x).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            46 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multimin_fdfminimizer;
    }
    (*s).gradient = gsl_vector_calloc(n);
    if ((*s).gradient).is_null() {
        gsl_vector_free((*s).x);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for gradient\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            55 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multimin_fdfminimizer;
    }
    (*s).dx = gsl_vector_calloc(n);
    if ((*s).dx).is_null() {
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).gradient);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dx\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            65 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multimin_fdfminimizer;
    }
    (*s).state = malloc((*T).size);
    if ((*s).state).is_null() {
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).gradient);
        gsl_vector_free((*s).dx);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for minimizer state\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            77 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multimin_fdfminimizer;
    }
    status = ((*T).alloc).expect("non-null function pointer")((*s).state, n);
    if status != GSL_SUCCESS as i32 {
        free((*s).state);
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).gradient);
        gsl_vector_free((*s).dx);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to initialize minimizer state\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            90 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_multimin_fdfminimizer;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_set(
    mut s: *mut gsl_multimin_fdfminimizer,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut step_size: libc::c_double,
    mut tol: libc::c_double,
) -> i32 {
    if (*(*s).x).size != (*fdf).n {
        gsl_error(
            b"function incompatible with solver size\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            104 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    if (*x).size != (*fdf).n {
        gsl_error(
            b"vector length not compatible with function\0" as *const u8 as *const i8,
            b"fdfminimizer.c\0" as *const u8 as *const i8,
            109 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    (*s).fdf = fdf;
    gsl_vector_memcpy((*s).x, x);
    gsl_vector_set_zero((*s).dx);
    return ((*(*s).type_0).set)
        .expect(
            "non-null function pointer",
        )((*s).state, (*s).fdf, (*s).x, &mut (*s).f, (*s).gradient, step_size, tol);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_free(
    mut s: *mut gsl_multimin_fdfminimizer,
) {
    if s.is_null() {
        return;
    }
    ((*(*s).type_0).free).expect("non-null function pointer")((*s).state);
    free((*s).state);
    gsl_vector_free((*s).dx);
    gsl_vector_free((*s).gradient);
    gsl_vector_free((*s).x);
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_iterate(
    mut s: *mut gsl_multimin_fdfminimizer,
) -> i32 {
    return ((*(*s).type_0).iterate)
        .expect(
            "non-null function pointer",
        )((*s).state, (*s).fdf, (*s).x, &mut (*s).f, (*s).gradient, (*s).dx);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_restart(
    mut s: *mut gsl_multimin_fdfminimizer,
) -> i32 {
    return ((*(*s).type_0).restart).expect("non-null function pointer")((*s).state);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_name(
    mut s: *const gsl_multimin_fdfminimizer,
) -> *const i8 {
    return (*(*s).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_x(
    mut s: *const gsl_multimin_fdfminimizer,
) -> *mut gsl_vector {
    return (*s).x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_dx(
    mut s: *const gsl_multimin_fdfminimizer,
) -> *mut gsl_vector {
    return (*s).dx;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_gradient(
    mut s: *const gsl_multimin_fdfminimizer,
) -> *mut gsl_vector {
    return (*s).gradient;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multimin_fdfminimizer_minimum(
    mut s: *const gsl_multimin_fdfminimizer,
) -> libc::c_double {
    return (*s).f;
}