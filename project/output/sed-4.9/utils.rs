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
    _SC_THREAD_ROBUST_PRIO_PROTECT,
    _SC_THREAD_ROBUST_PRIO_INHERIT,
    _SC_XOPEN_STREAMS,
    _SC_TRACE_USER_EVENT_MAX,
    _SC_TRACE_SYS_MAX,
    _SC_TRACE_NAME_MAX,
    _SC_TRACE_EVENT_NAME_MAX,
    _SC_SS_REPL_MAX,
    _SC_V7_LPBIG_OFFBIG,
    _SC_V7_LP64_OFF64,
    _SC_V7_ILP32_OFFBIG,
    _SC_V7_ILP32_OFF32,
    _SC_RAW_SOCKETS,
    _SC_IPV6,
    _SC_LEVEL4_CACHE_LINESIZE,
    _SC_LEVEL4_CACHE_ASSOC,
    _SC_LEVEL4_CACHE_SIZE,
    _SC_LEVEL3_CACHE_LINESIZE,
    _SC_LEVEL3_CACHE_ASSOC,
    _SC_LEVEL3_CACHE_SIZE,
    _SC_LEVEL2_CACHE_LINESIZE,
    _SC_LEVEL2_CACHE_ASSOC,
    _SC_LEVEL2_CACHE_SIZE,
    _SC_LEVEL1_DCACHE_LINESIZE,
    _SC_LEVEL1_DCACHE_ASSOC,
    _SC_LEVEL1_DCACHE_SIZE,
    _SC_LEVEL1_ICACHE_LINESIZE,
    _SC_LEVEL1_ICACHE_ASSOC,
    _SC_LEVEL1_ICACHE_SIZE,
    _SC_TRACE_LOG,
    _SC_TRACE_INHERIT,
    _SC_TRACE_EVENT_FILTER,
    _SC_TRACE,
    _SC_HOST_NAME_MAX,
    _SC_V6_LPBIG_OFFBIG,
    _SC_V6_LP64_OFF64,
    _SC_V6_ILP32_OFFBIG,
    _SC_V6_ILP32_OFF32,
    _SC_2_PBS_CHECKPOINT,
    _SC_STREAMS,
    _SC_SYMLOOP_MAX,
    _SC_2_PBS_TRACK,
    _SC_2_PBS_MESSAGE,
    _SC_2_PBS_LOCATE,
    _SC_2_PBS_ACCOUNTING,
    _SC_2_PBS,
    _SC_USER_GROUPS_R,
    _SC_USER_GROUPS,
    _SC_TYPED_MEMORY_OBJECTS,
    _SC_TIMEOUTS,
    _SC_SYSTEM_DATABASE_R,
    _SC_SYSTEM_DATABASE,
    _SC_THREAD_SPORADIC_SERVER,
    _SC_SPORADIC_SERVER,
    _SC_SPAWN,
    _SC_SIGNALS,
    _SC_SHELL,
    _SC_REGEX_VERSION,
    _SC_REGEXP,
    _SC_SPIN_LOCKS,
    _SC_READER_WRITER_LOCKS,
    _SC_NETWORKING,
    _SC_SINGLE_PROCESS,
    _SC_MULTI_PROCESS,
    _SC_MONOTONIC_CLOCK,
    _SC_FILE_SYSTEM,
    _SC_FILE_LOCKING,
    _SC_FILE_ATTRIBUTES,
    _SC_PIPE,
    _SC_FIFO,
    _SC_FD_MGMT,
    _SC_DEVICE_SPECIFIC_R,
    _SC_DEVICE_SPECIFIC,
    _SC_DEVICE_IO,
    _SC_THREAD_CPUTIME,
    _SC_CPUTIME,
    _SC_CLOCK_SELECTION,
    _SC_C_LANG_SUPPORT_R,
    _SC_C_LANG_SUPPORT,
    _SC_BASE,
    _SC_BARRIERS,
    _SC_ADVISORY_INFO,
    _SC_XOPEN_REALTIME_THREADS,
    _SC_XOPEN_REALTIME,
    _SC_XOPEN_LEGACY,
    _SC_XBS5_LPBIG_OFFBIG,
    _SC_XBS5_LP64_OFF64,
    _SC_XBS5_ILP32_OFFBIG,
    _SC_XBS5_ILP32_OFF32,
    _SC_NL_TEXTMAX,
    _SC_NL_SETMAX,
    _SC_NL_NMAX,
    _SC_NL_MSGMAX,
    _SC_NL_LANGMAX,
    _SC_NL_ARGMAX,
    _SC_USHRT_MAX,
    _SC_ULONG_MAX,
    _SC_UINT_MAX,
    _SC_UCHAR_MAX,
    _SC_SHRT_MIN,
    _SC_SHRT_MAX,
    _SC_SCHAR_MIN,
    _SC_SCHAR_MAX,
    _SC_SSIZE_MAX,
    _SC_NZERO,
    _SC_MB_LEN_MAX,
    _SC_WORD_BIT,
    _SC_LONG_BIT,
    _SC_INT_MIN,
    _SC_INT_MAX,
    _SC_CHAR_MIN,
    _SC_CHAR_MAX,
    _SC_CHAR_BIT,
    _SC_XOPEN_XPG4,
    _SC_XOPEN_XPG3,
    _SC_XOPEN_XPG2,
    _SC_2_UPE,
    _SC_2_C_VERSION,
    _SC_2_CHAR_TERM,
    _SC_XOPEN_SHM,
    _SC_XOPEN_ENH_I18N,
    _SC_XOPEN_CRYPT,
    _SC_XOPEN_UNIX,
    _SC_XOPEN_XCU_VERSION,
    _SC_XOPEN_VERSION,
    _SC_PASS_MAX,
    _SC_ATEXIT_MAX,
    _SC_AVPHYS_PAGES,
    _SC_PHYS_PAGES,
    _SC_NPROCESSORS_ONLN,
    _SC_NPROCESSORS_CONF,
    _SC_THREAD_PROCESS_SHARED,
    _SC_THREAD_PRIO_PROTECT,
    _SC_THREAD_PRIO_INHERIT,
    _SC_THREAD_PRIORITY_SCHEDULING,
    _SC_THREAD_ATTR_STACKSIZE,
    _SC_THREAD_ATTR_STACKADDR,
    _SC_THREAD_THREADS_MAX,
    _SC_THREAD_STACK_MIN,
    _SC_THREAD_KEYS_MAX,
    _SC_THREAD_DESTRUCTOR_ITERATIONS,
    _SC_TTY_NAME_MAX,
    _SC_LOGIN_NAME_MAX,
    _SC_GETPW_R_SIZE_MAX,
    _SC_GETGR_R_SIZE_MAX,
    _SC_THREAD_SAFE_FUNCTIONS,
    _SC_THREADS,
    _SC_T_IOV_MAX,
    _SC_PII_OSI_M,
    _SC_PII_OSI_CLTS,
    _SC_PII_OSI_COTS,
    _SC_PII_INTERNET_DGRAM,
    _SC_PII_INTERNET_STREAM,
    _SC_IOV_MAX,
    _SC_UIO_MAXIOV,
    _SC_SELECT,
    _SC_POLL,
    _SC_PII_OSI,
    _SC_PII_INTERNET,
    _SC_PII_SOCKET,
    _SC_PII_XTI,
    _SC_PII,
    _SC_2_LOCALEDEF,
    _SC_2_SW_DEV,
    _SC_2_FORT_RUN,
    _SC_2_FORT_DEV,
    _SC_2_C_DEV,
    _SC_2_C_BIND,
    _SC_2_VERSION,
    _SC_CHARCLASS_NAME_MAX,
    _SC_RE_DUP_MAX,
    _SC_LINE_MAX,
    _SC_EXPR_NEST_MAX,
    _SC_EQUIV_CLASS_MAX,
    _SC_COLL_WEIGHTS_MAX,
    _SC_BC_STRING_MAX,
    _SC_BC_SCALE_MAX,
    _SC_BC_DIM_MAX,
    _SC_BC_BASE_MAX,
    _SC_TIMER_MAX,
    _SC_SIGQUEUE_MAX,
    _SC_SEM_VALUE_MAX,
    _SC_SEM_NSEMS_MAX,
    _SC_RTSIG_MAX,
    _SC_PAGESIZE,
    _SC_VERSION,
    _SC_MQ_PRIO_MAX,
    _SC_MQ_OPEN_MAX,
    _SC_DELAYTIMER_MAX,
    _SC_AIO_PRIO_DELTA_MAX,
    _SC_AIO_MAX,
    _SC_AIO_LISTIO_MAX,
    _SC_SHARED_MEMORY_OBJECTS,
    _SC_SEMAPHORES,
    _SC_MESSAGE_PASSING,
    _SC_MEMORY_PROTECTION,
    _SC_MEMLOCK_RANGE,
    _SC_MEMLOCK,
    _SC_MAPPED_FILES,
    _SC_FSYNC,
    _SC_SYNCHRONIZED_IO,
    _SC_PRIORITIZED_IO,
    _SC_ASYNCHRONOUS_IO,
    _SC_TIMERS,
    _SC_PRIORITY_SCHEDULING,
    _SC_REALTIME_SIGNALS,
    _SC_SAVED_IDS,
    _SC_JOB_CONTROL,
    _SC_TZNAME_MAX,
    _SC_STREAM_MAX,
    _SC_OPEN_MAX,
    _SC_NGROUPS_MAX,
    _SC_CLK_TCK,
    _SC_CHILD_MAX,
    _SC_ARG_MAX,
}  // end of enum

pub type idx_t = ptrdiff_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum exit_codes {
    EXIT_PANIC,
    EXIT_BAD_INPUT,
    EXIT_BAD_USAGE,
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
