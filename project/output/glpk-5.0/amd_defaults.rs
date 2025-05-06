#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_defaults(mut Control: *mut libc::c_double) {
    let mut i: i32 = 0;
    if !Control.is_null() {
        i = 0 as i32;
        while i < 5 as i32 {
            *Control.offset(i as isize) = 0 as i32 as libc::c_double;
            i += 1;
            i;
        }
        *Control.offset(0 as i32 as isize) = 10.0f64;
        *Control.offset(1 as i32 as isize) = 1 as i32 as libc::c_double;
    }
}