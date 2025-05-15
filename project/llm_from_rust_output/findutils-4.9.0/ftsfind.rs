use libc::{c_char, c_int, c_uint, c_ulong, c_void, size_t, time_t, mode_t, dev_t, ino_t, uid_t, gid_t, off_t, blkcnt_t, blksize_t};
use std::ffi::{CString, CStr};
use std::ptr;
use std::os::unix::io::RawFd;
use std::mem;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct stat {
    st_dev: dev_t,
    st_ino: ino_t,
    st_nlink: c_ulong,
    st_mode: mode_t,
    st_uid: uid_t,
    st_gid: gid_t,
    __pad0: c_int,
    st_rdev: dev_t,
    st_size: off_t,
    st_blksize: blksize_t,
    st_blocks: blkcnt_t,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [i64; 3],
}

struct FTS;
struct FTSENT;

struct Options {
    do_dir_first: bool,
    explicit_depth: bool,
    maxdepth: c_int,
    mindepth: c_int,
    no_leaf_check: bool,
    stay_on_filesystem: bool,
    ignore_readdir_race: bool,
    literal_control_chars: bool,
    warnings: bool,
    posixly_correct: bool,
    start_time: timespec,
    cur_day_start: timespec,
    full_days: bool,
    output_block_size: c_int,
    debug_options: c_ulong,
    symlink_handling: SymlinkOption,
    open_nofollow_available: bool,
    regex_options: c_int,
    optimisation_level: c_ushort,
    err_quoting_style: QuotingStyle,
    files0_from: Option<CString>,
    ok_prompt_stdin: bool,
}

#[derive(Debug, Clone, Copy)]
enum SymlinkOption {
    NeverDeref = 0,
    AlwaysDeref = 1,
    DerefArgsOnly = 2,
}

#[derive(Debug, Clone, Copy)]
enum QuotingStyle {
    Literal = 0,
    Shell = 1,
    ShellAlways = 2,
    ShellEscape = 3,
    ShellEscapeAlways = 4,
    C = 5,
    CMaybe = 6,
    Escape = 7,
    Locale = 8,
    Clocale = 9,
    Custom = 10,
}

struct State {
    curdepth: c_int,
    have_stat: bool,
    have_type: bool,
    type_: mode_t,
    rel_pathname: Option<CString>,
    cwd_dir_fd: RawFd,
    starting_path_length: c_int,
    stop_at_current_level: bool,
    exit_status: c_int,
    execdirs_outstanding: bool,
    already_issued_stat_error_msg: bool,
}

static mut OPTIONS: Options = Options {
    do_dir_first: false,
    explicit_depth: false,
    maxdepth: -1,
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
    output_block_size: 512,
    debug_options: 0,
    symlink_handling: SymlinkOption::NeverDeref,
    open_nofollow_available: false,
    regex_options: 0,
    optimisation_level: 0,
    err_quoting_style: QuotingStyle::Locale,
    files0_from: None,
    ok_prompt_stdin: false,
};

static mut STATE: State = State {
    curdepth: 0,
    have_stat: false,
    have_type: false,
    type_: 0,
    rel_pathname: None,
    cwd_dir_fd: -100,
    starting_path_length: 0,
    stop_at_current_level: false,
    exit_status: 0,
    execdirs_outstanding: false,
    already_issued_stat_error_msg: false,
};

fn main() {
    unsafe {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        argv.push(ptr::null());

        let argc = argv.len() as c_int - 1;
        let exit_status = main_0(argc, argv.as_mut_ptr() as *mut *mut c_char);
        std::process::exit(exit_status);
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // Initialize program
    set_program_name(argv);
    record_initial_cwd();
    initialize_state();
    initialize_options();

    // Process arguments and execute find
    let end_of_leading_options = process_leading_options(argc, argv);
    let eval_tree = build_expression_tree(argc, argv, end_of_leading_options);

    if process_all_startpoints(argc - end_of_leading_options, argv.offset(end_of_leading_options as isize)) {
        show_success_rates(eval_tree);
        cleanup();
    }

    STATE.exit_status
}

// Helper functions would be implemented here following Rust safety practices
// Each would be a safe wrapper around the unsafe operations when needed