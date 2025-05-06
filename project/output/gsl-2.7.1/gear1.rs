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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
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
pub struct gear1_state_t {
    pub k: *mut libc::c_double,
    pub y0: *mut libc::c_double,
    pub y0_orig: *mut libc::c_double,
    pub y_onestep: *mut libc::c_double,
}
unsafe extern "C" fn gear1_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut gear1_state_t = malloc(
        ::core::mem::size_of::<gear1_state_t>() as u64,
    ) as *mut gear1_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for gear1_state\0" as *const u8 as *const i8,
            b"gear1.c\0" as *const u8 as *const i8,
            56 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).k = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*state).k).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k\0" as *const u8 as *const i8,
            b"gear1.c\0" as *const u8 as *const i8,
            64 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).y0 = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*state).y0).is_null() {
        free((*state).k as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0\0" as *const u8 as *const i8,
            b"gear1.c\0" as *const u8 as *const i8,
            73 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).y0_orig = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*state).y0_orig).is_null() {
        free((*state).y0 as *mut libc::c_void);
        free((*state).k as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0_orig\0" as *const u8 as *const i8,
            b"gear1.c\0" as *const u8 as *const i8,
            83 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).y_onestep = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*state).y_onestep).is_null() {
        free((*state).y0_orig as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free((*state).k as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_onestep\0" as *const u8 as *const i8,
            b"gear1.c\0" as *const u8 as *const i8,
            94 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn gear1_step(
    mut y: *mut libc::c_double,
    mut state: *mut gear1_state_t,
    h: libc::c_double,
    t: libc::c_double,
    dim: size_t,
    mut sys: *const gsl_odeiv_system,
) -> i32 {
    let iter_steps: i32 = 3 as i32;
    let mut nu: i32 = 0;
    let mut i: size_t = 0;
    let mut y0: *mut libc::c_double = (*state).y0;
    let mut k: *mut libc::c_double = (*state).k;
    nu = 0 as i32;
    while nu < iter_steps {
        let mut s: i32 = (Some(((*sys).function).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(t + h, y as *const libc::c_double, k, (*sys).params);
        if s != GSL_SUCCESS as i32 {
            return s;
        }
        i = 0 as i32 as size_t;
        while i < dim {
            *y.offset(i as isize) = *y0.offset(i as isize) + h * *k.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        nu += 1;
        nu;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn gear1_apply(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut t: libc::c_double,
    mut h: libc::c_double,
    mut y: *mut libc::c_double,
    mut yerr: *mut libc::c_double,
    mut dydt_in: *const libc::c_double,
    mut dydt_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv_system,
) -> i32 {
    let mut state: *mut gear1_state_t = vstate as *mut gear1_state_t;
    let mut i: size_t = 0;
    let mut y0: *mut libc::c_double = (*state).y0;
    let mut y0_orig: *mut libc::c_double = (*state).y0_orig;
    let mut y_onestep: *mut libc::c_double = (*state).y_onestep;
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    memcpy(
        y0_orig as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    memcpy(
        y_onestep as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    let mut s: i32 = gear1_step(y_onestep, state, h, t, dim, sys);
    if s != GSL_SUCCESS as i32 {
        return s;
    }
    let mut s_0: i32 = gear1_step(y, state, h / 2.0f64, t, dim, sys);
    if s_0 != GSL_SUCCESS as i32 {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
        return s_0;
    }
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    let mut s_1: i32 = gear1_step(y, state, h / 2.0f64, t + h / 2.0f64, dim, sys);
    if s_1 != GSL_SUCCESS as i32 {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
        return s_1;
    }
    if !dydt_out.is_null() {
        let mut s_2: i32 = (Some(((*sys).function).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_2 != GSL_SUCCESS as i32 {
            memcpy(
                y as *mut libc::c_void,
                y0_orig as *const libc::c_void,
                dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            return s_2;
        }
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *yerr.offset(i as isize) = 4.0f64
            * (*y.offset(i as isize) - *y_onestep.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn gear1_reset(mut vstate: *mut libc::c_void, mut dim: size_t) -> i32 {
    let mut state: *mut gear1_state_t = vstate as *mut gear1_state_t;
    memset(
        (*state).y_onestep as *mut libc::c_void,
        0 as i32,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    memset(
        (*state).y0_orig as *mut libc::c_void,
        0 as i32,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    memset(
        (*state).y0 as *mut libc::c_void,
        0 as i32,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    memset(
        (*state).k as *mut libc::c_void,
        0 as i32,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn gear1_order(mut vstate: *mut libc::c_void) -> u32 {
    let mut state: *mut gear1_state_t = vstate as *mut gear1_state_t;
    state = 0 as *mut gear1_state_t;
    return 1 as i32 as u32;
}
unsafe extern "C" fn gear1_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut gear1_state_t = vstate as *mut gear1_state_t;
    free((*state).y_onestep as *mut libc::c_void);
    free((*state).y0_orig as *mut libc::c_void);
    free((*state).y0 as *mut libc::c_void);
    free((*state).k as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut gear1_type: gsl_odeiv_step_type = {
    let mut init = gsl_odeiv_step_type {
        name: b"gear1\0" as *const u8 as *const i8,
        can_use_dydt_in: 0 as i32,
        gives_exact_dydt_out: 1 as i32,
        alloc: Some(gear1_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            gear1_apply
                as unsafe extern "C" fn(
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
        ),
        reset: Some(
            gear1_reset as unsafe extern "C" fn(*mut libc::c_void, size_t) -> i32,
        ),
        order: Some(gear1_order as unsafe extern "C" fn(*mut libc::c_void) -> u32),
        free: Some(gear1_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
#[no_mangle]
pub static mut gsl_odeiv_step_gear1: *const gsl_odeiv_step_type = unsafe {
    &gear1_type as *const gsl_odeiv_step_type
};