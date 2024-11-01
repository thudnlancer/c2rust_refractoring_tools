#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rk2_state_t {
    pub k1: *mut libc::c_double,
    pub k2: *mut libc::c_double,
    pub k3: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
}
unsafe extern "C" fn stepper_set_driver_null(
    mut vstate: *mut libc::c_void,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut rk2_state_t = malloc(
        ::core::mem::size_of::<rk2_state_t>() as libc::c_ulong,
    ) as *mut rk2_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for rk2_state\0" as *const u8
                as *const libc::c_char,
            b"rk2.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k1 = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k1).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k1\0" as *const u8 as *const libc::c_char,
            b"rk2.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k2 = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k2).is_null() {
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k2\0" as *const u8 as *const libc::c_char,
            b"rk2.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k3 = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k3).is_null() {
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k3\0" as *const u8 as *const libc::c_char,
            b"rk2.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ytmp = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).ytmp).is_null() {
        free((*state).k3 as *mut libc::c_void);
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const libc::c_char,
            b"rk2.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rk2_apply(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut t: libc::c_double,
    mut h: libc::c_double,
    mut y: *mut libc::c_double,
    mut yerr: *mut libc::c_double,
    mut dydt_in: *const libc::c_double,
    mut dydt_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv2_system,
) -> libc::c_int {
    let mut state: *mut rk2_state_t = vstate as *mut rk2_state_t;
    let mut i: size_t = 0;
    let k1: *mut libc::c_double = (*state).k1;
    let k2: *mut libc::c_double = (*state).k2;
    let k3: *mut libc::c_double = (*state).k3;
    let ytmp: *mut libc::c_double = (*state).ytmp;
    if !dydt_in.is_null() {
        memcpy(
            k1 as *mut libc::c_void,
            dydt_in as *const libc::c_void,
            dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    } else {
        let mut s: libc::c_int = (Some(
            ((*sys).function).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(t, y as *const libc::c_double, k1, (*sys).params);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize) + 0.5f64 * h * *k1.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let mut s_0: libc::c_int = (Some(
        ((*sys).function).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(t + 0.5f64 * h, ytmp as *const libc::c_double, k2, (*sys).params);
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize)
            + h * (-*k1.offset(i as isize) + 2.0f64 * *k2.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_1: libc::c_int = (Some(
        ((*sys).function).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(t + h, ytmp as *const libc::c_double, k3, (*sys).params);
    if s_1 != GSL_SUCCESS as libc::c_int {
        return s_1;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize);
        let ksum3: libc::c_double = (*k1.offset(i as isize)
            + 4.0f64 * *k2.offset(i as isize) + *k3.offset(i as isize)) / 6.0f64;
        *y.offset(i as isize) += h * ksum3;
        i = i.wrapping_add(1);
        i;
    }
    if !dydt_out.is_null() {
        let mut s_2: libc::c_int = (Some(
            ((*sys).function).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_2 != GSL_SUCCESS as libc::c_int {
            memcpy(
                y as *mut libc::c_void,
                ytmp as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            return s_2;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        let ksum3_0: libc::c_double = (*k1.offset(i as isize)
            + 4.0f64 * *k2.offset(i as isize) + *k3.offset(i as isize)) / 6.0f64;
        *yerr.offset(i as isize) = h * (*k2.offset(i as isize) - ksum3_0);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut rk2_state_t = vstate as *mut rk2_state_t;
    memset(
        (*state).k1 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k2 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k3 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).ytmp as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut rk2_state_t = vstate as *mut rk2_state_t;
    state = 0 as *mut rk2_state_t;
    return 2 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rk2_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut rk2_state_t = vstate as *mut rk2_state_t;
    free((*state).k1 as *mut libc::c_void);
    free((*state).k2 as *mut libc::c_void);
    free((*state).k3 as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut rk2_type: gsl_odeiv2_step_type = {
    let mut init = gsl_odeiv2_step_type {
        name: b"rk2\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(rk2_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            rk2_apply
                as unsafe extern "C" fn(
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
        ),
        set_driver: Some(
            stepper_set_driver_null
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_odeiv2_driver,
                ) -> libc::c_int,
        ),
        reset: Some(
            rk2_reset as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            rk2_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(rk2_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
#[no_mangle]
pub static mut gsl_odeiv2_step_rk2: *const gsl_odeiv2_step_type = unsafe {
    &rk2_type as *const gsl_odeiv2_step_type
};
