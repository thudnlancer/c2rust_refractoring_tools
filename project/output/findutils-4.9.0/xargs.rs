#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn close_stdin();
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn pipe_safer(_: *mut libc::c_int) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn bc_size_of_environment() -> size_t;
    fn bc_do_insert(
        ctl: *mut buildcmd_control,
        state: *mut buildcmd_state,
        arg: *mut libc::c_char,
        arglen: size_t,
        prefix: *const libc::c_char,
        pfxlen: size_t,
        linebuf_0: *const libc::c_char,
        lblen: size_t,
        initial_args_0: libc::c_int,
    );
    fn bc_do_exec(ctl: *mut buildcmd_control, state: *mut buildcmd_state);
    fn bc_push_arg(
        ctl: *mut buildcmd_control,
        state: *mut buildcmd_state,
        arg: *const libc::c_char,
        len: size_t,
        prefix: *const libc::c_char,
        pfxlen: size_t,
        initial_args_0: libc::c_int,
    );
    fn bc_init_controlinfo(
        ctl: *mut buildcmd_control,
        arglen_headroom: size_t,
    ) -> BC_INIT_STATUS;
    fn bc_use_sensible_arg_max(ctl: *mut buildcmd_control);
    fn bc_clear_args(ctl: *const buildcmd_control, state: *mut buildcmd_state);
    fn bc_args_exceed_testing_limit(argv: *mut *mut libc::c_char) -> bool;
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const libc::c_char,
    ) -> libc::c_int;
    fn remember_non_cloexec_fds();
    fn complain_about_leaky_fds();
    fn fd_leak_check_is_enabled() -> bool;
    fn open_cloexec(
        path: *const libc::c_char,
        flags: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn display_findutils_version(official_name: *const libc::c_char);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum,
    _ISpunct,
    _IScntrl,
    _ISblank,
    _ISgraph,
    _ISprint,
    _ISspace,
    _ISxdigit,
    _ISdigit,
    _ISalpha,
    _ISlower,
    _ISupper,
}  // end of enum

pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
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
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
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
    pub si_status: libc::c_int,
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
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_style {
    custom_quoting_style,
    clocale_quoting_style,
    locale_quoting_style,
    escape_quoting_style,
    c_maybe_quoting_style,
    c_quoting_style,
    shell_escape_always_quoting_style,
    shell_escape_quoting_style,
    shell_always_quoting_style,
    shell_quoting_style,
    literal_quoting_style,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: size_t,
    pub cmd_argv: *mut *mut libc::c_char,
    pub cmd_argv_alloc: size_t,
    pub argbuf: *mut libc::c_char,
    pub cmd_argv_chars: size_t,
    pub cmd_initial_argv_chars: size_t,
    pub usercontext: *mut libc::c_void,
    pub todo: libc::c_int,
    pub dir_fd: libc::c_int,
    pub largest_successful_arg_count: size_t,
    pub smallest_failed_arg_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: libc::c_int,
    pub posix_arg_size_max: size_t,
    pub posix_arg_size_min: size_t,
    pub arg_max: size_t,
    pub max_arg_count: size_t,
    pub rplen: size_t,
    pub replace_pat: *const libc::c_char,
    pub initial_argc: size_t,
    pub exec_callback: Option::<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut libc::c_void,
            libc::c_int,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub lines_per_exec: libc::c_ulong,
    pub args_per_exec: size_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum BC_INIT_STATUS {
    BC_INIT_CANNOT_ACCOMODATE_HEADROOM,
    BC_INIT_ENV_TOO_BIG,
    BC_INIT_OK,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum LongOptionIdentifier {
    PROCESS_SLOT_VAR,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum XargsStatusValues {
    XARGS_EXIT_COMMAND_NOT_FOUND,
    XARGS_EXIT_COMMAND_CANNOT_BE_RUN,
    XARGS_EXIT_CLIENT_FATAL_SIG,
    XARGS_EXIT_CLIENT_EXIT_255,
    XARGS_EXIT_CLIENT_EXIT_NONZERO,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ClientStatusValues {
    CHILD_EXIT_PLEASE_STOP_IMMEDIATELY,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum read_line_state {
    BACKSLASH,
    QUOTE,
    SPACE,
    NORM,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub _gl_dummy: libc::c_int,
}
pub const XARGS_POSIX_HEADROOM: C2RustUnnamed_36 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub _gl_dummy: libc::c_int,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_36 {
    XARGS_POSIX_HEADROOM,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_36 = libc::c_uint;
static mut input_stream: *mut FILE = 0 as *const FILE as *mut FILE;
static mut linebuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut keep_stdin: libc::c_int = 0 as libc::c_int;
static mut lineno: size_t = 0 as libc::c_int as size_t;
static mut bc_state: buildcmd_state = buildcmd_state {
    cmd_argc: 0,
    cmd_argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    cmd_argv_alloc: 0,
    argbuf: 0 as *const libc::c_char as *mut libc::c_char,
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
    replace_pat: 0 as *const libc::c_char,
    initial_argc: 0,
    exec_callback: None,
    lines_per_exec: 0,
    args_per_exec: 0,
};
static mut nullwarning_given: libc::c_int = 0 as libc::c_int;
static mut eof_str: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut initial_args: bool = 1 as libc::c_int != 0;
static mut proc_max: sig_atomic_t = 1 as libc::c_int;
static mut procs_executed: bool = 0 as libc::c_int != 0;
static mut procs_executing: libc::c_ulong = 0 as libc::c_ulong;
static mut pids: *mut pid_t = 0 as *const pid_t as *mut pid_t;
static mut pids_alloc: size_t = 0 as libc::c_uint as size_t;
static mut parent: pid_t = 0;
static mut stop_waiting: sig_atomic_t = 0 as libc::c_int;
static mut child_error: libc::c_int = 0 as libc::c_int;
static mut original_exit_value: libc::c_int = 0;
static mut open_tty: bool = 0 as libc::c_int != 0;
static mut print_command: bool = 0 as libc::c_int != 0;
static mut query_before_executing: bool = 0 as libc::c_int != 0;
static mut input_delimiter: libc::c_char = '\0' as i32 as libc::c_char;
static mut slot_var_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut longopts: [option; 19] = [
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"arg-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"delimiter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"eof\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"replace\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-args\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"open-tty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-run-if-empty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-limits\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"exit\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-procs\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"process-slot-var\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PROCESS_SLOT_VAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn get_char_oct_or_hex_escape(
    mut s: *const libc::c_char,
) -> libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: libc::c_int = 8 as libc::c_int;
    let mut val: libc::c_ulong = 0;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    if '\\' as i32 == *s.offset(0 as libc::c_int as isize) as libc::c_int {} else {
        __assert_fail(
            b"'\\\\' == s[0]\0" as *const u8 as *const libc::c_char,
            b"xargs.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"char get_char_oct_or_hex_escape(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_10800: {
        if '\\' as i32 == *s.offset(0 as libc::c_int as isize) as libc::c_int {} else {
            __assert_fail(
                b"'\\\\' == s[0]\0" as *const u8 as *const libc::c_char,
                b"xargs.c\0" as *const u8 as *const libc::c_char,
                236 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"char get_char_oct_or_hex_escape(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if 'x' as i32 == *s.offset(1 as libc::c_int as isize) as libc::c_int {
        p = s.offset(2 as libc::c_int as isize);
        base = 16 as libc::c_int;
    } else if *(*__ctype_b_loc())
        .offset(
            *s.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int as isize,
        ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = s.offset(1 as libc::c_int as isize);
        base = 8 as libc::c_int;
    } else {
        p = 0 as *const libc::c_char;
        if ::core::mem::size_of::<C2RustUnnamed_29>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid escape sequence %s in input delimiter specification.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid escape sequence %s in input delimiter specification.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    *__errno_location() = 0 as libc::c_int;
    endp = 0 as *mut libc::c_char;
    val = strtoul(p, &mut endp, base);
    if (9223372036854775807 as libc::c_long as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong) == val
        && 34 as libc::c_int == *__errno_location()
        || val
            > (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
    {
        if 16 as libc::c_int == base {
            if ::core::mem::size_of::<C2RustUnnamed_28>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lx.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    s,
                    (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                        as libc::c_ulong,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lx.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    s,
                    (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                        as libc::c_ulong,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_27>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lo.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    s,
                    (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                        as libc::c_ulong,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lo.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    s,
                    (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                        as libc::c_ulong,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if 0 as libc::c_int != *endp as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid escape sequence %s in input delimiter specification; trailing characters %s not recognised.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
                endp,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid escape sequence %s in input delimiter specification; trailing characters %s not recognised.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
                endp,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return val as libc::c_char;
}
unsafe extern "C" fn get_input_delimiter(mut s: *const libc::c_char) -> libc::c_char {
    if 1 as libc::c_int as libc::c_ulong == strlen(s) {
        return *s.offset(0 as libc::c_int as isize)
    } else if '\\' as i32 == *s.offset(0 as libc::c_int as isize) as libc::c_int {
        match *s.offset(1 as libc::c_int as isize) as libc::c_int {
            97 => return '\u{7}' as i32 as libc::c_char,
            98 => return '\u{8}' as i32 as libc::c_char,
            102 => return '\u{c}' as i32 as libc::c_char,
            110 => return '\n' as i32 as libc::c_char,
            114 => return '\r' as i32 as libc::c_char,
            116 => return '\t' as i32 as libc::c_char,
            118 => return '\u{b}' as i32 as libc::c_char,
            92 => return '\\' as i32 as libc::c_char,
            _ => return get_char_oct_or_hex_escape(s),
        }
    } else {
        if ::core::mem::size_of::<C2RustUnnamed_30>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid input delimiter specification %s: the delimiter must be either a single character or an escape sequence starting with \\.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid input delimiter specification %s: the delimiter must be either a single character or an escape sequence starting with \\.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return 0 as libc::c_int as libc::c_char;
    };
}
unsafe extern "C" fn noop() {}
unsafe extern "C" fn fail_due_to_env_size() {
    if ::core::mem::size_of::<C2RustUnnamed_31>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"environment is too large for exec\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"environment is too large for exec\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn smaller_of(mut a: size_t, mut b: size_t) -> size_t {
    if a < b { return a } else { return b };
}
unsafe extern "C" fn fopen_cloexec_for_read_only(
    mut file_name: *const libc::c_char,
) -> *mut FILE {
    let mut fd: libc::c_int = open_cloexec(file_name, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as *mut FILE
    } else {
        let mut result: *mut FILE = fdopen(
            fd,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if result.is_null() {
            let mut saved_errno: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = saved_errno;
            return 0 as *mut FILE;
        }
        return result;
    };
}
unsafe extern "C" fn warn_mutually_exclusive(
    mut option: *const libc::c_char,
    mut offending: *const libc::c_char,
) {
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"warning: options %s and %s are mutually exclusive, ignoring previous %s value\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        offending,
        option,
        offending,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut option_index: libc::c_int = 0;
    let mut show_limits: libc::c_int = 0 as libc::c_int;
    let mut always_run_command: libc::c_int = 1 as libc::c_int;
    let mut input_file: *const libc::c_char = b"-\0" as *const u8 as *const libc::c_char;
    let mut default_cmd: [libc::c_char; 5] = *::core::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"echo\0");
    let mut default_arglist: [*mut libc::c_char; 1] = [0 as *mut libc::c_char; 1];
    let mut read_args: Option::<unsafe extern "C" fn() -> libc::c_int> = Some(
        read_line as unsafe extern "C" fn() -> libc::c_int,
    );
    let mut act_on_init_result: Option::<unsafe extern "C" fn() -> ()> = Some(
        noop as unsafe extern "C" fn() -> (),
    );
    let mut bcstatus: BC_INIT_STATUS = BC_INIT_OK;
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    if !(*argv.offset(0 as libc::c_int as isize)).is_null() {
        set_program_name(*argv.offset(0 as libc::c_int as isize));
    } else {
        set_program_name(b"xargs\0" as *const u8 as *const libc::c_char);
    }
    remember_non_cloexec_fds();
    parent = getpid();
    ::core::ptr::write_volatile(
        &mut original_exit_value as *mut libc::c_int,
        0 as libc::c_int,
    );
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"findutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"findutils\0" as *const u8 as *const libc::c_char);
    if atexit(Some(close_stdin as unsafe extern "C" fn() -> ())) != 0
        || atexit(Some(wait_for_proc_all as unsafe extern "C" fn() -> ())) != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_35>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    bcstatus = bc_init_controlinfo(
        &mut bc_ctl,
        XARGS_POSIX_HEADROOM as libc::c_int as size_t,
    );
    if BC_INIT_ENV_TOO_BIG as libc::c_int as libc::c_uint == bcstatus as libc::c_uint {
        act_on_init_result = Some(fail_due_to_env_size as unsafe extern "C" fn() -> ());
    } else if BC_INIT_CANNOT_ACCOMODATE_HEADROOM as libc::c_int as libc::c_uint
        == bcstatus as libc::c_uint
    {
        act_on_init_result = Some(fail_due_to_env_size as unsafe extern "C" fn() -> ());
    } else {
        let mut val: libc::c_long = 0;
        val = sysconf(_SC_ARG_MAX as libc::c_int);
        if val > 0 as libc::c_int as libc::c_long {
            if val > XARGS_POSIX_HEADROOM as libc::c_int as libc::c_long {} else {
                __assert_fail(
                    b"val > XARGS_POSIX_HEADROOM\0" as *const u8 as *const libc::c_char,
                    b"xargs.c\0" as *const u8 as *const libc::c_char,
                    483 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_12933: {
                if val > XARGS_POSIX_HEADROOM as libc::c_int as libc::c_long {} else {
                    __assert_fail(
                        b"val > XARGS_POSIX_HEADROOM\0" as *const u8
                            as *const libc::c_char,
                        b"xargs.c\0" as *const u8 as *const libc::c_char,
                        483 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            bc_ctl
                .arg_max = smaller_of(
                bc_ctl.arg_max,
                (val as size_t)
                    .wrapping_sub(XARGS_POSIX_HEADROOM as libc::c_int as libc::c_ulong),
            );
        }
        if bc_ctl.arg_max >= 2048 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"bc_ctl.arg_max >= LINE_MAX\0" as *const u8 as *const libc::c_char,
                b"xargs.c\0" as *const u8 as *const libc::c_char,
                511 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_12869: {
            if bc_ctl.arg_max >= 2048 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"bc_ctl.arg_max >= LINE_MAX\0" as *const u8 as *const libc::c_char,
                    b"xargs.c\0" as *const u8 as *const libc::c_char,
                    511 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        bc_ctl
            .exec_callback = Some(
            xargs_do_exec
                as unsafe extern "C" fn(
                    *mut buildcmd_control,
                    *mut libc::c_void,
                    libc::c_int,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        );
        bc_use_sensible_arg_max(&mut bc_ctl);
    }
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"+0a:E:e::i::I:l::L:n:oprs:txP:d:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            &mut option_index,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            48 => {
                read_args = Some(read_string as unsafe extern "C" fn() -> libc::c_int);
                input_delimiter = '\0' as i32 as libc::c_char;
            }
            100 => {
                read_args = Some(read_string as unsafe extern "C" fn() -> libc::c_int);
                input_delimiter = get_input_delimiter(optarg);
            }
            69 | 101 => {
                if !optarg.is_null()
                    && strlen(optarg) > 0 as libc::c_int as libc::c_ulong
                {
                    eof_str = optarg;
                } else {
                    eof_str = 0 as *mut libc::c_char;
                }
            }
            104 => {
                usage(0 as libc::c_int);
            }
            73 | 105 => {
                if !optarg.is_null() {
                    bc_ctl.replace_pat = optarg;
                } else {
                    bc_ctl.replace_pat = b"{}\0" as *const u8 as *const libc::c_char;
                }
                if bc_ctl.args_per_exec != 0 as libc::c_int as libc::c_ulong {
                    warn_mutually_exclusive(
                        b"--replace/-I/-i\0" as *const u8 as *const libc::c_char,
                        b"--max-args\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.args_per_exec = 0 as libc::c_int as size_t;
                }
                if bc_ctl.lines_per_exec != 0 as libc::c_int as libc::c_ulong {
                    warn_mutually_exclusive(
                        b"--replace/-I/-i\0" as *const u8 as *const libc::c_char,
                        b"--max-lines\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.lines_per_exec = 0 as libc::c_int as libc::c_ulong;
                }
            }
            76 => {
                bc_ctl
                    .lines_per_exec = parse_num(
                    optarg,
                    'L' as i32,
                    1 as libc::c_long,
                    -(1 as libc::c_long),
                    1 as libc::c_int,
                ) as libc::c_ulong;
                if bc_ctl.args_per_exec != 0 as libc::c_int as libc::c_ulong {
                    warn_mutually_exclusive(
                        b"-L\0" as *const u8 as *const libc::c_char,
                        b"--max-args\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.args_per_exec = 0 as libc::c_int as size_t;
                }
                if !(bc_ctl.replace_pat).is_null() {
                    warn_mutually_exclusive(
                        b"-L\0" as *const u8 as *const libc::c_char,
                        b"--replace\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.replace_pat = 0 as *const libc::c_char;
                }
            }
            108 => {
                if !optarg.is_null() {
                    bc_ctl
                        .lines_per_exec = parse_num(
                        optarg,
                        'l' as i32,
                        1 as libc::c_long,
                        -(1 as libc::c_long),
                        1 as libc::c_int,
                    ) as libc::c_ulong;
                } else {
                    bc_ctl.lines_per_exec = 1 as libc::c_int as libc::c_ulong;
                }
                if bc_ctl.args_per_exec != 0 as libc::c_int as libc::c_ulong {
                    warn_mutually_exclusive(
                        b"--max-lines/-l\0" as *const u8 as *const libc::c_char,
                        b"--max-args\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.args_per_exec = 0 as libc::c_int as size_t;
                }
                if !(bc_ctl.replace_pat).is_null() {
                    warn_mutually_exclusive(
                        b"--max-lines/-l\0" as *const u8 as *const libc::c_char,
                        b"--replace\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.replace_pat = 0 as *const libc::c_char;
                }
            }
            110 => {
                bc_ctl
                    .args_per_exec = parse_num(
                    optarg,
                    'n' as i32,
                    1 as libc::c_long,
                    -(1 as libc::c_long),
                    1 as libc::c_int,
                ) as size_t;
                if bc_ctl.lines_per_exec != 0 as libc::c_int as libc::c_ulong {
                    warn_mutually_exclusive(
                        b"--max-args/-n\0" as *const u8 as *const libc::c_char,
                        b"--max-lines\0" as *const u8 as *const libc::c_char,
                    );
                    bc_ctl.lines_per_exec = 0 as libc::c_int as libc::c_ulong;
                }
                if !(bc_ctl.replace_pat).is_null() {
                    if bc_ctl.args_per_exec == 1 as libc::c_int as libc::c_ulong {
                        bc_ctl.args_per_exec = 0 as libc::c_int as size_t;
                    } else {
                        warn_mutually_exclusive(
                            b"--max-args/-n\0" as *const u8 as *const libc::c_char,
                            b"--replace\0" as *const u8 as *const libc::c_char,
                        );
                        bc_ctl.replace_pat = 0 as *const libc::c_char;
                    }
                }
            }
            115 => {
                let mut arg_size: size_t = 0;
                act_on_init_result.expect("non-null function pointer")();
                arg_size = parse_num(
                    optarg,
                    's' as i32,
                    1 as libc::c_long,
                    bc_ctl.posix_arg_size_max as libc::c_long,
                    0 as libc::c_int,
                ) as size_t;
                if arg_size > bc_ctl.posix_arg_size_max {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: value %ld for -s option is too large, using %ld instead\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        arg_size as libc::c_long,
                        bc_ctl.posix_arg_size_max as libc::c_long,
                    );
                    arg_size = bc_ctl.posix_arg_size_max;
                }
                bc_ctl.arg_max = arg_size;
            }
            83 => {
                show_limits = 1 as libc::c_int;
            }
            116 => {
                print_command = 1 as libc::c_int != 0;
            }
            120 => {
                bc_ctl.exit_if_size_exceeded = 1 as libc::c_int;
            }
            111 => {
                open_tty = 1 as libc::c_int != 0;
            }
            112 => {
                query_before_executing = 1 as libc::c_int != 0;
                print_command = 1 as libc::c_int != 0;
            }
            114 => {
                always_run_command = 0 as libc::c_int;
            }
            80 => {
                ::core::ptr::write_volatile(
                    &mut proc_max as *mut sig_atomic_t,
                    parse_num(
                        optarg,
                        'P' as i32,
                        0 as libc::c_long,
                        2147483647 as libc::c_int as libc::c_long,
                        1 as libc::c_int,
                    ) as sig_atomic_t,
                );
            }
            97 => {
                input_file = optarg;
            }
            118 => {
                display_findutils_version(
                    b"xargs\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            128 => {
                if !(strchr(optarg, '=' as i32)).is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"option --%s may not be set to a value which includes `='\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            longopts[option_index as usize].name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"option --%s may not be set to a value which includes `='\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            longopts[option_index as usize].name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                slot_var_name = optarg;
                if 0 as libc::c_int != unsetenv(slot_var_name) {
                    if ::core::mem::size_of::<C2RustUnnamed_33>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"failed to unset environment variable %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            slot_var_name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"failed to unset environment variable %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            slot_var_name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if !eof_str.is_null()
        && read_args == Some(read_string as unsafe extern "C" fn() -> libc::c_int)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: the -E option has no effect if -0 or -d is used.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    act_on_init_result.expect("non-null function pointer")();
    if BC_INIT_OK as libc::c_int as libc::c_uint == bcstatus as libc::c_uint {} else {
        __assert_fail(
            b"BC_INIT_OK == bcstatus\0" as *const u8 as *const libc::c_char,
            b"xargs.c\0" as *const u8 as *const libc::c_char,
            723 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_12024: {
        if BC_INIT_OK as libc::c_int as libc::c_uint == bcstatus as libc::c_uint
        {} else {
            __assert_fail(
                b"BC_INIT_OK == bcstatus\0" as *const u8 as *const libc::c_char,
                b"xargs.c\0" as *const u8 as *const libc::c_char,
                723 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    sigact
        .__sigaction_handler
        .sa_handler = Some(
        increment_proc_max as unsafe extern "C" fn(libc::c_int) -> (),
    );
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 0 as libc::c_int;
    if 0 as libc::c_int
        != sigaction(
            10 as libc::c_int,
            &mut sigact,
            0 as *mut libc::c_void as *mut sigaction,
        )
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot set SIGUSR1 signal handler\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    sigact
        .__sigaction_handler
        .sa_handler = Some(
        decrement_proc_max as unsafe extern "C" fn(libc::c_int) -> (),
    );
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 0 as libc::c_int;
    if 0 as libc::c_int
        != sigaction(
            12 as libc::c_int,
            &mut sigact,
            0 as *mut libc::c_void as *mut sigaction,
        )
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot set SIGUSR2 signal handler\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if 0 as libc::c_int == strcmp(input_file, b"-\0" as *const u8 as *const libc::c_char)
    {
        input_stream = stdin;
    } else {
        keep_stdin = 1 as libc::c_int;
        input_stream = fopen_cloexec_for_read_only(input_file);
        if input_stream.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot open input file %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, input_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot open input file %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, input_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if !(bc_ctl.replace_pat).is_null() || bc_ctl.lines_per_exec != 0 {
        bc_ctl.exit_if_size_exceeded = 1 as libc::c_int;
    }
    if optind == argc {
        optind = 0 as libc::c_int;
        argc = 1 as libc::c_int;
        default_arglist[0 as libc::c_int as usize] = default_cmd.as_mut_ptr();
        argv = default_arglist.as_mut_ptr();
    }
    if show_limits != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Your environment variables take up %lu bytes\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bc_size_of_environment(),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"POSIX upper limit on argument length (this system): %lu\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            bc_ctl.posix_arg_size_max,
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"POSIX smallest allowable upper limit on argument length (all systems): %lu\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            bc_ctl.posix_arg_size_min,
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Maximum length of command we could actually use: %lu\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (bc_ctl.posix_arg_size_max).wrapping_sub(bc_size_of_environment()),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Size of command buffer we are actually using: %lu\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            bc_ctl.arg_max,
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Maximum parallelism (--max-procs must be no greater): %lu\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            2147483647 as libc::c_int as uintmax_t,
        );
        if isatty(0 as libc::c_int) != 0 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\nExecution of xargs will continue now, and it will try to read its input and run commands; if this is not what you wanted to happen, please type the end-of-file keystroke.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if always_run_command != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Warning: %s will be run at least once.  If you do not want that to happen, then press the interrupt keystroke.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(optind as isize),
                );
            }
        }
    }
    linebuf = xmalloc((bc_ctl.arg_max).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    bc_state
        .argbuf = xmalloc(
        (bc_ctl.arg_max).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    signal(17 as libc::c_int, None);
    if (bc_ctl.replace_pat).is_null() {
        while optind < argc {
            bc_push_arg(
                &mut bc_ctl,
                &mut bc_state,
                *argv.offset(optind as isize),
                (strlen(*argv.offset(optind as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as *const libc::c_char,
                0 as libc::c_int as size_t,
                initial_args as libc::c_int,
            );
            optind += 1;
            optind;
        }
        initial_args = 0 as libc::c_int != 0;
        bc_ctl.initial_argc = bc_state.cmd_argc;
        bc_state.cmd_initial_argv_chars = bc_state.cmd_argv_chars;
        bc_ctl.initial_argc = bc_state.cmd_argc;
        while (Some(read_args.expect("non-null function pointer")))
            .expect("non-null function pointer")() != -(1 as libc::c_int)
        {
            if bc_ctl.lines_per_exec != 0 && lineno >= bc_ctl.lines_per_exec {
                bc_do_exec(&mut bc_ctl, &mut bc_state);
                lineno = 0 as libc::c_int as size_t;
            }
        }
        if bc_state.cmd_argc != bc_ctl.initial_argc
            || always_run_command != 0
                && procs_executed as libc::c_int == 0 as libc::c_int
        {
            bc_do_exec(&mut bc_ctl, &mut bc_state);
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut args: libc::c_int = 0;
        let mut arglen: *mut size_t = xmalloc(
            (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(argc as libc::c_ulong),
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
            if !(args != -(1 as libc::c_int)) {
                break;
            }
            let mut len: size_t = args as size_t;
            bc_clear_args(&mut bc_ctl, &mut bc_state);
            bc_state.cmd_argv_chars = 0 as libc::c_int as size_t;
            bc_push_arg(
                &mut bc_ctl,
                &mut bc_state,
                *argv.offset(optind as isize),
                (*arglen.offset(optind as isize))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as *const libc::c_char,
                0 as libc::c_int as size_t,
                initial_args as libc::c_int,
            );
            len = len.wrapping_sub(1);
            len;
            initial_args = 0 as libc::c_int != 0;
            i = optind + 1 as libc::c_int;
            while i < argc {
                bc_do_insert(
                    &mut bc_ctl,
                    &mut bc_state,
                    *argv.offset(i as isize),
                    *arglen.offset(i as isize),
                    0 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                    linebuf,
                    len,
                    initial_args as libc::c_int,
                );
                i += 1;
                i;
            }
            bc_do_exec(&mut bc_ctl, &mut bc_state);
        }
    }
    ::core::ptr::write_volatile(
        &mut original_exit_value as *mut libc::c_int,
        child_error,
    );
    return child_error;
}
unsafe extern "C" fn read_line() -> libc::c_int {
    static mut eof: bool = 0 as libc::c_int != 0;
    let mut state: read_line_state = SPACE;
    let mut prevc: libc::c_int = 0;
    let mut quotc: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = -(1 as libc::c_int);
    let mut first: bool = 1 as libc::c_int != 0;
    let mut seen_arg: bool = 0 as libc::c_int != 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = linebuf;
    let mut endbuf: *mut libc::c_char = linebuf
        .offset(bc_ctl.arg_max as isize)
        .offset(-(bc_state.cmd_initial_argv_chars as isize))
        .offset(-(1 as libc::c_int as isize));
    if eof {
        return -(1 as libc::c_int);
    }
    let mut current_block_64: u64;
    loop {
        prevc = c;
        c = _IO_getc(input_stream);
        if c == -(1 as libc::c_int) {
            eof = 1 as libc::c_int != 0;
            if p == linebuf {
                return -(1 as libc::c_int);
            }
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\0' as i32 as libc::c_char;
            len = p.offset_from(linebuf) as libc::c_long as libc::c_int;
            if state as libc::c_uint == QUOTE as libc::c_int as libc::c_uint {
                exec_if_possible();
                if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (if quotc == '"' as i32 {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"double\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"single\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        }),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (if quotc == '"' as i32 {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"double\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"single\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        }),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if first as libc::c_int != 0
                && (!eof_str.is_null()
                    && *eof_str as libc::c_int == *linebuf as libc::c_int
                    && strcmp(eof_str, linebuf) == 0)
            {
                return -(1 as libc::c_int);
            }
            if (bc_ctl.replace_pat).is_null() {
                bc_push_arg(
                    &mut bc_ctl,
                    &mut bc_state,
                    linebuf,
                    len as size_t,
                    0 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                    initial_args as libc::c_int,
                );
            }
            return len;
        }
        match state as libc::c_uint {
            1 => {
                if c & !(0x7f as libc::c_int) == 0 as libc::c_int
                    && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || c == '\n' as i32 || c == '\r' as i32 || c == '\u{c}' as i32
                    || c == '\u{b}' as i32
                {
                    continue;
                }
                state = NORM;
                current_block_64 = 5609494561082130361;
            }
            0 => {
                current_block_64 = 5609494561082130361;
            }
            2 => {
                if c == '\n' as i32 {
                    exec_if_possible();
                    if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (if quotc == '"' as i32 {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"double\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            } else {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"single\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            }),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (if quotc == '"' as i32 {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"double\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            } else {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"single\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            }),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if c == quotc {
                    state = NORM;
                    seen_arg = 1 as libc::c_int != 0;
                    continue;
                } else {
                    current_block_64 = 13325891313334703151;
                }
            }
            3 => {
                state = NORM;
                current_block_64 = 13325891313334703151;
            }
            _ => {
                current_block_64 = 13325891313334703151;
            }
        }
        match current_block_64 {
            5609494561082130361 => {
                if c == '\n' as i32 {
                    if !(prevc & !(0x7f as libc::c_int) == 0 as libc::c_int
                        && *(*__ctype_b_loc()).offset(prevc as isize) as libc::c_int
                            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                    {
                        lineno = lineno.wrapping_add(1);
                        lineno;
                    }
                    if p == linebuf {
                        if !seen_arg {
                            state = SPACE;
                            continue;
                        }
                    }
                    let fresh1 = p;
                    p = p.offset(1);
                    *fresh1 = '\0' as i32 as libc::c_char;
                    len = p.offset_from(linebuf) as libc::c_long as libc::c_int;
                    if !eof_str.is_null()
                        && *eof_str as libc::c_int == *linebuf as libc::c_int
                        && strcmp(eof_str, linebuf) == 0
                    {
                        eof = 1 as libc::c_int != 0;
                        return if first as libc::c_int != 0 {
                            -(1 as libc::c_int)
                        } else {
                            len
                        };
                    }
                    if (bc_ctl.replace_pat).is_null() {
                        bc_push_arg(
                            &mut bc_ctl,
                            &mut bc_state,
                            linebuf,
                            len as size_t,
                            0 as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            initial_args as libc::c_int,
                        );
                    }
                    return len;
                } else {
                    seen_arg = 1 as libc::c_int != 0;
                    if (bc_ctl.replace_pat).is_null()
                        && (c & !(0x7f as libc::c_int) == 0 as libc::c_int
                            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                    {
                        let fresh2 = p;
                        p = p.offset(1);
                        *fresh2 = '\0' as i32 as libc::c_char;
                        len = p.offset_from(linebuf) as libc::c_long as libc::c_int;
                        if !eof_str.is_null()
                            && *eof_str as libc::c_int == *linebuf as libc::c_int
                            && strcmp(eof_str, linebuf) == 0
                        {
                            eof = 1 as libc::c_int != 0;
                            return if first as libc::c_int != 0 {
                                -(1 as libc::c_int)
                            } else {
                                len
                            };
                        }
                        bc_push_arg(
                            &mut bc_ctl,
                            &mut bc_state,
                            linebuf,
                            len as size_t,
                            0 as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            initial_args as libc::c_int,
                        );
                        p = linebuf;
                        state = SPACE;
                        first = 0 as libc::c_int != 0;
                        continue;
                    } else {
                        match c {
                            92 => {
                                current_block_64 = 1614245230000234774;
                                match current_block_64 {
                                    1614245230000234774 => {
                                        state = BACKSLASH;
                                        continue;
                                    }
                                    _ => {
                                        state = QUOTE;
                                        quotc = c;
                                        continue;
                                    }
                                }
                            }
                            39 | 34 => {
                                current_block_64 = 4187689338455206605;
                                match current_block_64 {
                                    1614245230000234774 => {
                                        state = BACKSLASH;
                                        continue;
                                    }
                                    _ => {
                                        state = QUOTE;
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
        if 0 as libc::c_int == c && nullwarning_given == 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: a NUL character occurred in the input.  It cannot be passed through in the argument list.  Did you mean to use the --null option?\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            nullwarning_given = 1 as libc::c_int;
        }
        if p >= endbuf {
            exec_if_possible();
            if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"argument line too long\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"argument line too long\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = c as libc::c_char;
    };
}
unsafe extern "C" fn read_string() -> libc::c_int {
    static mut eof: bool = 0 as libc::c_int != 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = linebuf;
    let mut endbuf: *mut libc::c_char = linebuf
        .offset(bc_ctl.arg_max as isize)
        .offset(-(bc_state.cmd_initial_argv_chars as isize))
        .offset(-(1 as libc::c_int as isize));
    if eof {
        return -(1 as libc::c_int);
    }
    loop {
        let mut c: libc::c_int = _IO_getc(input_stream);
        if c == -(1 as libc::c_int) {
            eof = 1 as libc::c_int != 0;
            if p == linebuf {
                return -(1 as libc::c_int);
            }
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = '\0' as i32 as libc::c_char;
            len = p.offset_from(linebuf) as libc::c_long as libc::c_int;
            if (bc_ctl.replace_pat).is_null() {
                bc_push_arg(
                    &mut bc_ctl,
                    &mut bc_state,
                    linebuf,
                    len as size_t,
                    0 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                    initial_args as libc::c_int,
                );
            }
            return len;
        }
        if c == input_delimiter as libc::c_int {
            lineno = lineno.wrapping_add(1);
            lineno;
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = '\0' as i32 as libc::c_char;
            len = p.offset_from(linebuf) as libc::c_long as libc::c_int;
            if (bc_ctl.replace_pat).is_null() {
                bc_push_arg(
                    &mut bc_ctl,
                    &mut bc_state,
                    linebuf,
                    len as size_t,
                    0 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                    initial_args as libc::c_int,
                );
            }
            return len;
        }
        if p >= endbuf {
            exec_if_possible();
            if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"argument line too long\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"argument line too long\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = c as libc::c_char;
    };
}
unsafe extern "C" fn print_args(mut ask: bool) -> bool {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (bc_state.cmd_argc).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        if fprintf(
            stderr,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (if i == 0 as libc::c_int as libc::c_ulong {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b" \0" as *const u8 as *const libc::c_char
            }),
            quotearg_n_style(
                0 as libc::c_int,
                shell_escape_quoting_style,
                *(bc_state.cmd_argv).offset(i as isize),
            ),
        ) < 0 as libc::c_int
        {
            if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to write to stderr\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to write to stderr\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        i = i.wrapping_add(1);
        i;
    }
    if ask {
        static mut tty_stream: *mut FILE = 0 as *const FILE as *mut FILE;
        let mut c: libc::c_int = 0;
        let mut savec: libc::c_int = 0;
        if tty_stream.is_null() {
            tty_stream = fopen_cloexec_for_read_only(
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
            );
            if tty_stream.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to open /dev/tty for reading\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to open /dev/tty for reading\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        fputs(b"?...\0" as *const u8 as *const libc::c_char, stderr);
        if rpl_fflush(stderr) != 0 as libc::c_int {
            if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to write to stderr\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to write to stderr\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        savec = _IO_getc(tty_stream);
        c = savec;
        while c != -(1 as libc::c_int) && c != '\n' as i32 {
            c = _IO_getc(tty_stream);
        }
        if -(1 as libc::c_int) == c {
            if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to read from stdin\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to read from stdin\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if savec == 'y' as i32 || savec == 'Y' as i32 {
            return 1 as libc::c_int != 0;
        }
    } else {
        _IO_putc('\n' as i32, stderr);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn set_slot_var(mut n: libc::c_uint) {
    let mut buf: [libc::c_char; 20] = [0; 20];
    if snprintf(
        buf.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"%u\0" as *const u8 as *const libc::c_char,
        n,
    ) as libc::c_ulong
        <= (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"snprintf (buf, sizeof buf - 1, \"%u\", n) <= sizeof buf - 1\0" as *const u8
                as *const libc::c_char,
            b"xargs.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void set_slot_var(unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_8760: {
        if snprintf(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%u\0" as *const u8 as *const libc::c_char,
            n,
        ) as libc::c_ulong
            <= (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {} else {
            __assert_fail(
                b"snprintf (buf, sizeof buf - 1, \"%u\", n) <= sizeof buf - 1\0"
                    as *const u8 as *const libc::c_char,
                b"xargs.c\0" as *const u8 as *const libc::c_char,
                1190 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void set_slot_var(unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !slot_var_name.is_null() {
        if setenv(slot_var_name, buf.as_mut_ptr(), 1 as libc::c_int) < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to set environment variable %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
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
    let mut slot: libc::c_uint = add_proc(0 as libc::c_int);
    set_slot_var(slot);
    if keep_stdin == 0 || open_tty as libc::c_int != 0 {
        let mut fd: libc::c_int = 0;
        let mut inputfile: *const libc::c_char = if open_tty as libc::c_int != 0 {
            b"/dev/tty\0" as *const u8 as *const libc::c_char
        } else {
            b"/dev/null\0" as *const u8 as *const libc::c_char
        };
        close(0 as libc::c_int);
        fd = open_safer(inputfile, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            if open_tty {
                if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style(
                            0 as libc::c_int,
                            locale_quoting_style,
                            inputfile,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style(
                            0 as libc::c_int,
                            locale_quoting_style,
                            inputfile,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, inputfile),
                );
            }
        }
        if (0 as libc::c_int) < fd {
            if dup2(fd, 0 as libc::c_int) != 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to redirect standard input of the child process\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"failed to redirect standard input of the child process\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
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
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut child: pid_t = 0;
    let mut fd: [libc::c_int; 2] = [0; 2];
    let mut buf: libc::c_int = 0;
    let mut r: size_t = 0;
    if proc_max != 0 {
        while procs_executing >= proc_max as libc::c_ulong {
            wait_for_proc(0 as libc::c_int != 0, 1 as libc::c_uint);
        }
    }
    if !query_before_executing || print_args(1 as libc::c_int != 0) as libc::c_int != 0 {
        if !query_before_executing && print_command as libc::c_int != 0 {
            print_args(0 as libc::c_int != 0);
        }
        wait_for_proc(0 as libc::c_int != 0, 0 as libc::c_uint);
        if pipe_safer(fd.as_mut_ptr()) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"could not create pipe before fork\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"could not create pipe before fork\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        rpl_fcntl(fd[1 as libc::c_int as usize], 2 as libc::c_int, 1 as libc::c_int);
        loop {
            child = fork();
            if !(child < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int
                && procs_executing != 0)
            {
                break;
            }
            wait_for_proc(0 as libc::c_int != 0, 1 as libc::c_uint);
        }
        's_149: {
            match child {
                -1 => {
                    if ::core::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot fork\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot fork\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                0 => {}
                _ => {
                    close(fd[1 as libc::c_int as usize]);
                    break 's_149;
                }
            }
            close(fd[0 as libc::c_int as usize]);
            ::core::ptr::write_volatile(
                &mut child_error as *mut libc::c_int,
                0 as libc::c_int,
            );
            prep_child_for_exec();
            if bc_args_exceed_testing_limit(argv) {
                *__errno_location() = 7 as libc::c_int;
            } else {
                execvp(
                    *argv.offset(0 as libc::c_int as isize),
                    argv as *const *mut libc::c_char,
                );
            }
            if *__errno_location() != 0 {
                write(
                    fd[1 as libc::c_int as usize],
                    __errno_location() as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                );
            }
            close(fd[1 as libc::c_int as usize]);
            if 7 as libc::c_int != *__errno_location() {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
            }
            _exit(
                if *__errno_location() == 2 as libc::c_int {
                    XARGS_EXIT_COMMAND_NOT_FOUND as libc::c_int
                } else {
                    XARGS_EXIT_COMMAND_CANNOT_BE_RUN as libc::c_int
                },
            );
        }
        r = safe_read(
            fd[0 as libc::c_int as usize],
            &mut buf as *mut libc::c_int as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        match r {
            18446744073709551615 => {
                close(fd[0 as libc::c_int as usize]);
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"errno-buffer safe_read failed in xargs_do_exec (this is probably a bug, please report it)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            4 => {
                let mut childstatus: libc::c_int = 0;
                close(fd[0 as libc::c_int as usize]);
                waitpid(child, &mut childstatus, 0 as libc::c_int);
                if 7 as libc::c_int == buf {
                    return 0 as libc::c_int
                } else if 2 as libc::c_int == buf {
                    exit(XARGS_EXIT_COMMAND_NOT_FOUND as libc::c_int);
                } else {
                    exit(XARGS_EXIT_COMMAND_CANNOT_BE_RUN as libc::c_int);
                }
            }
            0 => {
                add_proc(child);
            }
            _ => {
                if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"read returned unexpected value %lu; this is probably a bug, please report it\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        r,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"read returned unexpected value %lu; this is probably a bug, please report it\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        r,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        close(fd[0 as libc::c_int as usize]);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn exec_if_possible() {
    if !(bc_ctl.replace_pat).is_null() || initial_args as libc::c_int != 0
        || bc_state.cmd_argc == bc_ctl.initial_argc || bc_ctl.exit_if_size_exceeded != 0
    {
        return;
    }
    bc_do_exec(&mut bc_ctl, &mut bc_state);
}
unsafe extern "C" fn add_proc(mut pid: pid_t) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < pids_alloc && *pids.offset(i as isize) != 0 {
        i = i.wrapping_add(1);
        i;
    }
    if i as libc::c_ulong == pids_alloc {
        pids = x2nrealloc(
            pids as *mut libc::c_void,
            &mut pids_alloc,
            ::core::mem::size_of::<pid_t>() as libc::c_ulong,
        ) as *mut pid_t;
        j = i;
        while (j as libc::c_ulong) < pids_alloc {
            *pids.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
    }
    if 0 as libc::c_int == *pids.offset(i as isize) {} else {
        __assert_fail(
            b"0 == pids[i]\0" as *const u8 as *const libc::c_char,
            b"xargs.c\0" as *const u8 as *const libc::c_char,
            1477 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"unsigned int add_proc(pid_t)\0"))
                .as_ptr(),
        );
    }
    'c_8097: {
        if 0 as libc::c_int == *pids.offset(i as isize) {} else {
            __assert_fail(
                b"0 == pids[i]\0" as *const u8 as *const libc::c_char,
                b"xargs.c\0" as *const u8 as *const libc::c_char,
                1477 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"unsigned int add_proc(pid_t)\0"))
                    .as_ptr(),
            );
        }
    };
    *pids.offset(i as isize) = pid;
    procs_executing = procs_executing.wrapping_add(1);
    procs_executing;
    procs_executed = 1 as libc::c_int != 0;
    return i;
}
unsafe extern "C" fn wait_for_proc(mut all: bool, mut minreap: libc::c_uint) {
    let mut reaped: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while procs_executing != 0 {
        let mut i: libc::c_uint = 0;
        let mut status: libc::c_int = 0;
        let mut pid: pid_t = 0;
        let mut wflags: libc::c_int = 0 as libc::c_int;
        if !all {
            if reaped >= minreap {
                wflags = 1 as libc::c_int;
            }
        }
        ::core::ptr::write_volatile(
            &mut stop_waiting as *mut sig_atomic_t,
            0 as libc::c_int,
        );
        loop {
            loop {
                pid = waitpid(-(1 as libc::c_int), &mut status, wflags);
                if !(pid == -(1 as libc::c_int)) {
                    break;
                }
                if *__errno_location() != 4 as libc::c_int {
                    if ::core::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error waiting for child process\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error waiting for child process\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if stop_waiting != 0 && !all {
                    wflags = 1 as libc::c_int;
                }
            }
            if pid != 0 {
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < pids_alloc
                    && pid != *pids.offset(i as isize)
                {
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if !(pid != 0 && i as libc::c_ulong == pids_alloc) {
                break;
            }
        }
        if pid == 0 {
            if wflags & 1 as libc::c_int == 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"WARNING: Lost track of %lu child processes\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    procs_executing,
                );
            }
            break;
        } else {
            *pids.offset(i as isize) = 0 as libc::c_int;
            procs_executing = procs_executing.wrapping_sub(1);
            procs_executing;
            reaped = reaped.wrapping_add(1);
            reaped;
            if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int
                == CHILD_EXIT_PLEASE_STOP_IMMEDIATELY as libc::c_int
            {
                error(
                    XARGS_EXIT_CLIENT_EXIT_255 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: exited with status 255; aborting\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *(bc_state.cmd_argv).offset(0 as libc::c_int as isize),
                );
            }
            if status & 0xff as libc::c_int == 0x7f as libc::c_int {
                error(
                    XARGS_EXIT_CLIENT_FATAL_SIG as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: stopped by signal %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *(bc_state.cmd_argv).offset(0 as libc::c_int as isize),
                    (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                );
            }
            if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
            {
                error(
                    XARGS_EXIT_CLIENT_FATAL_SIG as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: terminated by signal %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *(bc_state.cmd_argv).offset(0 as libc::c_int as isize),
                    status & 0x7f as libc::c_int,
                );
            }
            if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0 as libc::c_int {
                ::core::ptr::write_volatile(
                    &mut child_error as *mut libc::c_int,
                    XARGS_EXIT_CLIENT_EXIT_NONZERO as libc::c_int,
                );
            }
        }
    }
}
unsafe extern "C" fn wait_for_proc_all() {
    static mut waiting: bool = 0 as libc::c_int != 0;
    if getpid() == parent {} else {
        __assert_fail(
            b"getpid () == parent\0" as *const u8 as *const libc::c_char,
            b"xargs.c\0" as *const u8 as *const libc::c_char,
            1605 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void wait_for_proc_all(void)\0"))
                .as_ptr(),
        );
    }
    'c_9561: {
        if getpid() == parent {} else {
            __assert_fail(
                b"getpid () == parent\0" as *const u8 as *const libc::c_char,
                b"xargs.c\0" as *const u8 as *const libc::c_char,
                1605 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void wait_for_proc_all(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if waiting {
        return;
    }
    waiting = 1 as libc::c_int != 0;
    wait_for_proc(1 as libc::c_int != 0, 0 as libc::c_uint);
    waiting = 0 as libc::c_int != 0;
    if original_exit_value != child_error {
        _exit(child_error);
    }
}
unsafe extern "C" fn increment_proc_max(mut ignore: libc::c_int) {
    if proc_max < 2147483647 as libc::c_int {
        ::core::ptr::write_volatile(
            &mut proc_max as *mut sig_atomic_t,
            ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t)
                + 1,
        );
        ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t);
    }
    ::core::ptr::write_volatile(
        &mut stop_waiting as *mut sig_atomic_t,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn decrement_proc_max(mut ignore: libc::c_int) {
    if proc_max > 1 as libc::c_int {
        ::core::ptr::write_volatile(
            &mut proc_max as *mut sig_atomic_t,
            ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t)
                - 1,
        );
        ::core::ptr::read_volatile::<sig_atomic_t>(&proc_max as *const sig_atomic_t);
    }
}
unsafe extern "C" fn parse_num(
    mut str: *mut libc::c_char,
    mut option: libc::c_int,
    mut min: libc::c_long,
    mut max: libc::c_long,
    mut fatal: libc::c_int,
) -> libc::c_long {
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_long = 0;
    val = strtol(str, &mut eptr, 10 as libc::c_int);
    if eptr == str || *eptr as libc::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: invalid number \"%s\" for -%c option\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            str,
            option,
        );
        usage(1 as libc::c_int);
    } else if val < min {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: value %s for -%c option should be >= %ld\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            str,
            option,
            min,
        );
        if fatal != 0 {
            usage(1 as libc::c_int);
        }
        val = min;
    } else if max >= 0 as libc::c_int as libc::c_long && val > max {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: value %s for -%c option should be <= %ld\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
            str,
            option,
            max,
        );
        if fatal != 0 {
            usage(1 as libc::c_int);
        }
        val = max;
    }
    return val;
}
unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... COMMAND [INITIAL-ARGS]...\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Run COMMAND with arguments INITIAL-ARGS and more arguments read from input.\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Mandatory and optional arguments to long options are also\nmandatory or optional for the corresponding short option.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -0, --null                   items are separated by a null, not whitespace;\n                                 disables quote and backslash processing and\n                                 logical EOF processing\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -a, --arg-file=FILE          read arguments from FILE, not standard input\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -d, --delimiter=CHARACTER    items in input stream are separated by CHARACTER,\n                                 not by whitespace; disables quote and backslash\n                                 processing and logical EOF processing\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -E END                       set logical EOF string; if END occurs as a line\n                                 of input, the rest of the input is ignored\n                                 (ignored if -0 or -d was specified)\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -e, --eof[=END]              equivalent to -E END if END is specified;\n                                 otherwise, there is no end-of-file string\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -I R                         same as --replace=R\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -i, --replace[=R]            replace R in INITIAL-ARGS with names read\n                                 from standard input, split at newlines;\n                                 if R is unspecified, assume {}\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -L, --max-lines=MAX-LINES    use at most MAX-LINES non-blank input lines per\n                                 command line\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -l[MAX-LINES]                similar to -L but defaults to at most one non-\n                                 blank input line if MAX-LINES is not specified\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -n, --max-args=MAX-ARGS      use at most MAX-ARGS arguments per command line\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -o, --open-tty               Reopen stdin as /dev/tty in the child process\n                                 before executing the command; useful to run an\n                                 interactive application.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -P, --max-procs=MAX-PROCS    run at most MAX-PROCS processes at a time\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -p, --interactive            prompt before running commands\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --process-slot-var=VAR   set environment variable VAR in child processes\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -r, --no-run-if-empty        if there are no arguments, then do not run COMMAND;\n                                 if this option is not given, COMMAND will be\n                                 run at least once\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -s, --max-chars=MAX-CHARS    limit length of command line to MAX-CHARS\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --show-limits            show limits on command-line length\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -t, --verbose                print commands before executing them\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"  -x, --exit                   exit if the size (see -s) is exceeded\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --help                   display this help and exit\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --version                output version information and exit\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
