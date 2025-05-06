#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_xmax(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let nx: i32 = (*h).nx as i32;
    return *((*h).xrange).offset(nx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_xmin(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    return *((*h).xrange).offset(0 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_ymax(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let ny: i32 = (*h).ny as i32;
    return *((*h).yrange).offset(ny as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_ymin(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    return *((*h).yrange).offset(0 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_nx(mut h: *const gsl_histogram2d) -> size_t {
    return (*h).nx;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_ny(mut h: *const gsl_histogram2d) -> size_t {
    return (*h).ny;
}