use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type mode_t = __mode_t;
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
#[no_mangle]
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
    err_quoting_style: quoting_style::literal_quoting_style,
    files0_from: 0 as *const i8,
    ok_prompt_stdin: false,
};
#[no_mangle]
pub static mut state: state = state {
    curdepth: 0,
    have_stat: false,
    have_type: false,
    type_0: 0,
    rel_pathname: 0 as *const i8,
    cwd_dir_fd: 0,
    starting_path_length: 0,
    stop_at_current_level: false,
    exit_status: 0,
    execdirs_outstanding: false,
    shared_files: 0 as *const libc::c_void as *mut libc::c_void,
    already_issued_stat_error_msg: false,
};
#[no_mangle]
pub static mut initial_wd: *mut saved_cwd = 0 as *const saved_cwd as *mut saved_cwd;