use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn floorl(_: f128::f128) -> f128::f128;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sysconf(__name: i32) -> i64;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
}
pub type __uint64_t = u64;
pub type __intmax_t = i64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type uint64_t = __uint64_t;
pub type uintptr_t = u64;
pub type intmax_t = __intmax_t;
pub type size_t = u64;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    NFL = 422,
    WDSZ = 8,
    VERS = 2,
    ALGN = 1073741824,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::NFL => 422,
            C2RustUnnamed_1::WDSZ => 8,
            C2RustUnnamed_1::VERS => 2,
            C2RustUnnamed_1::ALGN => 1073741824,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            422 => C2RustUnnamed_1::NFL,
            8 => C2RustUnnamed_1::WDSZ,
            2 => C2RustUnnamed_1::VERS,
            1073741824 => C2RustUnnamed_1::ALGN,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub init: i32,
    pub vrb: i32,
    pub file: *const i8,
    pub hdr: *mut pma_hdr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pma_hdr_t {
    pub mapaddr: *mut libc::c_void,
    pub bf_vers: uint64_t,
    pub nallocs: uint64_t,
    pub nfrees: uint64_t,
    pub res_0: uint64_t,
    pub root: *mut libc::c_void,
    pub afirst: *mut ao_t,
    pub abound: *mut ao_t,
    pub free: [ao_t; 422],
}
pub type ao_t = ao;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ao {
    pub anext: *mut ao,
    pub fprev: *mut ao,
    pub fnext: *mut ao,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    IU = 0,
    PIU = 1,
    GROWN = 2,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::IU => 0,
            C2RustUnnamed_2::PIU => 1,
            C2RustUnnamed_2::GROWN => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            0 => C2RustUnnamed_2::IU,
            1 => C2RustUnnamed_2::PIU,
            2 => C2RustUnnamed_2::GROWN,
            _ => panic!("Invalid value for C2RustUnnamed_2: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_2 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_2 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_2 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn add(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn sub(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn mul(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn div(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn rem(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type ul_t = u64;
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[no_mangle]
pub static mut pma_version: [i8; 37] = unsafe {
    *::core::mem::transmute::<
        &[u8; 37],
        &[i8; 37],
    >(b"2022.10Oct.30.1667172241 (Avon 8-g1)\0")
};
#[no_mangle]
pub static mut pma_errno: i32 = 0;
static mut lomask: uintptr_t = 0x7 as i32 as uintptr_t;
static mut himask: uintptr_t = !(0x7 as i32 as uintptr_t);
unsafe extern "C" fn slobh(mut p: *mut ao_t, mut iu: i32, mut piu: i32, mut grown: i32) {
    let mut h: uintptr_t = 0;
    h = (*p).anext as uintptr_t;
    h &= himask;
    h |= (iu | piu << 1 as i32 | grown << 2 as i32) as uintptr_t;
    (*p).anext = h as *mut ao_t;
}
unsafe extern "C" fn globh(
    mut p: *const ao_t,
    mut iu: *mut i32,
    mut piu: *mut i32,
    mut grown: *mut i32,
) {
    let mut h: uintptr_t = 0;
    h = (*p).anext as uintptr_t;
    *iu = (h & 0x1 as i32 as u64 != 0) as i32;
    *piu = (h & 0x2 as i32 as u64 != 0) as i32;
    *grown = (h & 0x4 as i32 as u64 != 0) as i32;
}
static mut state: C2RustUnnamed_0 = C2RustUnnamed_0 {
    init: 0,
    vrb: 0,
    file: 0 as *const i8,
    hdr: 0 as *const pma_hdr_t as *mut pma_hdr_t,
};
unsafe extern "C" fn getbit(mut p: *mut ao_t, mut bit: i32) -> i32 {
    let mut iu: i32 = 0;
    let mut piu: i32 = 0;
    let mut grown: i32 = 0;
    globh(p, &mut iu, &mut piu, &mut grown);
    match bit {
        0 => return iu,
        1 => return piu,
        2 => return grown,
        _ => {
            if (0 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:148: ERROR: bad bit: %d\n\0" as *const u8 as *const i8,
                    bit,
                );
            }
            pma_errno = 149 as i32;
            return -(2147483647 as i32) - 1 as i32;
        }
    };
}
unsafe extern "C" fn pao(mut p: *mut ao_t) {
    let mut iu: i32 = 0;
    let mut piu: i32 = 0;
    let mut grown: i32 = 0;
    let mut footer: *mut *mut ao_t = ((p as *mut i8)
        .offset(
            (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
                .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t)
                as isize,
        ) as *mut *mut ao_t)
        .offset(-(1 as i32 as isize));
    globh(p, &mut iu, &mut piu, &mut grown);
    fprintf(
        stderr,
        b"    AO at %p:  size %lu B / %lu w\n      hdr %p (H 0%lo L 0%lo) iu %d piu %d grown %d\n      fp  %p%s\n      fn  %p%s\n      ft  %p%s\n\0"
            as *const u8 as *const i8,
        p as *mut libc::c_void,
        (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t),
        (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_div(C2RustUnnamed_1::WDSZ as i32 as u64),
        (*p).anext as *mut libc::c_void,
        ((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t,
        ((*p).anext as uintptr_t & lomask) as *mut ao_t as uintptr_t,
        iu,
        piu,
        grown,
        (*p).fprev as *mut libc::c_void,
        if iu != 0 {
            b" (ignore)\0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
        (*p).fnext as *mut libc::c_void,
        if iu != 0 {
            b" (ignore)\0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
        *footer as *mut libc::c_void,
        if iu != 0 {
            b" (ignore)\0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
}
static mut UB: [size_t; 422] = [0; 422];
unsafe extern "C" fn integrity_check(mut line: i32) -> i32 {
    let mut h: *mut pma_hdr_t = state.hdr;
    let mut nadd: i32 = 0 as i32;
    let mut naddfree: i32 = 0 as i32;
    let mut tiu: i32 = 0 as i32;
    let mut tpiu: i32 = 0 as i32;
    let mut tf: i32 = 0 as i32;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:197: FYI: integrity_check called from line %d\n\0" as *const u8
                as *const i8,
            line,
        );
    }
    if (1 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:199: Warning: integrity check relies on assertions, which are disabled (call at line %d)\n\0"
                as *const u8 as *const i8,
            line,
        );
    }
    let mut next: *mut ao_t = 0 as *mut ao_t;
    let mut a: *mut ao_t = (*h).afirst;
    while a < (*h).abound {
        next = ((*a).anext as uintptr_t & himask) as *mut ao_t;
        nadd += 1;
        if (1000000 as i32) < nadd {
            if (1 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:209: Warning: integrity check discontinued; anext list too long (call at line %d)\n\0"
                        as *const u8 as *const i8,
                    line,
                );
            }
            pma_errno = 210 as i32;
            return 0 as i32;
        }
        if 0 as i32 == getbit(a, C2RustUnnamed_2::IU as i32) {
            naddfree += 1;
            naddfree;
        }
        tiu += getbit(a, C2RustUnnamed_2::IU as i32);
        tpiu += getbit(a, C2RustUnnamed_2::PIU as i32);
        a = next;
    }
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:220: FYI: anext list length: %d  tiu %d  tpiu %d  nallocs %lu  nfrees %lu\n\0"
                as *const u8 as *const i8,
            nadd,
            tiu,
            tpiu,
            (*h).nallocs,
            (*h).nfrees,
        );
    }
    let mut i: i32 = 0 as i32;
    while i < C2RustUnnamed_1::NFL as i32 {
        let mut p: *mut ao_t = 0 as *mut ao_t;
        let mut f: *mut ao_t = &mut *((*h).free).as_mut_ptr().offset(i as isize)
            as *mut ao_t;
        if !((*f).fprev == f) {
            let mut nfwd: i32 = 0 as i32;
            let mut nrev: i32 = 0 as i32;
            p = (*f).fnext;
            while p != f {
                nfwd += 1;
                nfwd;
                p = (*p).fnext;
            }
            p = (*f).fprev;
            while p != f {
                nrev += 1;
                nrev;
                p = (*p).fprev;
            }
            tf += nfwd;
            p = (*f).fnext;
            while p != f {
                (0 as i32) < i;
                p = (*p).fnext;
            }
        }
        i += 1;
        i;
    }
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:245: FYI: total free aos: %d  naddfree %d  integrity check line %d\n\0"
                as *const u8 as *const i8,
            tf,
            naddfree,
            line,
        );
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pma_check_and_dump() {
    let mut h: *mut pma_hdr_t = state.hdr;
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:254: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 254 as i32;
        return;
    }
    if 2 as i32 == state.init {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:255: ERROR: check_and_dump not meaningful in fallback mode\n\0"
                    as *const u8 as *const i8,
            );
        }
        pma_errno = 255 as i32;
        return;
    }
    if integrity_check(256 as i32) != 0 {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:256: ERROR: integrity check failed\n\0" as *const u8 as *const i8,
            );
        }
    }
    fprintf(
        stderr,
        b"pma.c:257: check data structures and dump\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr,
        b"header version:    %s\n\0" as *const u8 as *const i8,
        b"2022.10Oct.30.1667172241 (Avon 8-g1)\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr,
        b"software version:  %s\n\0" as *const u8 as *const i8,
        pma_version.as_ptr(),
    );
    fprintf(
        stderr,
        b"sizeof state:  %lu\n\0" as *const u8 as *const i8,
        ::core::mem::size_of::<C2RustUnnamed_0>() as u64,
    );
    fprintf(
        stderr,
        b"sizeof header: %lu\n\0" as *const u8 as *const i8,
        ::core::mem::size_of::<pma_hdr_t>() as u64,
    );
    fprintf(stderr, b"state:\n\0" as *const u8 as *const i8);
    fprintf(stderr, b"  init: %d\n\0" as *const u8 as *const i8, state.init);
    fprintf(stderr, b"  vrb:  %d\n\0" as *const u8 as *const i8, state.vrb);
    fprintf(
        stderr,
        b"  file: %p \"%s\"\n\0" as *const u8 as *const i8,
        state.file as *const libc::c_void,
        state.file,
    );
    fprintf(stderr, b"  hdr:  %p\n\0" as *const u8 as *const i8, h as *mut libc::c_void);
    if !h.is_null() {
        fprintf(stderr, b"header:\n\0" as *const u8 as *const i8);
        fprintf(stderr, b"  mapaddr: %p\n\0" as *const u8 as *const i8, (*h).mapaddr);
        fprintf(stderr, b"  bf_vers: %lu\n\0" as *const u8 as *const i8, (*h).bf_vers);
        fprintf(stderr, b"  root:   %p\n\0" as *const u8 as *const i8, (*h).root);
        fprintf(
            stderr,
            b"  afirst: %p\n\0" as *const u8 as *const i8,
            (*h).afirst as *mut libc::c_void,
        );
        fprintf(
            stderr,
            b"  abound: %p\n\0" as *const u8 as *const i8,
            (*h).abound as *mut libc::c_void,
        );
        fprintf(
            stderr,
            b"  all allocated objects in addr order:\n\0" as *const u8 as *const i8,
        );
        let mut a: *mut ao_t = (*h).afirst;
        while a < (*h).abound {
            pao(a);
            a = ((*a).anext as uintptr_t & himask) as *mut ao_t;
        }
        let mut i: i32 = 0 as i32;
        while i < C2RustUnnamed_1::NFL as i32 {
            let mut f: *mut ao_t = &mut *((*h).free).as_mut_ptr().offset(i as isize)
                as *mut ao_t;
            if !((*f).fprev == f) {
                fprintf(
                    stderr,
                    b"  free list of size class %d UB %lu (prev %lu) list head %p:\n\0"
                        as *const u8 as *const i8,
                    i,
                    UB[i as usize],
                    if i > 0 as i32 {
                        UB[(i - 1 as i32) as usize]
                    } else {
                        0 as i32 as u64
                    },
                    f as *mut libc::c_void,
                );
                let mut p: *mut ao_t = (*f).fnext;
                while p != f {
                    pao(p);
                    p = (*p).fnext;
                }
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn sc(mut nbytes: size_t) -> i32 {
    static mut init: i32 = 0;
    let maxwords: size_t = (0x1 as i32 as size_t) << 61 as i32;
    let mut nwords: size_t = 0;
    let mut c: i32 = 0;
    if init == 0 {
        if (2 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:299: FYI: initializing UB[]\n\0" as *const u8 as *const i8,
            );
        }
        UB[0 as i32 as usize] = 3 as i32 as size_t;
        c = 1 as i32;
        loop {
            let mut hi: f128::f128 = floorl(
                f128::f128::new(
                    (1 as i32 as u64).wrapping_add(UB[(c - 1 as i32) as usize]),
                ) * f128::f128::new(1.1),
            );
            if hi >= f128::f128::new(maxwords) {
                UB[c as usize] = maxwords;
                break;
            } else {
                UB[c as usize] = hi.to_u64().unwrap();
                c += 1;
                c;
            }
        }
        init = 1 as i32;
    }
    nwords = nbytes
        .wrapping_div(C2RustUnnamed_1::WDSZ as i32 as u64)
        .wrapping_add(
            (nbytes.wrapping_rem(C2RustUnnamed_1::WDSZ as i32 as u64) != 0) as i32 as u64,
        );
    c = 0 as i32;
    while C2RustUnnamed_1::NFL as i32 > c {
        if nwords <= UB[c as usize] {
            break;
        }
        c += 1;
        c;
    }
    return c;
}
unsafe extern "C" fn fli(mut p: *mut ao_t) {
    let mut h: *mut ao_t = 0 as *mut ao_t;
    let mut fl: i32 = 0;
    fl = sc(
        (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_sub(C2RustUnnamed_1::WDSZ as i32 as u64),
    );
    h = &mut *((*state.hdr).free).as_mut_ptr().offset(fl as isize) as *mut ao_t;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:327: FYI: fli(%p) h == %p h->fn %p h->fp %p\n\0" as *const u8
                as *const i8,
            p as *mut libc::c_void,
            h as *mut libc::c_void,
            (*h).fnext as *mut libc::c_void,
            (*h).fprev as *mut libc::c_void,
        );
    }
    (*p).fprev = h;
    (*p).fnext = (*h).fnext;
    (*(*h).fnext).fprev = p;
    (*h).fnext = p;
}
unsafe extern "C" fn flr(mut p: *mut ao_t) {
    (*(*p).fnext).fprev = (*p).fprev;
    (*(*p).fprev).fnext = (*p).fnext;
    (*p).fnext = 0 as *mut ao;
    (*p).fprev = (*p).fnext;
}
unsafe extern "C" fn addrgap(mut n: off_t) -> *mut libc::c_void {
    let mut A: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut Amax: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut L: size_t = 0;
    let mut U: size_t = 0;
    let mut Max: size_t = 0 as i32 as size_t;
    let mut N: size_t = n as size_t;
    let mut r: *mut i8 = 0 as *mut i8;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:352: FYI: addrgap(%jd)\n\0" as *const u8 as *const i8,
            n,
        );
    }
    if N < (::core::mem::size_of::<pma_hdr_t>() as u64).wrapping_add(40960 as i32 as u64)
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:353: ERROR: file size %zu too small\n\0" as *const u8
                    as *const i8,
                N,
            );
        }
        pma_errno = 353 as i32;
        return 0 as *mut libc::c_void;
    }
    L = 1 as i32 as size_t;
    U = 18446744073709551615 as u64;
    while L <= U {
        let mut M: size_t = L
            .wrapping_add(U.wrapping_sub(L).wrapping_div(2 as i32 as u64));
        A = mmap(
            0 as *mut libc::c_void,
            M,
            0 as i32,
            0x2 as i32 | 0x20 as i32 | 0x4000 as i32,
            -(1 as i32),
            0 as i32 as __off_t,
        );
        if -(1 as i32) as *mut libc::c_void != A {
            Max = M;
            Amax = A;
            if 0 as i32 != munmap(A, M) {
                if (0 as i32) < state.vrb {
                    fprintf(
                        stderr,
                        b"pma.c:363: ERROR: munmap() errno => '%s'\n\0" as *const u8
                            as *const i8,
                        strerror(*__errno_location()),
                    );
                }
                pma_errno = 363 as i32;
                return 0 as *mut libc::c_void;
            }
            if 18446744073709551615 as u64 == M {
                break;
            }
            L = M.wrapping_add(1 as i32 as u64);
        } else {
            U = M.wrapping_sub(1 as i32 as u64);
        }
    }
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:366: FYI: max gap: %zu bytes at %p\n\0" as *const u8 as *const i8,
            Max,
            Amax,
        );
    }
    if Max
        < N
            .wrapping_add(
                (C2RustUnnamed_1::ALGN as i32 as size_t).wrapping_mul(2 as i32 as u64),
            )
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:368: ERROR: max gap %zu too small for required %zu\n\0"
                    as *const u8 as *const i8,
                Max,
                N,
            );
        }
        pma_errno = 369 as i32;
        return 0 as *mut libc::c_void;
    }
    r = (Amax as *mut i8)
        .offset(Max.wrapping_sub(N).wrapping_div(2 as i32 as u64) as isize);
    if (r as uintptr_t).wrapping_rem(C2RustUnnamed_1::ALGN as i32 as u64) != 0 {
        r = r
            .offset(
                (C2RustUnnamed_1::ALGN as i32 as uintptr_t)
                    .wrapping_sub(
                        (r as uintptr_t)
                            .wrapping_rem(C2RustUnnamed_1::ALGN as i32 as u64),
                    ) as isize,
            );
    }
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:375: FYI: addrgap returns %p == %lu\n\0" as *const u8 as *const i8,
            r as *mut libc::c_void,
            r as uintptr_t,
        );
    }
    return r as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn pma_init(mut verbose: i32, mut file: *const i8) -> i32 {
    let mut fd: i32 = 0;
    let mut pwr2flag: i32 = 0 as i32;
    let mut ps: i64 = 0;
    let mut pwr2: i64 = 0;
    let mut a1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut a2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ev: *mut i8 = 0 as *mut i8;
    let mut as_0: size_t = ::core::mem::size_of::<*mut libc::c_void>() as u64;
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut h: *mut pma_hdr_t = 0 as *mut pma_hdr_t;
    if !(0 as i32 <= verbose && 3 as i32 >= verbose) {
        pma_errno = 386 as i32;
        return 386 as i32;
    }
    state.vrb = verbose;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:388: FYI: pma_init(%d,\"%s\")\n\0" as *const u8 as *const i8,
            verbose,
            file,
        );
    }
    ev = getenv(b"PMA_VERBOSITY\0" as *const u8 as *const i8);
    if !ev.is_null() {
        let mut newvrb: i32 = 0;
        if 1 as i32
            != sscanf(ev, b"%1d\0" as *const u8 as *const i8, &mut newvrb as *mut i32)
        {
            if (0 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:391: ERROR: parsing envar verbosity \"%s\"\n\0" as *const u8
                        as *const i8,
                    ev,
                );
            }
            pma_errno = 391 as i32;
            return 391 as i32;
        }
        if !(0 as i32 <= newvrb && 3 as i32 >= newvrb) {
            if (0 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:392: ERROR: bad envar verbosity %d\n\0" as *const u8
                        as *const i8,
                    newvrb,
                );
            }
            pma_errno = 392 as i32;
            return 392 as i32;
        }
        state.vrb = newvrb;
        if (1 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:394: Warning: envar verbosity over-ride %d -> %d\n\0"
                    as *const u8 as *const i8,
                verbose,
                newvrb,
            );
        }
    }
    if state.init != 0 {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:396: ERROR: already initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 396 as i32;
        return 396 as i32;
    }
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:398: FYI: software version '%s' expects backing file format version %d\n\0"
                as *const u8 as *const i8,
            pma_version.as_ptr(),
            C2RustUnnamed_1::VERS as i32,
        );
    }
    if strcmp(
        pma_version.as_ptr(),
        b"2022.10Oct.30.1667172241 (Avon 8-g1)\0" as *const u8 as *const i8,
    ) != 0
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:400: ERROR: software version mismatch: '%s' / '%s'\n\0"
                    as *const u8 as *const i8,
                pma_version.as_ptr(),
                b"2022.10Oct.30.1667172241 (Avon 8-g1)\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 401 as i32;
        return 401 as i32;
    }
    if !(C2RustUnnamed_1::WDSZ as i32 as u64
        == ::core::mem::size_of::<*mut libc::c_void>() as u64
        && C2RustUnnamed_1::WDSZ as i32 as u64 == ::core::mem::size_of::<size_t>() as u64
        && C2RustUnnamed_1::WDSZ as i32 as u64 == ::core::mem::size_of::<off_t>() as u64
        && C2RustUnnamed_1::WDSZ as i32 as u64 == ::core::mem::size_of::<u64>() as u64)
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:408: ERROR: word size not 64 bits\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 408 as i32;
        return 408 as i32;
    }
    if file.is_null() {
        if (1 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:411: Warning: no backing file provided; falling back on standard malloc\n\0"
                    as *const u8 as *const i8,
            );
        }
        state.init = 2 as i32;
        state.file = 0 as *const i8;
        state.hdr = 0 as *mut pma_hdr_t;
        return 0 as i32;
    }
    ps = sysconf(_SC_PAGESIZE as i32);
    if 4096 as i32 as i64 > ps {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:418: ERROR: bad page size %ld, errno '%s'\n\0" as *const u8
                    as *const i8,
                ps,
                strerror(*__errno_location()),
            );
        }
        pma_errno = 418 as i32;
        return 418 as i32;
    }
    pwr2 = 4096 as i32 as i64;
    while pwr2 <= C2RustUnnamed_1::ALGN as i32 as i64 {
        if pwr2 == ps {
            pwr2flag = 1 as i32;
            break;
        } else {
            pwr2 *= 2 as i32 as i64;
        }
    }
    if 0 as i32 == pwr2flag {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:422: ERROR: page size %ld not a reasonable power of two\n\0"
                    as *const u8 as *const i8,
                ps,
            );
        }
        pma_errno = 422 as i32;
        return 422 as i32;
    }
    fd = open(file, 0o2 as i32);
    if 0 as i32 > fd {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:424: ERROR: open() errno => '%s'\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        pma_errno = 424 as i32;
        return 424 as i32;
    }
    if 0 as i32 != fstat(fd, &mut s) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:425: ERROR: fstat() errno => '%s'\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        pma_errno = 425 as i32;
        return 425 as i32;
    }
    if !(s.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:426: ERROR: %s not regular file\n\0" as *const u8 as *const i8,
                file,
            );
        }
        pma_errno = 426 as i32;
        return 426 as i32;
    }
    if as_0 as ssize_t
        != read(fd, &mut a1 as *mut *mut libc::c_void as *mut libc::c_void, as_0)
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:427: ERROR: read() errno => '%s'\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        pma_errno = 427 as i32;
        return 427 as i32;
    }
    if a1.is_null() {
        a1 = addrgap(s.st_size);
    }
    if a1.is_null() {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:429: ERROR: addrgap() errno => '%s'\n\0" as *const u8
                    as *const i8,
                strerror(*__errno_location()),
            );
        }
        return 429 as i32;
    }
    if (2 as i32) < state.vrb {
        fprintf(stderr, b"pma.c:430: FYI: map at %p\n\0" as *const u8 as *const i8, a1);
    }
    a2 = mmap(
        a1,
        s.st_size as size_t,
        0x1 as i32 | 0x2 as i32,
        0x1 as i32,
        fd,
        0 as i32 as __off_t,
    );
    if a1 != a2 {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:431: ERROR: mmap() errno => '%s'\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        pma_errno = 431 as i32;
        return 431 as i32;
    }
    if 0 as i32 != close(fd) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:432: ERROR: close() errno => '%s'\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        pma_errno = 432 as i32;
    }
    state.init = 1 as i32;
    state.file = file;
    h = a2 as *mut pma_hdr_t;
    state.hdr = h;
    if ((*h).mapaddr).is_null() {
        let mut i: i32 = 0;
        let mut w: *mut ao_t = 0 as *mut ao_t;
        let mut ftr: *mut *mut ao_t = 0 as *mut *mut ao_t;
        if (2 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:439: FYI: initializing persistent heap\n\0" as *const u8
                    as *const i8,
            );
        }
        if s.st_size % ps != 0 {
            if (0 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:441: ERROR: backing file size %jd not multiple of page size %ld\n\0"
                        as *const u8 as *const i8,
                    s.st_size,
                    ps,
                );
            }
            pma_errno = 442 as i32;
            return 442 as i32;
        }
        i = 0 as i32;
        while i < C2RustUnnamed_1::NFL as i32 {
            (*h).free[i as usize].fnext = &mut *((*h).free)
                .as_mut_ptr()
                .offset(i as isize) as *mut ao_t;
            (*h).free[i as usize].fprev = (*h).free[i as usize].fnext;
            i += 1;
            i;
        }
        (*h).mapaddr = h as *mut libc::c_void;
        (*h).bf_vers = C2RustUnnamed_1::VERS as i32 as uint64_t;
        (*h).nallocs = 0 as i32 as uint64_t;
        (*h).nfrees = 0 as i32 as uint64_t;
        (*h).res_0 = 0 as i32 as uint64_t;
        (*h).afirst = h.offset(1 as i32 as isize) as *mut ao_t;
        (*h).abound = (h as *mut i8).offset(s.st_size as isize) as *mut ao_t;
        w = (*h).afirst;
        (*w).anext = (*h).abound;
        ftr = ((*h).abound as *mut *mut ao_t).offset(-(1 as i32 as isize));
        *ftr = w;
        fli(w);
    } else {
        if (2 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:476: FYI: persistent heap already initialized\n\0" as *const u8
                    as *const i8,
            );
        }
        if s.st_size % ps != 0 {
            if (1 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:481: Warning: backing file size %jd not multiple of page size %ld\n\0"
                        as *const u8 as *const i8,
                    s.st_size,
                    ps,
                );
            }
        }
        if C2RustUnnamed_1::VERS as i32 as u64 != (*h).bf_vers {
            if (0 as i32) < state.vrb {
                fprintf(
                    stderr,
                    b"pma.c:483: ERROR: backing file version mismatch: %d vs. %lu\n\0"
                        as *const u8 as *const i8,
                    C2RustUnnamed_1::VERS as i32,
                    (*h).bf_vers,
                );
            }
            pma_errno = 484 as i32;
            return 484 as i32;
        }
        sc(1 as i32 as size_t);
    }
    return 0 as i32;
}
unsafe extern "C" fn split_ao(mut p: *mut ao_t, mut s: size_t) -> *mut ao_t {
    let mut c: size_t = (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
        .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t)
        .wrapping_sub(C2RustUnnamed_1::WDSZ as i32 as u64);
    let mut Cw: size_t = c.wrapping_div(C2RustUnnamed_1::WDSZ as i32 as u64);
    let mut Sw: size_t = 0;
    let mut iu: i32 = 0;
    let mut piu: i32 = 0;
    let mut grown: i32 = 0;
    let mut n: *mut ao_t = 0 as *mut ao_t;
    if s < 24 as i32 as u64 {
        s = 24 as i32 as size_t;
    }
    Sw = s
        .wrapping_div(C2RustUnnamed_1::WDSZ as i32 as u64)
        .wrapping_add(
            (s.wrapping_rem(C2RustUnnamed_1::WDSZ as i32 as u64) != 0) as i32 as u64,
        );
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:503: FYI: split_ao(%p,%zu) AOCAP %zu words req %zu words cap %zu\n\0"
                as *const u8 as *const i8,
            p as *mut libc::c_void,
            s,
            c,
            Sw,
            Cw,
        );
    }
    globh(p, &mut iu, &mut piu, &mut grown);
    if 4 as i32 as u64 <= Cw.wrapping_sub(Sw) {
        let mut rem: *mut ao_t = (&mut (*p).fprev as *mut *mut ao).offset(Sw as isize)
            as *mut ao_t;
        let mut rft: *mut *mut ao_t = (((*p).anext as uintptr_t & himask) as *mut ao_t
            as *mut *mut ao_t)
            .offset(-(1 as i32 as isize));
        if (2 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:508: FYI: splitting at %p\n\0" as *const u8 as *const i8,
                rem as *mut libc::c_void,
            );
        }
        (*rem).anext = ((*p).anext as uintptr_t & himask) as *mut ao_t;
        *rft = rem;
        fli(rem);
        (*p).anext = rem;
    }
    slobh(p, 1 as i32, piu, grown);
    n = ((*p).anext as uintptr_t & himask) as *mut ao_t;
    if n < (*state.hdr).abound {
        globh(n, &mut iu, &mut piu, &mut grown);
        slobh(n, iu, 1 as i32, grown);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn pma_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut r: *mut ao_t = 0 as *mut ao_t;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:530: FYI: malloc(%zu)\n\0" as *const u8 as *const i8,
            size,
        );
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:531: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 531 as i32;
        return 0 as *mut libc::c_void;
    }
    if 2 as i32 == state.init {
        return malloc(size);
    }
    if 0 as i32 as u64 >= size {
        if (1 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:535: Warning: malloc(%zu) argument <= zero\n\0" as *const u8
                    as *const i8,
                size,
            );
        }
        pma_errno = 535 as i32;
        return 0 as *mut libc::c_void;
    }
    let mut c: i32 = sc(size);
    's_115: while c < C2RustUnnamed_1::NFL as i32 {
        let mut h: *mut ao_t = &mut *((*state.hdr).free).as_mut_ptr().offset(c as isize)
            as *mut ao_t;
        let mut f: *mut ao_t = (*h).fnext;
        while f != h {
            if (((*f).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
                .wrapping_sub((f as uintptr_t & himask) as *mut ao_t as uintptr_t)
                .wrapping_sub(C2RustUnnamed_1::WDSZ as i32 as u64) >= size
            {
                r = f;
                break 's_115;
            } else {
                f = (*f).fnext;
            }
        }
        c += 1;
        c;
    }
    if !r.is_null() {
        flr(r);
        r = split_ao(r, size);
        if (2 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:551: FYI: malloc returning %p\n\0" as *const u8 as *const i8,
                &mut (*r).fprev as *mut *mut ao as *mut libc::c_void,
            );
        }
        (*state.hdr).nallocs = ((*state.hdr).nallocs).wrapping_add(1);
        (*state.hdr).nallocs;
        return &mut (*r).fprev as *mut *mut ao as *mut libc::c_void;
    } else {
        if (1 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:557: Warning: malloc(%zu) cannot satisfy request at this time\n\0"
                    as *const u8 as *const i8,
                size,
            );
        }
        pma_errno = 558 as i32;
        return 0 as *mut libc::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pma_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:564: FYI: calloc(%zu,%zu)\n\0" as *const u8 as *const i8,
            nmemb,
            size,
        );
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:565: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 565 as i32;
        return 0 as *mut libc::c_void;
    }
    if 2 as i32 == state.init {
        return calloc(nmemb, size);
    }
    if 0 as i32 as u64 >= nmemb || 0 as i32 as u64 >= size {
        if (1 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:568: Warning: calloc(%zu,%zu) argument <= zero\n\0" as *const u8
                    as *const i8,
                nmemb,
                size,
            );
        }
        pma_errno = 568 as i32;
        return 0 as *mut libc::c_void;
    }
    if nmemb > (18446744073709551615 as u64).wrapping_div(size) {
        if (1 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:571: Warning: calloc(%zu,%zu) arguments overflow\n\0"
                    as *const u8 as *const i8,
                nmemb,
                size,
            );
        }
        pma_errno = 571 as i32;
        return 0 as *mut libc::c_void;
    }
    p = pma_malloc(nmemb.wrapping_mul(size));
    if !p.is_null() {
        memset(p, 0 as i32, nmemb.wrapping_mul(size));
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn pma_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut ao_t = 0 as *mut ao_t;
    let mut nu: *mut libc::c_void = 0 as *mut libc::c_void;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:579: FYI: realloc(%p,%zu)\n\0" as *const u8 as *const i8,
            ptr,
            size,
        );
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:580: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 580 as i32;
        return 0 as *mut libc::c_void;
    }
    if 2 as i32 == state.init {
        return realloc(ptr, size);
    }
    if ptr.is_null() {
        return pma_malloc(size);
    }
    if 0 as i32 as u64 >= size {
        pma_free(ptr);
        return 0 as *mut libc::c_void;
    }
    p = (ptr as *mut *mut ao_t).offset(-(1 as i32 as isize)) as *mut ao_t;
    if (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
        .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t)
        .wrapping_sub(C2RustUnnamed_1::WDSZ as i32 as u64) >= size
    {
        return ptr;
    }
    nu = pma_malloc(size);
    if nu.is_null() {
        pma_errno = 589 as i32;
        return 0 as *mut libc::c_void;
    }
    memcpy(
        nu,
        ptr,
        (((*p).anext as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_sub((p as uintptr_t & himask) as *mut ao_t as uintptr_t)
            .wrapping_sub(C2RustUnnamed_1::WDSZ as i32 as u64),
    );
    pma_free(ptr);
    return nu;
}
unsafe extern "C" fn coalesce(mut p: *mut ao_t, mut flr_lo_hi: i32) -> i32 {
    let mut n: *mut ao_t = ((*p).anext as uintptr_t & himask) as *mut ao_t;
    let mut ftr: *mut *mut ao_t = 0 as *mut *mut ao_t;
    let mut piu: i32 = 0;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:600: FYI: coalesce(%p)\n\0" as *const u8 as *const i8,
            p as *mut libc::c_void,
        );
    }
    if n >= (*state.hdr).abound {
        return 0 as i32;
    }
    if 0 as i32 == getbit(n, C2RustUnnamed_2::IU as i32) {
        flr(if flr_lo_hi != 0 { n } else { p });
        piu = getbit(p, C2RustUnnamed_2::PIU as i32);
        (*p).anext = ((*n).anext as uintptr_t & himask) as *mut ao_t;
        ftr = ((*p).anext as *mut *mut ao_t).offset(-(1 as i32 as isize));
        *ftr = p;
        slobh(p, 0 as i32, piu, 0 as i32);
        return 1 as i32;
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn pma_free(mut ptr: *mut libc::c_void) {
    let mut p: *mut ao_t = 0 as *mut ao_t;
    let mut n: *mut ao_t = 0 as *mut ao_t;
    let mut ftr: *mut *mut ao_t = 0 as *mut *mut ao_t;
    let mut r: i32 = 0;
    if (2 as i32) < state.vrb {
        fprintf(stderr, b"pma.c:618: FYI: free(%p)\n\0" as *const u8 as *const i8, ptr);
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:619: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 619 as i32;
        return;
    }
    if 2 as i32 == state.init {
        free(ptr);
        return;
    }
    if ptr.is_null() {
        return;
    }
    if !((*state.hdr).afirst as *mut libc::c_void <= ptr
        && (*state.hdr).abound as *mut libc::c_void > ptr)
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:625: ERROR: freed ptr %p outside allocatable area bounds %p %p\n\0"
                    as *const u8 as *const i8,
                ptr,
                (*state.hdr).afirst as *mut libc::c_void,
                (*state.hdr).abound as *mut libc::c_void,
            );
        }
        pma_errno = 626 as i32;
        return;
    }
    p = (ptr as *mut *mut ao_t).offset(-(1 as i32 as isize)) as *mut ao_t;
    n = ((*p).anext as uintptr_t & himask) as *mut ao_t;
    slobh(p, 0 as i32, getbit(p, C2RustUnnamed_2::PIU as i32), 0 as i32);
    n < (*state.hdr).abound;
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:639: FYI: merge with right/higher ao\n\0" as *const u8 as *const i8,
        );
    }
    r = coalesce(p, 1 as i32);
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:641: FYI: %s\n\0" as *const u8 as *const i8,
            if r != 0 {
                b"yes\0" as *const u8 as *const i8
            } else {
                b"no\0" as *const u8 as *const i8
            },
        );
    }
    if 0 as i32 == getbit(p, C2RustUnnamed_2::PIU as i32) && p > (*state.hdr).afirst {
        let mut prev: *mut ao_t = *(p as *mut *mut ao_t).offset(-(1 as i32 as isize));
        if (2 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:645: FYI: merge with left/lower ao\n\0" as *const u8 as *const i8,
            );
        }
        r = coalesce(prev, 0 as i32);
        p = prev;
    }
    n = ((*p).anext as uintptr_t & himask) as *mut ao_t;
    ftr = (n as *mut *mut ao_t).offset(-(1 as i32 as isize));
    *ftr = p;
    if n < (*state.hdr).abound {
        slobh(
            n,
            getbit(n, C2RustUnnamed_2::IU as i32),
            0 as i32,
            getbit(n, C2RustUnnamed_2::GROWN as i32),
        );
    }
    fli(p);
    (*state.hdr).nfrees = ((*state.hdr).nfrees).wrapping_add(1);
    (*state.hdr).nfrees;
}
#[no_mangle]
pub unsafe extern "C" fn pma_set_root(mut p: *mut libc::c_void) {
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:665: FYI: set_root(%p)\n\0" as *const u8 as *const i8,
            p,
        );
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:666: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 666 as i32;
        return;
    }
    if 2 as i32 == state.init {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:667: ERROR: set_root not meaningful in fallback mode\n\0"
                    as *const u8 as *const i8,
            );
        }
        pma_errno = 667 as i32;
        return;
    }
    if !(p.is_null()
        || (*state.hdr).afirst as *mut libc::c_void <= p
            && (*state.hdr).abound as *mut libc::c_void > p)
    {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:668: ERROR: bad root %p\n\0" as *const u8 as *const i8,
                p,
            );
        }
        pma_errno = 668 as i32;
        return;
    }
    (*state.hdr).root = p;
}
#[no_mangle]
pub unsafe extern "C" fn pma_get_root() -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if (2 as i32) < state.vrb {
        fprintf(stderr, b"pma.c:676: FYI: get_root()\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:677: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 677 as i32;
        return 0 as *mut libc::c_void;
    }
    if 2 as i32 == state.init {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:678: ERROR: get_root not meaningful in fallback mode\n\0"
                    as *const u8 as *const i8,
            );
        }
        pma_errno = 678 as i32;
        return 0 as *mut libc::c_void;
    }
    p = (*state.hdr).root;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn pma_set_avail_mem(v: ul_t) {
    if (2 as i32) < state.vrb {
        fprintf(
            stderr,
            b"pma.c:686: FYI: set_avail_mem(0x%lx)\n\0" as *const u8 as *const i8,
            v,
        );
    }
    if !(1 as i32 == state.init || 2 as i32 == state.init) {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:687: ERROR: not initialized\n\0" as *const u8 as *const i8,
            );
        }
        pma_errno = 687 as i32;
        return;
    }
    if 2 as i32 == state.init {
        if (0 as i32) < state.vrb {
            fprintf(
                stderr,
                b"pma.c:688: ERROR: set_avail_mem not meaningful in fallback mode\n\0"
                    as *const u8 as *const i8,
            );
        }
        pma_errno = 688 as i32;
        return;
    }
    let mut i: i32 = 0 as i32;
    while i < C2RustUnnamed_1::NFL as i32 {
        let mut p: *mut ao_t = 0 as *mut ao_t;
        let mut f: *mut ao_t = &mut *((*state.hdr).free).as_mut_ptr().offset(i as isize)
            as *mut ao_t;
        if (*f).fprev != f {
            p = (*f).fnext;
            while p != f {
                let mut q: *mut ul_t = p.offset(1 as i32 as isize) as *mut ul_t;
                let mut e: *mut ul_t = (((*p).anext as uintptr_t & himask) as *mut ao_t
                    as *mut ul_t)
                    .offset(-(1 as i32 as isize));
                while q != e {
                    if *q != v {
                        *q = v;
                    }
                    q = q.offset(1);
                    q;
                }
                while q != e {
                    q = q.offset(1);
                    q;
                }
                p = (*p).fnext;
            }
        }
        i += 1;
        i;
    }
}