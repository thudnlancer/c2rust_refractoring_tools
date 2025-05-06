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
    fn gsl_movstat_apply_accum(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        accum: *const gsl_movstat_accum,
        accum_params: *mut libc::c_void,
        y: *mut gsl_vector,
        z: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> i32;
    static mut gsl_movstat_accum_qqr: *const gsl_movstat_accum;
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
pub type gsl_movstat_end_t = u32;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_accum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32>,
    pub insert: Option<unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> i32>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub get: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_workspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut libc::c_double,
    pub state: *mut libc::c_void,
    pub state_size: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_movstat_qqr(
    endtype: gsl_movstat_end_t,
    mut x: *const gsl_vector,
    q: libc::c_double,
    mut xqqr: *mut gsl_vector,
    mut w: *mut gsl_movstat_workspace,
) -> i32 {
    if (*x).size != (*xqqr).size {
        gsl_error(
            b"x and xqqr vectors must have same length\0" as *const u8 as *const i8,
            b"movqqr.c\0" as *const u8 as *const i8,
            50 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if q < 0.0f64 || q > 0.5f64 {
        gsl_error(
            b"q must be between 0 and 0.5\0" as *const u8 as *const i8,
            b"movqqr.c\0" as *const u8 as *const i8,
            54 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        let mut qq: libc::c_double = q;
        let mut status: i32 = gsl_movstat_apply_accum(
            endtype,
            x,
            gsl_movstat_accum_qqr,
            &mut qq as *mut libc::c_double as *mut libc::c_void,
            xqqr,
            0 as *mut gsl_vector,
            w,
        );
        return status;
    };
}