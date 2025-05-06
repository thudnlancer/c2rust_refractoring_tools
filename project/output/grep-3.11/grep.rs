#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type exclude;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstatat(
        __ver: i32,
        __fildes: i32,
        __filename: *const i8,
        __stat_buf: *mut stat,
        __flag: i32,
    ) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn wcrtomb(__s: *mut i8, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn rpl_mbrlen(s: *const i8, n: size_t, ps: *mut mbstate_t) -> size_t;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush_unlocked(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn fputs_unlocked(__s: *const i8, __stream: *mut FILE) -> i32;
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
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn __uflow(_: *mut _IO_FILE) -> i32;
    fn rpl_fopen(filename_0: *const i8, mode: *const i8) -> *mut FILE;
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn __errno_location() -> *mut i32;
    static mut optind: i32;
    static mut optarg: *mut i8;
    fn isatty(__fd: i32) -> i32;
    fn sysconf(__name: i32) -> i64;
    fn close(__fd: i32) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn getprogname() -> *const i8;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: i32) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strip_trailing_slashes(file: *mut i8) -> bool;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn splice(
        __fdin: i32,
        __offin: *mut __off64_t,
        __fdout: i32,
        __offout: *mut __off64_t,
        __len: size_t,
        __flags: u32,
    ) -> __ssize_t;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const i8,
        arg: *const i8,
        arglist: *const *const i8,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn c_stack_action(_: Option<unsafe extern "C" fn(i32) -> ()>) -> i32;
    fn close_stdout();
    fn should_colorize() -> i32;
    fn init_colorize();
    fn print_start_colorize(sgr_start_0: *const i8, sgr_seq: *const i8);
    fn print_end_colorize(sgr_end_0: *const i8);
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn new_exclude() -> *mut exclude;
    fn add_exclude(_: *mut exclude, _: *const i8, _: i32);
    fn add_exclude_file(
        _: Option<unsafe extern "C" fn(*mut exclude, *const i8, i32) -> ()>,
        _: *mut exclude,
        _: *const i8,
        _: i32,
        _: i8,
    ) -> i32;
    fn excluded_file_name(_: *const exclude, _: *const i8) -> bool;
    static mut exit_failure: i32;
    fn openat_safer(_: i32, _: *const i8, _: i32, _: ...) -> i32;
    fn rpl_fts_open(
        _: *const *mut i8,
        _: i32,
        _: Option<unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> i32>,
    ) -> *mut FTS;
    fn rpl_fts_close(_: *mut FTS) -> i32;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: i32) -> i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert_if_absent(
        table: *mut Hash_table,
        entry: *const libc::c_void,
        matched_ent: *mut *const libc::c_void,
    ) -> i32;
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn init_localeinfo(_: *mut localeinfo);
    fn xstrdup(str: *const i8) -> *mut i8;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn Fexecute(
        _: *mut libc::c_void,
        _: *const i8,
        _: idx_t,
        _: *mut idx_t,
        _: *const i8,
    ) -> ptrdiff_t;
    fn Fcompile(_: *mut i8, _: idx_t, _: reg_syntax_t, _: bool) -> *mut libc::c_void;
    fn EGexecute(
        _: *mut libc::c_void,
        _: *const i8,
        _: idx_t,
        _: *mut idx_t,
        _: *const i8,
    ) -> ptrdiff_t;
    fn GEAcompile(_: *mut i8, _: idx_t, _: reg_syntax_t, _: bool) -> *mut libc::c_void;
    fn wordinit();
    fn case_folded_counterparts(_: wint_t, _: *mut wchar_t) -> i32;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const i8,
        package: *const i8,
        version: *const i8,
        _: ...
    );
    fn emit_bug_reporting_address();
    fn xstrtoimax(
        _: *const i8,
        _: *mut *mut i8,
        _: i32,
        _: *mut intmax_t,
        _: *const i8,
    ) -> strtol_error;
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
pub type __int32_t = i32;
pub type __intmax_t = i64;
pub type __uintmax_t = u64;
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
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = i64;
pub type wchar_t = i32;
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
pub type va_list = __builtin_va_list;
pub type wint_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type mbstate_t = __mbstate_t;
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
pub type uint_fast64_t = u64;
pub type intptr_t = i64;
pub type uintptr_t = u64;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_0 = u32;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_0 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_0 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_0 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_0 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_0 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_0 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_0 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_0 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_0 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_0 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_0 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_0 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_0 = 236;
pub const _SC_IPV6: C2RustUnnamed_0 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_0 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_0 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_0 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_0 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_0 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_0 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_0 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_0 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_0 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_0 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_0 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_0 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_0 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_0 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_0 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_0 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_0 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_0 = 182;
pub const _SC_TRACE: C2RustUnnamed_0 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_0 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_0 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_0 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_0 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_0 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_0 = 175;
pub const _SC_STREAMS: C2RustUnnamed_0 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_0 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_0 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_0 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_0 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_0 = 169;
pub const _SC_2_PBS: C2RustUnnamed_0 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_0 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_0 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_0 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_0 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_0 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_0 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_0 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_0 = 160;
pub const _SC_SPAWN: C2RustUnnamed_0 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_0 = 158;
pub const _SC_SHELL: C2RustUnnamed_0 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_0 = 156;
pub const _SC_REGEXP: C2RustUnnamed_0 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_0 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_0 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_0 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_0 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_0 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_0 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_0 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_0 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_0 = 146;
pub const _SC_PIPE: C2RustUnnamed_0 = 145;
pub const _SC_FIFO: C2RustUnnamed_0 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_0 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_0 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_0 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_0 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_0 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_0 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_0 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_0 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_0 = 135;
pub const _SC_BASE: C2RustUnnamed_0 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_0 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_0 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_0 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_0 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_0 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_0 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_0 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_0 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_0 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_0 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_0 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_0 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_0 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_0 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_0 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_0 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_0 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_0 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_0 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_0 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_0 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_0 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_0 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_0 = 110;
pub const _SC_NZERO: C2RustUnnamed_0 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_0 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_0 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_0 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_0 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_0 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_0 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_0 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_0 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_0 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_0 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_0 = 98;
pub const _SC_2_UPE: C2RustUnnamed_0 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_0 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_0 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_0 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_0 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_0 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_0 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_0 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_0 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_0 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_0 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_0 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_0 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_0 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_0 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_0 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_0 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_0 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_0 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_0 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_0 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_0 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_0 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_0 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_0 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_0 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_0 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_0 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_0 = 68;
pub const _SC_THREADS: C2RustUnnamed_0 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_0 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_0 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_0 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_0 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_0 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_0 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_0 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_0 = 60;
pub const _SC_SELECT: C2RustUnnamed_0 = 59;
pub const _SC_POLL: C2RustUnnamed_0 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_0 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_0 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_0 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_0 = 54;
pub const _SC_PII: C2RustUnnamed_0 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_0 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_0 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_0 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_0 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_0 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_0 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_0 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_0 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_0 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_0 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_0 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_0 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_0 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_0 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_0 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_0 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_0 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_0 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_0 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_0 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_0 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_0 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_0 = 30;
pub const _SC_VERSION: C2RustUnnamed_0 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_0 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_0 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_0 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_0 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_0 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_0 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_0 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_0 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_0 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_0 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_0 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_0 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_0 = 16;
pub const _SC_FSYNC: C2RustUnnamed_0 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_0 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_0 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_0 = 12;
pub const _SC_TIMERS: C2RustUnnamed_0 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_0 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_0 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_0 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_0 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_0 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_0 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_0 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_0 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_0 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_0 = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    EXIT_TROUBLE = 2,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::EXIT_TROUBLE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            2 => C2RustUnnamed_1::EXIT_TROUBLE,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    NCHAR = 256,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::NCHAR => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            256 => C2RustUnnamed_2::NCHAR,
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
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [i32; 4],
    pub ir_default_val: i32,
    pub ir_front: u32,
    pub ir_back: u32,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut i8,
    pub fts_rfd: i32,
    pub fts_cwd_fd: i32,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> i32,
    >,
    pub fts_options: i32,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed_3,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: i64,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut i8,
    pub fts_path: *mut i8,
    pub fts_errno: i32,
    pub fts_symfd: i32,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [i8; 0],
}
pub type FTSENT = _ftsent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct patloc {
    pub lineno: idx_t,
    pub filename: *const i8,
    pub fileline: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    SEP_CHAR_SELECTED,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::SEP_CHAR_SELECTED => 58,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            58 => C2RustUnnamed_4::SEP_CHAR_SELECTED,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    SEP_CHAR_REJECTED,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_5::SEP_CHAR_REJECTED => 45,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_5 {
        match value {
            45 => C2RustUnnamed_5::SEP_CHAR_REJECTED,
            _ => panic!("Invalid value for C2RustUnnamed_5: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_5 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_5 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_5 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_5 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_5 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn add(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn sub(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn mul(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn div(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn rem(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_cap {
    pub name: *const i8,
    pub var: *mut *const i8,
    pub fct: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
    NO_IGNORE_CASE_OPTION = 137,
    LABEL_OPTION = 136,
    LINE_BUFFERED_OPTION = 135,
    INCLUDE_OPTION = 134,
    GROUP_SEPARATOR_OPTION = 133,
    EXCLUDE_FROM_OPTION = 132,
    EXCLUDE_OPTION = 131,
    EXCLUDE_DIRECTORY_OPTION = 130,
    COLOR_OPTION = 129,
    BINARY_FILES_OPTION = 128,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::NO_IGNORE_CASE_OPTION => 137,
            C2RustUnnamed_6::LABEL_OPTION => 136,
            C2RustUnnamed_6::LINE_BUFFERED_OPTION => 135,
            C2RustUnnamed_6::INCLUDE_OPTION => 134,
            C2RustUnnamed_6::GROUP_SEPARATOR_OPTION => 133,
            C2RustUnnamed_6::EXCLUDE_FROM_OPTION => 132,
            C2RustUnnamed_6::EXCLUDE_OPTION => 131,
            C2RustUnnamed_6::EXCLUDE_DIRECTORY_OPTION => 130,
            C2RustUnnamed_6::COLOR_OPTION => 129,
            C2RustUnnamed_6::BINARY_FILES_OPTION => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            137 => C2RustUnnamed_6::NO_IGNORE_CASE_OPTION,
            136 => C2RustUnnamed_6::LABEL_OPTION,
            135 => C2RustUnnamed_6::LINE_BUFFERED_OPTION,
            134 => C2RustUnnamed_6::INCLUDE_OPTION,
            133 => C2RustUnnamed_6::GROUP_SEPARATOR_OPTION,
            132 => C2RustUnnamed_6::EXCLUDE_FROM_OPTION,
            131 => C2RustUnnamed_6::EXCLUDE_OPTION,
            130 => C2RustUnnamed_6::EXCLUDE_DIRECTORY_OPTION,
            129 => C2RustUnnamed_6::COLOR_OPTION,
            128 => C2RustUnnamed_6::BINARY_FILES_OPTION,
            _ => panic!("Invalid value for C2RustUnnamed_6: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_6 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_6 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_6 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_6 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_6 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn add(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn sub(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn mul(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn div(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn rem(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum directories_type {
    READ_DIRECTORIES = 2,
    RECURSE_DIRECTORIES,
    SKIP_DIRECTORIES,
}
impl directories_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            directories_type::READ_DIRECTORIES => 2,
            directories_type::RECURSE_DIRECTORIES => 3,
            directories_type::SKIP_DIRECTORIES => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> directories_type {
        match value {
            2 => directories_type::READ_DIRECTORIES,
            3 => directories_type::RECURSE_DIRECTORIES,
            4 => directories_type::SKIP_DIRECTORIES,
            _ => panic!("Invalid value for directories_type: {}", value),
        }
    }
}
impl AddAssign<u32> for directories_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = directories_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for directories_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = directories_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for directories_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = directories_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for directories_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = directories_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for directories_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = directories_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for directories_type {
    type Output = directories_type;
    fn add(self, rhs: u32) -> directories_type {
        directories_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for directories_type {
    type Output = directories_type;
    fn sub(self, rhs: u32) -> directories_type {
        directories_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for directories_type {
    type Output = directories_type;
    fn mul(self, rhs: u32) -> directories_type {
        directories_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for directories_type {
    type Output = directories_type;
    fn div(self, rhs: u32) -> directories_type {
        directories_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for directories_type {
    type Output = directories_type;
    fn rem(self, rhs: u32) -> directories_type {
        directories_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_7 {
    basic_fts_options = 776,
}
impl C2RustUnnamed_7 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_7::basic_fts_options => 776,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_7 {
        match value {
            776 => C2RustUnnamed_7::basic_fts_options,
            _ => panic!("Invalid value for C2RustUnnamed_7: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_7 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_7 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_7 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_7 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_7 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn add(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn sub(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn mul(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn div(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn rem(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_8 {
    SKIP_DEVICES = 2,
    READ_DEVICES = 1,
    READ_COMMAND_LINE_DEVICES = 0,
}
impl C2RustUnnamed_8 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_8::SKIP_DEVICES => 2,
            C2RustUnnamed_8::READ_DEVICES => 1,
            C2RustUnnamed_8::READ_COMMAND_LINE_DEVICES => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_8 {
        match value {
            2 => C2RustUnnamed_8::SKIP_DEVICES,
            1 => C2RustUnnamed_8::READ_DEVICES,
            0 => C2RustUnnamed_8::READ_COMMAND_LINE_DEVICES,
            _ => panic!("Invalid value for C2RustUnnamed_8: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_8 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_8 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_8 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_8 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_8 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn add(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn sub(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn mul(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn div(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn rem(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    LISTFILES_MATCHING = 1,
    LISTFILES_NONMATCHING = 2,
    LISTFILES_NONE = 0,
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_9::LISTFILES_MATCHING => 1,
            C2RustUnnamed_9::LISTFILES_NONMATCHING => 2,
            C2RustUnnamed_9::LISTFILES_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_9 {
        match value {
            1 => C2RustUnnamed_9::LISTFILES_MATCHING,
            2 => C2RustUnnamed_9::LISTFILES_NONMATCHING,
            0 => C2RustUnnamed_9::LISTFILES_NONE,
            _ => panic!("Invalid value for C2RustUnnamed_9: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_9 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_9 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_9 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_9 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_9 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn add(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn sub(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn mul(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn div(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn rem(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_15 {
    INITIAL_BUFSIZE = 98304,
}
impl C2RustUnnamed_15 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_15::INITIAL_BUFSIZE => 98304,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_15 {
        match value {
            98304 => C2RustUnnamed_15::INITIAL_BUFSIZE,
            _ => panic!("Invalid value for C2RustUnnamed_15: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_15 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_15 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_15 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_15 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_15 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_15 {
    type Output = C2RustUnnamed_15;
    fn add(self, rhs: u32) -> C2RustUnnamed_15 {
        C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_15 {
    type Output = C2RustUnnamed_15;
    fn sub(self, rhs: u32) -> C2RustUnnamed_15 {
        C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_15 {
    type Output = C2RustUnnamed_15;
    fn mul(self, rhs: u32) -> C2RustUnnamed_15 {
        C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_15 {
    type Output = C2RustUnnamed_15;
    fn div(self, rhs: u32) -> C2RustUnnamed_15 {
        C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_15 {
    type Output = C2RustUnnamed_15;
    fn rem(self, rhs: u32) -> C2RustUnnamed_15 {
        C2RustUnnamed_15::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_10 {
    BINARY_BINARY_FILES = 0,
    WITHOUT_MATCH_BINARY_FILES = 2,
    TEXT_BINARY_FILES = 1,
}
impl C2RustUnnamed_10 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_10::BINARY_BINARY_FILES => 0,
            C2RustUnnamed_10::WITHOUT_MATCH_BINARY_FILES => 2,
            C2RustUnnamed_10::TEXT_BINARY_FILES => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_10 {
        match value {
            0 => C2RustUnnamed_10::BINARY_BINARY_FILES,
            2 => C2RustUnnamed_10::WITHOUT_MATCH_BINARY_FILES,
            1 => C2RustUnnamed_10::TEXT_BINARY_FILES,
            _ => panic!("Invalid value for C2RustUnnamed_10: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_10 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_10 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_10 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_10 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_10 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn add(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn sub(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn mul(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn div(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn rem(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: i32,
}
pub type uword = uintmax_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_13 {
    uword_size = 8,
}
impl C2RustUnnamed_13 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_13::uword_size => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_13 {
        match value {
            8 => C2RustUnnamed_13::uword_size,
            _ => panic!("Invalid value for C2RustUnnamed_13: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_13 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_13 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_13 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_13 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_13 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn add(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn sub(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn mul(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn div(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn rem(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type execute_fp_t = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const i8,
        idx_t,
        *mut idx_t,
        *const i8,
    ) -> ptrdiff_t,
>;
pub type compile_fp_t = Option<
    unsafe extern "C" fn(*mut i8, idx_t, reg_syntax_t, bool) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub name: [i8; 12],
    pub syntax: i32,
    pub compile: compile_fp_t,
    pub execute: execute_fp_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_17 {
    E_MATCHER_INDEX = 1,
    F_MATCHER_INDEX = 2,
    G_MATCHER_INDEX = 0,
}
impl C2RustUnnamed_17 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_17::E_MATCHER_INDEX => 1,
            C2RustUnnamed_17::F_MATCHER_INDEX => 2,
            C2RustUnnamed_17::G_MATCHER_INDEX => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_17 {
        match value {
            1 => C2RustUnnamed_17::E_MATCHER_INDEX,
            2 => C2RustUnnamed_17::F_MATCHER_INDEX,
            0 => C2RustUnnamed_17::G_MATCHER_INDEX,
            _ => panic!("Invalid value for C2RustUnnamed_17: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_17 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_17 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_17 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_17 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_17 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_17 {
    type Output = C2RustUnnamed_17;
    fn add(self, rhs: u32) -> C2RustUnnamed_17 {
        C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_17 {
    type Output = C2RustUnnamed_17;
    fn sub(self, rhs: u32) -> C2RustUnnamed_17 {
        C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_17 {
    type Output = C2RustUnnamed_17;
    fn mul(self, rhs: u32) -> C2RustUnnamed_17 {
        C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_17 {
    type Output = C2RustUnnamed_17;
    fn div(self, rhs: u32) -> C2RustUnnamed_17 {
        C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_17 {
    type Output = C2RustUnnamed_17;
    fn rem(self, rhs: u32) -> C2RustUnnamed_17 {
        C2RustUnnamed_17::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: i32,
    mut __filename: *const i8,
    mut __statbuf: *mut stat,
    mut __flag: i32,
) -> i32 {
    return __fxstatat(1 as i32, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: i32) -> i32 {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as i32 as i64 != 0 {
        __overflow(stdout, __c as u8 as i32)
    } else {
        let fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x20 as i32 != 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: i8) -> u8 {
    return ch as u8;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn imbrlen(
    mut s: *const i8,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: size_t = rpl_mbrlen(s, n as size_t, mbs);
    if len <= 16 as i32 as u64 {
        return len as ptrdiff_t;
    }
    let mut neglen: ptrdiff_t = len.wrapping_neg() as ptrdiff_t;
    return -neglen;
}
#[inline]
unsafe extern "C" fn mb_clen(
    mut s: *const i8,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: libc::c_schar = localeinfo.sbclen[to_uchar(*s) as usize];
    return if len as i32 == -(2 as i32) { imbrlen(s, n, mbs) } else { len as i64 };
}
#[inline]
unsafe extern "C" fn __gl_setmode(mut fd: i32, mut mode: i32) -> i32 {
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn set_binary_mode(mut fd: i32, mut mode: i32) -> i32 {
    return __gl_setmode(fd, mode);
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: i32, mut mode: i32) {
    if set_binary_mode(fd, mode) < 0 as i32 {
        xset_binary_mode_error();
    }
}
static mut SEP_STR_GROUP: [i8; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[i8; 3]>(b"--\0")
};
static mut out_stat: stat = stat {
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
static mut show_help: i32 = 0;
static mut show_version: bool = false;
static mut suppress_errors: bool = false;
static mut color_option: i32 = 0;
static mut only_matching: bool = false;
static mut align_tabs: bool = false;
static mut offset_width: i32 = 0;
static mut patloc: *mut patloc = 0 as *const patloc as *mut patloc;
static mut patlocs_allocated: idx_t = 0;
static mut patlocs_used: idx_t = 0;
static mut pattern_array: *mut i8 = 0 as *const i8 as *mut i8;
static mut n_patterns: idx_t = 0;
static mut pattern_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn hash_pattern(
    mut pat: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut h15: uint_fast64_t = 5381 as i32 as uint_fast64_t;
    let mut h32: uint_fast64_t = 3657500101 as i64 as uint_fast64_t;
    let mut h64: uint_fast64_t = 4123221751654370051 as i64 as uint_fast64_t;
    let mut h: size_t = if h64 <= 18446744073709551615 as u64 {
        h64
    } else if h32 <= 18446744073709551615 as u64 {
        h32
    } else {
        h15
    };
    let mut pat_offset: intptr_t = pat as intptr_t - 1 as i32 as i64;
    let mut s: *const u8 = (pattern_array as *const u8).offset(pat_offset as isize);
    while *s as i32 != '\n' as i32 {
        h = h.wrapping_mul(33 as i32 as u64) ^ *s as u64;
        s = s.offset(1);
        s;
    }
    return h.wrapping_rem(n_buckets);
}
unsafe extern "C" fn compare_patterns(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> bool {
    let mut a_offset: intptr_t = a as intptr_t - 1 as i32 as i64;
    let mut b_offset: intptr_t = b as intptr_t - 1 as i32 as i64;
    let mut p: *const i8 = pattern_array.offset(a_offset as isize);
    let mut q: *const i8 = pattern_array.offset(b_offset as isize);
    while *p as i32 == *q as i32 {
        if *p as i32 == '\n' as i32 {
            return 1 as i32 != 0;
        }
        p = p.offset(1);
        p;
        q = q.offset(1);
        q;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn update_patterns(
    mut keys: *mut i8,
    mut dupfree_size: idx_t,
    mut size: idx_t,
    mut filename_0: *const i8,
) -> idx_t {
    let mut dst: *mut i8 = keys.offset(dupfree_size as isize);
    let mut fileline: idx_t = 1 as i32 as idx_t;
    let mut prev_inserted: i32 = 0 as i32;
    let mut srclim: *const i8 = keys.offset(size as isize);
    let mut patsize: idx_t = 0;
    let mut src: *const i8 = keys.offset(dupfree_size as isize);
    while src < srclim {
        let mut patend: *const i8 = rawmemchr(src as *const libc::c_void, '\n' as i32)
            as *const i8;
        patsize = patend.offset(1 as i32 as isize).offset_from(src) as i64;
        memmove(dst as *mut libc::c_void, src as *const libc::c_void, patsize as u64);
        let mut dst_offset_1: intptr_t = dst.offset_from(keys) as i64 + 1 as i32 as i64;
        let mut inserted: i32 = hash_insert_if_absent(
            pattern_table,
            dst_offset_1 as *mut libc::c_void,
            0 as *mut *const libc::c_void,
        );
        if inserted != 0 {
            if inserted < 0 as i32 {
                xalloc_die();
            }
            dst = dst.offset(patsize as isize);
            if prev_inserted == 0 {
                if patlocs_used == patlocs_allocated {
                    patloc = xpalloc(
                        patloc as *mut libc::c_void,
                        &mut patlocs_allocated,
                        1 as i32 as idx_t,
                        -(1 as i32) as ptrdiff_t,
                        ::core::mem::size_of::<patloc>() as u64 as idx_t,
                    ) as *mut patloc;
                }
                let fresh1 = patlocs_used;
                patlocs_used = patlocs_used + 1;
                *patloc.offset(fresh1 as isize) = {
                    let mut init = patloc {
                        lineno: n_patterns,
                        filename: filename_0,
                        fileline: fileline,
                    };
                    init
                };
            }
            n_patterns += 1;
            n_patterns;
        }
        prev_inserted = inserted;
        fileline += 1;
        fileline;
        src = src.offset(patsize as isize);
    }
    return dst.offset_from(keys) as i64;
}
#[no_mangle]
pub unsafe extern "C" fn pattern_file_name(
    mut lineno: idx_t,
    mut new_lineno: *mut idx_t,
) -> *const i8 {
    let mut i: idx_t = 0;
    i = 1 as i32 as idx_t;
    while i < patlocs_used {
        if lineno < (*patloc.offset(i as isize)).lineno {
            break;
        }
        i += 1;
        i;
    }
    *new_lineno = lineno - (*patloc.offset((i - 1 as i32 as i64) as isize)).lineno
        + (*patloc.offset((i - 1 as i32 as i64) as isize)).fileline;
    return (*patloc.offset((i - 1 as i32 as i64) as isize)).filename;
}
unsafe extern "C" fn clear_asan_poison() {}
unsafe extern "C" fn asan_poison(mut addr: *const libc::c_void, mut size: idx_t) {}
static mut group_separator: *const i8 = unsafe { SEP_STR_GROUP.as_ptr() };
static mut selected_match_color: *const i8 = b"01;31\0" as *const u8 as *const i8;
static mut context_match_color: *const i8 = b"01;31\0" as *const u8 as *const i8;
static mut filename_color: *const i8 = b"35\0" as *const u8 as *const i8;
static mut line_num_color: *const i8 = b"32\0" as *const u8 as *const i8;
static mut byte_num_color: *const i8 = b"32\0" as *const u8 as *const i8;
static mut sep_color: *const i8 = b"36\0" as *const u8 as *const i8;
static mut selected_line_color: *const i8 = b"\0" as *const u8 as *const i8;
static mut context_line_color: *const i8 = b"\0" as *const u8 as *const i8;
static mut sgr_start: *const i8 = b"\x1B[%sm\x1B[K\0" as *const u8 as *const i8;
static mut sgr_end: *const i8 = b"\x1B[m\x1B[K\0" as *const u8 as *const i8;
unsafe extern "C" fn pr_sgr_start(mut s: *const i8) {
    if *s != 0 {
        print_start_colorize(sgr_start, s);
    }
}
unsafe extern "C" fn pr_sgr_end(mut s: *const i8) {
    if *s != 0 {
        print_end_colorize(sgr_end);
    }
}
unsafe extern "C" fn pr_sgr_start_if(mut s: *const i8) {
    if color_option != 0 {
        pr_sgr_start(s);
    }
}
unsafe extern "C" fn pr_sgr_end_if(mut s: *const i8) {
    if color_option != 0 {
        pr_sgr_end(s);
    }
}
unsafe extern "C" fn color_cap_mt_fct() {
    context_match_color = selected_match_color;
}
unsafe extern "C" fn color_cap_rv_fct() {
    color_option = -(1 as i32);
}
unsafe extern "C" fn color_cap_ne_fct() {
    sgr_start = b"\x1B[%sm\0" as *const u8 as *const i8;
    sgr_end = b"\x1B[m\0" as *const u8 as *const i8;
}
static mut color_dict: [color_cap; 12] = unsafe {
    [
        {
            let mut init = color_cap {
                name: b"mt\0" as *const u8 as *const i8,
                var: &selected_match_color as *const *const i8 as *mut *const i8,
                fct: Some(color_cap_mt_fct as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"ms\0" as *const u8 as *const i8,
                var: &selected_match_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"mc\0" as *const u8 as *const i8,
                var: &context_match_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"fn\0" as *const u8 as *const i8,
                var: &filename_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"ln\0" as *const u8 as *const i8,
                var: &line_num_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"bn\0" as *const u8 as *const i8,
                var: &byte_num_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"se\0" as *const u8 as *const i8,
                var: &sep_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"sl\0" as *const u8 as *const i8,
                var: &selected_line_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"cx\0" as *const u8 as *const i8,
                var: &context_line_color as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"rv\0" as *const u8 as *const i8,
                var: 0 as *const *const i8 as *mut *const i8,
                fct: Some(color_cap_rv_fct as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"ne\0" as *const u8 as *const i8,
                var: 0 as *const *const i8 as *mut *const i8,
                fct: Some(color_cap_ne_fct as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = color_cap {
                name: 0 as *const i8,
                var: 0 as *const *const i8 as *mut *const i8,
                fct: None,
            };
            init
        },
    ]
};
static mut stdout_errno: i32 = 0;
unsafe extern "C" fn putchar_errno(mut c: i32) {
    if putchar_unlocked(c) < 0 as i32 {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fputs_errno(mut s: *const i8) {
    if fputs_unlocked(s, stdout) < 0 as i32 {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn printf_errno(mut format: *const i8, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if vfprintf(stdout, format, ap.as_va_list()) < 0 as i32 {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fwrite_errno(
    mut ptr: *const libc::c_void,
    mut size: idx_t,
    mut nmemb: idx_t,
) {
    if (if 0 != 0 && 0 != 0
        && (size as size_t).wrapping_mul(nmemb as size_t) <= 8 as i32 as u64
        && size as size_t != 0 as i32 as u64
    {
        ({
            let mut __ptr: *const i8 = ptr as *const i8;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = (size as size_t).wrapping_mul(nmemb as size_t);
            while __cnt > 0 as i32 as u64 {
                if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32
                    as i64 != 0
                {
                    let fresh2 = __ptr;
                    __ptr = __ptr.offset(1);
                    __overflow(__stream, *fresh2 as u8 as i32)
                } else {
                    let fresh3 = __ptr;
                    __ptr = __ptr.offset(1);
                    let fresh4 = (*__stream)._IO_write_ptr;
                    (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
                    *fresh4 = *fresh3;
                    *fresh4 as u8 as i32
                }) == -(1 as i32)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            (size as size_t)
                .wrapping_mul(nmemb as size_t)
                .wrapping_sub(__cnt)
                .wrapping_div(size as size_t)
        })
    } else {
        (if 0 != 0 && size as size_t == 0 as i32 as u64
            || 0 != 0 && nmemb as size_t == 0 as i32 as u64
        {
            0 as i32 as size_t
        } else {
            fwrite_unlocked(ptr, size as size_t, nmemb as size_t, stdout)
        })
    }) != nmemb as u64
    {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fflush_errno() {
    if fflush_unlocked(stdout) != 0 as i32 {
        stdout_errno = *__errno_location();
    }
}
static mut excluded_patterns: [*mut exclude; 2] = [0 as *const exclude
    as *mut exclude; 2];
static mut excluded_directory_patterns: [*mut exclude; 2] = [0 as *const exclude
    as *mut exclude; 2];
static mut short_options: [i8; 58] = unsafe {
    *::core::mem::transmute::<
        &[u8; 58],
        &[i8; 58],
    >(b"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz\0")
};
static mut long_options: [option; 51] = unsafe {
    [
        {
            let mut init = option {
                name: b"basic-regexp\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"extended-regexp\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fixed-regexp\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fixed-strings\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"perl-regexp\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"after-context\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'A' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"before-context\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"binary-files\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::BINARY_FILES_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"byte-offset\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"context\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"color\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::COLOR_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"colour\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::COLOR_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"count\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"devices\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"directories\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::EXCLUDE_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude-from\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::EXCLUDE_FROM_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude-dir\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::EXCLUDE_DIRECTORY_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"files-with-matches\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"files-without-match\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"group-separator\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::GROUP_SEPARATOR_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &show_help as *const i32 as *mut i32,
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"include\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::INCLUDE_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignore-case\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-ignore-case\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::NO_IGNORE_CASE_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"initial-tab\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"label\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::LABEL_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-buffered\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::LINE_BUFFERED_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-number\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-regexp\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'x' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"max-count\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-filename\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-group-separator\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_6::GROUP_SEPARATOR_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-messages\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'Z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null-data\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"only-matching\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"recursive\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dereference-recursive\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'R' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"regexp\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"invert-match\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"text\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"binary\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unix-byte-offsets\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"with-filename\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"word-regexp\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'w' as i32,
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
    ]
};
#[no_mangle]
pub static mut match_icase: bool = false;
#[no_mangle]
pub static mut match_words: bool = false;
#[no_mangle]
pub static mut match_lines: bool = false;
#[no_mangle]
pub static mut eolbyte: i8 = 0;
static mut filename: *const i8 = 0 as *const i8;
static mut omit_dot_slash: bool = false;
static mut errseen: bool = false;
static mut encoding_error_output: bool = false;
static mut directories_args: [*const i8; 4] = [
    b"read\0" as *const u8 as *const i8,
    b"recurse\0" as *const u8 as *const i8,
    b"skip\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut directories_types: [directories_type; 3] = [
    directories_type::READ_DIRECTORIES,
    directories_type::RECURSE_DIRECTORIES,
    directories_type::SKIP_DIRECTORIES,
];
static mut directories: directories_type = directories_type::READ_DIRECTORIES;
static mut fts_options: i32 = C2RustUnnamed_7::basic_fts_options as i32 | 0x1 as i32
    | 0x10 as i32;
static mut devices: C2RustUnnamed_8 = C2RustUnnamed_8::READ_COMMAND_LINE_DEVICES;
unsafe extern "C" fn is_device_mode(mut m: mode_t) -> bool {
    return m & 0o170000 as i32 as u32 == 0o20000 as i32 as u32
        || m & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        || m & 0o170000 as i32 as u32 == 0o140000 as i32 as u32
        || m & 0o170000 as i32 as u32 == 0o10000 as i32 as u32;
}
unsafe extern "C" fn skip_devices(mut command_line: bool) -> bool {
    return devices as u32 == C2RustUnnamed_8::SKIP_DEVICES as i32 as u32
        || (devices as u32 == C2RustUnnamed_8::READ_COMMAND_LINE_DEVICES as i32 as u32)
            as i32 & !command_line as i32 != 0;
}
unsafe extern "C" fn usable_st_size(mut st: *const stat) -> bool {
    return (*st).st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
        || ((*st).st_mode).wrapping_sub((*st).st_mode) != 0 || 0 as i32 != 0;
}
static mut seek_failed: bool = false;
static mut seek_data_failed: bool = false;
static mut execute: execute_fp_t = None;
static mut compiled_pattern: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn input_filename() -> *const i8 {
    if filename.is_null() {
        filename = dcgettext(
            0 as *const i8,
            b"(standard input)\0" as *const u8 as *const i8,
            5 as i32,
        );
    }
    return filename;
}
unsafe extern "C" fn suppressible_error(mut errnum: i32) {
    if !suppress_errors {
        error(0 as i32, errnum, b"%s\0" as *const u8 as *const i8, input_filename());
    }
    errseen = 1 as i32 != 0;
}
unsafe extern "C" fn clean_up_stdout() {
    if stdout_errno == 0 {
        close_stdout();
    }
}
static mut uword_max: uword = 18446744073709551615 as u64;
#[no_mangle]
pub static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
static mut unibyte_mask: uword = 0;
unsafe extern "C" fn initialize_unibyte_mask() {
    let mut mask: u8 = 0 as i32 as u8;
    let mut ms1b: i32 = 1 as i32;
    let mut i: i32 = 1 as i32;
    while i <= 127 as i32 * 2 as i32 + 1 as i32 {
        if (localeinfo.sbclen[i as usize] as i32 != 1 as i32) as i32
            & (mask as i32 & i == 0) as i32 != 0
        {
            while ms1b * 2 as i32 <= i {
                ms1b *= 2 as i32;
            }
            mask = (mask as i32 | ms1b) as u8;
        }
        i += 1;
        i;
    }
    unibyte_mask = uword_max
        .wrapping_div((127 as i32 * 2 as i32 + 1 as i32) as u64)
        .wrapping_mul(mask as u64);
}
unsafe extern "C" fn skip_easy_bytes(mut buf: *const i8) -> *const i8 {
    let mut p: *const i8 = 0 as *const i8;
    let mut s: *const uword = 0 as *const uword;
    p = buf;
    while (p as uintptr_t).wrapping_rem(C2RustUnnamed_13::uword_size as i32 as u64)
        != 0 as i32 as u64
    {
        if to_uchar(*p) as u64 & unibyte_mask != 0 {
            return p;
        }
        p = p.offset(1);
        p;
    }
    s = p as *const uword;
    while *s & unibyte_mask == 0 {
        s = s.offset(1);
        s;
    }
    p = s as *const i8;
    while to_uchar(*p) as u64 & unibyte_mask == 0 {
        p = p.offset(1);
        p;
    }
    return p;
}
unsafe extern "C" fn buf_has_encoding_errors(mut buf: *mut i8, mut size: idx_t) -> bool {
    if unibyte_mask == 0 {
        return 0 as i32 != 0;
    }
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut clen: ptrdiff_t = 0;
    *buf.offset(size as isize) = -(1 as i32) as i8;
    let mut p: *const i8 = buf;
    loop {
        p = skip_easy_bytes(p);
        if !(p < buf.offset(size as isize)) {
            break;
        }
        clen = imbrlen(p, buf.offset(size as isize).offset_from(p) as i64, &mut mbs);
        if clen < 0 as i32 as i64 {
            return 1 as i32 != 0;
        }
        p = p.offset(clen as isize);
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn buf_has_nulls(mut buf: *mut i8, mut size: idx_t) -> bool {
    *buf.offset(size as isize) = 0 as i32 as i8;
    return strlen(buf) != size as u64;
}
unsafe extern "C" fn file_must_have_nulls(
    mut size: idx_t,
    mut fd: i32,
    mut st: *const stat,
) -> bool {
    if 4 as i32 != 0 as i32 && !seek_failed && usable_st_size(st) as i32 != 0
        && size < (*st).st_size
    {
        let mut cur: off_t = size;
        if 0 as i32 != 0 || fd == 0 as i32 {
            cur = lseek(fd, 0 as i32 as __off_t, 1 as i32);
            if cur < 0 as i32 as i64 {
                return 0 as i32 != 0;
            }
        }
        let mut hole_start: off_t = lseek(fd, cur, 4 as i32);
        if 0 as i32 as i64 <= hole_start {
            if lseek(fd, cur, 0 as i32) < 0 as i32 as i64 {
                suppressible_error(*__errno_location());
            }
            if hole_start < (*st).st_size {
                return 1 as i32 != 0;
            }
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn context_length_arg(mut str: *const i8, mut out: *mut intmax_t) {
    let mut current_block_1: u64;
    match xstrtoimax(
        str,
        0 as *mut *mut i8,
        10 as i32,
        out,
        b"\0" as *const u8 as *const i8,
    ) as u32
    {
        0 | 1 => {
            if 0 as i32 as i64 <= *out {
                current_block_1 = 715039052867723359;
            } else {
                current_block_1 = 14683666992175689893;
            }
        }
        _ => {
            current_block_1 = 14683666992175689893;
        }
    }
    match current_block_1 {
        14683666992175689893 => {
            if ::core::mem::size_of::<C2RustUnnamed_14>() as u64 != 0 {
                error(
                    C2RustUnnamed_1::EXIT_TROUBLE as i32,
                    0 as i32,
                    b"%s: %s\0" as *const u8 as *const i8,
                    str,
                    dcgettext(
                        0 as *const i8,
                        b"invalid context length argument\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    C2RustUnnamed_1::EXIT_TROUBLE as i32,
                    0 as i32,
                    b"%s: %s\0" as *const u8 as *const i8,
                    str,
                    dcgettext(
                        0 as *const i8,
                        b"invalid context length argument\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        _ => {}
    };
}
unsafe extern "C" fn exclude_options(mut command_line: bool) -> i32 {
    return (1 as i32) << 28 as i32
        | (if command_line as i32 != 0 { 0 as i32 } else { (1 as i32) << 30 as i32 });
}
unsafe extern "C" fn skipped_file(
    mut name: *const i8,
    mut command_line: bool,
    mut is_dir: bool,
) -> bool {
    let mut pats: *mut *mut exclude = 0 as *mut *mut exclude;
    if !is_dir {
        pats = excluded_patterns.as_mut_ptr();
    } else if directories as u32 == directories_type::SKIP_DIRECTORIES as i32 as u32 {
        return 1 as i32 != 0
    } else if command_line as i32 != 0 && omit_dot_slash as i32 != 0 {
        return 0 as i32 != 0
    } else {
        pats = excluded_directory_patterns.as_mut_ptr();
    }
    return !(*pats.offset(command_line as isize)).is_null()
        && excluded_file_name(*pats.offset(command_line as isize), name) as i32 != 0;
}
static mut buffer: *mut i8 = 0 as *const i8 as *mut i8;
static mut bufalloc: idx_t = 0;
static mut bufdesc: i32 = 0;
static mut bufbeg: *mut i8 = 0 as *const i8 as *mut i8;
static mut buflim: *mut i8 = 0 as *const i8 as *mut i8;
static mut pagesize: idx_t = 0;
static mut bufoffset: off_t = 0;
static mut after_last_match: off_t = 0;
static mut skip_nuls: bool = false;
static mut skip_empty_lines: bool = false;
static mut totalnl: intmax_t = 0;
unsafe extern "C" fn add_count(mut a: intmax_t, mut b: idx_t) -> intmax_t {
    let mut sum: intmax_t = 0;
    let (fresh5, fresh6) = a.overflowing_add(b);
    *(&mut sum as *mut intmax_t) = fresh5;
    if fresh6 {
        if ::core::mem::size_of::<C2RustUnnamed_12>() as u64 != 0 {
            error(
                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"input is too large to count\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"input is too large to count\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    return sum;
}
unsafe extern "C" fn all_zeros(mut buf: *const i8, mut size: idx_t) -> bool {
    let mut p: *const i8 = buf;
    while p < buf.offset(size as isize) {
        if *p != 0 {
            return 0 as i32 != 0;
        }
        p = p.offset(1);
        p;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn reset(mut fd: i32, mut st: *const stat) -> bool {
    buflim = if (buffer.offset(1 as i32 as isize) as uintptr_t)
        .wrapping_rem(pagesize as u64) == 0 as i32 as u64
    {
        buffer.offset(1 as i32 as isize)
    } else {
        buffer
            .offset(1 as i32 as isize)
            .offset(
                (pagesize as u64)
                    .wrapping_sub(
                        (buffer.offset(1 as i32 as isize) as uintptr_t)
                            .wrapping_rem(pagesize as u64),
                    ) as isize,
            )
    };
    bufbeg = buflim;
    *bufbeg.offset(-(1 as i32) as isize) = eolbyte;
    bufdesc = fd;
    bufoffset = if fd == 0 as i32 {
        lseek(fd, 0 as i32 as __off_t, 1 as i32)
    } else {
        0 as i32 as i64
    };
    seek_failed = bufoffset < 0 as i32 as i64;
    seek_data_failed = seek_failed;
    if seek_failed {
        if *__errno_location() != 29 as i32 {
            suppressible_error(*__errno_location());
            return 0 as i32 != 0;
        }
        bufoffset = 0 as i32 as off_t;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn fillbuf(mut save: idx_t, mut st: *const stat) -> bool {
    let mut readbuf: *mut i8 = 0 as *mut i8;
    let mut min_after_buflim: idx_t = pagesize
        + C2RustUnnamed_13::uword_size as i32 as i64;
    if min_after_buflim <= buffer.offset(bufalloc as isize).offset_from(buflim) as i64 {
        readbuf = buflim;
    } else {
        let mut newbuf: *mut i8 = 0 as *mut i8;
        let mut minsize: idx_t = save + pagesize;
        let mut incr_min: ptrdiff_t = minsize - bufalloc + min_after_buflim;
        if incr_min <= 0 as i32 as i64 {
            newbuf = buffer;
        } else {
            let mut alloc_max: ptrdiff_t = -(1 as i32) as ptrdiff_t;
            if usable_st_size(st) {
                let mut to_be_read: off_t = (*st).st_size - bufoffset;
                let mut a: ptrdiff_t = 0;
                if 0 as i32 as i64 <= to_be_read
                    && {
                        let (fresh7, fresh8) = to_be_read
                            .overflowing_add(save + min_after_buflim);
                        *(&mut a as *mut ptrdiff_t) = fresh7;
                        !fresh8
                    }
                {
                    alloc_max = if a > bufalloc + incr_min {
                        a
                    } else {
                        bufalloc + incr_min
                    };
                }
            }
            newbuf = xpalloc(
                0 as *mut libc::c_void,
                &mut bufalloc,
                incr_min,
                alloc_max,
                1 as i32 as idx_t,
            ) as *mut i8;
        }
        readbuf = if (newbuf.offset(1 as i32 as isize).offset(save as isize)
            as uintptr_t)
            .wrapping_rem(pagesize as u64) == 0 as i32 as u64
        {
            newbuf.offset(1 as i32 as isize).offset(save as isize)
        } else {
            newbuf
                .offset(1 as i32 as isize)
                .offset(save as isize)
                .offset(
                    (pagesize as u64)
                        .wrapping_sub(
                            (newbuf.offset(1 as i32 as isize).offset(save as isize)
                                as uintptr_t)
                                .wrapping_rem(pagesize as u64),
                        ) as isize,
                )
        };
        let mut moved: idx_t = save + 1 as i32 as i64;
        memmove(
            readbuf.offset(-(moved as isize)) as *mut libc::c_void,
            buflim.offset(-(moved as isize)) as *const libc::c_void,
            moved as u64,
        );
        if (0 as i32 as i64) < incr_min {
            rpl_free(buffer as *mut libc::c_void);
            buffer = newbuf;
        }
    }
    bufbeg = readbuf.offset(-(save as isize));
    clear_asan_poison();
    let mut readsize: idx_t = buffer
        .offset(bufalloc as isize)
        .offset(-(C2RustUnnamed_13::uword_size as i32 as isize))
        .offset_from(readbuf) as i64;
    readsize -= readsize % pagesize;
    let mut fillsize: idx_t = 0;
    let mut cc: bool = 1 as i32 != 0;
    loop {
        fillsize = safe_read(bufdesc, readbuf as *mut libc::c_void, readsize as size_t)
            as idx_t;
        if fillsize as u64 == -(1 as i32) as size_t {
            fillsize = 0 as i32 as idx_t;
            cc = 0 as i32 != 0;
        }
        bufoffset += fillsize;
        if (fillsize == 0 as i32 as i64) as i32 | !skip_nuls as i32 != 0
            || !all_zeros(readbuf, fillsize)
        {
            break;
        }
        totalnl = add_count(totalnl, fillsize);
        if 3 as i32 != 0 as i32 && !seek_data_failed {
            let mut data_start: off_t = lseek(bufdesc, bufoffset, 3 as i32);
            if data_start < 0 as i32 as i64 && *__errno_location() == 6 as i32
                && usable_st_size(st) as i32 != 0 && bufoffset < (*st).st_size
            {
                data_start = lseek(bufdesc, 0 as i32 as __off_t, 2 as i32);
            }
            if data_start < 0 as i32 as i64 {
                seek_data_failed = 1 as i32 != 0;
            } else {
                totalnl = add_count(totalnl, data_start - bufoffset);
                bufoffset = data_start;
            }
        }
    }
    buflim = readbuf.offset(fillsize as isize);
    memset(
        buflim as *mut libc::c_void,
        0 as i32,
        C2RustUnnamed_13::uword_size as i32 as u64,
    );
    asan_poison(
        buflim.offset(C2RustUnnamed_13::uword_size as i32 as isize)
            as *const libc::c_void,
        bufalloc - buflim.offset_from(buffer) as i64
            - C2RustUnnamed_13::uword_size as i32 as i64,
    );
    return cc;
}
static mut binary_files: C2RustUnnamed_10 = C2RustUnnamed_10::BINARY_BINARY_FILES;
static mut list_files: C2RustUnnamed_9 = C2RustUnnamed_9::LISTFILES_NONE;
static mut out_file: i32 = 0;
static mut filename_mask: i32 = 0;
static mut out_quiet: bool = false;
static mut out_invert: bool = false;
static mut out_line: bool = false;
static mut out_byte: bool = false;
static mut out_before: intmax_t = 0;
static mut out_after: intmax_t = 0;
static mut count_matches: bool = false;
static mut max_count: intmax_t = 0;
static mut line_buffered: bool = false;
static mut label: *mut i8 = 0 as *const i8 as *mut i8;
static mut totalcc: intmax_t = 0;
static mut lastnl: *const i8 = 0 as *const i8;
static mut lastout: *mut i8 = 0 as *const i8 as *mut i8;
static mut outleft: intmax_t = 0;
static mut pending: intmax_t = 0;
static mut done_on_match: bool = false;
static mut exit_on_match: bool = false;
static mut dev_null_output: bool = false;
static mut binary: bool = false;
unsafe extern "C" fn nlscan(mut lim: *const i8) {
    let mut newlines: idx_t = 0 as i32 as idx_t;
    let mut beg: *const i8 = lastnl;
    while beg < lim {
        beg = memchr(
            beg as *const libc::c_void,
            eolbyte as i32,
            lim.offset_from(beg) as i64 as u64,
        ) as *const i8;
        if beg.is_null() {
            break;
        }
        newlines += 1;
        newlines;
        beg = beg.offset(1);
        beg;
    }
    totalnl = add_count(totalnl, newlines);
    lastnl = lim;
}
unsafe extern "C" fn print_filename() {
    pr_sgr_start_if(filename_color);
    fputs_errno(input_filename());
    pr_sgr_end_if(filename_color);
}
unsafe extern "C" fn print_sep(mut sep: i8) {
    pr_sgr_start_if(sep_color);
    putchar_errno(sep as i32);
    pr_sgr_end_if(sep_color);
}
unsafe extern "C" fn print_offset(mut pos: intmax_t, mut color: *const i8) {
    pr_sgr_start_if(color);
    printf_errno(b"%*ld\0" as *const u8 as *const i8, offset_width, pos);
    pr_sgr_end_if(color);
}
unsafe extern "C" fn print_line_head(
    mut beg: *mut i8,
    mut len: idx_t,
    mut lim: *const i8,
    mut sep: i8,
) -> bool {
    if binary_files as u32 != C2RustUnnamed_10::TEXT_BINARY_FILES as i32 as u32 {
        let mut ch: i8 = *beg.offset(len as isize);
        let mut encoding_errors: bool = buf_has_encoding_errors(beg, len);
        *beg.offset(len as isize) = ch;
        if encoding_errors {
            encoding_error_output = 1 as i32 != 0;
            return 0 as i32 != 0;
        }
    }
    if out_file != 0 {
        print_filename();
        if filename_mask != 0 {
            print_sep(sep);
        } else {
            putchar_errno(0 as i32);
        }
    }
    if out_line {
        if lastnl < lim {
            nlscan(beg);
            totalnl = add_count(totalnl, 1 as i32 as idx_t);
            lastnl = lim;
        }
        print_offset(totalnl, line_num_color);
        print_sep(sep);
    }
    if out_byte {
        let mut pos: intmax_t = add_count(totalcc, beg.offset_from(bufbeg) as i64);
        print_offset(pos, byte_num_color);
        print_sep(sep);
    }
    if align_tabs as i32 != 0 && out_file | out_line as i32 | out_byte as i32 != 0
        && len != 0 as i32 as i64
    {
        putchar_errno('\t' as i32);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn print_line_middle(
    mut beg: *mut i8,
    mut lim: *mut i8,
    mut line_color: *const i8,
    mut match_color: *const i8,
) -> *mut i8 {
    let mut match_size: idx_t = 0;
    let mut match_offset: ptrdiff_t = 0;
    let mut cur: *mut i8 = 0 as *mut i8;
    let mut mid: *mut i8 = 0 as *mut i8;
    let mut b: *mut i8 = 0 as *mut i8;
    cur = beg;
    while cur < lim
        && {
            match_offset = execute
                .expect(
                    "non-null function pointer",
                )(
                compiled_pattern,
                beg,
                lim.offset_from(beg) as i64,
                &mut match_size,
                cur,
            );
            0 as i32 as i64 <= match_offset
        }
    {
        b = beg.offset(match_offset as isize);
        if b == lim {
            break;
        }
        if match_size == 0 as i32 as i64 {
            match_size = 1 as i32 as idx_t;
            if mid.is_null() {
                mid = cur;
            }
        } else {
            if only_matching {
                let mut sep: i8 = (if out_invert as i32 != 0 {
                    C2RustUnnamed_5::SEP_CHAR_REJECTED as i32
                } else {
                    C2RustUnnamed_4::SEP_CHAR_SELECTED as i32
                }) as i8;
                if !print_line_head(b, match_size, lim, sep) {
                    return 0 as *mut i8;
                }
            } else {
                pr_sgr_start(line_color);
                if !mid.is_null() {
                    cur = mid;
                    mid = 0 as *mut i8;
                }
                fwrite_errno(
                    cur as *const libc::c_void,
                    1 as i32 as idx_t,
                    b.offset_from(cur) as i64,
                );
            }
            pr_sgr_start_if(match_color);
            fwrite_errno(b as *const libc::c_void, 1 as i32 as idx_t, match_size);
            pr_sgr_end_if(match_color);
            if only_matching {
                putchar_errno(eolbyte as i32);
            }
        }
        cur = b.offset(match_size as isize);
    }
    if only_matching {
        cur = lim;
    } else if !mid.is_null() {
        cur = mid;
    }
    return cur;
}
unsafe extern "C" fn print_line_tail(
    mut beg: *mut i8,
    mut lim: *const i8,
    mut line_color: *const i8,
) -> *mut i8 {
    let mut eol_size: idx_t = 0;
    let mut tail_size: idx_t = 0;
    eol_size = (lim > beg && *lim.offset(-(1 as i32) as isize) as i32 == eolbyte as i32)
        as i32 as idx_t;
    eol_size
        += (lim.offset(-(eol_size as isize)) > beg
            && *lim.offset(-(1 as i32 as i64 + eol_size) as isize) as i32 == '\r' as i32)
            as i32 as i64;
    tail_size = lim.offset(-(eol_size as isize)).offset_from(beg) as i64;
    if tail_size > 0 as i32 as i64 {
        pr_sgr_start(line_color);
        fwrite_errno(beg as *const libc::c_void, 1 as i32 as idx_t, tail_size);
        beg = beg.offset(tail_size as isize);
        pr_sgr_end(line_color);
    }
    return beg;
}
unsafe extern "C" fn prline(mut beg: *mut i8, mut lim: *mut i8, mut sep: i8) {
    let mut matching: bool = false;
    let mut line_color: *const i8 = 0 as *const i8;
    let mut match_color: *const i8 = 0 as *const i8;
    if !only_matching {
        if !print_line_head(
            beg,
            lim.offset_from(beg) as i64 - 1 as i32 as i64,
            lim,
            sep,
        ) {
            return;
        }
    }
    matching = (sep as i32 == C2RustUnnamed_4::SEP_CHAR_SELECTED as i32) as i32
        ^ out_invert as i32 != 0;
    if color_option != 0 {
        line_color = if (sep as i32 == C2RustUnnamed_4::SEP_CHAR_SELECTED as i32) as i32
            ^ (out_invert as i32 != 0 && color_option < 0 as i32) as i32 != 0
        {
            selected_line_color
        } else {
            context_line_color
        };
        match_color = if sep as i32 == C2RustUnnamed_4::SEP_CHAR_SELECTED as i32 {
            selected_match_color
        } else {
            context_match_color
        };
    } else {
        match_color = 0 as *const i8;
        line_color = match_color;
    }
    if only_matching as i32 != 0 && matching as i32 != 0
        || color_option != 0 && (*line_color as i32 != 0 || *match_color as i32 != 0)
    {
        if matching as i32 != 0
            && (only_matching as i32 != 0 || *match_color as i32 != 0)
        {
            beg = print_line_middle(beg, lim, line_color, match_color);
            if beg.is_null() {
                return;
            }
        }
        if !only_matching && *line_color as i32 != 0 {
            beg = print_line_tail(beg, lim, line_color);
        }
    }
    if !only_matching && lim > beg {
        fwrite_errno(
            beg as *const libc::c_void,
            1 as i32 as idx_t,
            lim.offset_from(beg) as i64,
        );
    }
    if line_buffered {
        fflush_errno();
    }
    if stdout_errno != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_11>() as u64 != 0 {
            error(
                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                stdout_errno,
                dcgettext(
                    0 as *const i8,
                    b"write error\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                stdout_errno,
                dcgettext(
                    0 as *const i8,
                    b"write error\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    lastout = lim;
}
unsafe extern "C" fn prpending(mut lim: *const i8) {
    if lastout.is_null() {
        lastout = bufbeg;
    }
    while (0 as i32 as i64) < pending && lastout < lim as *mut i8 {
        let mut nl: *mut i8 = rawmemchr(lastout as *const libc::c_void, eolbyte as i32)
            as *mut i8;
        prline(
            lastout,
            nl.offset(1 as i32 as isize),
            C2RustUnnamed_5::SEP_CHAR_REJECTED as i32 as i8,
        );
        pending -= 1;
        pending;
    }
}
unsafe extern "C" fn prtext(mut beg: *mut i8, mut lim: *mut i8) {
    static mut used: bool = false;
    let mut eol: i8 = eolbyte;
    if !out_quiet && pending > 0 as i32 as i64 {
        prpending(beg);
    }
    let mut p: *mut i8 = beg;
    if !out_quiet {
        let mut bp: *const i8 = if !lastout.is_null() { lastout } else { bufbeg };
        let mut i: intmax_t = 0;
        i = 0 as i32 as intmax_t;
        while i < out_before {
            if p > bp as *mut i8 {
                loop {
                    p = p.offset(-1);
                    p;
                    if !(*p.offset(-(1 as i32) as isize) as i32 != eol as i32) {
                        break;
                    }
                }
            }
            i += 1;
            i;
        }
        if (0 as i32 as i64 <= out_before || 0 as i32 as i64 <= out_after)
            && used as i32 != 0 && p != lastout && !group_separator.is_null()
        {
            pr_sgr_start_if(sep_color);
            fputs_errno(group_separator);
            pr_sgr_end_if(sep_color);
            putchar_errno('\n' as i32);
        }
        while p < beg {
            let mut nl: *mut i8 = rawmemchr(p as *const libc::c_void, eol as i32)
                as *mut i8;
            nl = nl.offset(1);
            nl;
            prline(p, nl, C2RustUnnamed_5::SEP_CHAR_REJECTED as i32 as i8);
            p = nl;
        }
    }
    let mut n: intmax_t = 0;
    if out_invert {
        n = 0 as i32 as intmax_t;
        while p < lim && n < outleft {
            let mut nl_0: *mut i8 = rawmemchr(p as *const libc::c_void, eol as i32)
                as *mut i8;
            nl_0 = nl_0.offset(1);
            nl_0;
            if !out_quiet {
                prline(p, nl_0, C2RustUnnamed_4::SEP_CHAR_SELECTED as i32 as i8);
            }
            p = nl_0;
            n += 1;
            n;
        }
    } else {
        if !out_quiet {
            prline(beg, lim, C2RustUnnamed_4::SEP_CHAR_SELECTED as i32 as i8);
        }
        n = 1 as i32 as intmax_t;
        p = lim;
    }
    after_last_match = bufoffset - buflim.offset_from(p) as i64;
    pending = if out_quiet as i32 != 0 {
        0 as i32 as i64
    } else if 0 as i32 as i64 > out_after {
        0 as i32 as i64
    } else {
        out_after
    };
    used = 1 as i32 != 0;
    outleft -= n;
}
unsafe extern "C" fn zap_nuls(mut p: *mut i8, mut lim: *mut i8, mut eol: i8) {
    if eol != 0 {
        loop {
            *lim = '\0' as i32 as i8;
            p = p.offset(strlen(p) as isize);
            *lim = eol;
            if p == lim {
                break;
            }
            loop {
                let fresh9 = p;
                p = p.offset(1);
                *fresh9 = eol;
                if !(*p == 0) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn grepbuf(mut beg: *mut i8, mut lim: *const i8) -> intmax_t {
    let mut outleft0: intmax_t = outleft;
    let mut endp: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = beg;
    while p < lim as *mut i8 {
        let mut match_size: idx_t = 0;
        let mut match_offset: ptrdiff_t = execute
            .expect(
                "non-null function pointer",
            )(
            compiled_pattern,
            p,
            lim.offset_from(p) as i64,
            &mut match_size,
            0 as *const i8,
        );
        if match_offset < 0 as i32 as i64 {
            if !out_invert {
                break;
            }
            match_offset = lim.offset_from(p) as i64;
            match_size = 0 as i32 as idx_t;
        }
        let mut b: *mut i8 = p.offset(match_offset as isize);
        endp = b.offset(match_size as isize);
        if !out_invert && b == lim as *mut i8 {
            break;
        }
        if !out_invert || p < b {
            let mut prbeg: *mut i8 = if out_invert as i32 != 0 { p } else { b };
            let mut prend: *mut i8 = if out_invert as i32 != 0 { b } else { endp };
            prtext(prbeg, prend);
            if outleft == 0 || done_on_match as i32 != 0 {
                if exit_on_match {
                    exit(if errseen as i32 != 0 { exit_failure } else { 0 as i32 });
                }
                break;
            }
        }
        p = endp;
    }
    return outleft0 - outleft;
}
unsafe extern "C" fn grep(
    mut fd: i32,
    mut st: *const stat,
    mut ineof: *mut bool,
) -> intmax_t {
    let mut current_block: u64;
    let mut nlines: intmax_t = 0;
    let mut i: intmax_t = 0;
    let mut residue: idx_t = 0;
    let mut save: idx_t = 0;
    let mut oldc: i8 = 0;
    let mut beg: *mut i8 = 0 as *mut i8;
    let mut lim: *mut i8 = 0 as *mut i8;
    let mut eol: i8 = eolbyte;
    let mut nul_zapper: i8 = '\0' as i32 as i8;
    let mut done_on_match_0: bool = done_on_match;
    let mut out_quiet_0: bool = out_quiet;
    let mut nlines_first_null: intmax_t = -(1 as i32) as intmax_t;
    if !reset(fd, st) {
        return 0 as i32 as intmax_t;
    }
    totalcc = 0 as i32 as intmax_t;
    lastout = 0 as *mut i8;
    totalnl = 0 as i32 as intmax_t;
    outleft = max_count;
    after_last_match = 0 as i32 as off_t;
    pending = 0 as i32 as intmax_t;
    skip_nuls = skip_empty_lines as i32 != 0 && eol == 0;
    encoding_error_output = 0 as i32 != 0;
    nlines = 0 as i32 as intmax_t;
    residue = 0 as i32 as idx_t;
    save = 0 as i32 as idx_t;
    if !fillbuf(save, st) {
        suppressible_error(*__errno_location());
        return 0 as i32 as intmax_t;
    }
    offset_width = 0 as i32;
    if align_tabs {
        let mut num: intmax_t = if usable_st_size(st) as i32 != 0 {
            (*st).st_size
        } else {
            9223372036854775807 as i64
        };
        num += (out_line as i32 != 0 && num < 9223372036854775807 as i64) as i32 as i64;
        loop {
            offset_width += 1;
            offset_width;
            num /= 10 as i32 as i64;
            if !(num != 0 as i32 as i64) {
                break;
            }
        }
    }
    let mut firsttime: bool = 1 as i32 != 0;
    loop {
        if nlines_first_null < 0 as i32 as i64 && eol as i32 != 0
            && binary_files as u32 != C2RustUnnamed_10::TEXT_BINARY_FILES as i32 as u32
            && (buf_has_nulls(bufbeg, buflim.offset_from(bufbeg) as i64) as i32 != 0
                || firsttime as i32 != 0
                    && file_must_have_nulls(buflim.offset_from(bufbeg) as i64, fd, st)
                        as i32 != 0)
        {
            if binary_files as u32
                == C2RustUnnamed_10::WITHOUT_MATCH_BINARY_FILES as i32 as u32
            {
                return 0 as i32 as intmax_t;
            }
            if !count_matches {
                out_quiet = 1 as i32 != 0;
                done_on_match = out_quiet;
            }
            nlines_first_null = nlines;
            nul_zapper = eol;
            skip_nuls = skip_empty_lines;
        }
        lastnl = bufbeg;
        if !lastout.is_null() {
            lastout = bufbeg;
        }
        beg = bufbeg.offset(save as isize);
        if beg == buflim {
            *ineof = 1 as i32 != 0;
            current_block = 6560072651652764009;
            break;
        } else {
            zap_nuls(beg, buflim, nul_zapper);
            oldc = *beg.offset(-(1 as i32) as isize);
            *beg.offset(-(1 as i32) as isize) = eol;
            lim = memrchr(
                beg.offset(-(1 as i32 as isize)) as *const libc::c_void,
                eol as i32,
                (buflim.offset_from(beg) as i64 + 1 as i32 as i64) as size_t,
            ) as *mut i8;
            lim = lim.offset(1);
            lim;
            *beg.offset(-(1 as i32) as isize) = oldc;
            if lim == beg {
                lim = beg.offset(-(residue as isize));
            }
            beg = beg.offset(-(residue as isize));
            residue = buflim.offset_from(lim) as i64;
            if beg < lim {
                if outleft != 0 {
                    nlines += grepbuf(beg, lim);
                }
                if pending != 0 {
                    prpending(lim);
                }
                if outleft == 0 && pending == 0
                    || done_on_match as i32 != 0
                        && (if 0 as i32 as i64 > nlines_first_null {
                            0 as i32 as i64
                        } else {
                            nlines_first_null
                        }) < nlines
                {
                    current_block = 1311191223484434307;
                    break;
                }
            }
            i = 0 as i32 as intmax_t;
            beg = lim;
            while i < out_before && beg > bufbeg && beg != lastout {
                i += 1;
                i;
                loop {
                    beg = beg.offset(-1);
                    beg;
                    if !(*beg.offset(-(1 as i32) as isize) as i32 != eol as i32) {
                        break;
                    }
                }
            }
            if beg != lastout {
                lastout = 0 as *mut i8;
            }
            save = lim.offset(residue as isize).offset_from(beg) as i64;
            if out_byte {
                totalcc = add_count(totalcc, buflim.offset_from(bufbeg) as i64 - save);
            }
            if out_line {
                nlscan(beg);
            }
            if !fillbuf(save, st) {
                suppressible_error(*__errno_location());
                current_block = 1311191223484434307;
                break;
            } else {
                firsttime = 0 as i32 != 0;
            }
        }
    }
    match current_block {
        6560072651652764009 => {
            if residue != 0 {
                let fresh10 = buflim;
                buflim = buflim.offset(1);
                *fresh10 = eol;
                if outleft != 0 {
                    nlines
                        += grepbuf(
                            bufbeg.offset(save as isize).offset(-(residue as isize)),
                            buflim,
                        );
                }
                if pending != 0 {
                    prpending(buflim);
                }
            }
        }
        _ => {}
    }
    done_on_match = done_on_match_0;
    out_quiet = out_quiet_0;
    if binary_files as u32 == C2RustUnnamed_10::BINARY_BINARY_FILES as i32 as u32
        && !out_quiet
        && (encoding_error_output as i32 != 0
            || 0 as i32 as i64 <= nlines_first_null && nlines_first_null < nlines)
    {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: binary file matches\0" as *const u8 as *const i8,
                5 as i32,
            ),
            input_filename(),
        );
    }
    return nlines;
}
unsafe extern "C" fn grepdirent(
    mut fts: *mut FTS,
    mut ent: *mut FTSENT,
    mut command_line: bool,
) -> bool {
    let mut follow: bool = false;
    command_line = (command_line as i32 & ((*ent).fts_level == 0 as i32 as i64) as i32)
        as bool;
    if (*ent).fts_info as i32 == 6 as i32 {
        return 1 as i32 != 0;
    }
    if !command_line
        && skipped_file(
            ((*ent).fts_name).as_mut_ptr(),
            0 as i32 != 0,
            (*ent).fts_info as i32 == 1 as i32 || (*ent).fts_info as i32 == 2 as i32
                || (*ent).fts_info as i32 == 4 as i32,
        ) as i32 != 0
    {
        rpl_fts_set(fts, ent, 4 as i32);
        return 1 as i32 != 0;
    }
    filename = (*ent).fts_path;
    if omit_dot_slash as i32 != 0 && *filename.offset(1 as i32 as isize) as i32 != 0 {
        filename = filename.offset(2 as i32 as isize);
    }
    follow = (*fts).fts_options & 0x2 as i32 != 0
        || (*fts).fts_options & 0x1 as i32 != 0 && command_line as i32 != 0;
    match (*ent).fts_info as i32 {
        1 => {
            if directories as u32 == directories_type::RECURSE_DIRECTORIES as i32 as u32
            {
                return 1 as i32 != 0;
            }
            rpl_fts_set(fts, ent, 4 as i32);
        }
        2 => {
            if !suppress_errors {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s: warning: recursive directory loop\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    filename,
                );
            }
            return 1 as i32 != 0;
        }
        4 | 7 | 10 => {
            suppressible_error((*ent).fts_errno);
            return 1 as i32 != 0;
        }
        3 | 11 => {
            if skip_devices(command_line) {
                let mut st: *mut stat = ((*ent).fts_statp).as_mut_ptr();
                let mut st1: stat = stat {
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
                if (*st).st_mode == 0 {
                    let mut flag: i32 = if follow as i32 != 0 {
                        0 as i32
                    } else {
                        0x100 as i32
                    };
                    if fstatat((*fts).fts_cwd_fd, (*ent).fts_accpath, &mut st1, flag)
                        != 0 as i32
                    {
                        suppressible_error(*__errno_location());
                        return 1 as i32 != 0;
                    }
                    st = &mut st1;
                }
                if is_device_mode((*st).st_mode) {
                    return 1 as i32 != 0;
                }
            }
        }
        8 | 13 => {}
        12 | 14 => return 1 as i32 != 0,
        _ => {
            abort();
        }
    }
    return grepfile((*fts).fts_cwd_fd, (*ent).fts_accpath, follow, command_line);
}
unsafe extern "C" fn open_symlink_nofollow_error(mut err: i32) -> bool {
    if err == 40 as i32 || err == 31 as i32 {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn grepfile(
    mut dirdesc: i32,
    mut name: *const i8,
    mut follow: bool,
    mut command_line: bool,
) -> bool {
    let mut oflag: i32 = 0 as i32 | 0o400 as i32
        | (if binary as i32 != 0 { 0 as i32 } else { 0 as i32 })
        | (if follow as i32 != 0 { 0 as i32 } else { 0o400000 as i32 })
        | (if skip_devices(command_line) as i32 != 0 {
            0o4000 as i32
        } else {
            0 as i32
        });
    let mut desc: i32 = openat_safer(dirdesc, name, oflag);
    if desc < 0 as i32 {
        if follow as i32 != 0 || !open_symlink_nofollow_error(*__errno_location()) {
            suppressible_error(*__errno_location());
        }
        return 1 as i32 != 0;
    }
    return grepdesc(desc, command_line);
}
unsafe extern "C" fn drain_input(mut fd: i32, mut st: *const stat) -> bool {
    let mut nbytes: ssize_t = 0;
    if (*st).st_mode & 0o170000 as i32 as u32 == 0o10000 as i32 as u32
        && dev_null_output as i32 != 0
    {
        nbytes = splice(
            fd,
            0 as *mut __off64_t,
            1 as i32,
            0 as *mut __off64_t,
            C2RustUnnamed_15::INITIAL_BUFSIZE as i32 as size_t,
            1 as i32 as u32,
        );
        if 0 as i32 as i64 <= nbytes || *__errno_location() != 22 as i32 {
            while (0 as i32 as i64) < nbytes {
                nbytes = splice(
                    fd,
                    0 as *mut __off64_t,
                    1 as i32,
                    0 as *mut __off64_t,
                    C2RustUnnamed_15::INITIAL_BUFSIZE as i32 as size_t,
                    1 as i32 as u32,
                );
            }
            return nbytes == 0 as i32 as i64;
        }
    }
    loop {
        nbytes = safe_read(fd, buffer as *mut libc::c_void, bufalloc as size_t)
            as ssize_t;
        if !(nbytes != 0) {
            break;
        }
        if nbytes as u64 == -(1 as i32) as size_t {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn finalize_input(mut fd: i32, mut st: *const stat, mut ineof: bool) {
    if fd == 0 as i32
        && (if outleft != 0 {
            (!ineof
                && (seek_failed as i32 != 0
                    || lseek(fd, 0 as i32 as __off_t, 2 as i32) < 0 as i32 as i64
                        && *__errno_location() != 22 as i32) && !drain_input(fd, st))
                as i32
        } else {
            (bufoffset != after_last_match && !seek_failed
                && lseek(fd, after_last_match, 0 as i32) < 0 as i32 as i64) as i32
        }) != 0
    {
        suppressible_error(*__errno_location());
    }
}
unsafe extern "C" fn grepdesc(mut desc: i32, mut command_line: bool) -> bool {
    let mut count: intmax_t = 0;
    let mut status: bool = 1 as i32 != 0;
    let mut ineof: bool = 0 as i32 != 0;
    let mut st: stat = stat {
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
    if fstat(desc, &mut st) != 0 as i32 {
        suppressible_error(*__errno_location());
    } else if !(desc != 0 as i32 && skip_devices(command_line) as i32 != 0
        && is_device_mode(st.st_mode) as i32 != 0)
    {
        if !(desc != 0 as i32 && command_line as i32 != 0
            && skipped_file(
                filename,
                1 as i32 != 0,
                (st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) as i32
                    != 0 as i32,
            ) as i32 != 0)
        {
            if out_file < 0 as i32 {
                out_file = (st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
                    as i32;
            }
            if desc != 0 as i32
                && directories as u32
                    == directories_type::RECURSE_DIRECTORIES as i32 as u32
                && st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
            {
                let mut fts: *mut FTS = 0 as *mut FTS;
                let mut ent: *mut FTSENT = 0 as *mut FTSENT;
                let mut opts: i32 = fts_options
                    & !(if command_line as i32 != 0 { 0 as i32 } else { 0x1 as i32 });
                let mut fts_arg: [*mut i8; 2] = [0 as *mut i8; 2];
                if close(desc) != 0 as i32 {
                    suppressible_error(*__errno_location());
                }
                fts_arg[0 as i32 as usize] = filename as *mut i8;
                fts_arg[1 as i32 as usize] = 0 as *mut i8;
                fts = rpl_fts_open(fts_arg.as_mut_ptr(), opts, None);
                if fts.is_null() {
                    xalloc_die();
                }
                loop {
                    ent = rpl_fts_read(fts);
                    if ent.is_null() {
                        break;
                    }
                    status = (status as i32 & grepdirent(fts, ent, command_line) as i32)
                        as bool;
                }
                if *__errno_location() != 0 {
                    suppressible_error(*__errno_location());
                }
                if rpl_fts_close(fts) != 0 as i32 {
                    suppressible_error(*__errno_location());
                }
                return status;
            }
            if !(desc != 0 as i32
                && (directories as u32
                    == directories_type::SKIP_DIRECTORIES as i32 as u32
                    && st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
                    || (devices as u32 == C2RustUnnamed_8::SKIP_DEVICES as i32 as u32
                        || devices as u32
                            == C2RustUnnamed_8::READ_COMMAND_LINE_DEVICES as i32 as u32
                            && !command_line) && is_device_mode(st.st_mode) as i32 != 0))
            {
                if !out_quiet
                    && list_files as u32 == C2RustUnnamed_9::LISTFILES_NONE as i32 as u32
                    && (1 as i32 as i64) < max_count
                    && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
                    && (st.st_ino == out_stat.st_ino && st.st_dev == out_stat.st_dev)
                {
                    if !suppress_errors {
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"%s: input file is also the output\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            input_filename(),
                        );
                    }
                    errseen = 1 as i32 != 0;
                } else {
                    count = grep(desc, &mut st, &mut ineof);
                    if count_matches {
                        if out_file != 0 {
                            print_filename();
                            if filename_mask != 0 {
                                print_sep(C2RustUnnamed_4::SEP_CHAR_SELECTED as i32 as i8);
                            } else {
                                putchar_errno(0 as i32);
                            }
                        }
                        printf_errno(b"%ld\n\0" as *const u8 as *const i8, count);
                        if line_buffered {
                            fflush_errno();
                        }
                    }
                    status = count == 0;
                    if list_files as u32 == C2RustUnnamed_9::LISTFILES_NONE as i32 as u32
                    {
                        finalize_input(desc, &mut st, ineof);
                    } else if list_files as u32
                        == (if status as i32 != 0 {
                            C2RustUnnamed_9::LISTFILES_NONMATCHING as i32
                        } else {
                            C2RustUnnamed_9::LISTFILES_MATCHING as i32
                        }) as u32
                    {
                        print_filename();
                        putchar_errno('\n' as i32 & filename_mask);
                        if line_buffered {
                            fflush_errno();
                        }
                    }
                }
            }
        }
    }
    if desc != 0 as i32 && close(desc) != 0 as i32 {
        suppressible_error(*__errno_location());
    }
    return status;
}
unsafe extern "C" fn grep_command_line_arg(mut arg: *const i8) -> bool {
    if strcmp(arg, b"-\0" as *const u8 as *const i8) == 0 as i32 {
        filename = label;
        if binary {
            xset_binary_mode(0 as i32, 0 as i32);
        }
        return grepdesc(0 as i32, 1 as i32 != 0);
    } else {
        filename = arg;
        return grepfile(-(100 as i32), arg, 1 as i32 != 0, 1 as i32 != 0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: i32) {
    if status != 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Usage: %s [OPTION]... PATTERNS [FILE]...\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            getprogname(),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Try '%s --help' for more information.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            getprogname(),
        );
    } else {
        printf(
            dcgettext(
                0 as *const i8,
                b"Usage: %s [OPTION]... PATTERNS [FILE]...\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            getprogname(),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"Search for PATTERNS in each FILE.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"Example: %s -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            getprogname(),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is 'binary', 'text', or 'without-match'\n  -a, --text                equivalent to --binary-files=text\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is 'read', 'recurse', or 'skip'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is 'read' or 'skip'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is 'always', 'never', or 'auto'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        printf(
            dcgettext(
                0 as *const i8,
                b"When FILE is '-', read standard input.  With no FILE, read '.' if\nrecursive, '-' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        emit_bug_reporting_address();
    }
    exit(status);
}
static mut matchers: [C2RustUnnamed_16; 6] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_16 {
                name: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [i8; 12],
                >(b"grep\0\0\0\0\0\0\0\0"),
                syntax: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                    | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (1 as i32 as u64) << 1 as i32
                    | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                    | (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32)
                    & !(((((((((((((((((((((((((1 as i32 as u64) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                        | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)) as i32,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut i8,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            idx_t,
                            *mut idx_t,
                            *const i8,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [i8; 12],
                >(b"egrep\0\0\0\0\0\0\0"),
                syntax: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                    | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                    | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                    | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32)
                    & !((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                        | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                            << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)) as i32,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut i8,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            idx_t,
                            *mut idx_t,
                            *const i8,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [i8; 12],
                >(b"fgrep\0\0\0\0\0\0\0"),
                syntax: 0 as i32,
                compile: Some(
                    Fcompile
                        as unsafe extern "C" fn(
                            *mut i8,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    Fexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            idx_t,
                            *mut idx_t,
                            *const i8,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [i8; 12],
                >(b"awk\0\0\0\0\0\0\0\0\0"),
                syntax: (1 as i32 as u64
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                    | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                    | ((1 as i32 as u64) << 1 as i32) << 1 as i32
                    | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) as i32,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut i8,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            idx_t,
                            *mut idx_t,
                            *const i8,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [i8; 12],
                >(b"gawk\0\0\0\0\0\0\0\0"),
                syntax: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                    | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                    | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                    | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | 1 as i32 as u64
                    | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    & !((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                        | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                            << 1 as i32
                        | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                            << 1 as i32) << 1 as i32)) as i32,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut i8,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            idx_t,
                            *mut idx_t,
                            *const i8,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [i8; 12],
                >(b"posixawk\0\0\0\0"),
                syntax: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                    | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                    | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32
                    | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                    | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | 1 as i32 as u64
                    | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32
                    | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut i8,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            idx_t,
                            *mut idx_t,
                            *const i8,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
    ]
};
unsafe extern "C" fn setmatcher(mut m: *const i8, mut matcher: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[C2RustUnnamed_16; 6]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_16>() as u64)
    {
        if strcmp(m, (matchers[i as usize].name).as_ptr()) == 0 as i32 {
            if 0 as i32 <= matcher && matcher != i {
                if ::core::mem::size_of::<C2RustUnnamed_20>() as u64 != 0 {
                    error(
                        C2RustUnnamed_1::EXIT_TROUBLE as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"conflicting matchers specified\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        C2RustUnnamed_1::EXIT_TROUBLE as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"conflicting matchers specified\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            return i;
        }
        i += 1;
        i;
    }
    if strcmp(m, b"perl\0" as *const u8 as *const i8) == 0 as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_19>() as u64 != 0 {
            error(
                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Perl matching not supported in a --disable-perl-regexp build\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Perl matching not supported in a --disable-perl-regexp build\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    if ::core::mem::size_of::<C2RustUnnamed_18>() as u64 != 0 {
        error(
            C2RustUnnamed_1::EXIT_TROUBLE as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"invalid matcher %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            m,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            C2RustUnnamed_1::EXIT_TROUBLE as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"invalid matcher %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            m,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn get_nondigit_option(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut default_context: *mut intmax_t,
) -> i32 {
    static mut prev_digit_optind: i32 = -(1 as i32);
    let mut this_digit_optind: i32 = 0;
    let mut was_digit: bool = false;
    let mut buf: [i8; 25] = [0; 25];
    let mut p: *mut i8 = buf.as_mut_ptr();
    let mut opt: i32 = 0;
    was_digit = 0 as i32 != 0;
    this_digit_optind = optind;
    loop {
        opt = getopt_long(
            argc,
            argv as *mut *mut i8,
            short_options.as_ptr(),
            long_options.as_ptr(),
            0 as *mut i32,
        );
        if !c_isdigit(opt) {
            break;
        }
        if prev_digit_optind != this_digit_optind || !was_digit {
            p = buf.as_mut_ptr();
        } else {
            p = p
                .offset(
                    -((buf[0 as i32 as usize] as i32 == '0' as i32) as i32 as isize),
                );
        }
        if p
            == buf
                .as_mut_ptr()
                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                .offset(-(4 as i32 as isize))
        {
            strcpy(p, b"...\0" as *const u8 as *const i8);
            p = p.offset(3 as i32 as isize);
            break;
        } else {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = opt as i8;
            was_digit = 1 as i32 != 0;
            prev_digit_optind = this_digit_optind;
            this_digit_optind = optind;
        }
    }
    if p != buf.as_mut_ptr() {
        *p = '\0' as i32 as i8;
        context_length_arg(buf.as_mut_ptr(), default_context);
    }
    return opt;
}
unsafe extern "C" fn parse_grep_colors() {
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut val: *mut i8 = 0 as *mut i8;
    p = getenv(b"GREP_COLORS\0" as *const u8 as *const i8);
    if p.is_null() || *p as i32 == '\0' as i32 {
        return;
    }
    q = xstrdup(p);
    name = q;
    val = 0 as *mut i8;
    loop {
        if *q as i32 == ':' as i32 || *q as i32 == '\0' as i32 {
            let mut c: i8 = *q;
            let mut cap: *const color_cap = 0 as *const color_cap;
            let fresh12 = q;
            q = q.offset(1);
            *fresh12 = '\0' as i32 as i8;
            cap = color_dict.as_ptr();
            while !((*cap).name).is_null() {
                if strcmp((*cap).name, name) == 0 as i32 {
                    break;
                }
                cap = cap.offset(1);
                cap;
            }
            if !((*cap).var).is_null() && !val.is_null() {
                *(*cap).var = val;
            }
            if ((*cap).fct).is_some() {
                ((*cap).fct).expect("non-null function pointer")();
            }
            if c as i32 == '\0' as i32 {
                return;
            }
            name = q;
            val = 0 as *mut i8;
        } else if *q as i32 == '=' as i32 {
            if q == name || !val.is_null() {
                return;
            }
            let fresh13 = q;
            q = q.offset(1);
            *fresh13 = '\0' as i32 as i8;
            val = q;
        } else if val.is_null() {
            q = q.offset(1);
            q;
        } else if *q as i32 == ';' as i32 || c_isdigit(*q as i32) as i32 != 0 {
            q = q.offset(1);
            q;
        } else {
            return
        }
    };
}
unsafe extern "C" fn contains_encoding_error(
    mut pat: *const i8,
    mut patlen: idx_t,
) -> bool {
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut charlen: ptrdiff_t = 0;
    let mut i: idx_t = 0 as i32 as idx_t;
    while i < patlen {
        charlen = mb_clen(pat.offset(i as isize), patlen - i, &mut mbs);
        if charlen < 0 as i32 as i64 {
            return 1 as i32 != 0;
        }
        i += charlen;
    }
    return 0 as i32 != 0;
}
static mut ok_fold: [libc::c_schar; 256] = [0; 256];
unsafe extern "C" fn setup_ok_fold() {
    let mut i: i32 = 0 as i32;
    while i < C2RustUnnamed_2::NCHAR as i32 {
        let mut wi: wint_t = localeinfo.sbctowc[i as usize];
        if !(wi == 0xffffffff as u32) {
            let mut ok: i32 = 1 as i32;
            let mut folded: [wchar_t; 32] = [0; 32];
            let mut n: i32 = case_folded_counterparts(wi, folded.as_mut_ptr());
            loop {
                n -= 1;
                if !(0 as i32 <= n) {
                    break;
                }
                let mut buf: [i8; 16] = [0; 16];
                let mut s: mbstate_t = {
                    let mut init = __mbstate_t {
                        __count: 0 as i32,
                        __value: C2RustUnnamed { __wch: 0 },
                    };
                    init
                };
                if !(wcrtomb(buf.as_mut_ptr(), folded[n as usize], &mut s)
                    != 1 as i32 as u64)
                {
                    continue;
                }
                ok = -(1 as i32);
                break;
            }
            ok_fold[i as usize] = ok as libc::c_schar;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn fgrep_icase_charlen(
    mut pat: *const i8,
    mut patlen: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut pat0: u8 = *pat.offset(0 as i32 as isize) as u8;
    if localeinfo.sbctowc[pat0 as usize] != 0xffffffff as u32 {
        return ok_fold[pat0 as usize] as ptrdiff_t;
    }
    let mut wc: wchar_t = 0;
    let mut wn: size_t = rpl_mbrtowc(&mut wc, pat, patlen as size_t, mbs);
    if (16 as i32 as u64) < wn {
        return -(1 as i32) as ptrdiff_t;
    }
    let mut folded: [wchar_t; 32] = [0; 32];
    if case_folded_counterparts(wc as wint_t, folded.as_mut_ptr()) != 0 {
        return -(1 as i32) as ptrdiff_t;
    }
    let mut i: idx_t = wn as idx_t;
    loop {
        i -= 1;
        if !((0 as i32 as i64) < i) {
            break;
        }
        let mut c: u8 = *pat.offset(i as isize) as u8;
        if ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = c as i32;
                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    });
                } else {
                    __res = toupper(c as i32);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(c as i32 as isize);
            }
            __res
        }) != c as i32
        {
            return -(1 as i32) as ptrdiff_t;
        }
    }
    return wn as ptrdiff_t;
}
unsafe extern "C" fn fgrep_icase_available(
    mut pat: *const i8,
    mut patlen: idx_t,
) -> bool {
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut i: idx_t = 0 as i32 as idx_t;
    while i < patlen {
        let mut n: i32 = fgrep_icase_charlen(
            pat.offset(i as isize),
            patlen - i,
            &mut mbs,
        ) as i32;
        if n < 0 as i32 {
            return 0 as i32 != 0;
        }
        i += n as i64;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fgrep_to_grep_pattern(
    mut keys_p: *mut *mut i8,
    mut len_p: *mut idx_t,
) {
    let mut len: idx_t = *len_p;
    let mut keys: *mut i8 = *keys_p;
    let mut mb_state: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut new_keys: *mut i8 = xnmalloc(
        (len + 1 as i32 as i64) as size_t,
        2 as i32 as size_t,
    ) as *mut i8;
    let mut p: *mut i8 = new_keys;
    let mut n: ptrdiff_t = 0;
    while len != 0 {
        n = mb_clen(keys, len, &mut mb_state);
        let mut current_block_9: u64;
        match n {
            -2 => {
                n = len;
                current_block_9 = 17954294594993715973;
            }
            -1 => {
                memset(
                    &mut mb_state as *mut mbstate_t as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<mbstate_t>() as u64,
                );
                n = 1 as i32 as ptrdiff_t;
                current_block_9 = 4058174902840472168;
            }
            1 => {
                current_block_9 = 4058174902840472168;
            }
            _ => {
                current_block_9 = 17954294594993715973;
            }
        }
        match current_block_9 {
            17954294594993715973 => {
                p = mempcpy(
                    p as *mut libc::c_void,
                    keys as *const libc::c_void,
                    n as size_t,
                ) as *mut i8;
            }
            _ => {
                match *keys as i32 {
                    36 | 42 | 46 | 91 | 92 | 94 => {
                        let fresh14 = p;
                        p = p.offset(1);
                        *fresh14 = '\\' as i32 as i8;
                    }
                    _ => {}
                }
                let fresh15 = p;
                p = p.offset(1);
                *fresh15 = *keys;
            }
        }
        keys = keys.offset(n as isize);
        len -= n;
    }
    *p = '\n' as i32 as i8;
    rpl_free(*keys_p as *mut libc::c_void);
    *keys_p = new_keys;
    *len_p = p.offset_from(new_keys) as i64;
}
unsafe extern "C" fn try_fgrep_pattern(
    mut matcher: i32,
    mut keys: *mut i8,
    mut len_p: *mut idx_t,
) -> i32 {
    let mut current_block: u64;
    let mut result: i32 = matcher;
    let mut len: idx_t = *len_p;
    let mut new_keys: *mut i8 = ximalloc(len + 1 as i32 as i64) as *mut i8;
    let mut p: *mut i8 = new_keys;
    let mut q: *const i8 = keys;
    let mut mb_state: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    loop {
        if !(len != 0 as i32 as i64) {
            current_block = 9828876828309294594;
            break;
        }
        match *q as i32 {
            36 | 42 | 46 | 91 | 94 => {
                current_block = 14048763042179579263;
                break;
            }
            40 | 43 | 63 | 123 | 124 => {
                if matcher != C2RustUnnamed_17::G_MATCHER_INDEX as i32 {
                    current_block = 14048763042179579263;
                    break;
                }
            }
            92 => {
                if (1 as i32 as i64) < len {
                    match *q.offset(1 as i32 as isize) as i32 {
                        10 | 66 | 83 | 87 | 39 | 60 | 98 | 115 | 119 | 96 | 62 | 49 | 50
                        | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                            current_block = 14048763042179579263;
                            break;
                        }
                        40 | 43 | 63 | 123 | 124 | 41 => {
                            if matcher == C2RustUnnamed_17::G_MATCHER_INDEX as i32 {
                                current_block = 14048763042179579263;
                                break;
                            }
                        }
                        _ => {}
                    }
                    q = q.offset(1);
                    q;
                    len -= 1;
                    len;
                }
            }
            _ => {}
        }
        let mut clen: ptrdiff_t = if match_icase as i32 != 0 {
            fgrep_icase_charlen(q, len, &mut mb_state)
        } else {
            mb_clen(q, len, &mut mb_state)
        };
        if clen < 0 as i32 as i64 {
            current_block = 14048763042179579263;
            break;
        }
        p = mempcpy(p as *mut libc::c_void, q as *const libc::c_void, clen as size_t)
            as *mut i8;
        q = q.offset(clen as isize);
        len -= clen;
    }
    match current_block {
        9828876828309294594 => {
            if *len_p != p.offset_from(new_keys) as i64 {
                *len_p = p.offset_from(new_keys) as i64;
                let mut keys_end: *mut i8 = mempcpy(
                    keys as *mut libc::c_void,
                    new_keys as *const libc::c_void,
                    p.offset_from(new_keys) as i64 as size_t,
                ) as *mut i8;
                *keys_end = '\n' as i32 as i8;
            }
            result = C2RustUnnamed_17::F_MATCHER_INDEX as i32;
        }
        _ => {}
    }
    rpl_free(new_keys as *mut libc::c_void);
    return result;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut keys: *mut i8 = 0 as *mut i8;
    let mut keycc: idx_t = 0 as i32 as idx_t;
    let mut keyalloc: idx_t = 0 as i32 as idx_t;
    let mut matcher: i32 = -(1 as i32);
    let mut opt: i32 = 0;
    let mut prev_optind: i32 = 0;
    let mut last_recursive: i32 = 0;
    let mut default_context: intmax_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    ::core::ptr::write_volatile(
        &mut exit_failure as *mut i32,
        C2RustUnnamed_1::EXIT_TROUBLE as i32,
    );
    let mut filename_option: i32 = 0 as i32;
    eolbyte = '\n' as i32 as i8;
    filename_mask = !(0 as i32);
    max_count = 9223372036854775807 as i64;
    out_before = -(1 as i32) as intmax_t;
    out_after = out_before;
    default_context = -(1 as i32) as intmax_t;
    only_matching = 0 as i32 != 0;
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"grep\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"grep\0" as *const u8 as *const i8);
    init_localeinfo(&mut localeinfo);
    atexit(Some(clean_up_stdout as unsafe extern "C" fn() -> ()));
    c_stack_action(None);
    last_recursive = 0 as i32;
    pattern_table = hash_initialize(
        0 as i32 as size_t,
        0 as *const Hash_tuning,
        Some(
            hash_pattern as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            compare_patterns
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    if pattern_table.is_null() {
        xalloc_die();
    }
    loop {
        prev_optind = optind;
        opt = get_nondigit_option(argc, argv, &mut default_context);
        if !(opt != -(1 as i32)) {
            break;
        }
        let mut current_block_134: u64;
        match opt {
            65 => {
                context_length_arg(optarg, &mut out_after);
                current_block_134 = 4534765400774009001;
            }
            66 => {
                context_length_arg(optarg, &mut out_before);
                current_block_134 = 4534765400774009001;
            }
            67 => {
                context_length_arg(optarg, &mut default_context);
                current_block_134 = 4534765400774009001;
            }
            68 => {
                if strcmp(optarg, b"read\0" as *const u8 as *const i8) == 0 as i32 {
                    devices = C2RustUnnamed_8::READ_DEVICES;
                } else if strcmp(optarg, b"skip\0" as *const u8 as *const i8) == 0 as i32
                {
                    devices = C2RustUnnamed_8::SKIP_DEVICES;
                } else {
                    if ::core::mem::size_of::<C2RustUnnamed_26>() as u64 != 0 {
                        error(
                            C2RustUnnamed_1::EXIT_TROUBLE as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"unknown devices method\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            C2RustUnnamed_1::EXIT_TROUBLE as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"unknown devices method\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                current_block_134 = 4534765400774009001;
            }
            69 => {
                matcher = setmatcher(b"egrep\0" as *const u8 as *const i8, matcher);
                current_block_134 = 4534765400774009001;
            }
            70 => {
                matcher = setmatcher(b"fgrep\0" as *const u8 as *const i8, matcher);
                current_block_134 = 4534765400774009001;
            }
            80 => {
                matcher = setmatcher(b"perl\0" as *const u8 as *const i8, matcher);
                current_block_134 = 4534765400774009001;
            }
            71 => {
                matcher = setmatcher(b"grep\0" as *const u8 as *const i8, matcher);
                current_block_134 = 4534765400774009001;
            }
            88 => {
                matcher = setmatcher(optarg, matcher);
                current_block_134 = 4534765400774009001;
            }
            72 => {
                filename_option = 1 as i32;
                current_block_134 = 4534765400774009001;
            }
            73 => {
                binary_files = C2RustUnnamed_10::WITHOUT_MATCH_BINARY_FILES;
                current_block_134 = 4534765400774009001;
            }
            84 => {
                align_tabs = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            117 => {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"warning: --unix-byte-offsets (-u) is obsolete\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                current_block_134 = 4534765400774009001;
            }
            86 => {
                show_version = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            97 => {
                binary_files = C2RustUnnamed_10::TEXT_BINARY_FILES;
                current_block_134 = 4534765400774009001;
            }
            98 => {
                out_byte = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            99 => {
                count_matches = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            100 => {
                directories = directories_types[__xargmatch_internal(
                    b"--directories\0" as *const u8 as *const i8,
                    optarg,
                    directories_args.as_ptr(),
                    directories_types.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<directories_type>() as u64,
                    argmatch_die,
                    1 as i32 != 0,
                ) as usize];
                if directories as u32
                    == directories_type::RECURSE_DIRECTORIES as i32 as u32
                {
                    last_recursive = prev_optind;
                }
                current_block_134 = 4534765400774009001;
            }
            101 => {
                let mut cc: idx_t = strlen(optarg) as idx_t;
                let mut shortage: ptrdiff_t = keycc - keyalloc + cc + 1 as i32 as i64;
                if (0 as i32 as i64) < shortage {
                    keys = xpalloc(
                        keys as *mut libc::c_void,
                        &mut keyalloc,
                        shortage,
                        -(1 as i32) as ptrdiff_t,
                        1 as i32 as idx_t,
                    ) as *mut i8;
                    pattern_array = keys;
                }
                let mut keyend: *mut i8 = mempcpy(
                    keys.offset(keycc as isize) as *mut libc::c_void,
                    optarg as *const libc::c_void,
                    cc as size_t,
                ) as *mut i8;
                *keyend = '\n' as i32 as i8;
                keycc = update_patterns(
                    keys,
                    keycc,
                    keycc + cc + 1 as i32 as i64,
                    b"\0" as *const u8 as *const i8,
                );
                current_block_134 = 4534765400774009001;
            }
            102 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const i8) == 0 as i32 {
                    if binary {
                        xset_binary_mode(0 as i32, 0 as i32);
                    }
                    fp = stdin;
                } else {
                    fp = rpl_fopen(
                        optarg,
                        if binary as i32 != 0 {
                            b"rb\0" as *const u8 as *const i8
                        } else {
                            b"r\0" as *const u8 as *const i8
                        },
                    );
                    if fp.is_null() {
                        if ::core::mem::size_of::<C2RustUnnamed_25>() as u64 != 0 {
                            error(
                                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const i8,
                                optarg,
                            );
                            if 0 as i32 != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const i8,
                                optarg,
                            );
                            if 0 as i32 != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                let mut newkeycc: idx_t = keycc;
                let mut cc_0: idx_t = 0;
                loop {
                    let mut shortage_0: ptrdiff_t = newkeycc - keyalloc
                        + 2 as i32 as i64;
                    if (0 as i32 as i64) < shortage_0 {
                        keys = xpalloc(
                            keys as *mut libc::c_void,
                            &mut keyalloc,
                            shortage_0,
                            -(1 as i32) as ptrdiff_t,
                            1 as i32 as idx_t,
                        ) as *mut i8;
                        pattern_array = keys;
                    }
                    cc_0 = (if 0 != 0 && 0 != 0
                        && (1 as i32 as size_t)
                            .wrapping_mul(
                                (keyalloc - (newkeycc + 1 as i32 as i64)) as size_t,
                            ) <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
                    {
                        ({
                            let mut __ptr: *mut i8 = keys.offset(newkeycc as isize);
                            let mut __stream: *mut FILE = fp;
                            let mut __cnt: size_t = 0;
                            __cnt = (1 as i32 as size_t)
                                .wrapping_mul(
                                    (keyalloc - (newkeycc + 1 as i32 as i64)) as size_t,
                                );
                            while __cnt > 0 as i32 as u64 {
                                let mut __c: i32 = if ((*__stream)._IO_read_ptr
                                    >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                {
                                    __uflow(__stream)
                                } else {
                                    let fresh16 = (*__stream)._IO_read_ptr;
                                    (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                        .offset(1);
                                    *(fresh16 as *mut u8) as i32
                                };
                                if __c == -(1 as i32) {
                                    break;
                                }
                                let fresh17 = __ptr;
                                __ptr = __ptr.offset(1);
                                *fresh17 = __c as i8;
                                __cnt = __cnt.wrapping_sub(1);
                                __cnt;
                            }
                            (1 as i32 as size_t)
                                .wrapping_mul(
                                    (keyalloc - (newkeycc + 1 as i32 as i64)) as size_t,
                                )
                                .wrapping_sub(__cnt)
                                .wrapping_div(1 as i32 as size_t)
                        })
                    } else if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                        || 0 != 0
                            && (keyalloc - (newkeycc + 1 as i32 as i64)) as size_t
                                == 0 as i32 as u64
                    {
                        0 as i32 as size_t
                    } else {
                        fread_unlocked(
                            keys.offset(newkeycc as isize) as *mut libc::c_void,
                            1 as i32 as size_t,
                            (keyalloc - (newkeycc + 1 as i32 as i64)) as size_t,
                            fp,
                        )
                    }) as idx_t;
                    if cc_0 == 0 as i32 as i64 {
                        break;
                    }
                    newkeycc += cc_0;
                }
                let mut err: i32 = *__errno_location();
                if ferror_unlocked(fp) == 0 {
                    err = 0 as i32;
                    if fp == stdin {
                        clearerr_unlocked(fp);
                    } else if fclose(fp) != 0 as i32 {
                        err = *__errno_location();
                    }
                }
                if err != 0 {
                    if ::core::mem::size_of::<C2RustUnnamed_24>() as u64 != 0 {
                        error(
                            C2RustUnnamed_1::EXIT_TROUBLE as i32,
                            err,
                            b"%s\0" as *const u8 as *const i8,
                            optarg,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            C2RustUnnamed_1::EXIT_TROUBLE as i32,
                            err,
                            b"%s\0" as *const u8 as *const i8,
                            optarg,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if newkeycc != keycc
                    && *keys.offset((newkeycc - 1 as i32 as i64) as isize) as i32
                        != '\n' as i32
                {
                    let fresh18 = newkeycc;
                    newkeycc = newkeycc + 1;
                    *keys.offset(fresh18 as isize) = '\n' as i32 as i8;
                }
                keycc = update_patterns(keys, keycc, newkeycc, optarg);
                current_block_134 = 4534765400774009001;
            }
            104 => {
                filename_option = -(1 as i32);
                current_block_134 = 4534765400774009001;
            }
            105 | 121 => {
                match_icase = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            137 => {
                match_icase = 0 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            76 => {
                list_files = C2RustUnnamed_9::LISTFILES_NONMATCHING;
                current_block_134 = 4534765400774009001;
            }
            108 => {
                list_files = C2RustUnnamed_9::LISTFILES_MATCHING;
                current_block_134 = 4534765400774009001;
            }
            109 => {
                match xstrtoimax(
                    optarg,
                    0 as *mut *mut i8,
                    10 as i32,
                    &mut max_count,
                    b"\0" as *const u8 as *const i8,
                ) as u32
                {
                    0 | 1 => {}
                    _ => {
                        if ::core::mem::size_of::<C2RustUnnamed_23>() as u64 != 0 {
                            error(
                                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                                0 as i32,
                                dcgettext(
                                    0 as *const i8,
                                    b"invalid max count\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            if 0 as i32 != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                                0 as i32,
                                dcgettext(
                                    0 as *const i8,
                                    b"invalid max count\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            if 0 as i32 != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                current_block_134 = 4534765400774009001;
            }
            110 => {
                out_line = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            111 => {
                only_matching = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            113 => {
                exit_on_match = 1 as i32 != 0;
                ::core::ptr::write_volatile(&mut exit_failure as *mut i32, 0 as i32);
                current_block_134 = 4534765400774009001;
            }
            82 => {
                fts_options = C2RustUnnamed_7::basic_fts_options as i32 | 0x2 as i32;
                current_block_134 = 10748006672221819954;
            }
            114 => {
                current_block_134 = 10748006672221819954;
            }
            115 => {
                suppress_errors = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            118 => {
                out_invert = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            119 => {
                wordinit();
                match_words = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            120 => {
                match_lines = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            90 => {
                filename_mask = 0 as i32;
                current_block_134 = 4534765400774009001;
            }
            122 => {
                eolbyte = '\0' as i32 as i8;
                current_block_134 = 4534765400774009001;
            }
            128 => {
                if strcmp(optarg, b"binary\0" as *const u8 as *const i8) == 0 as i32 {
                    binary_files = C2RustUnnamed_10::BINARY_BINARY_FILES;
                } else if strcmp(optarg, b"text\0" as *const u8 as *const i8) == 0 as i32
                {
                    binary_files = C2RustUnnamed_10::TEXT_BINARY_FILES;
                } else if strcmp(optarg, b"without-match\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    binary_files = C2RustUnnamed_10::WITHOUT_MATCH_BINARY_FILES;
                } else {
                    if ::core::mem::size_of::<C2RustUnnamed_22>() as u64 != 0 {
                        error(
                            C2RustUnnamed_1::EXIT_TROUBLE as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"unknown binary-files type\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            C2RustUnnamed_1::EXIT_TROUBLE as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"unknown binary-files type\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                current_block_134 = 4534765400774009001;
            }
            129 => {
                if !optarg.is_null() {
                    if c_strcasecmp(optarg, b"always\0" as *const u8 as *const i8) == 0
                        || c_strcasecmp(optarg, b"yes\0" as *const u8 as *const i8) == 0
                        || c_strcasecmp(optarg, b"force\0" as *const u8 as *const i8)
                            == 0
                    {
                        color_option = 1 as i32;
                    } else if c_strcasecmp(optarg, b"never\0" as *const u8 as *const i8)
                        == 0
                        || c_strcasecmp(optarg, b"no\0" as *const u8 as *const i8) == 0
                        || c_strcasecmp(optarg, b"none\0" as *const u8 as *const i8) == 0
                    {
                        color_option = 0 as i32;
                    } else if c_strcasecmp(optarg, b"auto\0" as *const u8 as *const i8)
                        == 0
                        || c_strcasecmp(optarg, b"tty\0" as *const u8 as *const i8) == 0
                        || c_strcasecmp(optarg, b"if-tty\0" as *const u8 as *const i8)
                            == 0
                    {
                        color_option = 2 as i32;
                    } else {
                        show_help = 1 as i32;
                    }
                } else {
                    color_option = 2 as i32;
                }
                current_block_134 = 4534765400774009001;
            }
            131 | 134 => {
                let mut cmd: i32 = 0 as i32;
                while cmd < 2 as i32 {
                    if (excluded_patterns[cmd as usize]).is_null() {
                        excluded_patterns[cmd as usize] = new_exclude();
                    }
                    add_exclude(
                        excluded_patterns[cmd as usize],
                        optarg,
                        (if opt == C2RustUnnamed_6::INCLUDE_OPTION as i32 {
                            (1 as i32) << 29 as i32
                        } else {
                            0 as i32
                        }) | exclude_options(cmd != 0),
                    );
                    cmd += 1;
                    cmd;
                }
                current_block_134 = 4534765400774009001;
            }
            132 => {
                let mut cmd_0: i32 = 0 as i32;
                while cmd_0 < 2 as i32 {
                    if (excluded_patterns[cmd_0 as usize]).is_null() {
                        excluded_patterns[cmd_0 as usize] = new_exclude();
                    }
                    if add_exclude_file(
                        Some(
                            add_exclude
                                as unsafe extern "C" fn(*mut exclude, *const i8, i32) -> (),
                        ),
                        excluded_patterns[cmd_0 as usize],
                        optarg,
                        exclude_options(cmd_0 != 0),
                        '\n' as i32 as i8,
                    ) != 0 as i32
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_21>() as u64 != 0 {
                            error(
                                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const i8,
                                optarg,
                            );
                            if 0 as i32 != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                C2RustUnnamed_1::EXIT_TROUBLE as i32,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const i8,
                                optarg,
                            );
                            if 0 as i32 != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    cmd_0 += 1;
                    cmd_0;
                }
                current_block_134 = 4534765400774009001;
            }
            130 => {
                strip_trailing_slashes(optarg);
                let mut cmd_1: i32 = 0 as i32;
                while cmd_1 < 2 as i32 {
                    if (excluded_directory_patterns[cmd_1 as usize]).is_null() {
                        excluded_directory_patterns[cmd_1 as usize] = new_exclude();
                    }
                    add_exclude(
                        excluded_directory_patterns[cmd_1 as usize],
                        optarg,
                        exclude_options(cmd_1 != 0),
                    );
                    cmd_1 += 1;
                    cmd_1;
                }
                current_block_134 = 4534765400774009001;
            }
            133 => {
                group_separator = optarg;
                current_block_134 = 4534765400774009001;
            }
            135 => {
                line_buffered = 1 as i32 != 0;
                current_block_134 = 4534765400774009001;
            }
            136 => {
                label = optarg;
                current_block_134 = 4534765400774009001;
            }
            85 | 0 => {
                current_block_134 = 4534765400774009001;
            }
            _ => {
                usage(C2RustUnnamed_1::EXIT_TROUBLE as i32);
                current_block_134 = 4534765400774009001;
            }
        }
        match current_block_134 {
            10748006672221819954 => {
                directories = directories_type::RECURSE_DIRECTORIES;
                last_recursive = prev_optind;
            }
            _ => {}
        }
    }
    if show_version {
        version_etc(
            stdout,
            getprogname(),
            b"GNU grep\0" as *const u8 as *const i8,
            b"3.11\0" as *const u8 as *const i8,
            0 as *mut libc::c_void as *mut i8,
        );
        puts(
            dcgettext(
                0 as *const i8,
                b"Written by Mike Haertel and others; see\n<https://git.savannah.gnu.org/cgit/grep.git/tree/AUTHORS>.\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if show_help != 0 {
        usage(0 as i32);
    }
    if !keys.is_null() {
        if keycc == 0 as i32 as i64 {
            out_invert = (out_invert as i32 ^ 1 as i32) as bool;
            match_words = 0 as i32 != 0;
            match_lines = match_words;
            let fresh19 = keycc;
            keycc = keycc + 1;
            *keys.offset(fresh19 as isize) = '\n' as i32 as i8;
        }
    } else if optind < argc {
        let fresh20 = optind;
        optind = optind + 1;
        let mut pat: *const i8 = *argv.offset(fresh20 as isize);
        let mut skip_bs: bool = matcher != C2RustUnnamed_17::F_MATCHER_INDEX as i32
            && *pat.offset(0 as i32 as isize) as i32 == '\\' as i32
            && *pat.offset(1 as i32 as isize) as i32 == '-' as i32;
        keys = xstrdup(pat.offset(skip_bs as i32 as isize));
        pattern_array = keys;
        let mut patlen: idx_t = strlen(keys) as idx_t;
        *keys.offset(patlen as isize) = '\n' as i32 as i8;
        keycc = update_patterns(
            keys,
            0 as i32 as idx_t,
            patlen + 1 as i32 as i64,
            b"\0" as *const u8 as *const i8,
        );
    } else {
        usage(C2RustUnnamed_1::EXIT_TROUBLE as i32);
    }
    keycc -= 1;
    keycc;
    hash_free(pattern_table);
    let mut possibly_tty: bool = 0 as i32 != 0;
    let mut tmp_stat: stat = stat {
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
    if !exit_on_match && fstat(1 as i32, &mut tmp_stat) == 0 as i32 {
        if tmp_stat.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
            out_stat = tmp_stat;
        } else if tmp_stat.st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32 {
            let mut null_stat: stat = stat {
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
            if stat(b"/dev/null\0" as *const u8 as *const i8, &mut null_stat) == 0 as i32
                && (tmp_stat.st_ino == null_stat.st_ino
                    && tmp_stat.st_dev == null_stat.st_dev)
            {
                dev_null_output = 1 as i32 != 0;
            } else {
                possibly_tty = 1 as i32 != 0;
            }
        }
    }
    if exit_on_match as i32 | dev_null_output as i32 != 0 {
        list_files = C2RustUnnamed_9::LISTFILES_NONE;
    }
    if exit_on_match as i32 | dev_null_output as i32 != 0
        || list_files as u32 != C2RustUnnamed_9::LISTFILES_NONE as i32 as u32
    {
        count_matches = 0 as i32 != 0;
        done_on_match = 1 as i32 != 0;
    }
    out_quiet = count_matches as i32 | done_on_match as i32 != 0;
    if out_after < 0 as i32 as i64 {
        out_after = default_context;
    }
    if out_before < 0 as i32 as i64 {
        out_before = default_context;
    }
    if (max_count == 0 as i32 as i64
        || keycc == 0 as i32 as i64 && out_invert as i32 != 0 && !match_lines
            && !match_words)
        && list_files as u32 != C2RustUnnamed_9::LISTFILES_NONMATCHING as i32 as u32
    {
        return 1 as i32;
    }
    if color_option == 2 as i32 {
        color_option = (possibly_tty as i32 != 0 && should_colorize() != 0
            && isatty(1 as i32) != 0) as i32;
    }
    init_colorize();
    if color_option != 0 {
        let mut userval: *mut i8 = getenv(b"GREP_COLOR\0" as *const u8 as *const i8);
        if !userval.is_null() && *userval as i32 != '\0' as i32 {
            let mut q: *mut i8 = userval;
            while *q as i32 == ';' as i32 || c_isdigit(*q as i32) as i32 != 0 {
                if *q.offset(1 as i32 as isize) == 0 {
                    context_match_color = userval;
                    selected_match_color = context_match_color;
                    break;
                } else {
                    q = q.offset(1);
                    q;
                }
            }
        }
        parse_grep_colors();
        if selected_match_color == userval || context_match_color == userval {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                userval,
                userval,
            );
        }
    }
    initialize_unibyte_mask();
    if matcher < 0 as i32 {
        matcher = C2RustUnnamed_17::G_MATCHER_INDEX as i32;
    }
    if matcher == C2RustUnnamed_17::F_MATCHER_INDEX as i32
        || matcher == C2RustUnnamed_17::E_MATCHER_INDEX as i32
        || matcher == C2RustUnnamed_17::G_MATCHER_INDEX as i32
    {
        if match_icase {
            setup_ok_fold();
        }
        if matcher == C2RustUnnamed_17::F_MATCHER_INDEX as i32 {
            if if !localeinfo.multibyte {
                (n_patterns == 1 as i32 as i64 && match_words as i32 != 0) as i32
            } else {
                (contains_encoding_error(keys, keycc) as i32 != 0
                    || match_icase as i32 != 0 && !fgrep_icase_available(keys, keycc))
                    as i32
            } != 0
            {
                fgrep_to_grep_pattern(&mut pattern_array, &mut keycc);
                keys = pattern_array;
                matcher = C2RustUnnamed_17::G_MATCHER_INDEX as i32;
            }
        } else if (1 as i32 as i64) < n_patterns {
            matcher = try_fgrep_pattern(matcher, keys, &mut keycc);
        }
    }
    execute = matchers[matcher as usize].execute;
    compiled_pattern = (matchers[matcher as usize].compile)
        .expect(
            "non-null function pointer",
        )(
        keys,
        keycc,
        matchers[matcher as usize].syntax as reg_syntax_t,
        only_matching as i32 | color_option != 0,
    );
    let mut eolbytes: [i8; 3] = [0 as i32 as i8, eolbyte, 0 as i32 as i8];
    let mut match_size: idx_t = 0;
    skip_empty_lines = (execute
        .expect(
            "non-null function pointer",
        )(
        compiled_pattern,
        eolbytes.as_mut_ptr().offset(1 as i32 as isize),
        1 as i32 as idx_t,
        &mut match_size,
        0 as *const i8,
    ) == 0 as i32 as i64) as i32 == out_invert as i32;
    let mut num_operands: i32 = argc - optind;
    out_file = if filename_option == 0 as i32 && num_operands <= 1 as i32 {
        -((directories as u32 == directories_type::RECURSE_DIRECTORIES as i32 as u32)
            as i32)
    } else {
        (0 as i32 <= filename_option) as i32
    };
    if binary {
        xset_binary_mode(1 as i32, 0 as i32);
    }
    let mut psize: i64 = sysconf(_SC_PAGESIZE as i32);
    if !((0 as i32 as i64) < psize
        && psize
            <= (9223372036854775807 as i64 - C2RustUnnamed_13::uword_size as i32 as i64)
                / 2 as i32 as i64)
    {
        abort();
    }
    pagesize = psize;
    bufalloc = (if (C2RustUnnamed_15::INITIAL_BUFSIZE as i32 as uintptr_t)
        .wrapping_rem(pagesize as u64) == 0 as i32 as u64
    {
        C2RustUnnamed_15::INITIAL_BUFSIZE as i32 as u64
    } else {
        (C2RustUnnamed_15::INITIAL_BUFSIZE as i32 as u64)
            .wrapping_add(
                (pagesize as u64)
                    .wrapping_sub(
                        (C2RustUnnamed_15::INITIAL_BUFSIZE as i32 as uintptr_t)
                            .wrapping_rem(pagesize as u64),
                    ),
            )
    })
        .wrapping_add(pagesize as u64)
        .wrapping_add(C2RustUnnamed_13::uword_size as i32 as u64) as idx_t;
    buffer = ximalloc(bufalloc) as *mut i8;
    if fts_options & 0x2 as i32 != 0
        && devices as u32 == C2RustUnnamed_8::READ_COMMAND_LINE_DEVICES as i32 as u32
    {
        devices = C2RustUnnamed_8::READ_DEVICES;
    }
    let mut files: *const *mut i8 = 0 as *const *mut i8;
    if (0 as i32) < num_operands {
        files = argv.offset(optind as isize);
    } else if directories as u32 == directories_type::RECURSE_DIRECTORIES as i32 as u32
        && (0 as i32) < last_recursive
    {
        static mut cwd_only: [*mut i8; 2] = [
            b".\0" as *const u8 as *const i8 as *mut i8,
            0 as *const i8 as *mut i8,
        ];
        files = cwd_only.as_ptr();
        omit_dot_slash = 1 as i32 != 0;
    } else {
        static mut stdin_only: [*mut i8; 2] = [
            b"-\0" as *const u8 as *const i8 as *mut i8,
            0 as *const i8 as *mut i8,
        ];
        files = stdin_only.as_ptr();
    }
    let mut status: bool = 1 as i32 != 0;
    loop {
        let fresh21 = files;
        files = files.offset(1);
        status = (status as i32 & grep_command_line_arg(*fresh21) as i32) as bool;
        if (*files).is_null() {
            break;
        }
    }
    return if errseen as i32 != 0 {
        C2RustUnnamed_1::EXIT_TROUBLE as i32
    } else {
        status as i32
    };
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