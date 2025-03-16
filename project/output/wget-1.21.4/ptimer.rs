#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn __errno_location() -> *mut libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_options {
    LOG_VERBOSE,
    LOG_NOTQUIET,
    LOG_NONVERBOSE,
    LOG_ALWAYS,
    LOG_PROGRESS,
}
impl log_options {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            log_options::LOG_VERBOSE => 0,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_PROGRESS => 4,
        }
    }
}

pub const LOG_PROGRESS: log_options = 4;
pub const LOG_ALWAYS: log_options = 3;
pub const LOG_NONVERBOSE: log_options = 2;
pub const LOG_NOTQUIET: log_options = 1;
pub const LOG_VERBOSE: log_options = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _SC_THREAD_ROBUST_PRIO_PROTECT = 248,
    _SC_THREAD_ROBUST_PRIO_INHERIT = 247,
    _SC_XOPEN_STREAMS = 246,
    _SC_TRACE_USER_EVENT_MAX = 245,
    _SC_TRACE_SYS_MAX = 244,
    _SC_TRACE_NAME_MAX = 243,
    _SC_TRACE_EVENT_NAME_MAX = 242,
    _SC_SS_REPL_MAX = 241,
    _SC_V7_LPBIG_OFFBIG = 240,
    _SC_V7_LP64_OFF64 = 239,
    _SC_V7_ILP32_OFFBIG = 238,
    _SC_V7_ILP32_OFF32 = 237,
    _SC_RAW_SOCKETS = 236,
    _SC_IPV6 = 235,
    _SC_LEVEL4_CACHE_LINESIZE = 199,
    _SC_LEVEL4_CACHE_ASSOC = 198,
    _SC_LEVEL4_CACHE_SIZE = 197,
    _SC_LEVEL3_CACHE_LINESIZE = 196,
    _SC_LEVEL3_CACHE_ASSOC = 195,
    _SC_LEVEL3_CACHE_SIZE = 194,
    _SC_LEVEL2_CACHE_LINESIZE = 193,
    _SC_LEVEL2_CACHE_ASSOC = 192,
    _SC_LEVEL2_CACHE_SIZE = 191,
    _SC_LEVEL1_DCACHE_LINESIZE = 190,
    _SC_LEVEL1_DCACHE_ASSOC = 189,
    _SC_LEVEL1_DCACHE_SIZE = 188,
    _SC_LEVEL1_ICACHE_LINESIZE = 187,
    _SC_LEVEL1_ICACHE_ASSOC = 186,
    _SC_LEVEL1_ICACHE_SIZE = 185,
    _SC_TRACE_LOG = 184,
    _SC_TRACE_INHERIT = 183,
    _SC_TRACE_EVENT_FILTER = 182,
    _SC_TRACE = 181,
    _SC_HOST_NAME_MAX = 180,
    _SC_V6_LPBIG_OFFBIG = 179,
    _SC_V6_LP64_OFF64 = 178,
    _SC_V6_ILP32_OFFBIG = 177,
    _SC_V6_ILP32_OFF32 = 176,
    _SC_2_PBS_CHECKPOINT = 175,
    _SC_STREAMS = 174,
    _SC_SYMLOOP_MAX = 173,
    _SC_2_PBS_TRACK = 172,
    _SC_2_PBS_MESSAGE = 171,
    _SC_2_PBS_LOCATE = 170,
    _SC_2_PBS_ACCOUNTING = 169,
    _SC_2_PBS = 168,
    _SC_USER_GROUPS_R = 167,
    _SC_USER_GROUPS = 166,
    _SC_TYPED_MEMORY_OBJECTS = 165,
    _SC_TIMEOUTS = 164,
    _SC_SYSTEM_DATABASE_R = 163,
    _SC_SYSTEM_DATABASE = 162,
    _SC_THREAD_SPORADIC_SERVER = 161,
    _SC_SPORADIC_SERVER = 160,
    _SC_SPAWN = 159,
    _SC_SIGNALS = 158,
    _SC_SHELL = 157,
    _SC_REGEX_VERSION = 156,
    _SC_REGEXP = 155,
    _SC_SPIN_LOCKS = 154,
    _SC_READER_WRITER_LOCKS = 153,
    _SC_NETWORKING = 152,
    _SC_SINGLE_PROCESS = 151,
    _SC_MULTI_PROCESS = 150,
    _SC_MONOTONIC_CLOCK = 149,
    _SC_FILE_SYSTEM = 148,
    _SC_FILE_LOCKING = 147,
    _SC_FILE_ATTRIBUTES = 146,
    _SC_PIPE = 145,
    _SC_FIFO = 144,
    _SC_FD_MGMT = 143,
    _SC_DEVICE_SPECIFIC_R = 142,
    _SC_DEVICE_SPECIFIC = 141,
    _SC_DEVICE_IO = 140,
    _SC_THREAD_CPUTIME = 139,
    _SC_CPUTIME = 138,
    _SC_CLOCK_SELECTION = 137,
    _SC_C_LANG_SUPPORT_R = 136,
    _SC_C_LANG_SUPPORT = 135,
    _SC_BASE = 134,
    _SC_BARRIERS = 133,
    _SC_ADVISORY_INFO = 132,
    _SC_XOPEN_REALTIME_THREADS = 131,
    _SC_XOPEN_REALTIME = 130,
    _SC_XOPEN_LEGACY = 129,
    _SC_XBS5_LPBIG_OFFBIG = 128,
    _SC_XBS5_LP64_OFF64 = 127,
    _SC_XBS5_ILP32_OFFBIG = 126,
    _SC_XBS5_ILP32_OFF32 = 125,
    _SC_NL_TEXTMAX = 124,
    _SC_NL_SETMAX = 123,
    _SC_NL_NMAX = 122,
    _SC_NL_MSGMAX = 121,
    _SC_NL_LANGMAX = 120,
    _SC_NL_ARGMAX = 119,
    _SC_USHRT_MAX = 118,
    _SC_ULONG_MAX = 117,
    _SC_UINT_MAX = 116,
    _SC_UCHAR_MAX = 115,
    _SC_SHRT_MIN = 114,
    _SC_SHRT_MAX = 113,
    _SC_SCHAR_MIN = 112,
    _SC_SCHAR_MAX = 111,
    _SC_SSIZE_MAX = 110,
    _SC_NZERO = 109,
    _SC_MB_LEN_MAX = 108,
    _SC_WORD_BIT = 107,
    _SC_LONG_BIT = 106,
    _SC_INT_MIN = 105,
    _SC_INT_MAX = 104,
    _SC_CHAR_MIN = 103,
    _SC_CHAR_MAX = 102,
    _SC_CHAR_BIT = 101,
    _SC_XOPEN_XPG4 = 100,
    _SC_XOPEN_XPG3 = 99,
    _SC_XOPEN_XPG2 = 98,
    _SC_2_UPE = 97,
    _SC_2_C_VERSION = 96,
    _SC_2_CHAR_TERM = 95,
    _SC_XOPEN_SHM = 94,
    _SC_XOPEN_ENH_I18N = 93,
    _SC_XOPEN_CRYPT = 92,
    _SC_XOPEN_UNIX = 91,
    _SC_XOPEN_XCU_VERSION = 90,
    _SC_XOPEN_VERSION = 89,
    _SC_PASS_MAX = 88,
    _SC_ATEXIT_MAX = 87,
    _SC_AVPHYS_PAGES = 86,
    _SC_PHYS_PAGES = 85,
    _SC_NPROCESSORS_ONLN = 84,
    _SC_NPROCESSORS_CONF = 83,
    _SC_THREAD_PROCESS_SHARED = 82,
    _SC_THREAD_PRIO_PROTECT = 81,
    _SC_THREAD_PRIO_INHERIT = 80,
    _SC_THREAD_PRIORITY_SCHEDULING = 79,
    _SC_THREAD_ATTR_STACKSIZE = 78,
    _SC_THREAD_ATTR_STACKADDR = 77,
    _SC_THREAD_THREADS_MAX = 76,
    _SC_THREAD_STACK_MIN = 75,
    _SC_THREAD_KEYS_MAX = 74,
    _SC_THREAD_DESTRUCTOR_ITERATIONS = 73,
    _SC_TTY_NAME_MAX = 72,
    _SC_LOGIN_NAME_MAX = 71,
    _SC_GETPW_R_SIZE_MAX = 70,
    _SC_GETGR_R_SIZE_MAX = 69,
    _SC_THREAD_SAFE_FUNCTIONS = 68,
    _SC_THREADS = 67,
    _SC_T_IOV_MAX = 66,
    _SC_PII_OSI_M = 65,
    _SC_PII_OSI_CLTS = 64,
    _SC_PII_OSI_COTS = 63,
    _SC_PII_INTERNET_DGRAM = 62,
    _SC_PII_INTERNET_STREAM = 61,
    _SC_IOV_MAX = 60,
    _SC_UIO_MAXIOV = 60,
    _SC_SELECT = 59,
    _SC_POLL = 58,
    _SC_PII_OSI = 57,
    _SC_PII_INTERNET = 56,
    _SC_PII_SOCKET = 55,
    _SC_PII_XTI = 54,
    _SC_PII = 53,
    _SC_2_LOCALEDEF = 52,
    _SC_2_SW_DEV = 51,
    _SC_2_FORT_RUN = 50,
    _SC_2_FORT_DEV = 49,
    _SC_2_C_DEV = 48,
    _SC_2_C_BIND = 47,
    _SC_2_VERSION = 46,
    _SC_CHARCLASS_NAME_MAX = 45,
    _SC_RE_DUP_MAX = 44,
    _SC_LINE_MAX = 43,
    _SC_EXPR_NEST_MAX = 42,
    _SC_EQUIV_CLASS_MAX = 41,
    _SC_COLL_WEIGHTS_MAX = 40,
    _SC_BC_STRING_MAX = 39,
    _SC_BC_SCALE_MAX = 38,
    _SC_BC_DIM_MAX = 37,
    _SC_BC_BASE_MAX = 36,
    _SC_TIMER_MAX = 35,
    _SC_SIGQUEUE_MAX = 34,
    _SC_SEM_VALUE_MAX = 33,
    _SC_SEM_NSEMS_MAX = 32,
    _SC_RTSIG_MAX = 31,
    _SC_PAGESIZE = 30,
    _SC_VERSION = 29,
    _SC_MQ_PRIO_MAX = 28,
    _SC_MQ_OPEN_MAX = 27,
    _SC_DELAYTIMER_MAX = 26,
    _SC_AIO_PRIO_DELTA_MAX = 25,
    _SC_AIO_MAX = 24,
    _SC_AIO_LISTIO_MAX = 23,
    _SC_SHARED_MEMORY_OBJECTS = 22,
    _SC_SEMAPHORES = 21,
    _SC_MESSAGE_PASSING = 20,
    _SC_MEMORY_PROTECTION = 19,
    _SC_MEMLOCK_RANGE = 18,
    _SC_MEMLOCK = 17,
    _SC_MAPPED_FILES = 16,
    _SC_FSYNC = 15,
    _SC_SYNCHRONIZED_IO = 14,
    _SC_PRIORITIZED_IO = 13,
    _SC_ASYNCHRONOUS_IO = 12,
    _SC_TIMERS = 11,
    _SC_PRIORITY_SCHEDULING = 10,
    _SC_REALTIME_SIGNALS = 9,
    _SC_SAVED_IDS = 8,
    _SC_JOB_CONTROL = 7,
    _SC_TZNAME_MAX = 6,
    _SC_STREAM_MAX = 5,
    _SC_OPEN_MAX = 4,
    _SC_NGROUPS_MAX = 3,
    _SC_CLK_TCK = 2,
    _SC_CHILD_MAX = 1,
    _SC_ARG_MAX = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::_SC_THREAD_ROBUST_PRIO_PROTECT => 248,
            C2RustUnnamed::_SC_THREAD_ROBUST_PRIO_INHERIT => 247,
            C2RustUnnamed::_SC_XOPEN_STREAMS => 246,
            C2RustUnnamed::_SC_TRACE_USER_EVENT_MAX => 245,
            C2RustUnnamed::_SC_TRACE_SYS_MAX => 244,
            C2RustUnnamed::_SC_TRACE_NAME_MAX => 243,
            C2RustUnnamed::_SC_TRACE_EVENT_NAME_MAX => 242,
            C2RustUnnamed::_SC_SS_REPL_MAX => 241,
            C2RustUnnamed::_SC_V7_LPBIG_OFFBIG => 240,
            C2RustUnnamed::_SC_V7_LP64_OFF64 => 239,
            C2RustUnnamed::_SC_V7_ILP32_OFFBIG => 238,
            C2RustUnnamed::_SC_V7_ILP32_OFF32 => 237,
            C2RustUnnamed::_SC_RAW_SOCKETS => 236,
            C2RustUnnamed::_SC_IPV6 => 235,
            C2RustUnnamed::_SC_LEVEL4_CACHE_LINESIZE => 199,
            C2RustUnnamed::_SC_LEVEL4_CACHE_ASSOC => 198,
            C2RustUnnamed::_SC_LEVEL4_CACHE_SIZE => 197,
            C2RustUnnamed::_SC_LEVEL3_CACHE_LINESIZE => 196,
            C2RustUnnamed::_SC_LEVEL3_CACHE_ASSOC => 195,
            C2RustUnnamed::_SC_LEVEL3_CACHE_SIZE => 194,
            C2RustUnnamed::_SC_LEVEL2_CACHE_LINESIZE => 193,
            C2RustUnnamed::_SC_LEVEL2_CACHE_ASSOC => 192,
            C2RustUnnamed::_SC_LEVEL2_CACHE_SIZE => 191,
            C2RustUnnamed::_SC_LEVEL1_DCACHE_LINESIZE => 190,
            C2RustUnnamed::_SC_LEVEL1_DCACHE_ASSOC => 189,
            C2RustUnnamed::_SC_LEVEL1_DCACHE_SIZE => 188,
            C2RustUnnamed::_SC_LEVEL1_ICACHE_LINESIZE => 187,
            C2RustUnnamed::_SC_LEVEL1_ICACHE_ASSOC => 186,
            C2RustUnnamed::_SC_LEVEL1_ICACHE_SIZE => 185,
            C2RustUnnamed::_SC_TRACE_LOG => 184,
            C2RustUnnamed::_SC_TRACE_INHERIT => 183,
            C2RustUnnamed::_SC_TRACE_EVENT_FILTER => 182,
            C2RustUnnamed::_SC_TRACE => 181,
            C2RustUnnamed::_SC_HOST_NAME_MAX => 180,
            C2RustUnnamed::_SC_V6_LPBIG_OFFBIG => 179,
            C2RustUnnamed::_SC_V6_LP64_OFF64 => 178,
            C2RustUnnamed::_SC_V6_ILP32_OFFBIG => 177,
            C2RustUnnamed::_SC_V6_ILP32_OFF32 => 176,
            C2RustUnnamed::_SC_2_PBS_CHECKPOINT => 175,
            C2RustUnnamed::_SC_STREAMS => 174,
            C2RustUnnamed::_SC_SYMLOOP_MAX => 173,
            C2RustUnnamed::_SC_2_PBS_TRACK => 172,
            C2RustUnnamed::_SC_2_PBS_MESSAGE => 171,
            C2RustUnnamed::_SC_2_PBS_LOCATE => 170,
            C2RustUnnamed::_SC_2_PBS_ACCOUNTING => 169,
            C2RustUnnamed::_SC_2_PBS => 168,
            C2RustUnnamed::_SC_USER_GROUPS_R => 167,
            C2RustUnnamed::_SC_USER_GROUPS => 166,
            C2RustUnnamed::_SC_TYPED_MEMORY_OBJECTS => 165,
            C2RustUnnamed::_SC_TIMEOUTS => 164,
            C2RustUnnamed::_SC_SYSTEM_DATABASE_R => 163,
            C2RustUnnamed::_SC_SYSTEM_DATABASE => 162,
            C2RustUnnamed::_SC_THREAD_SPORADIC_SERVER => 161,
            C2RustUnnamed::_SC_SPORADIC_SERVER => 160,
            C2RustUnnamed::_SC_SPAWN => 159,
            C2RustUnnamed::_SC_SIGNALS => 158,
            C2RustUnnamed::_SC_SHELL => 157,
            C2RustUnnamed::_SC_REGEX_VERSION => 156,
            C2RustUnnamed::_SC_REGEXP => 155,
            C2RustUnnamed::_SC_SPIN_LOCKS => 154,
            C2RustUnnamed::_SC_READER_WRITER_LOCKS => 153,
            C2RustUnnamed::_SC_NETWORKING => 152,
            C2RustUnnamed::_SC_SINGLE_PROCESS => 151,
            C2RustUnnamed::_SC_MULTI_PROCESS => 150,
            C2RustUnnamed::_SC_MONOTONIC_CLOCK => 149,
            C2RustUnnamed::_SC_FILE_SYSTEM => 148,
            C2RustUnnamed::_SC_FILE_LOCKING => 147,
            C2RustUnnamed::_SC_FILE_ATTRIBUTES => 146,
            C2RustUnnamed::_SC_PIPE => 145,
            C2RustUnnamed::_SC_FIFO => 144,
            C2RustUnnamed::_SC_FD_MGMT => 143,
            C2RustUnnamed::_SC_DEVICE_SPECIFIC_R => 142,
            C2RustUnnamed::_SC_DEVICE_SPECIFIC => 141,
            C2RustUnnamed::_SC_DEVICE_IO => 140,
            C2RustUnnamed::_SC_THREAD_CPUTIME => 139,
            C2RustUnnamed::_SC_CPUTIME => 138,
            C2RustUnnamed::_SC_CLOCK_SELECTION => 137,
            C2RustUnnamed::_SC_C_LANG_SUPPORT_R => 136,
            C2RustUnnamed::_SC_C_LANG_SUPPORT => 135,
            C2RustUnnamed::_SC_BASE => 134,
            C2RustUnnamed::_SC_BARRIERS => 133,
            C2RustUnnamed::_SC_ADVISORY_INFO => 132,
            C2RustUnnamed::_SC_XOPEN_REALTIME_THREADS => 131,
            C2RustUnnamed::_SC_XOPEN_REALTIME => 130,
            C2RustUnnamed::_SC_XOPEN_LEGACY => 129,
            C2RustUnnamed::_SC_XBS5_LPBIG_OFFBIG => 128,
            C2RustUnnamed::_SC_XBS5_LP64_OFF64 => 127,
            C2RustUnnamed::_SC_XBS5_ILP32_OFFBIG => 126,
            C2RustUnnamed::_SC_XBS5_ILP32_OFF32 => 125,
            C2RustUnnamed::_SC_NL_TEXTMAX => 124,
            C2RustUnnamed::_SC_NL_SETMAX => 123,
            C2RustUnnamed::_SC_NL_NMAX => 122,
            C2RustUnnamed::_SC_NL_MSGMAX => 121,
            C2RustUnnamed::_SC_NL_LANGMAX => 120,
            C2RustUnnamed::_SC_NL_ARGMAX => 119,
            C2RustUnnamed::_SC_USHRT_MAX => 118,
            C2RustUnnamed::_SC_ULONG_MAX => 117,
            C2RustUnnamed::_SC_UINT_MAX => 116,
            C2RustUnnamed::_SC_UCHAR_MAX => 115,
            C2RustUnnamed::_SC_SHRT_MIN => 114,
            C2RustUnnamed::_SC_SHRT_MAX => 113,
            C2RustUnnamed::_SC_SCHAR_MIN => 112,
            C2RustUnnamed::_SC_SCHAR_MAX => 111,
            C2RustUnnamed::_SC_SSIZE_MAX => 110,
            C2RustUnnamed::_SC_NZERO => 109,
            C2RustUnnamed::_SC_MB_LEN_MAX => 108,
            C2RustUnnamed::_SC_WORD_BIT => 107,
            C2RustUnnamed::_SC_LONG_BIT => 106,
            C2RustUnnamed::_SC_INT_MIN => 105,
            C2RustUnnamed::_SC_INT_MAX => 104,
            C2RustUnnamed::_SC_CHAR_MIN => 103,
            C2RustUnnamed::_SC_CHAR_MAX => 102,
            C2RustUnnamed::_SC_CHAR_BIT => 101,
            C2RustUnnamed::_SC_XOPEN_XPG4 => 100,
            C2RustUnnamed::_SC_XOPEN_XPG3 => 99,
            C2RustUnnamed::_SC_XOPEN_XPG2 => 98,
            C2RustUnnamed::_SC_2_UPE => 97,
            C2RustUnnamed::_SC_2_C_VERSION => 96,
            C2RustUnnamed::_SC_2_CHAR_TERM => 95,
            C2RustUnnamed::_SC_XOPEN_SHM => 94,
            C2RustUnnamed::_SC_XOPEN_ENH_I18N => 93,
            C2RustUnnamed::_SC_XOPEN_CRYPT => 92,
            C2RustUnnamed::_SC_XOPEN_UNIX => 91,
            C2RustUnnamed::_SC_XOPEN_XCU_VERSION => 90,
            C2RustUnnamed::_SC_XOPEN_VERSION => 89,
            C2RustUnnamed::_SC_PASS_MAX => 88,
            C2RustUnnamed::_SC_ATEXIT_MAX => 87,
            C2RustUnnamed::_SC_AVPHYS_PAGES => 86,
            C2RustUnnamed::_SC_PHYS_PAGES => 85,
            C2RustUnnamed::_SC_NPROCESSORS_ONLN => 84,
            C2RustUnnamed::_SC_NPROCESSORS_CONF => 83,
            C2RustUnnamed::_SC_THREAD_PROCESS_SHARED => 82,
            C2RustUnnamed::_SC_THREAD_PRIO_PROTECT => 81,
            C2RustUnnamed::_SC_THREAD_PRIO_INHERIT => 80,
            C2RustUnnamed::_SC_THREAD_PRIORITY_SCHEDULING => 79,
            C2RustUnnamed::_SC_THREAD_ATTR_STACKSIZE => 78,
            C2RustUnnamed::_SC_THREAD_ATTR_STACKADDR => 77,
            C2RustUnnamed::_SC_THREAD_THREADS_MAX => 76,
            C2RustUnnamed::_SC_THREAD_STACK_MIN => 75,
            C2RustUnnamed::_SC_THREAD_KEYS_MAX => 74,
            C2RustUnnamed::_SC_THREAD_DESTRUCTOR_ITERATIONS => 73,
            C2RustUnnamed::_SC_TTY_NAME_MAX => 72,
            C2RustUnnamed::_SC_LOGIN_NAME_MAX => 71,
            C2RustUnnamed::_SC_GETPW_R_SIZE_MAX => 70,
            C2RustUnnamed::_SC_GETGR_R_SIZE_MAX => 69,
            C2RustUnnamed::_SC_THREAD_SAFE_FUNCTIONS => 68,
            C2RustUnnamed::_SC_THREADS => 67,
            C2RustUnnamed::_SC_T_IOV_MAX => 66,
            C2RustUnnamed::_SC_PII_OSI_M => 65,
            C2RustUnnamed::_SC_PII_OSI_CLTS => 64,
            C2RustUnnamed::_SC_PII_OSI_COTS => 63,
            C2RustUnnamed::_SC_PII_INTERNET_DGRAM => 62,
            C2RustUnnamed::_SC_PII_INTERNET_STREAM => 61,
            C2RustUnnamed::_SC_IOV_MAX => 60,
            C2RustUnnamed::_SC_UIO_MAXIOV => 60,
            C2RustUnnamed::_SC_SELECT => 59,
            C2RustUnnamed::_SC_POLL => 58,
            C2RustUnnamed::_SC_PII_OSI => 57,
            C2RustUnnamed::_SC_PII_INTERNET => 56,
            C2RustUnnamed::_SC_PII_SOCKET => 55,
            C2RustUnnamed::_SC_PII_XTI => 54,
            C2RustUnnamed::_SC_PII => 53,
            C2RustUnnamed::_SC_2_LOCALEDEF => 52,
            C2RustUnnamed::_SC_2_SW_DEV => 51,
            C2RustUnnamed::_SC_2_FORT_RUN => 50,
            C2RustUnnamed::_SC_2_FORT_DEV => 49,
            C2RustUnnamed::_SC_2_C_DEV => 48,
            C2RustUnnamed::_SC_2_C_BIND => 47,
            C2RustUnnamed::_SC_2_VERSION => 46,
            C2RustUnnamed::_SC_CHARCLASS_NAME_MAX => 45,
            C2RustUnnamed::_SC_RE_DUP_MAX => 44,
            C2RustUnnamed::_SC_LINE_MAX => 43,
            C2RustUnnamed::_SC_EXPR_NEST_MAX => 42,
            C2RustUnnamed::_SC_EQUIV_CLASS_MAX => 41,
            C2RustUnnamed::_SC_COLL_WEIGHTS_MAX => 40,
            C2RustUnnamed::_SC_BC_STRING_MAX => 39,
            C2RustUnnamed::_SC_BC_SCALE_MAX => 38,
            C2RustUnnamed::_SC_BC_DIM_MAX => 37,
            C2RustUnnamed::_SC_BC_BASE_MAX => 36,
            C2RustUnnamed::_SC_TIMER_MAX => 35,
            C2RustUnnamed::_SC_SIGQUEUE_MAX => 34,
            C2RustUnnamed::_SC_SEM_VALUE_MAX => 33,
            C2RustUnnamed::_SC_SEM_NSEMS_MAX => 32,
            C2RustUnnamed::_SC_RTSIG_MAX => 31,
            C2RustUnnamed::_SC_PAGESIZE => 30,
            C2RustUnnamed::_SC_VERSION => 29,
            C2RustUnnamed::_SC_MQ_PRIO_MAX => 28,
            C2RustUnnamed::_SC_MQ_OPEN_MAX => 27,
            C2RustUnnamed::_SC_DELAYTIMER_MAX => 26,
            C2RustUnnamed::_SC_AIO_PRIO_DELTA_MAX => 25,
            C2RustUnnamed::_SC_AIO_MAX => 24,
            C2RustUnnamed::_SC_AIO_LISTIO_MAX => 23,
            C2RustUnnamed::_SC_SHARED_MEMORY_OBJECTS => 22,
            C2RustUnnamed::_SC_SEMAPHORES => 21,
            C2RustUnnamed::_SC_MESSAGE_PASSING => 20,
            C2RustUnnamed::_SC_MEMORY_PROTECTION => 19,
            C2RustUnnamed::_SC_MEMLOCK_RANGE => 18,
            C2RustUnnamed::_SC_MEMLOCK => 17,
            C2RustUnnamed::_SC_MAPPED_FILES => 16,
            C2RustUnnamed::_SC_FSYNC => 15,
            C2RustUnnamed::_SC_SYNCHRONIZED_IO => 14,
            C2RustUnnamed::_SC_PRIORITIZED_IO => 13,
            C2RustUnnamed::_SC_ASYNCHRONOUS_IO => 12,
            C2RustUnnamed::_SC_TIMERS => 11,
            C2RustUnnamed::_SC_PRIORITY_SCHEDULING => 10,
            C2RustUnnamed::_SC_REALTIME_SIGNALS => 9,
            C2RustUnnamed::_SC_SAVED_IDS => 8,
            C2RustUnnamed::_SC_JOB_CONTROL => 7,
            C2RustUnnamed::_SC_TZNAME_MAX => 6,
            C2RustUnnamed::_SC_STREAM_MAX => 5,
            C2RustUnnamed::_SC_OPEN_MAX => 4,
            C2RustUnnamed::_SC_NGROUPS_MAX => 3,
            C2RustUnnamed::_SC_CLK_TCK => 2,
            C2RustUnnamed::_SC_CHILD_MAX => 1,
            C2RustUnnamed::_SC_ARG_MAX => 0,
        }
    }
}

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptimer {
    pub start: ptimer_system_time,
    pub elapsed_last: libc::c_double,
    pub elapsed_pre_start: libc::c_double,
}
pub type ptimer_system_time = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub id: libc::c_int,
    pub sysconf_name: libc::c_int,
}
static mut posix_clock_id: libc::c_int = 0;
static mut posix_clock_resolution: libc::c_double = 0.;
unsafe extern "C" fn posix_init() {
    static mut clocks: [C2RustUnnamed_0; 2] = [
        {
            let mut init = C2RustUnnamed_0 {
                id: 1 as libc::c_int,
                sysconf_name: _SC_MONOTONIC_CLOCK as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                id: 0 as libc::c_int,
                sysconf_name: -(1 as libc::c_int),
            };
            init
        },
    ];
    let mut i: size_t = 0;
    let mut current_block_4: u64;
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_0; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        if clocks[i as usize].sysconf_name != -(1 as libc::c_int) {
            if sysconf(clocks[i as usize].sysconf_name)
                < 0 as libc::c_int as libc::c_long
            {
                current_block_4 = 16668937799742929182;
            } else {
                current_block_4 = 735147466149431745;
            }
        } else {
            current_block_4 = 735147466149431745;
        }
        match current_block_4 {
            735147466149431745 => {
                if !(clock_getres(clocks[i as usize].id, &mut r) < 0 as libc::c_int) {
                    posix_clock_id = clocks[i as usize].id;
                    posix_clock_resolution = r.tv_sec as libc::c_double
                        + r.tv_nsec as libc::c_double / 1e9f64;
                    if posix_clock_resolution == 0 as libc::c_int as libc::c_double {
                        posix_clock_resolution = 1e-3f64;
                    }
                    break;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    if i
        == (::core::mem::size_of::<[C2RustUnnamed_0; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot get REALTIME clock frequency: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        posix_clock_id = 0 as libc::c_int;
        posix_clock_resolution = 1e-3f64;
    }
}
#[inline]
unsafe extern "C" fn posix_measure(mut pst: *mut ptimer_system_time) {
    clock_gettime(posix_clock_id, pst);
}
#[inline]
unsafe extern "C" fn posix_diff(
    mut pst1: *mut ptimer_system_time,
    mut pst2: *mut ptimer_system_time,
) -> libc::c_double {
    return ((*pst1).tv_sec - (*pst2).tv_sec) as libc::c_double
        + ((*pst1).tv_nsec - (*pst2).tv_nsec) as libc::c_double / 1e9f64;
}
#[inline]
unsafe extern "C" fn posix_resolution() -> libc::c_double {
    return posix_clock_resolution;
}
#[no_mangle]
pub unsafe extern "C" fn ptimer_new() -> *mut ptimer {
    let mut pt: *mut ptimer = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<ptimer>() as libc::c_ulong,
    ) as *mut ptimer;
    static mut init_done: bool = false;
    if !init_done {
        init_done = 1 as libc::c_int != 0;
        posix_init();
    }
    ptimer_reset(pt);
    return pt;
}
#[no_mangle]
pub unsafe extern "C" fn ptimer_destroy(mut pt: *mut ptimer) {
    rpl_free(pt as *mut libc::c_void);
    pt = 0 as *mut ptimer;
}
#[no_mangle]
pub unsafe extern "C" fn ptimer_reset(mut pt: *mut ptimer) {
    posix_measure(&mut (*pt).start);
    (*pt).elapsed_last = 0 as libc::c_int as libc::c_double;
    (*pt).elapsed_pre_start = 0 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn ptimer_measure(mut pt: *mut ptimer) -> libc::c_double {
    let mut now: ptimer_system_time = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut elapsed: libc::c_double = 0.;
    posix_measure(&mut now);
    elapsed = (*pt).elapsed_pre_start + posix_diff(&mut now, &mut (*pt).start);
    if elapsed < (*pt).elapsed_last {
        (*pt).start = now;
        (*pt).elapsed_pre_start = (*pt).elapsed_last;
        elapsed = (*pt).elapsed_last;
    }
    (*pt).elapsed_last = elapsed;
    return elapsed;
}
#[no_mangle]
pub unsafe extern "C" fn ptimer_read(mut pt: *const ptimer) -> libc::c_double {
    return (*pt).elapsed_last;
}
#[no_mangle]
pub unsafe extern "C" fn ptimer_resolution() -> libc::c_double {
    return posix_resolution();
}
