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
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_strtrim(mut str: *mut i8) -> *mut i8 {
    let mut t: *mut i8 = 0 as *mut i8;
    t = (strrchr(str, '\0' as i32)).offset(-(1 as i32 as isize));
    while t >= str {
        if *t as i32 != ' ' as i32 {
            break;
        }
        *t = '\0' as i32 as i8;
        t = t.offset(-1);
        t;
    }
    return str;
}