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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    pub type pth_msgport_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut i8;
    fn pth_init() -> i32;
    fn pth_kill() -> i32;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: i32, _: ...) -> i32;
    fn pth_attr_destroy(_: pth_attr_t) -> i32;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_yield(_: pth_t) -> i32;
    fn pth_wait(_: pth_event_t) -> i32;
    fn pth_cancel(_: pth_t) -> i32;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> i32;
    fn pth_timeout(_: i64, _: i64) -> pth_time_t;
    fn pth_event(_: u64, _: ...) -> pth_event_t;
    fn pth_event_status(_: pth_event_t) -> pth_status_t;
    fn pth_event_free(_: pth_event_t, _: i32) -> i32;
    fn pth_msgport_create(_: *const i8) -> pth_msgport_t;
    fn pth_msgport_destroy(_: pth_msgport_t);
    fn pth_msgport_find(_: *const i8) -> pth_msgport_t;
    fn pth_msgport_put(_: pth_msgport_t, _: *mut pth_message_t) -> i32;
    fn pth_msgport_get(_: pth_msgport_t) -> *mut pth_message_t;
    fn pth_msgport_reply(_: *mut pth_message_t) -> i32;
    fn pth_cleanup_push(
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> i32;
    fn pth_sleep(_: u32) -> u32;
    fn pth_readline_ev(
        _: i32,
        _: *mut libc::c_void,
        _: size_t,
        _: pth_event_t,
    ) -> ssize_t;
}
pub type size_t = u64;
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pth_time_t = timeval;
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    PTH_ATTR_BOUND = 14,
    PTH_ATTR_EVENTS = 13,
    PTH_ATTR_STATE = 12,
    PTH_ATTR_START_ARG = 11,
    PTH_ATTR_START_FUNC = 10,
    PTH_ATTR_TIME_RAN = 9,
    PTH_ATTR_TIME_LAST = 8,
    PTH_ATTR_TIME_SPAWN = 7,
    PTH_ATTR_DISPATCHES = 6,
    PTH_ATTR_STACK_ADDR = 5,
    PTH_ATTR_STACK_SIZE = 4,
    PTH_ATTR_CANCEL_STATE = 3,
    PTH_ATTR_JOINABLE = 2,
    PTH_ATTR_NAME = 1,
    PTH_ATTR_PRIO = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::PTH_ATTR_BOUND => 14,
            C2RustUnnamed::PTH_ATTR_EVENTS => 13,
            C2RustUnnamed::PTH_ATTR_STATE => 12,
            C2RustUnnamed::PTH_ATTR_START_ARG => 11,
            C2RustUnnamed::PTH_ATTR_START_FUNC => 10,
            C2RustUnnamed::PTH_ATTR_TIME_RAN => 9,
            C2RustUnnamed::PTH_ATTR_TIME_LAST => 8,
            C2RustUnnamed::PTH_ATTR_TIME_SPAWN => 7,
            C2RustUnnamed::PTH_ATTR_DISPATCHES => 6,
            C2RustUnnamed::PTH_ATTR_STACK_ADDR => 5,
            C2RustUnnamed::PTH_ATTR_STACK_SIZE => 4,
            C2RustUnnamed::PTH_ATTR_CANCEL_STATE => 3,
            C2RustUnnamed::PTH_ATTR_JOINABLE => 2,
            C2RustUnnamed::PTH_ATTR_NAME => 1,
            C2RustUnnamed::PTH_ATTR_PRIO => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            14 => C2RustUnnamed::PTH_ATTR_BOUND,
            13 => C2RustUnnamed::PTH_ATTR_EVENTS,
            12 => C2RustUnnamed::PTH_ATTR_STATE,
            11 => C2RustUnnamed::PTH_ATTR_START_ARG,
            10 => C2RustUnnamed::PTH_ATTR_START_FUNC,
            9 => C2RustUnnamed::PTH_ATTR_TIME_RAN,
            8 => C2RustUnnamed::PTH_ATTR_TIME_LAST,
            7 => C2RustUnnamed::PTH_ATTR_TIME_SPAWN,
            6 => C2RustUnnamed::PTH_ATTR_DISPATCHES,
            5 => C2RustUnnamed::PTH_ATTR_STACK_ADDR,
            4 => C2RustUnnamed::PTH_ATTR_STACK_SIZE,
            3 => C2RustUnnamed::PTH_ATTR_CANCEL_STATE,
            2 => C2RustUnnamed::PTH_ATTR_JOINABLE,
            1 => C2RustUnnamed::PTH_ATTR_NAME,
            0 => C2RustUnnamed::PTH_ATTR_PRIO,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type pth_event_t = *mut pth_event_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    PTH_FREE_THIS,
    PTH_FREE_ALL,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::PTH_FREE_THIS => 0,
            C2RustUnnamed_0::PTH_FREE_ALL => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            0 => C2RustUnnamed_0::PTH_FREE_THIS,
            1 => C2RustUnnamed_0::PTH_FREE_ALL,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum pth_status_t {
    PTH_STATUS_PENDING,
    PTH_STATUS_OCCURRED,
    PTH_STATUS_FAILED,
}
impl pth_status_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            pth_status_t::PTH_STATUS_PENDING => 0,
            pth_status_t::PTH_STATUS_OCCURRED => 1,
            pth_status_t::PTH_STATUS_FAILED => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> pth_status_t {
        match value {
            0 => pth_status_t::PTH_STATUS_PENDING,
            1 => pth_status_t::PTH_STATUS_OCCURRED,
            2 => pth_status_t::PTH_STATUS_FAILED,
            _ => panic!("Invalid value for pth_status_t: {}", value),
        }
    }
}
impl AddAssign<u32> for pth_status_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for pth_status_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for pth_status_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for pth_status_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for pth_status_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for pth_status_t {
    type Output = pth_status_t;
    fn add(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for pth_status_t {
    type Output = pth_status_t;
    fn sub(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for pth_status_t {
    type Output = pth_status_t;
    fn mul(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for pth_status_t {
    type Output = pth_status_t;
    fn div(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for pth_status_t {
    type Output = pth_status_t;
    fn rem(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_ringnode_t = pth_ringnode_st;
pub type pth_msgport_t = *mut pth_msgport_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_message_st {
    pub m_node: pth_ringnode_t,
    pub m_replyport: pth_msgport_t,
    pub m_size: u32,
    pub m_data: *mut libc::c_void,
}
pub type pth_message_t = pth_message_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct query {
    pub head: pth_message_t,
    pub string: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_cleanup_t {
    pub mp: pth_msgport_t,
    pub ev: pth_event_t,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn worker_cleanup(mut arg: *mut libc::c_void) {
    let mut wc: *mut worker_cleanup_t = arg as *mut worker_cleanup_t;
    pth_event_free((*wc).ev, C2RustUnnamed_0::PTH_FREE_THIS as i32);
    pth_msgport_destroy((*wc).mp);
}
unsafe extern "C" fn worker(mut _dummy: *mut libc::c_void) -> *mut libc::c_void {
    let mut wc: worker_cleanup_t = worker_cleanup_t {
        mp: 0 as *mut pth_msgport_st,
        ev: 0 as *mut pth_event_st,
    };
    let mut mp: pth_msgport_t = 0 as *mut pth_msgport_st;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut q: *mut query = 0 as *mut query;
    let mut i: i32 = 0;
    fprintf(stderr, b"worker: start\n\0" as *const u8 as *const i8);
    mp = pth_msgport_create(b"worker\0" as *const u8 as *const i8);
    wc.mp = mp;
    ev = pth_event(((1 as i32) << 5 as i32) as u64, mp);
    wc.ev = ev;
    pth_cleanup_push(
        Some(worker_cleanup as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut wc as *mut worker_cleanup_t as *mut libc::c_void,
    );
    loop {
        i = pth_wait(ev);
        if i != 1 as i32 {
            continue;
        }
        loop {
            q = pth_msgport_get(mp) as *mut query;
            if q.is_null() {
                break;
            }
            fprintf(
                stderr,
                b"worker: recv query <%s>\n\0" as *const u8 as *const i8,
                (*q).string,
            );
            i = 0 as i32;
            while *((*q).string).offset(i as isize) as i32 != '\0' as i32 {
                *((*q).string).offset(i as isize) = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *((*q).string).offset(i as isize) as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*((*q).string).offset(i as isize) as i32);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*((*q).string).offset(i as isize) as i32 as isize);
                    }
                    __res
                }) as i8;
                i += 1;
                i;
            }
            fprintf(
                stderr,
                b"worker: send reply <%s>\n\0" as *const u8 as *const i8,
                (*q).string,
            );
            pth_msgport_reply(q as *mut pth_message_t);
        }
    };
}
unsafe extern "C" fn ticker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut now: time_t = 0;
    fprintf(stderr, b"ticker: start\n\0" as *const u8 as *const i8);
    loop {
        pth_sleep(5 as i32 as u32);
        now = time(0 as *mut time_t);
        fprintf(
            stderr,
            b"ticker was woken up on %s\0" as *const u8 as *const i8,
            ctime(&mut now),
        );
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut caLine: [i8; 1024] = [0; 1024];
    let mut ev: pth_event_t = 0 as pth_event_t;
    let mut evt: pth_event_t = 0 as pth_event_t;
    let mut t_worker: pth_t = 0 as pth_t;
    let mut t_ticker: pth_t = 0 as pth_t;
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut mp: pth_msgport_t = 0 as pth_msgport_t;
    let mut mp_worker: pth_msgport_t = 0 as pth_msgport_t;
    let mut q: *mut query = 0 as *mut query;
    let mut n: i32 = 0;
    if pth_init() == 0 {
        perror(b"pth_init\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    fprintf(
        stderr,
        b"This is TEST_MP, a Pth test using message ports.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"Lines on stdin are send to a worker thread via message\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"ports, translated to upper case by the worker thread and\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"send back to the main thread via message ports.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr,
        b"Additionally a useless ticker thread awakens every 5s.\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"Enter \"quit\" on stdin for stopping this test.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    t_attr = pth_attr_new();
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"worker\0" as *const u8 as *const i8,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_JOINABLE as i32,
        (0 as i32 == 0) as i32,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_STACK_SIZE as i32,
        16 as i32 * 1024 as i32,
    );
    t_worker = pth_spawn(
        t_attr,
        Some(worker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"ticker\0" as *const u8 as *const i8,
    );
    t_ticker = pth_spawn(
        t_attr,
        Some(ticker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pth_attr_destroy(t_attr);
    pth_yield(0 as pth_t);
    mp_worker = pth_msgport_find(b"worker\0" as *const u8 as *const i8);
    mp = pth_msgport_create(b"main\0" as *const u8 as *const i8);
    q = malloc(::core::mem::size_of::<query>() as u64) as *mut query;
    ev = pth_event(((1 as i32) << 5 as i32) as u64, mp);
    evt = 0 as pth_event_t;
    loop {
        if evt.is_null() {
            evt = pth_event(
                ((1 as i32) << 4 as i32) as u64,
                pth_timeout(20 as i32 as i64, 0 as i32 as i64),
            );
        } else {
            evt = pth_event(
                ((1 as i32) << 4 as i32 | (1 as i32) << 20 as i32) as u64,
                evt,
                pth_timeout(20 as i32 as i64, 0 as i32 as i64),
            );
        }
        n = pth_readline_ev(
            0 as i32,
            caLine.as_mut_ptr() as *mut libc::c_void,
            1024 as i32 as size_t,
            evt,
        ) as i32;
        if n == -(1 as i32)
            && pth_event_status(evt) as u32
                == pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
        {
            fprintf(
                stderr,
                b"main: Hey, what are you waiting for? Type in something!\n\0"
                    as *const u8 as *const i8,
            );
        } else if n < 0 as i32 {
            fprintf(
                stderr,
                b"main: I/O read error on stdin\n\0" as *const u8 as *const i8,
            );
            break;
        } else if n == 0 as i32 {
            fprintf(stderr, b"main: EOF on stdin\n\0" as *const u8 as *const i8);
            break;
        } else {
            caLine[(n - 1 as i32) as usize] = '\0' as i32 as i8;
            if strcmp(caLine.as_mut_ptr(), b"quit\0" as *const u8 as *const i8)
                == 0 as i32
            {
                fprintf(stderr, b"main: quit\n\0" as *const u8 as *const i8);
                break;
            } else {
                fprintf(
                    stderr,
                    b"main: out --> <%s>\n\0" as *const u8 as *const i8,
                    caLine.as_mut_ptr(),
                );
                (*q).string = caLine.as_mut_ptr();
                (*q).head.m_replyport = mp;
                pth_msgport_put(mp_worker, q as *mut pth_message_t);
                pth_wait(ev);
                q = pth_msgport_get(mp) as *mut query;
                fprintf(
                    stderr,
                    b"main: in <-- <%s>\n\0" as *const u8 as *const i8,
                    (*q).string,
                );
            }
        }
    }
    free(q as *mut libc::c_void);
    pth_event_free(ev, C2RustUnnamed_0::PTH_FREE_THIS as i32);
    pth_event_free(evt, C2RustUnnamed_0::PTH_FREE_THIS as i32);
    pth_msgport_destroy(mp);
    pth_cancel(t_worker);
    pth_join(t_worker, 0 as *mut *mut libc::c_void);
    pth_cancel(t_ticker);
    pth_join(t_ticker, 0 as *mut *mut libc::c_void);
    pth_kill();
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