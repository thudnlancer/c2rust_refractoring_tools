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
    pub type re_dfa_t;
    pub type quoting_options;
    pub type saved_cwd;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn access(__name: *const i8, __type: i32) -> i32;
    fn find_parser(search_name: *const i8) -> *const parser_table;
    fn parse_print(
        _: *const parser_table,
        argv: *mut *mut i8,
        arg_ptr: *mut i32,
    ) -> bool;
    fn pred_sanity_check(predicates_0: *const predicate);
    fn check_option_combinations(p: *const predicate);
    fn parse_begin_user_args(
        args: *mut *mut i8,
        argno: i32,
        last: *const predicate,
        predicates_0: *const predicate,
    );
    fn parse_end_user_args(
        args: *mut *mut i8,
        argno: i32,
        last: *const predicate,
        predicates_0: *const predicate,
    );
    fn parse_openparen(
        entry: *const parser_table,
        argv: *mut *mut i8,
        arg_ptr: *mut i32,
    ) -> bool;
    fn parse_closeparen(
        entry: *const parser_table,
        argv: *mut *mut i8,
        arg_ptr: *mut i32,
    ) -> bool;
    fn pred_amin(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_and(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_anewer(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_atime(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_closeparen(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_cmin(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_cnewer(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_comma(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ctime(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_delete(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_empty(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_exec(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_execdir(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_executable(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_false(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fls(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint0(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprintf(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fstype(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_gid(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_group(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ilname(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_iname(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_inum(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ipath(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_links(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_lname(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ls(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_mmin(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_mtime(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_name(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_negate(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_newer(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_newerXY(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_nogroup(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_nouser(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ok(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_okdir(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_openparen(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_or(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_path(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_perm(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_print(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_print0(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_prune(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_readable(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_regex(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_samefile(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_size(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_true(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_type(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_uid(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_used(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_user(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_writable(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_xtype(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_context(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_quit(
        pathname: *const i8,
        stat_buf: *mut stat,
        pred_ptr: *mut predicate,
    ) -> !;
    fn print_list(_: *mut FILE, node: *mut predicate);
    fn print_optlist(fp: *mut FILE, node: *const predicate);
    static mut options: options;
    static mut state: state;
    fn looks_like_expression(arg: *const i8, leading: bool) -> bool;
    fn default_prints(pred: *mut predicate) -> bool;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type size_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
pub type uintmax_t = __uintmax_t;
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
    pub args: C2RustUnnamed,
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
pub union C2RustUnnamed {
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
pub type PREDICATEFUNCTION = unsafe extern "C" fn(
    *const i8,
    *mut stat,
    *mut predicate,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cost_assoc {
    pub cost: EvaluationCost,
    pub name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prec_assoc {
    pub prec: libc::c_short,
    pub prec_name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct op_assoc {
    pub type_0: libc::c_short,
    pub type_name: *const i8,
}
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugExpressionTree: DebugOption = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predlist {
    pub head: *mut predicate,
    pub tail: *mut predicate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pred_cost_lookup {
    pub fn_0: PRED_FUNC,
    pub cost: EvaluationCost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub pfn: PRED_FUNC,
    pub mem: [i8; 8],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: i32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: i32,
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
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: i32,
}
pub type DebugOption = i32;
pub const DebugAll: DebugOption = -17;
pub const DebugTime: DebugOption = 128;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugExec: DebugOption = 32;
pub const DebugHelp: DebugOption = 16;
pub const DebugSearch: DebugOption = 4;
pub const DebugStat: DebugOption = 2;
pub const DebugNone: DebugOption = 0;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: i32 = 0;
    __l = 0 as i32 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as i32 as u64);
        __p = (__base as *const i8).offset(__idx.wrapping_mul(__size) as isize)
            as *mut libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as i32 {
            __u = __idx;
        } else if __comparison > 0 as i32 {
            __l = __idx.wrapping_add(1 as i32 as u64);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
static mut predicates: *mut predicate = 0 as *const predicate as *mut predicate;
static mut eval_tree: *mut predicate = 0 as *const predicate as *mut predicate;
static mut last_pred: *mut predicate = 0 as *const predicate as *mut predicate;
static mut start_points: *mut *mut i8 = 0 as *const *mut i8 as *mut *mut i8;
static mut num_start_points: size_t = 0 as i32 as size_t;
#[no_mangle]
pub unsafe extern "C" fn matches_start_point(
    mut glob: *const i8,
    mut foldcase: bool,
) -> bool {
    let mut fnmatch_flags: i32 = 0 as i32;
    if foldcase {
        fnmatch_flags |= (1 as i32) << 4 as i32;
    }
    if num_start_points != 0 {
        let mut i: size_t = 0;
        i = 0 as i32 as size_t;
        while i < num_start_points {
            if fnmatch(glob, *start_points.offset(i as isize), fnmatch_flags) == 0 as i32
            {
                return 1 as i32 != 0;
            }
            i = i.wrapping_add(1);
            i;
        }
        return 0 as i32 != 0;
    } else {
        return fnmatch(glob, b".\0" as *const u8 as *const i8, fnmatch_flags) == 0 as i32
    };
}
unsafe extern "C" fn get_expr(
    mut input: *mut *mut predicate,
    mut prev_prec: libc::c_short,
    mut prev_pred: *const predicate,
) -> *mut predicate {
    let mut next: *mut predicate = 0 as *mut predicate;
    let mut this_pred: *mut predicate = *input;
    if (*input).is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_20>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"invalid expression\0" as *const u8 as *const i8,
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
                    b"invalid expression\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    match (**input).p_type as u32 {
        0 => {
            if ::core::mem::size_of::<C2RustUnnamed_19>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid expression\0" as *const u8 as *const i8,
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
                        b"invalid expression\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        3 => {
            if ::core::mem::size_of::<C2RustUnnamed_18>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid expression; you have used a binary operator '%s' with nothing before it.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*this_pred).p_name,
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
                        b"invalid expression; you have used a binary operator '%s' with nothing before it.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*this_pred).p_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        5 => {
            if prev_pred.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_17>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*this_pred).p_name,
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
                            b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*this_pred).p_name,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if (predicate_type::UNI_OP as i32 as u32 == (*prev_pred).p_type as u32
                || predicate_type::BI_OP as i32 as u32 == (*prev_pred).p_type as u32)
                && !(*this_pred).artificial
            {
                if ::core::mem::size_of::<C2RustUnnamed_16>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"expected an expression between '%s' and ')'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*prev_pred).p_name,
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
                            b"expected an expression between '%s' and ')'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*prev_pred).p_name,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else if (**input).artificial {
                if ::core::mem::size_of::<C2RustUnnamed_15>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"expected an expression after '%s'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*prev_pred).p_name,
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
                            b"expected an expression after '%s'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*prev_pred).p_name,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                if ::core::mem::size_of::<C2RustUnnamed_14>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression; you have too many ')'\0" as *const u8
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
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression; you have too many ')'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        1 => {
            next = *input;
            *input = (**input).pred_next;
        }
        2 => {
            next = *input;
            *input = (**input).pred_next;
            (*next).pred_right = get_expr(
                input,
                predicate_precedence::NEGATE_PREC as i32 as libc::c_short,
                next,
            );
        }
        4 => {
            if ((**input).pred_next).is_null()
                || (*(**input).pred_next).artificial as i32 != 0
            {
                if ::core::mem::size_of::<C2RustUnnamed_13>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*this_pred).p_name,
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
                            b"invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*this_pred).p_name,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            prev_pred = *input;
            *input = (**input).pred_next;
            if (**input).p_type as u32 == predicate_type::CLOSE_PAREN as i32 as u32 {
                if (*prev_pred).artificial {
                    if ::core::mem::size_of::<C2RustUnnamed_12>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (**input).p_name,
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
                                b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (**input).p_name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if ::core::mem::size_of::<C2RustUnnamed_11>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression; empty parentheses are not allowed.\0"
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
                            b"invalid expression; empty parentheses are not allowed.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            next = get_expr(
                input,
                predicate_precedence::NO_PREC as i32 as libc::c_short,
                prev_pred,
            );
            if (*input).is_null()
                || (**input).p_type as u32 != predicate_type::CLOSE_PAREN as i32 as u32
            {
                if ::core::mem::size_of::<C2RustUnnamed_10>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression; I was expecting to find a ')' somewhere but did not see one.\0"
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
                            b"invalid expression; I was expecting to find a ')' somewhere but did not see one.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            *input = (**input).pred_next;
        }
        _ => {
            if ::core::mem::size_of::<C2RustUnnamed_9>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"oops -- invalid expression type!\0" as *const u8 as *const i8,
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
                        b"oops -- invalid expression type!\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if (*input).is_null() {
        return next;
    }
    if (**input).p_prec as i32 > prev_prec as i32 {
        next = scan_rest(input, next, prev_prec);
        if next.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"invalid expression\0" as *const u8 as *const i8,
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
                        b"invalid expression\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    return next;
}
unsafe extern "C" fn scan_rest(
    mut input: *mut *mut predicate,
    mut head: *mut predicate,
    mut prev_prec: libc::c_short,
) -> *mut predicate {
    let mut tree: *mut predicate = 0 as *mut predicate;
    if (*input).is_null()
        || (**input).p_type as u32 == predicate_type::CLOSE_PAREN as i32 as u32
    {
        return 0 as *mut predicate;
    }
    tree = head;
    while !(*input).is_null() && (**input).p_prec as i32 > prev_prec as i32 {
        match (**input).p_type as u32 {
            0 | 1 | 2 | 4 => {
                if ::core::mem::size_of::<C2RustUnnamed_8>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"invalid expression\0" as *const u8 as *const i8,
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
                            b"invalid expression\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            3 => {
                let mut prev: *mut predicate = *input;
                (**input).pred_left = tree;
                tree = *input;
                *input = (**input).pred_next;
                (*tree).pred_right = get_expr(
                    input,
                    (*tree).p_prec as libc::c_short,
                    prev,
                );
            }
            5 => return tree,
            _ => {
                if ::core::mem::size_of::<C2RustUnnamed_7>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"oops -- invalid expression type (%d)!\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (**input).p_type as i32,
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
                            b"oops -- invalid expression type (%d)!\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (**input).p_type as i32,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
    return tree;
}
unsafe extern "C" fn predicate_is_cost_free(mut p: *const predicate) -> bool {
    if (*p).pred_func == Some(pred_name as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_path as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_iname as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_ipath as PREDICATEFUNCTION)
    {
        return 1 as i32 != 0
    } else if options.optimisation_level as i32 > 0 as i32 {
        if (*p).pred_func == Some(pred_and as PREDICATEFUNCTION)
            || (*p).pred_func == Some(pred_negate as PREDICATEFUNCTION)
            || (*p).pred_func == Some(pred_comma as PREDICATEFUNCTION)
            || (*p).pred_func == Some(pred_or as PREDICATEFUNCTION)
        {
            return 0 as i32 != 0
        } else {
            return EvaluationCost::NeedsNothing as i32 as u32 == (*p).p_cost as u32
        }
    } else {
        return 0 as i32 != 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_predicate(mut fp: *mut FILE, mut p: *const predicate) {
    if !((*p).arg_text).is_null() {
        fprintf(fp, b"%s %s\0" as *const u8 as *const i8, (*p).p_name, (*p).arg_text);
    } else {
        fprintf(fp, b"%s\0" as *const u8 as *const i8, (*p).p_name);
    };
}
unsafe extern "C" fn predlist_init(mut p: *mut predlist) {
    (*p).tail = 0 as *mut predicate;
    (*p).head = (*p).tail;
}
unsafe extern "C" fn predlist_insert(
    mut list: *mut predlist,
    mut curr: *mut predicate,
    mut pprev: *mut *mut predicate,
) {
    let mut insertpos: *mut *mut predicate = &mut (*list).head;
    *pprev = (*curr).pred_left;
    (*curr).pred_left = *insertpos;
    *insertpos = curr;
    if ((*list).tail).is_null() {
        (*list).tail = (*list).head;
    }
}
unsafe extern "C" fn pred_cost_compare(
    mut p1: *const predicate,
    mut p2: *const predicate,
    mut wantfailure: bool,
) -> i32 {
    if (*p1).p_cost as u32 == (*p2).p_cost as u32 {
        if (*p1).est_success_rate == (*p2).est_success_rate {
            return 0 as i32
        } else if wantfailure {
            return if (*p1).est_success_rate < (*p2).est_success_rate {
                -(1 as i32)
            } else {
                1 as i32
            }
        } else {
            return if (*p1).est_success_rate < (*p2).est_success_rate {
                1 as i32
            } else {
                -(1 as i32)
            }
        }
    } else {
        return if ((*p1).p_cost as u32) < (*p2).p_cost as u32 {
            -(1 as i32)
        } else {
            1 as i32
        }
    };
}
unsafe extern "C" fn predlist_merge_sort(
    mut list: *mut predlist,
    mut last: *mut *mut predicate,
) {
    let mut new_list: predlist = predlist {
        head: 0 as *mut predicate,
        tail: 0 as *mut predicate,
    };
    let mut p: *mut predicate = 0 as *mut predicate;
    let mut q: *mut predicate = 0 as *mut predicate;
    if ((*list).head).is_null() {
        return;
    }
    if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"%s:\n\0" as *const u8 as *const i8,
            b"predlist before merge sort\0" as *const u8 as *const i8,
        );
        print_tree(stderr, (*list).head, 2 as i32);
    }
    calculate_derived_rates((*list).head);
    predlist_init(&mut new_list);
    while !((*list).head).is_null() {
        q = (*list).head;
        (*list).head = (*(*list).head).pred_left;
        (*q).pred_left = 0 as *mut predicate;
        p = new_list.head;
        while !p.is_null() {
            let wantfailure: bool = predicate_precedence::OR_PREC as i32 as u32
                != (*p).p_prec as u32;
            if pred_cost_compare((*p).pred_right, (*q).pred_right, wantfailure)
                >= 0 as i32
            {
                break;
            }
            p = (*p).pred_left;
        }
        if !p.is_null() {
            (*q).pred_left = (*p).pred_left;
            if ((*q).pred_left).is_null() {
                new_list.tail = q;
            }
            (*p).pred_left = q;
        } else {
            (*q).pred_left = new_list.head;
            new_list.head = q;
            if (new_list.tail).is_null() {
                new_list.tail = q;
            }
        }
    }
    if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"%s:\n\0" as *const u8 as *const i8,
            b"predlist after merge sort\0" as *const u8 as *const i8,
        );
        print_tree(stderr, new_list.head, 2 as i32);
    }
    calculate_derived_rates(new_list.head);
    merge_pred(new_list.head, new_list.tail, last);
    predlist_init(list);
}
unsafe extern "C" fn merge_lists(
    mut lists: *mut predlist,
    mut nlists: i32,
    mut name_list: *mut predlist,
    mut regex_list: *mut predlist,
    mut last: *mut *mut predicate,
) {
    let mut i: i32 = 0;
    static mut mergefn: Option<
        unsafe extern "C" fn(*mut predlist, *mut *mut predicate) -> (),
    > = None;
    mergefn = Some(
        predlist_merge_sort
            as unsafe extern "C" fn(*mut predlist, *mut *mut predicate) -> (),
    );
    mergefn.expect("non-null function pointer")(name_list, last);
    mergefn.expect("non-null function pointer")(regex_list, last);
    i = 0 as i32;
    while i < nlists {
        mergefn
            .expect("non-null function pointer")(&mut *lists.offset(i as isize), last);
        i += 1;
        i;
    }
}
unsafe extern "C" fn subtree_has_side_effects(mut p: *const predicate) -> bool {
    if !p.is_null() {
        return (*p).side_effects as i32 != 0
            || subtree_has_side_effects((*p).pred_left) as i32 != 0
            || subtree_has_side_effects((*p).pred_right) as i32 != 0
    } else {
        return 0 as i32 != 0
    };
}
unsafe extern "C" fn worst_cost(mut p: *const predicate) -> i32 {
    if !p.is_null() {
        let mut cost_r: u32 = 0;
        let mut cost_l: u32 = 0;
        let mut worst: u32 = 0;
        cost_l = worst_cost((*p).pred_left) as u32;
        cost_r = worst_cost((*p).pred_right) as u32;
        worst = if cost_l > cost_r { cost_l } else { cost_r };
        if worst < (*p).p_cost as u32 {
            worst = (*p).p_cost as u32;
        }
        return worst as i32;
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn perform_arm_swap(mut p: *mut predicate) {
    let mut tmp: *mut predicate = (*(*p).pred_left).pred_right;
    (*(*p).pred_left).pred_right = (*p).pred_right;
    (*p).pred_right = tmp;
}
unsafe extern "C" fn consider_arm_swap(mut p: *mut predicate) -> bool {
    let mut left_cost: i32 = 0;
    let mut right_cost: i32 = 0;
    let mut reason: *const i8 = 0 as *const i8;
    let mut pl: *mut *mut predicate = 0 as *mut *mut predicate;
    let mut pr: *mut *mut predicate = 0 as *mut *mut predicate;
    if predicate_type::BI_OP as i32 as u32 != (*p).p_type as u32 {
        reason = b"Not a binary operation\0" as *const u8 as *const i8;
    }
    if reason.is_null() {
        if ((*p).pred_left).is_null() || ((*p).pred_right).is_null() {
            reason = b"Doesn't have two arms\0" as *const u8 as *const i8;
        }
    }
    if reason.is_null() {
        if ((*(*p).pred_left).pred_right).is_null() {
            reason = b"Left arm has no child on RHS\0" as *const u8 as *const i8;
        }
    }
    pr = &mut (*p).pred_right;
    pl = &mut (*(*p).pred_left).pred_right;
    if reason.is_null() {
        if subtree_has_side_effects(*pl) {
            reason = b"Left subtree has side-effects\0" as *const u8 as *const i8;
        }
    }
    if reason.is_null() {
        if subtree_has_side_effects(*pr) {
            reason = b"Right subtree has side-effects\0" as *const u8 as *const i8;
        }
    }
    if reason.is_null() {
        left_cost = worst_cost(*pl);
        right_cost = worst_cost(*pr);
        if left_cost < right_cost {
            reason = b"efficient as-is\0" as *const u8 as *const i8;
        }
    }
    if reason.is_null() {
        let mut want_swap: bool = false;
        if left_cost == right_cost {
            let mut succ_rate_l: libc::c_float = (**pl).est_success_rate;
            let mut succ_rate_r: libc::c_float = (**pr).est_success_rate;
            if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
                fprintf(
                    stderr,
                    b"Success rates: l=%f, r=%f\n\0" as *const u8 as *const i8,
                    succ_rate_l as libc::c_double,
                    succ_rate_r as libc::c_double,
                );
            }
            if (*p).pred_func == Some(pred_or as PREDICATEFUNCTION) {
                want_swap = succ_rate_r < succ_rate_l;
                if !want_swap {
                    reason = b"Operation is OR; right success rate >= left\0"
                        as *const u8 as *const i8;
                }
            } else if (*p).pred_func == Some(pred_and as PREDICATEFUNCTION) {
                want_swap = succ_rate_r > succ_rate_l;
                if !want_swap {
                    reason = b"Operation is AND; right success rate <= left\0"
                        as *const u8 as *const i8;
                }
            } else {
                want_swap = 0 as i32 != 0;
                reason = b"Not 'AND' or 'OR'\0" as *const u8 as *const i8;
            }
        } else {
            want_swap = 1 as i32 != 0;
        }
        if want_swap {
            if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
                fprintf(
                    stderr,
                    b"Performing arm swap on:\n\0" as *const u8 as *const i8,
                );
                print_tree(stderr, p, 0 as i32);
            }
            perform_arm_swap(p);
            return 1 as i32 != 0;
        }
    }
    if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"Not an arm swap candidate (%s):\n\0" as *const u8 as *const i8,
            reason,
        );
        print_tree(stderr, p, 0 as i32);
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn do_arm_swaps(mut p: *mut predicate) -> bool {
    if !p.is_null() {
        let mut swapped: bool = false;
        loop {
            swapped = 0 as i32 != 0;
            if consider_arm_swap(p) as i32 != 0
                || do_arm_swaps((*p).pred_left) as i32 != 0
                || do_arm_swaps((*p).pred_right) as i32 != 0
            {
                swapped = 1 as i32 != 0;
            }
            if !swapped {
                break;
            }
        }
        return swapped;
    } else {
        return 0 as i32 != 0
    };
}
unsafe extern "C" fn opt_expr(mut eval_treep: *mut *mut predicate) -> bool {
    let mut regex_list: predlist = {
        let mut init = predlist {
            head: 0 as *mut predicate,
            tail: 0 as *mut predicate,
        };
        init
    };
    let mut name_list: predlist = {
        let mut init = predlist {
            head: 0 as *mut predicate,
            tail: 0 as *mut predicate,
        };
        init
    };
    let mut cbo_list: [predlist; 11] = [predlist {
        head: 0 as *mut predicate,
        tail: 0 as *mut predicate,
    }; 11];
    let mut i: i32 = 0;
    let mut curr: *mut predicate = 0 as *mut predicate;
    let mut prevp: *mut *mut predicate = 0 as *mut *mut predicate;
    let mut last_sidep: *mut *mut predicate = 0 as *mut *mut predicate;
    let mut pred_func: PRED_FUNC = None;
    let mut p_type: predicate_type = predicate_type::NO_TYPE;
    let mut has_side_effects: bool = 0 as i32 != 0;
    let mut prev_prec: predicate_precedence = predicate_precedence::NO_PREC;
    let mut biop_prec: predicate_precedence = predicate_precedence::NO_PREC;
    if eval_treep.is_null() || (*eval_treep).is_null() {
        return 0 as i32 != 0;
    }
    i = 0 as i32;
    while i < EvaluationCost::NumEvaluationCosts as i32 {
        predlist_init(&mut *cbo_list.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
    prevp = eval_treep;
    prev_prec = predicate_precedence::AND_PREC;
    curr = *prevp;
    while !((*curr).pred_left).is_null() {
        prevp = &mut (*curr).pred_left;
        prev_prec = (*curr).p_prec;
        curr = (*curr).pred_left;
    }
    if (*curr).p_type as u32 != predicate_type::BI_OP as i32 as u32 {
        set_new_parent(curr, prev_prec, prevp);
    }
    if options.debug_options & (DebugExpressionTree as i32 | DebugTreeOpt as i32) as u64
        != 0
    {
        fprintf(stderr, b"Normalized Eval Tree:\n\0" as *const u8 as *const i8);
        print_tree(stderr, *eval_treep, 0 as i32);
    }
    prevp = eval_treep;
    biop_prec = predicate_precedence::NO_PREC;
    if !(*prevp).is_null()
        && (**prevp).p_type as u32 == predicate_type::BI_OP as i32 as u32
    {
        biop_prec = (**prevp).p_prec;
    }
    loop {
        curr = *prevp;
        if curr.is_null() {
            break;
        }
        if (*curr).p_type as u32 == predicate_type::BI_OP as i32 as u32 {
            if (*curr).p_prec as u32 != biop_prec as u32 {
                curr = set_new_parent(curr, biop_prec, prevp);
            }
        }
        p_type = (*(*curr).pred_right).p_type;
        pred_func = (*(*curr).pred_right).pred_func;
        match p_type as u32 {
            0 | 1 => {
                if !(biop_prec as u32 == predicate_precedence::COMMA_PREC as i32 as u32)
                {
                    if !(*(*curr).pred_right).side_effects {
                        let mut reorder: bool = false;
                        if predicate_is_cost_free((*curr).pred_right) {
                            if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
                                fprintf(
                                    stderr,
                                    b"-O%d: promoting cheap predicate \0" as *const u8
                                        as *const i8,
                                    options.optimisation_level as i32,
                                );
                                print_predicate(stderr, (*curr).pred_right);
                                fprintf(
                                    stderr,
                                    b" into name_list\n\0" as *const u8 as *const i8,
                                );
                            }
                            predlist_insert(&mut name_list, curr, prevp);
                            continue;
                        } else if pred_func == Some(pred_regex as PREDICATEFUNCTION) {
                            predlist_insert(&mut regex_list, curr, prevp);
                            continue;
                        } else {
                            reorder = options.optimisation_level as i32 > 1 as i32
                                && (EvaluationCost::NeedsType as i32 as u32
                                    == (*(*curr).pred_right).p_cost as u32
                                    || EvaluationCost::NeedsInodeNumber as i32 as u32
                                        == (*(*curr).pred_right).p_cost as u32)
                                && !(*(*curr).pred_right).need_stat
                                || options.optimisation_level as i32 > 2 as i32;
                            if reorder {
                                if options.debug_options & DebugTreeOpt as i32 as u64 != 0 {
                                    fprintf(
                                        stderr,
                                        b"-O%d: categorising predicate \0" as *const u8
                                            as *const i8,
                                        options.optimisation_level as i32,
                                    );
                                    print_predicate(stderr, (*curr).pred_right);
                                    fprintf(
                                        stderr,
                                        b" by cost (%s)\n\0" as *const u8 as *const i8,
                                        cost_name((*(*curr).pred_right).p_cost),
                                    );
                                }
                                predlist_insert(
                                    &mut *cbo_list
                                        .as_mut_ptr()
                                        .offset((*(*curr).pred_right).p_cost as isize),
                                    curr,
                                    prevp,
                                );
                                continue;
                            }
                        }
                    }
                }
            }
            2 => {
                (*(*curr).pred_right).side_effects = opt_expr(
                    &mut (*(*curr).pred_right).pred_right,
                );
            }
            3 => {
                (*(*curr).pred_right).side_effects = opt_expr(&mut (*curr).pred_right);
            }
            _ => {
                if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"oops -- invalid expression type!\0" as *const u8
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
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"oops -- invalid expression type!\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        if (*(*curr).pred_right).side_effects as i32 == 1 as i32 {
            last_sidep = prevp;
            merge_lists(
                cbo_list.as_mut_ptr(),
                EvaluationCost::NumEvaluationCosts as i32,
                &mut name_list,
                &mut regex_list,
                last_sidep,
            );
            has_side_effects = 1 as i32 != 0;
        }
        prevp = &mut (*curr).pred_left;
    }
    last_sidep = prevp;
    merge_lists(
        cbo_list.as_mut_ptr(),
        EvaluationCost::NumEvaluationCosts as i32,
        &mut name_list,
        &mut regex_list,
        last_sidep,
    );
    return has_side_effects;
}
unsafe extern "C" fn constrain_rate(mut rate: libc::c_float) -> libc::c_float {
    if rate > 1.0f32 {
        return 1.0f64 as libc::c_float
    } else if (rate as libc::c_double) < 0.0f64 {
        return 0.0f64 as libc::c_float
    } else {
        return rate
    };
}
unsafe extern "C" fn set_new_parent(
    mut curr: *mut predicate,
    mut high_prec: predicate_precedence,
    mut prevp: *mut *mut predicate,
) -> *mut predicate {
    let mut new_parent: *mut predicate = 0 as *mut predicate;
    new_parent = xzalloc(::core::mem::size_of::<predicate>() as u64) as *mut predicate;
    (*new_parent).p_type = predicate_type::BI_OP;
    (*new_parent).p_prec = high_prec;
    (*new_parent).p_cost = EvaluationCost::NeedsNothing;
    match high_prec as u32 {
        1 => {
            (*new_parent).pred_func = Some(pred_comma as PREDICATEFUNCTION);
            (*new_parent).p_name = b",\0" as *const u8 as *const i8;
            (*new_parent).est_success_rate = 1.0f64 as libc::c_float;
        }
        2 => {
            (*new_parent).pred_func = Some(pred_or as PREDICATEFUNCTION);
            (*new_parent).p_name = b"-o\0" as *const u8 as *const i8;
            (*new_parent).est_success_rate = constrain_rate((*curr).est_success_rate);
        }
        3 => {
            (*new_parent).pred_func = Some(pred_and as PREDICATEFUNCTION);
            (*new_parent).p_name = b"-a\0" as *const u8 as *const i8;
            (*new_parent).est_success_rate = constrain_rate((*curr).est_success_rate);
        }
        _ => {}
    }
    (*new_parent).pred_right = curr;
    *prevp = new_parent;
    return new_parent;
}
unsafe extern "C" fn merge_pred(
    mut beg_list: *mut predicate,
    mut end_list: *mut predicate,
    mut last_p: *mut *mut predicate,
) {
    (*end_list).pred_left = *last_p;
    *last_p = beg_list;
}
static mut costlookup: [pred_cost_lookup; 59] = unsafe {
    [
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_amin as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_and as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_anewer as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_atime as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_closeparen as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_cmin as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_cnewer as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_comma as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_context as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ctime as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_delete as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsSyncDiskHit,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_empty as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_exec as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsEventualExec,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_execdir as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsEventualExec,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_executable as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_false as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fprint as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fprint0 as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fprintf as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fstype as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_gid as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_group as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ilname as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsLinkName,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_iname as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_inum as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsInodeNumber,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ipath as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_links as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_lname as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsLinkName,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ls as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fls as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_mmin as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_mtime as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_name as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_negate as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_newer as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_newerXY as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_nogroup as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_nouser as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ok as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsUserInteraction,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_okdir as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsUserInteraction,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_openparen as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_or as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_path as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_perm as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_print as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_print0 as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_prune as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(*const i8, *mut stat, *mut predicate) -> !,
                    >,
                    PRED_FUNC,
                >(
                    Some(
                        pred_quit
                            as unsafe extern "C" fn(
                                *const i8,
                                *mut stat,
                                *mut predicate,
                            ) -> !,
                    ),
                ),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_readable as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_regex as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_samefile as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_size as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_true as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_type as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsType,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_uid as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_used as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_user as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_writable as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_xtype as PREDICATEFUNCTION),
                cost: EvaluationCost::NeedsType,
            };
            init
        },
    ]
};
static mut pred_table_sorted: i32 = 0 as i32;
unsafe extern "C" fn check_sorted(
    mut base: *mut libc::c_void,
    mut members: size_t,
    mut membersize: size_t,
    mut cmpfn: Option<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
    >,
) -> bool {
    let mut p: *const i8 = base as *const i8;
    let mut i: size_t = 0;
    i = 1 as u32 as size_t;
    while i < members {
        let mut result: i32 = cmpfn
            .expect(
                "non-null function pointer",
            )(
            p.offset(i.wrapping_mul(membersize) as isize) as *const libc::c_void,
            p.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(membersize) as isize)
                as *const libc::c_void,
        );
        if result < 0 as i32 {
            return 0 as i32 != 0;
        }
        result = cmpfn
            .expect(
                "non-null function pointer",
            )(
            p.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(membersize) as isize)
                as *const libc::c_void,
            p.offset(i.wrapping_mul(membersize) as isize) as *const libc::c_void,
        );
        if result <= 0 as i32 {} else {
            __assert_fail(
                b"result <= 0\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1013 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[i8; 80],
                >(
                    b"_Bool check_sorted(void *, size_t, size_t, int (*)(const void *, const void *))\0",
                ))
                    .as_ptr(),
            );
        }
        'c_10125: {
            if result <= 0 as i32 {} else {
                __assert_fail(
                    b"result <= 0\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1013 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 80],
                        &[i8; 80],
                    >(
                        b"_Bool check_sorted(void *, size_t, size_t, int (*)(const void *, const void *))\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cost_table_comparison(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut pc1: *const pred_cost_lookup = p1 as *const pred_cost_lookup;
    let mut pc2: *const pred_cost_lookup = p2 as *const pred_cost_lookup;
    let mut u1: C2RustUnnamed_1 = C2RustUnnamed_1 { pfn: None };
    let mut u2: C2RustUnnamed_1 = C2RustUnnamed_1 { pfn: None };
    u1.pfn = (*pc1).fn_0;
    u2.pfn = (*pc2).fn_0;
    return memcmp(
        (u1.mem).as_mut_ptr() as *const libc::c_void,
        (u2.mem).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<PRED_FUNC>() as u64,
    );
}
unsafe extern "C" fn get_pred_cost(mut p: *const predicate) -> EvaluationCost {
    let mut data_requirement_cost: EvaluationCost = EvaluationCost::NeedsNothing;
    let mut inherent_cost: EvaluationCost = EvaluationCost::NeedsUnknown;
    if (*p).need_stat {
        data_requirement_cost = EvaluationCost::NeedsStatInfo;
    } else if (*p).need_inum {
        data_requirement_cost = EvaluationCost::NeedsInodeNumber;
    } else if (*p).need_type {
        data_requirement_cost = EvaluationCost::NeedsType;
    } else {
        data_requirement_cost = EvaluationCost::NeedsNothing;
    }
    if (*p).pred_func == Some(pred_exec as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_execdir as PREDICATEFUNCTION)
    {
        if (*p).args.exec_vec.multiple {
            inherent_cost = EvaluationCost::NeedsEventualExec;
        } else {
            inherent_cost = EvaluationCost::NeedsImmediateExec;
        }
    } else if (*p).pred_func == Some(pred_fprintf as PREDICATEFUNCTION) {
        inherent_cost = (*p).p_cost;
    } else {
        let mut key: pred_cost_lookup = pred_cost_lookup {
            fn_0: None,
            cost: EvaluationCost::NeedsNothing,
        };
        let mut entry: *mut libc::c_void = 0 as *mut libc::c_void;
        if pred_table_sorted == 0 {
            qsort(
                costlookup.as_mut_ptr() as *mut libc::c_void,
                (::core::mem::size_of::<[pred_cost_lookup; 59]>() as u64)
                    .wrapping_div(::core::mem::size_of::<pred_cost_lookup>() as u64),
                ::core::mem::size_of::<pred_cost_lookup>() as u64,
                Some(
                    cost_table_comparison
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> i32,
                ),
            );
            if !check_sorted(
                costlookup.as_mut_ptr() as *mut libc::c_void,
                (::core::mem::size_of::<[pred_cost_lookup; 59]>() as u64)
                    .wrapping_div(::core::mem::size_of::<pred_cost_lookup>() as u64),
                ::core::mem::size_of::<pred_cost_lookup>() as u64,
                Some(
                    cost_table_comparison
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> i32,
                ),
            ) {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        b"failed to sort the costlookup array\0" as *const u8
                            as *const i8,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as i32,
                        0 as i32,
                        b"failed to sort the costlookup array\0" as *const u8
                            as *const i8,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            pred_table_sorted = 1 as i32;
        }
        key.fn_0 = (*p).pred_func;
        entry = bsearch(
            &mut key as *mut pred_cost_lookup as *const libc::c_void,
            costlookup.as_mut_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[pred_cost_lookup; 59]>() as u64)
                .wrapping_div(::core::mem::size_of::<pred_cost_lookup>() as u64),
            ::core::mem::size_of::<pred_cost_lookup>() as u64,
            Some(
                cost_table_comparison
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
        if !entry.is_null() {
            inherent_cost = (*(entry as *const pred_cost_lookup)).cost;
        } else {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*p).p_name,
            );
            inherent_cost = EvaluationCost::NeedsUnknown;
        }
    }
    if inherent_cost as u32 > data_requirement_cost as u32 {
        return inherent_cost
    } else {
        return data_requirement_cost
    };
}
unsafe extern "C" fn estimate_costs(mut tree: *mut predicate) {
    if !tree.is_null() {
        estimate_costs((*tree).pred_right);
        estimate_costs((*tree).pred_left);
        (*tree).p_cost = get_pred_cost(tree);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_eval_tree() -> *mut predicate {
    return eval_tree;
}
unsafe extern "C" fn getrate(mut p: *const predicate) -> libc::c_float {
    if !p.is_null() { return (*p).est_success_rate } else { return 1.0f32 };
}
#[no_mangle]
pub unsafe extern "C" fn calculate_derived_rates(
    mut p: *mut predicate,
) -> libc::c_float {
    if !p.is_null() {} else {
        __assert_fail(
            b"NULL != p\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1156 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_8132: {
        if !p.is_null() {} else {
            __assert_fail(
                b"NULL != p\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1156 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"float calculate_derived_rates(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*p).pred_right).is_null() {
        calculate_derived_rates((*p).pred_right);
    }
    if !((*p).pred_left).is_null() {
        calculate_derived_rates((*p).pred_left);
    }
    if (*p).p_type as u32 != predicate_type::CLOSE_PAREN as i32 as u32 {} else {
        __assert_fail(
            b"p->p_type != predicate_type::CLOSE_PAREN\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1163 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_8064: {
        if (*p).p_type as u32 != predicate_type::CLOSE_PAREN as i32 as u32 {} else {
            __assert_fail(
                b"p->p_type != predicate_type::CLOSE_PAREN\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1163 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"float calculate_derived_rates(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*p).p_type as u32 != predicate_type::OPEN_PAREN as i32 as u32 {} else {
        __assert_fail(
            b"p->p_type != predicate_type::OPEN_PAREN\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1164 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_8019: {
        if (*p).p_type as u32 != predicate_type::OPEN_PAREN as i32 as u32 {} else {
            __assert_fail(
                b"p->p_type != predicate_type::OPEN_PAREN\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1164 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"float calculate_derived_rates(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    match (*p).p_type as u32 {
        0 => {
            if ((*p).pred_right).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_right\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1169 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_7968: {
                if ((*p).pred_right).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_right\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1169 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if ((*p).pred_left).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_left\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1170 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_7920: {
                if ((*p).pred_left).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_left\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1170 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            return (*p).est_success_rate;
        }
        1 => {
            if ((*p).pred_right).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_right\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1174 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_7869: {
                if ((*p).pred_right).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_right\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1174 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if ((*p).pred_left).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_left\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1175 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_7821: {
                if ((*p).pred_left).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_left\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1175 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            return (*p).est_success_rate;
        }
        2 => {
            if (*p).pred_func == Some(pred_negate as PREDICATEFUNCTION) {} else {
                __assert_fail(
                    b"pred_is (p, pred_negate)\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1180 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_7766: {
                if (*p).pred_func == Some(pred_negate as PREDICATEFUNCTION) {} else {
                    __assert_fail(
                        b"pred_is (p, pred_negate)\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1180 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if ((*p).pred_left).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_left\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1181 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_7717: {
                if ((*p).pred_left).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_left\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1181 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*p).est_success_rate = (1.0f64
                - (*(*p).pred_right).est_success_rate as libc::c_double)
                as libc::c_float;
            return (*p).est_success_rate;
        }
        3 => {
            let mut rate: libc::c_float = 0.;
            if (*p).pred_func == Some(pred_and as PREDICATEFUNCTION) {
                rate = getrate((*p).pred_right) * getrate((*p).pred_left);
            } else if (*p).pred_func == Some(pred_comma as PREDICATEFUNCTION) {
                rate = 1.0f32;
            } else if (*p).pred_func == Some(pred_or as PREDICATEFUNCTION) {
                rate = getrate((*p).pred_right) + getrate((*p).pred_left);
            } else {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1204 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
                'c_7578: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const i8,
                        b"tree.c\0" as *const u8 as *const i8,
                        1204 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                };
                abort();
            }
            (*p).est_success_rate = constrain_rate(rate);
            return (*p).est_success_rate;
        }
        4 | 5 => {
            (*p).est_success_rate = 1.0f64 as libc::c_float;
            return (*p).est_success_rate;
        }
        _ => {}
    }
    __assert_fail(
        b"0\0" as *const u8 as *const i8,
        b"tree.c\0" as *const u8 as *const i8,
        1216 as i32 as u32,
        (*::core::mem::transmute::<
            &[u8; 50],
            &[i8; 50],
        >(b"float calculate_derived_rates(struct predicate *)\0"))
            .as_ptr(),
    );
    'c_7457: {
        __assert_fail(
            b"0\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1216 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    };
    abort();
}
unsafe extern "C" fn check_normalization(mut p: *mut predicate, mut at_root: bool) {
    if at_root {
        if predicate_type::BI_OP as i32 as u32 == (*p).p_type as u32 {} else {
            __assert_fail(
                b"predicate_type::BI_OP == p->p_type\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1230 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[i8; 52],
                >(b"void check_normalization(struct predicate *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_6568: {
            if predicate_type::BI_OP as i32 as u32 == (*p).p_type as u32 {} else {
                __assert_fail(
                    b"predicate_type::BI_OP == p->p_type\0" as *const u8 as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1230 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 52],
                        &[i8; 52],
                    >(b"void check_normalization(struct predicate *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if !((*p).pred_left).is_null() {
        if predicate_type::BI_OP as i32 as u32 == (*(*p).pred_left).p_type as u32
        {} else {
            __assert_fail(
                b"predicate_type::BI_OP == p->pred_left->p_type\0" as *const u8
                    as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1235 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[i8; 52],
                >(b"void check_normalization(struct predicate *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_6510: {
            if predicate_type::BI_OP as i32 as u32 == (*(*p).pred_left).p_type as u32
            {} else {
                __assert_fail(
                    b"predicate_type::BI_OP == p->pred_left->p_type\0" as *const u8
                        as *const i8,
                    b"tree.c\0" as *const u8 as *const i8,
                    1235 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 52],
                        &[i8; 52],
                    >(b"void check_normalization(struct predicate *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
        check_normalization((*p).pred_left, 0 as i32 != 0);
    }
    if !((*p).pred_right).is_null() {
        check_normalization((*p).pred_right, 0 as i32 != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn build_expression_tree(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut end_of_leading_options: i32,
) -> *mut predicate {
    let mut parse_entry: *const parser_table = 0 as *const parser_table;
    let mut predicate_name: *mut i8 = 0 as *mut i8;
    let mut cur_pred: *mut predicate = 0 as *mut predicate;
    let mut entry_close: *const parser_table = 0 as *const parser_table;
    let mut entry_print: *const parser_table = 0 as *const parser_table;
    let mut entry_open: *const parser_table = 0 as *const parser_table;
    let mut i: i32 = 0;
    let mut oldi: i32 = 0;
    predicates = 0 as *mut predicate;
    start_points = argv.offset(end_of_leading_options as isize);
    i = end_of_leading_options;
    while i < argc && !looks_like_expression(*argv.offset(i as isize), 1 as i32 != 0) {
        num_start_points = num_start_points.wrapping_add(1);
        num_start_points;
        i += 1;
        i;
    }
    entry_open = find_parser(b"(\0" as *const u8 as *const i8);
    entry_close = find_parser(b")\0" as *const u8 as *const i8);
    entry_print = find_parser(b"print\0" as *const u8 as *const i8);
    if !entry_open.is_null() {} else {
        __assert_fail(
            b"entry_open != NULL\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1270 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_13158: {
        if !entry_open.is_null() {} else {
            __assert_fail(
                b"entry_open != NULL\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1270 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !entry_close.is_null() {} else {
        __assert_fail(
            b"entry_close != NULL\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1271 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_13116: {
        if !entry_close.is_null() {} else {
            __assert_fail(
                b"entry_close != NULL\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1271 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !entry_print.is_null() {} else {
        __assert_fail(
            b"entry_print != NULL\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1272 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_13072: {
        if !entry_print.is_null() {} else {
            __assert_fail(
                b"entry_print != NULL\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1272 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    parse_openparen(entry_open, argv, &mut argc);
    (*last_pred).p_name = b"(\0" as *const u8 as *const i8;
    (*predicates).artificial = 1 as i32 != 0;
    parse_begin_user_args(argv, argc, last_pred, predicates);
    pred_sanity_check(last_pred);
    while i < argc {
        state.already_issued_stat_error_msg = 0 as i32 != 0;
        if !looks_like_expression(*argv.offset(i as isize), 0 as i32 != 0) {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"paths must precede expression: `%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *argv.offset(i as isize),
            );
            if access(*argv.offset(i as isize), 0 as i32) == 0 as i32 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"possible unquoted pattern after predicate `%s'?\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*last_pred).p_name,
                );
            }
            exit(1 as i32);
        }
        predicate_name = *argv.offset(i as isize);
        parse_entry = find_parser(predicate_name);
        if parse_entry.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_24>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"unknown predicate `%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    predicate_name,
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
                        b"unknown predicate `%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    predicate_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        if (*parse_entry).type_0 as u32 != arg_type::ARG_SPECIAL_PARSE as i32 as u32 {
            i += 1;
            i;
        }
        oldi = i;
        if !(Some(((*parse_entry).parser_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(parse_entry, argv, &mut i)
        {
            if !(*argv.offset(i as isize)).is_null() {
                if arg_type::ARG_SPECIAL_PARSE as i32 as u32
                    == (*parse_entry).type_0 as u32 && i == oldi
                {
                    if ::core::mem::size_of::<C2RustUnnamed_23>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"invalid predicate `%s'\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            predicate_name,
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
                                b"invalid predicate `%s'\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            predicate_name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                } else {
                    if ::core::mem::size_of::<C2RustUnnamed_22>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"invalid argument `%s' to `%s'\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            *argv.offset(i as isize),
                            predicate_name,
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
                                b"invalid argument `%s' to `%s'\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            *argv.offset(i as isize),
                            predicate_name,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            } else {
                if ::core::mem::size_of::<C2RustUnnamed_21>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"missing argument to `%s'\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        predicate_name,
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
                            b"missing argument to `%s'\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        predicate_name,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
        } else {
            (*last_pred).p_name = predicate_name;
            if i != oldi {
                (*last_pred).arg_text = *argv.offset(oldi as isize);
            } else {
                (*last_pred).arg_text = 0 as *const i8;
            }
        }
        pred_sanity_check(last_pred);
        pred_sanity_check(predicates);
    }
    parse_end_user_args(argv, argc, last_pred, predicates);
    if ((*predicates).pred_next).is_null() {
        cur_pred = predicates;
        last_pred = (*predicates).pred_next;
        predicates = last_pred;
        rpl_free(cur_pred as *mut libc::c_void);
        parse_print(entry_print, argv, &mut argc);
        (*last_pred).p_name = b"-print\0" as *const u8 as *const i8;
        pred_sanity_check(last_pred);
        pred_sanity_check(predicates);
    } else if !default_prints((*predicates).pred_next) {
        cur_pred = predicates;
        predicates = (*predicates).pred_next;
        pred_sanity_check(predicates);
        rpl_free(cur_pred as *mut libc::c_void);
    } else {
        parse_closeparen(entry_close, argv, &mut argc);
        (*last_pred).p_name = b")\0" as *const u8 as *const i8;
        (*last_pred).artificial = 1 as i32 != 0;
        pred_sanity_check(last_pred);
        parse_print(entry_print, argv, &mut argc);
        (*last_pred).p_name = b"-print\0" as *const u8 as *const i8;
        (*last_pred).artificial = 1 as i32 != 0;
        pred_sanity_check(last_pred);
        pred_sanity_check(predicates);
    }
    if options.debug_options & (DebugExpressionTree as i32 | DebugTreeOpt as i32) as u64
        != 0
    {
        fprintf(stderr, b"Predicate List:\n\0" as *const u8 as *const i8);
        print_list(stderr, predicates);
    }
    check_option_combinations(predicates);
    pred_sanity_check(predicates);
    cur_pred = predicates;
    eval_tree = get_expr(
        &mut cur_pred,
        predicate_precedence::NO_PREC as i32 as libc::c_short,
        0 as *const predicate,
    );
    calculate_derived_rates(eval_tree);
    if !cur_pred.is_null() {
        if (*cur_pred).pred_func == Some(pred_closeparen as PREDICATEFUNCTION) {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"you have too many ')'\0" as *const u8 as *const i8,
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
                        b"you have too many ')'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else if !((*cur_pred).p_name).is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"unexpected extra predicate '%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*cur_pred).p_name,
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
                        b"unexpected extra predicate '%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*cur_pred).p_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"unexpected extra predicate\0" as *const u8 as *const i8,
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
                        b"unexpected extra predicate\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if options.debug_options & (DebugExpressionTree as i32 | DebugTreeOpt as i32) as u64
        != 0
    {
        fprintf(stderr, b"Eval Tree:\n\0" as *const u8 as *const i8);
        print_tree(stderr, eval_tree, 0 as i32);
    }
    estimate_costs(eval_tree);
    opt_expr(&mut eval_tree);
    check_normalization(eval_tree, 1 as i32 != 0);
    do_arm_swaps(eval_tree);
    check_normalization(eval_tree, 1 as i32 != 0);
    if options.debug_options & (DebugExpressionTree as i32 | DebugTreeOpt as i32) as u64
        != 0
    {
        fprintf(stderr, b"Optimized Eval Tree:\n\0" as *const u8 as *const i8);
        print_tree(stderr, eval_tree, 0 as i32);
        fprintf(stderr, b"Optimized command line:\n\0" as *const u8 as *const i8);
        print_optlist(stderr, eval_tree);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    }
    return eval_tree;
}
unsafe extern "C" fn init_pred_perf(mut pred: *mut predicate) {
    let mut p: *mut predicate_performance_info = &mut (*pred).perf;
    (*p).successes = 0 as i32 as u64;
    (*p).visits = (*p).successes;
}
#[no_mangle]
pub unsafe extern "C" fn get_new_pred_noarg(
    mut entry: *const parser_table,
) -> *mut predicate {
    let mut p: *mut predicate = get_new_pred(entry);
    if !p.is_null() {
        (*p).arg_text = 0 as *const i8;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn get_new_pred(mut entry: *const parser_table) -> *mut predicate {
    let mut new_pred: *mut predicate = 0 as *mut predicate;
    if (*entry).type_0 as u32 != arg_type::ARG_OPTION as i32 as u32 {} else {
        __assert_fail(
            b"entry->type != arg_type::ARG_OPTION\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1485 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                .as_ptr(),
        );
    }
    'c_13471: {
        if (*entry).type_0 as u32 != arg_type::ARG_OPTION as i32 as u32 {} else {
            __assert_fail(
                b"entry->type != arg_type::ARG_OPTION\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1485 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*entry).type_0 as u32 != arg_type::ARG_POSITIONAL_OPTION as i32 as u32 {} else {
        __assert_fail(
            b"entry->type != arg_type::ARG_POSITIONAL_OPTION\0" as *const u8
                as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1486 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                .as_ptr(),
        );
    }
    'c_13425: {
        if (*entry).type_0 as u32 != arg_type::ARG_POSITIONAL_OPTION as i32 as u32
        {} else {
            __assert_fail(
                b"entry->type != arg_type::ARG_POSITIONAL_OPTION\0" as *const u8
                    as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1486 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                    .as_ptr(),
            );
        }
    };
    new_pred = xzalloc(::core::mem::size_of::<predicate>() as u64) as *mut predicate;
    if predicates.is_null() {
        predicates = new_pred;
        last_pred = predicates;
    } else {
        (*last_pred).pred_next = new_pred;
        last_pred = new_pred;
    }
    (*last_pred).parser_entry = entry;
    (*last_pred).p_type = predicate_type::NO_TYPE;
    (*last_pred).p_prec = predicate_precedence::NO_PREC;
    (*last_pred).need_stat = 1 as i32 != 0;
    (*last_pred).need_type = 1 as i32 != 0;
    (*last_pred).p_cost = EvaluationCost::NeedsUnknown;
    (*last_pred).arg_text = b"ThisShouldBeSetToSomethingElse\0" as *const u8
        as *const i8;
    (*last_pred).literal_control_chars = options.literal_control_chars;
    (*last_pred).est_success_rate = 1.0f64 as libc::c_float;
    init_pred_perf(last_pred);
    return last_pred;
}
#[no_mangle]
pub unsafe extern "C" fn get_new_pred_chk_op(
    mut entry: *const parser_table,
    mut arg: *const i8,
) -> *mut predicate {
    let mut new_pred: *mut predicate = 0 as *mut predicate;
    static mut entry_and: *const parser_table = 0 as *const parser_table;
    if entry_and.is_null() {
        entry_and = find_parser(b"and\0" as *const u8 as *const i8);
    }
    if !entry_and.is_null() {} else {
        __assert_fail(
            b"entry_and != NULL\0" as *const u8 as *const i8,
            b"tree.c\0" as *const u8 as *const i8,
            1528 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[i8; 81],
            >(
                b"struct predicate *get_new_pred_chk_op(const struct parser_table *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13750: {
        if !entry_and.is_null() {} else {
            __assert_fail(
                b"entry_and != NULL\0" as *const u8 as *const i8,
                b"tree.c\0" as *const u8 as *const i8,
                1528 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 81],
                    &[i8; 81],
                >(
                    b"struct predicate *get_new_pred_chk_op(const struct parser_table *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !last_pred.is_null() {
        match (*last_pred).p_type as u32 {
            0 => {
                if ::core::mem::size_of::<C2RustUnnamed_25>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"oops -- invalid default insertion of and!\0" as *const u8
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
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"oops -- invalid default insertion of and!\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            }
            1 | 5 => {
                new_pred = get_new_pred_noarg(entry_and);
                (*new_pred).pred_func = Some(pred_and as PREDICATEFUNCTION);
                (*new_pred).p_name = b"-a\0" as *const u8 as *const i8;
                (*new_pred).p_type = predicate_type::BI_OP;
                (*new_pred).p_prec = predicate_precedence::AND_PREC;
                (*new_pred).need_stat = 0 as i32 != 0;
                (*new_pred).need_type = 0 as i32 != 0;
                (*new_pred).need_inum = 0 as i32 != 0;
                (*new_pred).arg_text = 0 as *const i8;
                (*new_pred).args.str_0 = 0 as *const i8;
                (*new_pred).side_effects = 0 as i32 != 0;
                (*new_pred).no_default_print = 0 as i32 != 0;
            }
            _ => {}
        }
    }
    new_pred = get_new_pred(entry);
    (*new_pred).arg_text = arg;
    (*new_pred).parser_entry = entry;
    return new_pred;
}
#[no_mangle]
pub static mut cost_table: [cost_assoc; 11] = [
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsNothing,
            name: b"Nothing\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsInodeNumber,
            name: b"InodeNumber\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsType,
            name: b"Type\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsStatInfo,
            name: b"StatInfo\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsLinkName,
            name: b"LinkName\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsAccessInfo,
            name: b"AccessInfo\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsSyncDiskHit,
            name: b"SyncDiskHit\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsEventualExec,
            name: b"EventualExec\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsImmediateExec,
            name: b"ImmediateExec\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsUserInteraction,
            name: b"UserInteraction\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: EvaluationCost::NeedsUnknown,
            name: b"Unknown\0" as *const u8 as *const i8,
        };
        init
    },
];
static mut prec_table: [prec_assoc; 7] = [
    {
        let mut init = prec_assoc {
            prec: predicate_precedence::NO_PREC as i32 as libc::c_short,
            prec_name: b"no\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: predicate_precedence::COMMA_PREC as i32 as libc::c_short,
            prec_name: b"comma\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: predicate_precedence::OR_PREC as i32 as libc::c_short,
            prec_name: b"or\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: predicate_precedence::AND_PREC as i32 as libc::c_short,
            prec_name: b"and\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: predicate_precedence::NEGATE_PREC as i32 as libc::c_short,
            prec_name: b"negate\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: predicate_precedence::MAX_PREC as i32 as libc::c_short,
            prec_name: b"max\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: -(1 as i32) as libc::c_short,
            prec_name: b"unknown \0" as *const u8 as *const i8,
        };
        init
    },
];
static mut type_table: [op_assoc; 7] = [
    {
        let mut init = op_assoc {
            type_0: predicate_type::NO_TYPE as i32 as libc::c_short,
            type_name: b"no\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: predicate_type::PRIMARY_TYPE as i32 as libc::c_short,
            type_name: b"primary\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: predicate_type::UNI_OP as i32 as libc::c_short,
            type_name: b"uni_op\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: predicate_type::BI_OP as i32 as libc::c_short,
            type_name: b"bi_op\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: predicate_type::OPEN_PAREN as i32 as libc::c_short,
            type_name: b"open_paren  \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: predicate_type::CLOSE_PAREN as i32 as libc::c_short,
            type_name: b"close_paren \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: -(1 as i32) as libc::c_short,
            type_name: b"unknown\0" as *const u8 as *const i8,
        };
        init
    },
];
unsafe extern "C" fn cost_name(mut cost: EvaluationCost) -> *const i8 {
    let mut i: u32 = 0;
    let mut n: u32 = (::core::mem::size_of::<[cost_assoc; 11]>() as u64)
        .wrapping_div(::core::mem::size_of::<cost_assoc>() as u64) as u32;
    i = 0 as i32 as u32;
    while i < n {
        if cost_table[i as usize].cost as u32 == cost as u32 {
            return cost_table[i as usize].name;
        }
        i = i.wrapping_add(1);
        i;
    }
    return b"unknown\0" as *const u8 as *const i8;
}
unsafe extern "C" fn type_name(mut type_0: libc::c_short) -> *const i8 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while type_table[i as usize].type_0 as i32 != -(1 as i32) as libc::c_short as i32 {
        if type_table[i as usize].type_0 as i32 == type_0 as i32 {
            break;
        }
        i += 1;
        i;
    }
    return type_table[i as usize].type_name;
}
unsafe extern "C" fn prec_name(mut prec: libc::c_short) -> *const i8 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while prec_table[i as usize].prec as i32 != -(1 as i32) as libc::c_short as i32 {
        if prec_table[i as usize].prec as i32 == prec as i32 {
            break;
        }
        i += 1;
        i;
    }
    return prec_table[i as usize].prec_name;
}
#[no_mangle]
pub unsafe extern "C" fn print_tree(
    mut fp: *mut FILE,
    mut node: *mut predicate,
    mut indent: i32,
) {
    let mut i: i32 = 0;
    if node.is_null() {
        return;
    }
    i = 0 as i32;
    while i < indent {
        fprintf(fp, b"    \0" as *const u8 as *const i8);
        i += 1;
        i;
    }
    fprintf(fp, b"pred=[\0" as *const u8 as *const i8);
    print_predicate(fp, node);
    fprintf(
        fp,
        b"] type=%s prec=%s\0" as *const u8 as *const i8,
        type_name((*node).p_type as libc::c_short),
        prec_name((*node).p_prec as libc::c_short),
    );
    fprintf(
        fp,
        b" cost=%s est_success_rate=%#.4g %sside effects \0" as *const u8 as *const i8,
        cost_name((*node).p_cost),
        (*node).est_success_rate as libc::c_double,
        if (*node).side_effects as i32 != 0 {
            b"\0" as *const u8 as *const i8
        } else {
            b"no \0" as *const u8 as *const i8
        },
    );
    if (*node).need_stat as i32 != 0 || (*node).need_type as i32 != 0
        || (*node).need_inum as i32 != 0
    {
        let mut comma: i32 = 0 as i32;
        fprintf(fp, b"Needs \0" as *const u8 as *const i8);
        if (*node).need_stat {
            fprintf(fp, b"stat\0" as *const u8 as *const i8);
            comma = 1 as i32;
        }
        if (*node).need_inum {
            fprintf(
                fp,
                b"%sinode\0" as *const u8 as *const i8,
                if comma != 0 {
                    b",\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
            comma = 1 as i32;
        }
        if (*node).need_type {
            fprintf(
                fp,
                b"%stype\0" as *const u8 as *const i8,
                if comma != 0 {
                    b",\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
        }
    }
    fprintf(fp, b"\n\0" as *const u8 as *const i8);
    i = 0 as i32;
    while i < indent {
        fprintf(fp, b"    \0" as *const u8 as *const i8);
        i += 1;
        i;
    }
    if ((*node).pred_left).is_null() && ((*node).pred_right).is_null() {
        fprintf(fp, b"no children.\n\0" as *const u8 as *const i8);
    } else {
        if !((*node).pred_left).is_null() {
            fprintf(fp, b"left:\n\0" as *const u8 as *const i8);
            print_tree(fp, (*node).pred_left, indent + 1 as i32);
        } else {
            fprintf(fp, b"no left.\n\0" as *const u8 as *const i8);
        }
        i = 0 as i32;
        while i < indent {
            fprintf(fp, b"    \0" as *const u8 as *const i8);
            i += 1;
            i;
        }
        if !((*node).pred_right).is_null() {
            fprintf(fp, b"right:\n\0" as *const u8 as *const i8);
            print_tree(fp, (*node).pred_right, indent + 1 as i32);
        } else {
            fprintf(fp, b"no right.\n\0" as *const u8 as *const i8);
        }
    };
}