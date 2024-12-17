#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
extern "C" {
    fn __uflow(_: *mut _IO_FILE) -> libc::c_int;
    fn __overflow(_: *mut _IO_FILE, _: libc::c_int) -> libc::c_int;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
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
    fn __errno_location() -> *mut libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rpl_free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut program_name: *const libc::c_char;
    fn __fwriting(__fp: *mut FILE) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = libc::c_long;
pub type mode_t = __mode_t;
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
}  // end of enum

pub type idx_t = ptrdiff_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum exit_codes {
    EXIT_PANIC = 4,
    EXIT_BAD_INPUT = 2,
    EXIT_BAD_USAGE = 1,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_file {
    pub fp: *mut FILE,
    pub name: *mut libc::c_char,
    pub link: *mut open_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub allocated: size_t,
    pub length: size_t,
    pub b: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __eloop_threshold() -> libc::c_uint {
    static mut sysconf_symloop_max: libc::c_long = 0;
    if sysconf_symloop_max == 0 as libc::c_int as libc::c_long {
        sysconf_symloop_max = sysconf(_SC_SYMLOOP_MAX as libc::c_int);
    }
    let symloop_max: libc::c_uint = (if sysconf_symloop_max
        <= 0 as libc::c_int as libc::c_long
    {
        8 as libc::c_int as libc::c_long
    } else {
        sysconf_symloop_max
    }) as libc::c_uint;
    return if symloop_max > 40 as libc::c_int as libc::c_uint {
        symloop_max
    } else {
        40 as libc::c_int as libc::c_uint
    };
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
pub unsafe extern "C" fn panic(mut str: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program_name);
    ap = args.clone();
    vfprintf(stderr, str, ap.as_va_list());
    putc_unlocked('\n' as i32, stderr);
    exit(EXIT_PANIC as libc::c_int);
}
unsafe extern "C" fn utils_fp_name(mut fp: *mut FILE) -> *const libc::c_char {
    let mut p: *mut open_file = 0 as *mut open_file;
    p = open_files;
    while !p.is_null() {
        if (*p).fp == fp {
            return (*p).name;
        }
        p = (*p).link;
    }
    if fp == stdin {
        return b"stdin\0" as *const u8 as *const libc::c_char
    } else if fp == stdout {
        return b"stdout\0" as *const u8 as *const libc::c_char
    } else if fp == stderr {
        return b"stderr\0" as *const u8 as *const libc::c_char
    }
    return b"<unknown>\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn register_open_file(
    mut fp: *mut FILE,
    mut name: *const libc::c_char,
) {
    let mut p: *mut open_file = 0 as *mut open_file;
    p = xmalloc(::core::mem::size_of::<open_file>() as libc::c_ulong) as *mut open_file;
    (*p).link = open_files;
    open_files = p;
    (*p).name = xstrdup(name);
    (*p).fp = fp;
}
#[no_mangle]
pub unsafe extern "C" fn ck_fopen(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut fail: libc::c_int,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, mode);
    if fp.is_null() {
        if fail != 0 {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't open file %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
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
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut fail: libc::c_int,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fdopen(fd, mode);
    if fp.is_null() {
        if fail != 0 {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't attach to %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
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
static mut G_file_to_unlink: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn remove_cleanup_file() {
    if !G_file_to_unlink.is_null() {
        unlink(G_file_to_unlink);
    }
}
unsafe extern "C" fn register_cleanup_file(mut file: *const libc::c_char) {
    G_file_to_unlink = file;
}
#[no_mangle]
pub unsafe extern "C" fn cancel_cleanup() {
    G_file_to_unlink = 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ck_mkstemp(
    mut p_filename: *mut *mut libc::c_char,
    mut tmpdir: *const libc::c_char,
    mut base: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut template: *mut libc::c_char = xmalloc(
        (strlen(tmpdir))
            .wrapping_add(strlen(base))
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        template,
        b"%s/%sXXXXXX\0" as *const u8 as *const libc::c_char,
        tmpdir,
        base,
    );
    let mut save_umask: mode_t = umask(0o77 as libc::c_int as __mode_t);
    let mut fd: libc::c_int = mkostemp(template, 0 as libc::c_int);
    let mut err: libc::c_int = *__errno_location();
    umask(save_umask);
    let mut fp: *mut FILE = 0 as *mut FILE;
    if 0 as libc::c_int <= fd {
        *p_filename = template;
        register_cleanup_file(template);
        fp = fdopen(fd, mode);
        err = *__errno_location();
    }
    if fp.is_null() {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't open temporary file %s: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
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
        && (if 0 != 0 && 0 != 0
            && size.wrapping_mul(nmemb) <= 8 as libc::c_int as libc::c_ulong
            && size != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = ptr as *const libc::c_char;
                let mut __stream: *mut FILE = stream;
                let mut __cnt: size_t = 0;
                __cnt = size.wrapping_mul(nmemb);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                        as libc::c_int as libc::c_long != 0
                    {
                        let fresh1 = __ptr;
                        __ptr = __ptr.offset(1);
                        __overflow(__stream, *fresh1 as libc::c_uchar as libc::c_int)
                    } else {
                        let fresh2 = __ptr;
                        __ptr = __ptr.offset(1);
                        let fresh3 = (*__stream)._IO_write_ptr;
                        (*__stream)
                            ._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
                        *fresh3 = *fresh2;
                        *fresh3 as libc::c_uchar as libc::c_int
                    }) == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                size.wrapping_mul(nmemb).wrapping_sub(__cnt).wrapping_div(size)
            })
        } else {
            (if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && nmemb == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(ptr, size, nmemb, stream)
            })
        }) != nmemb
    {
        panic(
            dcngettext(
                0 as *const libc::c_char,
                b"couldn't write %llu item to %s: %s\0" as *const u8
                    as *const libc::c_char,
                b"couldn't write %llu items to %s: %s\0" as *const u8
                    as *const libc::c_char,
                nmemb,
                5 as libc::c_int,
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
            nmemb = (if 0 != 0 && 0 != 0
                && size.wrapping_mul(nmemb) <= 8 as libc::c_int as libc::c_ulong
                && size != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *mut libc::c_char = ptr as *mut libc::c_char;
                    let mut __stream: *mut FILE = stream;
                    let mut __cnt: size_t = 0;
                    __cnt = size.wrapping_mul(nmemb);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let mut __c: libc::c_int = (if ((*__stream)._IO_read_ptr
                            >= (*__stream)._IO_read_end) as libc::c_int as libc::c_long
                            != 0
                        {
                            __uflow(__stream)
                        } else {
                            let fresh4 = (*__stream)._IO_read_ptr;
                            (*__stream)
                                ._IO_read_ptr = ((*__stream)._IO_read_ptr).offset(1);
                            *(fresh4 as *mut libc::c_uchar) as libc::c_int
                        });
                        if __c == -(1 as libc::c_int) {
                            break;
                        }
                        let fresh5 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh5 = __c as libc::c_char;
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    size.wrapping_mul(nmemb).wrapping_sub(__cnt).wrapping_div(size)
                })
            } else {
                (if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0 && nmemb == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int as size_t
                } else {
                    fread_unlocked(ptr, size, nmemb, stream)
                })
            });
            nmemb <= 0 as libc::c_int as libc::c_ulong
        } && ferror_unlocked(stream) != 0
    {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"read error on %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
    return nmemb;
}
#[no_mangle]
pub unsafe extern "C" fn ck_getdelim(
    mut text: *mut *mut libc::c_char,
    mut buflen: *mut size_t,
    mut delim: libc::c_char,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: ssize_t = 0;
    let mut error: bool = false;
    error = ferror_unlocked(stream) != 0;
    if !error {
        result = getdelim(text, buflen, delim as libc::c_int, stream);
        error = ferror_unlocked(stream) != 0;
    }
    if error {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"read error on %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
    return result as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ck_fflush(mut stream: *mut FILE) {
    if !(__fwriting(stream) != 0 as libc::c_int) {
        return;
    }
    clearerr_unlocked(stream);
    if fflush_unlocked(stream) == -(1 as libc::c_int)
        && *__errno_location() != 9 as libc::c_int
    {
        panic(
            b"couldn't flush %s: %s\0" as *const u8 as *const libc::c_char,
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
        do_ck_fclose(stdout, b"stdout\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn do_ck_fclose(mut fp: *mut FILE, mut name: *const libc::c_char) {
    ck_fflush(fp);
    clearerr_unlocked(fp);
    if fclose(fp) == -(1 as libc::c_int) {
        panic(
            b"couldn't close %s: %s\0" as *const u8 as *const libc::c_char,
            name,
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn follow_symlink(
    mut fname: *const libc::c_char,
) -> *const libc::c_char {
    let mut fn_0: *const libc::c_char = fname;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buf_size: idx_t = 0;
    let mut buf_used: idx_t = 0 as libc::c_int as idx_t;
    let mut num_links: idx_t = 0 as libc::c_int as idx_t;
    loop {
        let mut linklen: ssize_t = 0;
        let mut newlen: idx_t = 0;
        let mut c: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            linklen = (if buf_used < buf_size {
                readlink(
                    fn_0,
                    buf.offset(buf_used as isize),
                    (buf_size - buf_used) as size_t,
                )
            } else {
                0 as libc::c_int as libc::c_long
            });
            if !(linklen == buf_size) {
                break;
            }
            buf = xpalloc(
                buf as *mut libc::c_void,
                &mut buf_size,
                1 as libc::c_int as idx_t,
                if (9223372036854775807 as libc::c_long)
                    < 9223372036854775807 as libc::c_long
                {
                    9223372036854775807 as libc::c_long
                } else {
                    9223372036854775807 as libc::c_long
                },
                1 as libc::c_int as idx_t,
            ) as *mut libc::c_char;
            if num_links != 0 {
                fn_0 = buf;
            }
        }
        if linklen < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 22 as libc::c_int {
                break;
            }
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't readlink %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fn_0,
                strerror(*__errno_location()),
            );
        }
        if __eloop_threshold() as libc::c_long <= num_links {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't follow symlink %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                strerror(40 as libc::c_int),
            );
        }
        if (linklen == 0 as libc::c_int as libc::c_long
            || *buf.offset(buf_used as isize) as libc::c_int != '/' as i32)
            && {
                c = strrchr(fn_0, '/' as i32);
                !c.is_null()
            }
        {
            let mut dirlen: idx_t = c.offset_from(fn_0) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            newlen = dirlen + linklen;
            if buf_size <= newlen {
                buf = xpalloc(
                    buf as *mut libc::c_void,
                    &mut buf_size,
                    newlen + 1 as libc::c_int as libc::c_long - buf_size,
                    if (9223372036854775807 as libc::c_long)
                        < 9223372036854775807 as libc::c_long
                    {
                        9223372036854775807 as libc::c_long
                    } else {
                        9223372036854775807 as libc::c_long
                    },
                    1 as libc::c_int as idx_t,
                ) as *mut libc::c_char;
                if num_links != 0 {
                    fn_0 = buf;
                }
            }
            memmove(
                buf.offset(dirlen as isize) as *mut libc::c_void,
                buf.offset(buf_used as isize) as *const libc::c_void,
                linklen as libc::c_ulong,
            );
            if fn_0 != buf {
                memcpy(
                    buf as *mut libc::c_void,
                    fn_0 as *const libc::c_void,
                    dirlen as libc::c_ulong,
                );
            }
        } else {
            memmove(
                buf as *mut libc::c_void,
                buf.offset(buf_used as isize) as *const libc::c_void,
                linklen as libc::c_ulong,
            );
            newlen = linklen;
        }
        *buf.offset(newlen as isize) = '\0' as i32 as libc::c_char;
        buf_used = newlen + 1 as libc::c_int as libc::c_long;
        fn_0 = buf;
        num_links += 1;
        num_links;
    }
    return fn_0;
}
#[no_mangle]
pub unsafe extern "C" fn ck_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    let mut rd: libc::c_int = rename(from, to);
    if rd != -(1 as libc::c_int) {
        return;
    }
    panic(
        dcgettext(
            0 as *const libc::c_char,
            b"cannot rename %s: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        from,
        strerror(*__errno_location()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn init_buffer() -> *mut buffer {
    let mut b: *mut buffer = (if ::core::mem::size_of::<buffer>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xzalloc(1 as libc::c_int as size_t)
    } else {
        xcalloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<buffer>() as libc::c_ulong,
        )
    }) as *mut buffer;
    (*b)
        .b = (if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xzalloc(50 as libc::c_int as size_t)
    } else {
        xcalloc(
            50 as libc::c_int as size_t,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    (*b).allocated = 50 as libc::c_int as size_t;
    (*b).length = 0 as libc::c_int as size_t;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn get_buffer(mut b: *const buffer) -> *mut libc::c_char {
    return (*b).b;
}
#[no_mangle]
pub unsafe extern "C" fn size_buffer(mut b: *const buffer) -> size_t {
    return (*b).length;
}
unsafe extern "C" fn resize_buffer(mut b: *mut buffer, mut newlen: size_t) {
    let mut try_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alen: size_t = (*b).allocated;
    if newlen <= alen {
        return;
    }
    alen = (alen as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if newlen < alen {
        try_0 = realloc((*b).b as *mut libc::c_void, alen) as *mut libc::c_char;
    }
    if try_0.is_null() {
        alen = newlen;
        try_0 = xnrealloc(
            (*b).b as *mut libc::c_void,
            alen,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    (*b).allocated = alen;
    (*b).b = try_0;
}
#[no_mangle]
pub unsafe extern "C" fn add_buffer(
    mut b: *mut buffer,
    mut p: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*b).allocated).wrapping_sub((*b).length) < n {
        resize_buffer(b, ((*b).length).wrapping_add(n));
    }
    result = memcpy(
        ((*b).b).offset((*b).length as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        n,
    ) as *mut libc::c_char;
    (*b).length = ((*b).length as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn add1_buffer(
    mut b: *mut buffer,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    if c != -(1 as libc::c_int) {
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        if ((*b).allocated).wrapping_sub((*b).length) < 1 as libc::c_int as libc::c_ulong
        {
            resize_buffer(
                b,
                ((*b).length).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        let fresh6 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        result = ((*b).b).offset(fresh6 as isize);
        *result = c as libc::c_char;
        return result;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn free_buffer(mut b: *mut buffer) {
    if !b.is_null() {
        rpl_free((*b).b as *mut libc::c_void);
    }
    rpl_free(b as *mut libc::c_void);
}
