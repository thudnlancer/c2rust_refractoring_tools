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
    pub type tm_zone;
    pub type wordsplit_node;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush_unlocked(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn sleep(__seconds: u32) -> u32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn sigwait(__set: *const sigset_t, __sig: *mut i32) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn getenv(__name: *const i8) -> *mut i8;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn fputs_unlocked(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    static mut exit_status: i32;
    fn fatal_exit() -> !;
    static mut checkpoint_option: u32;
    static mut archive_name_cursor: *mut *const i8;
    static mut stdlis: *mut FILE;
    fn compute_duration() -> libc::c_double;
    fn print_total_stats();
    fn format_total_stats(
        fp: *mut FILE,
        formats: *const *const i8,
        eor: i32,
        eol: i32,
    ) -> i32;
    fn unquote_string(str: *mut i8) -> i32;
    fn decode_signal(_: *const i8) -> i32;
    fn sys_exec_checkpoint_script(
        script_name: *const i8,
        archive_name: *const i8,
        checkpoint_number: i32,
    );
    static mut program_name: *const i8;
    fn wordsplit(s: *const i8, ws: *mut wordsplit_t, flags: u32) -> i32;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const i8;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn fprintftime(
        fp: *mut FILE,
        fmt: *const i8,
        tm: *const tm,
        zone: timezone_t,
        nanoseconds: i32,
    ) -> size_t;
}
pub type __uintmax_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
pub type timezone_t = *mut tm_zone;
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub time: time_t,
    pub command: *mut i8,
    pub signal: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct checkpoint_action {
    pub next: *mut checkpoint_action,
    pub opcode: checkpoint_opcode,
    pub v: C2RustUnnamed,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum checkpoint_opcode {
    cop_wait = 7,
    cop_totals = 6,
    cop_exec = 5,
    cop_sleep = 4,
    cop_ttyout = 3,
    cop_echo = 2,
    cop_bell = 1,
    cop_dot = 0,
}
impl checkpoint_opcode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            checkpoint_opcode::cop_wait => 7,
            checkpoint_opcode::cop_totals => 6,
            checkpoint_opcode::cop_exec => 5,
            checkpoint_opcode::cop_sleep => 4,
            checkpoint_opcode::cop_ttyout => 3,
            checkpoint_opcode::cop_echo => 2,
            checkpoint_opcode::cop_bell => 1,
            checkpoint_opcode::cop_dot => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> checkpoint_opcode {
        match value {
            7 => checkpoint_opcode::cop_wait,
            6 => checkpoint_opcode::cop_totals,
            5 => checkpoint_opcode::cop_exec,
            4 => checkpoint_opcode::cop_sleep,
            3 => checkpoint_opcode::cop_ttyout,
            2 => checkpoint_opcode::cop_echo,
            1 => checkpoint_opcode::cop_bell,
            0 => checkpoint_opcode::cop_dot,
            _ => panic!("Invalid value for checkpoint_opcode: {}", value),
        }
    }
}
impl AddAssign<u32> for checkpoint_opcode {
    fn add_assign(&mut self, rhs: u32) {
        *self = checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for checkpoint_opcode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for checkpoint_opcode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for checkpoint_opcode {
    fn div_assign(&mut self, rhs: u32) {
        *self = checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for checkpoint_opcode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for checkpoint_opcode {
    type Output = checkpoint_opcode;
    fn add(self, rhs: u32) -> checkpoint_opcode {
        checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for checkpoint_opcode {
    type Output = checkpoint_opcode;
    fn sub(self, rhs: u32) -> checkpoint_opcode {
        checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for checkpoint_opcode {
    type Output = checkpoint_opcode;
    fn mul(self, rhs: u32) -> checkpoint_opcode {
        checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for checkpoint_opcode {
    type Output = checkpoint_opcode;
    fn div(self, rhs: u32) -> checkpoint_opcode {
        checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for checkpoint_opcode {
    type Output = checkpoint_opcode;
    fn rem(self, rhs: u32) -> checkpoint_opcode {
        checkpoint_opcode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    CHKP_INIT,
    CHKP_COMPILE,
    CHKP_RUN,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::CHKP_INIT => 0,
            C2RustUnnamed_0::CHKP_COMPILE => 1,
            C2RustUnnamed_0::CHKP_RUN => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            0 => C2RustUnnamed_0::CHKP_INIT,
            1 => C2RustUnnamed_0::CHKP_COMPILE,
            2 => C2RustUnnamed_0::CHKP_RUN,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut i8,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: u32,
    pub ws_options: u32,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const i8,
    pub ws_comment: *const i8,
    pub ws_escape: [*const i8; 2],
    pub ws_alloc_die: Option<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option<unsafe extern "C" fn(*const i8, ...) -> ()>,
    pub ws_debug: Option<unsafe extern "C" fn(*const i8, ...) -> ()>,
    pub ws_env: *mut *const i8,
    pub ws_envbuf: *mut *mut i8,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option<
        unsafe extern "C" fn(*mut *mut i8, *const i8, size_t, *mut libc::c_void) -> i32,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option<
        unsafe extern "C" fn(
            *mut *mut i8,
            *const i8,
            size_t,
            *mut *mut i8,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub ws_input: *const i8,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: i32,
    pub ws_usererr: *mut i8,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: i32,
}
pub type wordsplit_t = wordsplit;
#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32 as i64 != 0
    {
        __overflow(__stream, __c as u8 as i32)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
static mut checkpoint: u32 = 0;
static mut checkpoint_action: *mut checkpoint_action = 0 as *const checkpoint_action
    as *mut checkpoint_action;
static mut checkpoint_action_tail: *mut checkpoint_action = 0 as *const checkpoint_action
    as *mut checkpoint_action;
static mut checkpoint_state: i32 = 0;
static mut sigs: sigset_t = sigset_t { __val: [0; 16] };
unsafe extern "C" fn alloc_action(
    mut opcode: checkpoint_opcode,
) -> *mut checkpoint_action {
    let mut p: *mut checkpoint_action = xzalloc(
        ::core::mem::size_of::<checkpoint_action>() as u64,
    ) as *mut checkpoint_action;
    if !checkpoint_action_tail.is_null() {
        (*checkpoint_action_tail).next = p;
    } else {
        checkpoint_action = p;
    }
    checkpoint_action_tail = p;
    (*p).opcode = opcode;
    return p;
}
unsafe extern "C" fn copy_string_unquote(mut str: *const i8) -> *mut i8 {
    let mut output: *mut i8 = xstrdup(str);
    let mut len: size_t = strlen(output);
    if (*output as i32 == '"' as i32 || *output as i32 == '\'' as i32)
        && *output.offset(len.wrapping_sub(1 as i32 as u64) as isize) as i32
            == *output as i32
    {
        memmove(
            output as *mut libc::c_void,
            output.offset(1 as i32 as isize) as *const libc::c_void,
            len.wrapping_sub(2 as i32 as u64),
        );
        *output.offset(len.wrapping_sub(2 as i32 as u64) as isize) = 0 as i32 as i8;
    }
    unquote_string(output);
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn checkpoint_compile_action(mut str: *const i8) {
    let mut act: *mut checkpoint_action = 0 as *mut checkpoint_action;
    if checkpoint_state == C2RustUnnamed_0::CHKP_INIT as i32 {
        sigemptyset(&mut sigs);
        checkpoint_state = C2RustUnnamed_0::CHKP_COMPILE as i32;
    }
    if strcmp(str, b".\0" as *const u8 as *const i8) == 0 as i32
        || strcmp(str, b"dot\0" as *const u8 as *const i8) == 0 as i32
    {
        alloc_action(checkpoint_opcode::cop_dot);
    } else if strcmp(str, b"bell\0" as *const u8 as *const i8) == 0 as i32 {
        alloc_action(checkpoint_opcode::cop_bell);
    } else if strcmp(str, b"echo\0" as *const u8 as *const i8) == 0 as i32 {
        alloc_action(checkpoint_opcode::cop_echo);
    } else if strncmp(str, b"echo=\0" as *const u8 as *const i8, 5 as i32 as u64)
        == 0 as i32
    {
        act = alloc_action(checkpoint_opcode::cop_echo);
        (*act).v.command = copy_string_unquote(str.offset(5 as i32 as isize));
    } else if strncmp(str, b"exec=\0" as *const u8 as *const i8, 5 as i32 as u64)
        == 0 as i32
    {
        act = alloc_action(checkpoint_opcode::cop_exec);
        (*act).v.command = copy_string_unquote(str.offset(5 as i32 as isize));
    } else if strncmp(str, b"ttyout=\0" as *const u8 as *const i8, 7 as i32 as u64)
        == 0 as i32
    {
        act = alloc_action(checkpoint_opcode::cop_ttyout);
        (*act).v.command = copy_string_unquote(str.offset(7 as i32 as isize));
    } else if strncmp(str, b"sleep=\0" as *const u8 as *const i8, 6 as i32 as u64)
        == 0 as i32
    {
        let mut p: *mut i8 = 0 as *mut i8;
        let mut n: time_t = strtoul(str.offset(6 as i32 as isize), &mut p, 10 as i32)
            as time_t;
        if *p != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s: not a valid timeout\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                str,
            );
            fatal_exit();
        }
        act = alloc_action(checkpoint_opcode::cop_sleep);
        (*act).v.time = n;
    } else if strcmp(str, b"totals\0" as *const u8 as *const i8) == 0 as i32 {
        alloc_action(checkpoint_opcode::cop_totals);
    } else if strncmp(str, b"wait=\0" as *const u8 as *const i8, 5 as i32 as u64)
        == 0 as i32
    {
        act = alloc_action(checkpoint_opcode::cop_wait);
        (*act).v.signal = decode_signal(str.offset(5 as i32 as isize));
        sigaddset(&mut sigs, (*act).v.signal);
    } else {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: unknown checkpoint action\0" as *const u8 as *const i8,
                5 as i32,
            ),
            str,
        );
        fatal_exit();
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkpoint_finish_compile() {
    if checkpoint_state == C2RustUnnamed_0::CHKP_INIT as i32 && checkpoint_option != 0
        && checkpoint_action.is_null()
    {
        checkpoint_compile_action(b"echo\0" as *const u8 as *const i8);
    }
    if checkpoint_state == C2RustUnnamed_0::CHKP_COMPILE as i32 {
        sigprocmask(0 as i32, &mut sigs, 0 as *mut sigset_t);
        if checkpoint_option == 0 {
            checkpoint_option = 10 as i32 as u32;
        }
        checkpoint_state = C2RustUnnamed_0::CHKP_RUN as i32;
    }
}
static mut checkpoint_total_format: [*const i8; 3] = [
    b"R\0" as *const u8 as *const i8,
    b"W\0" as *const u8 as *const i8,
    b"D\0" as *const u8 as *const i8,
];
unsafe extern "C" fn getwidth(mut fp: *mut FILE) -> i64 {
    let mut columns: *const i8 = 0 as *const i8;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(fileno(fp), 0x5413 as i32 as u64, &mut ws as *mut winsize) == 0 as i32
        && (0 as i32) < ws.ws_col as i32
    {
        return ws.ws_col as i64;
    }
    columns = getenv(b"COLUMNS\0" as *const u8 as *const i8);
    if !columns.is_null() {
        let mut col: i64 = strtol(columns, 0 as *mut *mut i8, 10 as i32);
        if (0 as i32 as i64) < col {
            return col;
        }
    }
    return 80 as i32 as i64;
}
unsafe extern "C" fn getarg(
    mut input: *const i8,
    mut endp: *mut *const i8,
    mut argbuf: *mut *mut i8,
    mut arglen: *mut size_t,
) -> *mut i8 {
    if *input.offset(0 as i32 as isize) as i32 == '{' as i32 {
        let mut p: *mut i8 = strchr(input.offset(1 as i32 as isize), '}' as i32);
        if !p.is_null() {
            let mut n: size_t = p.offset_from(input) as i64 as size_t;
            if n > *arglen {
                *arglen = n;
                *argbuf = xrealloc(*argbuf as *mut libc::c_void, *arglen) as *mut i8;
            }
            n = n.wrapping_sub(1);
            n;
            memcpy(
                *argbuf as *mut libc::c_void,
                input.offset(1 as i32 as isize) as *const libc::c_void,
                n,
            );
            *(*argbuf).offset(n as isize) = 0 as i32 as i8;
            *endp = p.offset(1 as i32 as isize);
            return *argbuf;
        }
    }
    *endp = input;
    return 0 as *mut i8;
}
static mut tty_cleanup: i32 = 0;
static mut def_format: *const i8 = b"%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r\0"
    as *const u8 as *const i8;
unsafe extern "C" fn format_checkpoint_string(
    mut fp: *mut FILE,
    mut len: size_t,
    mut input: *const i8,
    mut do_write: bool,
    mut cpn: u32,
) -> i32 {
    let mut opstr: *const i8 = if do_write as i32 != 0 {
        dcgettext(0 as *const i8, b"write\0" as *const u8 as *const i8, 5 as i32)
    } else {
        dcgettext(0 as *const i8, b"read\0" as *const u8 as *const i8, 5 as i32)
    };
    let mut uintbuf: [i8; 21] = [0; 21];
    let mut cps: *mut i8 = umaxtostr(cpn as uintmax_t, uintbuf.as_mut_ptr());
    let mut ip: *const i8 = 0 as *const i8;
    static mut argbuf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut arglen: size_t = 0 as i32 as size_t;
    let mut arg: *mut i8 = 0 as *mut i8;
    if input.is_null() {
        if do_write {
            input = dcgettext(
                0 as *const i8,
                b"Write checkpoint %u\0" as *const u8 as *const i8,
                5 as i32,
            );
        } else {
            input = dcgettext(
                0 as *const i8,
                b"Read checkpoint %u\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
    }
    let mut current_block_56: u64;
    ip = input;
    while *ip != 0 {
        if *ip as i32 == '%' as i32 {
            ip = ip.offset(1);
            if *ip as i32 == '{' as i32 {
                arg = getarg(ip, &mut ip, &mut argbuf, &mut arglen);
                if arg.is_null() {
                    fputc_unlocked('%' as i32, fp);
                    fputc_unlocked(*ip as i32, fp);
                    len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
                    current_block_56 = 7815301370352969686;
                } else {
                    current_block_56 = 10048703153582371463;
                }
            } else {
                current_block_56 = 10048703153582371463;
            }
            match current_block_56 {
                7815301370352969686 => {}
                _ => {
                    match *ip as i32 {
                        99 => {
                            len = (len as u64)
                                .wrapping_add(
                                    format_checkpoint_string(fp, len, def_format, do_write, cpn)
                                        as u64,
                                ) as size_t as size_t;
                        }
                        117 => {
                            fputs_unlocked(cps, fp);
                            len = (len as u64).wrapping_add(strlen(cps)) as size_t
                                as size_t;
                        }
                        115 => {
                            fputs_unlocked(opstr, fp);
                            len = (len as u64).wrapping_add(strlen(opstr)) as size_t
                                as size_t;
                        }
                        100 => {
                            len = (len as u64)
                                .wrapping_add(
                                    fprintf(
                                        fp,
                                        b"%.0f\0" as *const u8 as *const i8,
                                        compute_duration(),
                                    ) as u64,
                                ) as size_t as size_t;
                        }
                        84 => {
                            let mut fmt: *mut *const i8 = checkpoint_total_format
                                .as_mut_ptr();
                            let mut fmtbuf: [*const i8; 3] = [0 as *const i8; 3];
                            let mut ws: wordsplit = wordsplit {
                                ws_wordc: 0,
                                ws_wordv: 0 as *mut *mut i8,
                                ws_offs: 0,
                                ws_wordn: 0,
                                ws_flags: 0,
                                ws_options: 0,
                                ws_maxwords: 0,
                                ws_wordi: 0,
                                ws_delim: 0 as *const i8,
                                ws_comment: 0 as *const i8,
                                ws_escape: [0 as *const i8; 2],
                                ws_alloc_die: None,
                                ws_error: None,
                                ws_debug: None,
                                ws_env: 0 as *mut *const i8,
                                ws_envbuf: 0 as *mut *mut i8,
                                ws_envidx: 0,
                                ws_envsiz: 0,
                                ws_getvar: None,
                                ws_closure: 0 as *mut libc::c_void,
                                ws_command: None,
                                ws_input: 0 as *const i8,
                                ws_len: 0,
                                ws_endp: 0,
                                ws_errno: 0,
                                ws_usererr: 0 as *mut i8,
                                ws_head: 0 as *mut wordsplit_node,
                                ws_tail: 0 as *mut wordsplit_node,
                                ws_lvl: 0,
                            };
                            compute_duration();
                            if !arg.is_null() {
                                ws.ws_delim = b",\0" as *const u8 as *const i8;
                                if wordsplit(
                                    arg,
                                    &mut ws,
                                    (0x40 as i32 | 0x4 as i32 | (0x200 as i32 | 0x400 as i32)
                                        | 0x4000 as i32) as u32,
                                ) != 0
                                {
                                    if error_hook.is_some() {
                                        error_hook.expect("non-null function pointer")();
                                    }
                                    error(
                                        0 as i32,
                                        0 as i32,
                                        dcgettext(
                                            0 as *const i8,
                                            b"cannot split string '%s': %s\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                        arg,
                                        wordsplit_strerror(&mut ws),
                                    );
                                    exit_status = 2 as i32;
                                } else {
                                    let mut i: i32 = 0;
                                    i = 0 as i32;
                                    while (i as u64) < ws.ws_wordc {
                                        fmtbuf[i as usize] = *(ws.ws_wordv).offset(i as isize);
                                        i += 1;
                                        i;
                                    }
                                    while i < 3 as i32 {
                                        fmtbuf[i as usize] = 0 as *const i8;
                                        i += 1;
                                        i;
                                    }
                                    fmt = fmtbuf.as_mut_ptr();
                                }
                            }
                            len = (len as u64)
                                .wrapping_add(
                                    format_total_stats(fp, fmt, ',' as i32, 0 as i32) as u64,
                                ) as size_t as size_t;
                            if !arg.is_null() {
                                wordsplit_free(&mut ws);
                            }
                        }
                        116 => {
                            let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
                            let mut tm: *mut tm = 0 as *mut tm;
                            let mut fmt_0: *const i8 = if !arg.is_null() {
                                arg
                            } else {
                                b"%c\0" as *const u8 as *const i8
                            };
                            gettimeofday(&mut tv, 0 as *mut timezone);
                            tm = localtime(&mut tv.tv_sec);
                            len = (len as u64)
                                .wrapping_add(
                                    fprintftime(
                                        fp,
                                        fmt_0,
                                        tm,
                                        0 as timezone_t,
                                        (tv.tv_usec * 1000 as i32 as i64) as i32,
                                    ),
                                ) as size_t as size_t;
                        }
                        42 => {
                            let mut w: i64 = if !arg.is_null() {
                                strtol(arg, 0 as *mut *mut i8, 10 as i32)
                            } else {
                                getwidth(fp)
                            };
                            while w as u64 > len {
                                fputc_unlocked(' ' as i32, fp);
                                len = len.wrapping_add(1);
                                len;
                            }
                        }
                        _ => {
                            fputc_unlocked('%' as i32, fp);
                            fputc_unlocked(*ip as i32, fp);
                            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t
                                as size_t;
                        }
                    }
                    arg = 0 as *mut i8;
                }
            }
        } else {
            fputc_unlocked(*ip as i32, fp);
            if *ip as i32 == '\r' as i32 {
                len = 0 as i32 as size_t;
                tty_cleanup = 1 as i32;
            } else {
                len = len.wrapping_add(1);
                len;
            }
        }
        ip = ip.offset(1);
        ip;
    }
    fflush_unlocked(fp);
    return len as i32;
}
static mut tty: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn run_checkpoint_actions(mut do_write: bool) {
    let mut p: *mut checkpoint_action = 0 as *mut checkpoint_action;
    p = checkpoint_action;
    while !p.is_null() {
        match (*p).opcode as u32 {
            0 => {
                fputc_unlocked('.' as i32, stdlis);
                fflush_unlocked(stdlis);
            }
            1 => {
                if tty.is_null() {
                    tty = fopen(
                        b"/dev/tty\0" as *const u8 as *const i8,
                        b"w\0" as *const u8 as *const i8,
                    );
                }
                if !tty.is_null() {
                    fputc_unlocked('\u{7}' as i32, tty);
                    fflush_unlocked(tty);
                }
            }
            2 => {
                let mut n: i32 = fprintf(
                    stderr,
                    b"%s: \0" as *const u8 as *const i8,
                    program_name,
                );
                format_checkpoint_string(
                    stderr,
                    n as size_t,
                    (*p).v.command,
                    do_write,
                    checkpoint,
                );
                fputc_unlocked('\n' as i32, stderr);
            }
            3 => {
                if tty.is_null() {
                    tty = fopen(
                        b"/dev/tty\0" as *const u8 as *const i8,
                        b"w\0" as *const u8 as *const i8,
                    );
                }
                if !tty.is_null() {
                    format_checkpoint_string(
                        tty,
                        0 as i32 as size_t,
                        (*p).v.command,
                        do_write,
                        checkpoint,
                    );
                }
            }
            4 => {
                sleep((*p).v.time as u32);
            }
            5 => {
                sys_exec_checkpoint_script(
                    (*p).v.command,
                    *archive_name_cursor.offset(0 as i32 as isize),
                    checkpoint as i32,
                );
            }
            6 => {
                compute_duration();
                print_total_stats();
            }
            7 => {
                let mut n_0: i32 = 0;
                sigwait(&mut sigs, &mut n_0);
            }
            _ => {}
        }
        p = (*p).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn checkpoint_flush_actions() {
    let mut p: *mut checkpoint_action = 0 as *mut checkpoint_action;
    p = checkpoint_action;
    while !p.is_null() {
        match (*p).opcode as u32 {
            3 => {
                if !tty.is_null() && tty_cleanup != 0 {
                    let mut w: i64 = getwidth(tty);
                    loop {
                        let fresh1 = w;
                        w = w - 1;
                        if !(fresh1 != 0) {
                            break;
                        }
                        fputc_unlocked(' ' as i32, tty);
                    }
                    fputc_unlocked('\r' as i32, tty);
                    fflush_unlocked(tty);
                }
            }
            _ => {}
        }
        p = (*p).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn checkpoint_run(mut do_write: bool) {
    if checkpoint_option != 0
        && {
            checkpoint = checkpoint.wrapping_add(1);
            checkpoint.wrapping_rem(checkpoint_option) == 0
        }
    {
        run_checkpoint_actions(do_write);
    }
}
#[no_mangle]
pub unsafe extern "C" fn checkpoint_finish() {
    if checkpoint_option != 0 {
        checkpoint_flush_actions();
        if !tty.is_null() {
            fclose(tty);
        }
    }
}