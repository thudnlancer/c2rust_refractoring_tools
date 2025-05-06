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
    fn __errno_location() -> *mut i32;
}
#[no_mangle]
pub unsafe extern "C" fn xstrtod(
    mut str: *const i8,
    mut ptr: *mut *const i8,
    mut result: *mut libc::c_double,
    mut convert: Option<unsafe extern "C" fn(*const i8, *mut *mut i8) -> libc::c_double>,
) -> bool {
    let mut val: libc::c_double = 0.;
    let mut terminator: *mut i8 = 0 as *mut i8;
    let mut ok: bool = 1 as i32 != 0;
    *__errno_location() = 0 as i32;
    val = convert.expect("non-null function pointer")(str, &mut terminator);
    if terminator == str as *mut i8 || ptr.is_null() && *terminator as i32 != '\0' as i32
    {
        ok = 0 as i32 != 0;
    } else if val != 0 as i32 as libc::c_double && *__errno_location() == 34 as i32 {
        ok = 0 as i32 != 0;
    }
    if !ptr.is_null() {
        *ptr = terminator;
    }
    *result = val;
    return ok;
}