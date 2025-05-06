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
    fn _glp_put_err_msg(msg: *const i8);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dlopen(mut module: *const i8) -> *mut libc::c_void {
    (module == module
        || {
            glp_assert_(
                b"module == module\0" as *const u8 as *const i8,
                b"env/dlsup.c\0" as *const u8 as *const i8,
                145 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_put_err_msg(b"Shared libraries not supported\0" as *const u8 as *const i8);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dlsym(
    mut h: *mut libc::c_void,
    mut symbol: *const i8,
) -> *mut libc::c_void {
    (h != h
        || {
            glp_assert_(
                b"h != h\0" as *const u8 as *const i8,
                b"env/dlsup.c\0" as *const u8 as *const i8,
                152 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (symbol != symbol
        || {
            glp_assert_(
                b"symbol != symbol\0" as *const u8 as *const i8,
                b"env/dlsup.c\0" as *const u8 as *const i8,
                153 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_dlclose(mut h: *mut libc::c_void) {
    (h != h
        || {
            glp_assert_(
                b"h != h\0" as *const u8 as *const i8,
                b"env/dlsup.c\0" as *const u8 as *const i8,
                159 as i32,
            );
            1 as i32 != 0
        }) as i32;
}