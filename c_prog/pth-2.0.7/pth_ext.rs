#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type Sfdisc_t = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn pth_sfiodisc() -> *mut Sfdisc_t {
    *__errno_location() = 38 as libc::c_int;
    return 0 as *mut libc::c_void as *mut Sfdisc_t;
}
