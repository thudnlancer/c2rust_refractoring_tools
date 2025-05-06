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
pub struct gsl_interp_accel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_type {
    pub name: *const i8,
    pub min_size: u32,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub init: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
        ) -> i32,
    >,
    pub eval: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub eval_deriv: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub eval_deriv2: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub eval_integ: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            *mut gsl_interp_accel,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp {
    pub type_0: *const gsl_interp_type,
    pub xmin: libc::c_double,
    pub xmax: libc::c_double,
    pub size: size_t,
    pub state: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_alloc(
    mut T: *const gsl_interp_type,
    mut size: size_t,
) -> *mut gsl_interp {
    let mut interp: *mut gsl_interp = 0 as *mut gsl_interp;
    if size < (*T).min_size as u64 {
        gsl_error(
            b"insufficient number of points for interpolation type\0" as *const u8
                as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            38 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_interp;
    }
    interp = malloc(::core::mem::size_of::<gsl_interp>() as u64) as *mut gsl_interp;
    if interp.is_null() {
        gsl_error(
            b"failed to allocate space for interp struct\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            46 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_interp;
    }
    (*interp).type_0 = T;
    (*interp).size = size;
    if ((*(*interp).type_0).alloc).is_none() {
        (*interp).state = 0 as *mut libc::c_void;
        return interp;
    }
    (*interp).state = ((*(*interp).type_0).alloc)
        .expect("non-null function pointer")(size);
    if ((*interp).state).is_null() {
        free(interp as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for interp state\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_interp;
    }
    return interp;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_init(
    mut interp: *mut gsl_interp,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> i32 {
    let mut i: size_t = 0;
    if size != (*interp).size {
        gsl_error(
            b"data must match size of interpolation object\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            76 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    i = 1 as i32 as size_t;
    while i < size {
        if !(*x_array.offset(i.wrapping_sub(1 as i32 as u64) as isize)
            < *x_array.offset(i as isize))
        {
            gsl_error(
                b"x values must be strictly increasing\0" as *const u8 as *const i8,
                b"interp.c\0" as *const u8 as *const i8,
                83 as i32,
                GSL_EINVAL as i32,
            );
            return GSL_EINVAL as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*interp).xmin = *x_array.offset(0 as i32 as isize);
    (*interp).xmax = *x_array.offset(size.wrapping_sub(1 as i32 as u64) as isize);
    let mut status: i32 = ((*(*interp).type_0).init)
        .expect("non-null function pointer")((*interp).state, x_array, y_array, size);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_name(mut interp: *const gsl_interp) -> *const i8 {
    return (*(*interp).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_min_size(mut interp: *const gsl_interp) -> u32 {
    return (*(*interp).type_0).min_size;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_type_min_size(mut T: *const gsl_interp_type) -> u32 {
    return (*T).min_size;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_free(mut interp: *mut gsl_interp) {
    if interp.is_null() {
        return;
    }
    if ((*(*interp).type_0).free).is_some() {
        ((*(*interp).type_0).free).expect("non-null function pointer")((*interp).state);
    }
    free(interp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> i32 {
    if x < (*interp).xmin || x > (*interp).xmax {
        *y = ::core::f32::NAN as libc::c_double;
        return GSL_EDOM as i32;
    }
    return ((*(*interp).type_0).eval)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, x, a, y);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    let mut status: i32 = 0;
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            150 as i32,
            GSL_EDOM as i32,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    status = ((*(*interp).type_0).eval)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, x, a, &mut y);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            155 as i32,
            status,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_deriv_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> i32 {
    if x < (*interp).xmin || x > (*interp).xmax {
        *dydx = ::core::f32::NAN as libc::c_double;
        return GSL_EDOM as i32;
    }
    return ((*(*interp).type_0).eval_deriv)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, x, a, dydx);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_deriv(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut dydx: libc::c_double = 0.;
    let mut status: i32 = 0;
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            186 as i32,
            GSL_EDOM as i32,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    status = ((*(*interp).type_0).eval_deriv)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, x, a, &mut dydx);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            191 as i32,
            status,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    return dydx;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_deriv2_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut d2: *mut libc::c_double,
) -> i32 {
    if x < (*interp).xmin || x > (*interp).xmax {
        *d2 = ::core::f32::NAN as libc::c_double;
        return GSL_EDOM as i32;
    }
    return ((*(*interp).type_0).eval_deriv2)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, x, a, d2);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_deriv2(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut d2: libc::c_double = 0.;
    let mut status: i32 = 0;
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            222 as i32,
            GSL_EDOM as i32,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    status = ((*(*interp).type_0).eval_deriv2)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, x, a, &mut d2);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            227 as i32,
            status,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    return d2;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_integ_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut acc: *mut gsl_interp_accel,
    mut result: *mut libc::c_double,
) -> i32 {
    if a > b || a < (*interp).xmin || b > (*interp).xmax {
        *result = ::core::f32::NAN as libc::c_double;
        return GSL_EDOM as i32;
    } else if a == b {
        *result = 0.0f64;
        return GSL_SUCCESS as i32;
    }
    return ((*(*interp).type_0).eval_integ)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, acc, a, b, result);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_eval_integ(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut acc: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut status: i32 = 0;
    if a > b || a < (*interp).xmin || b > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            266 as i32,
            GSL_EDOM as i32,
        );
        return ::core::f32::NAN as libc::c_double;
    } else if a == b {
        return 0.0f64
    }
    status = ((*(*interp).type_0).eval_integ)
        .expect(
            "non-null function pointer",
        )((*interp).state, xa, ya, (*interp).size, acc, a, b, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const i8,
            b"interp.c\0" as *const u8 as *const i8,
            275 as i32,
            status,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    return result;
}