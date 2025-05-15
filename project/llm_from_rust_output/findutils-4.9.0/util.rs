use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

#[repr(C)]
pub struct saved_cwd {
    pub desc: c_int,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct predicate {
    pub pred_func: Option<unsafe extern "C" fn(*const c_char, *mut stat, *mut predicate) -> bool>,
    pub p_name: *const c_char,
    pub p_type: predicate_type,
    pub p_prec: predicate_precedence,
    pub side_effects: bool,
    pub no_default_print: bool,
    pub need_stat: bool,
    pub need_type: bool,
    pub need_inum: bool,
    pub p_cost: EvaluationCost,
    pub est_success_rate: f32,
    pub literal_control_chars: bool,
    pub artificial: bool,
    pub arg_text: *const c_char,
    pub args: C2RustUnnamed_0,
    pub pred_next: *mut predicate,
    pub pred_left: *mut predicate,
    pub pred_right: *mut predicate,
    pub perf: predicate_performance_info,
    pub parser_entry: *const parser_table,
}

#[repr(C)]
pub struct predicate_performance_info {
    pub visits: c_ulong,
    pub successes: c_ulong,
}

#[repr(C)]
pub union C2RustUnnamed_0 {
    pub str_0: *const c_char,
    pub regex: *mut re_pattern_buffer,
    pub exec_vec: exec_val,
    pub numinfo: long_val,
    pub size: size_val,
    pub uid: u32,
    pub gid: u32,
    pub reftime: time_val,
    pub perm: perm_val,
    pub samefileid: samefile_file_id,
    pub types: [bool; 7],
    pub printf_vec: format_val,
    pub scontext: *mut c_char,
}

#[repr(C)]
pub struct format_val {
    pub segment: *mut segment,
    pub stream: *mut FILE,
    pub filename: *const c_char,
    pub dest_is_tty: bool,
    pub quote_opts: *mut quoting_options,
}

#[repr(C)]
pub struct segment {
    pub segkind: SegmentKind,
    pub format_char: [c_char; 2],
    pub text: *mut c_char,
    pub text_len: c_int,
    pub next: *mut segment,
}

#[repr(C)]
pub struct samefile_file_id {
    pub ino: u64,
    pub dev: u64,
    pub fd: c_int,
}

#[repr(C)]
pub struct perm_val {
    pub kind: permissions_type,
    pub val: [u32; 2],
}

#[repr(C)]
pub struct time_val {
    pub xval: xval,
    pub kind: comparison_type,
    pub ts: timespec,
}

#[repr(C)]
pub struct size_val {
    pub kind: comparison_type,
    pub blocksize: c_int,
    pub size: u64,
}

#[repr(C)]
pub struct long_val {
    pub kind: comparison_type,
    pub negative: bool,
    pub l_val: u64,
}

#[repr(C)]
pub struct exec_val {
    pub multiple: bool,
    pub ctl: buildcmd_control,
    pub state: buildcmd_state,
    pub replace_vec: *mut *mut c_char,
    pub num_args: c_int,
    pub close_stdin: bool,
    pub wd_for_exec: *mut saved_cwd,
    pub last_child_status: c_int,
}

#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: c_int,
    pub posix_arg_size_max: usize,
    pub posix_arg_size_min: usize,
    pub arg_max: usize,
    pub max_arg_count: usize,
    pub rplen: usize,
    pub replace_pat: *const c_char,
    pub initial_argc: usize,
    pub exec_callback: Option<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut c_void,
            c_int,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub lines_per_exec: c_ulong,
    pub args_per_exec: usize,
}

#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: usize,
    pub cmd_argv: *mut *mut c_char,
    pub cmd_argv_alloc: usize,
    pub argbuf: *mut c_char,
    pub cmd_argv_chars: usize,
    pub cmd_initial_argv_chars: usize,
    pub usercontext: *mut c_void,
    pub todo: c_int,
    pub dir_fd: c_int,
    pub largest_successful_arg_count: usize,
    pub smallest_failed_arg_count: usize,
}

#[repr(C)]
pub struct state {
    pub curdepth: c_int,
    pub have_stat: bool,
    pub have_type: bool,
    pub type_0: u32,
    pub rel_pathname: *const c_char,
    pub cwd_dir_fd: c_int,
    pub starting_path_length: c_int,
    pub stop_at_current_level: bool,
    pub exit_status: c_int,
    pub execdirs_outstanding: bool,
    pub shared_files: sharefile_handle,
    pub already_issued_stat_error_msg: bool,
}

pub type sharefile_handle = *mut c_void;

