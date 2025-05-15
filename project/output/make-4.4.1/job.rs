use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __spawn_action;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn remove(__filename: *const i8) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut environ: *mut *mut i8;
    fn execvp(__file: *const i8, __argv: *const *mut i8) -> i32;
    fn confstr(__name: i32, __buf: *mut i8, __len: size_t) -> size_t;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const i8) -> *mut i8;
    fn getloadavg(__loadavg: *mut libc::c_double, __nelem: i32) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strsignal(__sig: i32) -> *mut i8;
    fn stpcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn message(prefix: i32, length: size_t, fmt: *const i8, _: ...);
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn die(_: i32) -> !;
    fn pfatal_with_name(_: *const i8) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn make_toui(_: *const i8, _: *mut *const i8) -> u32;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn show_goal_error();
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut just_print_flag: i32;
    static mut run_silent: i32;
    static mut ignore_errors_flag: i32;
    static mut keep_going_flag: i32;
    static mut question_flag: i32;
    static mut touch_flag: i32;
    static mut warn_undefined_variables_flag: i32;
    static mut posix_pedantic: i32;
    static mut not_parallel: i32;
    static mut one_shell: i32;
    static mut output_sync: i32;
    static mut command_count: u64;
    static mut job_slots: u32;
    static mut max_load_average: libc::c_double;
    fn start_remote_job_p(_: i32) -> i32;
    fn start_remote_job(
        _: *mut *mut i8,
        _: *mut *mut i8,
        _: i32,
        _: *mut i32,
        _: *mut pid_t,
        _: *mut i32,
    ) -> i32;
    fn remote_status(_: *mut i32, _: *mut i32, _: *mut i32, _: i32) -> i32;
    static mut commands_started: u32;
    static mut handling_fatal_signal: sig_atomic_t;
    static mut output_context: *mut output;
    fn output_init(out: *mut output);
    fn output_close(out: *mut output);
    fn output_start();
    fn output_dump(out: *mut output);
    static mut db_level: i32;
    fn set_command_state(file: *mut file, state: cmd_state);
    fn notice_finished_file(file: *mut file);
    fn lookup_file(name: *const i8) -> *mut file;
    fn delete_child_targets(child: *mut child);
    fn chop_commands(cmds: *mut commands);
    fn allocated_variable_expand_for_file(line: *const i8, file: *mut file) -> *mut i8;
    fn shell_completed(exit_code: i32, exit_sig: i32);
    fn lookup_variable_for_file(
        name: *const i8,
        length: size_t,
        file: *mut file,
    ) -> *mut variable;
    fn target_environment(file: *mut file, recursive: i32) -> *mut *mut i8;
    fn fd_noinherit(_: i32);
    fn jobserver_enabled() -> u32;
    fn jobserver_release(is_fatal: i32);
    fn jobserver_signal();
    fn jobserver_pre_child(_: i32);
    fn jobserver_post_child(_: i32);
    fn jobserver_pre_acquire();
    fn jobserver_acquire(timeout: i32) -> u32;
    fn get_bad_stdin() -> i32;
    fn shuffle_get_mode() -> *const i8;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn wait(__stat_loc: *mut i32) -> __pid_t;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> i32;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> i32;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const i8,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut i8,
        __envp: *const *mut i8,
    ) -> i32;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: libc::c_short,
    ) -> i32;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: i32,
        __newfd: i32,
    ) -> i32;
    fn posix_spawnattr_setsigmask(
        __attr: *mut posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> i32;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> i32;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> i32;
    fn find_in_given_path(
        progname: *const i8,
        path: *const i8,
        directory: *const i8,
        optimize_for_exec: bool,
    ) -> *const i8;
    static mut fatal_signal_set: sigset_t;
    static mut shell_function_pid: pid_t;
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
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type __sig_atomic_t = i32;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
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
pub type sig_atomic_t = __sig_atomic_t;
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
pub enum C2RustUnnamed {
    _CS_V7_ENV = 1149,
    _CS_V6_ENV = 1148,
    _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS = 1147,
    _CS_POSIX_V7_LPBIG_OFFBIG_LIBS = 1146,
    _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS = 1145,
    _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS = 1144,
    _CS_POSIX_V7_LP64_OFF64_LINTFLAGS = 1143,
    _CS_POSIX_V7_LP64_OFF64_LIBS = 1142,
    _CS_POSIX_V7_LP64_OFF64_LDFLAGS = 1141,
    _CS_POSIX_V7_LP64_OFF64_CFLAGS = 1140,
    _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS = 1139,
    _CS_POSIX_V7_ILP32_OFFBIG_LIBS = 1138,
    _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS = 1137,
    _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS = 1136,
    _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS = 1135,
    _CS_POSIX_V7_ILP32_OFF32_LIBS = 1134,
    _CS_POSIX_V7_ILP32_OFF32_LDFLAGS = 1133,
    _CS_POSIX_V7_ILP32_OFF32_CFLAGS = 1132,
    _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS = 1131,
    _CS_POSIX_V6_LPBIG_OFFBIG_LIBS = 1130,
    _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS = 1129,
    _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS = 1128,
    _CS_POSIX_V6_LP64_OFF64_LINTFLAGS = 1127,
    _CS_POSIX_V6_LP64_OFF64_LIBS = 1126,
    _CS_POSIX_V6_LP64_OFF64_LDFLAGS = 1125,
    _CS_POSIX_V6_LP64_OFF64_CFLAGS = 1124,
    _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS = 1123,
    _CS_POSIX_V6_ILP32_OFFBIG_LIBS = 1122,
    _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS = 1121,
    _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS = 1120,
    _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS = 1119,
    _CS_POSIX_V6_ILP32_OFF32_LIBS = 1118,
    _CS_POSIX_V6_ILP32_OFF32_LDFLAGS = 1117,
    _CS_POSIX_V6_ILP32_OFF32_CFLAGS = 1116,
    _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS = 1115,
    _CS_XBS5_LPBIG_OFFBIG_LIBS = 1114,
    _CS_XBS5_LPBIG_OFFBIG_LDFLAGS = 1113,
    _CS_XBS5_LPBIG_OFFBIG_CFLAGS = 1112,
    _CS_XBS5_LP64_OFF64_LINTFLAGS = 1111,
    _CS_XBS5_LP64_OFF64_LIBS = 1110,
    _CS_XBS5_LP64_OFF64_LDFLAGS = 1109,
    _CS_XBS5_LP64_OFF64_CFLAGS = 1108,
    _CS_XBS5_ILP32_OFFBIG_LINTFLAGS = 1107,
    _CS_XBS5_ILP32_OFFBIG_LIBS = 1106,
    _CS_XBS5_ILP32_OFFBIG_LDFLAGS = 1105,
    _CS_XBS5_ILP32_OFFBIG_CFLAGS = 1104,
    _CS_XBS5_ILP32_OFF32_LINTFLAGS = 1103,
    _CS_XBS5_ILP32_OFF32_LIBS = 1102,
    _CS_XBS5_ILP32_OFF32_LDFLAGS = 1101,
    _CS_XBS5_ILP32_OFF32_CFLAGS = 1100,
    _CS_LFS64_LINTFLAGS = 1007,
    _CS_LFS64_LIBS = 1006,
    _CS_LFS64_LDFLAGS = 1005,
    _CS_LFS64_CFLAGS = 1004,
    _CS_LFS_LINTFLAGS = 1003,
    _CS_LFS_LIBS = 1002,
    _CS_LFS_LDFLAGS = 1001,
    _CS_LFS_CFLAGS = 1000,
    _CS_V7_WIDTH_RESTRICTED_ENVS = 5,
    _CS_V5_WIDTH_RESTRICTED_ENVS = 4,
    _CS_GNU_LIBPTHREAD_VERSION = 3,
    _CS_GNU_LIBC_VERSION = 2,
    _CS_V6_WIDTH_RESTRICTED_ENVS = 1,
    _CS_PATH = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_CS_V7_ENV => 1149,
            C2RustUnnamed::_CS_V6_ENV => 1148,
            C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS => 1147,
            C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_LIBS => 1146,
            C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS => 1145,
            C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS => 1144,
            C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_LINTFLAGS => 1143,
            C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_LIBS => 1142,
            C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_LDFLAGS => 1141,
            C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_CFLAGS => 1140,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS => 1139,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_LIBS => 1138,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS => 1137,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS => 1136,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS => 1135,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_LIBS => 1134,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_LDFLAGS => 1133,
            C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_CFLAGS => 1132,
            C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS => 1131,
            C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_LIBS => 1130,
            C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS => 1129,
            C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS => 1128,
            C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_LINTFLAGS => 1127,
            C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_LIBS => 1126,
            C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_LDFLAGS => 1125,
            C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_CFLAGS => 1124,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS => 1123,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_LIBS => 1122,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS => 1121,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS => 1120,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS => 1119,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_LIBS => 1118,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_LDFLAGS => 1117,
            C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_CFLAGS => 1116,
            C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_LINTFLAGS => 1115,
            C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_LIBS => 1114,
            C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_LDFLAGS => 1113,
            C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_CFLAGS => 1112,
            C2RustUnnamed::_CS_XBS5_LP64_OFF64_LINTFLAGS => 1111,
            C2RustUnnamed::_CS_XBS5_LP64_OFF64_LIBS => 1110,
            C2RustUnnamed::_CS_XBS5_LP64_OFF64_LDFLAGS => 1109,
            C2RustUnnamed::_CS_XBS5_LP64_OFF64_CFLAGS => 1108,
            C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_LINTFLAGS => 1107,
            C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_LIBS => 1106,
            C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_LDFLAGS => 1105,
            C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_CFLAGS => 1104,
            C2RustUnnamed::_CS_XBS5_ILP32_OFF32_LINTFLAGS => 1103,
            C2RustUnnamed::_CS_XBS5_ILP32_OFF32_LIBS => 1102,
            C2RustUnnamed::_CS_XBS5_ILP32_OFF32_LDFLAGS => 1101,
            C2RustUnnamed::_CS_XBS5_ILP32_OFF32_CFLAGS => 1100,
            C2RustUnnamed::_CS_LFS64_LINTFLAGS => 1007,
            C2RustUnnamed::_CS_LFS64_LIBS => 1006,
            C2RustUnnamed::_CS_LFS64_LDFLAGS => 1005,
            C2RustUnnamed::_CS_LFS64_CFLAGS => 1004,
            C2RustUnnamed::_CS_LFS_LINTFLAGS => 1003,
            C2RustUnnamed::_CS_LFS_LIBS => 1002,
            C2RustUnnamed::_CS_LFS_LDFLAGS => 1001,
            C2RustUnnamed::_CS_LFS_CFLAGS => 1000,
            C2RustUnnamed::_CS_V7_WIDTH_RESTRICTED_ENVS => 5,
            C2RustUnnamed::_CS_V5_WIDTH_RESTRICTED_ENVS => 4,
            C2RustUnnamed::_CS_GNU_LIBPTHREAD_VERSION => 3,
            C2RustUnnamed::_CS_GNU_LIBC_VERSION => 2,
            C2RustUnnamed::_CS_V6_WIDTH_RESTRICTED_ENVS => 1,
            C2RustUnnamed::_CS_PATH => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            1149 => C2RustUnnamed::_CS_V7_ENV,
            1148 => C2RustUnnamed::_CS_V6_ENV,
            1147 => C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS,
            1146 => C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_LIBS,
            1145 => C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS,
            1144 => C2RustUnnamed::_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS,
            1143 => C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_LINTFLAGS,
            1142 => C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_LIBS,
            1141 => C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_LDFLAGS,
            1140 => C2RustUnnamed::_CS_POSIX_V7_LP64_OFF64_CFLAGS,
            1139 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS,
            1138 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_LIBS,
            1137 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS,
            1136 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS,
            1135 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS,
            1134 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_LIBS,
            1133 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_LDFLAGS,
            1132 => C2RustUnnamed::_CS_POSIX_V7_ILP32_OFF32_CFLAGS,
            1131 => C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS,
            1130 => C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_LIBS,
            1129 => C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS,
            1128 => C2RustUnnamed::_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS,
            1127 => C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_LINTFLAGS,
            1126 => C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_LIBS,
            1125 => C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_LDFLAGS,
            1124 => C2RustUnnamed::_CS_POSIX_V6_LP64_OFF64_CFLAGS,
            1123 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS,
            1122 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_LIBS,
            1121 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS,
            1120 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS,
            1119 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS,
            1118 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_LIBS,
            1117 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_LDFLAGS,
            1116 => C2RustUnnamed::_CS_POSIX_V6_ILP32_OFF32_CFLAGS,
            1115 => C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_LINTFLAGS,
            1114 => C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_LIBS,
            1113 => C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_LDFLAGS,
            1112 => C2RustUnnamed::_CS_XBS5_LPBIG_OFFBIG_CFLAGS,
            1111 => C2RustUnnamed::_CS_XBS5_LP64_OFF64_LINTFLAGS,
            1110 => C2RustUnnamed::_CS_XBS5_LP64_OFF64_LIBS,
            1109 => C2RustUnnamed::_CS_XBS5_LP64_OFF64_LDFLAGS,
            1108 => C2RustUnnamed::_CS_XBS5_LP64_OFF64_CFLAGS,
            1107 => C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_LINTFLAGS,
            1106 => C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_LIBS,
            1105 => C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_LDFLAGS,
            1104 => C2RustUnnamed::_CS_XBS5_ILP32_OFFBIG_CFLAGS,
            1103 => C2RustUnnamed::_CS_XBS5_ILP32_OFF32_LINTFLAGS,
            1102 => C2RustUnnamed::_CS_XBS5_ILP32_OFF32_LIBS,
            1101 => C2RustUnnamed::_CS_XBS5_ILP32_OFF32_LDFLAGS,
            1100 => C2RustUnnamed::_CS_XBS5_ILP32_OFF32_CFLAGS,
            1007 => C2RustUnnamed::_CS_LFS64_LINTFLAGS,
            1006 => C2RustUnnamed::_CS_LFS64_LIBS,
            1005 => C2RustUnnamed::_CS_LFS64_LDFLAGS,
            1004 => C2RustUnnamed::_CS_LFS64_CFLAGS,
            1003 => C2RustUnnamed::_CS_LFS_LINTFLAGS,
            1002 => C2RustUnnamed::_CS_LFS_LIBS,
            1001 => C2RustUnnamed::_CS_LFS_LDFLAGS,
            1000 => C2RustUnnamed::_CS_LFS_CFLAGS,
            5 => C2RustUnnamed::_CS_V7_WIDTH_RESTRICTED_ENVS,
            4 => C2RustUnnamed::_CS_V5_WIDTH_RESTRICTED_ENVS,
            3 => C2RustUnnamed::_CS_GNU_LIBPTHREAD_VERSION,
            2 => C2RustUnnamed::_CS_GNU_LIBC_VERSION,
            1 => C2RustUnnamed::_CS_V6_WIDTH_RESTRICTED_ENVS,
            0 => C2RustUnnamed::_CS_PATH,
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const i8,
    pub hname: *const i8,
    pub vpath: *const i8,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const i8,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: u32,
    pub command_flags: i32,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cmd_state {
    cs_finished = 3,
    cs_running = 2,
    cs_deps_running = 1,
    cs_not_started = 0,
}
impl cmd_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cmd_state::cs_finished => 3,
            cmd_state::cs_running => 2,
            cmd_state::cs_deps_running => 1,
            cmd_state::cs_not_started => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> cmd_state {
        match value {
            3 => cmd_state::cs_finished,
            2 => cmd_state::cs_running,
            1 => cmd_state::cs_deps_running,
            0 => cmd_state::cs_not_started,
            _ => panic!("Invalid value for cmd_state: {}", value),
        }
    }
}
impl AddAssign<u32> for cmd_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cmd_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cmd_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cmd_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cmd_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cmd_state {
    type Output = cmd_state;
    fn add(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cmd_state {
    type Output = cmd_state;
    fn sub(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cmd_state {
    type Output = cmd_state;
    fn mul(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cmd_state {
    type Output = cmd_state;
    fn div(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cmd_state {
    type Output = cmd_state;
    fn rem(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum update_status {
    us_failed = 3,
    us_question = 2,
    us_none = 1,
    us_success = 0,
}
impl update_status {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            update_status::us_failed => 3,
            update_status::us_question => 2,
            update_status::us_none => 1,
            update_status::us_success => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> update_status {
        match value {
            3 => update_status::us_failed,
            2 => update_status::us_question,
            1 => update_status::us_none,
            0 => update_status::us_success,
            _ => panic!("Invalid value for update_status: {}", value),
        }
    }
}
impl AddAssign<u32> for update_status {
    fn add_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for update_status {
    fn sub_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for update_status {
    fn mul_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for update_status {
    fn div_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for update_status {
    fn rem_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for update_status {
    type Output = update_status;
    fn add(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for update_status {
    type Output = update_status;
    fn sub(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for update_status {
    type Output = update_status;
    fn mul(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for update_status {
    type Output = update_status;
    fn div(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for update_status {
    type Output = update_status;
    fn rem(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: u64,
    pub ht_capacity: u64,
    pub ht_fill: u64,
    pub ht_empty_slots: u64,
    pub ht_collisions: u64,
    pub ht_lookups: u64,
    pub ht_rehashes: u32,
}
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type hash_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> u64>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const i8,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const i8,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
    pub commands: *mut i8,
    pub command_lines: *mut *mut i8,
    pub lines_flags: *mut u8,
    pub ncommand_lines: libc::c_ushort,
    pub recipe_prefix: i8,
    #[bitfield(name = "any_recurse", ty = "libc::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_origin {
    o_default,
    o_env,
    o_file,
    o_env_override,
    o_command,
    o_override,
    o_automatic,
    o_invalid,
}
impl variable_origin {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_origin::o_default => 0,
            variable_origin::o_env => 1,
            variable_origin::o_file => 2,
            variable_origin::o_env_override => 3,
            variable_origin::o_command => 4,
            variable_origin::o_override => 5,
            variable_origin::o_automatic => 6,
            variable_origin::o_invalid => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_origin {
        match value {
            0 => variable_origin::o_default,
            1 => variable_origin::o_env,
            2 => variable_origin::o_file,
            3 => variable_origin::o_env_override,
            4 => variable_origin::o_command,
            5 => variable_origin::o_override,
            6 => variable_origin::o_automatic,
            7 => variable_origin::o_invalid,
            _ => panic!("Invalid value for variable_origin: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_origin {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_origin {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_origin {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_origin {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_origin {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_origin {
    type Output = variable_origin;
    fn add(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_origin {
    type Output = variable_origin;
    fn sub(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_origin {
    type Output = variable_origin;
    fn mul(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_origin {
    type Output = variable_origin;
    fn div(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_origin {
    type Output = variable_origin;
    fn rem(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut i8,
    pub value: *mut i8,
    pub fileinfo: floc,
    pub length: u32,
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "libc::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_export {
    v_default = 0,
    v_export,
    v_noexport,
    v_ifset,
}
impl variable_export {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_export::v_default => 0,
            variable_export::v_export => 1,
            variable_export::v_noexport => 2,
            variable_export::v_ifset => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_export {
        match value {
            0 => variable_export::v_default,
            1 => variable_export::v_export,
            2 => variable_export::v_noexport,
            3 => variable_export::v_ifset,
            _ => panic!("Invalid value for variable_export: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_export {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_export {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_export {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_export {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_export {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_export {
    type Output = variable_export;
    fn add(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_export {
    type Output = variable_export;
    fn sub(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_export {
    type Output = variable_export;
    fn mul(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_export {
    type Output = variable_export;
    fn div(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_export {
    type Output = variable_export;
    fn rem(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_flavor {
    f_bogus,
    f_simple,
    f_recursive,
    f_expand,
    f_append,
    f_conditional,
    f_shell,
    f_append_value,
}
impl variable_flavor {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_flavor::f_bogus => 0,
            variable_flavor::f_simple => 1,
            variable_flavor::f_recursive => 2,
            variable_flavor::f_expand => 3,
            variable_flavor::f_append => 4,
            variable_flavor::f_conditional => 5,
            variable_flavor::f_shell => 6,
            variable_flavor::f_append_value => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_flavor {
        match value {
            0 => variable_flavor::f_bogus,
            1 => variable_flavor::f_simple,
            2 => variable_flavor::f_recursive,
            3 => variable_flavor::f_expand,
            4 => variable_flavor::f_append,
            5 => variable_flavor::f_conditional,
            6 => variable_flavor::f_shell,
            7 => variable_flavor::f_append_value,
            _ => panic!("Invalid value for variable_flavor: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_flavor {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_flavor {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_flavor {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_flavor {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_flavor {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_flavor {
    type Output = variable_flavor;
    fn add(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_flavor {
    type Output = variable_flavor;
    fn sub(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_flavor {
    type Output = variable_flavor;
    fn mul(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_flavor {
    type Output = variable_flavor;
    fn div(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_flavor {
    type Output = variable_flavor;
    fn rem(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: i32,
    pub err: i32,
    #[bitfield(name = "syncout", ty = "libc::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct childbase {
    pub cmd_name: *mut i8,
    pub environment: *mut *mut i8,
    pub output: output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct child {
    pub cmd_name: *mut i8,
    pub environment: *mut *mut i8,
    pub output: output,
    pub next: *mut child,
    pub file: *mut file,
    pub sh_batch_file: *mut i8,
    pub command_lines: *mut *mut i8,
    pub command_ptr: *mut i8,
    pub command_line: u32,
    pub pid: pid_t,
    #[bitfield(name = "remote", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "noerror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "good_stdin", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "deleted", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "jobslot", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "6..=6")]
    pub remote_noerror_good_stdin_deleted_recursive_jobslot_dontcare: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: libc::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: i32,
    pub __pad: [i32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: i32,
    pub __used: i32,
    pub __actions: *mut __spawn_action,
    pub __pad: [i32; 16],
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub static mut default_shell: *const i8 = b"/bin/sh\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut batch_mode_shell: i32 = 0 as i32;
unsafe extern "C" fn pid2str(mut pid: pid_t) -> *const i8 {
    static mut pidstring: [i8; 100] = [0; 100];
    sprintf(pidstring.as_mut_ptr(), b"%lu\0" as *const u8 as *const i8, pid as u64);
    return pidstring.as_mut_ptr();
}
#[no_mangle]
pub static mut children: *mut child = 0 as *const child as *mut child;
#[no_mangle]
pub static mut job_slots_used: u32 = 0 as i32 as u32;
static mut good_stdin_used: i32 = 0 as i32;
static mut waiting_jobs: *mut child = 0 as *const child as *mut child;
#[no_mangle]
pub static mut unixy_shell: i32 = 1 as i32;
#[no_mangle]
pub static mut job_counter: u64 = 0 as i32 as u64;
#[no_mangle]
pub static mut jobserver_tokens: u32 = 0 as i32 as u32;
#[no_mangle]
pub unsafe extern "C" fn is_bourne_compatible_shell(mut path: *const i8) -> i32 {
    static mut unix_shells: [*const i8; 8] = [
        b"sh\0" as *const u8 as *const i8,
        b"bash\0" as *const u8 as *const i8,
        b"dash\0" as *const u8 as *const i8,
        b"ksh\0" as *const u8 as *const i8,
        b"rksh\0" as *const u8 as *const i8,
        b"zsh\0" as *const u8 as *const i8,
        b"ash\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut s: *mut *const i8 = 0 as *mut *const i8;
    let mut cp: *const i8 = path.offset(strlen(path) as isize);
    while cp > path
        && !(*stopchar_map
            .as_mut_ptr()
            .offset(*cp.offset(-(1 as i32) as isize) as u8 as isize) as i32
            & 0x8000 as i32 != 0 as i32)
    {
        cp = cp.offset(-1);
        cp;
    }
    s = unix_shells.as_mut_ptr();
    while !(*s).is_null() {
        if strcmp(cp, *s) == 0 as i32 {
            return 1 as i32;
        }
        s = s.offset(1);
        s;
    }
    return 0 as i32;
}
unsafe extern "C" fn block_sigs() {
    sigprocmask(0 as i32, &mut fatal_signal_set, 0 as *mut sigset_t);
}
unsafe extern "C" fn unblock_sigs() {
    sigprocmask(1 as i32, &mut fatal_signal_set, 0 as *mut sigset_t);
}
#[no_mangle]
pub unsafe extern "C" fn unblock_all_sigs() {
    let mut empty: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut empty);
    sigprocmask(2 as i32, &mut empty, 0 as *mut sigset_t);
}
unsafe extern "C" fn child_error(
    mut child: *mut child,
    mut exit_code: i32,
    mut exit_sig: i32,
    mut coredump: i32,
    mut ignored: i32,
) {
    let mut pre: *const i8 = b"*** \0" as *const u8 as *const i8;
    let mut post: *const i8 = b"\0" as *const u8 as *const i8;
    let mut dump: *const i8 = b"\0" as *const u8 as *const i8;
    let mut f: *const file = (*child).file;
    let mut flocp: *const floc = &mut (*(*f).cmds).fileinfo;
    let mut nm: *const i8 = 0 as *const i8;
    let mut smode: *const i8 = 0 as *const i8;
    let mut l: size_t = 0;
    if ignored != 0 && run_silent != 0 {
        return;
    }
    if exit_sig != 0 && coredump != 0 {
        dump = dcgettext(
            0 as *const i8,
            b" (core dumped)\0" as *const u8 as *const i8,
            5 as i32,
        );
    }
    if ignored != 0 {
        pre = b"\0" as *const u8 as *const i8;
        post = dcgettext(
            0 as *const i8,
            b" (ignored)\0" as *const u8 as *const i8,
            5 as i32,
        );
    }
    if ((*flocp).filenm).is_null() {
        nm = dcgettext(
            0 as *const i8,
            b"<builtin>\0" as *const u8 as *const i8,
            5 as i32,
        );
    } else {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (strlen((*flocp).filenm))
                .wrapping_add(6 as i32 as u64)
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                )
                .wrapping_add(1 as i32 as u64) as usize,
        );
        let mut a: *mut i8 = fresh0.as_mut_ptr() as *mut i8;
        sprintf(
            a,
            b"%s:%lu\0" as *const u8 as *const i8,
            (*flocp).filenm,
            ((*flocp).lineno).wrapping_add((*flocp).offset),
        );
        nm = a;
    }
    l = (strlen(pre))
        .wrapping_add(strlen(nm))
        .wrapping_add(strlen((*f).name))
        .wrapping_add(strlen(post));
    smode = shuffle_get_mode();
    if !smode.is_null() {
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::core::mem::size_of::<[i8; 10]>() as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_add(strlen(smode))
                .wrapping_add(1 as i32 as u64) as usize,
        );
        let mut a_0: *mut i8 = fresh1.as_mut_ptr() as *mut i8;
        sprintf(a_0, b" shuffle=%s\0" as *const u8 as *const i8, smode);
        smode = a_0;
        l = (l as u64).wrapping_add(strlen(smode)) as size_t as size_t;
    }
    output_context = if ((*child).output).syncout() as i32 != 0 {
        &mut (*child).output
    } else {
        0 as *mut output
    };
    show_goal_error();
    if exit_sig == 0 as i32 {
        error(
            0 as *mut floc,
            l
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                ),
            dcgettext(
                0 as *const i8,
                b"%s[%s: %s] Error %d%s%s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            pre,
            nm,
            (*f).name,
            exit_code,
            post,
            if !smode.is_null() { smode } else { b"\0" as *const u8 as *const i8 },
        );
    } else {
        let mut s: *const i8 = strsignal(exit_sig);
        error(
            0 as *mut floc,
            l.wrapping_add(strlen(s)).wrapping_add(strlen(dump)),
            b"%s[%s: %s] %s%s%s%s\0" as *const u8 as *const i8,
            pre,
            nm,
            (*f).name,
            s,
            dump,
            post,
            if !smode.is_null() { smode } else { b"\0" as *const u8 as *const i8 },
        );
    }
    output_context = 0 as *mut output;
}
static mut dead_children: u32 = 0 as i32 as u32;
#[no_mangle]
pub unsafe extern "C" fn child_handler(mut sig: i32) {
    dead_children = dead_children.wrapping_add(1);
    dead_children;
    jobserver_signal();
}
#[no_mangle]
pub unsafe extern "C" fn reap_children(mut block: i32, mut err: i32) {
    let mut status: i32 = 0;
    let mut reap_more: i32 = 1 as i32;
    let mut current_block_143: u64;
    while (!children.is_null() || shell_function_pid != 0 as i32)
        && (block != 0 || reap_more != 0)
    {
        let mut remote: u32 = 0 as i32 as u32;
        let mut pid: pid_t = 0;
        let mut exit_code: i32 = 0;
        let mut exit_sig: i32 = 0;
        let mut coredump: i32 = 0;
        let mut lastc: *mut child = 0 as *mut child;
        let mut c: *mut child = 0 as *mut child;
        let mut child_failed: i32 = 0;
        let mut any_remote: i32 = 0;
        let mut any_local: i32 = 0;
        let mut dontcare: i32 = 0;
        if err != 0 && block != 0 {
            static mut printed: i32 = 0 as i32;
            fflush(stdout);
            if printed == 0 {
                error(
                    0 as *mut floc,
                    0 as i32 as size_t,
                    dcgettext(
                        0 as *const i8,
                        b"*** Waiting for unfinished jobs....\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            printed = 1 as i32;
        }
        if dead_children > 0 as i32 as u32 {
            dead_children = dead_children.wrapping_sub(1);
            dead_children;
        }
        any_remote = 0 as i32;
        any_local = (shell_function_pid != 0 as i32) as i32;
        lastc = 0 as *mut child;
        c = children;
        loop {
            if c.is_null() {
                current_block_143 = 17478428563724192186;
                break;
            }
            any_remote |= (*c).remote() as i32;
            any_local |= ((*c).remote() == 0) as i32;
            if (*c).pid < 0 as i32 {
                exit_sig = 0 as i32;
                coredump = 0 as i32;
                exit_code = 127 as i32;
                current_block_143 = 14031446765654731216;
                break;
            } else {
                if 0x4 as i32 & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"Live child %p (%s) PID %s %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        c,
                        (*(*c).file).name,
                        pid2str((*c).pid),
                        if (*c).remote() as i32 != 0 {
                            dcgettext(
                                0 as *const i8,
                                b" (remote)\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        } else {
                            b"\0" as *const u8 as *const i8
                        },
                    );
                    fflush(stdout);
                }
                lastc = c;
                c = (*c).next;
            }
        }
        match current_block_143 {
            17478428563724192186 => {
                if any_remote != 0 {
                    pid = remote_status(
                        &mut exit_code,
                        &mut exit_sig,
                        &mut coredump,
                        0 as i32,
                    );
                } else {
                    pid = 0 as i32;
                }
                if pid > 0 as i32 {
                    remote = 1 as i32 as u32;
                } else if pid < 0 as i32 {
                    pfatal_with_name(b"remote_status\0" as *const u8 as *const i8);
                } else {
                    if any_local != 0 {
                        if block == 0 {
                            pid = waitpid(-(1 as i32), &mut status, 1 as i32);
                        } else {
                            loop {
                                pid = wait(&mut status);
                                if !(pid == -(1 as i32) && *__errno_location() == 4 as i32)
                                {
                                    break;
                                }
                            }
                        }
                    } else {
                        pid = 0 as i32;
                    }
                    if pid < 0 as i32 {
                        pfatal_with_name(b"wait\0" as *const u8 as *const i8);
                    } else if pid > 0 as i32 {
                        exit_code = (status & 0xff00 as i32) >> 8 as i32;
                        exit_sig = if ((status & 0x7f as i32) + 1 as i32)
                            as libc::c_schar as i32 >> 1 as i32 > 0 as i32
                        {
                            status & 0x7f as i32
                        } else {
                            0 as i32
                        };
                        coredump = status & 0x80 as i32;
                    } else {
                        reap_more = 0 as i32;
                        if block == 0 || any_remote == 0 {
                            break;
                        }
                        pid = remote_status(
                            &mut exit_code,
                            &mut exit_sig,
                            &mut coredump,
                            1 as i32,
                        );
                        if pid < 0 as i32 {
                            pfatal_with_name(
                                b"remote_status\0" as *const u8 as *const i8,
                            );
                        }
                        if pid == 0 as i32 {
                            break;
                        }
                        remote = 1 as i32 as u32;
                    }
                }
                command_count = command_count.wrapping_add(1);
                command_count;
                if remote == 0 && pid == shell_function_pid {
                    shell_completed(exit_code, exit_sig);
                    break;
                } else {
                    lastc = 0 as *mut child;
                    c = children;
                    while !c.is_null() {
                        if (*c).pid == pid && (*c).remote() == remote {
                            break;
                        }
                        lastc = c;
                        c = (*c).next;
                    }
                    if c.is_null() {
                        continue;
                    } else {
                        if 0x4 as i32 & db_level != 0 {
                            printf(
                                if exit_sig == 0 as i32 && exit_code == 0 as i32 {
                                    dcgettext(
                                        0 as *const i8,
                                        b"Reaping winning child %p PID %s %s\n\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    )
                                } else {
                                    dcgettext(
                                        0 as *const i8,
                                        b"Reaping losing child %p PID %s %s\n\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    )
                                },
                                c,
                                pid2str((*c).pid),
                                if (*c).remote() as i32 != 0 {
                                    dcgettext(
                                        0 as *const i8,
                                        b" (remote)\0" as *const u8 as *const i8,
                                        5 as i32,
                                    )
                                } else {
                                    b"\0" as *const u8 as *const i8
                                },
                            );
                            fflush(stdout);
                        }
                        if job_counter != 0 {
                            job_counter = job_counter.wrapping_sub(1);
                            job_counter;
                        }
                    }
                }
            }
            _ => {}
        }
        if exit_sig == 0 as i32 && exit_code == 127 as i32 && !((*c).cmd_name).is_null()
        {
            let mut e: *const i8 = 0 as *const i8;
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
            let mut r: i32 = 0;
            loop {
                r = stat((*c).cmd_name, &mut st);
                if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if r < 0 as i32 {
                e = strerror(*__errno_location());
            } else if st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
                || st.st_mode & 0o100 as i32 as u32 == 0
            {
                e = strerror(13 as i32);
            } else if st.st_size == 0 as i32 as i64 {
                e = strerror(8 as i32);
            }
            if !e.is_null() {
                error(
                    0 as *mut floc,
                    (strlen((*c).cmd_name)).wrapping_add(strlen(e)),
                    b"%s: %s\0" as *const u8 as *const i8,
                    (*c).cmd_name,
                    e,
                );
            }
        }
        if exit_sig == 0 as i32 && exit_code == 0 as i32 {
            child_failed = 0 as i32;
        } else if exit_sig == 0 as i32 && exit_code == 1 as i32 && question_flag != 0
            && (*c).recursive() as i32 != 0
        {
            child_failed = 1 as i32;
        } else {
            child_failed = 2 as i32;
        }
        if !((*c).sh_batch_file).is_null() {
            let mut rm_status: i32 = 0;
            if 0x4 as i32 & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Cleaning up temp batch file %s\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*c).sh_batch_file,
                );
                fflush(stdout);
            }
            *__errno_location() = 0 as i32;
            rm_status = remove((*c).sh_batch_file);
            if rm_status != 0 {
                if 0x4 as i32 & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"Cleaning up temp batch file %s failed (%d)\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*c).sh_batch_file,
                        *__errno_location(),
                    );
                    fflush(stdout);
                }
            }
            free((*c).sh_batch_file as *mut libc::c_void);
            (*c).sh_batch_file = 0 as *mut i8;
        }
        if (*c).good_stdin() != 0 {
            good_stdin_used = 0 as i32;
        }
        dontcare = (*c).dontcare() as i32;
        if child_failed != 0 && (*c).noerror() == 0 && ignore_errors_flag == 0 {
            static mut delete_on_error: i32 = -(1 as i32);
            if dontcare == 0 && child_failed == 2 as i32 {
                child_error(c, exit_code, exit_sig, coredump, 0 as i32);
            }
            (*(*c).file)
                .set_update_status(
                    update_status::from_libc_c_uint(
                        (if child_failed == 2 as i32 {
                            update_status::us_failed as i32
                        } else {
                            update_status::us_question as i32
                        }) as u32,
                    ),
                );
            if delete_on_error == -(1 as i32) {
                let mut f: *mut file = lookup_file(
                    b".DELETE_ON_ERROR\0" as *const u8 as *const i8,
                );
                delete_on_error = (!f.is_null() && (*f).is_target() as i32 != 0) as i32;
            }
            if exit_sig != 0 as i32 || delete_on_error != 0 {
                delete_child_targets(c);
            }
        } else {
            if child_failed != 0 {
                child_error(c, exit_code, exit_sig, coredump, 1 as i32);
                child_failed = 0 as i32;
            }
            if job_next_command(c) != 0 {
                if handling_fatal_signal != 0 {
                    (*(*c).file).set_update_status(update_status::us_failed);
                } else {
                    if output_sync == 1 as i32 {
                        output_dump(&mut (*c).output);
                    }
                    (*c).set_remote(start_remote_job_p(0 as i32) as u32);
                    start_job_command(c);
                    unblock_sigs();
                    if (*(*c).file).command_state() as i32
                        == cmd_state::cs_running as i32
                    {
                        continue;
                    }
                }
                if (*(*c).file).update_status() as i32
                    != update_status::us_success as i32
                {
                    delete_child_targets(c);
                }
            } else {
                (*(*c).file).set_update_status(update_status::us_success);
            }
        }
        output_dump(&mut (*c).output);
        if handling_fatal_signal == 0 {
            notice_finished_file((*c).file);
        }
        block_sigs();
        if (*c).pid > 0 as i32 {
            if 0x4 as i32 & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Removing child %p PID %s%s from chain.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    c,
                    pid2str((*c).pid),
                    if (*c).remote() as i32 != 0 {
                        dcgettext(
                            0 as *const i8,
                            b" (remote)\0" as *const u8 as *const i8,
                            5 as i32,
                        )
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                );
                fflush(stdout);
            }
        }
        if job_slots_used > 0 as i32 as u32 {
            job_slots_used = job_slots_used.wrapping_sub((*c).jobslot());
        }
        if lastc.is_null() {
            children = (*c).next;
        } else {
            (*lastc).next = (*c).next;
        }
        free_child(c);
        unblock_sigs();
        if err == 0 && child_failed != 0 && dontcare == 0 && keep_going_flag == 0
            && handling_fatal_signal == 0
        {
            die(child_failed);
        }
        block = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_childbase(mut child: *mut childbase) {
    if !((*child).environment).is_null() {
        let mut ep: *mut *mut i8 = (*child).environment;
        while !(*ep).is_null() {
            let fresh2 = ep;
            ep = ep.offset(1);
            free(*fresh2 as *mut libc::c_void);
        }
        free((*child).environment as *mut libc::c_void);
    }
    free((*child).cmd_name as *mut libc::c_void);
}
unsafe extern "C" fn free_child(mut child: *mut child) {
    output_close(&mut (*child).output);
    if jobserver_tokens == 0 {
        fatal(
            0 as *mut floc,
            (53 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_div(22 as i32 as u64)
                .wrapping_add(3 as i32 as u64)
                .wrapping_add(strlen((*(*child).file).name)),
            b"INTERNAL: Freeing child %p (%s) but no tokens left\0" as *const u8
                as *const i8,
            child,
            (*(*child).file).name,
        );
    }
    if jobserver_enabled() != 0 && jobserver_tokens > 1 as i32 as u32 {
        jobserver_release(1 as i32);
        if 0x4 as i32 & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Released token for child %p (%s).\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                child,
                (*(*child).file).name,
            );
            fflush(stdout);
        }
    }
    jobserver_tokens = jobserver_tokens.wrapping_sub(1);
    jobserver_tokens;
    if handling_fatal_signal != 0 {
        return;
    }
    if !((*child).command_lines).is_null() {
        let mut i: u32 = 0;
        i = 0 as i32 as u32;
        while i < (*(*(*child).file).cmds).ncommand_lines as u32 {
            free(*((*child).command_lines).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*child).command_lines as *mut libc::c_void);
    }
    free_childbase(child as *mut childbase);
    free(child as *mut libc::c_void);
}
unsafe extern "C" fn start_job_command(mut child: *mut child) {
    let mut flags: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
    if !((*child).command_ptr).is_null() {
        flags = (*(*child).file).command_flags
            | *((*(*(*child).file).cmds).lines_flags)
                .offset(((*child).command_line).wrapping_sub(1 as i32 as u32) as isize)
                as i32;
        p = (*child).command_ptr;
        (*child).set_noerror((flags & 4 as i32 != 0 as i32) as i32 as u32);
        while *p as i32 != '\0' as i32 {
            if *p as i32 == '@' as i32 {
                flags |= 2 as i32;
            } else if *p as i32 == '+' as i32 {
                flags |= 1 as i32;
            } else if *p as i32 == '-' as i32 {
                (*child).set_noerror(1 as i32 as u32);
            } else if !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
                & 0x2 as i32 != 0 as i32)
            {
                break;
            }
            p = p.offset(1);
            p;
        }
        (*child).set_recursive((flags & 1 as i32 != 0 as i32) as i32 as u32);
        let ref mut fresh3 = *((*(*(*child).file).cmds).lines_flags)
            .offset(((*child).command_line).wrapping_sub(1 as i32 as u32) as isize);
        *fresh3 = (*fresh3 as i32 | flags & 1 as i32) as u8;
        let mut prefix: i8 = (*(*(*child).file).cmds).recipe_prefix;
        let mut p1: *mut i8 = 0 as *mut i8;
        let mut p2: *mut i8 = 0 as *mut i8;
        p2 = p;
        p1 = p2;
        while *p1 as i32 != '\0' as i32 {
            let fresh4 = p2;
            p2 = p2.offset(1);
            *fresh4 = *p1;
            if *p1.offset(0 as i32 as isize) as i32 == '\n' as i32
                && *p1.offset(1 as i32 as isize) as i32 == prefix as i32
            {
                p1 = p1.offset(1);
                p1;
            }
            p1 = p1.offset(1);
            p1;
        }
        *p2 = *p1;
        let mut end: *mut i8 = 0 as *mut i8;
        argv = construct_command_argv(
            p,
            &mut end,
            (*child).file,
            *((*(*(*child).file).cmds).lines_flags)
                .offset(((*child).command_line).wrapping_sub(1 as i32 as u32) as isize)
                as i32,
            &mut (*child).sh_batch_file,
        );
        if end.is_null() {
            (*child).command_ptr = 0 as *mut i8;
        } else {
            let fresh5 = end;
            end = end.offset(1);
            *fresh5 = '\0' as i32 as i8;
            (*child).command_ptr = end;
        }
        if !argv.is_null() && question_flag != 0 && !(flags & 1 as i32 != 0 as i32) {
            if !argv.is_null() {
                free(*argv.offset(0 as i32 as isize) as *mut libc::c_void);
                free(argv as *mut libc::c_void);
            }
            (*(*child).file).set_update_status(update_status::us_question);
            notice_finished_file((*child).file);
            return;
        }
        if touch_flag != 0 && !(flags & 1 as i32 != 0 as i32) {
            if !argv.is_null() {
                free(*argv.offset(0 as i32 as isize) as *mut libc::c_void);
                free(argv as *mut libc::c_void);
            }
            argv = 0 as *mut *mut i8;
        }
        if !argv.is_null() {
            ((*child).output)
                .set_syncout(
                    (output_sync != 0
                        && (output_sync == 3 as i32 || !(flags & 1 as i32 != 0 as i32)))
                        as i32 as u32,
                );
            output_context = if ((*child).output).syncout() as i32 != 0 {
                &mut (*child).output
            } else {
                0 as *mut output
            };
            if ((*child).output).syncout() == 0 {
                output_dump(&mut (*child).output);
            }
            if just_print_flag != 0 || 0x10 as i32 & db_level != 0
                || !(flags & 2 as i32 != 0 as i32) && run_silent == 0
            {
                message(0 as i32, strlen(p), b"%s\0" as *const u8 as *const i8, p);
            }
            commands_started = commands_started.wrapping_add(1);
            commands_started;
            if !(*argv.offset(0 as i32 as isize)).is_null()
                && is_bourne_compatible_shell(*argv.offset(0 as i32 as isize)) != 0
                && (!(*argv.offset(1 as i32 as isize)).is_null()
                    && *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize)
                        as i32 == '-' as i32
                    && (*(*argv.offset(1 as i32 as isize)).offset(1 as i32 as isize)
                        as i32 == 'c' as i32
                        && *(*argv.offset(1 as i32 as isize)).offset(2 as i32 as isize)
                            as i32 == '\0' as i32
                        || *(*argv.offset(1 as i32 as isize)).offset(1 as i32 as isize)
                            as i32 == 'e' as i32
                            && *(*argv.offset(1 as i32 as isize))
                                .offset(2 as i32 as isize) as i32 == 'c' as i32
                            && *(*argv.offset(1 as i32 as isize))
                                .offset(3 as i32 as isize) as i32 == '\0' as i32))
                && (!(*argv.offset(2 as i32 as isize)).is_null()
                    && *(*argv.offset(2 as i32 as isize)).offset(0 as i32 as isize)
                        as i32 == ':' as i32
                    && *(*argv.offset(2 as i32 as isize)).offset(1 as i32 as isize)
                        as i32 == '\0' as i32)
                && (*argv.offset(3 as i32 as isize)).is_null()
            {
                if !argv.is_null() {
                    free(*argv.offset(0 as i32 as isize) as *mut libc::c_void);
                    free(argv as *mut libc::c_void);
                }
            } else if just_print_flag != 0 && !(flags & 1 as i32 != 0 as i32) {
                if !argv.is_null() {
                    free(*argv.offset(0 as i32 as isize) as *mut libc::c_void);
                    free(argv as *mut libc::c_void);
                }
            } else {
                output_start();
                fflush(stdout);
                fflush(stderr);
                (*child).set_good_stdin((good_stdin_used == 0) as i32 as u32);
                if (*child).good_stdin() != 0 {
                    good_stdin_used = 1 as i32;
                }
                (*child).set_deleted(0 as i32 as u32);
                if ((*child).environment).is_null() {
                    (*child).environment = target_environment(
                        (*child).file,
                        (*(*(*child).file).cmds).any_recurse() as i32,
                    );
                }
                let mut current_block_97: u64;
                if (*child).remote() != 0 {
                    let mut is_remote: i32 = 0;
                    let mut used_stdin: i32 = 0;
                    let mut id: pid_t = 0;
                    if start_remote_job(
                        argv,
                        (*child).environment,
                        if (*child).good_stdin() as i32 != 0 {
                            0 as i32
                        } else {
                            get_bad_stdin()
                        },
                        &mut is_remote,
                        &mut id,
                        &mut used_stdin,
                    ) != 0
                    {
                        current_block_97 = 9425266452497093757;
                    } else {
                        if (*child).good_stdin() as i32 != 0 && used_stdin == 0 {
                            (*child).set_good_stdin(0 as i32 as u32);
                            good_stdin_used = 0 as i32;
                        }
                        (*child).set_remote(is_remote as u32);
                        (*child).pid = id;
                        current_block_97 = 10261677128829721533;
                    }
                } else {
                    current_block_97 = 9425266452497093757;
                }
                match current_block_97 {
                    9425266452497093757 => {
                        block_sigs();
                        (*child).set_remote(0 as i32 as u32);
                        jobserver_pre_child((flags & 1 as i32 != 0 as i32) as i32);
                        (*child).pid = child_execute_job(
                            child as *mut childbase,
                            (*child).good_stdin() as i32,
                            argv,
                        );
                        jobserver_post_child((flags & 1 as i32 != 0 as i32) as i32);
                    }
                    _ => {}
                }
                if (*child).pid >= 0 as i32 {
                    job_counter = job_counter.wrapping_add(1);
                    job_counter;
                }
                set_command_state((*child).file, cmd_state::cs_running);
                if !argv.is_null() {
                    free(*argv.offset(0 as i32 as isize) as *mut libc::c_void);
                    free(argv as *mut libc::c_void);
                }
                output_context = 0 as *mut output;
                return;
            }
        }
    }
    if job_next_command(child) != 0 {
        start_job_command(child);
    } else {
        set_command_state((*child).file, cmd_state::cs_running);
        (*(*child).file).set_update_status(update_status::us_success);
        notice_finished_file((*child).file);
    }
    output_context = 0 as *mut output;
}
unsafe extern "C" fn start_waiting_job(mut c: *mut child) -> i32 {
    let mut f: *mut file = (*c).file;
    (*c).set_remote(start_remote_job_p(1 as i32) as u32);
    if (*c).remote() == 0 && (job_slots_used > 0 as i32 as u32 && load_too_high() != 0) {
        set_command_state(f, cmd_state::cs_running);
        (*c).next = waiting_jobs;
        waiting_jobs = c;
        return 0 as i32;
    }
    start_job_command(c);
    let mut current_block_25: u64;
    match (*f).command_state() as i32 {
        2 => {
            (*c).next = children;
            if (*c).pid > 0 as i32 {
                if 0x4 as i32 & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"Putting child %p (%s) PID %s%s on the chain.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        c,
                        (*(*c).file).name,
                        pid2str((*c).pid),
                        if (*c).remote() as i32 != 0 {
                            dcgettext(
                                0 as *const i8,
                                b" (remote)\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        } else {
                            b"\0" as *const u8 as *const i8
                        },
                    );
                    fflush(stdout);
                }
                job_slots_used = job_slots_used.wrapping_add(1);
                job_slots_used;
                (*c).set_jobslot(1 as i32 as u32);
            }
            children = c;
            unblock_sigs();
            current_block_25 = 15089075282327824602;
        }
        0 => {
            (*f).set_update_status(update_status::us_success);
            current_block_25 = 5112011579493293328;
        }
        3 => {
            current_block_25 = 5112011579493293328;
        }
        _ => {
            current_block_25 = 15089075282327824602;
        }
    }
    match current_block_25 {
        5112011579493293328 => {
            notice_finished_file(f);
            free_child(c);
        }
        _ => {}
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn new_job(mut file: *mut file) {
    let mut cmds: *mut commands = (*file).cmds;
    let mut c: *mut child = 0 as *mut child;
    let mut lines: *mut *mut i8 = 0 as *mut *mut i8;
    let mut i: u32 = 0;
    start_waiting_jobs();
    reap_children(0 as i32, 0 as i32);
    chop_commands(cmds);
    c = xcalloc(::core::mem::size_of::<child>() as u64) as *mut child;
    output_init(&mut (*c).output);
    (*c).file = file;
    (*c).sh_batch_file = 0 as *mut i8;
    (*c).set_dontcare((*file).dontcare());
    output_context = if ((*c).output).syncout() as i32 != 0 {
        &mut (*c).output
    } else {
        0 as *mut output
    };
    lines = xmalloc(
        ((*cmds).ncommand_lines as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    i = 0 as i32 as u32;
    while i < (*cmds).ncommand_lines as u32 {
        let mut in_0: *mut i8 = 0 as *mut i8;
        let mut out: *mut i8 = 0 as *mut i8;
        let mut ref_0: *mut i8 = 0 as *mut i8;
        out = *((*cmds).command_lines).offset(i as isize);
        in_0 = out;
        loop {
            ref_0 = strchr(in_0, '$' as i32);
            if ref_0.is_null() {
                break;
            }
            ref_0 = ref_0.offset(1);
            ref_0;
            if out != in_0 {
                memmove(
                    out as *mut libc::c_void,
                    in_0 as *const libc::c_void,
                    ref_0.offset_from(in_0) as i64 as u64,
                );
            }
            out = out.offset(ref_0.offset_from(in_0) as i64 as isize);
            in_0 = ref_0;
            if *ref_0 as i32 == '(' as i32 || *ref_0 as i32 == '{' as i32 {
                let mut openparen: i8 = *ref_0;
                let mut closeparen: i8 = (if openparen as i32 == '(' as i32 {
                    ')' as i32
                } else {
                    '}' as i32
                }) as i8;
                let mut outref: *mut i8 = 0 as *mut i8;
                let mut count: i32 = 0;
                let mut p: *mut i8 = 0 as *mut i8;
                let fresh6 = in_0;
                in_0 = in_0.offset(1);
                let fresh7 = out;
                out = out.offset(1);
                *fresh7 = *fresh6;
                outref = out;
                count = 0 as i32;
                while *in_0 as i32 != '\0' as i32 {
                    if *in_0 as i32 == closeparen as i32
                        && {
                            count -= 1;
                            count < 0 as i32
                        }
                    {
                        break;
                    }
                    if *in_0 as i32 == '\\' as i32
                        && *in_0.offset(1 as i32 as isize) as i32 == '\n' as i32
                    {
                        let mut quoted: i32 = 0 as i32;
                        p = in_0.offset(-(1 as i32 as isize));
                        while p > ref_0 && *p as i32 == '\\' as i32 {
                            quoted = (quoted == 0) as i32;
                            p = p.offset(-1);
                            p;
                        }
                        if quoted != 0 {
                            let fresh8 = in_0;
                            in_0 = in_0.offset(1);
                            let fresh9 = out;
                            out = out.offset(1);
                            *fresh9 = *fresh8;
                        } else {
                            in_0 = in_0.offset(2 as i32 as isize);
                            while *stopchar_map.as_mut_ptr().offset(*in_0 as u8 as isize)
                                as i32 & (0x2 as i32 | 0x4 as i32) != 0 as i32
                            {
                                in_0 = in_0.offset(1);
                                in_0;
                            }
                            while out > outref
                                && *stopchar_map
                                    .as_mut_ptr()
                                    .offset(*out.offset(-(1 as i32) as isize) as u8 as isize)
                                    as i32 & 0x2 as i32 != 0 as i32
                            {
                                out = out.offset(-1);
                                out;
                            }
                            let fresh10 = out;
                            out = out.offset(1);
                            *fresh10 = ' ' as i32 as i8;
                        }
                    } else {
                        if *in_0 as i32 == openparen as i32 {
                            count += 1;
                            count;
                        }
                        let fresh11 = in_0;
                        in_0 = in_0.offset(1);
                        let fresh12 = out;
                        out = out.offset(1);
                        *fresh12 = *fresh11;
                    }
                }
            }
        }
        if out != in_0 {
            memmove(
                out as *mut libc::c_void,
                in_0 as *const libc::c_void,
                (strlen(in_0)).wrapping_add(1 as i32 as u64),
            );
        }
        (*cmds).fileinfo.offset = i as u64;
        let ref mut fresh13 = *lines.offset(i as isize);
        *fresh13 = allocated_variable_expand_for_file(
            *((*cmds).command_lines).offset(i as isize),
            file,
        );
        i = i.wrapping_add(1);
        i;
    }
    (*cmds).fileinfo.offset = 0 as i32 as u64;
    (*c).command_lines = lines;
    job_next_command(c);
    if job_slots != 0 as i32 as u32 {
        while job_slots_used == job_slots {
            reap_children(1 as i32, 0 as i32);
        }
    } else if jobserver_enabled() != 0 {
        loop {
            let mut got_token: i32 = 0;
            if 0x4 as i32 & db_level != 0 {
                printf(
                    b"Need a job token; we %shave children\n\0" as *const u8
                        as *const i8,
                    if !children.is_null() {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"don't \0" as *const u8 as *const i8
                    },
                );
                fflush(stdout);
            }
            if jobserver_tokens == 0 {
                break;
            }
            jobserver_pre_acquire();
            reap_children(0 as i32, 0 as i32);
            start_waiting_jobs();
            if jobserver_tokens == 0 {
                break;
            }
            if children.is_null() {
                fatal(
                    0 as *mut floc,
                    0 as i32 as size_t,
                    b"INTERNAL: no children as we go to sleep on read\0" as *const u8
                        as *const i8,
                );
            }
            got_token = jobserver_acquire(
                (waiting_jobs != 0 as *mut libc::c_void as *mut child) as i32,
            ) as i32;
            if !(got_token == 1 as i32) {
                continue;
            }
            if 0x4 as i32 & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Obtained token for child %p (%s).\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    c,
                    (*(*c).file).name,
                );
                fflush(stdout);
            }
            break;
        }
    }
    jobserver_tokens = jobserver_tokens.wrapping_add(1);
    jobserver_tokens;
    if 0x20 as i32 & db_level != 0 {
        let mut nm: *const i8 = 0 as *const i8;
        if ((*cmds).fileinfo.filenm).is_null() {
            nm = dcgettext(
                0 as *const i8,
                b"<builtin>\0" as *const u8 as *const i8,
                5 as i32,
            );
        } else {
            let mut fresh14 = ::std::vec::from_elem(
                0,
                (strlen((*cmds).fileinfo.filenm))
                    .wrapping_add(1 as i32 as u64)
                    .wrapping_add(11 as i32 as u64)
                    .wrapping_add(1 as i32 as u64) as usize,
            );
            let mut n: *mut i8 = fresh14.as_mut_ptr() as *mut i8;
            sprintf(
                n,
                b"%s:%lu\0" as *const u8 as *const i8,
                (*cmds).fileinfo.filenm,
                (*cmds).fileinfo.lineno,
            );
            nm = n;
        }
        if (*(*c).file).phony() != 0 {
            message(
                0 as i32,
                (strlen(nm)).wrapping_add(strlen((*(*c).file).name)),
                dcgettext(
                    0 as *const i8,
                    b"%s: update target '%s' due to: target is .PHONY\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                nm,
                (*(*c).file).name,
            );
        } else if (*(*c).file).last_mtime == 1 as i32 as u64 {
            message(
                0 as i32,
                (strlen(nm)).wrapping_add(strlen((*(*c).file).name)),
                dcgettext(
                    0 as *const i8,
                    b"%s: update target '%s' due to: target does not exist\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                nm,
                (*(*c).file).name,
            );
        } else {
            let mut newer: *mut i8 = allocated_variable_expand_for_file(
                b"$?\0" as *const u8 as *const i8,
                (*c).file,
            );
            if *newer.offset(0 as i32 as isize) as i32 != '\0' as i32 {
                message(
                    0 as i32,
                    (strlen(nm))
                        .wrapping_add(strlen((*(*c).file).name))
                        .wrapping_add(strlen(newer)),
                    dcgettext(
                        0 as *const i8,
                        b"%s: update target '%s' due to: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    nm,
                    (*(*c).file).name,
                    newer,
                );
                free(newer as *mut libc::c_void);
            } else {
                let mut len: size_t = 0 as i32 as size_t;
                let mut d: *mut dep = 0 as *mut dep;
                d = (*(*c).file).deps;
                while !d.is_null() {
                    if (*(*d).file).last_mtime == 1 as i32 as u64 {
                        len = (len as u64)
                            .wrapping_add(
                                (strlen((*(*d).file).name)).wrapping_add(1 as i32 as u64),
                            ) as size_t as size_t;
                    }
                    d = (*d).next;
                }
                if len == 0 {
                    message(
                        0 as i32,
                        (strlen(nm)).wrapping_add(strlen((*(*c).file).name)),
                        dcgettext(
                            0 as *const i8,
                            b"%s: update target '%s' due to: unknown reasons\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        nm,
                        (*(*c).file).name,
                    );
                } else {
                    let mut fresh15 = ::std::vec::from_elem(0, len as usize);
                    newer = fresh15.as_mut_ptr() as *mut i8;
                    let mut cp: *mut i8 = newer;
                    d = (*(*c).file).deps;
                    while !d.is_null() {
                        if (*(*d).file).last_mtime == 1 as i32 as u64 {
                            if cp > newer {
                                let fresh16 = cp;
                                cp = cp.offset(1);
                                *fresh16 = ' ' as i32 as i8;
                            }
                            cp = stpcpy(cp, (*(*d).file).name);
                        }
                        d = (*d).next;
                    }
                    message(
                        0 as i32,
                        (strlen(nm))
                            .wrapping_add(strlen((*(*c).file).name))
                            .wrapping_add(strlen(newer)),
                        dcgettext(
                            0 as *const i8,
                            b"%s: update target '%s' due to: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        nm,
                        (*(*c).file).name,
                        newer,
                    );
                }
            }
        }
    }
    start_waiting_job(c);
    if job_slots == 1 as i32 as u32 || not_parallel != 0 {
        while (*file).command_state() as i32 == cmd_state::cs_running as i32 {
            reap_children(1 as i32, 0 as i32);
        }
    }
    output_context = 0 as *mut output;
}
unsafe extern "C" fn job_next_command(mut child: *mut child) -> i32 {
    while ((*child).command_ptr).is_null() || *(*child).command_ptr as i32 == '\0' as i32
    {
        if (*child).command_line == (*(*(*child).file).cmds).ncommand_lines as u32 {
            (*child).command_ptr = 0 as *mut i8;
            (*(*(*child).file).cmds).fileinfo.offset = 0 as i32 as u64;
            return 0 as i32;
        } else {
            let fresh17 = (*child).command_line;
            (*child).command_line = ((*child).command_line).wrapping_add(1);
            (*child).command_ptr = *((*child).command_lines).offset(fresh17 as isize);
        }
    }
    (*(*(*child).file).cmds).fileinfo.offset = ((*child).command_line)
        .wrapping_sub(1 as i32 as u32) as u64;
    return 1 as i32;
}
unsafe extern "C" fn load_too_high() -> i32 {
    static mut last_sec: libc::c_double = 0.;
    static mut last_now: time_t = 0;
    static mut proc_fd: i32 = -(2 as i32);
    let mut load: libc::c_double = 0.;
    let mut guess: libc::c_double = 0.;
    let mut now: time_t = 0;
    if max_load_average < 0 as i32 as libc::c_double {
        return 0 as i32;
    }
    if proc_fd == -(2 as i32) {
        loop {
            proc_fd = open(b"/proc/loadavg\0" as *const u8 as *const i8, 0 as i32);
            if !(proc_fd == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if proc_fd < 0 as i32 {
            if 0x4 as i32 & db_level != 0 {
                printf(
                    b"Using system load detection method.\n\0" as *const u8 as *const i8,
                );
                fflush(stdout);
            }
        } else {
            if 0x4 as i32 & db_level != 0 {
                printf(
                    b"Using /proc/loadavg load detection method.\n\0" as *const u8
                        as *const i8,
                );
                fflush(stdout);
            }
            fd_noinherit(proc_fd);
        }
    }
    if proc_fd >= 0 as i32 {
        let mut r: i32 = 0;
        loop {
            r = lseek(proc_fd, 0 as i32 as __off_t, 0 as i32) as i32;
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r >= 0 as i32 {
            let mut avg: [i8; 65] = [0; 65];
            loop {
                r = read(
                    proc_fd,
                    avg.as_mut_ptr() as *mut libc::c_void,
                    64 as i32 as size_t,
                ) as i32;
                if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if r >= 0 as i32 {
                let mut p: *const i8 = 0 as *const i8;
                avg[r as usize] = '\0' as i32 as i8;
                p = strchr(avg.as_mut_ptr(), ' ' as i32);
                if !p.is_null() {
                    p = strchr(p.offset(1 as i32 as isize), ' ' as i32);
                }
                if !p.is_null() {
                    p = strchr(p.offset(1 as i32 as isize), ' ' as i32);
                }
                if !p.is_null()
                    && (*p.offset(1 as i32 as isize) as u32)
                        .wrapping_sub('0' as i32 as u32) <= 9 as i32 as u32
                {
                    let mut cnt: u32 = make_toui(
                        p.offset(1 as i32 as isize),
                        0 as *mut *const i8,
                    );
                    if 0x4 as i32 & db_level != 0 {
                        printf(
                            b"Running: system = %u / make = %u (max requested = %f)\n\0"
                                as *const u8 as *const i8,
                            cnt,
                            job_slots_used,
                            max_load_average,
                        );
                        fflush(stdout);
                    }
                    return (cnt as libc::c_double > max_load_average) as i32;
                }
                if 0x4 as i32 & db_level != 0 {
                    printf(
                        b"Failed to parse /proc/loadavg: %s\n\0" as *const u8
                            as *const i8,
                        avg.as_mut_ptr(),
                    );
                    fflush(stdout);
                }
            }
        }
        if r < 0 as i32 {
            if 0x4 as i32 & db_level != 0 {
                printf(
                    b"Failed to read /proc/loadavg: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
                fflush(stdout);
            }
        }
        close(proc_fd);
        proc_fd = -(1 as i32);
    }
    *__errno_location() = 0 as i32;
    if getloadavg(&mut load, 1 as i32) != 1 as i32 {
        static mut lossage: i32 = -(1 as i32);
        if lossage == -(1 as i32) || *__errno_location() != lossage {
            if *__errno_location() == 0 as i32 {
                error(
                    0 as *mut floc,
                    0 as i32 as size_t,
                    dcgettext(
                        0 as *const i8,
                        b"cannot enforce load limits on this operating system\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            } else {
                perror_with_name(
                    dcgettext(
                        0 as *const i8,
                        b"cannot enforce load limit: \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"getloadavg\0" as *const u8 as *const i8,
                );
            }
        }
        lossage = *__errno_location();
        load = 0 as i32 as libc::c_double;
    }
    now = time(0 as *mut time_t);
    if last_now < now {
        if last_now == now - 1 as i32 as i64 {
            last_sec = 0.25f64 * job_counter as libc::c_double;
        } else {
            last_sec = 0.0f64;
        }
        job_counter = 0 as i32 as u64;
        last_now = now;
    }
    guess = load + 0.25f64 * (job_counter as libc::c_double + last_sec);
    if 0x4 as i32 & db_level != 0 {
        printf(
            b"Estimated system load = %f (actual = %f) (max requested = %f)\n\0"
                as *const u8 as *const i8,
            guess,
            load,
            max_load_average,
        );
        fflush(stdout);
    }
    return (guess >= max_load_average) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn start_waiting_jobs() {
    let mut job: *mut child = 0 as *mut child;
    if waiting_jobs.is_null() {
        return;
    }
    loop {
        reap_children(0 as i32, 0 as i32);
        job = waiting_jobs;
        waiting_jobs = (*job).next;
        if !(start_waiting_job(job) != 0 && !waiting_jobs.is_null()) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn child_execute_job(
    mut child: *mut childbase,
    mut good_stdin: i32,
    mut argv: *mut *mut i8,
) -> pid_t {
    let mut current_block: u64;
    let fdin: i32 = if good_stdin != 0 { fileno(stdin) } else { get_bad_stdin() };
    let mut fdout: i32 = fileno(stdout);
    let mut fderr: i32 = fileno(stderr);
    let mut pid: pid_t = -(1 as i32);
    let mut r: i32 = 0;
    let mut cmd: *mut i8 = 0 as *mut i8;
    let mut attr: posix_spawnattr_t = posix_spawnattr_t {
        __flags: 0,
        __pgrp: 0,
        __sd: sigset_t { __val: [0; 16] },
        __ss: sigset_t { __val: [0; 16] },
        __sp: sched_param { sched_priority: 0 },
        __policy: 0,
        __pad: [0; 16],
    };
    let mut fa: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    let mut flags: libc::c_short = 0 as i32 as libc::c_short;
    if ((*child).output).syncout() != 0 {
        if (*child).output.out >= 0 as i32 {
            fdout = (*child).output.out;
        }
        if (*child).output.err >= 0 as i32 {
            fderr = (*child).output.err;
        }
    }
    r = posix_spawnattr_init(&mut attr);
    if !(r != 0 as i32) {
        r = posix_spawn_file_actions_init(&mut fa);
        if r != 0 as i32 {
            posix_spawnattr_destroy(&mut attr);
        } else {
            let mut mask: sigset_t = sigset_t { __val: [0; 16] };
            sigemptyset(&mut mask);
            r = posix_spawnattr_setsigmask(&mut attr, &mut mask);
            if !(r != 0 as i32) {
                flags = (flags as i32 | 0x8 as i32) as libc::c_short;
                flags = (flags as i32 | 0x40 as i32) as libc::c_short;
                if fdin >= 0 as i32 && fdin != fileno(stdin) {
                    r = posix_spawn_file_actions_adddup2(&mut fa, fdin, fileno(stdin));
                    if r != 0 as i32 {
                        current_block = 1644677767690084885;
                    } else {
                        current_block = 17833034027772472439;
                    }
                } else {
                    current_block = 17833034027772472439;
                }
                match current_block {
                    1644677767690084885 => {}
                    _ => {
                        if fdout != fileno(stdout) {
                            r = posix_spawn_file_actions_adddup2(
                                &mut fa,
                                fdout,
                                fileno(stdout),
                            );
                            if r != 0 as i32 {
                                current_block = 1644677767690084885;
                            } else {
                                current_block = 7175849428784450219;
                            }
                        } else {
                            current_block = 7175849428784450219;
                        }
                        match current_block {
                            1644677767690084885 => {}
                            _ => {
                                if fderr != fileno(stderr) {
                                    r = posix_spawn_file_actions_adddup2(
                                        &mut fa,
                                        fderr,
                                        fileno(stderr),
                                    );
                                    if r != 0 as i32 {
                                        current_block = 1644677767690084885;
                                    } else {
                                        current_block = 5601891728916014340;
                                    }
                                } else {
                                    current_block = 5601891728916014340;
                                }
                                match current_block {
                                    1644677767690084885 => {}
                                    _ => {
                                        r = posix_spawnattr_setflags(&mut attr, flags);
                                        if !(r != 0 as i32) {
                                            let mut p: *const i8 = 0 as *const i8;
                                            let mut pp: *mut *mut i8 = 0 as *mut *mut i8;
                                            pp = (*child).environment;
                                            while !(*pp).is_null() {
                                                if *(*pp).offset(0 as i32 as isize) as i32 == 'P' as i32
                                                    && *(*pp).offset(1 as i32 as isize) as i32 == 'A' as i32
                                                    && *(*pp).offset(2 as i32 as isize) as i32 == 'T' as i32
                                                    && *(*pp).offset(3 as i32 as isize) as i32 == 'H' as i32
                                                    && *(*pp).offset(4 as i32 as isize) as i32 == '=' as i32
                                                {
                                                    p = (*pp).offset(5 as i32 as isize);
                                                    break;
                                                } else {
                                                    pp = pp.offset(1);
                                                    pp;
                                                }
                                            }
                                            if p.is_null() {
                                                let mut l: size_t = confstr(
                                                    C2RustUnnamed::_CS_PATH as i32,
                                                    0 as *mut i8,
                                                    0 as i32 as size_t,
                                                );
                                                if l != 0 {
                                                    let mut fresh18 = ::std::vec::from_elem(0, l as usize);
                                                    let mut dp: *mut i8 = fresh18.as_mut_ptr() as *mut i8;
                                                    confstr(C2RustUnnamed::_CS_PATH as i32, dp, l);
                                                    p = dp;
                                                }
                                            }
                                            cmd = find_in_given_path(
                                                *argv.offset(0 as i32 as isize),
                                                p,
                                                0 as *const i8,
                                                0 as i32 != 0,
                                            ) as *mut i8;
                                            if cmd.is_null() {
                                                r = *__errno_location();
                                            } else {
                                                loop {
                                                    r = posix_spawn(
                                                        &mut pid,
                                                        cmd,
                                                        &mut fa,
                                                        &mut attr,
                                                        argv,
                                                        (*child).environment,
                                                    );
                                                    if !(r == 4 as i32) {
                                                        break;
                                                    }
                                                }
                                                if r == 8 as i32 {
                                                    let mut nargv: *mut *mut i8 = 0 as *mut *mut i8;
                                                    let mut pp_0: *mut *mut i8 = 0 as *mut *mut i8;
                                                    let mut l_0: size_t = 0 as i32 as size_t;
                                                    pp_0 = argv;
                                                    while !(*pp_0).is_null() {
                                                        l_0 = l_0.wrapping_add(1);
                                                        l_0;
                                                        pp_0 = pp_0.offset(1);
                                                        pp_0;
                                                    }
                                                    nargv = xmalloc(
                                                        (::core::mem::size_of::<*mut i8>() as u64)
                                                            .wrapping_mul(l_0.wrapping_add(3 as i32 as u64)),
                                                    ) as *mut *mut i8;
                                                    let ref mut fresh19 = *nargv.offset(0 as i32 as isize);
                                                    *fresh19 = default_shell as *mut i8;
                                                    let ref mut fresh20 = *nargv.offset(1 as i32 as isize);
                                                    *fresh20 = cmd;
                                                    memcpy(
                                                        &mut *nargv.offset(2 as i32 as isize) as *mut *mut i8
                                                            as *mut libc::c_void,
                                                        &mut *argv.offset(1 as i32 as isize) as *mut *mut i8
                                                            as *const libc::c_void,
                                                        (::core::mem::size_of::<*mut i8>() as u64).wrapping_mul(l_0),
                                                    );
                                                    loop {
                                                        r = posix_spawn(
                                                            &mut pid,
                                                            *nargv.offset(0 as i32 as isize),
                                                            &mut fa,
                                                            &mut attr,
                                                            nargv,
                                                            (*child).environment,
                                                        );
                                                        if !(r == 4 as i32) {
                                                            break;
                                                        }
                                                    }
                                                    free(nargv as *mut libc::c_void);
                                                }
                                                if r == 0 as i32 {
                                                    free((*child).cmd_name as *mut libc::c_void);
                                                    if cmd != *argv.offset(0 as i32 as isize) {
                                                        (*child).cmd_name = cmd;
                                                    } else {
                                                        (*child).cmd_name = xstrdup(cmd);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            posix_spawn_file_actions_destroy(&mut fa);
            posix_spawnattr_destroy(&mut attr);
        }
    }
    if r != 0 as i32 {
        pid = -(1 as i32);
    }
    if pid < 0 as i32 {
        error(
            0 as *mut floc,
            (strlen(*argv.offset(0 as i32 as isize))).wrapping_add(strlen(strerror(r))),
            b"%s: %s\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
            strerror(r),
        );
    }
    return pid;
}
#[no_mangle]
pub unsafe extern "C" fn exec_command(
    mut argv: *mut *mut i8,
    mut envp: *mut *mut i8,
) -> pid_t {
    let mut pid: pid_t = -(1 as i32);
    environ = envp;
    execvp(*argv.offset(0 as i32 as isize), argv as *const *mut i8);
    match *__errno_location() {
        2 => {
            error(
                0 as *mut floc,
                (strlen(*argv.offset(0 as i32 as isize)))
                    .wrapping_add(strlen(strerror(*__errno_location()))),
                b"%s: %s\0" as *const u8 as *const i8,
                *argv.offset(0 as i32 as isize),
                strerror(*__errno_location()),
            );
        }
        8 => {
            let mut shell: *const i8 = 0 as *const i8;
            let mut new_argv: *mut *mut i8 = 0 as *mut *mut i8;
            let mut argc: i32 = 0;
            let mut i: i32 = 1 as i32;
            shell = getenv(b"SHELL\0" as *const u8 as *const i8);
            if shell.is_null() {
                shell = default_shell;
            }
            argc = 1 as i32;
            while !(*argv.offset(argc as isize)).is_null() {
                argc += 1;
                argc;
            }
            let mut fresh21 = ::std::vec::from_elem(
                0,
                ((1 as i32 + argc + 1 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64) as usize,
            );
            new_argv = fresh21.as_mut_ptr() as *mut *mut i8;
            let ref mut fresh22 = *new_argv.offset(0 as i32 as isize);
            *fresh22 = shell as *mut i8;
            let ref mut fresh23 = *new_argv.offset(i as isize);
            *fresh23 = *argv.offset(0 as i32 as isize);
            while argc > 0 as i32 {
                let ref mut fresh24 = *new_argv.offset((i + argc) as isize);
                *fresh24 = *argv.offset(argc as isize);
                argc -= 1;
                argc;
            }
            execvp(shell, new_argv as *const *mut i8);
            error(
                0 as *mut floc,
                (strlen(*new_argv.offset(0 as i32 as isize)))
                    .wrapping_add(strlen(strerror(*__errno_location()))),
                b"%s: %s\0" as *const u8 as *const i8,
                *new_argv.offset(0 as i32 as isize),
                strerror(*__errno_location()),
            );
        }
        _ => {
            error(
                0 as *mut floc,
                (strlen(*argv.offset(0 as i32 as isize)))
                    .wrapping_add(strlen(strerror(*__errno_location()))),
                b"%s: %s\0" as *const u8 as *const i8,
                *argv.offset(0 as i32 as isize),
                strerror(*__errno_location()),
            );
        }
    }
    return pid;
}
unsafe extern "C" fn construct_command_argv_internal(
    mut line: *mut i8,
    mut restp: *mut *mut i8,
    mut shell: *const i8,
    mut shellflags: *const i8,
    mut ifs: *const i8,
    mut flags: i32,
    mut batch_filename: *mut *mut i8,
) -> *mut *mut i8 {
    let mut current_block: u64;
    static mut sh_chars: *const i8 = b"#;\"*?[]&|<>(){}$`^~!\0" as *const u8
        as *const i8;
    static mut sh_cmds: [*const i8; 38] = [
        b".\0" as *const u8 as *const i8,
        b":\0" as *const u8 as *const i8,
        b"alias\0" as *const u8 as *const i8,
        b"bg\0" as *const u8 as *const i8,
        b"break\0" as *const u8 as *const i8,
        b"case\0" as *const u8 as *const i8,
        b"cd\0" as *const u8 as *const i8,
        b"command\0" as *const u8 as *const i8,
        b"continue\0" as *const u8 as *const i8,
        b"eval\0" as *const u8 as *const i8,
        b"exec\0" as *const u8 as *const i8,
        b"exit\0" as *const u8 as *const i8,
        b"export\0" as *const u8 as *const i8,
        b"fc\0" as *const u8 as *const i8,
        b"fg\0" as *const u8 as *const i8,
        b"for\0" as *const u8 as *const i8,
        b"getopts\0" as *const u8 as *const i8,
        b"hash\0" as *const u8 as *const i8,
        b"if\0" as *const u8 as *const i8,
        b"jobs\0" as *const u8 as *const i8,
        b"login\0" as *const u8 as *const i8,
        b"logout\0" as *const u8 as *const i8,
        b"read\0" as *const u8 as *const i8,
        b"readonly\0" as *const u8 as *const i8,
        b"return\0" as *const u8 as *const i8,
        b"set\0" as *const u8 as *const i8,
        b"shift\0" as *const u8 as *const i8,
        b"test\0" as *const u8 as *const i8,
        b"times\0" as *const u8 as *const i8,
        b"trap\0" as *const u8 as *const i8,
        b"type\0" as *const u8 as *const i8,
        b"ulimit\0" as *const u8 as *const i8,
        b"umask\0" as *const u8 as *const i8,
        b"unalias\0" as *const u8 as *const i8,
        b"unset\0" as *const u8 as *const i8,
        b"wait\0" as *const u8 as *const i8,
        b"while\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut i: size_t = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut ap: *mut i8 = 0 as *mut i8;
    let mut cap: *const i8 = 0 as *const i8;
    let mut cp: *const i8 = 0 as *const i8;
    let mut instring: i32 = 0;
    let mut word_has_equals: i32 = 0;
    let mut seen_nonequals: i32 = 0;
    let mut last_argument_was_empty: i32 = 0;
    let mut new_argv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut argstr: *mut i8 = 0 as *mut i8;
    if !restp.is_null() {
        *restp = 0 as *mut i8;
    }
    while *stopchar_map.as_mut_ptr().offset(*line as u8 as isize) as i32 & 0x2 as i32
        != 0 as i32
    {
        line = line.offset(1);
        line;
    }
    if *line as i32 == '\0' as i32 {
        return 0 as *mut *mut i8;
    }
    if shellflags.is_null() {
        shellflags = if posix_pedantic != 0 && !(flags & 4 as i32 != 0 as i32) {
            b"-ec\0" as *const u8 as *const i8
        } else {
            b"-c\0" as *const u8 as *const i8
        };
    }
    if shell.is_null() {
        shell = default_shell;
        current_block = 2979737022853876585;
    } else if strcmp(shell, default_shell) != 0 {
        current_block = 8102658916883067714;
    } else {
        current_block = 2979737022853876585;
    }
    match current_block {
        2979737022853876585 => {
            if !ifs.is_null() {
                cap = ifs;
                loop {
                    if !(*cap as i32 != '\0' as i32) {
                        current_block = 7149356873433890176;
                        break;
                    }
                    if *cap as i32 != ' ' as i32 && *cap as i32 != '\t' as i32
                        && *cap as i32 != '\n' as i32
                    {
                        current_block = 8102658916883067714;
                        break;
                    }
                    cap = cap.offset(1);
                    cap;
                }
            } else {
                current_block = 7149356873433890176;
            }
            match current_block {
                8102658916883067714 => {}
                _ => {
                    if !shellflags.is_null() {
                        if *shellflags.offset(0 as i32 as isize) as i32 != '-' as i32
                            || (*shellflags.offset(1 as i32 as isize) as i32
                                != 'c' as i32
                                || *shellflags.offset(2 as i32 as isize) as i32
                                    != '\0' as i32)
                                && (*shellflags.offset(1 as i32 as isize) as i32
                                    != 'e' as i32
                                    || *shellflags.offset(2 as i32 as isize) as i32
                                        != 'c' as i32
                                    || *shellflags.offset(3 as i32 as isize) as i32
                                        != '\0' as i32)
                        {
                            current_block = 8102658916883067714;
                        } else {
                            current_block = 5948590327928692120;
                        }
                    } else {
                        current_block = 5948590327928692120;
                    }
                    match current_block {
                        8102658916883067714 => {}
                        _ => {
                            i = (strlen(line)).wrapping_add(1 as i32 as u64);
                            new_argv = xmalloc(
                                i.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                            ) as *mut *mut i8;
                            argstr = xmalloc(i) as *mut i8;
                            let ref mut fresh25 = *new_argv.offset(0 as i32 as isize);
                            *fresh25 = argstr;
                            ap = *fresh25;
                            i = 0 as i32 as size_t;
                            last_argument_was_empty = 0 as i32;
                            seen_nonequals = last_argument_was_empty;
                            word_has_equals = seen_nonequals;
                            instring = word_has_equals;
                            p = line;
                            's_110: loop {
                                if !(*p as i32 != '\0' as i32) {
                                    current_block = 18370338663882161955;
                                    break;
                                }
                                if instring != 0 {
                                    if *p as i32 == instring {
                                        instring = 0 as i32;
                                        if ap == *new_argv.offset(0 as i32 as isize)
                                            || *ap.offset(-(1 as i32 as isize)) as i32 == '\0' as i32
                                        {
                                            last_argument_was_empty = 1 as i32;
                                        }
                                    } else if *p as i32 == '\\' as i32
                                        && *p.offset(1 as i32 as isize) as i32 == '\n' as i32
                                    {
                                        if instring == '"' as i32 {
                                            p = p.offset(1);
                                            p;
                                        } else {
                                            let fresh26 = p;
                                            p = p.offset(1);
                                            let fresh27 = ap;
                                            ap = ap.offset(1);
                                            *fresh27 = *fresh26;
                                            let fresh28 = ap;
                                            ap = ap.offset(1);
                                            *fresh28 = *p;
                                        }
                                    } else if *p as i32 == '\n' as i32 && !restp.is_null() {
                                        *restp = p;
                                        current_block = 18370338663882161955;
                                        break;
                                    } else {
                                        if instring == '"' as i32
                                            && !(strchr(b"\\$`\0" as *const u8 as *const i8, *p as i32))
                                                .is_null() && unixy_shell != 0
                                        {
                                            current_block = 8102658916883067714;
                                            break;
                                        }
                                        let fresh29 = ap;
                                        ap = ap.offset(1);
                                        *fresh29 = *p;
                                    }
                                } else if !(strchr(sh_chars, *p as i32)).is_null() {
                                    current_block = 8102658916883067714;
                                    break;
                                } else if one_shell != 0 && *p as i32 == '\n' as i32 {
                                    current_block = 8102658916883067714;
                                    break;
                                } else {
                                    match *p as i32 {
                                        61 => {
                                            if seen_nonequals == 0 && unixy_shell != 0 {
                                                current_block = 8102658916883067714;
                                                break;
                                            }
                                            word_has_equals = 1 as i32;
                                            let fresh30 = ap;
                                            ap = ap.offset(1);
                                            *fresh30 = '=' as i32 as i8;
                                        }
                                        92 => {
                                            if *p.offset(1 as i32 as isize) as i32 == '\n' as i32 {
                                                p = p.offset(1);
                                                p;
                                                if ap == *new_argv.offset(i as isize) {
                                                    while *stopchar_map
                                                        .as_mut_ptr()
                                                        .offset(*p.offset(1 as i32 as isize) as u8 as isize) as i32
                                                        & 0x2 as i32 != 0 as i32
                                                    {
                                                        p = p.offset(1);
                                                        p;
                                                    }
                                                }
                                            } else if *p.offset(1 as i32 as isize) as i32 != '\0' as i32
                                            {
                                                p = p.offset(1);
                                                let fresh31 = ap;
                                                ap = ap.offset(1);
                                                *fresh31 = *p;
                                            }
                                        }
                                        39 | 34 => {
                                            instring = *p as i32;
                                        }
                                        10 => {
                                            if !restp.is_null() {
                                                *restp = p;
                                                current_block = 18370338663882161955;
                                                break;
                                            } else {
                                                let fresh32 = ap;
                                                ap = ap.offset(1);
                                                *fresh32 = '\n' as i32 as i8;
                                            }
                                        }
                                        32 | 9 => {
                                            let fresh33 = ap;
                                            ap = ap.offset(1);
                                            *fresh33 = '\0' as i32 as i8;
                                            i = i.wrapping_add(1);
                                            let ref mut fresh34 = *new_argv.offset(i as isize);
                                            *fresh34 = ap;
                                            last_argument_was_empty = 0 as i32;
                                            seen_nonequals |= (word_has_equals == 0) as i32;
                                            if word_has_equals != 0 && seen_nonequals == 0 {
                                                current_block = 8102658916883067714;
                                                break;
                                            } else {
                                                word_has_equals = 0 as i32;
                                                if i == 1 as i32 as u64 {
                                                    let mut j: i32 = 0;
                                                    j = 0 as i32;
                                                    while !(sh_cmds[j as usize]).is_null() {
                                                        if sh_cmds[j as usize]
                                                            == *new_argv.offset(0 as i32 as isize)
                                                            || *sh_cmds[j as usize] as i32
                                                                == **new_argv.offset(0 as i32 as isize) as i32
                                                                && (*sh_cmds[j as usize] as i32 == '\0' as i32
                                                                    || strcmp(
                                                                        (sh_cmds[j as usize]).offset(1 as i32 as isize),
                                                                        (*new_argv.offset(0 as i32 as isize))
                                                                            .offset(1 as i32 as isize),
                                                                    ) == 0)
                                                        {
                                                            current_block = 8102658916883067714;
                                                            break 's_110;
                                                        }
                                                        j += 1;
                                                        j;
                                                    }
                                                }
                                                while *stopchar_map
                                                    .as_mut_ptr()
                                                    .offset(*p.offset(1 as i32 as isize) as u8 as isize) as i32
                                                    & 0x2 as i32 != 0 as i32
                                                {
                                                    p = p.offset(1);
                                                    p;
                                                }
                                            }
                                        }
                                        _ => {
                                            let fresh35 = ap;
                                            ap = ap.offset(1);
                                            *fresh35 = *p;
                                        }
                                    }
                                }
                                p = p.offset(1);
                                p;
                            }
                            match current_block {
                                8102658916883067714 => {}
                                _ => {
                                    if !(instring != 0) {
                                        *ap = '\0' as i32 as i8;
                                        if *(*new_argv.offset(i as isize)).offset(0 as i32 as isize)
                                            as i32 != '\0' as i32 || last_argument_was_empty != 0
                                        {
                                            i = i.wrapping_add(1);
                                            i;
                                        }
                                        let ref mut fresh36 = *new_argv.offset(i as isize);
                                        *fresh36 = 0 as *mut i8;
                                        if i == 1 as i32 as u64 {
                                            let mut j_0: i32 = 0;
                                            j_0 = 0 as i32;
                                            loop {
                                                if (sh_cmds[j_0 as usize]).is_null() {
                                                    current_block = 3879520548144599102;
                                                    break;
                                                }
                                                if sh_cmds[j_0 as usize]
                                                    == *new_argv.offset(0 as i32 as isize)
                                                    || *sh_cmds[j_0 as usize] as i32
                                                        == **new_argv.offset(0 as i32 as isize) as i32
                                                        && (*sh_cmds[j_0 as usize] as i32 == '\0' as i32
                                                            || strcmp(
                                                                (sh_cmds[j_0 as usize]).offset(1 as i32 as isize),
                                                                (*new_argv.offset(0 as i32 as isize))
                                                                    .offset(1 as i32 as isize),
                                                            ) == 0)
                                                {
                                                    current_block = 8102658916883067714;
                                                    break;
                                                }
                                                j_0 += 1;
                                                j_0;
                                            }
                                        } else {
                                            current_block = 3879520548144599102;
                                        }
                                        match current_block {
                                            8102658916883067714 => {}
                                            _ => {
                                                if (*new_argv.offset(0 as i32 as isize)).is_null() {
                                                    free(argstr as *mut libc::c_void);
                                                    free(new_argv as *mut libc::c_void);
                                                    return 0 as *mut *mut i8;
                                                }
                                                return new_argv;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !new_argv.is_null() {
        free(argstr as *mut libc::c_void);
        free(new_argv as *mut libc::c_void);
    }
    let mut new_line: *mut i8 = 0 as *mut i8;
    let mut shell_len: size_t = strlen(shell);
    let mut line_len: size_t = strlen(line);
    let mut sflags_len: size_t = if !shellflags.is_null() {
        strlen(shellflags)
    } else {
        0 as i32 as u64
    };
    if one_shell != 0 {
        if is_bourne_compatible_shell(shell) != 0 {
            let mut f: *const i8 = line;
            let mut t: *mut i8 = line;
            while *f.offset(0 as i32 as isize) as i32 != '\0' as i32 {
                let mut esc: i32 = 0 as i32;
                while *stopchar_map.as_mut_ptr().offset(*f as u8 as isize) as i32
                    & 0x2 as i32 != 0 as i32 || *f as i32 == '-' as i32
                    || *f as i32 == '@' as i32 || *f as i32 == '+' as i32
                {
                    f = f.offset(1);
                    f;
                }
                while *f as i32 != '\0' as i32 {
                    let fresh37 = f;
                    f = f.offset(1);
                    let fresh38 = t;
                    t = t.offset(1);
                    *fresh38 = *fresh37;
                    if *f.offset(-(1 as i32) as isize) as i32 == '\\' as i32 {
                        esc = (esc == 0) as i32;
                    } else {
                        if *f.offset(-(1 as i32) as isize) as i32 == '\n' as i32
                            && esc == 0
                        {
                            break;
                        }
                        esc = 0 as i32;
                    }
                }
            }
            *t = '\0' as i32 as i8;
        }
        let mut n: i32 = 1 as i32;
        let mut nextp: *mut i8 = 0 as *mut i8;
        new_argv = xmalloc(
            (4 as i32 as u64)
                .wrapping_add(sflags_len.wrapping_div(2 as i32 as u64))
                .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        ) as *mut *mut i8;
        let ref mut fresh39 = *new_argv.offset(0 as i32 as isize);
        *fresh39 = xmalloc(
            shell_len
                .wrapping_add(sflags_len)
                .wrapping_add(line_len)
                .wrapping_add(3 as i32 as u64),
        ) as *mut i8;
        nextp = *fresh39;
        nextp = mempcpy(
            nextp as *mut libc::c_void,
            shell as *const libc::c_void,
            shell_len.wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        if shellflags.is_null() {
            let fresh40 = n;
            n = n + 1;
            let ref mut fresh41 = *new_argv.offset(fresh40 as isize);
            *fresh41 = nextp;
            let fresh42 = nextp;
            nextp = nextp.offset(1);
            *fresh42 = '\0' as i32 as i8;
        } else {
            let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
            let mut fresh43 = ::std::vec::from_elem(
                0,
                sflags_len.wrapping_add(1 as i32 as u64) as usize,
            );
            let mut f_0: *mut i8 = fresh43.as_mut_ptr() as *mut i8;
            memcpy(
                f_0 as *mut libc::c_void,
                shellflags as *const libc::c_void,
                sflags_len.wrapping_add(1 as i32 as u64),
            );
            argv = construct_command_argv_internal(
                f_0,
                0 as *mut *mut i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                flags,
                0 as *mut *mut i8,
            );
            if !argv.is_null() {
                let mut a: *mut *mut i8 = 0 as *mut *mut i8;
                a = argv;
                while !(*a).is_null() {
                    let fresh44 = n;
                    n = n + 1;
                    let ref mut fresh45 = *new_argv.offset(fresh44 as isize);
                    *fresh45 = nextp;
                    nextp = (stpcpy(nextp, *a)).offset(1 as i32 as isize);
                    a = a.offset(1);
                    a;
                }
                free(*argv.offset(0 as i32 as isize) as *mut libc::c_void);
                free(argv as *mut libc::c_void);
            }
        }
        let fresh46 = n;
        n = n + 1;
        let ref mut fresh47 = *new_argv.offset(fresh46 as isize);
        *fresh47 = nextp;
        memcpy(
            nextp as *mut libc::c_void,
            line as *const libc::c_void,
            line_len.wrapping_add(1 as i32 as u64),
        );
        let fresh48 = n;
        n = n + 1;
        let ref mut fresh49 = *new_argv.offset(fresh48 as isize);
        *fresh49 = 0 as *mut i8;
        return new_argv;
    }
    new_line = xmalloc(
        shell_len
            .wrapping_mul(2 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
            .wrapping_add(sflags_len)
            .wrapping_add(1 as i32 as u64)
            .wrapping_add(line_len.wrapping_mul(2 as i32 as u64))
            .wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    ap = new_line;
    cp = shell;
    while *cp as i32 != '\0' as i32 {
        if !(strchr(sh_chars, *cp as i32)).is_null() {
            let fresh50 = ap;
            ap = ap.offset(1);
            *fresh50 = '\\' as i32 as i8;
        }
        let fresh51 = ap;
        ap = ap.offset(1);
        *fresh51 = *cp;
        cp = cp.offset(1);
        cp;
    }
    let fresh52 = ap;
    ap = ap.offset(1);
    *fresh52 = ' ' as i32 as i8;
    if !shellflags.is_null() {
        ap = mempcpy(
            ap as *mut libc::c_void,
            shellflags as *const libc::c_void,
            sflags_len,
        ) as *mut i8;
        let fresh53 = ap;
        ap = ap.offset(1);
        *fresh53 = ' ' as i32 as i8;
    }
    p = line;
    while *p as i32 != '\0' as i32 {
        if !restp.is_null() && *p as i32 == '\n' as i32 {
            *restp = p;
            break;
        } else {
            if *p as i32 == '\\' as i32
                && *p.offset(1 as i32 as isize) as i32 == '\n' as i32
            {
                let fresh54 = ap;
                ap = ap.offset(1);
                *fresh54 = '\\' as i32 as i8;
                if batch_mode_shell == 0 {
                    let fresh55 = ap;
                    ap = ap.offset(1);
                    *fresh55 = '\\' as i32 as i8;
                }
                let fresh56 = ap;
                ap = ap.offset(1);
                *fresh56 = '\n' as i32 as i8;
                p = p.offset(1);
                p;
            } else {
                if unixy_shell != 0 && batch_mode_shell == 0
                    && (*p as i32 == '\\' as i32 || *p as i32 == '\'' as i32
                        || *p as i32 == '"' as i32
                        || *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
                            & (0x2 as i32 | 0x4 as i32) != 0 as i32
                        || !(strchr(sh_chars, *p as i32)).is_null())
                {
                    let fresh57 = ap;
                    ap = ap.offset(1);
                    *fresh57 = '\\' as i32 as i8;
                }
                let fresh58 = ap;
                ap = ap.offset(1);
                *fresh58 = *p;
            }
            p = p.offset(1);
            p;
        }
    }
    if ap
        == new_line
            .offset(shell_len as isize)
            .offset(sflags_len as isize)
            .offset(2 as i32 as isize)
    {
        free(new_line as *mut libc::c_void);
        return 0 as *mut *mut i8;
    }
    *ap = '\0' as i32 as i8;
    if unixy_shell != 0 {
        new_argv = construct_command_argv_internal(
            new_line,
            0 as *mut *mut i8,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            flags,
            0 as *mut *mut i8,
        );
    } else {
        fatal(
            0 as *mut floc,
            (::core::mem::size_of::<[i8; 10]>() as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                ),
            dcgettext(
                0 as *const i8,
                b"%s (line %d) Bad shell context (!unixy && !batch_mode_shell)\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            b"src/job.c\0" as *const u8 as *const i8,
            3688 as i32,
        );
    }
    free(new_line as *mut libc::c_void);
    return new_argv;
}
#[no_mangle]
pub unsafe extern "C" fn construct_command_argv(
    mut line: *mut i8,
    mut restp: *mut *mut i8,
    mut file: *mut file,
    mut cmd_flags: i32,
    mut batch_filename: *mut *mut i8,
) -> *mut *mut i8 {
    let mut shell: *mut i8 = 0 as *mut i8;
    let mut ifs: *mut i8 = 0 as *mut i8;
    let mut shellflags: *mut i8 = 0 as *mut i8;
    let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut var: *mut variable = 0 as *mut variable;
    let mut save: i32 = warn_undefined_variables_flag;
    warn_undefined_variables_flag = 0 as i32;
    shell = allocated_variable_expand_for_file(
        b"$(SHELL)\0" as *const u8 as *const i8,
        file,
    );
    var = lookup_variable_for_file(
        b".SHELLFLAGS\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 12]>() as u64).wrapping_sub(1 as i32 as u64),
        file,
    );
    if var.is_null() {
        shellflags = xstrdup(b"\0" as *const u8 as *const i8);
    } else if posix_pedantic != 0
        && (*var).origin() as i32 == variable_origin::o_default as i32
    {
        shellflags = xstrdup(
            if cmd_flags & 4 as i32 != 0 as i32 {
                b"-c\0" as *const u8 as *const i8
            } else {
                b"-ec\0" as *const u8 as *const i8
            },
        );
    } else {
        shellflags = allocated_variable_expand_for_file((*var).value, file);
    }
    ifs = allocated_variable_expand_for_file(
        b"$(IFS)\0" as *const u8 as *const i8,
        file,
    );
    warn_undefined_variables_flag = save;
    argv = construct_command_argv_internal(
        line,
        restp,
        shell,
        shellflags,
        ifs,
        cmd_flags,
        batch_filename,
    );
    free(shell as *mut libc::c_void);
    free(shellflags as *mut libc::c_void);
    free(ifs as *mut libc::c_void);
    return argv;
}