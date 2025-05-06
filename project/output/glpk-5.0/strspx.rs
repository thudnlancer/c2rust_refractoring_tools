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
pub unsafe extern "C" fn _glp_strspx(mut str: *mut i8) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut t: *mut i8 = 0 as *mut i8;
    t = str;
    s = t;
    while *s != 0 {
        if *s as i32 != ' ' as i32 {
            let fresh0 = t;
            t = t.offset(1);
            *fresh0 = *s;
        }
        s = s.offset(1);
        s;
    }
    *t = '\0' as i32 as i8;
    return str;
}