#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    fn setenv(__name: *const i8, __value: *const i8, __replace: i32) -> i32;
    fn unsetenv(__name: *const i8) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn close(__fd: i32) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    fn execvp(__file: *const i8, __argv: *const *mut i8) -> i32;
    fn _exit(_: i32) -> !;
    fn sysconf(__name: i32) -> i64;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: i32) -> i32;
    fn close_stdin();
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn open_safer(_: *const i8, _: i32, _: ...) -> i32;
    static mut program_name: *const i8;
    fn set_program_name(argv0: *const i8);
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn pipe_safer(_: *mut i32) -> i32;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn bc_size_of_environment() -> size_t;
    fn bc_do_insert(
        ctl: *mut buildcmd_control,
        state: *mut buildcmd_state,
        arg: *mut i8,
        arglen: size_t,
        prefix: *const i8,
        pfxlen: size_t,
        linebuf_0: *const i8,
        lblen: size_t,
        initial_args_0: i32,
    );
    fn bc_do_exec(ctl: *mut buildcmd_control, state: *mut buildcmd_state);
    fn bc_push_arg(
        ctl: *mut buildcmd_control,
        state: *mut buildcmd_state,
        arg: *const i8,
        len: size_t,
        prefix: *const i8,
        pfxlen: size_t,
        initial_args_0: i32,
    );
    fn bc_init_controlinfo(
        ctl: *mut buildcmd_control,
        arglen_headroom: size_t,
    ) -> BC_INIT_STATUS;
    fn bc_use_sensible_arg_max(ctl: *mut buildcmd_control);
    fn bc_clear_args(ctl: *const buildcmd_control, state: *mut buildcmd_state);
    fn bc_args_exceed_testing_limit(argv: *mut *mut i8) -> bool;
    fn explain_how_to_report_bugs(f: *mut FILE, program_name_0: *const i8) -> i32;
    fn remember_non_cloexec_fds();
    fn complain_about_leaky_fds();
    fn fd_leak_check_is_enabled() -> bool;
    fn open_cloexec(path: *const i8, flags: i32, _: ...) -> i32;
    fn display_findutils_version(official_name: *const i8);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
}
pub type __uint32_t = u32;
pub type __uintmax_t = u64;
pub type __uid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __ssize_t = i64;
pub type __sig_atomic_t = i32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
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
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
pub type uintmax_t = __uintmax_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
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
pub type C2RustUnnamed_11 = u32;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_11 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_11 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_11 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_11 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_11 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_11 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_11 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_11 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_11 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_11 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_11 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_11 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_11 = 236;
pub const _SC_IPV6: C2RustUnnamed_11 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_11 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_11 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_11 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_11 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_11 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_11 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_11 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_11 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_11 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_11 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_11 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_11 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_11 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_11 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_11 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_11 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_11 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_11 = 182;
pub const _SC_TRACE: C2RustUnnamed_11 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_11 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_11 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_11 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_11 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_11 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_11 = 175;
pub const _SC_STREAMS: C2RustUnnamed_11 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_11 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_11 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_11 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_11 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_11 = 169;
pub const _SC_2_PBS: C2RustUnnamed_11 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_11 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_11 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_11 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_11 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_11 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_11 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_11 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_11 = 160;
pub const _SC_SPAWN: C2RustUnnamed_11 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_11 = 158;
pub const _SC_SHELL: C2RustUnnamed_11 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_11 = 156;
pub const _SC_REGEXP: C2RustUnnamed_11 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_11 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_11 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_11 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_11 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_11 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_11 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_11 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_11 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_11 = 146;
pub const _SC_PIPE: C2RustUnnamed_11 = 145;
pub const _SC_FIFO: C2RustUnnamed_11 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_11 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_11 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_11 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_11 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_11 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_11 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_11 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_11 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_11 = 135;
pub const _SC_BASE: C2RustUnnamed_11 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_11 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_11 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_11 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_11 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_11 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_11 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_11 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_11 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_11 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_11 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_11 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_11 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_11 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_11 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_11 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_11 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_11 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_11 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_11 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_11 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_11 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_11 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_11 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_11 = 110;
pub const _SC_NZERO: C2RustUnnamed_11 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_11 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_11 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_11 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_11 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_11 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_11 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_11 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_11 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_11 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_11 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_11 = 98;
pub const _SC_2_UPE: C2RustUnnamed_11 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_11 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_11 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_11 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_11 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_11 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_11 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_11 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_11 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_11 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_11 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_11 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_11 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_11 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_11 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_11 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_11 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_11 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_11 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_11 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_11 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_11 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_11 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_11 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_11 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_11 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_11 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_11 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_11 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_11 = 68;
pub const _SC_THREADS: C2RustUnnamed_11 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_11 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_11 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_11 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_11 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_11 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_11 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_11 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_11 = 60;
pub const _SC_SELECT: C2RustUnnamed_11 = 59;
pub const _SC_POLL: C2RustUnnamed_11 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_11 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_11 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_11 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_11 = 54;
pub const _SC_PII: C2RustUnnamed_11 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_11 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_11 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_11 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_11 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_11 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_11 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_11 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_11 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_11 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_11 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_11 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_11 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_11 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_11 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_11 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_11 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_11 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_11 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_11 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_11 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_11 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_11 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_11 = 30;
pub const _SC_VERSION: C2RustUnnamed_11 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_11 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_11 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_11 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_11 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_11 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_11 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_11 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_11 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_11 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_11 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_11 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_11 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_11 = 16;
pub const _SC_FSYNC: C2RustUnnamed_11 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_11 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_11 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_11 = 12;
pub const _SC_TIMERS: C2RustUnnamed_11 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_11 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_11 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_11 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_11 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_11 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_11 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_11 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_11 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_11 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_11 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_11 = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: size_t,
    pub cmd_argv: *mut *mut i8,
    pub cmd_argv_alloc: size_t,
    pub argbuf: *mut i8,
    pub cmd_argv_chars: size_t,
    pub cmd_initial_argv_chars: size_t,
    pub usercontext: *mut libc::c_void,
    pub todo: i32,
    pub dir_fd: i32,
    pub largest_successful_arg_count: size_t,
    pub smallest_failed_arg_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: i32,
    pub posix_arg_size_max: size_t,
    pub posix_arg_size_min: size_t,
    pub arg_max: size_t,
    pub max_arg_count: size_t,
    pub rplen: size_t,
    pub replace_pat: *const i8,
    pub initial_argc: size_t,
    pub exec_callback: Option<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut libc::c_void,
            i32,
            *mut *mut i8,
        ) -> i32,
    >,
    pub lines_per_exec: u64,
    pub args_per_exec: size_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum BC_INIT_STATUS {
    BC_INIT_OK = 0,
    BC_INIT_ENV_TOO_BIG,
    BC_INIT_CANNOT_ACCOMODATE_HEADROOM,
}
impl BC_INIT_STATUS {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            BC_INIT_STATUS::BC_INIT_OK => 0,
            BC_INIT_STATUS::BC_INIT_ENV_TOO_BIG => 1,
            BC_INIT_STATUS::BC_INIT_CANNOT_ACCOMODATE_HEADROOM => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> BC_INIT_STATUS {
        match value {
            0 => BC_INIT_STATUS::BC_INIT_OK,
            1 => BC_INIT_STATUS::BC_INIT_ENV_TOO_BIG,
            2 => BC_INIT_STATUS::BC_INIT_CANNOT_ACCOMODATE_HEADROOM,
            _ => panic!("Invalid value for BC_INIT_STATUS: {}", value),
        }
    }
}
impl AddAssign<u32> for BC_INIT_STATUS {
    fn add_assign(&mut self, rhs: u32) {
        *self = BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for BC_INIT_STATUS {
    fn sub_assign(&mut self, rhs: u32) {
        *self = BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for BC_INIT_STATUS {
    fn mul_assign(&mut self, rhs: u32) {
        *self = BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for BC_INIT_STATUS {
    fn div_assign(&mut self, rhs: u32) {
        *self = BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for BC_INIT_STATUS {
    fn rem_assign(&mut self, rhs: u32) {
        *self = BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for BC_INIT_STATUS {
    type Output = BC_INIT_STATUS;
    fn add(self, rhs: u32) -> BC_INIT_STATUS {
        BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for BC_INIT_STATUS {
    type Output = BC_INIT_STATUS;
    fn sub(self, rhs: u32) -> BC_INIT_STATUS {
        BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for BC_INIT_STATUS {
    type Output = BC_INIT_STATUS;
    fn mul(self, rhs: u32) -> BC_INIT_STATUS {
        BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for BC_INIT_STATUS {
    type Output = BC_INIT_STATUS;
    fn div(self, rhs: u32) -> BC_INIT_STATUS {
        BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for BC_INIT_STATUS {
    type Output = BC_INIT_STATUS;
    fn rem(self, rhs: u32) -> BC_INIT_STATUS {
        BC_INIT_STATUS::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum LongOptionIdentifier {
    PROCESS_SLOT_VAR,
}
impl LongOptionIdentifier {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            LongOptionIdentifier::PROCESS_SLOT_VAR => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> LongOptionIdentifier {
        match value {
            128 => LongOptionIdentifier::PROCESS_SLOT_VAR,
            _ => panic!("Invalid value for LongOptionIdentifier: {}", value),
        }
    }
}
impl AddAssign<u32> for LongOptionIdentifier {
    fn add_assign(&mut self, rhs: u32) {
        *self = LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for LongOptionIdentifier {
    fn sub_assign(&mut self, rhs: u32) {
        *self = LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for LongOptionIdentifier {
    fn mul_assign(&mut self, rhs: u32) {
        *self = LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for LongOptionIdentifier {
    fn div_assign(&mut self, rhs: u32) {
        *self = LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for LongOptionIdentifier {
    fn rem_assign(&mut self, rhs: u32) {
        *self = LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for LongOptionIdentifier {
    type Output = LongOptionIdentifier;
    fn add(self, rhs: u32) -> LongOptionIdentifier {
        LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for LongOptionIdentifier {
    type Output = LongOptionIdentifier;
    fn sub(self, rhs: u32) -> LongOptionIdentifier {
        LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for LongOptionIdentifier {
    type Output = LongOptionIdentifier;
    fn mul(self, rhs: u32) -> LongOptionIdentifier {
        LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for LongOptionIdentifier {
    type Output = LongOptionIdentifier;
    fn div(self, rhs: u32) -> LongOptionIdentifier {
        LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for LongOptionIdentifier {
    type Output = LongOptionIdentifier;
    fn rem(self, rhs: u32) -> LongOptionIdentifier {
        LongOptionIdentifier::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum XargsStatusValues {
    XARGS_EXIT_CLIENT_EXIT_NONZERO = 123,
    XARGS_EXIT_CLIENT_EXIT_255 = 124,
    XARGS_EXIT_CLIENT_FATAL_SIG = 125,
    XARGS_EXIT_COMMAND_CANNOT_BE_RUN = 126,
    XARGS_EXIT_COMMAND_NOT_FOUND = 127,
}
impl XargsStatusValues {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_NONZERO => 123,
            XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_255 => 124,
            XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG => 125,
            XargsStatusValues::XARGS_EXIT_COMMAND_CANNOT_BE_RUN => 126,
            XargsStatusValues::XARGS_EXIT_COMMAND_NOT_FOUND => 127,
        }
    }
    fn from_libc_c_uint(value: u32) -> XargsStatusValues {
        match value {
            123 => XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_NONZERO,
            124 => XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_255,
            125 => XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG,
            126 => XargsStatusValues::XARGS_EXIT_COMMAND_CANNOT_BE_RUN,
            127 => XargsStatusValues::XARGS_EXIT_COMMAND_NOT_FOUND,
            _ => panic!("Invalid value for XargsStatusValues: {}", value),
        }
    }
}
impl AddAssign<u32> for XargsStatusValues {
    fn add_assign(&mut self, rhs: u32) {
        *self = XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for XargsStatusValues {
    fn sub_assign(&mut self, rhs: u32) {
        *self = XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for XargsStatusValues {
    fn mul_assign(&mut self, rhs: u32) {
        *self = XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for XargsStatusValues {
    fn div_assign(&mut self, rhs: u32) {
        *self = XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for XargsStatusValues {
    fn rem_assign(&mut self, rhs: u32) {
        *self = XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for XargsStatusValues {
    type Output = XargsStatusValues;
    fn add(self, rhs: u32) -> XargsStatusValues {
        XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for XargsStatusValues {
    type Output = XargsStatusValues;
    fn sub(self, rhs: u32) -> XargsStatusValues {
        XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for XargsStatusValues {
    type Output = XargsStatusValues;
    fn mul(self, rhs: u32) -> XargsStatusValues {
        XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for XargsStatusValues {
    type Output = XargsStatusValues;
    fn div(self, rhs: u32) -> XargsStatusValues {
        XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for XargsStatusValues {
    type Output = XargsStatusValues;
    fn rem(self, rhs: u32) -> XargsStatusValues {
        XargsStatusValues::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ClientStatusValues {
    CHILD_EXIT_PLEASE_STOP_IMMEDIATELY = 255,
}
impl ClientStatusValues {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ClientStatusValues::CHILD_EXIT_PLEASE_STOP_IMMEDIATELY => 255,
        }
    }
    fn from_libc_c_uint(value: u32) -> ClientStatusValues {
        match value {
            255 => ClientStatusValues::CHILD_EXIT_PLEASE_STOP_IMMEDIATELY,
            _ => panic!("Invalid value for ClientStatusValues: {}", value),
        }
    }
}
impl AddAssign<u32> for ClientStatusValues {
    fn add_assign(&mut self, rhs: u32) {
        *self = ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ClientStatusValues {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ClientStatusValues {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ClientStatusValues {
    fn div_assign(&mut self, rhs: u32) {
        *self = ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ClientStatusValues {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ClientStatusValues {
    type Output = ClientStatusValues;
    fn add(self, rhs: u32) -> ClientStatusValues {
        ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ClientStatusValues {
    type Output = ClientStatusValues;
    fn sub(self, rhs: u32) -> ClientStatusValues {
        ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ClientStatusValues {
    type Output = ClientStatusValues;
    fn mul(self, rhs: u32) -> ClientStatusValues {
        ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ClientStatusValues {
    type Output = ClientStatusValues;
    fn div(self, rhs: u32) -> ClientStatusValues {
        ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ClientStatusValues {
    type Output = ClientStatusValues;
    fn rem(self, rhs: u32) -> ClientStatusValues {
        ClientStatusValues::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum read_line_state {
    NORM = 0,
    SPACE = 1,
    QUOTE = 2,
    BACKSLASH = 3,
}
impl read_line_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            read_line_state::NORM => 0,
            read_line_state::SPACE => 1,
            read_line_state::QUOTE => 2,
            read_line_state::BACKSLASH => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> read_line_state {
        match value {
            0 => read_line_state::NORM,
            1 => read_line_state::SPACE,
            2 => read_line_state::QUOTE,
            3 => read_line_state::BACKSLASH,
            _ => panic!("Invalid value for read_line_state: {}", value),
        }
    }
}
impl AddAssign<u32> for read_line_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = read_line_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for read_line_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = read_line_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for read_line_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = read_line_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for read_line_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = read_line_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for read_line_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = read_line_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for read_line_state {
    type Output = read_line_state;
    fn add(self, rhs: u32) -> read_line_state {
        read_line_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for read_line_state {
    type Output = read_line_state;
    fn sub(self, rhs: u32) -> read_line_state {
        read_line_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for read_line_state {
    type Output = read_line_state;
    fn mul(self, rhs: u32) -> read_line_state {
        read_line_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for read_line_state {
    type Output = read_line_state;
    fn div(self, rhs: u32) -> read_line_state {
        read_line_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for read_line_state {
    type Output = read_line_state;
    fn rem(self, rhs: u32) -> read_line_state {
        read_line_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub _gl_dummy: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_36 {
    XARGS_POSIX_HEADROOM = 2048,
}
impl C2RustUnnamed_36 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_36::XARGS_POSIX_HEADROOM => 2048,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_36 {
        match value {
            2048 => C2RustUnnamed_36::XARGS_POSIX_HEADROOM,
            _ => panic!("Invalid value for C2RustUnnamed_36: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_36 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_36 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_36 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_36 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_36 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_36 {
    type Output = C2RustUnnamed_36;
    fn add(self, rhs: u32) -> C2RustUnnamed_36 {
        C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_36 {
    type Output = C2RustUnnamed_36;
    fn sub(self, rhs: u32) -> C2RustUnnamed_36 {
        C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_36 {
    type Output = C2RustUnnamed_36;
    fn mul(self, rhs: u32) -> C2RustUnnamed_36 {
        C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_36 {
    type Output = C2RustUnnamed_36;
    fn div(self, rhs: u32) -> C2RustUnnamed_36 {
        C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_36 {
    type Output = C2RustUnnamed_36;
    fn rem(self, rhs: u32) -> C2RustUnnamed_36 {
        C2RustUnnamed_36::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub _gl_dummy: i32,
}
static mut input_stream: *mut FILE = 0 as *const FILE as *mut FILE;
static mut linebuf: *mut i8 = 0 as *const i8 as *mut i8;
static mut keep_stdin: i32 = 0 as i32;
static mut lineno: size_t = 0 as i32 as size_t;
static mut bc_state: buildcmd_state = buildcmd_state {
    cmd_argc: 0,
    cmd_argv: 0 as *const *mut i8 as *mut *mut i8,
    cmd_argv_alloc: 0,
    argbuf: 0 as *const i8 as *mut i8,
    cmd_argv_chars: 0,
    cmd_initial_argv_chars: 0,
    usercontext: 0 as *const libc::c_void as *mut libc::c_void,
    todo: 0,
    dir_fd: 0,
    largest_successful_arg_count: 0,
    smallest_failed_arg_count: 0,
};
static mut bc_ctl: buildcmd_control = buildcmd_control {
    exit_if_size_exceeded: 0,
    posix_arg_size_max: 0,
    posix_arg_size_min: 0,
    arg_max: 0,
    max_arg_count: 0,
    rplen: 0,
    replace_pat: 0 as *const i8,
    initial_argc: 0,
    exec_callback: None,
    lines_per_exec: 0,
    args_per_exec: 0,
};
static mut nullwarning_given: i32 = 0 as i32;
static mut eof_str: *mut i8 = 0 as *const i8 as *mut i8;
static mut initial_args: bool = 1 as i32 != 0;
static mut proc_max: sig_atomic_t = 1 as i32;
static mut procs_executed: bool = 0 as i32 != 0;
static mut procs_executing: u64 = 0 as u64;
static mut pids: *mut pid_t = 0 as *const pid_t as *mut pid_t;
static mut pids_alloc: size_t = 0 as u32 as size_t;
static mut parent: pid_t = 0;
static mut stop_waiting: sig_atomic_t = 0 as i32;
static mut child_error: i32 = 0 as i32;
static mut original_exit_value: i32 = 0;
static mut open_tty: bool = 0 as i32 != 0;
static mut print_command: bool = 0 as i32 != 0;
static mut query_before_executing: bool = 0 as i32 != 0;
static mut input_delimiter: i8 = '\0' as i32 as i8;
static mut slot_var_name: *mut i8 = 0 as *const i8 as *mut i8;
static mut longopts: [option; 19] = [
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"arg-file\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"delimiter\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"eof\0" as *const u8 as *const i8,
            has_arg: 2 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"replace\0" as *const u8 as *const i8,
            has_arg: 2 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-lines\0" as *const u8 as *const i8,
            has_arg: 2 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-args\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"open-tty\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interactive\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-run-if-empty\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-chars\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-limits\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"exit\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-procs\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"process-slot-var\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: LongOptionIdentifier::PROCESS_SLOT_VAR as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 0 as i32,
        };
        init
    },
];
unsafe extern "C" fn get_char_oct_or_hex_escape(mut s: *const i8) -> i8 {
    let mut p: *const i8 = 0 as *const i8;
    let mut base: i32 = 8 as i32;
    let mut val: u64 = 0;
    let mut endp: *mut i8 = 0 as *mut i8;
    if '\\' as i32 == *s.offset(0 as i32 as isize) as i32 {} else {
        __assert_fail(
            b"'\\\\' == s[0]\0" as *const u8 as *const i8,
            b"xargs.c\0" as *const u8 as *const i8,
            236 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[i8; 46],
            >(b"char get_char_oct_or_hex_escape(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_10800: {
        if '\\' as i32 == *s.offset(0 as i32 as isize) as i32 {} else {
            __assert_fail(
                b"'\\\\' == s[0]\0" as *const u8 as *const i8,
                b"xargs.c\0" as *const u8 as *const i8,
                236 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[i8; 46],
                >(b"char get_char_oct_or_hex_escape(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if 'x' as i32 == *s.offset(1 as i32 as isize) as i32 {
        p = s.offset(2 as i32 as isize);
        base = 16 as i32;
    } else if *(*__ctype_b_loc())
        .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize) as i32
        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        p = s.offset(1 as i32 as isize);
        base = 8 as i32;
    } else {
        p = 0 as *const i8;
        if ::core::mem::size_of::<C2RustUnnamed_29>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid escape sequence %s in input delimiter specification.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                s,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid escape sequence %s in input delimiter specification.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                s,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    *__errno_location() = 0 as i32;
    endp = 0 as *mut i8;
    val = strtoul(p, &mut endp, base);
    if (9223372036854775807 as i64 as u64).wrapping_mul(2 as u64).wrapping_add(1 as u64)
        == val && 34 as i32 == *__errno_location()
        || val > (127 as i32 * 2 as i32 + 1 as i32) as u64
    {
        if 16 as i32 == base {
            if ::core::mem::size_of::<C2RustUnnamed_28>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lx.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    s,
                    (127 as i32 * 2 as i32 + 1 as i32) as u64,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lx.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    s,
                    (127 as i32 * 2 as i32 + 1 as i32) as u64,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_27>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lo.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    s,
                    (127 as i32 * 2 as i32 + 1 as i32) as u64,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lo.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    s,
                    (127 as i32 * 2 as i32 + 1 as i32) as u64,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if 0 as i32 != *endp as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_26>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid escape sequence %s in input delimiter specification; trailing characters %s not recognised.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                s,
                endp,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid escape sequence %s in input delimiter specification; trailing characters %s not recognised.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                s,
                endp,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    return val as i8;
}
unsafe extern "C" fn get_input_delimiter(mut s: *const i8) -> i8 {
    if 1 as i32 as u64 == strlen(s) {
        return *s.offset(0 as i32 as isize)
    } else if '\\' as i32 == *s.offset(0 as i32 as isize) as i32 {
        match *s.offset(1 as i32 as isize) as i32 {
            97 => return '\u{7}' as i32 as i8,
            98 => return '\u{8}' as i32 as i8,
            102 => return '\u{c}' as i32 as i8,
            110 => return '\n' as i32 as i8,
            114 => return '\r' as i32 as i8,
            116 => return '\t' as i32 as i8,
            118 => return '\u{b}' as i32 as i8,
            92 => return '\\' as i32 as i8,
            _ => return get_char_oct_or_hex_escape(s),
        }
    } else {
        if ::core::mem::size_of::<C2RustUnnamed_30>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid input delimiter specification %s: the delimiter must be either a single character or an escape sequence starting with \\.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                s,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Invalid input delimiter specification %s: the delimiter must be either a single character or an escape sequence starting with \\.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                s,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
        return 0 as i32 as i8;
    };
}
unsafe extern "C" fn noop() {}
unsafe extern "C" fn fail_due_to_env_size() {
    if ::core::mem::size_of::<C2RustUnnamed_31>() as u64 != 0 {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"environment is too large for exec\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"environment is too large for exec\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn smaller_of(mut a: size_t, mut b: size_t) -> size_t {
    if a < b { return a } else { return b };
}
unsafe extern "C" fn fopen_cloexec_for_read_only(mut file_name: *const i8) -> *mut FILE {
    let mut fd: i32 = open_cloexec(file_name, 0 as i32);
    if fd < 0 as i32 {
        return 0 as *mut FILE
    } else {
        let mut result: *mut FILE = fdopen(fd, b"r\0" as *const u8 as *const i8);
        if result.is_null() {
            let mut saved_errno: i32 = *__errno_location();
            close(fd);
            *__errno_location() = saved_errno;
            return 0 as *mut FILE;
        }
        return result;
    };
}
unsafe extern "C" fn warn_mutually_exclusive(
    mut option: *const i8,
    mut offending: *const i8,
) {
    error(
        0 as i32,
        0 as i32,
        dcgettext(
            0 as *const i8,
            b"warning: options %s and %s are mutually exclusive, ignoring previous %s value\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        offending,
        option,
        offending,
    );
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut optc: i32 = 0;
    let mut option_index: i32 = 0;
    let mut show_limits: i32 = 0 as i32;
    let mut always_run_command: i32 = 1 as i32;
    let mut input_file: *const i8 = b"-\0" as *const u8 as *const i8;
    let mut default_cmd: [i8; 5] = *::core::mem::transmute::<
        &[u8; 5],
        &mut [i8; 5],
    >(b"echo\0");
    let mut default_arglist: [*mut i8; 1] = [0 as *mut i8; 1];
    let mut read_args: Option<unsafe extern "C" fn() -> i32> = Some(
        read_line as unsafe extern "C" fn() -> i32,
    );
    let mut act_on_init_result: Option<unsafe extern "C" fn() -> ()> = Some(
        noop as unsafe extern "C" fn() -> (),
    );
    let mut bcstatus: BC_INIT_STATUS = BC_INIT_STATUS::BC_INIT_OK;
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    if !(*argv.offset(0 as i32 as isize)).is_null() {
        set_program_name(*argv.offset(0 as i32 as isize));
    } else {
        set_program_name(b"xargs\0" as *const u8 as *const i8);
    }
    remember_non_cloexec_fds();
    parent = getpid();
    ::core::ptr::write_volatile(&mut original_exit_value as *mut i32, 0 as i32);
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"findutils\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"findutils\0" as *const u8 as *const i8);
    if atexit(Some(close_stdin as unsafe extern "C" fn() -> ())) != 0
        || atexit(Some(wait_for_proc_all as unsafe extern "C" fn() -> ())) != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_35>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    bcstatus = bc_init_controlinfo(
        &mut bc_ctl,
        C2RustUnnamed_36::XARGS_POSIX_HEADROOM as i32 as size_t,
    );
    if BC_INIT_STATUS::BC_INIT_ENV_TOO_BIG as i32 as u32 == bcstatus as u32 {
        act_on_init_result = Some(fail_due_to_env_size as unsafe extern "C" fn() -> ());
    } else if BC_INIT_STATUS::BC_INIT_CANNOT_ACCOMODATE_HEADROOM as i32 as u32
        == bcstatus as u32
    {
        act_on_init_result = Some(fail_due_to_env_size as unsafe extern "C" fn() -> ());
    } else {
        let mut val: i64 = 0;
        val = sysconf(_SC_ARG_MAX as i32);
        if val > 0 as i32 as i64 {
            if val > C2RustUnnamed_36::XARGS_POSIX_HEADROOM as i32 as i64 {} else {
                __assert_fail(
                    b"val > C2RustUnnamed_36::XARGS_POSIX_HEADROOM\0" as *const u8
                        as *const i8,
                    b"xargs.c\0" as *const u8 as *const i8,
                    483 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_12933: {
                if val > C2RustUnnamed_36::XARGS_POSIX_HEADROOM as i32 as i64 {} else {
                    __assert_fail(
                        b"val > C2RustUnnamed_36::XARGS_POSIX_HEADROOM\0" as *const u8
                            as *const i8,
                        b"xargs.c\0" as *const u8 as *const i8,
                        483 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[i8; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            bc_ctl.arg_max = smaller_of(
                bc_ctl.arg_max,
                (val as size_t)
                    .wrapping_sub(C2RustUnnamed_36::XARGS_POSIX_HEADROOM as i32 as u64),
            );
        }
        if bc_ctl.arg_max >= 2048 as i32 as u64 {} else {
            __assert_fail(
                b"bc_ctl.arg_max >= LINE_MAX\0" as *const u8 as *const i8,
                b"xargs.c\0" as *const u8 as *const i8,
                511 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_12869: {
            if bc_ctl.arg_max >= 2048 as i32 as u64 {} else {
                __assert_fail(
                    b"bc_ctl.arg_max >= LINE_MAX\0" as *const u8 as *const i8,
                    b"xargs.c\0" as *const u8 as *const i8,
                    511 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        bc_ctl.exec_callback = Some(
            xargs_do_exec
                as unsafe extern "C" fn(
                    *mut buildcmd_control,
                    *mut libc::c_void,
                    i32,
                    *mut *mut i8,
                ) -> i32,
        );
        bc_use_sensible_arg_max(&mut bc_ctl);
    }
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"+0a:E:e::i::I:l::L:n:oprs:txP:d:\0" as *const u8 as *const i8,
            longopts.as_ptr(),
            &mut option_index,
        );
        if !(optc != -(1 as i32)) {
            break;
        }
        match optc {
            48 => {
                read_args = Some(read_string as unsafe extern "C" fn() -> i32);
                input_delimiter = '\0' as i32 as i8;
            }
            100 => {
                read_args = Some(read_string as unsafe extern "C" fn() -> i32);
                input_delimiter = get_input_delimiter(optarg);
            }
            69 | 101 => {
                if !optarg.is_null() && strlen(optarg) > 0 as i32 as u64 {
                    eof_str = optarg;
                } else {
                    eof_str = 0 as *mut i8;
                }
            }
            104 => {
                usage(0 as i32);
            }
            73 | 105 => {
                if !optarg.is_null() {
                    bc_ctl.replace_pat = optarg;
                } else {
                    bc_ctl.replace_pat = b"{}\0" as *const u8 as *const i8;
                }
                if bc_ctl.args_per_exec != 0 as i32 as u64 {
                    warn_mutually_exclusive(
                        b"--replace/-I/-i\0" as *const u8 as *const i8,
                        b"--max-args\0" as *const u8 as *const i8,
                    );
                    bc_ctl.args_per_exec = 0 as i32 as size_t;
                }
                if bc_ctl.lines_per_exec != 0 as i32 as u64 {
                    warn_mutually_exclusive(
                        b"--replace/-I/-i\0" as *const u8 as *const i8,
                        b"--max-lines\0" as *const u8 as *const i8,
                    );
                    bc_ctl.lines_per_exec = 0 as i32 as u64;
                }
            }
            76 => {
                bc_ctl.lines_per_exec = parse_num(
                    optarg,
                    'L' as i32,
                    1 as i64,
                    -(1 as i64),
                    1 as i32,
                ) as u64;
                if bc_ctl.args_per_exec != 0 as i32 as u64 {
                    warn_mutually_exclusive(
                        b"-L\0" as *const u8 as *const i8,
                        b"--max-args\0" as *const u8 as *const i8,
                    );
                    bc_ctl.args_per_exec = 0 as i32 as size_t;
                }
                if !(bc_ctl.replace_pat).is_null() {
                    warn_mutually_exclusive(
                        b"-L\0" as *const u8 as *const i8,
                        b"--replace\0" as *const u8 as *const i8,
                    );
                    bc_ctl.replace_pat = 0 as *const i8;
                }
            }
            108 => {
                if !optarg.is_null() {
                    bc_ctl.lines_per_exec = parse_num(
                        optarg,
                        'l' as i32,
                        1 as i64,
                        -(1 as i64),
                        1 as i32,
                    ) as u64;
                } else {
                    bc_ctl.lines_per_exec = 1 as i32 as u64;
                }
                if bc_ctl.args_per_exec != 0 as i32 as u64 {
                    warn_mutually_exclusive(
                        b"--max-lines/-l\0" as *const u8 as *const i8,
                        b"--max-args\0" as *const u8 as *const i8,
                    );
                    bc_ctl.args_per_exec = 0 as i32 as size_t;
                }
                if !(bc_ctl.replace_pat).is_null() {
                    warn_mutually_exclusive(
                        b"--max-lines/-l\0" as *const u8 as *const i8,
                        b"--replace\0" as *const u8 as *const i8,
                    );
                    bc_ctl.replace_pat = 0 as *const i8;
                }
            }
            110 => {
                bc_ctl.args_per_exec = parse_num(
                    optarg,
                    'n' as i32,
                    1 as i64,
                    -(1 as i64),
                    1 as i32,
                ) as size_t;
                if bc_ctl.lines_per_exec != 0 as i32 as u64 {
                    warn_mutually_exclusive(
                        b"--max-args/-n\0" as *const u8 as *const i8,
                        b"--max-lines\0" as *const u8 as *const i8,
                    );
                    bc_ctl.lines_per_exec = 0 as i32 as u64;
                }
                if !(bc_ctl.replace_pat).is_null() {
                    if bc_ctl.args_per_exec == 1 as i32 as u64 {
                        bc_ctl.args_per_exec = 0 as i32 as size_t;
                    } else {
                        warn_mutually_exclusive(
                            b"--max-args/-n\0" as *const u8 as *const i8,
                            b"--replace\0" as *const u8 as *const i8,
                        );
                        bc_ctl.replace_pat = 0 as *const i8;
                    }
                }
            }
            115 => {
                let mut arg_size: size_t = 0;
                act_on_init_result.expect("non-null function pointer")();
                arg_size = parse_num(
                    optarg,
                    's' as i32,
                    1 as i64,
                    bc_ctl.posix_arg_size_max as i64,
                    0 as i32,
                ) as size_t;
                if arg_size > bc_ctl.posix_arg_size_max {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"warning: value %ld for -s option is too large, using %ld instead\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        arg_size as i64,
                        bc_ctl.posix_arg_size_max as i64,
                    );
                    arg_size = bc_ctl.posix_arg_size_max;
                }
                bc_ctl.arg_max = arg_size;
            }
            83 => {
                show_limits = 1 as i32;
            }
            116 => {
                print_command = 1 as i32 != 0;
            }
            120 => {
                bc_ctl.exit_if_size_exceeded = 1 as i32;
            }
            111 => {
                open_tty = 1 as i32 != 0;
            }
            112 => {
                query_before_executing = 1 as i32 != 0;
                print_command = 1 as i32 != 0;
            }
            114 => {
                always_run_command = 0 as i32;
            }
            80 => {
                ::core::ptr::write_volatile(
                    &mut proc_max as *mut sig_atomic_t,
                    parse_num(
                        optarg,
                        'P' as i32,
                        0 as i64,
                        2147483647 as i32 as i64,
                        1 as i32,
                    ) as sig_atomic_t,
                );
            }
            97 => {
                input_file = optarg;
            }
            118 => {
                display_findutils_version(b"xargs\0" as *const u8 as *const i8);
                return 0 as i32;
            }
            128 => {
                if !(strchr(optarg, '=' as i32)).is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_34>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"option --%s may not be set to a value which includes `='\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            longopts[option_index as usize].name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"option --%s may not be set to a value which includes `='\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            longopts[option_index as usize].name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                slot_var_name = optarg;
                if 0 as i32 != unsetenv(slot_var_name) {
                    if ::core::mem::size_of::<C2RustUnnamed_33>() as u64 != 0 {
                        error(
                            1 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"failed to unset environment variable %s\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            slot_var_name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"failed to unset environment variable %s\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            slot_var_name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    if !eof_str.is_null()
        && read_args == Some(read_string as unsafe extern "C" fn() -> i32)
    {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"warning: the -E option has no effect if -0 or -d is used.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    act_on_init_result.expect("non-null function pointer")();
    if BC_INIT_STATUS::BC_INIT_OK as i32 as u32 == bcstatus as u32 {} else {
        __assert_fail(
            b"BC_INIT_STATUS::BC_INIT_OK == bcstatus\0" as *const u8 as *const i8,
            b"xargs.c\0" as *const u8 as *const i8,
            723 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_12024: {
        if BC_INIT_STATUS::BC_INIT_OK as i32 as u32 == bcstatus as u32 {} else {
            __assert_fail(
                b"BC_INIT_STATUS::BC_INIT_OK == bcstatus\0" as *const u8 as *const i8,
                b"xargs.c\0" as *const u8 as *const i8,
                723 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    sigact.__sigaction_handler.sa_handler = Some(
        increment_proc_max as unsafe extern "C" fn(i32) -> (),
    );
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 0 as i32;
    if 0 as i32
        != sigaction(10 as i32, &mut sigact, 0 as *mut libc::c_void as *mut sigaction)
    {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"Cannot set SIGUSR1 signal handler\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    sigact.__sigaction_handler.sa_handler = Some(
        decrement_proc_max as unsafe extern "C" fn(i32) -> (),
    );
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 0 as i32;
    if 0 as i32
        != sigaction(12 as i32, &mut sigact, 0 as *mut libc::c_void as *mut sigaction)
    {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"Cannot set SIGUSR2 signal handler\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if 0 as i32 == strcmp(input_file, b"-\0" as *const u8 as *const i8) {
        input_stream = stdin;
    } else {
        keep_stdin = 1 as i32;
        input_stream = fopen_cloexec_for_read_only(input_file);
        if input_stream.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_32>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Cannot open input file %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quotearg_n_style(
                        0 as i32,
                        quoting_style::locale_quoting_style,
                        input_file,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Cannot open input file %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quotearg_n_style(
                        0 as i32,
                        quoting_style::locale_quoting_style,
                        input_file,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if !(bc_ctl.replace_pat).is_null() || bc_ctl.lines_per_exec != 0 {
        bc_ctl.exit_if_size_exceeded = 1 as i32;
    }
    if optind == argc {
        optind = 0 as i32;
        argc = 1 as i32;
        default_arglist[0 as i32 as usize] = default_cmd.as_mut_ptr();
        argv = default_arglist.as_mut_ptr();
    }
    if show_limits != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Your environment variables take up %lu bytes\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            bc_size_of_environment(),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"POSIX upper limit on argument length (this system): %lu\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            bc_ctl.posix_arg_size_max,
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"POSIX smallest allowable upper limit on argument length (all systems): %lu\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            bc_ctl.posix_arg_size_min,
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Maximum length of command we could actually use: %lu\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (bc_ctl.posix_arg_size_max).wrapping_sub(bc_size_of_environment()),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Size of command buffer we are actually using: %lu\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            bc_ctl.arg_max,
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Maximum parallelism (--max-procs must be no greater): %lu\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            2147483647 as i32 as uintmax_t,
        );
        if isatty(0 as i32) != 0 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"\nExecution of xargs will continue now, and it will try to read its input and run commands; if this is not what you wanted to happen, please type the end-of-file keystroke.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if always_run_command != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"Warning: %s will be run at least once.  If you do not want that to happen, then press the interrupt keystroke.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(optind as isize),
                );
            }
        }
    }
    linebuf = xmalloc((bc_ctl.arg_max).wrapping_add(1 as i32 as u64)) as *mut i8;
    bc_state.argbuf = xmalloc((bc_ctl.arg_max).wrapping_add(1 as i32 as u64)) as *mut i8;
    signal(17 as i32, None);
    if (bc_ctl.replace_pat).is_null() {
        while optind < argc {
            bc_push_arg(
                &mut bc_ctl,
                &mut bc_state,
                *argv.offset(optind as isize),
                (strlen(*argv.offset(optind as isize))).wrapping_add(1 as i32 as u64),
                0 as *const i8,
                0 as i32 as size_t,
                initial_args as i32,
            );
            optind += 1;
            optind;
        }
        initial_args = 0 as i32 != 0;
        bc_ctl.initial_argc = bc_state.cmd_argc;
        bc_state.cmd_initial_argv_chars = bc_state.cmd_argv_chars;
        bc_ctl.initial_argc = bc_state.cmd_argc;
        while (Some(read_args.expect("non-null function pointer")))
            .expect("non-null function pointer")() != -(1 as i32)
        {
            if bc_ctl.lines_per_exec != 0 && lineno >= bc_ctl.lines_per_exec {
                bc_do_exec(&mut bc_ctl, &mut bc_state);
                lineno = 0 as i32 as size_t;
            }
        }
        if bc_state.cmd_argc != bc_ctl.initial_argc
            || always_run_command != 0 && procs_executed as i32 == 0 as i32
        {
            bc_do_exec(&mut bc_ctl, &mut bc_state);
        }
    } else {
        let mut i: i32 = 0;
        let mut args: i32 = 0;
        let mut arglen: *mut size_t = xmalloc(
            (::core::mem::size_of::<size_t>() as u64).wrapping_mul(argc as u64),
        ) as *mut size_t;
        i = optind;
        while i < argc {
            *arglen.offset(i as isize) = strlen(*argv.offset(i as isize));
            i += 1;
            i;
        }
        bc_ctl.rplen = strlen(bc_ctl.replace_pat);
        loop {
            args = (Some(read_args.expect("non-null function pointer")))
                .expect("non-null function pointer")();
            if !(args != -(1 as i32)) {
                break;
            }
            let mut len: size_t = args as size_t;
            bc_clear_args(&mut bc_ctl, &mut bc_state);
            bc_state.cmd_argv_chars = 0 as i32 as size_t;
            bc_push_arg(
                &mut bc_ctl,
                &mut bc_state,
                *argv.offset(optind as isize),
                (*arglen.offset(optind as isize)).wrapping_add(1 as i32 as u64),
                0 as *const i8,
                0 as i32 as size_t,
                initial_args as i32,
            );
            len = len.wrapping_sub(1);
            len;
            initial_args = 0 as i32 != 0;
            i = optind + 1 as i32;
            while i < argc {
                bc_do_insert(
                    &mut bc_ctl,
                    &mut bc_state,
                    *argv.offset(i as isize),
                    *arglen.offset(i as isize),
                    0 as *const i8,
                    0 as i32 as size_t,
                    linebuf,
                    len,
                    initial_args as i32,
                );
                i += 1;
                i;
            }
            bc_do_exec(&mut bc_ctl, &mut bc_state);
        }
    }
    ::core::ptr::write_volatile(&mut original_exit_value as *mut i32, child_error);
    return child_error;
}
unsafe extern "C" fn read_line() -> i32 {
    static mut eof: bool = 0 as i32 != 0;
    let mut state: read_line_state = read_line_state::SPACE;
    let mut prevc: i32 = 0;
    let mut quotc: i32 = 0 as i32;
    let mut c: i32 = -(1 as i32);
    let mut first: bool = 1 as i32 != 0;
    let mut seen_arg: bool = 0 as i32 != 0;
    let mut len: i32 = 0;
    let mut p: *mut i8 = linebuf;
    let mut endbuf: *mut i8 = linebuf
        .offset(bc_ctl.arg_max as isize)
        .offset(-(bc_state.cmd_initial_argv_chars as isize))
        .offset(-(1 as i32 as isize));
    if eof {
        return -(1 as i32);
    }
    let mut current_block_64: u64;
    loop {
        prevc = c;
        c = _IO_getc(input_stream);
        if c == -(1 as i32) {
            eof = 1 as i32 != 0;
            if p == linebuf {
                return -(1 as i32);
            }
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\0' as i32 as i8;
            len = p.offset_from(linebuf) as i64 as i32;
            if state as u32 == read_line_state::QUOTE as i32 as u32 {
                exec_if_possible();
                if ::core::mem::size_of::<C2RustUnnamed_14>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (if quotc == '"' as i32 {
                            dcgettext(
                                0 as *const i8,
                                b"double\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        } else {
                            dcgettext(
                                0 as *const i8,
                                b"single\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        }),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (if quotc == '"' as i32 {
                            dcgettext(
                                0 as *const i8,
                                b"double\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        } else {
                            dcgettext(
                                0 as *const i8,
                                b"single\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        }),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if first as i32 != 0
                && (!eof_str.is_null() && *eof_str as i32 == *linebuf as i32
                    && strcmp(eof_str, linebuf) == 0)
            {
                return -(1 as i32);
            }
            if (bc_ctl.replace_pat).is_null() {
                bc_push_arg(
                    &mut bc_ctl,
                    &mut bc_state,
                    linebuf,
                    len as size_t,
                    0 as *const i8,
                    0 as i32 as size_t,
                    initial_args as i32,
                );
            }
            return len;
        }
        match state as u32 {
            1 => {
                if c & !(0x7f as i32) == 0 as i32
                    && *(*__ctype_b_loc()).offset(c as isize) as i32
                        & C2RustUnnamed::_ISblank as i32 as libc::c_ushort as i32 != 0
                    || c == '\n' as i32 || c == '\r' as i32 || c == '\u{c}' as i32
                    || c == '\u{b}' as i32
                {
                    continue;
                }
                state = read_line_state::NORM;
                current_block_64 = 5609494561082130361;
            }
            0 => {
                current_block_64 = 5609494561082130361;
            }
            2 => {
                if c == '\n' as i32 {
                    exec_if_possible();
                    if ::core::mem::size_of::<C2RustUnnamed_13>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (if quotc == '"' as i32 {
                                dcgettext(
                                    0 as *const i8,
                                    b"double\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            } else {
                                dcgettext(
                                    0 as *const i8,
                                    b"single\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            }),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (if quotc == '"' as i32 {
                                dcgettext(
                                    0 as *const i8,
                                    b"double\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            } else {
                                dcgettext(
                                    0 as *const i8,
                                    b"single\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            }),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if c == quotc {
                    state = read_line_state::NORM;
                    seen_arg = 1 as i32 != 0;
                    continue;
                } else {
                    current_block_64 = 13325891313334703151;
                }
            }
            3 => {
                state = read_line_state::NORM;
                current_block_64 = 13325891313334703151;
            }
            _ => {
                current_block_64 = 13325891313334703151;
            }
        }
        match current_block_64 {
            5609494561082130361 => {
                if c == '\n' as i32 {
                    if !(prevc & !(0x7f as i32) == 0 as i32
                        && *(*__ctype_b_loc()).offset(prevc as isize) as i32
                            & C2RustUnnamed::_ISblank as i32 as libc::c_ushort as i32
                            != 0)
                    {
                        lineno = lineno.wrapping_add(1);
                        lineno;
                    }
                    if p == linebuf {
                        if !seen_arg {
                            state = read_line_state::SPACE;
                            continue;
                        }
                    }
                    let fresh1 = p;
                    p = p.offset(1);
                    *fresh1 = '\0' as i32 as i8;
                    len = p.offset_from(linebuf) as i64 as i32;
                    if !eof_str.is_null() && *eof_str as i32 == *linebuf as i32
                        && strcmp(eof_str, linebuf) == 0
                    {
                        eof = 1 as i32 != 0;
                        return if first as i32 != 0 { -(1 as i32) } else { len };
                    }
                    if (bc_ctl.replace_pat).is_null() {
                        bc_push_arg(
                            &mut bc_ctl,
                            &mut bc_state,
                            linebuf,
                            len as size_t,
                            0 as *const i8,
                            0 as i32 as size_t,
                            initial_args as i32,
                        );
                    }
                    return len;
                } else {
                    seen_arg = 1 as i32 != 0;
                    if (bc_ctl.replace_pat).is_null()
                        && (c & !(0x7f as i32) == 0 as i32
                            && *(*__ctype_b_loc()).offset(c as isize) as i32
                                & C2RustUnnamed::_ISblank as i32 as libc::c_ushort as i32
                                != 0)
                    {
                        let fresh2 = p;
                        p = p.offset(1);
                        *fresh2 = '\0' as i32 as i8;
                        len = p.offset_from(linebuf) as i64 as i32;
                        if !eof_str.is_null() && *eof_str as i32 == *linebuf as i32
                            && strcmp(eof_str, linebuf) == 0
                        {
                            eof = 1 as i32 != 0;
                            return if first as i32 != 0 { -(1 as i32) } else { len };
                        }
                        bc_push_arg(
                            &mut bc_ctl,
                            &mut bc_state,
                            linebuf,
                            len as size_t,
                            0 as *const i8,
                            0 as i32 as size_t,
                            initial_args as i32,
                        );
                        p = linebuf;
                        state = read_line_state::SPACE;
                        first = 0 as i32 != 0;
                        continue;
                    } else {
                        match c {
                            92 => {
                                current_block_64 = 1614245230000234774;
                                match current_block_64 {
                                    1614245230000234774 => {
                                        state = read_line_state::BACKSLASH;
                                        continue;
                                    }
                                    _ => {
                                        state = read_line_state::QUOTE;
                                        quotc = c;
                                        continue;
                                    }
                                }
                            }
                            39 | 34 => {
                                current_block_64 = 4187689338455206605;
                                match current_block_64 {
                                    1614245230000234774 => {
                                        state = read_line_state::BACKSLASH;
                                        continue;
                                    }
                                    _ => {
                                        state = read_line_state::QUOTE;
                                        quotc = c;
                                        continue;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        if 0 as i32 == c && nullwarning_given == 0 {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"WARNING: a NUL character occurred in the input.  It cannot be passed through in the argument list.  Did you mean to use the --null option?\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            nullwarning_given = 1 as i32;
        }
        if p >= endbuf {
            exec_if_possible();
            if ::core::mem::size_of::<C2RustUnnamed_12>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"argument line too long\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"argument line too long\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = c as i8;
    };
}
unsafe extern "C" fn read_string() -> i32 {
    static mut eof: bool = 0 as i32 != 0;
    let mut len: i32 = 0;
    let mut p: *mut i8 = linebuf;
    let mut endbuf: *mut i8 = linebuf
        .offset(bc_ctl.arg_max as isize)
        .offset(-(bc_state.cmd_initial_argv_chars as isize))
        .offset(-(1 as i32 as isize));
    if eof {
        return -(1 as i32);
    }
    loop {
        let mut c: i32 = _IO_getc(input_stream);
        if c == -(1 as i32) {
            eof = 1 as i32 != 0;
            if p == linebuf {
                return -(1 as i32);
            }
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = '\0' as i32 as i8;
            len = p.offset_from(linebuf) as i64 as i32;
            if (bc_ctl.replace_pat).is_null() {
                bc_push_arg(
                    &mut bc_ctl,
                    &mut bc_state,
                    linebuf,
                    len as size_t,
                    0 as *const i8,
                    0 as i32 as size_t,
                    initial_args as i32,
                );
            }
            return len;
        }
        if c == input_delimiter as i32 {
            lineno = lineno.wrapping_add(1);
            lineno;
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = '\0' as i32 as i8;
            len = p.offset_from(linebuf) as i64 as i32;
            if (bc_ctl.replace_pat).is_null() {
                bc_push_arg(
                    &mut bc_ctl,
                    &mut bc_state,
                    linebuf,
                    len as size_t,
                    0 as *const i8,
                    0 as i32 as size_t,
                    initial_args as i32,
                );
            }
            return len;
        }
        if p >= endbuf {
            exec_if_possible();
            if ::core::mem::size_of::<C2RustUnnamed_15>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"argument line too long\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"argument line too long\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = c as i8;
    };
}
unsafe extern "C" fn print_args(mut ask: bool) -> bool {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < (bc_state.cmd_argc).wrapping_sub(1 as i32 as u64) {
        if fprintf(
            stderr,
            b"%s%s\0" as *const u8 as *const i8,
            (if i == 0 as i32 as u64 {
                b"\0" as *const u8 as *const i8
            } else {
                b" \0" as *const u8 as *const i8
            }),
            quotearg_n_style(
                0 as i32,
                quoting_style::shell_escape_quoting_style,
                *(bc_state.cmd_argv).offset(i as isize),
            ),
        ) < 0 as i32
        {
            if ::core::mem::size_of::<C2RustUnnamed_19>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to write to stderr\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to write to stderr\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        i = i.wrapping_add(1);
        i;
    }
    if ask {
        static mut tty_stream: *mut FILE = 0 as *const FILE as *mut FILE;
        let mut c: i32 = 0;
        let mut savec: i32 = 0;
        if tty_stream.is_null() {
            tty_stream = fopen_cloexec_for_read_only(
                b"/dev/tty\0" as *const u8 as *const i8,
            );
            if tty_stream.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_18>() as u64 != 0 {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"failed to open /dev/tty for reading\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"failed to open /dev/tty for reading\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        fputs(b"?...\0" as *const u8 as *const i8, stderr);
        if rpl_fflush(stderr) != 0 as i32 {
            if ::core::mem::size_of::<C2RustUnnamed_17>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to write to stderr\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to write to stderr\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        savec = _IO_getc(tty_stream);
        c = savec;
        while c != -(1 as i32) && c != '\n' as i32 {
            c = _IO_getc(tty_stream);
        }
        if -(1 as i32) == c {
            if ::core::mem::size_of::<C2RustUnnamed_16>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to read from stdin\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to read from stdin\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        if savec == 'y' as i32 || savec == 'Y' as i32 {
            return 1 as i32 != 0;
        }
    } else {
        _IO_putc('\n' as i32, stderr);
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn set_slot_var(mut n: u32) {
    let mut buf: [i8; 20] = [0; 20];
    if snprintf(
        buf.as_mut_ptr(),
        (::core::mem::size_of::<[i8; 20]>() as u64).wrapping_sub(1 as i32 as u64),
        b"%u\0" as *const u8 as *const i8,
        n,
    ) as u64 <= (::core::mem::size_of::<[i8; 20]>() as u64).wrapping_sub(1 as i32 as u64)
    {} else {
        __assert_fail(
            b"snprintf (buf, sizeof buf - 1, \"%u\", n) <= sizeof buf - 1\0" as *const u8
                as *const i8,
            b"xargs.c\0" as *const u8 as *const i8,
            1190 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[i8; 32],
            >(b"void set_slot_var(unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_8760: {
        if snprintf(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[i8; 20]>() as u64).wrapping_sub(1 as i32 as u64),
            b"%u\0" as *const u8 as *const i8,
            n,
        ) as u64
            <= (::core::mem::size_of::<[i8; 20]>() as u64).wrapping_sub(1 as i32 as u64)
        {} else {
            __assert_fail(
                b"snprintf (buf, sizeof buf - 1, \"%u\", n) <= sizeof buf - 1\0"
                    as *const u8 as *const i8,
                b"xargs.c\0" as *const u8 as *const i8,
                1190 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[i8; 32],
                >(b"void set_slot_var(unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !slot_var_name.is_null() {
        if setenv(slot_var_name, buf.as_mut_ptr(), 1 as i32) < 0 as i32 {
            error(
                0 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"failed to set environment variable %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                slot_var_name,
            );
        }
    }
}
unsafe extern "C" fn prep_child_for_exec() {
    if fd_leak_check_is_enabled() {
        complain_about_leaky_fds();
    }
    let mut slot: u32 = add_proc(0 as i32);
    set_slot_var(slot);
    if keep_stdin == 0 || open_tty as i32 != 0 {
        let mut fd: i32 = 0;
        let mut inputfile: *const i8 = if open_tty as i32 != 0 {
            b"/dev/tty\0" as *const u8 as *const i8
        } else {
            b"/dev/null\0" as *const u8 as *const i8
        };
        close(0 as i32);
        fd = open_safer(inputfile, 0 as i32);
        if fd < 0 as i32 {
            if open_tty {
                if ::core::mem::size_of::<C2RustUnnamed_22>() as u64 != 0 {
                    error(
                        1 as i32,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const i8,
                        quotearg_n_style(
                            0 as i32,
                            quoting_style::locale_quoting_style,
                            inputfile,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const i8,
                        quotearg_n_style(
                            0 as i32,
                            quoting_style::locale_quoting_style,
                            inputfile,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                error(
                    0 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    quotearg_n_style(
                        0 as i32,
                        quoting_style::locale_quoting_style,
                        inputfile,
                    ),
                );
            }
        }
        if (0 as i32) < fd {
            if dup2(fd, 0 as i32) != 0 as i32 {
                if ::core::mem::size_of::<C2RustUnnamed_21>() as u64 != 0 {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"failed to redirect standard input of the child process\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"failed to redirect standard input of the child process\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            close(fd);
        }
    }
}
unsafe extern "C" fn xargs_do_exec(
    mut ctl: *mut buildcmd_control,
    mut usercontext: *mut libc::c_void,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut child: pid_t = 0;
    let mut fd: [i32; 2] = [0; 2];
    let mut buf: i32 = 0;
    let mut r: size_t = 0;
    if proc_max != 0 {
        while procs_executing >= proc_max as u64 {
            wait_for_proc(0 as i32 != 0, 1 as u32);
        }
    }
    if !query_before_executing || print_args(1 as i32 != 0) as i32 != 0 {
        if !query_before_executing && print_command as i32 != 0 {
            print_args(0 as i32 != 0);
        }
        wait_for_proc(0 as i32 != 0, 0 as u32);
        if pipe_safer(fd.as_mut_ptr()) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_25>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"could not create pipe before fork\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"could not create pipe before fork\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        rpl_fcntl(fd[1 as i32 as usize], 2 as i32, 1 as i32);
        loop {
            child = fork();
            if !(child < 0 as i32 && *__errno_location() == 11 as i32
                && procs_executing != 0)
            {
                break;
            }
            wait_for_proc(0 as i32 != 0, 1 as u32);
        }
        's_149: {
            match child {
                -1 => {
                    if ::core::mem::size_of::<C2RustUnnamed_23>() as u64 != 0 {
                        error(
                            1 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"cannot fork\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"cannot fork\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                0 => {}
                _ => {
                    close(fd[1 as i32 as usize]);
                    break 's_149;
                }
            }
            close(fd[0 as i32 as usize]);
            ::core::ptr::write_volatile(&mut child_error as *mut i32, 0 as i32);
            prep_child_for_exec();
            if bc_args_exceed_testing_limit(argv) {
                *__errno_location() = 7 as i32;
            } else {
                execvp(*argv.offset(0 as i32 as isize), argv as *const *mut i8);
            }
            if *__errno_location() != 0 {
                write(
                    fd[1 as i32 as usize],
                    __errno_location() as *const libc::c_void,
                    ::core::mem::size_of::<i32>() as u64,
                );
            }
            close(fd[1 as i32 as usize]);
            if 7 as i32 != *__errno_location() {
                error(
                    0 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    *argv.offset(0 as i32 as isize),
                );
            }
            _exit(
                if *__errno_location() == 2 as i32 {
                    XargsStatusValues::XARGS_EXIT_COMMAND_NOT_FOUND as i32
                } else {
                    XargsStatusValues::XARGS_EXIT_COMMAND_CANNOT_BE_RUN as i32
                },
            );
        }
        r = safe_read(
            fd[0 as i32 as usize],
            &mut buf as *mut i32 as *mut libc::c_void,
            ::core::mem::size_of::<i32>() as u64,
        );
        match r {
            18446744073709551615 => {
                close(fd[0 as i32 as usize]);
                error(
                    0 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"errno-buffer safe_read failed in xargs_do_exec (this is probably a bug, please report it)\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            4 => {
                let mut childstatus: i32 = 0;
                close(fd[0 as i32 as usize]);
                waitpid(child, &mut childstatus, 0 as i32);
                if 7 as i32 == buf {
                    return 0 as i32
                } else if 2 as i32 == buf {
                    exit(XargsStatusValues::XARGS_EXIT_COMMAND_NOT_FOUND as i32);
                } else {
                    exit(XargsStatusValues::XARGS_EXIT_COMMAND_CANNOT_BE_RUN as i32);
                }
            }
            0 => {
                add_proc(child);
            }
            _ => {
                if ::core::mem::size_of::<C2RustUnnamed_20>() as u64 != 0 {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"read returned unexpected value %lu; this is probably a bug, please report it\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        r,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"read returned unexpected value %lu; this is probably a bug, please report it\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        r,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        close(fd[0 as i32 as usize]);
    }
    return 1 as i32;
}
unsafe extern "C" fn exec_if_possible() {
    if !(bc_ctl.replace_pat).is_null() || initial_args as i32 != 0
        || bc_state.cmd_argc == bc_ctl.initial_argc || bc_ctl.exit_if_size_exceeded != 0
    {
        return;
    }
    bc_do_exec(&mut bc_ctl, &mut bc_state);
}
unsafe extern "C" fn add_proc(mut pid: pid_t) -> u32 {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64) < pids_alloc && *pids.offset(i as isize) != 0 {
        i = i.wrapping_add(1);
        i;
    }
    if i as u64 == pids_alloc {
        pids = x2nrealloc(
            pids as *mut libc::c_void,
            &mut pids_alloc,
            ::core::mem::size_of::<pid_t>() as u64,
        ) as *mut pid_t;
        j = i;
        while (j as u64) < pids_alloc {
            *pids.offset(j as isize) = 0 as i32;
            j = j.wrapping_add(1);
            j;
        }
    }
    if 0 as i32 == *pids.offset(i as isize) {} else {
        __assert_fail(
            b"0 == pids[i]\0" as *const u8 as *const i8,
            b"xargs.c\0" as *const u8 as *const i8,
            1477 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[i8; 29],
            >(b"unsigned int add_proc(pid_t)\0"))
                .as_ptr(),
        );
    }
    'c_8097: {
        if 0 as i32 == *pids.offset(i as isize) {} else {
            __assert_fail(
                b"0 == pids[i]\0" as *const u8 as *const i8,
                b"xargs.c\0" as *const u8 as *const i8,
                1477 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[i8; 29],
                >(b"unsigned int add_proc(pid_t)\0"))
                    .as_ptr(),
            );
        }
    };
    *pids.offset(i as isize) = pid;
    procs_executing = procs_executing.wrapping_add(1);
    procs_executing;
    procs_executed = 1 as i32 != 0;
    return i;
}
unsafe extern "C" fn wait_for_proc(mut all: bool, mut minreap: u32) {
    let mut reaped: u32 = 0 as i32 as u32;
    while procs_executing != 0 {
        let mut i: u32 = 0;
        let mut status: i32 = 0;
        let mut pid: pid_t = 0;
        let mut wflags: i32 = 0 as i32;
        if !all {
            if reaped >= minreap {
                wflags = 1 as i32;
            }
        }
        ::core::ptr::write_volatile(&mut stop_waiting as *mut sig_atomic_t, 0 as i32);
        loop {
            loop {
                pid = waitpid(-(1 as i32), &mut status, wflags);
                if !(pid == -(1 as i32)) {
                    break;
                }
                if *__errno_location() != 4 as i32 {
                    if ::core::mem::size_of::<C2RustUnnamed_24>() as u64 != 0 {
                        error(
                            1 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"error waiting for child process\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"error waiting for child process\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if stop_waiting != 0 && !all {
                    wflags = 1 as i32;
                }
            }
            if pid != 0 {
                i = 0 as i32 as u32;
                while (i as u64) < pids_alloc && pid != *pids.offset(i as isize) {
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if !(pid != 0 && i as u64 == pids_alloc) {
                break;
            }
        }
        if pid == 0 {
            if wflags & 1 as i32 == 0 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"WARNING: Lost track of %lu child processes\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    procs_executing,
                );
            }
            break;
        } else {
            *pids.offset(i as isize) = 0 as i32;
            procs_executing = procs_executing.wrapping_sub(1);
            procs_executing;
            reaped = reaped.wrapping_add(1);
            reaped;
            if (status & 0xff00 as i32) >> 8 as i32
                == ClientStatusValues::CHILD_EXIT_PLEASE_STOP_IMMEDIATELY as i32
            {
                error(
                    XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_255 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s: exited with status 255; aborting\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    *(bc_state.cmd_argv).offset(0 as i32 as isize),
                );
            }
            if status & 0xff as i32 == 0x7f as i32 {
                error(
                    XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s: stopped by signal %d\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    *(bc_state.cmd_argv).offset(0 as i32 as isize),
                    (status & 0xff00 as i32) >> 8 as i32,
                );
            }
            if ((status & 0x7f as i32) + 1 as i32) as libc::c_schar as i32 >> 1 as i32
                > 0 as i32
            {
                error(
                    XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s: terminated by signal %d\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    *(bc_state.cmd_argv).offset(0 as i32 as isize),
                    status & 0x7f as i32,
                );
            }
            if (status & 0xff00 as i32) >> 8 as i32 != 0 as i32 {
                ::core::ptr::write_volatile(
                    &mut child_error as *mut i32,
                    XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_NONZERO as i32,
                );
            }
        }
    }
}
unsafe extern "C" fn wait_for_proc_all() {
    static mut waiting: bool = 0 as i32 != 0;
    if getpid() == parent {} else {
        __assert_fail(
            b"getpid () == parent\0" as *const u8 as *const i8,
            b"xargs.c\0" as *const u8 as *const i8,
            1605 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[i8; 29],
            >(b"void wait_for_proc_all(void)\0"))
                .as_ptr(),
        );
    }
    'c_9561: {
        if getpid() == parent {} else {
            __assert_fail(
                b"getpid () == parent\0" as *const u8 as *const i8,
                b"xargs.c\0" as *const u8 as *const i8,
                1605 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[i8; 29],
                >(b"void wait_for_proc_all(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if waiting {
        return;
    }
    waiting = 1 as i32 != 0;
    wait_for_proc(1 as i32 != 0, 0 as u32);
    waiting = 0 as i32 != 0;
    if original_exit_value != child_error {
        _exit(child_error);
    }
}
unsafe extern "C" fn increment_proc_max(mut ignore: i32) {
    if proc_max < 2147483647 as i32 {
        ::core::ptr::write_volatile(
            &mut proc_max as *mut sig_atomic_t,
            ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t)
                + 1,
        );
        ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t);
    }
    ::core::ptr::write_volatile(&mut stop_waiting as *mut sig_atomic_t, 1 as i32);
}
unsafe extern "C" fn decrement_proc_max(mut ignore: i32) {
    if proc_max > 1 as i32 {
        ::core::ptr::write_volatile(
            &mut proc_max as *mut sig_atomic_t,
            ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t)
                - 1,
        );
        ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t);
    }
}
unsafe extern "C" fn parse_num(
    mut str: *mut i8,
    mut option: i32,
    mut min: i64,
    mut max: i64,
    mut fatal: i32,
) -> i64 {
    let mut eptr: *mut i8 = 0 as *mut i8;
    let mut val: i64 = 0;
    val = strtol(str, &mut eptr, 10 as i32);
    if eptr == str || *eptr as i32 != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: invalid number \"%s\" for -%c option\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            program_name,
            str,
            option,
        );
        usage(1 as i32);
    } else if val < min {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: value %s for -%c option should be >= %ld\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            program_name,
            str,
            option,
            min,
        );
        if fatal != 0 {
            usage(1 as i32);
        }
        val = min;
    } else if max >= 0 as i32 as i64 && val > max {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: value %s for -%c option should be <= %ld\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            program_name,
            str,
            option,
            max,
        );
        if fatal != 0 {
            usage(1 as i32);
        }
        val = max;
    }
    return val;
}
unsafe extern "C" fn usage(mut status: i32) -> ! {
    if status != 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Try '%s --help' for more information.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [OPTION]... COMMAND [INITIAL-ARGS]...\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"Run COMMAND with arguments INITIAL-ARGS and more arguments read from input.\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"Mandatory and optional arguments to long options are also\nmandatory or optional for the corresponding short option.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -0, --null                   items are separated by a null, not whitespace;\n                                 disables quote and backslash processing and\n                                 logical EOF processing\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -a, --arg-file=FILE          read arguments from FILE, not standard input\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -d, --delimiter=CHARACTER    items in input stream are separated by CHARACTER,\n                                 not by whitespace; disables quote and backslash\n                                 processing and logical EOF processing\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -E END                       set logical EOF string; if END occurs as a line\n                                 of input, the rest of the input is ignored\n                                 (ignored if -0 or -d was specified)\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -e, --eof[=END]              equivalent to -E END if END is specified;\n                                 otherwise, there is no end-of-file string\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -I R                         same as --replace=R\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -i, --replace[=R]            replace R in INITIAL-ARGS with names read\n                                 from standard input, split at newlines;\n                                 if R is unspecified, assume {}\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -L, --max-lines=MAX-LINES    use at most MAX-LINES non-blank input lines per\n                                 command line\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -l[MAX-LINES]                similar to -L but defaults to at most one non-\n                                 blank input line if MAX-LINES is not specified\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -n, --max-args=MAX-ARGS      use at most MAX-ARGS arguments per command line\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -o, --open-tty               Reopen stdin as /dev/tty in the child process\n                                 before executing the command; useful to run an\n                                 interactive application.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -P, --max-procs=MAX-PROCS    run at most MAX-PROCS processes at a time\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -p, --interactive            prompt before running commands\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"      --process-slot-var=VAR   set environment variable VAR in child processes\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -r, --no-run-if-empty        if there are no arguments, then do not run COMMAND;\n                                 if this option is not given, COMMAND will be\n                                 run at least once\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -s, --max-chars=MAX-CHARS    limit length of command line to MAX-CHARS\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"      --show-limits            show limits on command-line length\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -t, --verbose                print commands before executing them\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"  -x, --exit                   exit if the size (see -s) is exceeded\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"      --help                   display this help and exit\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"      --version                output version information and exit\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
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