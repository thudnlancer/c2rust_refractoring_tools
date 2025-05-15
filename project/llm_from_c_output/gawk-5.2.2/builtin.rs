use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_ulong};
use std::ptr;
use std::mem;
use std::str;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type AWKNUM = f64;
type NODE = Node;

#[derive(Debug)]
struct Node {
    type_: NodeType,
    flags: u32,
    stptr: *mut c_char,
    stlen: usize,
    numbr: AWKNUM,
    // Other fields omitted for brevity
}

#[derive(Debug)]
enum NodeType {
    NodeVal,
    NodeVar,
    NodeVarArray,
    NodeVarNew,
    NodeElemNew,
    NodeRegex,
    NodeArrayRef,
}

struct Redirect {
    flag: u32,
    value: *mut c_char,
    output: Output,
}

struct Output {
    fp: *mut FILE,
    opaque: *mut c_void,
    gawk_fflush: extern fn(*mut FILE, *mut c_void) -> c_int,
    gawk_fwrite: extern fn(*const c_void, usize, usize, *mut FILE, *mut c_void) -> usize,
    gawk_ferror: extern fn(*mut FILE, *mut c_void) -> c_int,
}

struct Regexp {
    // Regex implementation details
}

struct FILE;

// Constants
const GAWK_RANDOM_MAX: c_long = 0x7fffffff;
const INITIAL_OUT_SIZE: usize = 64;
const BUFSIZ: usize = 8192;

// Global variables
static mut args_array: *mut *mut NODE = ptr::null_mut();
static mut max_args: c_int = 0;
static mut fields_arr: *mut *mut NODE = ptr::null_mut();
static mut output_is_tty: bool = false;
static mut output_fp: *mut FILE = ptr::null_mut();

// Error handling
#[derive(Debug)]
struct GawkError {
    message: String,
}

impl fmt::Display for GawkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GawkError {}

// Utility functions
fn force_string(node: *mut NODE) -> *mut NODE {
    // Implementation
    node
}

fn force_number(node: *mut NODE) -> *mut NODE {
    // Implementation
    node
}

fn make_number(num: AWKNUM) -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn make_string(s: *const c_char, len: usize) -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn DEREF(node: *mut NODE) {
    // Implementation
}

fn POP() -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn POP_SCALAR() -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn POP_STRING() -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn POP_NUMBER() -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn POP_PARAM() -> *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn POP_ADDRESS() -> *mut *mut NODE {
    // Implementation
    ptr::null_mut()
}

fn PUSH(node: *mut NODE) {
    // Implementation
}

fn PUSH_ADDRESS(addr: *mut *mut NODE) {
    // Implementation
}

fn fatal(msg: &str) -> ! {
    panic!("{}", msg);
}

fn warning(msg: &str) {
    eprintln!("warning: {}", msg);
}

fn lintwarn(msg: &str) {
    eprintln!("lint warning: {}", msg);
}

fn check_exact_args(nargs: c_int, fname: &str, count: c_int) {
    if nargs != count {
        fatal(&format!("{}: called with {} arguments", fname, nargs));
    }
}

fn check_args_min_max(nargs: c_int, fname: &str, min: c_int, max: c_int) {
    if nargs < min || nargs > max {
        fatal(&format!("{}: called with {} arguments", fname, nargs));
    }
}

// Main functions
fn do_exp(nargs: c_int) -> *mut NODE {
    check_exact_args(nargs, "exp", 1);
    
    let tmp = POP_SCALAR();
    if do_lint && (fixtype(tmp).flags & NUMBER) == 0 {
        lintwarn("exp: received non-numeric argument");
    }
    
    let d = force_number(tmp).numbr;
    DEREF(tmp);
    
    let res = d.exp();
    if res.is_infinite() {
        warning(&format!("exp: argument {} is out of range", d));
    }
    
    make_number(res)
}

fn do_log(nargs: c_int) -> *mut NODE {
    check_exact_args(nargs, "log", 1);
    
    let tmp = POP_SCALAR();
    if do_lint && (fixtype(tmp).flags & NUMBER) == 0 {
        lintwarn("log: received non-numeric argument");
    }
    
    let arg = force_number(tmp).numbr;
    if arg < 0.0 {
        warning(&format!("log: received negative argument {}", arg));
    }
    
    let d = arg.ln();
    DEREF(tmp);
    make_number(d)
}

fn do_sqrt(nargs: c_int) -> *mut NODE {
    check_exact_args(nargs, "sqrt", 1);
    
    let tmp = POP_SCALAR();
    if do_lint && (fixtype(tmp).flags & NUMBER) == 0 {
        lintwarn("sqrt: received non-numeric argument");
    }
    
    let arg = force_number(tmp).numbr;
    if arg < 0.0 {
        warning(&format!("sqrt: received negative argument {}", arg));
    }
    
    DEREF(tmp);
    make_number(arg.sqrt())
}

fn do_int(nargs: c_int) -> *mut NODE {
    check_exact_args(nargs, "int", 1);
    
    let tmp = POP_SCALAR();
    if do_lint && (fixtype(tmp).flags & NUMBER) == 0 {
        lintwarn("int: received non-numeric argument");
    }
    
    let d = force_number(tmp).numbr;
    let d = if d >= 0.0 { d.floor() } else { d.ceil() };
    DEREF(tmp);
    make_number(d)
}

fn do_rand(nargs: c_int) -> *mut NODE {
    check_exact_args(nargs, "rand", 0);
    
    static mut firstrand: bool = true;
    static mut state: [u32; 256] = [0; 256];
    
    if firstrand {
        // Initialize random state
        firstrand = false;
    }
    
    let mut tmprand;
    loop {
        let d1 = rand() as f64 / (GAWK_RANDOM_MAX as f64 + 1.0);
        let d2 = rand() as f64 / (GAWK_RANDOM_MAX as f64 + 1.0);
        tmprand = 0.5 + ((d1 + d2) / (GAWK_RANDOM_MAX as f64 + 1.0)) - 0.5;
        if tmprand != 1.0 {
            break;
        }
    }
    
    make_number(tmprand)
}

fn do_srand(nargs: c_int) -> *mut NODE {
    static mut save_seed: c_long = 1;
    
    check_args_min_max(nargs, "srand", 0, 1);
    
    let ret = save_seed as AWKNUM;
    
    if nargs == 0 {
        save_seed = unsafe { time(ptr::null_mut()) };
    } else {
        let tmp = POP_SCALAR();
        if do_lint && (fixtype(tmp).flags & NUMBER) == 0 {
            lintwarn("srand: received non-numeric argument");
        }
        save_seed = force_number(tmp).numbr as c_long;
        DEREF(tmp);
    }
    
    unsafe { srand(save_seed as u32) };
    make_number(ret)
}

// External C functions
extern {
    fn time(arg: *mut c_void) -> c_long;
    fn srand(seed: u32);
    fn rand() -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn isdigit(c: c_int) -> c_int;
    fn isupper(c: c_int) -> c_int;
    fn islower(c: c_int) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
}

// Placeholder for do_lint
static do_lint: bool = false;

// Placeholder for fixtype
fn fixtype(node: *mut NODE) -> *mut NODE {
    node
}

// Additional helper functions would be implemented here