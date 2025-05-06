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
    pub type pth_event_st;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pth_once(
        _: *mut pth_once_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> i32;
    fn pth_key_create(
        _: *mut pth_key_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
    fn pth_key_setdata(_: pth_key_t, _: *const libc::c_void) -> i32;
    fn pth_key_getdata(_: pth_key_t) -> *mut libc::c_void;
    fn pth_read_ev(_: i32, _: *mut libc::c_void, _: size_t, _: pth_event_t) -> ssize_t;
}
pub type size_t = u64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type pth_event_t = *mut pth_event_st;
pub type pth_key_t = i32;
pub type pth_once_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readline_buf {
    pub rl_cnt: i32,
    pub rl_bufptr: *mut i8,
    pub rl_buf: [i8; 1024],
}
static mut readline_key: pth_key_t = 0;
static mut readline_once_ctrl: pth_once_t = 0 as i32;
unsafe extern "C" fn readline_buf_destroy(mut vp: *mut libc::c_void) {
    free(vp);
}
unsafe extern "C" fn readline_init(mut vp: *mut libc::c_void) {
    pth_key_create(
        &mut readline_key,
        Some(readline_buf_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn pth_readline(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut buflen: size_t,
) -> ssize_t {
    return pth_readline_ev(fd, buf, buflen, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_readline_ev(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut buflen: size_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut n: size_t = 0;
    let mut rc: ssize_t = 0;
    let mut c: i8 = '\0' as i32 as i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut rl: *mut readline_buf = 0 as *mut readline_buf;
    pth_once(
        &mut readline_once_ctrl,
        Some(readline_init as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    rl = pth_key_getdata(readline_key) as *mut readline_buf;
    if rl.is_null() {
        rl = malloc(::core::mem::size_of::<readline_buf>() as u64) as *mut readline_buf;
        (*rl).rl_cnt = 0 as i32;
        (*rl).rl_bufptr = 0 as *mut i8;
        pth_key_setdata(readline_key, rl as *const libc::c_void);
    }
    cp = buf as *mut i8;
    n = 1 as i32 as size_t;
    while n < buflen {
        rc = 1 as i32 as ssize_t;
        if (*rl).rl_cnt <= 0 as i32 {
            (*rl).rl_cnt = pth_read_ev(
                fd,
                ((*rl).rl_buf).as_mut_ptr() as *mut libc::c_void,
                1024 as i32 as size_t,
                ev_extra,
            ) as i32;
            if (*rl).rl_cnt < 0 as i32 {
                rc = -(1 as i32) as ssize_t;
            } else if (*rl).rl_cnt == 0 as i32 {
                rc = 0 as i32 as ssize_t;
            } else {
                (*rl).rl_bufptr = ((*rl).rl_buf).as_mut_ptr();
            }
        }
        if rc == 1 as i32 as i64 {
            (*rl).rl_cnt -= 1;
            (*rl).rl_cnt;
            let fresh0 = (*rl).rl_bufptr;
            (*rl).rl_bufptr = ((*rl).rl_bufptr).offset(1);
            c = *fresh0;
        }
        if rc == 1 as i32 as i64 {
            if c as i32 == '\r' as i32 {
                n = n.wrapping_sub(1);
                n;
            } else {
                let fresh1 = cp;
                cp = cp.offset(1);
                *fresh1 = c;
                if c as i32 == '\n' as i32 {
                    break;
                }
            }
            n = n.wrapping_add(1);
            n;
        } else if rc == 0 as i32 as i64 {
            if !(n == 1 as i32 as u64) {
                break;
            }
            return 0 as i32 as ssize_t;
        } else {
            return -(1 as i32) as ssize_t
        }
    }
    *cp = '\0' as i32 as i8;
    return n as ssize_t;
}