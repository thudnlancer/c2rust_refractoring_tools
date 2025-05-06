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
    fn gsl_finite(x: libc::c_double) -> i32;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_fdf_struct {
    pub f: Option<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub df: Option<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub fdf: Option<
        unsafe extern "C" fn(
            libc::c_double,
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function_fdf = gsl_function_fdf_struct;
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
pub struct gsl_root_fdfsolver_type {
    pub name: *const i8,
    pub size: size_t,
    pub set: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function_fdf,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub iterate: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function_fdf,
            *mut libc::c_double,
        ) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct steffenson_state_t {
    pub f: libc::c_double,
    pub df: libc::c_double,
    pub x: libc::c_double,
    pub x_1: libc::c_double,
    pub x_2: libc::c_double,
    pub count: i32,
}
unsafe extern "C" fn steffenson_init(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_function_fdf,
    mut root: *mut libc::c_double,
) -> i32 {
    let mut state: *mut steffenson_state_t = vstate as *mut steffenson_state_t;
    let x: libc::c_double = *root;
    (*state).f = (Some(((*fdf).f).expect("non-null function pointer")))
        .expect("non-null function pointer")(x, (*fdf).params);
    (*state).df = (Some(((*fdf).df).expect("non-null function pointer")))
        .expect("non-null function pointer")(x, (*fdf).params);
    (*state).x = x;
    (*state).x_1 = 0.0f64;
    (*state).x_2 = 0.0f64;
    (*state).count = 1 as i32;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn steffenson_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_function_fdf,
    mut root: *mut libc::c_double,
) -> i32 {
    let mut state: *mut steffenson_state_t = vstate as *mut steffenson_state_t;
    let mut x_new: libc::c_double = 0.;
    let mut f_new: libc::c_double = 0.;
    let mut df_new: libc::c_double = 0.;
    let mut x_1: libc::c_double = (*state).x_1;
    let mut x: libc::c_double = (*state).x;
    if (*state).df == 0.0f64 {
        gsl_error(
            b"derivative is zero\0" as *const u8 as *const i8,
            b"steffenson.c\0" as *const u8 as *const i8,
            94 as i32,
            GSL_EZERODIV as i32,
        );
        return GSL_EZERODIV as i32;
    }
    x_new = x - (*state).f / (*state).df;
    (Some(((*fdf).fdf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(x_new, (*fdf).params, &mut f_new, &mut df_new);
    (*state).x_2 = x_1;
    (*state).x_1 = x;
    (*state).x = x_new;
    (*state).f = f_new;
    (*state).df = df_new;
    if gsl_finite(f_new) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const i8,
            b"steffenson.c\0" as *const u8 as *const i8,
            110 as i32,
            GSL_EBADFUNC as i32,
        );
        return GSL_EBADFUNC as i32;
    }
    if (*state).count < 3 as i32 {
        *root = x_new;
        (*state).count += 1;
        (*state).count;
    } else {
        let mut u: libc::c_double = x - x_1;
        let mut v: libc::c_double = x_new - 2 as i32 as libc::c_double * x + x_1;
        if v == 0 as i32 as libc::c_double {
            *root = x_new;
        } else {
            *root = x_1 - u * u / v;
        }
    }
    if gsl_finite(df_new) == 0 {
        gsl_error(
            b"derivative value is not finite\0" as *const u8 as *const i8,
            b"steffenson.c\0" as *const u8 as *const i8,
            131 as i32,
            GSL_EBADFUNC as i32,
        );
        return GSL_EBADFUNC as i32;
    }
    return GSL_SUCCESS as i32;
}
static mut steffenson_type: gsl_root_fdfsolver_type = {
    let mut init = gsl_root_fdfsolver_type {
        name: b"steffenson\0" as *const u8 as *const i8,
        size: ::core::mem::size_of::<steffenson_state_t>() as u64,
        set: Some(
            steffenson_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function_fdf,
                    *mut libc::c_double,
                ) -> i32,
        ),
        iterate: Some(
            steffenson_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function_fdf,
                    *mut libc::c_double,
                ) -> i32,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_root_fdfsolver_steffenson: *const gsl_root_fdfsolver_type = unsafe {
    &steffenson_type as *const gsl_root_fdfsolver_type
};