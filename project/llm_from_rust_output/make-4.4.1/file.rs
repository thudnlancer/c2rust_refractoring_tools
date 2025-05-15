use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type size_t = c_ulong;
type uintmax_t = u64;
type time_t = i64;
type clockid_t = c_int;

#[derive(Debug, Clone, Copy)]
struct Timeval {
    tv_sec: time_t,
    tv_usec: c_long,
}

#[derive(Debug, Clone, Copy)]
struct Timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[derive(Debug, Clone, Copy)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UpdateStatus {
    UsSuccess = 0,
    UsNone = 1,
    UsQuestion = 2,
    UsFailed = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CmdState {
    CsNotStarted = 0,
    CsDepsRunning = 1,
    CsRunning = 2,
    CsFinished = 3,
}

#[derive(Debug, Clone)]
struct File {
    name: *const c_char,
    hname: *const c_char,
    vpath: *const c_char,
    deps: *mut Dep,
    cmds: *mut Commands,
    stem: *const c_char,
    also_make: *mut Dep,
    prev: *mut File,
    last: *mut File,
    renamed: *mut File,
    variables: *mut VariableSetList,
    pat_variables: *mut VariableSetList,
    parent: *mut File,
    double_colon: *mut File,
    last_mtime: uintmax_t,
    mtime_before_update: uintmax_t,
    considered: c_uint,
    command_flags: c_int,
    update_status: UpdateStatus,
    command_state: CmdState,
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

#[derive(Debug, Clone)]
struct VariableSetList {
    next: *mut VariableSetList,
    set: *mut VariableSet,
    next_is_parent: c_int,
}

#[derive(Debug, Clone)]
struct VariableSet {
    table: HashMap<*const c_char, *mut Variable>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Commands {
    fileinfo: Floc,
    commands: *mut c_char,
    command_lines: *mut *mut c_char,
    lines_flags: *mut u8,
    ncommand_lines: c_ushort,
    recipe_prefix: c_char,
    any_recurse: bool,
}

#[derive(Debug, Clone, Copy)]
struct Floc {
    filenm: *const c_char,
    lineno: c_ulong,
    offset: c_ulong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VariableOrigin {
    ODefault = 0,
    OEnv = 1,
    OFile = 2,
    OEnvOverride = 3,
    OCommand = 4,
    OOverride = 5,
    OAutomatic = 6,
    OInvalid = 7,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VariableFlavor {
    FBogus = 0,
    FSimple = 1,
    FRecursive = 2,
    FExpand = 3,
    FAppend = 4,
    FConditional = 5,
    FShell = 6,
    FAppendValue = 7,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VariableExport {
    VDefault = 0,
    VExport = 1,
    VNoexport = 2,
    VIfset = 3,
}

#[derive(Debug, Clone)]
struct Variable {
    name: *mut c_char,
    value: *mut c_char,
    fileinfo: Floc,
    length: c_uint,
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

static mut FILES: HashMap<*const c_char, *mut File> = HashMap::new();
static mut ALL_SECONDARY: c_int = 0;
static mut SNAPPED_DEPS: c_int = 0;

impl File {
    fn new(name: *const c_char) -> *mut File {
        unsafe {
            let mut file = Box::new(File {
                name,
                hname: name,
                vpath: ptr::null(),
                deps: ptr::null_mut(),
                cmds: ptr::null_mut(),
                stem: ptr::null(),
                also_make: ptr::null_mut(),
                prev: ptr::null_mut(),
                last: ptr::null_mut(),
                renamed: ptr::null_mut(),
                variables: ptr::null_mut(),
                pat_variables: ptr::null_mut(),
                parent: ptr::null_mut(),
                double_colon: ptr::null_mut(),
                last_mtime: 0,
                mtime_before_update: 0,
                considered: 0,
                command_flags: 0,
                update_status: UpdateStatus::UsNone,
                command_state: CmdState::CsNotStarted,
                builtin: false,
                precious: false,
                loaded: false,
                unloaded: false,
                low_resolution_time: false,
                tried_implicit: false,
                updating: false,
                updated: false,
                is_target: false,
                cmd_target: false,
                phony: false,
                intermediate: false,
                is_explicit: false,
                secondary: false,
                notintermediate: false,
                dontcare: false,
                ignore_vpath: false,
                pat_searched: false,
                no_diag: false,
                was_shuffled: false,
                snapped: false,
            });
            let file_ptr = Box::into_raw(file);
            (*file_ptr).last = file_ptr;
            FILES.insert(name, file_ptr);
            file_ptr
        }
    }
}

fn lookup_file(name: *const c_char) -> *mut File {
    unsafe { FILES.get(&name).cloned().unwrap_or(ptr::null_mut()) }
}

fn enter_file(name: *const c_char) -> *mut File {
    unsafe {
        let file = lookup_file(name);
        if !file.is_null() {
            (*file).builtin = false;
            file
        } else {
            File::new(name)
        }
    }
}

// Additional helper functions would be implemented here following similar patterns
// to convert the remaining C functions to safe Rust equivalents