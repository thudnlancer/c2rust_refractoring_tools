#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type argv_iterator;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    pub type quoting_options;
    pub type re_dfa_t;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn ctime(__timer: *const time_t) -> *mut i8;
    fn close(__fd: i32) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fclose(stream: *mut FILE) -> i32;
    fn argv_iter_free(_: *mut argv_iterator);
    fn argv_iter_init_argv(argv: *mut *mut i8) -> *mut argv_iterator;
    fn argv_iter_init_stream(fp: *mut FILE) -> *mut argv_iterator;
    fn argv_iter(_: *mut argv_iterator, _: *mut argv_iter_err) -> *mut i8;
    fn argv_iter_n_args(_: *const argv_iterator) -> size_t;
    fn set_cloexec_flag(desc: i32, value: bool) -> i32;
    fn dup_cloexec(fd: i32) -> i32;
    fn close_stdout();
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn rpl_fts_close(_: *mut FTS) -> i32;
    fn rpl_fts_open(
        _: *const *mut i8,
        _: i32,
        _: Option<unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> i32>,
    ) -> *mut FTS;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: i32) -> i32;
    fn set_program_name(argv0: *const i8);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn xalloc_die();
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn sharefile_init(mode: *const i8) -> sharefile_handle;
    fn debug_stat(file: *const i8, bufp: *mut stat) -> i32;
    fn cleanup();
    fn safely_quote_err_filename(n: i32, arg: *const i8) -> *const i8;
    fn complete_pending_execdirs();
    fn get_eval_tree() -> *mut predicate;
    fn build_expression_tree(
        argc: i32,
        argv: *mut *mut i8,
        end_of_leading_options: i32,
    ) -> *mut predicate;
    fn show_success_rates(node: *const predicate);
    fn record_initial_cwd();
    fn nonfatal_target_file_error(errno_value: i32, name: *const i8);
    fn process_leading_options(argc: i32, argv: *mut *mut i8) -> i32;
    fn set_option_defaults(p: *mut options);
    fn apply_predicate(
        pathname: *const i8,
        stat_buf: *mut stat,
        p: *mut predicate,
    ) -> bool;
    fn digest_mode(
        mode: *mut mode_t,
        pathname: *const i8,
        name: *const i8,
        pstat: *mut stat,
        leaf: bool,
    ) -> bool;
    fn looks_like_expression(arg: *const i8, leading: bool) -> bool;
    static mut options: options;
    static mut state: state;
    fn remember_non_cloexec_fds();
    fn fd_leak_check_is_enabled() -> bool;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
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
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type time_t = __time_t;
pub type size_t = u64;
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
pub type uintmax_t = __uintmax_t;
pub type ptrdiff_t = i64;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum argv_iter_err {
    AI_ERR_OK = 1,
    AI_ERR_EOF,
    AI_ERR_MEM,
    AI_ERR_READ,
}
impl argv_iter_err {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            argv_iter_err::AI_ERR_OK => 1,
            argv_iter_err::AI_ERR_EOF => 2,
            argv_iter_err::AI_ERR_MEM => 3,
            argv_iter_err::AI_ERR_READ => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> argv_iter_err {
        match value {
            1 => argv_iter_err::AI_ERR_OK,
            2 => argv_iter_err::AI_ERR_EOF,
            3 => argv_iter_err::AI_ERR_MEM,
            4 => argv_iter_err::AI_ERR_READ,
            _ => panic!("Invalid value for argv_iter_err: {}", value),
        }
    }
}
impl AddAssign<u32> for argv_iter_err {
    fn add_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for argv_iter_err {
    fn sub_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for argv_iter_err {
    fn mul_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for argv_iter_err {
    fn div_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for argv_iter_err {
    fn rem_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn add(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn sub(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn mul(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn div(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn rem(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
    pub fts_cycle: C2RustUnnamed,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut u8,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
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
pub type sharefile_handle = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate {
    pub pred_func: PRED_FUNC,
    pub p_name: *const i8,
    pub p_type: predicate_type,
    pub p_prec: predicate_precedence,
    pub side_effects: bool,
    pub no_default_print: bool,
    pub need_stat: bool,
    pub need_type: bool,
    pub need_inum: bool,
    pub p_cost: EvaluationCost,
    pub est_success_rate: libc::c_float,
    pub literal_control_chars: bool,
    pub artificial: bool,
    pub arg_text: *const i8,
    pub args: C2RustUnnamed_0,
    pub pred_next: *mut predicate,
    pub pred_left: *mut predicate,
    pub pred_right: *mut predicate,
    pub perf: predicate_performance_info,
    pub parser_entry: *const parser_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_table {
    pub type_0: arg_type,
    pub parser_name: *const i8,
    pub parser_func: PARSE_FUNC,
    pub pred_func: PRED_FUNC,
}
pub type PRED_FUNC = Option<
    unsafe extern "C" fn(*const i8, *mut stat, *mut predicate) -> bool,
>;
pub type PARSE_FUNC = Option<
    unsafe extern "C" fn(*const parser_table, *mut *mut i8, *mut i32) -> bool,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum arg_type {
    ARG_OPTION,
    ARG_NOOP,
    ARG_POSITIONAL_OPTION,
    ARG_TEST,
    ARG_SPECIAL_PARSE,
    ARG_PUNCTUATION,
    ARG_ACTION,
}
impl arg_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            arg_type::ARG_OPTION => 0,
            arg_type::ARG_NOOP => 1,
            arg_type::ARG_POSITIONAL_OPTION => 2,
            arg_type::ARG_TEST => 3,
            arg_type::ARG_SPECIAL_PARSE => 4,
            arg_type::ARG_PUNCTUATION => 5,
            arg_type::ARG_ACTION => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> arg_type {
        match value {
            0 => arg_type::ARG_OPTION,
            1 => arg_type::ARG_NOOP,
            2 => arg_type::ARG_POSITIONAL_OPTION,
            3 => arg_type::ARG_TEST,
            4 => arg_type::ARG_SPECIAL_PARSE,
            5 => arg_type::ARG_PUNCTUATION,
            6 => arg_type::ARG_ACTION,
            _ => panic!("Invalid value for arg_type: {}", value),
        }
    }
}
impl AddAssign<u32> for arg_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for arg_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for arg_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for arg_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for arg_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for arg_type {
    type Output = arg_type;
    fn add(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for arg_type {
    type Output = arg_type;
    fn sub(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for arg_type {
    type Output = arg_type;
    fn mul(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for arg_type {
    type Output = arg_type;
    fn div(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for arg_type {
    type Output = arg_type;
    fn rem(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate_performance_info {
    pub visits: u64,
    pub successes: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub str_0: *const i8,
    pub regex: *mut re_pattern_buffer,
    pub exec_vec: exec_val,
    pub numinfo: long_val,
    pub size: size_val,
    pub uid: uid_t,
    pub gid: gid_t,
    pub reftime: time_val,
    pub perm: perm_val,
    pub samefileid: samefile_file_id,
    pub types: [bool; 7],
    pub printf_vec: format_val,
    pub scontext: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_val {
    pub segment: *mut segment,
    pub stream: *mut FILE,
    pub filename: *const i8,
    pub dest_is_tty: bool,
    pub quote_opts: *mut quoting_options,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub segkind: SegmentKind,
    pub format_char: [i8; 2],
    pub text: *mut i8,
    pub text_len: i32,
    pub next: *mut segment,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum SegmentKind {
    KIND_PLAIN = 0,
    KIND_STOP = 1,
    KIND_FORMAT,
}
impl SegmentKind {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            SegmentKind::KIND_PLAIN => 0,
            SegmentKind::KIND_STOP => 1,
            SegmentKind::KIND_FORMAT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> SegmentKind {
        match value {
            0 => SegmentKind::KIND_PLAIN,
            1 => SegmentKind::KIND_STOP,
            2 => SegmentKind::KIND_FORMAT,
            _ => panic!("Invalid value for SegmentKind: {}", value),
        }
    }
}
impl AddAssign<u32> for SegmentKind {
    fn add_assign(&mut self, rhs: u32) {
        *self = SegmentKind::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for SegmentKind {
    fn sub_assign(&mut self, rhs: u32) {
        *self = SegmentKind::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for SegmentKind {
    fn mul_assign(&mut self, rhs: u32) {
        *self = SegmentKind::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for SegmentKind {
    fn div_assign(&mut self, rhs: u32) {
        *self = SegmentKind::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for SegmentKind {
    fn rem_assign(&mut self, rhs: u32) {
        *self = SegmentKind::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for SegmentKind {
    type Output = SegmentKind;
    fn add(self, rhs: u32) -> SegmentKind {
        SegmentKind::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for SegmentKind {
    type Output = SegmentKind;
    fn sub(self, rhs: u32) -> SegmentKind {
        SegmentKind::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for SegmentKind {
    type Output = SegmentKind;
    fn mul(self, rhs: u32) -> SegmentKind {
        SegmentKind::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for SegmentKind {
    type Output = SegmentKind;
    fn div(self, rhs: u32) -> SegmentKind {
        SegmentKind::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for SegmentKind {
    type Output = SegmentKind;
    fn rem(self, rhs: u32) -> SegmentKind {
        SegmentKind::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct samefile_file_id {
    pub ino: ino_t,
    pub dev: dev_t,
    pub fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perm_val {
    pub kind: permissions_type,
    pub val: [mode_t; 2],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum permissions_type {
    PERM_AT_LEAST,
    PERM_ANY,
    PERM_EXACT,
}
impl permissions_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            permissions_type::PERM_AT_LEAST => 0,
            permissions_type::PERM_ANY => 1,
            permissions_type::PERM_EXACT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> permissions_type {
        match value {
            0 => permissions_type::PERM_AT_LEAST,
            1 => permissions_type::PERM_ANY,
            2 => permissions_type::PERM_EXACT,
            _ => panic!("Invalid value for permissions_type: {}", value),
        }
    }
}
impl AddAssign<u32> for permissions_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = permissions_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for permissions_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = permissions_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for permissions_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = permissions_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for permissions_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = permissions_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for permissions_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = permissions_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for permissions_type {
    type Output = permissions_type;
    fn add(self, rhs: u32) -> permissions_type {
        permissions_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for permissions_type {
    type Output = permissions_type;
    fn sub(self, rhs: u32) -> permissions_type {
        permissions_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for permissions_type {
    type Output = permissions_type;
    fn mul(self, rhs: u32) -> permissions_type {
        permissions_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for permissions_type {
    type Output = permissions_type;
    fn div(self, rhs: u32) -> permissions_type {
        permissions_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for permissions_type {
    type Output = permissions_type;
    fn rem(self, rhs: u32) -> permissions_type {
        permissions_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct time_val {
    pub xval: xval,
    pub kind: comparison_type,
    pub ts: timespec,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum comparison_type {
    COMP_GT,
    COMP_LT,
    COMP_EQ,
}
impl comparison_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            comparison_type::COMP_GT => 0,
            comparison_type::COMP_LT => 1,
            comparison_type::COMP_EQ => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> comparison_type {
        match value {
            0 => comparison_type::COMP_GT,
            1 => comparison_type::COMP_LT,
            2 => comparison_type::COMP_EQ,
            _ => panic!("Invalid value for comparison_type: {}", value),
        }
    }
}
impl AddAssign<u32> for comparison_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = comparison_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for comparison_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = comparison_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for comparison_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = comparison_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for comparison_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = comparison_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for comparison_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = comparison_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for comparison_type {
    type Output = comparison_type;
    fn add(self, rhs: u32) -> comparison_type {
        comparison_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for comparison_type {
    type Output = comparison_type;
    fn sub(self, rhs: u32) -> comparison_type {
        comparison_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for comparison_type {
    type Output = comparison_type;
    fn mul(self, rhs: u32) -> comparison_type {
        comparison_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for comparison_type {
    type Output = comparison_type;
    fn div(self, rhs: u32) -> comparison_type {
        comparison_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for comparison_type {
    type Output = comparison_type;
    fn rem(self, rhs: u32) -> comparison_type {
        comparison_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum xval {
    XVAL_ATIME,
    XVAL_BIRTHTIME,
    XVAL_CTIME,
    XVAL_MTIME,
    XVAL_TIME,
}
impl xval {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            xval::XVAL_ATIME => 0,
            xval::XVAL_BIRTHTIME => 1,
            xval::XVAL_CTIME => 2,
            xval::XVAL_MTIME => 3,
            xval::XVAL_TIME => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> xval {
        match value {
            0 => xval::XVAL_ATIME,
            1 => xval::XVAL_BIRTHTIME,
            2 => xval::XVAL_CTIME,
            3 => xval::XVAL_MTIME,
            4 => xval::XVAL_TIME,
            _ => panic!("Invalid value for xval: {}", value),
        }
    }
}
impl AddAssign<u32> for xval {
    fn add_assign(&mut self, rhs: u32) {
        *self = xval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for xval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = xval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for xval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = xval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for xval {
    fn div_assign(&mut self, rhs: u32) {
        *self = xval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for xval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = xval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for xval {
    type Output = xval;
    fn add(self, rhs: u32) -> xval {
        xval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for xval {
    type Output = xval;
    fn sub(self, rhs: u32) -> xval {
        xval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for xval {
    type Output = xval;
    fn mul(self, rhs: u32) -> xval {
        xval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for xval {
    type Output = xval;
    fn div(self, rhs: u32) -> xval {
        xval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for xval {
    type Output = xval;
    fn rem(self, rhs: u32) -> xval {
        xval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct size_val {
    pub kind: comparison_type,
    pub blocksize: i32,
    pub size: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct long_val {
    pub kind: comparison_type,
    pub negative: bool,
    pub l_val: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_val {
    pub multiple: bool,
    pub ctl: buildcmd_control,
    pub state: buildcmd_state,
    pub replace_vec: *mut *mut i8,
    pub num_args: i32,
    pub close_stdin: bool,
    pub wd_for_exec: *mut saved_cwd,
    pub last_child_status: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum EvaluationCost {
    NeedsNothing,
    NeedsInodeNumber,
    NeedsType,
    NeedsStatInfo,
    NeedsLinkName,
    NeedsAccessInfo,
    NeedsSyncDiskHit,
    NeedsEventualExec,
    NeedsImmediateExec,
    NeedsUserInteraction,
    NeedsUnknown,
    NumEvaluationCosts,
}
impl EvaluationCost {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            EvaluationCost::NeedsNothing => 0,
            EvaluationCost::NeedsInodeNumber => 1,
            EvaluationCost::NeedsType => 2,
            EvaluationCost::NeedsStatInfo => 3,
            EvaluationCost::NeedsLinkName => 4,
            EvaluationCost::NeedsAccessInfo => 5,
            EvaluationCost::NeedsSyncDiskHit => 6,
            EvaluationCost::NeedsEventualExec => 7,
            EvaluationCost::NeedsImmediateExec => 8,
            EvaluationCost::NeedsUserInteraction => 9,
            EvaluationCost::NeedsUnknown => 10,
            EvaluationCost::NumEvaluationCosts => 11,
        }
    }
    fn from_libc_c_uint(value: u32) -> EvaluationCost {
        match value {
            0 => EvaluationCost::NeedsNothing,
            1 => EvaluationCost::NeedsInodeNumber,
            2 => EvaluationCost::NeedsType,
            3 => EvaluationCost::NeedsStatInfo,
            4 => EvaluationCost::NeedsLinkName,
            5 => EvaluationCost::NeedsAccessInfo,
            6 => EvaluationCost::NeedsSyncDiskHit,
            7 => EvaluationCost::NeedsEventualExec,
            8 => EvaluationCost::NeedsImmediateExec,
            9 => EvaluationCost::NeedsUserInteraction,
            10 => EvaluationCost::NeedsUnknown,
            11 => EvaluationCost::NumEvaluationCosts,
            _ => panic!("Invalid value for EvaluationCost: {}", value),
        }
    }
}
impl AddAssign<u32> for EvaluationCost {
    fn add_assign(&mut self, rhs: u32) {
        *self = EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for EvaluationCost {
    fn sub_assign(&mut self, rhs: u32) {
        *self = EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for EvaluationCost {
    fn mul_assign(&mut self, rhs: u32) {
        *self = EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for EvaluationCost {
    fn div_assign(&mut self, rhs: u32) {
        *self = EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for EvaluationCost {
    fn rem_assign(&mut self, rhs: u32) {
        *self = EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for EvaluationCost {
    type Output = EvaluationCost;
    fn add(self, rhs: u32) -> EvaluationCost {
        EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for EvaluationCost {
    type Output = EvaluationCost;
    fn sub(self, rhs: u32) -> EvaluationCost {
        EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for EvaluationCost {
    type Output = EvaluationCost;
    fn mul(self, rhs: u32) -> EvaluationCost {
        EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for EvaluationCost {
    type Output = EvaluationCost;
    fn div(self, rhs: u32) -> EvaluationCost {
        EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for EvaluationCost {
    type Output = EvaluationCost;
    fn rem(self, rhs: u32) -> EvaluationCost {
        EvaluationCost::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum predicate_precedence {
    NO_PREC,
    COMMA_PREC,
    OR_PREC,
    AND_PREC,
    NEGATE_PREC,
    MAX_PREC,
}
impl predicate_precedence {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            predicate_precedence::NO_PREC => 0,
            predicate_precedence::COMMA_PREC => 1,
            predicate_precedence::OR_PREC => 2,
            predicate_precedence::AND_PREC => 3,
            predicate_precedence::NEGATE_PREC => 4,
            predicate_precedence::MAX_PREC => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> predicate_precedence {
        match value {
            0 => predicate_precedence::NO_PREC,
            1 => predicate_precedence::COMMA_PREC,
            2 => predicate_precedence::OR_PREC,
            3 => predicate_precedence::AND_PREC,
            4 => predicate_precedence::NEGATE_PREC,
            5 => predicate_precedence::MAX_PREC,
            _ => panic!("Invalid value for predicate_precedence: {}", value),
        }
    }
}
impl AddAssign<u32> for predicate_precedence {
    fn add_assign(&mut self, rhs: u32) {
        *self = predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for predicate_precedence {
    fn sub_assign(&mut self, rhs: u32) {
        *self = predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for predicate_precedence {
    fn mul_assign(&mut self, rhs: u32) {
        *self = predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for predicate_precedence {
    fn div_assign(&mut self, rhs: u32) {
        *self = predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for predicate_precedence {
    fn rem_assign(&mut self, rhs: u32) {
        *self = predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for predicate_precedence {
    type Output = predicate_precedence;
    fn add(self, rhs: u32) -> predicate_precedence {
        predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for predicate_precedence {
    type Output = predicate_precedence;
    fn sub(self, rhs: u32) -> predicate_precedence {
        predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for predicate_precedence {
    type Output = predicate_precedence;
    fn mul(self, rhs: u32) -> predicate_precedence {
        predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for predicate_precedence {
    type Output = predicate_precedence;
    fn div(self, rhs: u32) -> predicate_precedence {
        predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for predicate_precedence {
    type Output = predicate_precedence;
    fn rem(self, rhs: u32) -> predicate_precedence {
        predicate_precedence::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum predicate_type {
    NO_TYPE,
    PRIMARY_TYPE,
    UNI_OP,
    BI_OP,
    OPEN_PAREN,
    CLOSE_PAREN,
}
impl predicate_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            predicate_type::NO_TYPE => 0,
            predicate_type::PRIMARY_TYPE => 1,
            predicate_type::UNI_OP => 2,
            predicate_type::BI_OP => 3,
            predicate_type::OPEN_PAREN => 4,
            predicate_type::CLOSE_PAREN => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> predicate_type {
        match value {
            0 => predicate_type::NO_TYPE,
            1 => predicate_type::PRIMARY_TYPE,
            2 => predicate_type::UNI_OP,
            3 => predicate_type::BI_OP,
            4 => predicate_type::OPEN_PAREN,
            5 => predicate_type::CLOSE_PAREN,
            _ => panic!("Invalid value for predicate_type: {}", value),
        }
    }
}
impl AddAssign<u32> for predicate_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = predicate_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for predicate_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = predicate_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for predicate_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = predicate_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for predicate_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = predicate_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for predicate_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = predicate_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for predicate_type {
    type Output = predicate_type;
    fn add(self, rhs: u32) -> predicate_type {
        predicate_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for predicate_type {
    type Output = predicate_type;
    fn sub(self, rhs: u32) -> predicate_type {
        predicate_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for predicate_type {
    type Output = predicate_type;
    fn mul(self, rhs: u32) -> predicate_type {
        predicate_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for predicate_type {
    type Output = predicate_type;
    fn div(self, rhs: u32) -> predicate_type {
        predicate_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for predicate_type {
    type Output = predicate_type;
    fn rem(self, rhs: u32) -> predicate_type {
        predicate_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub do_dir_first: bool,
    pub explicit_depth: bool,
    pub maxdepth: i32,
    pub mindepth: i32,
    pub no_leaf_check: bool,
    pub stay_on_filesystem: bool,
    pub ignore_readdir_race: bool,
    pub literal_control_chars: bool,
    pub warnings: bool,
    pub posixly_correct: bool,
    pub start_time: timespec,
    pub cur_day_start: timespec,
    pub full_days: bool,
    pub output_block_size: i32,
    pub debug_options: u64,
    pub symlink_handling: SymlinkOption,
    pub xstat: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
    pub open_nofollow_available: bool,
    pub regex_options: i32,
    pub x_getfilecon: Option<unsafe extern "C" fn(i32, *const i8, *mut *mut i8) -> i32>,
    pub optimisation_level: libc::c_ushort,
    pub err_quoting_style: quoting_style,
    pub files0_from: *const i8,
    pub ok_prompt_stdin: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum SymlinkOption {
    SYMLINK_NEVER_DEREF,
    SYMLINK_ALWAYS_DEREF,
    SYMLINK_DEREF_ARGSONLY,
}
impl SymlinkOption {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            SymlinkOption::SYMLINK_NEVER_DEREF => 0,
            SymlinkOption::SYMLINK_ALWAYS_DEREF => 1,
            SymlinkOption::SYMLINK_DEREF_ARGSONLY => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> SymlinkOption {
        match value {
            0 => SymlinkOption::SYMLINK_NEVER_DEREF,
            1 => SymlinkOption::SYMLINK_ALWAYS_DEREF,
            2 => SymlinkOption::SYMLINK_DEREF_ARGSONLY,
            _ => panic!("Invalid value for SymlinkOption: {}", value),
        }
    }
}
impl AddAssign<u32> for SymlinkOption {
    fn add_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for SymlinkOption {
    fn sub_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for SymlinkOption {
    fn mul_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for SymlinkOption {
    fn div_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for SymlinkOption {
    fn rem_assign(&mut self, rhs: u32) {
        *self = SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn add(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn sub(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn mul(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn div(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for SymlinkOption {
    type Output = SymlinkOption;
    fn rem(self, rhs: u32) -> SymlinkOption {
        SymlinkOption::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type DebugOption = i32;
pub const DebugAll: DebugOption = -17;
pub const DebugTime: DebugOption = 128;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugExec: DebugOption = 32;
pub const DebugHelp: DebugOption = 16;
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugSearch: DebugOption = 4;
pub const DebugStat: DebugOption = 2;
pub const DebugExpressionTree: DebugOption = 1;
pub const DebugNone: DebugOption = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub curdepth: i32,
    pub have_stat: bool,
    pub have_type: bool,
    pub type_0: mode_t,
    pub rel_pathname: *const i8,
    pub cwd_dir_fd: i32,
    pub starting_path_length: i32,
    pub stop_at_current_level: bool,
    pub exit_status: i32,
    pub execdirs_outstanding: bool,
    pub shared_files: sharefile_handle,
    pub already_issued_stat_error_msg: bool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: i32,
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
static mut ftsoptions: i32 = 0x8 as i32 | 0x100 as i32 | 0x200 as i32 | 0x800 as i32;
static mut prev_depth: i32 = -(2147483647 as i32) - 1 as i32;
static mut curr_fd: i32 = -(1 as i32);
unsafe extern "C" fn left_dir() {
    if ftsoptions & 0x200 as i32 != 0 {
        if curr_fd >= 0 as i32 {
            close(curr_fd);
            curr_fd = -(1 as i32);
        }
    }
}
unsafe extern "C" fn inside_dir(mut dir_fd: i32) {
    if ftsoptions & 0x200 as i32 != 0 {
        if dir_fd == -(100 as i32) || dir_fd >= 0 as i32 {} else {
            __assert_fail(
                b"dir_fd == AT_FDCWD || dir_fd >= 0\0" as *const u8 as *const i8,
                b"ftsfind.c\0" as *const u8 as *const i8,
                107 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[i8; 21],
                >(b"void inside_dir(int)\0"))
                    .as_ptr(),
            );
        }
        'c_7962: {
            if dir_fd == -(100 as i32) || dir_fd >= 0 as i32 {} else {
                __assert_fail(
                    b"dir_fd == AT_FDCWD || dir_fd >= 0\0" as *const u8 as *const i8,
                    b"ftsfind.c\0" as *const u8 as *const i8,
                    107 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 21],
                        &[i8; 21],
                    >(b"void inside_dir(int)\0"))
                        .as_ptr(),
                );
            }
        };
        state.cwd_dir_fd = dir_fd;
        if curr_fd < 0 as i32 {
            if -(100 as i32) == dir_fd {
                curr_fd = -(100 as i32);
            } else if dir_fd >= 0 as i32 {
                curr_fd = dup_cloexec(dir_fd);
            } else {
                if curr_fd >= 0 as i32 || dir_fd >= 0 as i32 {} else {
                    __assert_fail(
                        b"curr_fd >= 0 || dir_fd >= 0\0" as *const u8 as *const i8,
                        b"ftsfind.c\0" as *const u8 as *const i8,
                        125 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 21],
                            &[i8; 21],
                        >(b"void inside_dir(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_7884: {
                    if curr_fd >= 0 as i32 || dir_fd >= 0 as i32 {} else {
                        __assert_fail(
                            b"curr_fd >= 0 || dir_fd >= 0\0" as *const u8 as *const i8,
                            b"ftsfind.c\0" as *const u8 as *const i8,
                            125 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 21],
                                &[i8; 21],
                            >(b"void inside_dir(int)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
        }
    }
}
unsafe extern "C" fn get_fts_info_name(mut info: i32) -> *const i8 {
    static mut buf: [i8; 14] = [0; 14];
    match info {
        1 => return b"FTS_D\0" as *const u8 as *const i8,
        2 => return b"FTS_DC\0" as *const u8 as *const i8,
        3 => return b"FTS_DEFAULT\0" as *const u8 as *const i8,
        4 => return b"FTS_DNR\0" as *const u8 as *const i8,
        5 => return b"FTS_DOT\0" as *const u8 as *const i8,
        6 => return b"FTS_DP\0" as *const u8 as *const i8,
        7 => return b"FTS_ERR\0" as *const u8 as *const i8,
        8 => return b"FTS_F\0" as *const u8 as *const i8,
        9 => return b"FTS_INIT\0" as *const u8 as *const i8,
        10 => return b"FTS_NS\0" as *const u8 as *const i8,
        11 => return b"FTS_NSOK\0" as *const u8 as *const i8,
        12 => return b"FTS_SL\0" as *const u8 as *const i8,
        13 => return b"FTS_SLNONE\0" as *const u8 as *const i8,
        14 => return b"FTS_W\0" as *const u8 as *const i8,
        _ => {
            sprintf(buf.as_mut_ptr(), b"[%d]\0" as *const u8 as *const i8, info);
            return buf.as_mut_ptr();
        }
    };
}
unsafe extern "C" fn visit(mut p: *mut FTS, mut ent: *mut FTSENT, mut pstat: *mut stat) {
    let mut eval_tree: *mut predicate = 0 as *mut predicate;
    state.have_stat = (*ent).fts_info as i32 != 10 as i32
        && (*ent).fts_info as i32 != 11 as i32;
    state.rel_pathname = (*ent).fts_accpath;
    state.cwd_dir_fd = (*p).fts_cwd_fd;
    eval_tree = get_eval_tree();
    apply_predicate((*ent).fts_path, pstat, eval_tree);
    if state.stop_at_current_level {
        rpl_fts_set(p, ent, 4 as i32);
    }
}
unsafe extern "C" fn partial_quotearg_n(
    mut n: i32,
    mut s: *mut i8,
    mut len: size_t,
    mut style: quoting_style,
) -> *const i8 {
    if 0 as i32 as u64 == len {
        return quotearg_n_style(n, style, b"\0" as *const u8 as *const i8)
    } else {
        let mut saved: i8 = 0;
        let mut result: *const i8 = 0 as *const i8;
        saved = *s.offset(len as isize);
        *s.offset(len as isize) = 0 as i32 as i8;
        result = quotearg_n_style(n, style, s);
        *s.offset(len as isize) = saved;
        return result;
    };
}
unsafe extern "C" fn issue_loop_warning(mut ent: *mut FTSENT) {
    if (*((*ent).fts_statp).as_mut_ptr()).st_mode & 0o170000 as i32 as u32
        == 0o120000 as i32 as u32
    {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            safely_quote_err_filename(0 as i32, (*ent).fts_path),
        );
    } else {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"File system loop detected; %s is part of the same file system loop as %s.\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            safely_quote_err_filename(0 as i32, (*ent).fts_path),
            partial_quotearg_n(
                1 as i32,
                (*(*ent).fts_cycle).fts_path,
                (*(*ent).fts_cycle).fts_pathlen,
                options.err_quoting_style,
            ),
        );
    };
}
unsafe extern "C" fn symlink_loop(mut name: *const i8) -> bool {
    let mut stbuf: stat = stat {
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
    let rv: i32 = (options.xstat).expect("non-null function pointer")(name, &mut stbuf);
    return 0 as i32 != rv && 40 as i32 == *__errno_location();
}
unsafe extern "C" fn consider_visiting(mut p: *mut FTS, mut ent: *mut FTSENT) {
    let mut statbuf: stat = stat {
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
    let mut mode: mode_t = 0;
    let mut ignore: i32 = 0;
    let mut isdir: i32 = 0;
    if options.debug_options & DebugSearch as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n\0"
                as *const u8 as *const i8,
            quotearg_n_style(0 as i32, options.err_quoting_style, (*ent).fts_path),
            get_fts_info_name((*ent).fts_info as i32),
            (*ent).fts_level as i32,
            prev_depth,
            quotearg_n_style(1 as i32, options.err_quoting_style, (*ent).fts_path),
            quotearg_n_style(2 as i32, options.err_quoting_style, (*ent).fts_accpath),
        );
    }
    if (*ent).fts_info as i32 == 6 as i32 {
        left_dir();
    } else if (*ent).fts_level > prev_depth as i64 || (*ent).fts_level == 0 as i32 as i64
    {
        left_dir();
    }
    inside_dir((*p).fts_cwd_fd);
    prev_depth = (*ent).fts_level as i32;
    statbuf.st_ino = (*((*ent).fts_statp).as_mut_ptr()).st_ino;
    if (*ent).fts_info as i32 == 7 as i32 {
        nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        return;
    }
    if (*ent).fts_info as i32 == 4 as i32 {
        nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        if options.do_dir_first {
            return;
        }
    } else if (*ent).fts_info as i32 == 2 as i32 {
        issue_loop_warning(ent);
        state.exit_status = 1 as i32;
        return;
    } else if (*ent).fts_info as i32 == 13 as i32 {
        if symlink_loop((*ent).fts_accpath) {
            nonfatal_target_file_error(40 as i32, (*ent).fts_path);
            return;
        }
    } else if (*ent).fts_info as i32 == 10 as i32 {
        if (*ent).fts_level == 0 as i32 as i64 {
            nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
            return;
        } else if symlink_loop((*ent).fts_accpath) {
            nonfatal_target_file_error(40 as i32, (*ent).fts_path);
            return;
        } else {
            nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        }
    }
    if (*ent).fts_info as i32 == 11 as i32 || (*ent).fts_info as i32 == 10 as i32 {
        if !state.have_stat {} else {
            __assert_fail(
                b"!state.have_stat\0" as *const u8 as *const i8,
                b"ftsfind.c\0" as *const u8 as *const i8,
                380 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[i8; 40],
                >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                    .as_ptr(),
            );
        }
        'c_7407: {
            if !state.have_stat {} else {
                __assert_fail(
                    b"!state.have_stat\0" as *const u8 as *const i8,
                    b"ftsfind.c\0" as *const u8 as *const i8,
                    380 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 40],
                        &[i8; 40],
                    >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ent).fts_info as i32 == 11 as i32 || state.type_0 == 0 as i32 as u32
        {} else {
            __assert_fail(
                b"ent->fts_info == FTS_NSOK || state.type == 0\0" as *const u8
                    as *const i8,
                b"ftsfind.c\0" as *const u8 as *const i8,
                381 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[i8; 40],
                >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                    .as_ptr(),
            );
        }
        'c_7346: {
            if (*ent).fts_info as i32 == 11 as i32 || state.type_0 == 0 as i32 as u32
            {} else {
                __assert_fail(
                    b"ent->fts_info == FTS_NSOK || state.type == 0\0" as *const u8
                        as *const i8,
                    b"ftsfind.c\0" as *const u8 as *const i8,
                    381 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 40],
                        &[i8; 40],
                    >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                        .as_ptr(),
                );
            }
        };
        mode = state.type_0;
    } else {
        state.have_stat = 1 as i32 != 0;
        state.have_type = 1 as i32 != 0;
        statbuf = *((*ent).fts_statp).as_mut_ptr();
        mode = statbuf.st_mode;
        state.type_0 = mode;
        if 0 as i32 as u32 == mode {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"WARNING: file %s appears to have mode 0000\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, options.err_quoting_style, (*ent).fts_path),
            );
        }
    }
    state.curdepth = (*ent).fts_level as i32;
    if mode != 0 {
        if !digest_mode(
            &mut mode,
            (*ent).fts_path,
            ((*ent).fts_name).as_mut_ptr(),
            &mut statbuf,
            0 as i32 != 0,
        ) {
            return;
        }
    }
    ignore = 0 as i32;
    isdir = (mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
        || 1 as i32 == (*ent).fts_info as i32 || 6 as i32 == (*ent).fts_info as i32
        || 2 as i32 == (*ent).fts_info as i32) as i32;
    if isdir != 0 && (*ent).fts_info as i32 == 11 as i32 {
        rpl_fts_set(p, ent, 1 as i32);
        return;
    }
    if options.maxdepth >= 0 as i32 {
        if (*ent).fts_level >= options.maxdepth as i64 {
            rpl_fts_set(p, ent, 4 as i32);
            if (*ent).fts_level > options.maxdepth as i64 {
                ignore = 1 as i32;
            }
        }
    }
    if (*ent).fts_info as i32 == 1 as i32 && !options.do_dir_first {
        ignore = 1 as i32;
    } else if (*ent).fts_info as i32 == 6 as i32 && options.do_dir_first as i32 != 0 {
        ignore = 1 as i32;
    } else if (*ent).fts_level < options.mindepth as i64 {
        ignore = 1 as i32;
    }
    if options.debug_options & DebugSearch as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n\0"
                as *const u8 as *const i8,
            quotearg_n_style(0 as i32, options.err_quoting_style, (*ent).fts_path),
            get_fts_info_name((*ent).fts_info as i32),
            isdir,
            ignore,
            state.have_stat as i32,
            state.have_type as i32,
        );
    }
    if ignore == 0 {
        visit(p, ent, &mut statbuf);
    }
    if (*ent).fts_info as i32 == 6 as i32 {
        state.stop_at_current_level = 0 as i32 != 0;
    }
}
unsafe extern "C" fn find(mut arg: *mut i8) -> bool {
    let mut arglist: [*mut i8; 2] = [0 as *mut i8; 2];
    let mut p: *mut FTS = 0 as *mut FTS;
    let mut ent: *mut FTSENT = 0 as *mut FTSENT;
    state.starting_path_length = strlen(arg) as i32;
    inside_dir(-(100 as i32));
    arglist[0 as i32 as usize] = arg;
    arglist[1 as i32 as usize] = 0 as *mut i8;
    match options.symlink_handling as u32 {
        1 => {
            ftsoptions |= 0x1 as i32 | 0x2 as i32;
        }
        2 => {
            ftsoptions |= 0x1 as i32 | 0x10 as i32;
        }
        0 => {
            ftsoptions |= 0x10 as i32;
        }
        _ => {}
    }
    if options.stay_on_filesystem {
        ftsoptions |= 0x40 as i32;
    }
    p = rpl_fts_open(arglist.as_mut_ptr(), ftsoptions, None);
    if p.is_null() {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot search %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            safely_quote_err_filename(0 as i32, arg),
        );
        state.exit_status = 1 as i32;
    } else {
        let mut level: i32 = -(2147483647 as i32) - 1 as i32;
        loop {
            *__errno_location() = 0 as i32;
            ent = rpl_fts_read(p);
            if ent.is_null() {
                break;
            }
            if state.execdirs_outstanding as i32 != 0 && (*ent).fts_level as i32 != level
            {
                complete_pending_execdirs();
            }
            level = (*ent).fts_level as i32;
            state.already_issued_stat_error_msg = 0 as i32 != 0;
            state.have_stat = 0 as i32 != 0;
            state.have_type = (*((*ent).fts_statp).as_mut_ptr()).st_mode != 0;
            state.type_0 = if state.have_type as i32 != 0 {
                (*((*ent).fts_statp).as_mut_ptr()).st_mode
            } else {
                0 as i32 as u32
            };
            consider_visiting(p, ent);
        }
        if *__errno_location() != 0 {
            error(
                0 as i32,
                *__errno_location(),
                b"failed to read file names from file system at or below %s\0"
                    as *const u8 as *const i8,
                safely_quote_err_filename(0 as i32, arg),
            );
            state.exit_status = 1 as i32;
            return 0 as i32 != 0;
        }
        if 0 as i32 != rpl_fts_close(p) {
            error(
                0 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"failed to restore working directory after searching %s\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                arg,
            );
            state.exit_status = 1 as i32;
            return 0 as i32 != 0;
        }
        p = 0 as *mut FTS;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn process_all_startpoints(
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> bool {
    let mut argv_starting_points: bool = (0 as i32) < argc
        && !looks_like_expression(*argv.offset(0 as i32 as isize), 1 as i32 != 0);
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut files0_filename_quoted: *const i8 = 0 as *const i8;
    let mut ai: *mut argv_iterator = 0 as *mut argv_iterator;
    if !(options.files0_from).is_null() {
        if argv_starting_points {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"extra operand %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                safely_quote_err_filename(0 as i32, *argv.offset(0 as i32 as isize)),
            );
            if ::core::mem::size_of::<C2RustUnnamed_5>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    b"%s\0" as *const u8 as *const i8,
                    dcgettext(
                        0 as *const i8,
                        b"file operands cannot be combined with -files0-from\0"
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
                    b"%s\0" as *const u8 as *const i8,
                    dcgettext(
                        0 as *const i8,
                        b"file operands cannot be combined with -files0-from\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        if 0 as i32 == strcmp(options.files0_from, b"-\0" as *const u8 as *const i8) {
            if options.ok_prompt_stdin {
                if ::core::mem::size_of::<C2RustUnnamed_4>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        b"%s\n\0" as *const u8 as *const i8,
                        dcgettext(
                            0 as *const i8,
                            b"option -files0-from reading from standard input cannot be combined with -ok, -okdir\0"
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
                        b"%s\n\0" as *const u8 as *const i8,
                        dcgettext(
                            0 as *const i8,
                            b"option -files0-from reading from standard input cannot be combined with -ok, -okdir\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            files0_filename_quoted = safely_quote_err_filename(
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"(standard input)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            stream = stdin;
        } else {
            files0_filename_quoted = safely_quote_err_filename(
                0 as i32,
                options.files0_from,
            );
            stream = fopen(options.files0_from, b"r\0" as *const u8 as *const i8);
            if stream.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"cannot open %s for reading\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        files0_filename_quoted,
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
                            b"cannot open %s for reading\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        files0_filename_quoted,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            let fd: i32 = fileno(stream);
            if fd >= 0 as i32 {} else {
                __assert_fail(
                    b"fd >= 0\0" as *const u8 as *const i8,
                    b"ftsfind.c\0" as *const u8 as *const i8,
                    610 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[i8; 44],
                    >(b"_Bool process_all_startpoints(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_8947: {
                if fd >= 0 as i32 {} else {
                    __assert_fail(
                        b"fd >= 0\0" as *const u8 as *const i8,
                        b"ftsfind.c\0" as *const u8 as *const i8,
                        610 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 44],
                            &[i8; 44],
                        >(b"_Bool process_all_startpoints(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            if options.ok_prompt_stdin {
                let mut sb1: stat = stat {
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
                let mut sb2: stat = stat {
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
                if fstat(fd, &mut sb1) == 0 as i32
                    && fstat(0 as i32, &mut sb2) == 0 as i32
                    && (sb1.st_ino == sb2.st_ino && sb1.st_dev == sb2.st_dev)
                {
                    if ::core::mem::size_of::<C2RustUnnamed_2>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            b"%s: %s\n\0" as *const u8 as *const i8,
                            dcgettext(
                                0 as *const i8,
                                b"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            files0_filename_quoted,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            0 as i32,
                            b"%s: %s\n\0" as *const u8 as *const i8,
                            dcgettext(
                                0 as *const i8,
                                b"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            files0_filename_quoted,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            set_cloexec_flag(fd, 1 as i32 != 0);
        }
        ai = argv_iter_init_stream(stream);
    } else {
        if !argv_starting_points {
            let mut defaultpath: [i8; 2] = *::core::mem::transmute::<
                &[u8; 2],
                &mut [i8; 2],
            >(b".\0");
            return find(defaultpath.as_mut_ptr());
        }
        ai = argv_iter_init_argv(argv);
    }
    if ai.is_null() {
        xalloc_die();
    }
    let mut ok: bool = 1 as i32 != 0;
    loop {
        let mut ai_err: argv_iter_err = argv_iter_err::from_libc_c_uint(0 as u32);
        let mut file_name: *mut i8 = argv_iter(ai, &mut ai_err);
        if file_name.is_null() {
            match ai_err as u32 {
                2 => {
                    break;
                }
                4 => {
                    error(
                        0 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"%s: read error\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        files0_filename_quoted,
                    );
                    state.exit_status = 1 as i32;
                    ok = 0 as i32 != 0;
                    break;
                }
                3 => {
                    xalloc_die();
                }
                _ => {}
            }
            if (b"unexpected error code from argv_iter\0" as *const u8 as *const i8)
                .is_null()
            {} else {
                __assert_fail(
                    b"!\"unexpected error code from argv_iter\"\0" as *const u8
                        as *const i8,
                    b"ftsfind.c\0" as *const u8 as *const i8,
                    675 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[i8; 44],
                    >(b"_Bool process_all_startpoints(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_8651: {
                if (b"unexpected error code from argv_iter\0" as *const u8 as *const i8)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"!\"unexpected error code from argv_iter\"\0" as *const u8
                            as *const i8,
                        b"ftsfind.c\0" as *const u8 as *const i8,
                        675 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 44],
                            &[i8; 44],
                        >(b"_Bool process_all_startpoints(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        if *file_name.offset(0 as i32 as isize) == 0 {
            if (options.files0_from).is_null() {
                error(
                    0 as i32,
                    2 as i32,
                    b"%s\0" as *const u8 as *const i8,
                    safely_quote_err_filename(0 as i32, file_name),
                );
            } else {
                let mut file_number: u64 = argv_iter_n_args(ai);
                error(
                    0 as i32,
                    0 as i32,
                    b"%s:%lu: %s\0" as *const u8 as *const i8,
                    files0_filename_quoted,
                    file_number,
                    dcgettext(
                        0 as *const i8,
                        b"invalid zero-length file name\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            state.exit_status = 1 as i32;
            ok = 0 as i32 != 0;
        } else {
            if (options.files0_from).is_null()
                && looks_like_expression(file_name, 1 as i32 != 0) as i32 != 0
            {
                break;
            }
            state.starting_path_length = strlen(file_name) as i32;
            if find(file_name) {
                continue;
            }
            ok = 0 as i32 != 0;
            break;
        }
    }
    argv_iter_free(ai);
    if ok as i32 != 0 && !(options.files0_from).is_null()
        && (ferror(stream) != 0 || rpl_fclose(stream) != 0 as i32)
    {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"error reading %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                files0_filename_quoted,
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
                    b"error reading %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                files0_filename_quoted,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    return ok;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut end_of_leading_options: i32 = 0 as i32;
    let mut eval_tree: *mut predicate = 0 as *mut predicate;
    if !(*argv.offset(0 as i32 as isize)).is_null() {
        set_program_name(*argv.offset(0 as i32 as isize));
    } else {
        set_program_name(b"find\0" as *const u8 as *const i8);
    }
    record_initial_cwd();
    state.already_issued_stat_error_msg = 0 as i32 != 0;
    state.exit_status = 0 as i32;
    state.execdirs_outstanding = 0 as i32 != 0;
    state.cwd_dir_fd = -(100 as i32);
    if fd_leak_check_is_enabled() {
        remember_non_cloexec_fds();
    }
    state.shared_files = sharefile_init(b"w\0" as *const u8 as *const i8);
    if (state.shared_files).is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_7>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Failed to initialize shared-file hash table\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
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
                    b"Failed to initialize shared-file hash table\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    set_option_defaults(&mut options);
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"findutils\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"findutils\0" as *const u8 as *const i8);
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
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
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    end_of_leading_options = process_leading_options(argc, argv);
    if options.debug_options & DebugStat as i32 as u64 != 0 {
        options.xstat = Some(
            debug_stat as unsafe extern "C" fn(*const i8, *mut stat) -> i32,
        );
    }
    if options.debug_options & DebugTime as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"cur_day_start = %s\0" as *const u8 as *const i8,
            ctime(&mut options.cur_day_start.tv_sec),
        );
    }
    eval_tree = build_expression_tree(argc, argv, end_of_leading_options);
    !options.open_nofollow_available;
    if process_all_startpoints(
        argc - end_of_leading_options,
        argv.offset(end_of_leading_options as isize),
    ) {
        show_success_rates(eval_tree);
        cleanup();
    }
    return state.exit_status;
}
#[no_mangle]
pub unsafe extern "C" fn is_fts_enabled(mut fts_options: *mut i32) -> bool {
    *fts_options = ftsoptions;
    return 1 as i32 != 0;
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