#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn isatty(__fd: i32) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn tcgetattr(__fd: i32, __termios_p: *mut termios) -> i32;
    fn tcsetattr(__fd: i32, __optional_actions: i32, __termios_p: *const termios) -> i32;
    fn tcflush(__fd: i32, __queue_selector: i32) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn setup_signal();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
pub type va_list = __builtin_va_list;
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
pub type cc_t = u8;
pub type speed_t = u32;
pub type tcflag_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
pub type Terminal = termios;
static mut tty: *mut FILE = 0 as *const FILE as *mut FILE;
static mut notty: i32 = 0 as i32;
static mut ttyfd: i32 = -(1 as i32);
#[no_mangle]
pub static mut mtools_raw_tty: i32 = 1 as i32;
static mut tty_mode: i32 = -(1 as i32);
static mut need_tty_reset: i32 = 0 as i32;
static mut handlerIsSet: i32 = 0 as i32;
static mut in_orig: Terminal = Terminal {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
unsafe extern "C" fn tty_time_out(mut dummy: i32) -> ! {
    let mut exit_code: i32 = 0;
    signal(
        14 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    if !tty.is_null() && need_tty_reset != 0 {
        tcsetattr(ttyfd, 0 as i32, &mut in_orig);
    }
    exit_code = 0 as i32;
    exit(exit_code);
}
unsafe extern "C" fn cleanup_tty() {
    if !tty.is_null() && need_tty_reset != 0 {
        tcsetattr(ttyfd, 0 as i32, &mut in_orig);
        setup_signal();
    }
}
unsafe extern "C" fn set_raw_tty(mut mode: i32) {
    let mut in_raw: Terminal = Terminal {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if mode != tty_mode && mode != -(1 as i32) {
        if handlerIsSet == 0 {
            tcgetattr(ttyfd, &mut in_orig);
            need_tty_reset = 1 as i32;
            atexit(Some(cleanup_tty as unsafe extern "C" fn() -> ()));
            handlerIsSet = 1 as i32;
        }
        setup_signal();
        signal(
            14 as i32,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(i32) -> !>,
                __sighandler_t,
            >(Some(tty_time_out as unsafe extern "C" fn(i32) -> !)),
        );
        tcgetattr(ttyfd, &mut in_raw);
        if mode != 0 {
            in_raw.c_lflag &= !(0 as u32) ^ 0o2 as i32 as u32;
            in_raw.c_cc[6 as i32 as usize] = 1 as i32 as cc_t;
            in_raw.c_cc[5 as i32 as usize] = 0 as i32 as cc_t;
            tcsetattr(ttyfd, 0 as i32, &mut in_raw);
        } else {
            in_raw.c_lflag |= 0o2 as i32 as u32;
            tcsetattr(ttyfd, 0 as i32, &mut in_raw);
        }
        tty_mode = mode;
        tcflush(ttyfd, 0 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn opentty(mut mode: i32) -> *mut FILE {
    if notty != 0 {
        return 0 as *mut FILE;
    }
    if tty.is_null() {
        ttyfd = open(b"/dev/tty\0" as *const u8 as *const i8, 0 as i32);
        if ttyfd >= 0 as i32 {
            tty = fdopen(ttyfd, b"r\0" as *const u8 as *const i8);
        }
    }
    if tty.is_null() {
        if isatty(0 as i32) == 0 {
            notty = 1 as i32;
            return 0 as *mut FILE;
        }
        ttyfd = 0 as i32;
        tty = stdin;
    }
    if mtools_raw_tty != 0 {
        set_raw_tty(mode);
    }
    return tty;
}
#[no_mangle]
pub unsafe extern "C" fn ask_confirmation(mut format: *const i8, mut args: ...) -> i32 {
    let mut ans: [i8; 10] = [0; 10];
    let mut ap: ::core::ffi::VaListImpl;
    if (opentty(-(1 as i32))).is_null() {
        return 0 as i32;
    }
    loop {
        ap = args.clone();
        vfprintf(stderr, format, ap.as_va_list());
        fflush(stderr);
        fflush(opentty(-(1 as i32)));
        if mtools_raw_tty != 0 {
            let mut c: i32 = fgetc(opentty(1 as i32));
            if c < 0 as i32 {
                ans[0 as i32 as usize] = 'n' as i32 as i8;
            } else {
                ans[0 as i32 as usize] = c as i8;
            }
            fputs(b"\n\0" as *const u8 as *const i8, stderr);
        } else if (fgets(ans.as_mut_ptr(), 9 as i32, opentty(0 as i32))).is_null() {
            ans[0 as i32 as usize] = 'n' as i32 as i8;
        }
        if ans[0 as i32 as usize] as i32 == 'y' as i32
            || ans[0 as i32 as usize] as i32 == 'Y' as i32
        {
            return 0 as i32;
        }
        if ans[0 as i32 as usize] as i32 == 'n' as i32
            || ans[0 as i32 as usize] as i32 == 'N' as i32
        {
            return -(1 as i32);
        }
    };
}