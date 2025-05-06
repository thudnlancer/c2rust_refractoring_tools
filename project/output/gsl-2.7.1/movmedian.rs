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
    fn gsl_movstat_apply_accum(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        accum: *const gsl_movstat_accum,
        accum_params: *mut libc::c_void,
        y: *mut gsl_vector,
        z: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> i32;
    static mut gsl_movstat_accum_median: *const gsl_movstat_accum;
}
pub type size_t = u64;
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
pub unsafe extern "C" fn gsl_movstat_median(
    endtype: gsl_movstat_end_t,
    mut x: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut w: *mut gsl_movstat_workspace,
) -> i32 {
    let mut status: i32 = gsl_movstat_apply_accum(
        endtype,
        x,
        gsl_movstat_accum_median,
        0 as *mut libc::c_void,
        y,
        0 as *mut gsl_vector,
        w,
    );
    return status;
}