#[repr(C)]
pub struct options {
    pub do_dir_first: bool,
    pub explicit_depth: bool,
    pub maxdepth: c_int,
    pub mindepth: c_int,
    pub no_leaf_check: bool,
    pub stay_on_filesystem: bool,
    pub ignore_readdir_race: bool,
    pub literal_control_chars: bool,
    pub warnings: bool,
    pub posixly_correct: bool,
    pub start_time: timespec,
    pub cur_day_start: timespec,
    pub full_days: bool,
    pub output_block_size: c_int,
    pub debug_options: c_ulong,
    pub symlink_handling: SymlinkOption,
    pub xstat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> c_int>,
    pub open_nofollow_available: bool,
    pub regex_options: c_int,
    pub x_getfilecon: Option<
        unsafe extern "C" fn(
            c_int,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub optimisation_level: u16,
    pub err_quoting_style: quoting_style,
    pub files0_from: *const c_char,
    pub ok_prompt_stdin: bool,
}

#[repr(u32)]
pub enum SymlinkOption {
    SYMLINK_NEVER_DEREF = 0,
    SYMLINK_ALWAYS_DEREF = 1,
    SYMLINK_DEREF_ARGSONLY = 2,
}

#[repr(u32)]
pub enum predicate_type {
    NO_TYPE = 0,
    PRIMARY_TYPE = 1,
    UNI_OP = 2,
    BI_OP = 3,
    OPEN_PAREN = 4,
    CLOSE_PAREN = 5,
}

#[repr(u32)]
pub enum predicate_precedence {
    NO_PREC = 0,
    COMMA_PREC = 1,
    OR_PREC = 2,
    AND_PREC = 3,
    NEGATE_PREC = 4,
    MAX_PREC = 5,
}

#[repr(u32)]
pub enum EvaluationCost {
    NeedsNothing = 0,
    NeedsInodeNumber = 1,
    NeedsType = 2,
    NeedsStatInfo = 3,
    NeedsLinkName = 4,
    NeedsAccessInfo = 5,
    NeedsSyncDiskHit = 6,
    NeedsEventualExec = 7,
    NeedsImmediateExec = 8,
    NeedsUserInteraction = 9,
    NeedsUnknown = 10,
    NumEvaluationCosts = 11,
}

#[repr(u32)]
pub enum SegmentKind {
    KIND_PLAIN = 0,
    KIND_STOP = 1,
    KIND_FORMAT = 2,
}

#[repr(u32)]
pub enum permissions_type {
    PERM_AT_LEAST = 0,
    PERM_ANY = 1,
    PERM_EXACT = 2,
}

#[repr(u32)]
pub enum comparison_type {
    COMP_GT = 0,
    COMP_LT = 1,
    COMP_EQ = 2,
}

#[repr(u32)]
pub enum xval {
    XVAL_ATIME = 0,
    XVAL_BIRTHTIME = 1,
    XVAL_CTIME = 2,
    XVAL_MTIME = 3,
    XVAL_TIME = 4,
}

#[repr(u32)]
pub enum quoting_style {
    literal_quoting_style = 0,
    shell_quoting_style = 1,
    shell_always_quoting_style = 2,
    shell_escape_quoting_style = 3,
    shell_escape_always_quoting_style = 4,
    c_quoting_style = 5,
    c_maybe_quoting_style = 6,
    escape_quoting_style = 7,
    locale_quoting_style = 8,
    clocale_quoting_style = 9,
    custom_quoting_style = 10,
}

pub type FILE = _IO_FILE;

#[repr(C)]
pub struct _IO_FILE {
    _flags: c_int,
    _IO_read_ptr: *mut c_char,
    _IO_read_end: *mut c_char,
    _IO_read_base: *mut c_char,
    _IO_write_base: *mut c_char,
    _IO_write_ptr: *mut c_char,
    _IO_write_end: *mut c_char,
    _IO_buf_base: *mut c_char,
    _IO_buf_end: *mut c_char,
    _IO_save_base: *mut c_char,
    _IO_backup_base: *mut c_char,
    _IO_save_end: *mut c_char,
    _markers: *mut _IO_marker,
    _chain: *mut _IO_FILE,
    _fileno: c_int,
    _flags2: c_int,
    _old_offset: i64,
    _cur_column: u16,
    _vtable_offset: i8,
    _shortbuf: [c_char; 1],
    _lock: *mut c_void,
    _offset: i64,
    __pad1: *mut c_void,
    __pad2: *mut c_void,
    __pad3: *mut c_void,
    __pad4: *mut c_void,
    __pad5: usize,
    _mode: c_int,
    _unused2: [c_char; 20],
}

#[repr(C)]
pub struct _IO_marker {
    _next: *mut _IO_marker,
    _sbuf: *mut _IO_FILE,
    _pos: c_int,
}

pub type PRED_FUNC = Option<unsafe extern "C" fn(*const c_char, *mut stat, *mut predicate) -> bool>;
pub type PARSE_FUNC = Option<unsafe extern "C" fn(*const parser_table, *mut *mut c_char, *mut c_int) -> bool>;

#[repr(C)]
pub struct parser_table {
    pub type_0: arg_type,
    pub parser_name: *const c_char,
    pub parser_func: PARSE_FUNC,
    pub pred_func: PRED_FUNC,
}

#[repr(u32)]
pub enum arg_type {
    ARG_OPTION = 0,
    ARG_NOOP = 1,
    ARG_POSITIONAL_OPTION = 2,
    ARG_TEST = 3,
    ARG_SPECIAL_PARSE = 4,
    ARG_PUNCTUATION = 5,
    ARG_ACTION = 6,
}

#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: usize,
    pub used: usize,
    pub syntax: c_ulong,
    pub fastmap: *mut c_char,
    pub translate: *mut u8,
    pub re_nsub: usize,
    pub can_be_null: bool,
    pub regs_allocated: u8,
    pub fastmap_accurate: bool,
    pub no_sub: bool,
    pub not_bol: bool,
    pub not_eol: bool,
    pub newline_anchor: bool,
}

