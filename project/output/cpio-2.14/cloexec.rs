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
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
}
#[no_mangle]
pub unsafe extern "C" fn set_cloexec_flag(mut desc: i32, mut value: bool) -> i32 {
    let mut flags: i32 = rpl_fcntl(desc, 1 as i32, 0 as i32);
    if 0 as i32 <= flags {
        let mut newflags: i32 = if value as i32 != 0 {
            flags | 1 as i32
        } else {
            flags & !(1 as i32)
        };
        if flags == newflags || rpl_fcntl(desc, 2 as i32, newflags) != -(1 as i32) {
            return 0 as i32;
        }
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn dup_cloexec(mut fd: i32) -> i32 {
    return rpl_fcntl(fd, 1030 as i32, 0 as i32);
}