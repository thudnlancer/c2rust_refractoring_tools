use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::process;

#[repr(C)]
struct Symbol {
    owner: *mut c_void,
    next: *mut Symbol,
    entry: *mut c_void,
    type_: u32,
    name: *mut c_char,
    flag: u32,
    alias: *mut Symbol,
    active: c_int,
    expand_line: c_int,
    token_type: c_int,
    source: *mut c_char,
    def_line: c_int,
    ref_line: *mut c_void,
    level: c_int,
    decl: *mut c_char,
    storage: u32,
    arity: c_int,
    recursive: c_int,
    ord: usize,
    caller: *mut c_void,
    callee: *mut c_void,
}

#[repr(C)]
struct ArgpState {
    root_argp: *const c_void,
    argc: c_int,
    argv: *mut *mut c_char,
    next: c_int,
    flags: u32,
    arg_num: u32,
    quoted: c_int,
    input: *mut c_void,
    child_inputs: *mut *mut c_void,
    hook: *mut c_void,
    name: *mut c_char,
    err_stream: *mut c_void,
    out_stream: *mut c_void,
    pstate: *mut c_void,
}

#[repr(C)]
struct ArgpOption {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
    group: c_int,
}

#[repr(C)]
struct Argp {
    options: *const ArgpOption,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut ArgpState) -> c_int>,
    args_doc: *const c_char,
    doc: *const c_char,
    children: *const c_void,
    help_filter: Option<unsafe extern "C" fn(c_int, *const c_char, *mut c_void) -> *mut c_char>,
    argp_domain: *const c_char,
}

static mut DEBUG: c_int = 0;
static mut OUTNAME: *mut c_char = ptr::null_mut();
static mut PRINT_OPTION: c_int = 0;
static mut VERBOSE: c_int = 0;
static mut USE_INDENTATION: c_int = 0;
static mut RECORD_DEFINES: c_int = 0;
static mut STRICT_ANSI: c_int = 0;
static mut PRINT_LINE_NUMBERS: c_int = 0;
static mut PRINT_LEVELS: c_int = 0;
static mut PRINT_AS_TREE: c_int = 0;
static mut BRIEF_LISTING: c_int = 0;
static mut REVERSE_TREE: c_int = 0;
static mut MAX_DEPTH: c_int = 0;
static mut EMACS_OPTION: c_int = 0;
static mut OMIT_ARGUMENTS_OPTION: c_int = 0;
static mut OMIT_SYMBOL_NAMES_OPTION: c_int = 0;
static mut SYMBOL_MAP: c_int = 0;
static mut LEVEL_INDENT: [*mut c_char; 2] = [ptr::null_mut(); 2];
static mut LEVEL_END: [*mut c_char; 2] = [ptr::null_mut(); 2];
static mut LEVEL_BEGIN: *mut c_char = ptr::null_mut();
static mut PREPROCESS_OPTION: c_int = 0;
static mut START_NAME: *mut c_char = ptr::null_mut();
static mut ARGLIST: *mut c_void = ptr::null_mut();

unsafe extern "C" fn parse_opt(
    key: c_int,
    arg: *mut c_char,
    state: *mut ArgpState,
) -> c_int {
    0
}

unsafe extern "C" fn globals_only() -> c_int {
    (SYMBOL_MAP & 0x4) == 0
}

unsafe extern "C" fn include_symbol(sym: *mut Symbol) -> c_int {
    if sym.is_null() {
        return 0;
    }
    1
}

unsafe extern "C" fn xalloc_die() -> ! {
    process::exit(1);
}

unsafe extern "C" fn init() {
    if LEVEL_INDENT[0].is_null() {
        LEVEL_INDENT[0] = CString::new("    ").unwrap().into_raw();
    }
    if LEVEL_INDENT[1].is_null() {
        LEVEL_INDENT[1] = LEVEL_INDENT[0];
    }
    if LEVEL_END[0].is_null() {
        LEVEL_END[0] = CString::new("").unwrap().into_raw();
    }
    if LEVEL_END[1].is_null() {
        LEVEL_END[1] = CString::new("").unwrap().into_raw();
    }
}

fn main() {
    unsafe {
        START_NAME = CString::new("main").unwrap().into_raw();
        OUTNAME = CString::new("-").unwrap().into_raw();
        
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut c_args: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut _).collect();
        c_args.push(ptr::null_mut());
        
        let argc = c_args.len() as c_int - 1;
        let mut argv = c_args.as_mut_ptr();
        
        init();
        
        process::exit(0);
    }
}