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
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram_max(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let n: i32 = (*h).n as i32;
    return *((*h).range).offset(n as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram_min(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    return *((*h).range).offset(0 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram_bins(mut h: *const gsl_histogram) -> size_t {
    return (*h).n;
}