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
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigdelset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigismember(__set: *const sigset_t, __signo: i32) -> i32;
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
    fn pth_cancel(_: pth_t) -> i32;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> i32;
    fn pth_cleanup_push(
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> i32;
    fn pth_sleep(_: u32) -> u32;
    fn pth_sigmask(_: i32, _: *const sigset_t, _: *mut sigset_t) -> i32;
    fn pth_sigwait(_: *const sigset_t, _: *mut i32) -> i32;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
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
static mut child1: pth_t = 0 as *const pth_st as *mut pth_st;
static mut child2: pth_t = 0 as *const pth_st as *mut pth_st;
unsafe extern "C" fn inthandler(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: i32 = 0;
    let mut n: i32 = 0;
    fprintf(stderr, b"inthandler: enter\n\0" as *const u8 as *const i8);
    sigemptyset(&mut sigs);
    sigaddset(&mut sigs, 2 as i32);
    pth_sigmask(1 as i32, &mut sigs, 0 as *mut sigset_t);
    n = 0 as i32;
    while n < 3 as i32 {
        pth_sigwait(&mut sigs, &mut sig);
        fprintf(
            stderr,
            b"inthandler: SIGINT received (#%d)\n\0" as *const u8 as *const i8,
            n,
        );
        n += 1;
        n;
    }
    fprintf(
        stderr,
        b"inthandler: cancelling child1 and child2\n\0" as *const u8 as *const i8,
    );
    pth_cancel(child1);
    pth_cancel(child2);
    fprintf(stderr, b"inthandler: leave\n\0" as *const u8 as *const i8);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn child_cleanup(mut arg: *mut libc::c_void) {
    fprintf(
        stderr,
        b"%s: running cleanup\n\0" as *const u8 as *const i8,
        arg as *mut i8,
    );
}
unsafe extern "C" fn child(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut name: *mut i8 = _arg as *mut i8;
    let mut i: i32 = 0;
    fprintf(stderr, b"%s: enter\n\0" as *const u8 as *const i8, name);
    pth_cleanup_push(
        Some(child_cleanup as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        name as *mut libc::c_void,
    );
    pth_sigmask(2 as i32, 0 as *const sigset_t, &mut sigs);
    sigaddset(&mut sigs, 2 as i32);
    if strcmp(name, b"child1\0" as *const u8 as *const i8) == 0 as i32 {
        sigaddset(&mut sigs, 10 as i32);
        sigdelset(&mut sigs, 12 as i32);
    } else {
        sigdelset(&mut sigs, 10 as i32);
        sigaddset(&mut sigs, 12 as i32);
    }
    pth_sigmask(2 as i32, &mut sigs, 0 as *mut sigset_t);
    i = 0 as i32;
    while i < 10 as i32 {
        pth_sigmask(2 as i32, 0 as *const sigset_t, &mut sigs);
        fprintf(
            stderr,
            b"%s: SIGUSR1: %s\n\0" as *const u8 as *const i8,
            name,
            if sigismember(&mut sigs, 10 as i32) != 0 {
                b"blocked\0" as *const u8 as *const i8
            } else {
                b"unblocked\0" as *const u8 as *const i8
            },
        );
        fprintf(
            stderr,
            b"%s: SIGUSR2: %s\n\0" as *const u8 as *const i8,
            name,
            if sigismember(&mut sigs, 12 as i32) != 0 {
                b"blocked\0" as *const u8 as *const i8
            } else {
                b"unblocked\0" as *const u8 as *const i8
            },
        );
        fprintf(stderr, b"%s: leave to scheduler\n\0" as *const u8 as *const i8, name);
        pth_sleep(1 as i32 as u32);
        fprintf(
            stderr,
            b"%s: reentered from scheduler\n\0" as *const u8 as *const i8,
            name,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"%s: leave\n\0" as *const u8 as *const i8, name);
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    pth_init();
    fprintf(
        stderr,
        b"This is TEST_SIG, a Pth test using signals.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"Hit CTRL-C three times to stop this test.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr,
        b"But only after all threads were terminated.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(stderr, b"main: init\n\0" as *const u8 as *const i8);
    pth_sigmask(2 as i32, 0 as *const sigset_t, &mut sigs);
    sigaddset(&mut sigs, 10 as i32);
    sigaddset(&mut sigs, 12 as i32);
    sigaddset(&mut sigs, 2 as i32);
    pth_sigmask(2 as i32, &mut sigs, 0 as *mut sigset_t);
    attr = pth_attr_new();
    pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"child1\0" as *const u8 as *const i8,
    );
    child1 = pth_spawn(
        attr,
        Some(child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"child1\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"child2\0" as *const u8 as *const i8,
    );
    child2 = pth_spawn(
        attr,
        Some(child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"child2\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"inthandler\0" as *const u8 as *const i8,
    );
    pth_spawn(
        attr,
        Some(inthandler as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"inthandler\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_destroy(attr);
    while pth_join(0 as pth_t, 0 as *mut *mut libc::c_void) != 0 {}
    fprintf(stderr, b"main: exit\n\0" as *const u8 as *const i8);
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