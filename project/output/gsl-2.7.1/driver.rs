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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_odeiv2_evolve_set_driver(
        e: *mut gsl_odeiv2_evolve,
        d: *const gsl_odeiv2_driver,
    ) -> i32;
    fn gsl_odeiv2_evolve_free(e: *mut gsl_odeiv2_evolve);
    fn gsl_odeiv2_evolve_reset(e: *mut gsl_odeiv2_evolve) -> i32;
    fn gsl_odeiv2_evolve_apply_fixed_step(
        e: *mut gsl_odeiv2_evolve,
        con: *mut gsl_odeiv2_control,
        step: *mut gsl_odeiv2_step,
        dydt: *const gsl_odeiv2_system,
        t: *mut libc::c_double,
        h0: libc::c_double,
        y: *mut libc::c_double,
    ) -> i32;
    fn gsl_odeiv2_evolve_apply(
        e: *mut gsl_odeiv2_evolve,
        con: *mut gsl_odeiv2_control,
        step: *mut gsl_odeiv2_step,
        dydt: *const gsl_odeiv2_system,
        t: *mut libc::c_double,
        t1: libc::c_double,
        h: *mut libc::c_double,
        y: *mut libc::c_double,
    ) -> i32;
    fn gsl_odeiv2_evolve_alloc(dim: size_t) -> *mut gsl_odeiv2_evolve;
    fn gsl_odeiv2_control_scaled_new(
        eps_abs: libc::c_double,
        eps_rel: libc::c_double,
        a_y: libc::c_double,
        a_dydt: libc::c_double,
        scale_abs: *const libc::c_double,
        dim: size_t,
    ) -> *mut gsl_odeiv2_control;
    fn gsl_odeiv2_control_yp_new(
        eps_abs: libc::c_double,
        eps_rel: libc::c_double,
    ) -> *mut gsl_odeiv2_control;
    fn gsl_odeiv2_control_y_new(
        eps_abs: libc::c_double,
        eps_rel: libc::c_double,
    ) -> *mut gsl_odeiv2_control;
    fn gsl_odeiv2_control_standard_new(
        eps_abs: libc::c_double,
        eps_rel: libc::c_double,
        a_y: libc::c_double,
        a_dydt: libc::c_double,
    ) -> *mut gsl_odeiv2_control;
    fn gsl_odeiv2_control_set_driver(
        c: *mut gsl_odeiv2_control,
        d: *const gsl_odeiv2_driver,
    ) -> i32;
    fn gsl_odeiv2_control_free(c: *mut gsl_odeiv2_control);
    fn gsl_odeiv2_step_set_driver(
        s: *mut gsl_odeiv2_step,
        d: *const gsl_odeiv2_driver,
    ) -> i32;
    fn gsl_odeiv2_step_free(s: *mut gsl_odeiv2_step);
    fn gsl_odeiv2_step_reset(s: *mut gsl_odeiv2_step) -> i32;
    fn gsl_odeiv2_step_alloc(
        T: *const gsl_odeiv2_step_type,
        dim: size_t,
    ) -> *mut gsl_odeiv2_step;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
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
pub struct gsl_odeiv2_system {
    pub function: Option<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub jacobian: Option<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> i32,
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
    pub name: *const i8,
    pub can_use_dydt_in: i32,
    pub gives_exact_dydt_out: i32,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub apply: Option<
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
        ) -> i32,
    >,
    pub set_driver: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> i32,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> i32>,
    pub order: Option<unsafe extern "C" fn(*mut libc::c_void) -> u32>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
    pub n: u64,
    pub nmax: u64,
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
    pub count: u64,
    pub failed_steps: u64,
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
    pub errlevel: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            size_t,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub set_driver: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> i32,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_step = gsl_odeiv2_step_struct;
