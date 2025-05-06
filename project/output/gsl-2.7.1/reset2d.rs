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
pub unsafe extern "C" fn gsl_histogram2d_reset(mut h: *mut gsl_histogram2d) {
    let mut i: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    i = 0 as i32 as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as i32 as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
}