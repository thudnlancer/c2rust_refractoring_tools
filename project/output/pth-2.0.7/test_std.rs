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
    fn exit(_: i32) -> !;
    fn __errno_location() -> *mut i32;
    fn pth_init() -> i32;
    fn pth_kill() -> i32;
    fn pth_version() -> i64;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: i32, _: ...) -> i32;
    fn pth_attr_destroy(_: pth_attr_t) -> i32;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_yield(_: pth_t) -> i32;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> i32;
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
unsafe extern "C" fn t1_func(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: i32 = 0;
    let mut val: i64 = 0;
    val = arg as i64;
    if val != 123 as i32 as i64 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    i = 0 as i32;
    while i < 100 as i32 {
        val += 10 as i32 as i64;
        pth_yield(0 as pth_t);
        i += 1;
        i;
    }
    return val as *mut libc::c_void;
}
unsafe extern "C" fn t2_func(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut val: i64 = 0;
    let mut tid: pth_t = 0 as *mut pth_st;
    let mut rval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc: i32 = 0;
    val = arg as i64;
    if val < 9 as i32 as i64 {
        val += 1;
        val;
        fprintf(stderr, b"Spawning thread %ld\n\0" as *const u8 as *const i8, val);
        tid = pth_spawn(
            0 as pth_attr_t,
            Some(
                t2_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            val as *mut libc::c_void,
        );
        if tid.is_null() {
            fprintf(
                stderr,
                b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
                *__errno_location(),
            );
            exit(1 as i32);
        }
        rc = pth_join(tid, &mut rval);
        fprintf(stderr, b"Joined thread %ld\n\0" as *const u8 as *const i8, val);
        if rc == 0 as i32 {
            fprintf(
                stderr,
                b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
                *__errno_location(),
            );
            exit(1 as i32);
        }
        rval = (arg as i64 * rval as i64) as *mut libc::c_void;
    } else {
        rval = arg;
    }
    return rval;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    fprintf(
        stderr,
        b"\n=== TESTING GLOBAL LIBRARY API ===\n\n\0" as *const u8 as *const i8,
    );
    let mut version: i32 = 0;
    fprintf(stderr, b"Fetching library version\n\0" as *const u8 as *const i8);
    version = pth_version() as i32;
    if version == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    fprintf(stderr, b"version = 0x%X\n\0" as *const u8 as *const i8, version);
    fprintf(
        stderr,
        b"\n=== TESTING BASIC OPERATION ===\n\n\0" as *const u8 as *const i8,
    );
    let mut rc: i32 = 0;
    fprintf(
        stderr,
        b"Initializing Pth system (spawns scheduler and main thread)\n\0" as *const u8
            as *const i8,
    );
    rc = pth_init();
    if rc == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    fprintf(
        stderr,
        b"Killing Pth system for testing purposes\n\0" as *const u8 as *const i8,
    );
    pth_kill();
    fprintf(stderr, b"Re-Initializing Pth system\n\0" as *const u8 as *const i8);
    rc = pth_init();
    if rc == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    fprintf(
        stderr,
        b"\n=== TESTING BASIC THREAD OPERATION ===\n\n\0" as *const u8 as *const i8,
    );
    let mut attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut tid: pth_t = 0 as *mut pth_st;
    let mut val: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc_0: i32 = 0;
    fprintf(stderr, b"Creating attribute object\n\0" as *const u8 as *const i8);
    attr = pth_attr_new();
    if attr.is_null() {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    rc_0 = pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"test1\0" as *const u8 as *const i8,
    );
    if rc_0 == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    rc_0 = pth_attr_set(attr, C2RustUnnamed::PTH_ATTR_PRIO as i32, 5 as i32);
    if rc_0 == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    fprintf(stderr, b"Spawning thread\n\0" as *const u8 as *const i8);
    tid = pth_spawn(
        attr,
        Some(t1_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        123 as i32 as *mut libc::c_void,
    );
    if tid.is_null() {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    pth_attr_destroy(attr);
    fprintf(stderr, b"Joining thread\n\0" as *const u8 as *const i8);
    rc_0 = pth_join(tid, &mut val);
    if rc_0 == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    if val != 1123 as i32 as *mut libc::c_void {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    fprintf(
        stderr,
        b"\n=== TESTING NESTED THREAD OPERATION ===\n\n\0" as *const u8 as *const i8,
    );
    let mut tid_0: pth_t = 0 as *mut pth_st;
    let mut val_0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc_1: i32 = 0;
    fprintf(stderr, b"Spawning thread 1\n\0" as *const u8 as *const i8);
    tid_0 = pth_spawn(
        0 as pth_attr_t,
        Some(t2_func as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        1 as i32 as *mut libc::c_void,
    );
    if tid_0.is_null() {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    rc_1 = pth_join(tid_0, &mut val_0);
    fprintf(stderr, b"Joined thread 1\n\0" as *const u8 as *const i8);
    if rc_1 == 0 as i32 {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    if val_0
        != (1 as i32 * 2 as i32 * 3 as i32 * 4 as i32 * 5 as i32 * 6 as i32 * 7 as i32
            * 8 as i32 * 9 as i32) as *mut libc::c_void
    {
        fprintf(
            stderr,
            b"*** ERROR, TEST FAILED:\n*** errno=%d\n\n\0" as *const u8 as *const i8,
            *__errno_location(),
        );
        exit(1 as i32);
    }
    pth_kill();
    fprintf(
        stderr,
        b"\nOK - ALL TESTS SUCCESSFULLY PASSED.\n\n\0" as *const u8 as *const i8,
    );
    exit(0 as i32);
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