pub type re_dfa_t = c_void;
pub type quoting_options = c_void;

pub static mut stdout: *mut FILE = ptr::null_mut();
pub static mut stderr: *mut FILE = ptr::null_mut();
pub static mut program_name: *const c_char = ptr::null();
pub static mut state: state = state {
    curdepth: 0,
    have_stat: false,
    have_type: false,
    type_0: 0,
    rel_pathname: ptr::null(),
    cwd_dir_fd: 0,
    starting_path_length: 0,
    stop_at_current_level: false,
    exit_status: 0,
    execdirs_outstanding: false,
    shared_files: ptr::null_mut(),
    already_issued_stat_error_msg: false,
};
pub static mut options: options = options {
    do_dir_first: false,
    explicit_depth: false,
    maxdepth: 0,
    mindepth: 0,
    no_leaf_check: false,
    stay_on_filesystem: false,
    ignore_readdir_race: false,
    literal_control_chars: false,
    warnings: false,
    posixly_correct: false,
    start_time: timespec { tv_sec: 0, tv_nsec: 0 },
    cur_day_start: timespec { tv_sec: 0, tv_nsec: 0 },
    full_days: false,
    output_block_size: 0,
    debug_options: 0,
    symlink_handling: SymlinkOption::SYMLINK_NEVER_DEREF,
    xstat: None,
    open_nofollow_available: false,
    regex_options: 0,
    x_getfilecon: None,
    optimisation_level: 0,
    err_quoting_style: quoting_style::locale_quoting_style,
    files0_from: ptr::null(),
    ok_prompt_stdin: false,
};
pub static mut initial_wd: *mut saved_cwd = ptr::null_mut();

#[no_mangle]
pub extern "C" fn insert_primary_withpred(
    entry: *const parser_table,
    pred_func: PRED_FUNC,
    arg: *const c_char,
) -> *mut predicate {
    unsafe {
        let new_pred = get_new_pred_chk_op(entry, arg);
        (*new_pred).pred_func = pred_func;
        (*new_pred).p_name = (*entry).parser_name;
        (*new_pred).args.str_0 = ptr::null();
        (*new_pred).p_type = predicate_type::PRIMARY_TYPE;
        (*new_pred).p_prec = predicate_precedence::NO_PREC;
        new_pred
    }
}

#[no_mangle]
pub extern "C" fn insert_primary(
    entry: *const parser_table,
    arg: *const c_char,
) -> *mut predicate {
    unsafe {
        assert!(!(*entry).pred_func.is_none(), "entry->pred_func != NULL");
        insert_primary_withpred(entry, (*entry).pred_func, arg)
    }
}

#[no_mangle]
pub extern "C" fn insert_primary_noarg(entry: *const parser_table) -> *mut predicate {
    insert_primary(entry, ptr::null())
}

#[no_mangle]
pub extern "C" fn set_stat_placeholders(_p: *mut stat) {}

#[no_mangle]
pub extern "C" fn get_statinfo(
    pathname: *const c_char,
    name: *const c_char,
    p: *mut stat,
) -> c_int {
    unsafe {
        if !state.have_stat {
            set_stat_placeholders(p);
            if let Some(xstat_fn) = options.xstat {
                if xstat_fn(name, p) == 0 {
                    if (*p).st_mode == 0 {
                        error(
                            0,
                            0,
                            b"WARNING: file %s appears to have mode 0000\0".as_ptr() as *const c_char,
                            quotearg_n_style(0, options.err_quoting_style, name),
                        );
                        state.exit_status = 1;
                    }
                } else {
                    if !options.ignore_readdir_race || *__errno_location() != 2 {
                        nonfatal_target_file_error(*__errno_location(), pathname);
                    }
                    return -1;
                }
            }
        }
        state.have_stat = true;
        state.have_type = true;
        state.type_0 = (*p).st_mode;
        0
    }
}

#[no_mangle]
pub extern "C" fn following_