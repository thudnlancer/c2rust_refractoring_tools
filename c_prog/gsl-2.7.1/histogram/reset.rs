#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram_reset(mut h: *mut gsl_histogram) {
    let mut i: size_t = 0;
    let n: size_t = (*h).n;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *((*h).bin).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
}
