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
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn pth_init() -> i32;
    fn pth_kill() -> i32;
    fn pth_ctrl(_: u64, _: ...) -> i64;
    fn pth_attr_of(_: pth_t) -> pth_attr_t;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: i32, _: ...) -> i32;
    fn pth_attr_destroy(_: pth_attr_t) -> i32;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_mutex_acquire(_: *mut pth_mutex_t, _: i32, _: pth_event_t) -> i32;
    fn pth_mutex_release(_: *mut pth_mutex_t) -> i32;
    fn pth_usleep(_: u32) -> i32;
    fn pth_read(_: i32, _: *mut libc::c_void, _: size_t) -> ssize_t;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mutex_st {
    pub mx_node: pth_ringnode_t,
    pub mx_state: i32,
    pub mx_owner: pth_t,
    pub mx_count: u64,
}
pub type pth_mutex_t = pth_mutex_st;
#[no_mangle]
pub static mut mutex: pth_mutex_t = {
    let mut init = pth_mutex_st {
        mx_node: {
            let mut init = pth_ringnode_st {
                rn_next: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                rn_prev: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
            };
            init
        },
        mx_state: (1 as i32) << 0 as i32,
        mx_owner: 0 as *const pth_st as pth_t,
        mx_count: 0 as i32 as u64,
    };
    init
};
unsafe extern "C" fn my_reader(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut buf: [i8; 3] = [0; 3];
    let mut n: i32 = 0;
    loop {
        n = pth_read(0 as i32, buf.as_mut_ptr() as *mut libc::c_void, 1 as i32 as size_t)
            as i32;
        if n < 0 as i32 {
            fprintf(stderr, b"reader: error\n\0" as *const u8 as *const i8);
            break;
        } else if n == 0 as i32 {
            fprintf(stderr, b"reader: EOF\n\0" as *const u8 as *const i8);
            break;
        } else {
            if n == 1 as i32 && buf[0 as i32 as usize] as i32 == '\n' as i32 {
                buf[0 as i32 as usize] = '\\' as i32 as i8;
                buf[1 as i32 as usize] = 'n' as i32 as i8;
                n = 2 as i32;
            }
            buf[n as usize] = '\0' as i32 as i8;
            fprintf(
                stderr,
                b"reader: bytes=%d, char='%s'\n\0" as *const u8 as *const i8,
                n,
                buf.as_mut_ptr(),
            );
            if buf[0 as i32 as usize] as i32 == 'Q' as i32
                || buf[0 as i32 as usize] as i32 == 'q' as i32
            {
                break;
            }
            if buf[0 as i32 as usize] as i32 == 'L' as i32
                || buf[0 as i32 as usize] as i32 == 'l' as i32
            {
                fprintf(stderr, b"reader: ACQUIRE MUTEX\n\0" as *const u8 as *const i8);
                pth_mutex_acquire(&mut mutex, 0 as i32, 0 as pth_event_t);
            }
            if buf[0 as i32 as usize] as i32 == 'U' as i32
                || buf[0 as i32 as usize] as i32 == 'u' as i32
            {
                fprintf(stderr, b"reader: RELEASE MUTEX\n\0" as *const u8 as *const i8);
                pth_mutex_release(&mut mutex);
            }
            fflush(stderr);
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn my_child(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: i32 = 0;
    let mut name: *mut i8 = _arg as *mut i8;
    i = 0 as i32;
    while i < 10 as i32 {
        pth_mutex_acquire(&mut mutex, 0 as i32, 0 as pth_event_t);
        fprintf(stderr, b"%s: %d\n\0" as *const u8 as *const i8, name, i);
        pth_mutex_release(&mut mutex);
        pth_usleep(500000 as i32 as u32);
        i += 1;
        i;
    }
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut child: [pth_t; 10] = [0 as *mut pth_st; 10];
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut t_attr2: pth_attr_t = 0 as *mut pth_attr_st;
    let mut n: i64 = 0;
    pth_init();
    fprintf(
        stderr,
        b"This is TEST_MISC, a Pth test using various stuff.\n\0" as *const u8
            as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"A stdin reader child and various looping childs are\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"spawned. When you enter 'l' you can lock a mutex which\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"blocks the looping childs. 'u' unlocks this mutex.\n\0" as *const u8
            as *const i8,
    );
    fprintf(stderr, b"Enter 'q' to quit.\n\0" as *const u8 as *const i8);
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"Main Startup (%ld total threads running)\n\0" as *const u8 as *const i8,
        pth_ctrl(
            ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32 | (1 as i32) << 6 as i32
                | (1 as i32) << 7 as i32 | (1 as i32) << 8 as i32
                | (1 as i32) << 9 as i32) as u64,
        ),
    );
    t_attr = pth_attr_new();
    pth_attr_set(t_attr, C2RustUnnamed::PTH_ATTR_JOINABLE as i32, 0 as i32);
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"foo\0" as *const u8 as *const i8,
    );
    child[0 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"foo\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"bar\0" as *const u8 as *const i8,
    );
    child[1 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"bar\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"baz\0" as *const u8 as *const i8,
    );
    child[2 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"baz\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"quux\0" as *const u8 as *const i8,
    );
    child[3 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"quux\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"killer\0" as *const u8 as *const i8,
    );
    pth_attr_set(t_attr, C2RustUnnamed::PTH_ATTR_PRIO as i32, 4 as i32);
    child[4 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"killer\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"killer II\0" as *const u8 as *const i8,
    );
    pth_attr_set(t_attr, C2RustUnnamed::PTH_ATTR_PRIO as i32, 5 as i32);
    child[5 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"killer II\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"reader\0" as *const u8 as *const i8,
    );
    pth_attr_set(t_attr, C2RustUnnamed::PTH_ATTR_PRIO as i32, 0 as i32);
    child[6 as i32 as usize] = pth_spawn(
        t_attr,
        Some(my_reader as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"reader\0" as *const u8 as *const i8 as *mut libc::c_void,
    );
    pth_attr_destroy(t_attr);
    t_attr2 = pth_attr_of(child[0 as i32 as usize]);
    pth_attr_set(t_attr2, C2RustUnnamed::PTH_ATTR_PRIO as i32, -(1 as i32));
    pth_attr_destroy(t_attr2);
    t_attr2 = pth_attr_of(child[3 as i32 as usize]);
    pth_attr_set(t_attr2, C2RustUnnamed::PTH_ATTR_PRIO as i32, 1 as i32);
    pth_attr_destroy(t_attr2);
    fprintf(
        stderr,
        b"Main Loop (%ld total threads running)\n\0" as *const u8 as *const i8,
        pth_ctrl(
            ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32 | (1 as i32) << 6 as i32
                | (1 as i32) << 7 as i32 | (1 as i32) << 8 as i32
                | (1 as i32) << 9 as i32) as u64,
        ),
    );
    loop {
        n = pth_ctrl(
            ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32 | (1 as i32) << 6 as i32
                | (1 as i32) << 7 as i32 | (1 as i32) << 8 as i32
                | (1 as i32) << 9 as i32) as u64,
        );
        if !(n > 1 as i32 as i64) {
            break;
        }
        fprintf(
            stderr,
            b"Main Loop (%ld total threads still running)\n\0" as *const u8 as *const i8,
            n,
        );
        pth_usleep(500000 as i32 as u32);
    }
    fprintf(
        stderr,
        b"Main Exit (%ld total threads running)\n\0" as *const u8 as *const i8,
        pth_ctrl(
            ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32 | (1 as i32) << 6 as i32
                | (1 as i32) << 7 as i32 | (1 as i32) << 8 as i32
                | (1 as i32) << 9 as i32) as u64,
        ),
    );
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