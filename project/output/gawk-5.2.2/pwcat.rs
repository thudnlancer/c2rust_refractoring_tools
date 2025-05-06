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
    fn printf(_: *const i8, _: ...) -> i32;
    fn endpwent();
    fn getpwent() -> *mut passwd;
}
pub type __uid_t = u32;
pub type __gid_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut p: *mut passwd = 0 as *mut passwd;
    loop {
        p = getpwent();
        if p.is_null() {
            break;
        }
        printf(
            b"%s:%s:%ld:%ld:%s:%s:%s\n\0" as *const u8 as *const i8,
            (*p).pw_name,
            (*p).pw_passwd,
            (*p).pw_uid as i64,
            (*p).pw_gid as i64,
            (*p).pw_gecos,
            (*p).pw_dir,
            (*p).pw_shell,
        );
    }
    endpwent();
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}