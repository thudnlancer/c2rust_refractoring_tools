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
}  // end of enum

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
