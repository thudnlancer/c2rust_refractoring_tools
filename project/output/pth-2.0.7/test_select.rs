use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn exit(_: i32) -> !;
    fn __errno_location() -> *mut i32;
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
    fn pth_cancel(_: pth_t) -> i32;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> i32;
    fn pth_timeout(_: i64, _: i64) -> pth_time_t;
    fn pth_event(_: u64, _: ...) -> pth_event_t;
    fn pth_event_free(_: pth_event_t, _: i32) -> i32;
    fn pth_select_ev(
        _: i32,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut timeval,
        _: pth_event_t,
    ) -> i32;
    fn pth_sleep(_: u32) -> u32;
    fn pth_read(_: i32, _: *mut libc::c_void, _: size_t) -> ssize_t;
}
pub type size_t = u64;
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
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
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
    let mut evt: pth_event_t = 0 as *mut pth_event_st;
    let mut t_ticker: pth_t = 0 as *mut pth_st;
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut c: i8 = 0;
    let mut n: i32 = 0;
    pth_init();
    fprintf(
        stderr,
        b"This is TEST_SELECT, a Pth test using select.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"Enter data. Hit CTRL-C to stop this test.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    t_attr = pth_attr_new();
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
    evt = 0 as pth_event_t;
    loop {
        if evt.is_null() {
            evt = pth_event(
                ((1 as i32) << 4 as i32) as u64,
                pth_timeout(10 as i32 as i64, 0 as i32 as i64),
            );
        } else {
            evt = pth_event(
                ((1 as i32) << 4 as i32 | (1 as i32) << 20 as i32) as u64,
                evt,
                pth_timeout(10 as i32 as i64, 0 as i32 as i64),
            );
        }
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(rfds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
            fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
            fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        rfds
            .__fds_bits[(0 as i32
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << 0 as i32
                    % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        n = pth_select_ev(
            0 as i32 + 1 as i32,
            &mut rfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
            evt,
        );
        if n == -(1 as i32) && *__errno_location() == 4 as i32 {
            fprintf(stderr, b"main: timeout - repeating\n\0" as *const u8 as *const i8);
        } else {
            if !(rfds
                .__fds_bits[(0 as i32
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << 0 as i32
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64)
            {
                fprintf(
                    stderr,
                    b"main: Hmmmm... strange situation: bit not set\n\0" as *const u8
                        as *const i8,
                );
                exit(1 as i32);
            }
            fprintf(
                stderr,
                b"main: select returned %d\n\0" as *const u8 as *const i8,
                n,
            );
            while pth_read(
                0 as i32,
                &mut c as *mut i8 as *mut libc::c_void,
                1 as i32 as size_t,
            ) > 0 as i32 as i64
            {
                fprintf(
                    stderr,
                    b"main: read stdin '%c'\n\0" as *const u8 as *const i8,
                    c as i32,
                );
            }
        }
    };
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