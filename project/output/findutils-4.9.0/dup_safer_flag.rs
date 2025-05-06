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
pub unsafe extern "C" fn dup_safer_flag(mut fd: i32, mut flag: i32) -> i32 {
    return rpl_fcntl(
        fd,
        if flag & 0o2000000 as i32 != 0 { 1030 as i32 } else { 0 as i32 },
        2 as i32 + 1 as i32,
    );
}