#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type __dirstream;
    fn fdopendir(__fd: i32) -> *mut DIR;
    fn __errno_location() -> *mut i32;
    fn openat_safer(_: i32, _: *const i8, _: i32, _: ...) -> i32;
    fn close(__fd: i32) -> i32;
}
pub type DIR = __dirstream;
#[no_mangle]
pub unsafe extern "C" fn opendirat(
    mut dir_fd: i32,
    mut dir: *const i8,
    mut extra_flags: i32,
    mut pnew_fd: *mut i32,
) -> *mut DIR {
    let mut open_flags: i32 = 0 as i32 | 0o2000000 as i32 | 0o200000 as i32
        | 0o400 as i32 | 0o4000 as i32 | extra_flags;
    let mut new_fd: i32 = openat_safer(dir_fd, dir, open_flags);
    if new_fd < 0 as i32 {
        return 0 as *mut DIR;
    }
    let mut dirp: *mut DIR = fdopendir(new_fd);
    if !dirp.is_null() {
        *pnew_fd = new_fd;
    } else {
        let mut fdopendir_errno: i32 = *__errno_location();
        close(new_fd);
        *__errno_location() = fdopendir_errno;
    }
    return dirp;
}