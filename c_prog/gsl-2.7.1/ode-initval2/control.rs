#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub struct gsl_odeiv2_system {
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub jacobian: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub dimension: size_t,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_step_struct {
    pub type_0: *const gsl_odeiv2_step_type,
    pub dimension: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_step_type {
    pub name: *const libc::c_char,
    pub can_use_dydt_in: libc::c_int,
    pub gives_exact_dydt_out: libc::c_int,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub apply: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *const gsl_odeiv2_system,
        ) -> libc::c_int,
    >,
    pub set_driver: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> libc::c_int,
    >,
    pub reset: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub order: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_driver = gsl_odeiv2_driver_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_driver_struct {
    pub sys: *const gsl_odeiv2_system,
    pub s: *mut gsl_odeiv2_step,
    pub c: *mut gsl_odeiv2_control,
    pub e: *mut gsl_odeiv2_evolve,
    pub h: libc::c_double,
    pub hmin: libc::c_double,
    pub hmax: libc::c_double,
    pub n: libc::c_ulong,
    pub nmax: libc::c_ulong,
}
pub type gsl_odeiv2_evolve = gsl_odeiv2_evolve_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_evolve_struct {
    pub dimension: size_t,
    pub y0: *mut libc::c_double,
    pub yerr: *mut libc::c_double,
    pub dydt_in: *mut libc::c_double,
    pub dydt_out: *mut libc::c_double,
    pub last_step: libc::c_double,
    pub count: libc::c_ulong,
    pub failed_steps: libc::c_ulong,
    pub driver: *const gsl_odeiv2_driver,
}
pub type gsl_odeiv2_control = gsl_odeiv2_control_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_control_struct {
    pub type_0: *const gsl_odeiv2_control_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_control_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub hadjust: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_uint,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub errlevel: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            size_t,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub set_driver: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_step = gsl_odeiv2_step_struct;
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_alloc(
    mut T: *const gsl_odeiv2_control_type,
) -> *mut gsl_odeiv2_control {
    let mut c: *mut gsl_odeiv2_control = malloc(
        ::core::mem::size_of::<gsl_odeiv2_control>() as libc::c_ulong,
    ) as *mut gsl_odeiv2_control;
    if c.is_null() {
        gsl_error(
            b"failed to allocate space for control struct\0" as *const u8
                as *const libc::c_char,
            b"control.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_odeiv2_control;
    }
    (*c).type_0 = T;
    (*c).state = ((*(*c).type_0).alloc).expect("non-null function pointer")();
    if ((*c).state).is_null() {
        free(c as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for control state\0" as *const u8
                as *const libc::c_char,
            b"control.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_odeiv2_control;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_init(
    mut c: *mut gsl_odeiv2_control,
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
    mut a_y: libc::c_double,
    mut a_dydt: libc::c_double,
) -> libc::c_int {
    return ((*(*c).type_0).init)
        .expect("non-null function pointer")((*c).state, eps_abs, eps_rel, a_y, a_dydt);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_free(mut c: *mut gsl_odeiv2_control) {
    if c.is_null() {
        return;
    }
    ((*(*c).type_0).free).expect("non-null function pointer")((*c).state);
    free(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_name(
    mut c: *const gsl_odeiv2_control,
) -> *const libc::c_char {
    return (*(*c).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_hadjust(
    mut c: *mut gsl_odeiv2_control,
    mut s: *mut gsl_odeiv2_step,
    mut y: *const libc::c_double,
    mut yerr: *const libc::c_double,
    mut dydt: *const libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    return ((*(*c).type_0).hadjust)
        .expect(
            "non-null function pointer",
        )(
        (*c).state,
        (*s).dimension,
        ((*(*s).type_0).order).expect("non-null function pointer")((*s).state),
        y,
        yerr,
        dydt,
        h,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_errlevel(
    mut c: *mut gsl_odeiv2_control,
    y: libc::c_double,
    dydt: libc::c_double,
    h: libc::c_double,
    ind: size_t,
    mut errlev: *mut libc::c_double,
) -> libc::c_int {
    return ((*(*c).type_0).errlevel)
        .expect("non-null function pointer")((*c).state, y, dydt, h, ind, errlev);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_control_set_driver(
    mut c: *mut gsl_odeiv2_control,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    if !d.is_null() {
        ((*(*c).type_0).set_driver).expect("non-null function pointer")((*c).state, d);
    } else {
        gsl_error(
            b"driver pointer is null\0" as *const u8 as *const libc::c_char,
            b"control.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_EFAULT as libc::c_int,
        );
        return GSL_EFAULT as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
