use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type quoting_options;
    pub type re_dfa_t;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strtok_r(__s: *mut i8, __delim: *const i8, __save_ptr: *mut *mut i8) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    fn time(__timer: *mut time_t) -> time_t;
    fn __fxstatat(
        __ver: i32,
        __fildes: i32,
        __filename: *const i8,
        __stat_buf: *mut stat,
        __flag: i32,
    ) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn uname(__name: *mut utsname) -> i32;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn forget_non_cloexec_fds();
    fn complain_about_leaky_fds();
    fn fd_leak_check_is_enabled() -> bool;
    static mut program_name: *const i8;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn save_cwd(cwd: *mut saved_cwd) -> i32;
    fn restore_cwd(cwd: *const saved_cwd) -> i32;
    fn free_cwd(cwd: *mut saved_cwd);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn _exit(_: i32) -> !;
    fn isatty(__fd: i32) -> i32;
    fn bc_do_exec(ctl: *mut buildcmd_control, state_0: *mut buildcmd_state);
    fn sharefile_destroy(_: sharefile_handle);
    static mut state: state;
    static mut options: options;
    fn set_follow_state(opt: SymlinkOption);
    static mut initial_wd: *mut saved_cwd;
    fn pred_fprint0(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fls(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprintf(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn get_eval_tree() -> *mut predicate;
    fn pred_okdir(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_execdir(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_exec(_: *const i8, _: *mut stat, _: *mut predicate) -> bool;
    fn get_new_pred_chk_op(entry: *const parser_table, arg: *const i8) -> *mut predicate;
    fn run_in_dir(
        _: *const saved_cwd,
        callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
        usercontext: *mut libc::c_void,
    ) -> i32;
    fn explain_how_to_report_bugs(f: *mut FILE, program_name_0: *const i8) -> i32;
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
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [i8; 65],
    pub nodename: [i8; 65],
    pub release: [i8; 65],
    pub version: [i8; 65],
    pub machine: [i8; 65],
    pub domainname: [i8; 65],
}
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
pub type uintmax_t = __uintmax_t;
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
pub const DebugStat: DebugOption = 2;
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
pub type PREDICATEFUNCTION = unsafe extern "C" fn(
    *const i8,
    *mut stat,
    *mut predicate,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_option_assoc {
    pub name: *const i8,
    pub val: i32,
    pub docstring: *const i8,
}
pub const DebugHelp: DebugOption = 16;
pub const DebugAll: DebugOption = -17;
pub const DebugExpressionTree: DebugOption = 1;
pub const DebugTime: DebugOption = 128;
pub const DebugSearch: DebugOption = 4;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugExec: DebugOption = 32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: i32,
}
pub type DebugOption = i32;
pub const DebugNone: DebugOption = 0;
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
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
static mut debugassoc: [debug_option_assoc; 9] = [
    {
        let mut init = debug_option_assoc {
            name: b"exec\0" as *const u8 as *const i8,
            val: DebugExec as i32,
            docstring: b"Show diagnostic information relating to -exec, -execdir, -ok and -okdir\0"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"opt\0" as *const u8 as *const i8,
            val: DebugExpressionTree as i32 | DebugTreeOpt as i32,
            docstring: b"Show diagnostic information relating to optimisation\0"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"rates\0" as *const u8 as *const i8,
            val: DebugSuccessRates as i32,
            docstring: b"Indicate how often each predicate succeeded\0" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"search\0" as *const u8 as *const i8,
            val: DebugSearch as i32,
            docstring: b"Navigate the directory tree verbosely\0" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"stat\0" as *const u8 as *const i8,
            val: DebugStat as i32,
            docstring: b"Trace calls to stat(2) and lstat(2)\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"time\0" as *const u8 as *const i8,
            val: DebugTime as i32,
            docstring: b"Show diagnostic information relating to time-of-day and timestamp comparisons\0"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"tree\0" as *const u8 as *const i8,
            val: DebugExpressionTree as i32,
            docstring: b"Display the expression tree\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"all\0" as *const u8 as *const i8,
            val: DebugAll as i32,
            docstring: b"Set all of the debug flags (but help)\0" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"help\0" as *const u8 as *const i8,
            val: DebugHelp as i32,
            docstring: b"Explain the various -D options\0" as *const u8 as *const i8,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn insert_primary_withpred(
    mut entry: *const parser_table,
    mut pred_func: PRED_FUNC,
    mut arg: *const i8,
) -> *mut predicate {
    let mut new_pred: *mut predicate = 0 as *mut predicate;
    new_pred = get_new_pred_chk_op(entry, arg);
    (*new_pred).pred_func = pred_func;
    (*new_pred).p_name = (*entry).parser_name;
    (*new_pred).args.str_0 = 0 as *const i8;
    (*new_pred).p_type = predicate_type::PRIMARY_TYPE;
    (*new_pred).p_prec = predicate_precedence::NO_PREC;
    return new_pred;
}
#[no_mangle]
pub unsafe extern "C" fn insert_primary(
    mut entry: *const parser_table,
    mut arg: *const i8,
) -> *mut predicate {
    if ((*entry).pred_func).is_some() {} else {
        __assert_fail(
            b"entry->pred_func != NULL\0" as *const u8 as *const i8,
            b"util.c\0" as *const u8 as *const i8,
            123 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[i8; 76],
            >(
                b"struct predicate *insert_primary(const struct parser_table *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7367: {
        if ((*entry).pred_func).is_some() {} else {
            __assert_fail(
                b"entry->pred_func != NULL\0" as *const u8 as *const i8,
                b"util.c\0" as *const u8 as *const i8,
                123 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[i8; 76],
                >(
                    b"struct predicate *insert_primary(const struct parser_table *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return insert_primary_withpred(entry, (*entry).pred_func, arg);
}
#[no_mangle]
pub unsafe extern "C" fn insert_primary_noarg(
    mut entry: *const parser_table,
) -> *mut predicate {
    return insert_primary(entry, 0 as *const i8);
}
unsafe extern "C" fn show_valid_debug_options(mut full: i32) {
    let mut i: size_t = 0;
    fputs(
        dcgettext(
            0 as *const i8,
            b"Valid arguments for -D:\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    if full != 0 {
        i = 0 as i32 as size_t;
        while i
            < (::core::mem::size_of::<[debug_option_assoc; 9]>() as u64)
                .wrapping_div(::core::mem::size_of::<debug_option_assoc>() as u64)
        {
            fprintf(
                stdout,
                b"%-10s %s\n\0" as *const u8 as *const i8,
                debugassoc[i as usize].name,
                debugassoc[i as usize].docstring,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as i32 as size_t;
        while i
            < (::core::mem::size_of::<[debug_option_assoc; 9]>() as u64)
                .wrapping_div(::core::mem::size_of::<debug_option_assoc>() as u64)
        {
            fprintf(
                stdout,
                b"%s%s\0" as *const u8 as *const i8,
                if i > 0 as i32 as u64 {
                    b", \0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
                debugassoc[i as usize].name,
            );
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: i32) -> ! {
    if status != 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Try '%s --help' for more information.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nOther common options:\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"      --help                   display this help and exit\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"      --version                output version information and exit\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    show_valid_debug_options(0 as i32);
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nUse '-D help' for a description of the options, or see find(1)\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn set_stat_placeholders(mut p: *mut stat) {}
#[no_mangle]
pub unsafe extern "C" fn get_statinfo(
    mut pathname: *const i8,
    mut name: *const i8,
    mut p: *mut stat,
) -> i32 {
    if !state.have_stat {
        set_stat_placeholders(p);
        if 0 as i32
            == (Some((options.xstat).expect("non-null function pointer")))
                .expect("non-null function pointer")(name, p)
        {
            if 0 as i32 as u32 == (*p).st_mode {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"WARNING: file %s appears to have mode 0000\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    quotearg_n_style(0 as i32, options.err_quoting_style, name),
                );
                state.exit_status = 1 as i32;
            }
        } else {
            if !options.ignore_readdir_race || *__errno_location() != 2 as i32 {
                nonfatal_target_file_error(*__errno_location(), pathname);
            }
            return -(1 as i32);
        }
    }
    state.have_stat = 1 as i32 != 0;
    state.have_type = 1 as i32 != 0;
    state.type_0 = (*p).st_mode;
    return 0 as i32;
}
unsafe extern "C" fn get_info(
    mut pathname: *const i8,
    mut p: *mut stat,
    mut pred_ptr: *mut predicate,
) -> i32 {
    let mut todo: bool = 0 as i32 != 0;
    if (*pred_ptr).need_stat as i32 != 0 && !state.have_stat {
        todo = 1 as i32 != 0;
    } else if (*pred_ptr).need_type as i32 != 0 && !state.have_type {
        todo = 1 as i32 != 0;
    } else if (*pred_ptr).need_inum {
        if (*p).st_ino == 0 {
            todo = 1 as i32 != 0;
        } else if !state.have_type
            || (*p).st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
        {
            todo = 1 as i32 != 0;
        }
    }
    if todo {
        if get_statinfo(pathname, state.rel_pathname, p) != 0 as i32 {
            return -(1 as i32);
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn check_nofollow() -> bool {
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut release: libc::c_float = 0.;
    if 0 as i32 == 0o400000 as i32 {
        return 0 as i32 != 0;
    }
    if 0 as i32 == uname(&mut uts) {
        let mut conversion: Option<unsafe extern "C" fn(*const i8) -> libc::c_double> = Some(
            atof as unsafe extern "C" fn(*const i8) -> libc::c_double,
        );
        release = conversion
            .expect("non-null function pointer")((uts.release).as_mut_ptr())
            as libc::c_float;
        if 0 as i32
            == strcmp(b"Linux\0" as *const u8 as *const i8, (uts.sysname).as_mut_ptr())
        {
            return release >= 2.2f32
        } else if 0 as i32
            == strcmp(b"FreeBSD\0" as *const u8 as *const i8, (uts.sysname).as_mut_ptr())
        {
            return release >= 3.1f32
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn exec_cb(mut context: *mut libc::c_void) -> i32 {
    let mut execp: *mut exec_val = context as *mut exec_val;
    bc_do_exec(&mut (*execp).ctl, &mut (*execp).state);
    return 0 as i32;
}
unsafe extern "C" fn do_exec(mut execp: *mut exec_val) {
    run_in_dir(
        (*execp).wd_for_exec,
        Some(exec_cb as unsafe extern "C" fn(*mut libc::c_void) -> i32),
        execp as *mut libc::c_void,
    );
    if (*execp).wd_for_exec != initial_wd {
        free_cwd((*execp).wd_for_exec);
        rpl_free((*execp).wd_for_exec as *mut libc::c_void);
        (*execp).wd_for_exec = 0 as *mut saved_cwd;
    }
}
unsafe extern "C" fn do_complete_pending_execdirs(mut p: *mut predicate) {
    if p.is_null() {
        return;
    }
    if state.execdirs_outstanding {} else {
        __assert_fail(
            b"state.execdirs_outstanding\0" as *const u8 as *const i8,
            b"util.c\0" as *const u8 as *const i8,
            396 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[i8; 54],
            >(b"void do_complete_pending_execdirs(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_6856: {
        if state.execdirs_outstanding {} else {
            __assert_fail(
                b"state.execdirs_outstanding\0" as *const u8 as *const i8,
                b"util.c\0" as *const u8 as *const i8,
                396 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[i8; 54],
                >(b"void do_complete_pending_execdirs(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    do_complete_pending_execdirs((*p).pred_left);
    if (*p).pred_func == Some(pred_execdir as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_okdir as PREDICATEFUNCTION)
    {
        if (*p).args.exec_vec.multiple {
            let mut execp: *mut exec_val = &mut (*p).args.exec_vec;
            if (*execp).state.todo != 0 {
                do_exec(execp);
            }
        }
    }
    do_complete_pending_execdirs((*p).pred_right);
}
#[no_mangle]
pub unsafe extern "C" fn complete_pending_execdirs() {
    if state.execdirs_outstanding {
        do_complete_pending_execdirs(get_eval_tree());
        state.execdirs_outstanding = 0 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn complete_pending_execs(mut p: *mut predicate) {
    if p.is_null() {
        return;
    }
    complete_pending_execs((*p).pred_left);
    if (*p).pred_func == Some(pred_exec as PREDICATEFUNCTION)
        && (*p).args.exec_vec.multiple as i32 != 0
    {
        let mut execp: *mut exec_val = &mut (*p).args.exec_vec;
        if (*execp).state.todo != 0 {
            bc_do_exec(&mut (*execp).ctl, &mut (*execp).state);
        }
    }
    complete_pending_execs((*p).pred_right);
}
#[no_mangle]
pub unsafe extern "C" fn record_initial_cwd() {
    initial_wd = xmalloc(::core::mem::size_of::<saved_cwd>() as u64) as *mut saved_cwd;
    if 0 as i32 != save_cwd(initial_wd) {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Failed to save initial working directory%s%s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (if (*initial_wd).desc < 0 as i32 && !((*initial_wd).name).is_null() {
                    b": \0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                }),
                (if (*initial_wd).desc < 0 as i32 && !((*initial_wd).name).is_null() {
                    (*initial_wd).name
                } else {
                    b"\0" as *const u8 as *const i8
                }),
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
                    b"Failed to save initial working directory%s%s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (if (*initial_wd).desc < 0 as i32 && !((*initial_wd).name).is_null() {
                    b": \0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                }),
                (if (*initial_wd).desc < 0 as i32 && !((*initial_wd).name).is_null() {
                    (*initial_wd).name
                } else {
                    b"\0" as *const u8 as *const i8
                }),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn cleanup_initial_cwd() {
    if 0 as i32 == restore_cwd(initial_wd) {
        free_cwd(initial_wd);
        rpl_free(initial_wd as *mut libc::c_void);
        initial_wd = 0 as *mut saved_cwd;
    } else {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"Failed to restore initial working directory%s%s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            if (*initial_wd).desc < 0 as i32 && !((*initial_wd).name).is_null() {
                b": \0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if (*initial_wd).desc < 0 as i32 && !((*initial_wd).name).is_null() {
                (*initial_wd).name
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
        _exit(1 as i32);
    };
}
unsafe extern "C" fn traverse_tree(
    mut tree: *mut predicate,
    mut callback: Option<unsafe extern "C" fn(*mut predicate) -> ()>,
) {
    if !((*tree).pred_left).is_null() {
        traverse_tree((*tree).pred_left, callback);
    }
    callback.expect("non-null function pointer")(tree);
    if !((*tree).pred_right).is_null() {
        traverse_tree((*tree).pred_right, callback);
    }
}
unsafe extern "C" fn undangle_file_pointers(mut p: *mut predicate) {
    if (*p).pred_func == Some(pred_fprint as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_fprintf as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_fls as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_fprint0 as PREDICATEFUNCTION)
    {
        (*p).args.printf_vec.stream = 0 as *mut FILE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cleanup() {
    let mut eval_tree: *mut predicate = get_eval_tree();
    if !eval_tree.is_null() {
        traverse_tree(
            eval_tree,
            Some(complete_pending_execs as unsafe extern "C" fn(*mut predicate) -> ()),
        );
        complete_pending_execdirs();
    }
    sharefile_destroy(state.shared_files);
    if !eval_tree.is_null() {
        traverse_tree(
            eval_tree,
            Some(undangle_file_pointers as unsafe extern "C" fn(*mut predicate) -> ()),
        );
    }
    cleanup_initial_cwd();
    if fd_leak_check_is_enabled() {
        complain_about_leaky_fds();
        forget_non_cloexec_fds();
    }
    if rpl_fflush(stdout) == -(1 as i32) {
        nonfatal_nontarget_file_error(
            *__errno_location(),
            b"standard output\0" as *const u8 as *const i8,
        );
    }
}
unsafe extern "C" fn fallback_stat(
    mut name: *const i8,
    mut p: *mut stat,
    mut prev_rv: i32,
) -> i32 {
    match *__errno_location() {
        2 | 20 => {
            if options.debug_options & DebugStat as i32 as u64 != 0 {
                fprintf(
                    stderr,
                    b"fallback_stat(): stat(%s) failed; falling back on lstat()\n\0"
                        as *const u8 as *const i8,
                    name,
                );
            }
            return fstatat(state.cwd_dir_fd, name, p, 0x100 as i32);
        }
        13 | 5 | 40 | 36 | 75 | _ => return prev_rv,
    };
}
#[no_mangle]
pub unsafe extern "C" fn optionh_stat(mut name: *const i8, mut p: *mut stat) -> i32 {
    if -(100 as i32) != state.cwd_dir_fd {
        if state.cwd_dir_fd >= 0 as i32 {} else {
            __assert_fail(
                b"state.cwd_dir_fd >= 0\0" as *const u8 as *const i8,
                b"util.c\0" as *const u8 as *const i8,
                605 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[i8; 46],
                >(b"int optionh_stat(const char *, struct stat *)\0"))
                    .as_ptr(),
            );
        }
        'c_5755: {
            if state.cwd_dir_fd >= 0 as i32 {} else {
                __assert_fail(
                    b"state.cwd_dir_fd >= 0\0" as *const u8 as *const i8,
                    b"util.c\0" as *const u8 as *const i8,
                    605 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 46],
                        &[i8; 46],
                    >(b"int optionh_stat(const char *, struct stat *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    set_stat_placeholders(p);
    if 0 as i32 == state.curdepth {
        let mut rv: i32 = 0;
        rv = fstatat(state.cwd_dir_fd, name, p, 0 as i32);
        if 0 as i32 == rv { return 0 as i32 } else { return fallback_stat(name, p, rv) }
    } else {
        return fstatat(state.cwd_dir_fd, name, p, 0x100 as i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn optionl_stat(mut name: *const i8, mut p: *mut stat) -> i32 {
    let mut rv: i32 = 0;
    if -(100 as i32) != state.cwd_dir_fd {
        if state.cwd_dir_fd >= 0 as i32 {} else {
            __assert_fail(
                b"state.cwd_dir_fd >= 0\0" as *const u8 as *const i8,
                b"util.c\0" as *const u8 as *const i8,
                636 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[i8; 46],
                >(b"int optionl_stat(const char *, struct stat *)\0"))
                    .as_ptr(),
            );
        }
        'c_5560: {
            if state.cwd_dir_fd >= 0 as i32 {} else {
                __assert_fail(
                    b"state.cwd_dir_fd >= 0\0" as *const u8 as *const i8,
                    b"util.c\0" as *const u8 as *const i8,
                    636 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 46],
                        &[i8; 46],
                    >(b"int optionl_stat(const char *, struct stat *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    set_stat_placeholders(p);
    rv = fstatat(state.cwd_dir_fd, name, p, 0 as i32);
    if 0 as i32 == rv { return 0 as i32 } else { return fallback_stat(name, p, rv) };
}
#[no_mangle]
pub unsafe extern "C" fn optionp_stat(mut name: *const i8, mut p: *mut stat) -> i32 {
    if state.cwd_dir_fd >= 0 as i32 || state.cwd_dir_fd == -(100 as i32) {} else {
        __assert_fail(
            b"(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)\0" as *const u8
                as *const i8,
            b"util.c\0" as *const u8 as *const i8,
            653 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[i8; 46],
            >(b"int optionp_stat(const char *, struct stat *)\0"))
                .as_ptr(),
        );
    }
    'c_5632: {
        if state.cwd_dir_fd >= 0 as i32 || state.cwd_dir_fd == -(100 as i32) {} else {
            __assert_fail(
                b"(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)\0" as *const u8
                    as *const i8,
                b"util.c\0" as *const u8 as *const i8,
                653 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[i8; 46],
                >(b"int optionp_stat(const char *, struct stat *)\0"))
                    .as_ptr(),
            );
        }
    };
    set_stat_placeholders(p);
    return fstatat(state.cwd_dir_fd, name, p, 0x100 as i32);
}
static mut stat_count: uintmax_t = 0 as u32 as uintmax_t;
#[no_mangle]
pub unsafe extern "C" fn debug_stat(mut file: *const i8, mut bufp: *mut stat) -> i32 {
    stat_count = stat_count.wrapping_add(1);
    stat_count;
    fprintf(stderr, b"debug_stat (%s)\n\0" as *const u8 as *const i8, file);
    match options.symlink_handling as u32 {
        1 => return optionl_stat(file, bufp),
        2 => return optionh_stat(file, bufp),
        0 => return optionp_stat(file, bufp),
        _ => {}
    }
    __assert_fail(
        b"0\0" as *const u8 as *const i8,
        b"util.c\0" as *const u8 as *const i8,
        677 as i32 as u32,
        (*::core::mem::transmute::<
            &[u8; 44],
            &[i8; 44],
        >(b"int debug_stat(const char *, struct stat *)\0"))
            .as_ptr(),
    );
    'c_5809: {
        __assert_fail(
            b"0\0" as *const u8 as *const i8,
            b"util.c\0" as *const u8 as *const i8,
            677 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[i8; 44],
            >(b"int debug_stat(const char *, struct stat *)\0"))
                .as_ptr(),
        );
    };
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn following_links() -> bool {
    match options.symlink_handling as u32 {
        1 => return 1 as i32 != 0,
        2 => return state.curdepth == 0 as i32,
        0 | _ => return 0 as i32 != 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn digest_mode(
    mut mode: *mut mode_t,
    mut pathname: *const i8,
    mut name: *const i8,
    mut pstat: *mut stat,
    mut leaf: bool,
) -> bool {
    if *mode != 0 {
        if *mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32
            && following_links() as i32 != 0
        {
            if get_statinfo(pathname, name, pstat) != 0 as i32 {
                return 0 as i32 != 0;
            }
            state.type_0 = (*pstat).st_mode;
            *mode = state.type_0;
            state.have_type = 1 as i32 != 0;
        } else {
            state.have_type = 1 as i32 != 0;
            state.type_0 = *mode;
            (*pstat).st_mode = state.type_0;
        }
    } else if leaf {
        state.have_stat = 0 as i32 != 0;
        state.have_type = 0 as i32 != 0;
        state.type_0 = 0 as i32 as mode_t;
    } else {
        if get_statinfo(pathname, name, pstat) != 0 as i32 {
            return 0 as i32 != 0;
        }
        *mode = (*pstat).st_mode;
        state.type_0 = *mode;
        state.have_type = 1 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn default_prints(mut pred: *mut predicate) -> bool {
    while !pred.is_null() {
        if (*pred).no_default_print {
            return 0 as i32 != 0;
        }
        pred = (*pred).pred_next;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn looks_like_expression(
    mut arg: *const i8,
    mut leading: bool,
) -> bool {
    match *arg.offset(0 as i32 as isize) as i32 {
        45 => {
            if *arg.offset(1 as i32 as isize) != 0 {
                return 1 as i32 != 0
            } else {
                return 0 as i32 != 0
            }
        }
        41 | 44 => {
            if *arg.offset(1 as i32 as isize) != 0 {
                return 0 as i32 != 0
            } else {
                return !leading
            }
        }
        33 | 40 => {
            if *arg.offset(1 as i32 as isize) != 0 {
                return 0 as i32 != 0
            } else {
                return 1 as i32 != 0
            }
        }
        _ => return 0 as i32 != 0,
    };
}
unsafe extern "C" fn process_debug_options(mut arg: *mut i8) {
    let mut p: *const i8 = 0 as *const i8;
    let mut token_context: *mut i8 = 0 as *mut i8;
    let delimiters: [i8; 2] = *::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b",\0");
    let mut empty: bool = 1 as i32 != 0;
    let mut i: size_t = 0;
    p = strtok_r(arg, delimiters.as_ptr(), &mut token_context);
    while !p.is_null() {
        empty = 0 as i32 != 0;
        i = 0 as i32 as size_t;
        while i
            < (::core::mem::size_of::<[debug_option_assoc; 9]>() as u64)
                .wrapping_div(::core::mem::size_of::<debug_option_assoc>() as u64)
        {
            if 0 as i32 == strcmp(debugassoc[i as usize].name, p) {
                options.debug_options |= debugassoc[i as usize].val as u64;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if i
            >= (::core::mem::size_of::<[debug_option_assoc; 9]>() as u64)
                .wrapping_div(::core::mem::size_of::<debug_option_assoc>() as u64)
        {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Ignoring unrecognised debug flag %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, options.err_quoting_style, arg),
            );
        }
        p = strtok_r(0 as *mut i8, delimiters.as_ptr(), &mut token_context);
    }
    if empty {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Empty argument to the -D option.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(1 as i32);
    } else if options.debug_options & DebugHelp as i32 as u64 != 0 {
        show_valid_debug_options(1 as i32);
        exit(0 as i32);
    }
}
unsafe extern "C" fn process_optimisation_option(mut arg: *const i8) {
    if 0 as i32 == *arg.offset(0 as i32 as isize) as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_7>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"The -O option must be immediately followed by a decimal integer\0"
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
                    b"The -O option must be immediately followed by a decimal integer\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    } else {
        let mut opt_level: u64 = 0;
        let mut end: *mut i8 = 0 as *mut i8;
        if *(*__ctype_b_loc())
            .offset(*arg.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 == 0
        {
            if ::core::mem::size_of::<C2RustUnnamed_6>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Please specify a decimal number immediately after -O\0"
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
                        b"Please specify a decimal number immediately after -O\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            let mut prev_errno: i32 = *__errno_location();
            *__errno_location() = 0 as i32;
            opt_level = strtoul(arg, &mut end, 10 as i32);
            if 0 as i32 as u64 == opt_level && end == arg as *mut i8 {
                if ::core::mem::size_of::<C2RustUnnamed_5>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Please specify a decimal number immediately after -O\0"
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
                            b"Please specify a decimal number immediately after -O\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else if *end != 0 {
                if ::core::mem::size_of::<C2RustUnnamed_4>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Invalid optimisation level %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        arg,
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
                            b"Invalid optimisation level %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        arg,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else if (9223372036854775807 as i64 as u64)
                .wrapping_mul(2 as u64)
                .wrapping_add(1 as u64) == opt_level && *__errno_location() != 0
            {
                if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            0 as *const i8,
                            b"Invalid optimisation level %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        arg,
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
                            b"Invalid optimisation level %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        arg,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else if opt_level > (32767 as i32 * 2 as i32 + 1 as i32) as u64 {
                if ::core::mem::size_of::<C2RustUnnamed_2>() as u64 != 0 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        opt_level,
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
                            b"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        opt_level,
                    );
                    if 0 as i32 != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                options.optimisation_level = opt_level as libc::c_ushort;
                *__errno_location() = prev_errno;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn process_leading_options(
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut i: i32 = 0;
    let mut end_of_leading_options: i32 = 0;
    i = 1 as i32;
    loop {
        end_of_leading_options = i;
        if !(end_of_leading_options < argc) {
            break;
        }
        if 0 as i32
            == strcmp(b"-H\0" as *const u8 as *const i8, *argv.offset(i as isize))
        {
            set_follow_state(SymlinkOption::SYMLINK_DEREF_ARGSONLY);
        } else if 0 as i32
            == strcmp(b"-L\0" as *const u8 as *const i8, *argv.offset(i as isize))
        {
            set_follow_state(SymlinkOption::SYMLINK_ALWAYS_DEREF);
        } else if 0 as i32
            == strcmp(b"-P\0" as *const u8 as *const i8, *argv.offset(i as isize))
        {
            set_follow_state(SymlinkOption::SYMLINK_NEVER_DEREF);
        } else if 0 as i32
            == strcmp(b"--\0" as *const u8 as *const i8, *argv.offset(i as isize))
        {
            end_of_leading_options = i + 1 as i32;
            break;
        } else if 0 as i32
            == strcmp(b"-D\0" as *const u8 as *const i8, *argv.offset(i as isize))
        {
            if argc <= i + 1 as i32 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"Missing argument after the -D option.\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                usage(1 as i32);
            }
            process_debug_options(*argv.offset((i + 1 as i32) as isize));
            i += 1;
            i;
        } else if 0 as i32
            == strncmp(
                b"-O\0" as *const u8 as *const i8,
                *argv.offset(i as isize),
                2 as i32 as u64,
            )
        {
            process_optimisation_option(
                (*argv.offset(i as isize)).offset(2 as i32 as isize),
            );
        } else {
            end_of_leading_options = i;
            break;
        }
        i += 1;
        i;
    }
    return end_of_leading_options;
}
unsafe extern "C" fn now() -> timespec {
    let mut retval: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: time_t = 0;
    if 0 as i32 == gettimeofday(&mut tv, 0 as *mut timezone) {
        retval.tv_sec = tv.tv_sec;
        retval.tv_nsec = tv.tv_usec * 1000 as i32 as i64;
        return retval;
    }
    t = time(0 as *mut time_t);
    if t != -(1 as i32) as time_t {} else {
        __assert_fail(
            b"t != (time_t)-1\0" as *const u8 as *const i8,
            b"util.c\0" as *const u8 as *const i8,
            978 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[i8; 26],
            >(b"struct timespec now(void)\0"))
                .as_ptr(),
        );
    }
    'c_9477: {
        if t != -(1 as i32) as time_t {} else {
            __assert_fail(
                b"t != (time_t)-1\0" as *const u8 as *const i8,
                b"util.c\0" as *const u8 as *const i8,
                978 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[i8; 26],
                >(b"struct timespec now(void)\0"))
                    .as_ptr(),
            );
        }
    };
    retval.tv_sec = t;
    retval.tv_nsec = 0 as i32 as __syscall_slong_t;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn set_option_defaults(mut p: *mut options) {
    if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const i8)).is_null() {
        (*p).posixly_correct = 1 as i32 != 0;
    } else {
        (*p).posixly_correct = 0 as i32 != 0;
    }
    (*p).open_nofollow_available = check_nofollow();
    (*p).regex_options = 0 as i32;
    if isatty(0 as i32) != 0 {
        (*p).warnings = 1 as i32 != 0;
        (*p).literal_control_chars = 0 as i32 != 0;
    } else {
        (*p).warnings = 0 as i32 != 0;
        (*p).literal_control_chars = 0 as i32 != 0;
    }
    if (*p).posixly_correct {
        (*p).warnings = 0 as i32 != 0;
    }
    (*p).do_dir_first = 1 as i32 != 0;
    (*p).explicit_depth = 0 as i32 != 0;
    (*p).mindepth = -(1 as i32);
    (*p).maxdepth = (*p).mindepth;
    (*p).start_time = now();
    (*p).cur_day_start.tv_sec = (*p).start_time.tv_sec - 86400 as i32 as i64;
    (*p).cur_day_start.tv_nsec = (*p).start_time.tv_nsec;
    (*p).full_days = 0 as i32 != 0;
    (*p).stay_on_filesystem = 0 as i32 != 0;
    (*p).ignore_readdir_race = 0 as i32 != 0;
    if (*p).posixly_correct {
        (*p).output_block_size = 512 as i32;
    } else {
        (*p).output_block_size = 1024 as i32;
    }
    (*p).debug_options = 0 as u64;
    (*p).optimisation_level = 2 as i32 as libc::c_ushort;
    if !(getenv(b"FIND_BLOCK_SIZE\0" as *const u8 as *const i8)).is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_8>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable\0"
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
                    b"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    (*p).no_leaf_check = 0 as i32 != 0;
    set_follow_state(SymlinkOption::SYMLINK_NEVER_DEREF);
    (*p).err_quoting_style = quoting_style::locale_quoting_style;
    (*p).files0_from = 0 as *const i8;
    (*p).ok_prompt_stdin = 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn apply_predicate(
    mut pathname: *const i8,
    mut stat_buf: *mut stat,
    mut p: *mut predicate,
) -> bool {
    (*p).perf.visits = ((*p).perf.visits).wrapping_add(1);
    (*p).perf.visits;
    if (*p).need_stat as i32 != 0 || (*p).need_type as i32 != 0
        || (*p).need_inum as i32 != 0
    {
        if get_info(pathname, stat_buf, p) != 0 as i32 {
            return 0 as i32 != 0;
        }
    }
    if ((*p).pred_func).expect("non-null function pointer")(pathname, stat_buf, p) {
        (*p).perf.successes = ((*p).perf.successes).wrapping_add(1);
        (*p).perf.successes;
        return 1 as i32 != 0;
    } else {
        return 0 as i32 != 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn is_exec_in_local_dir(pred_func: PRED_FUNC) -> bool {
    return Some(pred_execdir as PREDICATEFUNCTION) == pred_func
        || Some(pred_okdir as PREDICATEFUNCTION) == pred_func;
}
#[no_mangle]
pub unsafe extern "C" fn safely_quote_err_filename(
    mut n: i32,
    mut arg: *const i8,
) -> *const i8 {
    return quotearg_n_style(n, options.err_quoting_style, arg);
}
unsafe extern "C" fn report_file_err(
    mut exitval: i32,
    mut errno_value: i32,
    mut is_target_file: bool,
    mut name: *const i8,
) {
    if !is_target_file || !state.already_issued_stat_error_msg {
        error(
            exitval,
            errno_value,
            b"%s\0" as *const u8 as *const i8,
            safely_quote_err_filename(0 as i32, name),
        );
        state.exit_status = 1 as i32;
    }
    if is_target_file {
        state.already_issued_stat_error_msg = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nonfatal_target_file_error(
    mut errno_value: i32,
    mut name: *const i8,
) {
    report_file_err(0 as i32, errno_value, 1 as i32 != 0, name);
}
#[no_mangle]
pub unsafe extern "C" fn fatal_target_file_error(
    mut errno_value: i32,
    mut name: *const i8,
) -> ! {
    report_file_err(1 as i32, errno_value, 1 as i32 != 0, name);
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn nonfatal_nontarget_file_error(
    mut errno_value: i32,
    mut name: *const i8,
) {
    report_file_err(0 as i32, errno_value, 0 as i32 != 0, name);
}
#[no_mangle]
pub unsafe extern "C" fn fatal_nontarget_file_error(
    mut errno_value: i32,
    mut name: *const i8,
) -> ! {
    state.already_issued_stat_error_msg = 0 as i32 != 0;
    report_file_err(1 as i32, errno_value, 0 as i32 != 0, name);
    abort();
}