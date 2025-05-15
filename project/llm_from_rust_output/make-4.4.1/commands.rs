use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{stat, kill, signal, unlink, exit, memcpy, memcmp, strcmp, strchr, strstr, strlen};
use std::collections::HashMap;

// Constants and types
const US_SUCCESS: c_uint = 0;
const US_NONE: c_uint = 1;
const US_QUESTION: c_uint = 2;
const US_FAILED: c_uint = 3;

const CS_NOT_STARTED: c_uint = 0;
const CS_DEPS_RUNNING: c_uint = 1;
const CS_RUNNING: c_uint = 2;
const CS_FINISHED: c_uint = 3;

type Pid = c_int;
type TimeT = i64;
type SizeT = usize;

// Struct definitions
#[derive(Debug)]
struct Timespec {
    tv_sec: TimeT,
    tv_nsec: i64,
}

#[derive(Debug)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
}

#[derive(Debug)]
struct Floc {
    filenm: *const c_char,
    lineno: c_ulong,
    offset: c_ulong,
}

#[derive(Debug)]
struct Commands {
    fileinfo: Floc,
    commands: *mut c_char,
    command_lines: *mut *mut c_char,
    lines_flags: *mut u8,
    ncommand_lines: u16,
    recipe_prefix: c_char,
    any_recurse: bool,
}

#[derive(Debug)]
struct Dep {
    next: *mut Dep,
    name: *const c_char,
    file: *mut File,
    shuf: *mut Dep,
    stem: *const c_char,
    flags: u8,
    changed: bool,
    ignore_mtime: bool,
    staticpattern: bool,
    need_2nd_expansion: bool,
    ignore_automatic_vars: bool,
    is_explicit: bool,
    wait_here: bool,
}

#[derive(Debug)]
struct VariableSet {
    table: HashMap<String, Variable>,
}

#[derive(Debug)]
struct Variable {
    name: String,
    value: String,
    fileinfo: Floc,
    length: u32,
    recursive: bool,
    append: bool,
    conditional: bool,
    per_target: bool,
    special: bool,
    exportable: bool,
    expanding: bool,
    private_var: bool,
    exp_count: u32,
    flavor: VariableFlavor,
    origin: VariableOrigin,
    export: VariableExport,
}

#[derive(Debug)]
enum VariableFlavor {
    Bogus,
    Simple,
    Recursive,
    Expand,
    Append,
    Conditional,
    Shell,
    AppendValue,
}

#[derive(Debug)]
enum VariableOrigin {
    Default,
    Env,
    File,
    EnvOverride,
    Command,
    Override,
    Automatic,
    Invalid,
}

#[derive(Debug)]
enum VariableExport {
    Default,
    Export,
    NoExport,
    IfSet,
}

#[derive(Debug)]
struct File {
    name: String,
    hname: String,
    vpath: String,
    deps: *mut Dep,
    cmds: *mut Commands,
    stem: String,
    also_make: *mut Dep,
    prev: *mut File,
    last: *mut File,
    renamed: *mut File,
    variables: *mut VariableSetList,
    pat_variables: *mut VariableSetList,
    parent: *mut File,
    double_colon: *mut File,
    last_mtime: u64,
    mtime_before_update: u64,
    considered: u32,
    command_flags: c_int,
    update_status: UpdateStatus,
    command_state: CommandState,
    builtin: bool,
    precious: bool,
    loaded: bool,
    unloaded: bool,
    low_resolution_time: bool,
    tried_implicit: bool,
    updating: bool,
    updated: bool,
    is_target: bool,
    cmd_target: bool,
    phony: bool,
    intermediate: bool,
    is_explicit: bool,
    secondary: bool,
    notintermediate: bool,
    dontcare: bool,
    ignore_vpath: bool,
    pat_searched: bool,
    no_diag: bool,
    was_shuffled: bool,
    snapped: bool,
}

#[derive(Debug)]
enum UpdateStatus {
    Success,
    None,
    Question,
    Failed,
}

#[derive(Debug)]
enum CommandState {
    NotStarted,
    DepsRunning,
    Running,
    Finished,
}

#[derive(Debug)]
struct VariableSetList {
    next: *mut VariableSetList,
    set: *mut VariableSet,
    next_is_parent: bool,
}

// Helper functions
fn stat_safe(path: &str) -> Result<Stat, i32> {
    unsafe {
        let mut stat_buf: Stat = mem::zeroed();
        let res = stat(CString::new(path).unwrap().as_ptr(), &mut stat_buf as *mut _);
        if res == 0 {
            Ok(stat_buf)
        } else {
            Err(res)
        }
    }
}

fn set_file_variables(file: &mut File, stem: Option<&str>) {
    // Implementation would mirror the C version but using safe Rust constructs
    // ...
}

fn chop_commands(cmds: &mut Commands) {
    // Implementation would mirror the C version but using safe Rust constructs
    // ...
}

fn execute_file_commands(file: &mut File) {
    // Implementation would mirror the C version but using safe Rust constructs
    // ...
}

fn delete_target(file: &File, on_behalf_of: Option<&str>) {
    // Implementation would mirror the C version but using safe Rust constructs
    // ...
}

fn delete_child_targets(child: &mut Child) {
    // Implementation would mirror the C version but using safe Rust constructs
    // ...
}

fn print_commands(cmds: &Commands) {
    // Implementation would mirror the C version but using safe Rust constructs
    // ...
}

// Main structs
#[derive(Debug)]
struct Child {
    cmd_name: String,
    environment: Vec<String>,
    output: Output,
    next: Option<Box<Child>>,
    file: *mut File,
    sh_batch_file: String,
    command_lines: Vec<String>,
    command_ptr: String,
    command_line: u32,
    pid: Pid,
    remote: bool,
    noerror: bool,
    good_stdin: bool,
    deleted: bool,
    recursive: bool,
    jobslot: bool,
    dontcare: bool,
}

#[derive(Debug)]
struct Output {
    out: c_int,
    err: c_int,
    syncout: bool,
}

// Global state
static mut HANDLING_FATAL_SIGNAL: c_int = 0;

// Signal handler
extern "C" fn fatal_error_signal(sig: c_int) {
    unsafe {
        HANDLING_FATAL_SIGNAL = 1;
        signal(sig, None);
        // Handle cleanup and child processes
        // ...
    }
}