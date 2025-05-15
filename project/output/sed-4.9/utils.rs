use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __uflow(_: *mut _IO_FILE) -> i32;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn rename(__old: *const i8, __new: *const i8) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush_unlocked(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn __errno_location() -> *mut i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn rpl_free(_: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn mkostemp(__template: *mut i8, __flags: i32) -> i32;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn sysconf(__name: i32) -> i64;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn unlink(__name: *const i8) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    static mut program_name: *const i8;
    fn __fwriting(__fp: *mut FILE) -> i32;
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
pub type size_t = u64;
pub type __mode_t = u32;
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
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = i64;
pub type mode_t = __mode_t;
pub type C2RustUnnamed = u32;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub type idx_t = ptrdiff_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum exit_codes {
    EXIT_PANIC = 4,
    EXIT_BAD_INPUT = 2,
    EXIT_BAD_USAGE = 1,
}
impl exit_codes {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            exit_codes::EXIT_PANIC => 4,
            exit_codes::EXIT_BAD_INPUT => 2,
            exit_codes::EXIT_BAD_USAGE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> exit_codes {
        match value {
            4 => exit_codes::EXIT_PANIC,
            2 => exit_codes::EXIT_BAD_INPUT,
            1 => exit_codes::EXIT_BAD_USAGE,
            _ => panic!("Invalid value for exit_codes: {}", value),
        }
    }
}
impl AddAssign<u32> for exit_codes {
    fn add_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for exit_codes {
    fn sub_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for exit_codes {
    fn mul_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for exit_codes {
    fn div_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for exit_codes {
    fn rem_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for exit_codes {
    type Output = exit_codes;
    fn add(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for exit_codes {
    type Output = exit_codes;
    fn sub(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for exit_codes {
    type Output = exit_codes;
    fn mul(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for exit_codes {
    type Output = exit_codes;
    fn div(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for exit_codes {
    type Output = exit_codes;
    fn rem(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_file {
    pub fp: *mut FILE,
    pub name: *mut i8,
    pub link: *mut open_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub allocated: size_t,
    pub length: size_t,
    pub b: *mut i8,
}
#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
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
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x20 as i32 != 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn __eloop_threshold() -> u32 {
    static mut sysconf_symloop_max: i64 = 0;
    if sysconf_symloop_max == 0 as i32 as i64 {
        sysconf_symloop_max = sysconf(_SC_SYMLOOP_MAX as i32);
    }
    let symloop_max: u32 = (if sysconf_symloop_max <= 0 as i32 as i64 {
        8 as i32 as i64
    } else {
        sysconf_symloop_max
    }) as u32;
    return if symloop_max > 40 as i32 as u32 { symloop_max } else { 40 as i32 as u32 };
}
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
}
static mut open_files: *mut open_file = 0 as *const open_file as *mut open_file;
#[no_mangle]
pub unsafe extern "C" fn panic(mut str: *const i8, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program_name);
    ap = args.clone();
    vfprintf(stderr, str, ap.as_va_list());
    putc_unlocked('\n' as i32, stderr);
    exit(exit_codes::EXIT_PANIC as i32);
}
unsafe extern "C" fn utils_fp_name(mut fp: *mut FILE) -> *const i8 {
    let mut p: *mut open_file = 0 as *mut open_file;
    p = open_files;
    while !p.is_null() {
        if (*p).fp == fp {
            return (*p).name;
        }
        p = (*p).link;
    }
    if fp == stdin {
        return b"stdin\0" as *const u8 as *const i8
    } else if fp == stdout {
        return b"stdout\0" as *const u8 as *const i8
    } else if fp == stderr {
        return b"stderr\0" as *const u8 as *const i8
    }
    return b"<unknown>\0" as *const u8 as *const i8;
}
unsafe extern "C" fn register_open_file(mut fp: *mut FILE, mut name: *const i8) {
    let mut p: *mut open_file = 0 as *mut open_file;
    p = xmalloc(::core::mem::size_of::<open_file>() as u64) as *mut open_file;
    (*p).link = open_files;
    open_files = p;
    (*p).name = xstrdup(name);
    (*p).fp = fp;
}
#[no_mangle]
pub unsafe extern "C" fn ck_fopen(
    mut name: *const i8,
    mut mode: *const i8,
    mut fail: i32,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, mode);
    if fp.is_null() {
        if fail != 0 {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't open file %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                name,
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut FILE;
    }
    register_open_file(fp, name);
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn ck_fdopen(
    mut fd: i32,
    mut name: *const i8,
    mut mode: *const i8,
    mut fail: i32,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fdopen(fd, mode);
    if fp.is_null() {
        if fail != 0 {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't attach to %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                name,
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut FILE;
    }
    register_open_file(fp, name);
    return fp;
}
static mut G_file_to_unlink: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn remove_cleanup_file() {
    if !G_file_to_unlink.is_null() {
        unlink(G_file_to_unlink);
    }
}
unsafe extern "C" fn register_cleanup_file(mut file: *const i8) {
    G_file_to_unlink = file;
}
#[no_mangle]
pub unsafe extern "C" fn cancel_cleanup() {
    G_file_to_unlink = 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn ck_mkstemp(
    mut p_filename: *mut *mut i8,
    mut tmpdir: *const i8,
    mut base: *const i8,
    mut mode: *const i8,
) -> *mut FILE {
    let mut template: *mut i8 = xmalloc(
        (strlen(tmpdir)).wrapping_add(strlen(base)).wrapping_add(8 as i32 as u64),
    ) as *mut i8;
    sprintf(template, b"%s/%sXXXXXX\0" as *const u8 as *const i8, tmpdir, base);
    let mut save_umask: mode_t = umask(0o77 as i32 as __mode_t);
    let mut fd: i32 = mkostemp(template, 0 as i32);
    let mut err: i32 = *__errno_location();
    umask(save_umask);
    let mut fp: *mut FILE = 0 as *mut FILE;
    if 0 as i32 <= fd {
        *p_filename = template;
        register_cleanup_file(template);
        fp = fdopen(fd, mode);
        err = *__errno_location();
    }
    if fp.is_null() {
        panic(
            dcgettext(
                0 as *const i8,
                b"couldn't open temporary file %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            template,
            strerror(err),
        );
    }
    register_open_file(fp, template);
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn ck_fwrite(
    mut ptr: *const libc::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut FILE,
) {
    clearerr_unlocked(stream);
    if size != 0
        && (if 0 != 0 && 0 != 0 && size.wrapping_mul(nmemb) <= 8 as i32 as u64
            && size != 0 as i32 as u64
        {
            ({
                let mut __ptr: *const i8 = ptr as *const i8;
                let mut __stream: *mut FILE = stream;
                let mut __cnt: size_t = 0;
                __cnt = size.wrapping_mul(nmemb);
                while __cnt > 0 as i32 as u64 {
                    if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                        as i32 as i64 != 0
                    {
                        let fresh1 = __ptr;
                        __ptr = __ptr.offset(1);
                        __overflow(__stream, *fresh1 as u8 as i32)
                    } else {
                        let fresh2 = __ptr;
                        __ptr = __ptr.offset(1);
                        let fresh3 = (*__stream)._IO_write_ptr;
                        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                            .offset(1);
                        *fresh3 = *fresh2;
                        *fresh3 as u8 as i32
                    }) == -(1 as i32)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                size.wrapping_mul(nmemb).wrapping_sub(__cnt).wrapping_div(size)
            })
        } else {
            (if 0 != 0 && size == 0 as i32 as u64 || 0 != 0 && nmemb == 0 as i32 as u64 {
                0 as i32 as size_t
            } else {
                fwrite_unlocked(ptr, size, nmemb, stream)
            })
        }) != nmemb
    {
        panic(
            dcngettext(
                0 as *const i8,
                b"couldn't write %llu item to %s: %s\0" as *const u8 as *const i8,
                b"couldn't write %llu items to %s: %s\0" as *const u8 as *const i8,
                nmemb,
                5 as i32,
            ),
            nmemb as libc::c_ulonglong,
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ck_fread(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut FILE,
) -> size_t {
    clearerr_unlocked(stream);
    if size != 0
        && {
            nmemb = (if 0 != 0 && 0 != 0 && size.wrapping_mul(nmemb) <= 8 as i32 as u64
                && size != 0 as i32 as u64
            {
                ({
                    let mut __ptr: *mut i8 = ptr as *mut i8;
                    let mut __stream: *mut FILE = stream;
                    let mut __cnt: size_t = 0;
                    __cnt = size.wrapping_mul(nmemb);
                    while __cnt > 0 as i32 as u64 {
                        let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                            >= (*__stream)._IO_read_end) as i32 as i64 != 0
                        {
                            __uflow(__stream)
                        } else {
                            let fresh4 = (*__stream)._IO_read_ptr;
                            (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                .offset(1);
                            *(fresh4 as *mut u8) as i32
                        });
                        if __c == -(1 as i32) {
                            break;
                        }
                        let fresh5 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh5 = __c as i8;
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    size.wrapping_mul(nmemb).wrapping_sub(__cnt).wrapping_div(size)
                })
            } else {
                (if 0 != 0 && size == 0 as i32 as u64
                    || 0 != 0 && nmemb == 0 as i32 as u64
                {
                    0 as i32 as size_t
                } else {
                    fread_unlocked(ptr, size, nmemb, stream)
                })
            });
            nmemb <= 0 as i32 as u64
        } && ferror_unlocked(stream) != 0
    {
        panic(
            dcgettext(
                0 as *const i8,
                b"read error on %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
    return nmemb;
}
#[no_mangle]
pub unsafe extern "C" fn ck_getdelim(
    mut text: *mut *mut i8,
    mut buflen: *mut size_t,
    mut delim: i8,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: ssize_t = 0;
    let mut error: bool = false;
    error = ferror_unlocked(stream) != 0;
    if !error {
        result = getdelim(text, buflen, delim as i32, stream);
        error = ferror_unlocked(stream) != 0;
    }
    if error {
        panic(
            dcgettext(
                0 as *const i8,
                b"read error on %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
    return result as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ck_fflush(mut stream: *mut FILE) {
    if !(__fwriting(stream) != 0 as i32) {
        return;
    }
    clearerr_unlocked(stream);
    if fflush_unlocked(stream) == -(1 as i32) && *__errno_location() != 9 as i32 {
        panic(
            b"couldn't flush %s: %s\0" as *const u8 as *const i8,
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ck_fclose(mut stream: *mut FILE) {
    let mut prev: *mut *mut open_file = &mut open_files;
    let mut cur: *mut open_file = 0 as *mut open_file;
    loop {
        cur = *prev;
        if cur.is_null() {
            break;
        }
        if stream.is_null() || stream == (*cur).fp {
            let mut fp: *mut FILE = (*cur).fp;
            *prev = (*cur).link;
            do_ck_fclose(fp, (*cur).name);
            rpl_free((*cur).name as *mut libc::c_void);
            rpl_free(cur as *mut libc::c_void);
        } else {
            prev = &mut (*cur).link;
        }
    }
    if stream.is_null() {
        do_ck_fclose(stdout, b"stdout\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn do_ck_fclose(mut fp: *mut FILE, mut name: *const i8) {
    ck_fflush(fp);
    clearerr_unlocked(fp);
    if fclose(fp) == -(1 as i32) {
        panic(
            b"couldn't close %s: %s\0" as *const u8 as *const i8,
            name,
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn follow_symlink(mut fname: *const i8) -> *const i8 {
    let mut fn_0: *const i8 = fname;
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut buf_size: idx_t = 0;
    let mut buf_used: idx_t = 0 as i32 as idx_t;
    let mut num_links: idx_t = 0 as i32 as idx_t;
    loop {
        let mut linklen: ssize_t = 0;
        let mut newlen: idx_t = 0;
        let mut c: *const i8 = 0 as *const i8;
        loop {
            linklen = (if buf_used < buf_size {
                readlink(
                    fn_0,
                    buf.offset(buf_used as isize),
                    (buf_size - buf_used) as size_t,
                )
            } else {
                0 as i32 as i64
            });
            if !(linklen == buf_size) {
                break;
            }
            buf = xpalloc(
                buf as *mut libc::c_void,
                &mut buf_size,
                1 as i32 as idx_t,
                if (9223372036854775807 as i64) < 9223372036854775807 as i64 {
                    9223372036854775807 as i64
                } else {
                    9223372036854775807 as i64
                },
                1 as i32 as idx_t,
            ) as *mut i8;
            if num_links != 0 {
                fn_0 = buf;
            }
        }
        if linklen < 0 as i32 as i64 {
            if *__errno_location() == 22 as i32 {
                break;
            }
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't readlink %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                fn_0,
                strerror(*__errno_location()),
            );
        }
        if __eloop_threshold() as i64 <= num_links {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't follow symlink %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
                strerror(40 as i32),
            );
        }
        if (linklen == 0 as i32 as i64
            || *buf.offset(buf_used as isize) as i32 != '/' as i32)
            && {
                c = strrchr(fn_0, '/' as i32);
                !c.is_null()
            }
        {
            let mut dirlen: idx_t = c.offset_from(fn_0) as i64 + 1 as i32 as i64;
            newlen = dirlen + linklen;
            if buf_size <= newlen {
                buf = xpalloc(
                    buf as *mut libc::c_void,
                    &mut buf_size,
                    newlen + 1 as i32 as i64 - buf_size,
                    if (9223372036854775807 as i64) < 9223372036854775807 as i64 {
                        9223372036854775807 as i64
                    } else {
                        9223372036854775807 as i64
                    },
                    1 as i32 as idx_t,
                ) as *mut i8;
                if num_links != 0 {
                    fn_0 = buf;
                }
            }
            memmove(
                buf.offset(dirlen as isize) as *mut libc::c_void,
                buf.offset(buf_used as isize) as *const libc::c_void,
                linklen as u64,
            );
            if fn_0 != buf {
                memcpy(
                    buf as *mut libc::c_void,
                    fn_0 as *const libc::c_void,
                    dirlen as u64,
                );
            }
        } else {
            memmove(
                buf as *mut libc::c_void,
                buf.offset(buf_used as isize) as *const libc::c_void,
                linklen as u64,
            );
            newlen = linklen;
        }
        *buf.offset(newlen as isize) = '\0' as i32 as i8;
        buf_used = newlen + 1 as i32 as i64;
        fn_0 = buf;
        num_links += 1;
        num_links;
    }
    return fn_0;
}
#[no_mangle]
pub unsafe extern "C" fn ck_rename(mut from: *const i8, mut to: *const i8) {
    let mut rd: i32 = rename(from, to);
    if rd != -(1 as i32) {
        return;
    }
    panic(
        dcgettext(
            0 as *const i8,
            b"cannot rename %s: %s\0" as *const u8 as *const i8,
            5 as i32,
        ),
        from,
        strerror(*__errno_location()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn init_buffer() -> *mut buffer {
    let mut b: *mut buffer = (if ::core::mem::size_of::<buffer>() as u64
        == 1 as i32 as u64
    {
        xzalloc(1 as i32 as size_t)
    } else {
        xcalloc(1 as i32 as size_t, ::core::mem::size_of::<buffer>() as u64)
    }) as *mut buffer;
    (*b).b = (if ::core::mem::size_of::<i8>() as u64 == 1 as i32 as u64 {
        xzalloc(50 as i32 as size_t)
    } else {
        xcalloc(50 as i32 as size_t, ::core::mem::size_of::<i8>() as u64)
    }) as *mut i8;
    (*b).allocated = 50 as i32 as size_t;
    (*b).length = 0 as i32 as size_t;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn get_buffer(mut b: *const buffer) -> *mut i8 {
    return (*b).b;
}
#[no_mangle]
pub unsafe extern "C" fn size_buffer(mut b: *const buffer) -> size_t {
    return (*b).length;
}
unsafe extern "C" fn resize_buffer(mut b: *mut buffer, mut newlen: size_t) {
    let mut try_0: *mut i8 = 0 as *mut i8;
    let mut alen: size_t = (*b).allocated;
    if newlen <= alen {
        return;
    }
    alen = (alen as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
    if newlen < alen {
        try_0 = realloc((*b).b as *mut libc::c_void, alen) as *mut i8;
    }
    if try_0.is_null() {
        alen = newlen;
        try_0 = xnrealloc(
            (*b).b as *mut libc::c_void,
            alen,
            ::core::mem::size_of::<i8>() as u64,
        ) as *mut i8;
    }
    (*b).allocated = alen;
    (*b).b = try_0;
}
#[no_mangle]
pub unsafe extern "C" fn add_buffer(
    mut b: *mut buffer,
    mut p: *const i8,
    mut n: size_t,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    if ((*b).allocated).wrapping_sub((*b).length) < n {
        resize_buffer(b, ((*b).length).wrapping_add(n));
    }
    result = memcpy(
        ((*b).b).offset((*b).length as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        n,
    ) as *mut i8;
    (*b).length = ((*b).length as u64).wrapping_add(n) as size_t as size_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn add1_buffer(mut b: *mut buffer, mut c: i32) -> *mut i8 {
    if c != -(1 as i32) {
        let mut result: *mut i8 = 0 as *mut i8;
        if ((*b).allocated).wrapping_sub((*b).length) < 1 as i32 as u64 {
            resize_buffer(b, ((*b).length).wrapping_add(1 as i32 as u64));
        }
        let fresh6 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        result = ((*b).b).offset(fresh6 as isize);
        *result = c as i8;
        return result;
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn free_buffer(mut b: *mut buffer) {
    if !b.is_null() {
        rpl_free((*b).b as *mut libc::c_void);
    }
    rpl_free(b as *mut libc::c_void);
}