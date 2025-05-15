use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type quoting_options;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn set_cloexec_flag(desc: i32, value: bool) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn base_name(file: *const i8) -> *mut i8;
    fn mdir_name(file: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn openat_safer(_: i32, _: *const i8, _: i32, _: ...) -> i32;
    fn open_safer(_: *const i8, _: i32, _: ...) -> i32;
    fn restore_cwd(cwd: *const saved_cwd) -> i32;
    fn free_cwd(cwd: *mut saved_cwd);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn bc_do_insert(
        ctl: *mut buildcmd_control,
        state_0: *mut buildcmd_state,
        arg: *mut i8,
        arglen: size_t,
        prefix: *const i8,
        pfxlen: size_t,
        linebuf: *const i8,
        lblen: size_t,
        initial_args: i32,
    );
    fn bc_do_exec(ctl: *mut buildcmd_control, state_0: *mut buildcmd_state);
    fn bc_push_arg(
        ctl: *mut buildcmd_control,
        state_0: *mut buildcmd_state,
        arg: *const i8,
        len: size_t,
        prefix: *const i8,
        pfxlen: size_t,
        initial_args: i32,
    );
    fn bc_args_exceed_testing_limit(argv: *mut *mut i8) -> bool;
    fn close(__fd: i32) -> i32;
    fn execvp(__file: *const i8, __argv: *const *mut i8) -> i32;
    fn _exit(_: i32) -> !;
    fn fork() -> __pid_t;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn safely_quote_err_filename(n: i32, arg: *const i8) -> *const i8;
    fn is_exec_in_local_dir(pred_func: PRED_FUNC) -> bool;
    static mut state: state;
    static mut initial_wd: *mut saved_cwd;
    static mut options: options;
    fn complain_about_leaky_fds();
    fn fd_leak_check_is_enabled() -> bool;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
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
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
pub type uintmax_t = __uintmax_t;
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
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: i32,
}
unsafe extern "C" fn initialize_wd_for_exec(
    mut execp: *mut exec_val,
    mut cwd_fd: i32,
    mut dir: *const i8,
) -> bool {
    (*execp).wd_for_exec = xmalloc(::core::mem::size_of::<saved_cwd>() as u64)
        as *mut saved_cwd;
    (*(*execp).wd_for_exec).name = 0 as *mut i8;
    (*(*execp).wd_for_exec).desc = openat_safer(cwd_fd, dir, 0 as i32);
    if (*(*execp).wd_for_exec).desc < 0 as i32 {
        return 0 as i32 != 0;
    }
    set_cloexec_flag((*(*execp).wd_for_exec).desc, 1 as i32 != 0);
    return 1 as i32 != 0;
}
unsafe extern "C" fn record_exec_dir(mut execp: *mut exec_val) -> bool {
    if (*execp).state.todo == 0 {
        if (*execp).state.todo == 0 {} else {
            __assert_fail(
                b"!execp->state.todo\0" as *const u8 as *const i8,
                b"exec.c\0" as *const u8 as *const i8,
                71 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"_Bool record_exec_dir(struct exec_val *)\0"))
                    .as_ptr(),
            );
        }
        'c_7047: {
            if (*execp).state.todo == 0 {} else {
                __assert_fail(
                    b"!execp->state.todo\0" as *const u8 as *const i8,
                    b"exec.c\0" as *const u8 as *const i8,
                    71 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[i8; 41],
                    >(b"_Bool record_exec_dir(struct exec_val *)\0"))
                        .as_ptr(),
                );
            }
        };
        if !(strchr(state.rel_pathname, '/' as i32)).is_null() {
            let mut dir: *mut i8 = mdir_name(state.rel_pathname);
            let mut result: bool = initialize_wd_for_exec(execp, state.cwd_dir_fd, dir);
            rpl_free(dir as *mut libc::c_void);
            return result;
        } else {
            return initialize_wd_for_exec(
                execp,
                state.cwd_dir_fd,
                b".\0" as *const u8 as *const i8,
            )
        }
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_pred_exec(
    mut pathname: *const i8,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut execp: *mut exec_val = &mut (*pred_ptr).args.exec_vec;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut target: *const i8 = 0 as *const i8;
    let mut result: bool = false;
    let local: bool = is_exec_in_local_dir((*pred_ptr).pred_func);
    let mut prefix: *const i8 = 0 as *const i8;
    let mut pfxlen: size_t = 0;
    if local {
        if !record_exec_dir(execp) {
            if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"Failed to save working directory in order to run a command on %s\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    safely_quote_err_filename(0 as i32, pathname),
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
                        b"Failed to save working directory in order to run a command on %s\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    safely_quote_err_filename(0 as i32, pathname),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        buf = base_name(state.rel_pathname);
        target = buf;
        if '/' as i32 == *target.offset(0 as i32 as isize) as i32 {
            prefix = 0 as *const i8;
            pfxlen = 0 as i32 as size_t;
        } else {
            prefix = b"./\0" as *const u8 as *const i8;
            pfxlen = 2 as u32 as size_t;
        }
    } else {
        if (*execp).wd_for_exec == initial_wd {} else {
            __assert_fail(
                b"execp->wd_for_exec == initial_wd\0" as *const u8 as *const i8,
                b"exec.c\0" as *const u8 as *const i8,
                148 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6700: {
            if (*execp).wd_for_exec == initial_wd {} else {
                __assert_fail(
                    b"execp->wd_for_exec == initial_wd\0" as *const u8 as *const i8,
                    b"exec.c\0" as *const u8 as *const i8,
                    148 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 70],
                        &[i8; 70],
                    >(
                        b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        target = pathname;
        prefix = 0 as *const i8;
        pfxlen = 0 as u32 as size_t;
    }
    if (*execp).multiple {
        bc_push_arg(
            &mut (*execp).ctl,
            &mut (*execp).state,
            target,
            (strlen(target)).wrapping_add(1 as i32 as u64),
            prefix,
            pfxlen,
            0 as i32,
        );
        if (*execp).state.todo != 0 {
            state.execdirs_outstanding = 1 as i32 != 0;
        }
        result = 1 as i32 != 0;
    } else {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*execp).num_args {
            bc_do_insert(
                &mut (*execp).ctl,
                &mut (*execp).state,
                *((*execp).replace_vec).offset(i as isize),
                strlen(*((*execp).replace_vec).offset(i as isize)),
                prefix,
                pfxlen,
                target,
                strlen(target),
                0 as i32,
            );
            i += 1;
            i;
        }
        bc_do_exec(&mut (*execp).ctl, &mut (*execp).state);
        if (*execp).last_child_status & 0x7f as i32 == 0 as i32 {
            if 0 as i32 == ((*execp).last_child_status & 0xff00 as i32) >> 8 as i32 {
                result = 1 as i32 != 0;
            } else {
                result = 0 as i32 != 0;
            }
        } else {
            result = 0 as i32 != 0;
        }
        if local {
            free_cwd((*execp).wd_for_exec);
        }
    }
    if !buf.is_null() {
        if local {} else {
            __assert_fail(
                b"local\0" as *const u8 as *const i8,
                b"exec.c\0" as *const u8 as *const i8,
                208 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6429: {
            if local {} else {
                __assert_fail(
                    b"local\0" as *const u8 as *const i8,
                    b"exec.c\0" as *const u8 as *const i8,
                    208 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 70],
                        &[i8; 70],
                    >(
                        b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        rpl_free(buf as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn prep_child_for_exec(
    mut close_stdin: bool,
    mut wd: *const saved_cwd,
) -> bool {
    let mut ok: bool = 1 as i32 != 0;
    if close_stdin {
        let inputfile: [i8; 10] = *::core::mem::transmute::<
            &[u8; 10],
            &[i8; 10],
        >(b"/dev/null\0");
        if close(0 as i32) < 0 as i32 {
            error(
                0 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Cannot close standard input\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            ok = 0 as i32 != 0;
        } else if open_safer(inputfile.as_ptr(), 0 as i32 | 0 as i32) < 0 as i32 {
            error(
                0 as i32,
                *__errno_location(),
                b"%s\0" as *const u8 as *const i8,
                safely_quote_err_filename(0 as i32, inputfile.as_ptr()),
            );
        }
    }
    if 0 as i32 != restore_cwd(wd) {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"Failed to change directory%s%s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            if (*wd).desc < 0 as i32 && !((*wd).name).is_null() {
                b": \0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if (*wd).desc < 0 as i32 && !((*wd).name).is_null() {
                (*wd).name
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
        ok = 0 as i32 != 0;
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn launch(
    mut ctl: *mut buildcmd_control,
    mut usercontext: *mut libc::c_void,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut child_pid: pid_t = 0;
    static mut first_time: i32 = 1 as i32;
    let mut execp: *mut exec_val = usercontext as *mut exec_val;
    if options.debug_options & DebugExec as i32 as u64 != 0 {
        let mut i: i32 = 0;
        fprintf(
            stderr,
            b"DebugExec: launching process (argc=%lu):\0" as *const u8 as *const i8,
            ((*execp).state.cmd_argc).wrapping_sub(1 as i32 as u64),
        );
        i = 0 as i32;
        while (i as u64) < ((*execp).state.cmd_argc).wrapping_sub(1 as i32 as u64) {
            fprintf(
                stderr,
                b" %s\0" as *const u8 as *const i8,
                safely_quote_err_filename(
                    0 as i32,
                    *((*execp).state.cmd_argv).offset(i as isize),
                ),
            );
            i += 1;
            i;
        }
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    }
    rpl_fflush(stdout);
    rpl_fflush(stderr);
    if first_time != 0 {
        first_time = 0 as i32;
        signal(17 as i32, None);
    }
    child_pid = fork();
    if child_pid == -(1 as i32) {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"cannot fork\0" as *const u8 as *const i8,
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
                    b"cannot fork\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    if child_pid == 0 as i32 {
        if !((*execp).wd_for_exec).is_null() {} else {
            __assert_fail(
                b"NULL != execp->wd_for_exec\0" as *const u8 as *const i8,
                b"exec.c\0" as *const u8 as *const i8,
                321 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"int launch(struct buildcmd_control *, void *, int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_7599: {
            if !((*execp).wd_for_exec).is_null() {} else {
                __assert_fail(
                    b"NULL != execp->wd_for_exec\0" as *const u8 as *const i8,
                    b"exec.c\0" as *const u8 as *const i8,
                    321 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 60],
                        &[i8; 60],
                    >(b"int launch(struct buildcmd_control *, void *, int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        if !prep_child_for_exec((*execp).close_stdin, (*execp).wd_for_exec) {
            _exit(1 as i32);
        } else if fd_leak_check_is_enabled() {
            complain_about_leaky_fds();
        }
        if bc_args_exceed_testing_limit(argv) {
            *__errno_location() = 7 as i32;
        } else {
            execvp(*argv.offset(0 as i32 as isize), argv as *const *mut i8);
        }
        error(
            0 as i32,
            *__errno_location(),
            b"%s\0" as *const u8 as *const i8,
            safely_quote_err_filename(0 as i32, *argv.offset(0 as i32 as isize)),
        );
        _exit(1 as i32);
    }
    while waitpid(child_pid, &mut (*execp).last_child_status, 0 as i32) == -(1 as i32) {
        if *__errno_location() != 4 as i32 {
            error(
                0 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"error waiting for %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                safely_quote_err_filename(0 as i32, *argv.offset(0 as i32 as isize)),
            );
            state.exit_status = 1 as i32;
            return 0 as i32;
        }
    }
    if (((*execp).last_child_status & 0x7f as i32) + 1 as i32) as libc::c_schar as i32
        >> 1 as i32 > 0 as i32
    {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s terminated by signal %d\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quotearg_n_style(
                0 as i32,
                options.err_quoting_style,
                *argv.offset(0 as i32 as isize),
            ),
            (*execp).last_child_status & 0x7f as i32,
        );
        if (*execp).multiple {
            state.exit_status = 1 as i32;
        }
        return 1 as i32;
    }
    let mut ex: i32 = ((*execp).last_child_status & 0xff00 as i32) >> 8 as i32;
    if options.debug_options & DebugExec as i32 as u64 != 0 {
        fprintf(
            stderr,
            b"DebugExec: process (PID=%ld) terminated with exit status: %d\n\0"
                as *const u8 as *const i8,
            child_pid as i64,
            ex,
        );
    }
    if 0 as i32 == ex {
        return 1 as i32
    } else {
        if (*execp).multiple {
            state.exit_status = 1 as i32;
        }
        return 1 as i32;
    };
}