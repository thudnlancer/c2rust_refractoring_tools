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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_coerce_double(x: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_odeiv_step_apply(
        s: *mut gsl_odeiv_step,
        t: libc::c_double,
        h: libc::c_double,
        y: *mut libc::c_double,
        yerr: *mut libc::c_double,
        dydt_in: *const libc::c_double,
        dydt_out: *mut libc::c_double,
        dydt: *const gsl_odeiv_system,
    ) -> i32;
    fn gsl_odeiv_control_hadjust(
        c: *mut gsl_odeiv_control,
        s: *mut gsl_odeiv_step,
        y: *const libc::c_double,
        yerr: *const libc::c_double,
        dydt: *const libc::c_double,
        h: *mut libc::c_double,
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
pub struct gsl_odeiv_system {
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
pub struct gsl_odeiv_step_type {
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
            *const gsl_odeiv_system,
        ) -> i32,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> i32>,
    pub order: Option<unsafe extern "C" fn(*mut libc::c_void) -> u32>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_step {
    pub type_0: *const gsl_odeiv_step_type,
    pub dimension: size_t,
    pub state: *mut libc::c_void,
}
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
pub struct gsl_odeiv_evolve {
    pub dimension: size_t,
    pub y0: *mut libc::c_double,
    pub yerr: *mut libc::c_double,
    pub dydt_in: *mut libc::c_double,
    pub dydt_out: *mut libc::c_double,
    pub last_step: libc::c_double,
    pub count: u64,
    pub failed_steps: u64,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_evolve_alloc(
    mut dim: size_t,
) -> *mut gsl_odeiv_evolve {
    let mut e: *mut gsl_odeiv_evolve = malloc(
        ::core::mem::size_of::<gsl_odeiv_evolve>() as u64,
    ) as *mut gsl_odeiv_evolve;
    if e.is_null() {
        gsl_error(
            b"failed to allocate space for evolve struct\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            40 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv_evolve;
    }
    (*e).y0 = malloc(dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64))
        as *mut libc::c_double;
    if ((*e).y0).is_null() {
        free(e as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            48 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv_evolve;
    }
    (*e).yerr = malloc(dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64))
        as *mut libc::c_double;
    if ((*e).yerr).is_null() {
        free((*e).y0 as *mut libc::c_void);
        free(e as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for yerr\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv_evolve;
    }
    (*e).dydt_in = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*e).dydt_in).is_null() {
        free((*e).yerr as *mut libc::c_void);
        free((*e).y0 as *mut libc::c_void);
        free(e as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dydt_in\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            67 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv_evolve;
    }
    (*e).dydt_out = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*e).dydt_out).is_null() {
        free((*e).dydt_in as *mut libc::c_void);
        free((*e).yerr as *mut libc::c_void);
        free((*e).y0 as *mut libc::c_void);
        free(e as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dydt_out\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            78 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_odeiv_evolve;
    }
    (*e).dimension = dim;
    (*e).count = 0 as i32 as u64;
    (*e).failed_steps = 0 as i32 as u64;
    (*e).last_step = 0.0f64;
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_evolve_reset(mut e: *mut gsl_odeiv_evolve) -> i32 {
    (*e).count = 0 as i32 as u64;
    (*e).failed_steps = 0 as i32 as u64;
    (*e).last_step = 0.0f64;
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_evolve_free(mut e: *mut gsl_odeiv_evolve) {
    if e.is_null() {
        return;
    }
    free((*e).dydt_out as *mut libc::c_void);
    free((*e).dydt_in as *mut libc::c_void);
    free((*e).yerr as *mut libc::c_void);
    free((*e).y0 as *mut libc::c_void);
    free(e as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_odeiv_evolve_apply(
    mut e: *mut gsl_odeiv_evolve,
    mut con: *mut gsl_odeiv_control,
    mut step: *mut gsl_odeiv_step,
    mut dydt: *const gsl_odeiv_system,
    mut t: *mut libc::c_double,
    mut t1: libc::c_double,
    mut h: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> i32 {
    let t0: libc::c_double = *t;
    let mut h0: libc::c_double = *h;
    let mut step_status: i32 = 0;
    let mut final_step: i32 = 0 as i32;
    let mut dt: libc::c_double = t1 - t0;
    if (*e).dimension != (*step).dimension {
        gsl_error(
            b"step dimension must match evolution size\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            128 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if dt < 0.0f64 && h0 > 0.0f64 || dt > 0.0f64 && h0 < 0.0f64 {
        gsl_error(
            b"step direction must match interval direction\0" as *const u8 as *const i8,
            b"evolve.c\0" as *const u8 as *const i8,
            133 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if !con.is_null() {
        memcpy(
            (*e).y0 as *mut libc::c_void,
            y as *const libc::c_void,
            ((*e).dimension)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
    }
    if (*(*step).type_0).can_use_dydt_in != 0 {
        let mut status: i32 = (Some(
            ((*dydt).function).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(t0, y as *const libc::c_double, (*e).dydt_in, (*dydt).params);
        if status != 0 {
            return status;
        }
    }
    loop {
        if dt >= 0.0f64 && h0 > dt || dt < 0.0f64 && h0 < dt {
            h0 = dt;
            final_step = 1 as i32;
        } else {
            final_step = 0 as i32;
        }
        if (*(*step).type_0).can_use_dydt_in != 0 {
            step_status = gsl_odeiv_step_apply(
                step,
                t0,
                h0,
                y,
                (*e).yerr,
                (*e).dydt_in as *const libc::c_double,
                (*e).dydt_out,
                dydt,
            );
        } else {
            step_status = gsl_odeiv_step_apply(
                step,
                t0,
                h0,
                y,
                (*e).yerr,
                0 as *const libc::c_double,
                (*e).dydt_out,
                dydt,
            );
        }
        if step_status != GSL_SUCCESS as i32 {
            *h = h0;
            *t = t0;
            return step_status;
        }
        (*e).count = ((*e).count).wrapping_add(1);
        (*e).count;
        (*e).last_step = h0;
        if final_step != 0 {
            *t = t1;
        } else {
            *t = t0 + h0;
        }
        if con.is_null() {
            break;
        }
        let mut h_old: libc::c_double = h0;
        let hadjust_status: i32 = gsl_odeiv_control_hadjust(
            con,
            step,
            y as *const libc::c_double,
            (*e).yerr as *const libc::c_double,
            (*e).dydt_out as *const libc::c_double,
            &mut h0,
        );
        if !(hadjust_status == -(1 as i32)) {
            break;
        }
        let mut t_curr: libc::c_double = gsl_coerce_double(*t);
        let mut t_next: libc::c_double = gsl_coerce_double(*t + h0);
        if fabs(h0) < fabs(h_old) && t_next != t_curr {
            memcpy(
                y as *mut libc::c_void,
                (*e).y0 as *const libc::c_void,
                ((*dydt).dimension)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            (*e).failed_steps = ((*e).failed_steps).wrapping_add(1);
            (*e).failed_steps;
        } else {
            h0 = h_old;
            break;
        }
    }
    *h = h0;
    return step_status;
}