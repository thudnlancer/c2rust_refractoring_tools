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
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror_r(__errnum: i32, __buf: *mut i8, __buflen: size_t) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn __xpg_strerror_r(errnum: i32, buf: *mut i8, buflen: size_t) -> i32;
}
pub type size_t = u64;
unsafe extern "C" fn safe_copy(
    mut buf: *mut i8,
    mut buflen: size_t,
    mut msg: *const i8,
) -> i32 {
    let mut len: size_t = strlen(msg);
    let mut moved: size_t = if len < buflen {
        len
    } else {
        buflen.wrapping_sub(1 as i32 as u64)
    };
    memmove(buf as *mut libc::c_void, msg as *const libc::c_void, moved);
    *buf.offset(moved as isize) = '\0' as i32 as i8;
    return if len < buflen { 0 as i32 } else { 34 as i32 };
}
#[no_mangle]
pub unsafe extern "C" fn rpl_strerror_r(
    mut errnum: i32,
    mut buf: *mut i8,
    mut buflen: size_t,
) -> i32 {
    if buflen <= 1 as i32 as u64 {
        if buflen != 0 {
            *buf = '\0' as i32 as i8;
        }
        return 34 as i32;
    }
    *buf = '\0' as i32 as i8;
    let mut msg: *const i8 = 0 as *const i8;
    if !msg.is_null() {
        return safe_copy(buf, buflen, msg);
    }
    let mut ret: i32 = 0;
    let mut saved_errno: i32 = *__errno_location();
    ret = 0 as i32;
    ret = __xpg_strerror_r(errnum, buf, buflen);
    if *buf == 0 {
        let mut stackbuf: [i8; 80] = [0; 80];
        let mut errstring: *mut i8 = strerror_r(
            errnum,
            stackbuf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 80]>() as u64,
        );
        ret = if !errstring.is_null() {
            safe_copy(buf, buflen, errstring)
        } else {
            *__errno_location()
        };
    }
    if ret == 22 as i32 && *buf == 0 {
        snprintf(buf, buflen, b"Unknown error %d\0" as *const u8 as *const i8, errnum);
    }
    *__errno_location() = saved_errno;
    return ret;
}