unsafe extern "C" fn driver_alloc(
    mut sys: *const gsl_odeiv2_system,
    hstart: libc::c_double,
    mut T: *const gsl_odeiv2_step_type,
) -> *mut gsl_odeiv2_driver {
    let mut state: *mut gsl_odeiv2_driver = 0 as *mut gsl_odeiv2_driver;
    if sys.is_null() {
        gsl_error(
            b"gsl_odeiv2_system must be defined\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            44 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    state = calloc(1 as i32 as u64, ::core::mem::size_of::<gsl_odeiv2_driver>() as u64)
        as *mut gsl_odeiv2_driver;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for driver state\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            52 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    let dim: size_t = (*sys).dimension;
    if dim == 0 as i32 as u64 {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"gsl_odeiv2_system dimension must be a positive integer\0" as *const u8
                as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    (*state).sys = sys;
    (*state).s = gsl_odeiv2_step_alloc(T, dim);
    if ((*state).s).is_null() {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"failed to allocate step object\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            73 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    (*state).e = gsl_odeiv2_evolve_alloc(dim);
    if ((*state).e).is_null() {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"failed to allocate evolve object\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            82 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    if hstart > 0.0f64 || hstart < 0.0f64 {
        (*state).h = hstart;
    } else {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"invalid hstart\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            92 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    (*state).h = hstart;
    (*state).hmin = 0.0f64;
    (*state).hmax = 1.7976931348623157e+308f64;
    (*state).nmax = 0 as i32 as u64;
    (*state).n = 0 as i32 as u64;
    (*state).c = 0 as *mut gsl_odeiv2_control;
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_set_hmin(
    mut d: *mut gsl_odeiv2_driver,
    hmin: libc::c_double,
) -> i32 {
    if fabs(hmin) > fabs((*d).h) || fabs(hmin) > (*d).hmax {
        gsl_error(
            b"hmin <= fabs(h) <= hmax required\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            113 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    (*d).hmin = fabs(hmin);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_set_hmax(
    mut d: *mut gsl_odeiv2_driver,
    hmax: libc::c_double,
) -> i32 {
    if fabs(hmax) < fabs((*d).h) || fabs(hmax) < (*d).hmin {
        gsl_error(
            b"hmin <= fabs(h) <= hmax required\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            129 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if hmax > 0.0f64 || hmax < 0.0f64 {
        (*d).hmax = fabs(hmax);
    } else {
        gsl_error(
            b"invalid hmax\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            138 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_set_nmax(
    mut d: *mut gsl_odeiv2_driver,
    nmax: u64,
) -> i32 {
    (*d).nmax = nmax;
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_alloc_y_new(
    mut sys: *const gsl_odeiv2_system,
    mut T: *const gsl_odeiv2_step_type,
    hstart: libc::c_double,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
) -> *mut gsl_odeiv2_driver {
    let mut state: *mut gsl_odeiv2_driver = driver_alloc(sys, hstart, T);
    if state.is_null() {
        gsl_error(
            b"failed to allocate driver object\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            167 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    if epsabs >= 0.0f64 && epsrel >= 0.0f64 {
        (*state).c = gsl_odeiv2_control_y_new(epsabs, epsrel);
        if ((*state).c).is_null() {
            gsl_odeiv2_driver_free(state);
            gsl_error(
                b"failed to allocate control object\0" as *const u8 as *const i8,
                b"driver.c\0" as *const u8 as *const i8,
                177 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_odeiv2_driver;
        }
    } else {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"epsabs and epsrel must be positive\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            183 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    gsl_odeiv2_step_set_driver((*state).s, state);
    gsl_odeiv2_evolve_set_driver((*state).e, state);
    gsl_odeiv2_control_set_driver((*state).c, state);
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_alloc_yp_new(
    mut sys: *const gsl_odeiv2_system,
    mut T: *const gsl_odeiv2_step_type,
    hstart: libc::c_double,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
) -> *mut gsl_odeiv2_driver {
    let mut state: *mut gsl_odeiv2_driver = driver_alloc(sys, hstart, T);
    if state.is_null() {
        gsl_error(
            b"failed to allocate driver object\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            207 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    if epsabs >= 0.0f64 && epsrel >= 0.0f64 {
        (*state).c = gsl_odeiv2_control_yp_new(epsabs, epsrel);
        if ((*state).c).is_null() {
            gsl_odeiv2_driver_free(state);
            gsl_error(
                b"failed to allocate control object\0" as *const u8 as *const i8,
                b"driver.c\0" as *const u8 as *const i8,
                217 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_odeiv2_driver;
        }
    } else {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"epsabs and epsrel must be positive\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            223 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    gsl_odeiv2_step_set_driver((*state).s, state);
    gsl_odeiv2_evolve_set_driver((*state).e, state);
    gsl_odeiv2_control_set_driver((*state).c, state);
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_alloc_standard_new(
    mut sys: *const gsl_odeiv2_system,
    mut T: *const gsl_odeiv2_step_type,
    hstart: libc::c_double,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
    a_y: libc::c_double,
    a_dydt: libc::c_double,
) -> *mut gsl_odeiv2_driver {
    let mut state: *mut gsl_odeiv2_driver = driver_alloc(sys, hstart, T);
    if state.is_null() {
        gsl_error(
            b"failed to allocate driver object\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            251 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    if epsabs >= 0.0f64 && epsrel >= 0.0f64 {
        (*state).c = gsl_odeiv2_control_standard_new(epsabs, epsrel, a_y, a_dydt);
        if ((*state).c).is_null() {
            gsl_odeiv2_driver_free(state);
            gsl_error(
                b"failed to allocate control object\0" as *const u8 as *const i8,
                b"driver.c\0" as *const u8 as *const i8,
                262 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_odeiv2_driver;
        }
    } else {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"epsabs and epsrel must be positive\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            268 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    gsl_odeiv2_step_set_driver((*state).s, state);
    gsl_odeiv2_evolve_set_driver((*state).e, state);
    gsl_odeiv2_control_set_driver((*state).c, state);
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_alloc_scaled_new(
    mut sys: *const gsl_odeiv2_system,
    mut T: *const gsl_odeiv2_step_type,
    hstart: libc::c_double,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
    a_y: libc::c_double,
    a_dydt: libc::c_double,
    mut scale_abs: *const libc::c_double,
) -> *mut gsl_odeiv2_driver {
    let mut state: *mut gsl_odeiv2_driver = driver_alloc(sys, hstart, T);
    if state.is_null() {
        gsl_error(
            b"failed to allocate driver object\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            296 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    if epsabs >= 0.0f64 && epsrel >= 0.0f64 {
        (*state).c = gsl_odeiv2_control_scaled_new(
            epsabs,
            epsrel,
            a_y,
            a_dydt,
            scale_abs,
            (*sys).dimension,
        );
        if ((*state).c).is_null() {
            gsl_odeiv2_driver_free(state);
            gsl_error(
                b"failed to allocate control object\0" as *const u8 as *const i8,
                b"driver.c\0" as *const u8 as *const i8,
                307 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut gsl_odeiv2_driver;
        }
    } else {
        gsl_odeiv2_driver_free(state);
        gsl_error(
            b"epsabs and epsrel must be positive\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            313 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_odeiv2_driver;
    }
    gsl_odeiv2_step_set_driver((*state).s, state);
    gsl_odeiv2_evolve_set_driver((*state).e, state);
    gsl_odeiv2_control_set_driver((*state).c, state);
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_apply(
    mut d: *mut gsl_odeiv2_driver,
    mut t: *mut libc::c_double,
    t1: libc::c_double,
    mut y: *mut libc::c_double,
) -> i32 {
    let mut sign: i32 = 0 as i32;
    (*d).n = 0 as i32 as u64;
    if (*d).h > 0.0f64 {
        sign = 1 as i32;
    } else {
        sign = -(1 as i32);
    }
    if sign as libc::c_double * (t1 - *t) < 0.0f64 {
        gsl_error(
            b"integration limits and/or step direction not consistent\0" as *const u8
                as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            356 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    while sign as libc::c_double * (t1 - *t) > 0.0f64 {
        let mut s: i32 = gsl_odeiv2_evolve_apply(
            (*d).e,
            (*d).c,
            (*d).s,
            (*d).sys,
            t,
            t1,
            &mut (*d).h,
            y,
        );
        if s != GSL_SUCCESS as i32 {
            return s;
        }
        if (*d).nmax > 0 as i32 as u64 && (*d).n > (*d).nmax {
            return GSL_EMAXITER as i32;
        }
        if fabs((*d).h) > (*d).hmax {
            (*d).h = sign as libc::c_double * (*d).hmax;
        }
        if fabs((*d).h) < (*d).hmin {
            return GSL_ENOPROG as i32;
        }
        (*d).n = ((*d).n).wrapping_add(1);
        (*d).n;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_apply_fixed_step(
    mut d: *mut gsl_odeiv2_driver,
    mut t: *mut libc::c_double,
    h: libc::c_double,
    n: u64,
    mut y: *mut libc::c_double,
) -> i32 {
    let mut i: u64 = 0;
    (*d).n = 0 as i32 as u64;
    i = 0 as i32 as u64;
    while i < n {
        let mut s: i32 = gsl_odeiv2_evolve_apply_fixed_step(
            (*d).e,
            (*d).c,
            (*d).s,
            (*d).sys,
            t,
            h,
            y,
        );
        if s != GSL_SUCCESS as i32 {
            return s;
        }
        (*d).n = ((*d).n).wrapping_add(1);
        (*d).n;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_reset(mut d: *mut gsl_odeiv2_driver) -> i32 {
    let mut s: i32 = gsl_odeiv2_evolve_reset((*d).e);
    if s != GSL_SUCCESS as i32 {
        return s;
    }
    let mut s_0: i32 = gsl_odeiv2_step_reset((*d).s);
    if s_0 != GSL_SUCCESS as i32 {
        return s_0;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_reset_hstart(
    mut d: *mut gsl_odeiv2_driver,
    hstart: libc::c_double,
) -> i32 {
    gsl_odeiv2_driver_reset(d);
    if (*d).hmin > fabs(hstart) || fabs(hstart) > (*d).hmax {
        gsl_error(
            b"hmin <= fabs(h) <= hmax required\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            466 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if hstart > 0.0f64 || hstart < 0.0f64 {
        (*d).h = hstart;
    } else {
        gsl_error(
            b"invalid hstart\0" as *const u8 as *const i8,
            b"driver.c\0" as *const u8 as *const i8,
            475 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv2_driver_free(mut state: *mut gsl_odeiv2_driver) {
    if !((*state).c).is_null() {
        gsl_odeiv2_control_free((*state).c);
    }
    if !((*state).e).is_null() {
        gsl_odeiv2_evolve_free((*state).e);
    }
    if !((*state).s).is_null() {
        gsl_odeiv2_step_free((*state).s);
    }
    free(state as *mut libc::c_void);
}