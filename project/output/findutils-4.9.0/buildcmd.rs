use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn mbsstr(haystack: *const i8, needle: *const i8) -> *mut i8;
    static mut environ: *mut *mut i8;
    fn sysconf(__name: i32) -> i64;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrtoul(
        _: *const i8,
        _: *mut *mut i8,
        _: i32,
        _: *mut u64,
        _: *const i8,
    ) -> strtol_error;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
}
pub type size_t = u64;
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
pub enum strtol_error {
    LONGINT_OK = 0,
    LONGINT_OVERFLOW,
    LONGINT_INVALID_SUFFIX_CHAR,
    LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
    LONGINT_INVALID = 4,
}
impl strtol_error {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            strtol_error::LONGINT_OK => 0,
            strtol_error::LONGINT_OVERFLOW => 1,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR => 2,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => 3,
            strtol_error::LONGINT_INVALID => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> strtol_error {
        match value {
            0 => strtol_error::LONGINT_OK,
            1 => strtol_error::LONGINT_OVERFLOW,
            2 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR,
            3 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
            4 => strtol_error::LONGINT_INVALID,
            _ => panic!("Invalid value for strtol_error: {}", value),
        }
    }
}
impl AddAssign<u32> for strtol_error {
    fn add_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for strtol_error {
    fn sub_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for strtol_error {
    fn mul_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for strtol_error {
    fn div_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for strtol_error {
    fn rem_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for strtol_error {
    type Output = strtol_error;
    fn add(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for strtol_error {
    type Output = strtol_error;
    fn sub(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for strtol_error {
    type Output = strtol_error;
    fn mul(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for strtol_error {
    type Output = strtol_error;
    fn div(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for strtol_error {
    type Output = strtol_error;
    fn rem(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    arg_size = 131072,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::arg_size => 131072,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            131072 => C2RustUnnamed_4::arg_size,
            _ => panic!("Invalid value for C2RustUnnamed_4: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_4 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_4 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_4 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_4 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_4 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn add(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn sub(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn mul(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn div(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn rem(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: i32,
}
static mut special_terminating_arg: *const i8 = b"do_not_care\0" as *const u8
    as *const i8;
unsafe extern "C" fn bc_args_complete(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
) {
    bc_push_arg(
        ctl,
        state,
        special_terminating_arg,
        0 as i32 as size_t,
        0 as *const i8,
        0 as i32 as size_t,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bc_do_insert(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut arg: *mut i8,
    mut arglen: size_t,
    mut prefix: *const i8,
    mut pfxlen: size_t,
    mut linebuf: *const i8,
    mut lblen: size_t,
    mut initial_args: i32,
) {
    static mut insertbuf: *mut i8 = 0 as *const i8 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut bytes_left: size_t = ((*ctl).arg_max).wrapping_sub(1 as i32 as u64);
    if insertbuf.is_null() {
        insertbuf = xmalloc(((*ctl).arg_max).wrapping_add(1 as i32 as u64)) as *mut i8;
    }
    p = insertbuf;
    loop {
        let mut len: size_t = 0;
        let mut s: *mut i8 = mbsstr(arg, (*ctl).replace_pat);
        if !s.is_null() {
            len = s.offset_from(arg) as i64 as size_t;
        } else {
            len = arglen;
        }
        if bytes_left <= len {
            break;
        }
        bytes_left = (bytes_left as u64).wrapping_sub(len) as size_t as size_t;
        strncpy(p, arg, len);
        p = p.offset(len as isize);
        arg = arg.offset(len as isize);
        arglen = (arglen as u64).wrapping_sub(len) as size_t as size_t;
        if !s.is_null() {
            if bytes_left <= lblen.wrapping_add(pfxlen) {
                break;
            }
            bytes_left = (bytes_left as u64).wrapping_sub(lblen.wrapping_add(pfxlen))
                as size_t as size_t;
            if !prefix.is_null() {
                strcpy(p, prefix);
                p = p.offset(pfxlen as isize);
            }
            strcpy(p, linebuf);
            p = p.offset(lblen as isize);
            arg = arg.offset((*ctl).rplen as isize);
            arglen = (arglen as u64).wrapping_sub((*ctl).rplen) as size_t as size_t;
        }
        if !(*arg != 0) {
            break;
        }
    }
    if *arg != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"command too long\0" as *const u8 as *const i8,
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
                    b"command too long\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\0' as i32 as i8;
    bc_push_arg(
        ctl,
        state,
        insertbuf,
        p.offset_from(insertbuf) as i64 as size_t,
        0 as *const i8,
        0 as i32 as size_t,
        initial_args,
    );
}
unsafe extern "C" fn update_limit(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut success: bool,
    mut limit: size_t,
) -> size_t {
    if success {
        if limit > (*state).largest_successful_arg_count {
            (*state).largest_successful_arg_count = limit;
        }
    } else if limit < (*state).smallest_failed_arg_count
        || 0 as i32 as u64 == (*state).smallest_failed_arg_count
    {
        (*state).smallest_failed_arg_count = limit;
    }
    if 0 as i32 as u64 == (*state).largest_successful_arg_count
        || (*state).smallest_failed_arg_count <= (*state).largest_successful_arg_count
    {
        if success {
            if limit < 18446744073709551615 as u64 {
                limit = limit.wrapping_add(1);
                limit;
            }
        } else {
            limit = (limit as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
        }
    } else {
        let shift: size_t = ((*state).smallest_failed_arg_count)
            .wrapping_sub((*state).largest_successful_arg_count)
            .wrapping_div(2 as i32 as u64);
        if success {
            if shift != 0 {
                limit = (limit as u64).wrapping_add(shift) as size_t as size_t;
            } else {
                limit = limit.wrapping_add(1);
                limit;
            }
        } else if shift != 0 {
            limit = (limit as u64).wrapping_sub(shift) as size_t as size_t;
        } else {
            limit = limit.wrapping_sub(1);
            limit;
        }
    }
    if (*ctl).initial_argc != 0
        && limit <= ((*ctl).initial_argc).wrapping_add(1 as u32 as u64)
    {
        limit = ((*ctl).initial_argc).wrapping_add(1 as u32 as u64);
    }
    if 0 as i32 as u64 == limit {
        limit = 1 as u32 as size_t;
    }
    return limit;
}
unsafe extern "C" fn copy_args(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut working_args: *mut *mut i8,
    mut limit: size_t,
    mut done: size_t,
) -> size_t {
    let mut dst_pos: size_t = 0 as i32 as size_t;
    let mut src_pos: size_t = 0 as i32 as size_t;
    while src_pos < (*ctl).initial_argc {
        let fresh1 = src_pos;
        src_pos = src_pos.wrapping_add(1);
        let fresh2 = dst_pos;
        dst_pos = dst_pos.wrapping_add(1);
        let ref mut fresh3 = *working_args.offset(fresh2 as isize);
        *fresh3 = *((*state).cmd_argv).offset(fresh1 as isize);
    }
    src_pos = (src_pos as u64).wrapping_add(done) as size_t as size_t;
    while src_pos < (*state).cmd_argc && dst_pos < limit {
        let fresh4 = src_pos;
        src_pos = src_pos.wrapping_add(1);
        let fresh5 = dst_pos;
        dst_pos = dst_pos.wrapping_add(1);
        let ref mut fresh6 = *working_args.offset(fresh5 as isize);
        *fresh6 = *((*state).cmd_argv).offset(fresh4 as isize);
    }
    if dst_pos >= (*ctl).initial_argc {} else {
        __assert_fail(
            b"dst_pos >= ctl->initial_argc\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            242 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[i8; 94],
            >(
                b"size_t copy_args(struct buildcmd_control *, struct buildcmd_state *, char **, size_t, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4244: {
        if dst_pos >= (*ctl).initial_argc {} else {
            __assert_fail(
                b"dst_pos >= ctl->initial_argc\0" as *const u8 as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                242 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 94],
                    &[i8; 94],
                >(
                    b"size_t copy_args(struct buildcmd_control *, struct buildcmd_state *, char **, size_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let ref mut fresh7 = *working_args.offset(dst_pos as isize);
    *fresh7 = 0 as *mut i8;
    return dst_pos;
}
#[no_mangle]
pub unsafe extern "C" fn bc_do_exec(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
) {
    let mut working_args: *mut *mut i8 = 0 as *mut *mut i8;
    let mut limit: size_t = 0;
    let mut done: size_t = 0;
    bc_args_complete(ctl, state);
    if (*state).cmd_argc > 0 as i32 as u64 {} else {
        __assert_fail(
            b"state->cmd_argc > 0\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            261 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i8; 68],
            >(b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0"))
                .as_ptr(),
        );
    }
    'c_4472: {
        if (*state).cmd_argc > 0 as i32 as u64 {} else {
            __assert_fail(
                b"state->cmd_argc > 0\0" as *const u8 as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                261 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*((*state).cmd_argv)
        .offset(((*state).cmd_argc).wrapping_sub(1 as i32 as u64) as isize))
        .is_null()
    {} else {
        __assert_fail(
            b"state->cmd_argv[state->cmd_argc-1] == NULL\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            262 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i8; 68],
            >(b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0"))
                .as_ptr(),
        );
    }
    'c_4406: {
        if (*((*state).cmd_argv)
            .offset(((*state).cmd_argc).wrapping_sub(1 as i32 as u64) as isize))
            .is_null()
        {} else {
            __assert_fail(
                b"state->cmd_argv[state->cmd_argc-1] == NULL\0" as *const u8
                    as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                262 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    working_args = xmalloc(
        (1 as i32 as u64)
            .wrapping_add((*state).cmd_argc)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    done = 0 as i32 as size_t;
    limit = (*state).cmd_argc;
    loop {
        let dst_pos: size_t = copy_args(ctl, state, working_args, limit, done);
        if ((*ctl).exec_callback)
            .expect(
                "non-null function pointer",
            )(ctl, (*state).usercontext, dst_pos as i32, working_args) != 0
        {
            limit = update_limit(ctl, state, 1 as i32 != 0, limit);
            done = (done as u64).wrapping_add(dst_pos.wrapping_sub((*ctl).initial_argc))
                as size_t as size_t;
        } else if limit <= ((*ctl).initial_argc).wrapping_add(1 as i32 as u64) {
            if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"can't call exec() due to argument size restrictions\0"
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
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"can't call exec() due to argument size restrictions\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            limit = update_limit(ctl, state, 0 as i32 != 0, limit);
        }
        if !(done.wrapping_add(1 as i32 as u64)
            < ((*state).cmd_argc).wrapping_sub((*ctl).initial_argc))
        {
            break;
        }
    }
    rpl_free(working_args as *mut libc::c_void);
    bc_clear_args(ctl, state);
}
unsafe extern "C" fn bc_argc_limit_reached(
    mut initial_args: i32,
    mut ctl: *const buildcmd_control,
    mut state: *mut buildcmd_state,
) -> i32 {
    if initial_args == 0 && (*ctl).args_per_exec != 0
        && ((*state).cmd_argc).wrapping_sub((*ctl).initial_argc) == (*ctl).args_per_exec
    {
        return 1 as i32;
    }
    return ((*state).cmd_argc == (*ctl).max_arg_count) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bc_push_arg(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut arg: *const i8,
    mut len: size_t,
    mut prefix: *const i8,
    mut pfxlen: size_t,
    mut initial_args: i32,
) {
    let terminate: i32 = (arg == special_terminating_arg) as i32;
    if !arg.is_null() {} else {
        __assert_fail(
            b"arg != NULL\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            341 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 118],
                &[i8; 118],
            >(
                b"void bc_push_arg(struct buildcmd_control *, struct buildcmd_state *, const char *, size_t, const char *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4987: {
        if !arg.is_null() {} else {
            __assert_fail(
                b"arg != NULL\0" as *const u8 as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                341 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 118],
                    &[i8; 118],
                >(
                    b"void bc_push_arg(struct buildcmd_control *, struct buildcmd_state *, const char *, size_t, const char *, size_t, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if terminate == 0 {
        if ((*state).cmd_argv_chars).wrapping_add(len).wrapping_add(pfxlen)
            > (*ctl).arg_max
        {
            if initial_args != 0 || (*state).cmd_argc == (*ctl).initial_argc {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"cannot fit single argument within argument list size limit\0"
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
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"cannot fit single argument within argument list size limit\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if !((*ctl).replace_pat).is_null()
                || (*ctl).exit_if_size_exceeded != 0
                    && ((*ctl).lines_per_exec != 0 || (*ctl).args_per_exec != 0)
            {
                if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"argument list too long\0" as *const u8 as *const i8,
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
                            b"argument list too long\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            bc_do_exec(ctl, state);
        }
        if bc_argc_limit_reached(initial_args, ctl, state) != 0 {
            bc_do_exec(ctl, state);
        }
    }
    if initial_args == 0 {
        (*state).todo = 1 as i32;
    }
    if (*state).cmd_argc >= (*state).cmd_argv_alloc {
        if ((*state).cmd_argv).is_null() {
            (*state).cmd_argv_alloc = 64 as i32 as size_t;
            (*state).cmd_argv = xmalloc(
                (::core::mem::size_of::<*mut i8>() as u64)
                    .wrapping_mul((*state).cmd_argv_alloc),
            ) as *mut *mut i8;
        } else {
            (*state).cmd_argv_alloc = ((*state).cmd_argv_alloc as u64)
                .wrapping_mul(2 as i32 as u64) as size_t as size_t;
            (*state).cmd_argv = xrealloc(
                (*state).cmd_argv as *mut libc::c_void,
                (::core::mem::size_of::<*mut i8>() as u64)
                    .wrapping_mul((*state).cmd_argv_alloc),
            ) as *mut *mut i8;
        }
    }
    if terminate != 0 {
        let fresh8 = (*state).cmd_argc;
        (*state).cmd_argc = ((*state).cmd_argc).wrapping_add(1);
        let ref mut fresh9 = *((*state).cmd_argv).offset(fresh8 as isize);
        *fresh9 = 0 as *mut i8;
    } else {
        let fresh10 = (*state).cmd_argc;
        (*state).cmd_argc = ((*state).cmd_argc).wrapping_add(1);
        let ref mut fresh11 = *((*state).cmd_argv).offset(fresh10 as isize);
        *fresh11 = ((*state).argbuf).offset((*state).cmd_argv_chars as isize);
        if !prefix.is_null() {
            strcpy(((*state).argbuf).offset((*state).cmd_argv_chars as isize), prefix);
            (*state).cmd_argv_chars = ((*state).cmd_argv_chars as u64)
                .wrapping_add(pfxlen) as size_t as size_t;
        }
        strcpy(((*state).argbuf).offset((*state).cmd_argv_chars as isize), arg);
        (*state).cmd_argv_chars = ((*state).cmd_argv_chars as u64).wrapping_add(len)
            as size_t as size_t;
        if bc_argc_limit_reached(initial_args, ctl, state) != 0 {
            bc_do_exec(ctl, state);
        }
    }
    if initial_args != 0 {
        (*state).cmd_initial_argv_chars = (*state).cmd_argv_chars;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bc_get_arg_max() -> size_t {
    let mut val: i64 = 0;
    if !(0 as i32 as size_t) >= 9223372036854775807 as i64 as u64 {} else {
        __assert_fail(
            b"(~(size_t)0) >= LONG_MAX\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            421 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[i8; 28],
            >(b"size_t bc_get_arg_max(void)\0"))
                .as_ptr(),
        );
    }
    'c_5669: {
        if !(0 as i32 as size_t) >= 9223372036854775807 as i64 as u64 {} else {
            __assert_fail(
                b"(~(size_t)0) >= LONG_MAX\0" as *const u8 as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                421 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[i8; 28],
                >(b"size_t bc_get_arg_max(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = sysconf(_SC_ARG_MAX as i32);
    if val > 0 as i32 as i64 {
        return val as size_t;
    }
    return 9223372036854775807 as i64 as size_t;
}
unsafe extern "C" fn cb_exec_noop(
    mut ctl: *mut buildcmd_control,
    mut usercontext: *mut libc::c_void,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bc_size_of_environment() -> size_t {
    let mut len: size_t = 0 as u32 as size_t;
    let mut envp: *mut *mut i8 = environ;
    while !(*envp).is_null() {
        let fresh12 = envp;
        envp = envp.offset(1);
        len = (len as u64).wrapping_add((strlen(*fresh12)).wrapping_add(1 as i32 as u64))
            as size_t as size_t;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn bc_init_controlinfo(
    mut ctl: *mut buildcmd_control,
    mut headroom: size_t,
) -> BC_INIT_STATUS {
    let mut size_of_environment: size_t = bc_size_of_environment();
    (*ctl).posix_arg_size_min = 4096 as i32 as size_t;
    (*ctl).posix_arg_size_max = bc_get_arg_max();
    (*ctl).exit_if_size_exceeded = 0 as i32;
    if size_of_environment > (*ctl).posix_arg_size_max {
        return BC_INIT_STATUS::BC_INIT_ENV_TOO_BIG
    } else if headroom.wrapping_add(size_of_environment) >= (*ctl).posix_arg_size_max {
        return BC_INIT_STATUS::BC_INIT_CANNOT_ACCOMODATE_HEADROOM
    } else {
        (*ctl).posix_arg_size_max = ((*ctl).posix_arg_size_max as u64)
            .wrapping_sub(size_of_environment) as size_t as size_t;
        (*ctl).posix_arg_size_max = ((*ctl).posix_arg_size_max as u64)
            .wrapping_sub(headroom) as size_t as size_t;
    }
    (*ctl).max_arg_count = ((*ctl).posix_arg_size_max)
        .wrapping_div(::core::mem::size_of::<*mut i8>() as u64)
        .wrapping_sub(2 as u32 as u64);
    if (*ctl).max_arg_count > 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctl->max_arg_count > 0\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            518 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"enum BC_INIT_STATUS bc_init_controlinfo(struct buildcmd_control *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5526: {
        if (*ctl).max_arg_count > 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctl->max_arg_count > 0\0" as *const u8 as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                518 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"enum BC_INIT_STATUS bc_init_controlinfo(struct buildcmd_control *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*ctl).rplen = 0 as u32 as size_t;
    (*ctl).replace_pat = 0 as *const i8;
    (*ctl).initial_argc = 0 as i32 as size_t;
    (*ctl).exec_callback = Some(
        cb_exec_noop
            as unsafe extern "C" fn(
                *mut buildcmd_control,
                *mut libc::c_void,
                i32,
                *mut *mut i8,
            ) -> i32,
    );
    (*ctl).lines_per_exec = 0 as i32 as u64;
    (*ctl).args_per_exec = 0 as i32 as size_t;
    (*ctl).arg_max = (*ctl).posix_arg_size_max;
    return BC_INIT_STATUS::BC_INIT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn bc_use_sensible_arg_max(mut ctl: *mut buildcmd_control) {
    if C2RustUnnamed_4::arg_size as i32 as u64 > (*ctl).posix_arg_size_max {
        (*ctl).arg_max = (*ctl).posix_arg_size_max;
    } else if (C2RustUnnamed_4::arg_size as i32 as u64) < (*ctl).posix_arg_size_min {
        (*ctl).arg_max = (*ctl).posix_arg_size_min;
    } else {
        (*ctl).arg_max = C2RustUnnamed_4::arg_size as i32 as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bc_init_state(
    mut ctl: *const buildcmd_control,
    mut state: *mut buildcmd_state,
    mut context: *mut libc::c_void,
) {
    (*state).cmd_argc = 0 as i32 as size_t;
    (*state).cmd_argv_chars = 0 as i32 as size_t;
    (*state).cmd_argv = 0 as *mut *mut i8;
    (*state).cmd_argv_alloc = 0 as i32 as size_t;
    (*state).largest_successful_arg_count = 0 as i32 as size_t;
    (*state).smallest_failed_arg_count = 0 as i32 as size_t;
    if (*ctl).arg_max <= (9223372036854775807 as i64 - 2048 as i64) as u64 {} else {
        __assert_fail(
            b"ctl->arg_max <= (LONG_MAX - 2048L)\0" as *const u8 as *const i8,
            b"buildcmd.c\0" as *const u8 as *const i8,
            572 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[i8; 85],
            >(
                b"void bc_init_state(const struct buildcmd_control *, struct buildcmd_state *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5343: {
        if (*ctl).arg_max <= (9223372036854775807 as i64 - 2048 as i64) as u64 {} else {
            __assert_fail(
                b"ctl->arg_max <= (LONG_MAX - 2048L)\0" as *const u8 as *const i8,
                b"buildcmd.c\0" as *const u8 as *const i8,
                572 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 85],
                    &[i8; 85],
                >(
                    b"void bc_init_state(const struct buildcmd_control *, struct buildcmd_state *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*state).argbuf = xmalloc(((*ctl).arg_max).wrapping_add(1 as u32 as u64)) as *mut i8;
    (*state).cmd_initial_argv_chars = 0 as i32 as size_t;
    (*state).cmd_argv_chars = (*state).cmd_initial_argv_chars;
    (*state).todo = 0 as i32;
    (*state).dir_fd = -(1 as i32);
    (*state).usercontext = context;
}
#[no_mangle]
pub unsafe extern "C" fn bc_clear_args(
    mut ctl: *const buildcmd_control,
    mut state: *mut buildcmd_state,
) {
    (*state).cmd_argc = (*ctl).initial_argc;
    (*state).cmd_argv_chars = (*state).cmd_initial_argv_chars;
    (*state).todo = 0 as i32;
    (*state).dir_fd = -(1 as i32);
}
unsafe extern "C" fn exceeds(mut env_var_name: *const i8, mut quantity: size_t) -> i32 {
    let mut val: *const i8 = getenv(env_var_name);
    if !val.is_null() {
        let mut tmp: *mut i8 = 0 as *mut i8;
        let mut limit: u64 = 0;
        if xstrtoul(val, &mut tmp, 10 as i32, &mut limit, 0 as *const i8) as u32
            == strtol_error::LONGINT_OK as i32 as u32
        {
            if quantity > limit {
                return 1 as i32;
            }
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Environment variable %s is not set to a valid decimal number\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    env_var_name,
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
                        b"Environment variable %s is not set to a valid decimal number\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    env_var_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
            return 0 as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bc_args_exceed_testing_limit(mut argv: *mut *mut i8) -> bool {
    let mut chars: size_t = 0;
    let mut args: size_t = 0;
    args = 0 as i32 as size_t;
    chars = args;
    while !(*argv).is_null() {
        args = args.wrapping_add(1);
        args;
        chars = (chars as u64).wrapping_add(strlen(*argv)) as size_t as size_t;
        argv = argv.offset(1);
        argv;
    }
    return exceeds(
        b"__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT\0" as *const u8 as *const i8,
        args,
    ) != 0
        || exceeds(
            b"__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT\0" as *const u8 as *const i8,
            chars,
        ) != 0;
}