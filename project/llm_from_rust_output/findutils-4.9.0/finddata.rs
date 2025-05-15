use std::ffi::CString;
use std::os::raw::{c_int, c_uint, c_ulong, c_long, c_char, c_void};
use std::ptr;

#[derive(Copy, Clone)]
pub struct SavedCwd {
    pub desc: c_int,
    pub name: Option<CString>,
}

pub type DevT = c_ulong;
pub type UidT = c_uint;
pub type GidT = c_uint;
pub type InoT = c_ulong;
pub type ModeT = c_uint;
pub type NlinkT = c_ulong;
pub type OffT = c_long;
pub type TimeT = c_long;
pub type BlksizeT = c_long;
pub type BlkcntT = c_long;
pub type SyscallSlongT = c_long;

#[derive(Copy, Clone)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: SyscallSlongT,
}

#[derive(Copy, Clone)]
pub struct Stat {
    pub st_dev: DevT,
    pub st_ino: InoT,
    pub st_nlink: NlinkT,
    pub st_mode: ModeT,
    pub st_uid: UidT,
    pub st_gid: GidT,
    pub __pad0: c_int,
    pub st_rdev: DevT,
    pub st_size: OffT,
    pub st_blksize: BlksizeT,
    pub st_blocks: BlkcntT,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [SyscallSlongT; 3],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QuotingStyle {
    Literal,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
    C,
    CMaybe,
    Escape,
    Locale,
    Clocale,
    Custom,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymlinkOption {
    NeverDeref,
    AlwaysDeref,
    DerefArgsOnly,
}

pub type SharefileHandle = Option<Box<dyn std::any::Any>>;

#[derive(Clone)]
pub struct Options {
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
    pub start_time: Timespec,
    pub cur_day_start: Timespec,
    pub full_days: bool,
    pub output_block_size: c_int,
    pub debug_options: c_ulong,
    pub symlink_handling: SymlinkOption,
    pub xstat: Option<Box<dyn Fn(&str, &mut Stat) -> c_int>>,
    pub open_nofollow_available: bool,
    pub regex_options: c_int,
    pub x_getfilecon: Option<Box<dyn Fn(c_int, &str) -> Result<CString, c_int>>,
    pub optimisation_level: c_uint,
    pub err_quoting_style: QuotingStyle,
    pub files0_from: Option<CString>,
    pub ok_prompt_stdin: bool,
}

#[derive(Clone)]
pub struct State {
    pub curdepth: c_int,
    pub have_stat: bool,
    pub have_type: bool,
    pub type_0: ModeT,
    pub rel_pathname: Option<CString>,
    pub cwd_dir_fd: c_int,
    pub starting_path_length: c_int,
    pub stop_at_current_level: bool,
    pub exit_status: c_int,
    pub execdirs_outstanding: bool,
    pub shared_files: SharefileHandle,
    pub already_issued_stat_error_msg: bool,
}

lazy_static::lazy_static! {
    pub static ref OPTIONS: std::sync::Mutex<Options> = std::sync::Mutex::new(Options {
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
        start_time: Timespec { tv_sec: 0, tv_nsec: 0 },
        cur_day_start: Timespec { tv_sec: 0, tv_nsec: 0 },
        full_days: false,
        output_block_size: 0,
        debug_options: 0,
        symlink_handling: SymlinkOption::NeverDeref,
        xstat: None,
        open_nofollow_available: false,
        regex_options: 0,
        x_getfilecon: None,
        optimisation_level: 0,
        err_quoting_style: QuotingStyle::Literal,
        files0_from: None,
        ok_prompt_stdin: false,
    });

    pub static ref STATE: std::sync::Mutex<State> = std::sync::Mutex::new(State {
        curdepth: 0,
        have_stat: false,
        have_type: false,
        type_0: 0,
        rel_pathname: None,
        cwd_dir_fd: 0,
        starting_path_length: 0,
        stop_at_current_level: false,
        exit_status: 0,
        execdirs_outstanding: false,
        shared_files: None,
        already_issued_stat_error_msg: false,
    });

    pub static ref INITIAL_WD: std::sync::Mutex<Option<SavedCwd>> = std::sync::Mutex::new(None);
}