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
    fn rpl_dirfd(_: *mut DIR) -> i32;
    fn opendir(__name: *const i8) -> *mut DIR;
    fn fdopendir(__fd: i32) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn __errno_location() -> *mut i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    fn close(__fd: i32) -> i32;
}
pub type DIR = __dirstream;
#[no_mangle]
pub unsafe extern "C" fn opendir_safer(mut name: *const i8) -> *mut DIR {
    let mut dp: *mut DIR = opendir(name);
    if !dp.is_null() {
        let mut fd: i32 = rpl_dirfd(dp);
        if 0 as i32 <= fd && fd <= 2 as i32 {
            let mut newdp: *mut DIR = 0 as *mut DIR;
            let mut e: i32 = 0;
            let mut f: i32 = rpl_fcntl(fd, 1030 as i32, 2 as i32 + 1 as i32);
            if f < 0 as i32 {
                e = *__errno_location();
                newdp = 0 as *mut DIR;
            } else {
                newdp = fdopendir(f);
                e = *__errno_location();
                if newdp.is_null() {
                    close(f);
                }
            }
            closedir(dp);
            *__errno_location() = e;
            dp = newdp;
        }
    }
    return dp;
}