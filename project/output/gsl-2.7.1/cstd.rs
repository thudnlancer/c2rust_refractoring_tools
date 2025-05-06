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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_odeiv_control_alloc(
        T: *const gsl_odeiv_control_type,
    ) -> *mut gsl_odeiv_control;
    fn gsl_odeiv_control_init(
        c: *mut gsl_odeiv_control,
        eps_abs: libc::c_double,
        eps_rel: libc::c_double,
        a_y: libc::c_double,
        a_dydt: libc::c_double,
    ) -> i32;
    fn gsl_odeiv_control_free(c: *mut gsl_odeiv_control);
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
pub struct gsl_odeiv_control_type {
    pub name: *const i8,
    pub alloc: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub init: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> i32,
    >,
    pub hadjust: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            u32,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_control {
    pub type_0: *const gsl_odeiv_control_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct std_control_state_t {
    pub eps_abs: libc::c_double,
    pub eps_rel: libc::c_double,
    pub a_y: libc::c_double,
    pub a_dydt: libc::c_double,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn std_control_alloc() -> *mut libc::c_void {
    let mut s: *mut std_control_state_t = malloc(
        ::core::mem::size_of::<std_control_state_t>() as u64,
    ) as *mut std_control_state_t;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for std_control_state\0" as *const u8
                as *const i8,
            b"cstd.c\0" as *const u8 as *const i8,
            45 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    return s as *mut libc::c_void;
}
unsafe extern "C" fn std_control_init(
    mut vstate: *mut libc::c_void,
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
    mut a_y: libc::c_double,
    mut a_dydt: libc::c_double,
) -> i32 {
    let mut s: *mut std_control_state_t = vstate as *mut std_control_state_t;
    if eps_abs < 0 as i32 as libc::c_double {
        gsl_error(
            b"eps_abs is negative\0" as *const u8 as *const i8,
            b"cstd.c\0" as *const u8 as *const i8,
            59 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else if eps_rel < 0 as i32 as libc::c_double {
        gsl_error(
            b"eps_rel is negative\0" as *const u8 as *const i8,
            b"cstd.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else if a_y < 0 as i32 as libc::c_double {
        gsl_error(
            b"a_y is negative\0" as *const u8 as *const i8,
            b"cstd.c\0" as *const u8 as *const i8,
            67 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else if a_dydt < 0 as i32 as libc::c_double {
        gsl_error(
            b"a_dydt is negative\0" as *const u8 as *const i8,
            b"cstd.c\0" as *const u8 as *const i8,
            71 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    (*s).eps_rel = eps_rel;
    (*s).eps_abs = eps_abs;
    (*s).a_y = a_y;
    (*s).a_dydt = a_dydt;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn std_control_hadjust(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut ord: u32,
    mut y: *const libc::c_double,
    mut yerr: *const libc::c_double,
    mut yp: *const libc::c_double,
    mut h: *mut libc::c_double,
) -> i32 {
    let mut state: *mut std_control_state_t = vstate as *mut std_control_state_t;
    let eps_abs: libc::c_double = (*state).eps_abs;
    let eps_rel: libc::c_double = (*state).eps_rel;
    let a_y: libc::c_double = (*state).a_y;
    let a_dydt: libc::c_double = (*state).a_dydt;
    let S: libc::c_double = 0.9f64;
    let h_old: libc::c_double = *h;
    let mut rmax: libc::c_double = 2.2250738585072014e-308f64;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < dim {
        let D0: libc::c_double = eps_rel
            * (a_y * fabs(*y.offset(i as isize))
                + a_dydt * fabs(h_old * *yp.offset(i as isize))) + eps_abs;
        let r: libc::c_double = fabs(*yerr.offset(i as isize)) / fabs(D0);
        rmax = GSL_MAX_DBL(r, rmax);
        i = i.wrapping_add(1);
        i;
    }
    if rmax > 1.1f64 {
        let mut r_0: libc::c_double = S / pow(rmax, 1.0f64 / ord as libc::c_double);
        if r_0 < 0.2f64 {
            r_0 = 0.2f64;
        }
        *h = r_0 * h_old;
        return -(1 as i32);
    } else if rmax < 0.5f64 {
        let mut r_1: libc::c_double = S
            / pow(rmax, 1.0f64 / (ord as libc::c_double + 1.0f64));
        if r_1 > 5.0f64 {
            r_1 = 5.0f64;
        }
        if r_1 < 1.0f64 {
            r_1 = 1.0f64;
        }
        *h = r_1 * h_old;
        return 1 as i32;
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn std_control_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut std_control_state_t = vstate as *mut std_control_state_t;
    free(state as *mut libc::c_void);
}
static mut std_control_type: gsl_odeiv_control_type = {
    let mut init = gsl_odeiv_control_type {
        name: b"standard\0" as *const u8 as *const i8,
        alloc: Some(std_control_alloc as unsafe extern "C" fn() -> *mut libc::c_void),
        init: Some(
            std_control_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> i32,
        ),
        hadjust: Some(
            std_control_hadjust
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    u32,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    *mut libc::c_double,
                ) -> i32,
        ),
        free: Some(std_control_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
#[no_mangle]
pub static mut gsl_odeiv_control_standard: *const gsl_odeiv_control_type = unsafe {
    &std_control_type as *const gsl_odeiv_control_type
};
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_control_standard_new(
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
    mut a_y: libc::c_double,
    mut a_dydt: libc::c_double,
) -> *mut gsl_odeiv_control {
    let mut c: *mut gsl_odeiv_control = gsl_odeiv_control_alloc(
        gsl_odeiv_control_standard,
    );
    let mut status: i32 = gsl_odeiv_control_init(c, eps_abs, eps_rel, a_y, a_dydt);
    if status != GSL_SUCCESS as i32 {
        gsl_odeiv_control_free(c);
        gsl_error(
            b"error trying to initialize control\0" as *const u8 as *const i8,
            b"cstd.c\0" as *const u8 as *const i8,
            167 as i32,
            status,
        );
        return 0 as *mut gsl_odeiv_control;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_control_y_new(
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
) -> *mut gsl_odeiv_control {
    return gsl_odeiv_control_standard_new(eps_abs, eps_rel, 1.0f64, 0.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_control_yp_new(
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
) -> *mut gsl_odeiv_control {
    return gsl_odeiv_control_standard_new(eps_abs, eps_rel, 0.0f64, 1.0f64);
}