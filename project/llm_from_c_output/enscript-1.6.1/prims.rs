use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use std::slice;
use std::str;

use regex::Regex;

type NodeType = i32;
type Primitive = fn(&str, &[Node], &Environment, u32) -> Node;

const N_VOID: NodeType = 0;
const N_STRING: NodeType = 1;
const N_REGEXP: NodeType = 2;
const N_INTEGER: NodeType = 3;
const N_REAL: NodeType = 4;
const N_SYMBOL: NodeType = 5;
const N_ARRAY: NodeType = 6;

struct Node {
    type_: NodeType,
    u: NodeUnion,
}

union NodeUnion {
    str: StringData,
    re: RegexData,
    integer: i32,
    real: f64,
    sym: *mut c_char,
    array: ArrayData,
}

struct StringData {
    data: *mut c_char,
    len: usize,
}

struct RegexData {
    data: *mut c_char,
    len: usize,
    matches: *mut re_registers,
}

struct ArrayData {
    array: *mut *mut Node,
    allocated: usize,
    len: usize,
}

struct Environment {
    // Environment fields
}

struct List {
    head: *mut ListItem,
}

struct ListItem {
    data: *mut Expr,
    next: *mut ListItem,
}

struct Expr {
    type_: i32,
    u: ExprUnion,
}

union ExprUnion {
    node: *mut Node,
}

struct re_registers {
    num_regs: usize,
    start: *mut i32,
    end: *mut i32,
}

static mut nvoid: Node = Node {
    type_: N_VOID,
    u: NodeUnion { integer: 0 },
};

static mut defs_file: *const c_char = ptr::null();
static mut program: *const c_char = ptr::null();
static mut ofp: *mut FILE = ptr::null_mut();
static mut start_state: *mut c_char = ptr::null_mut();
static mut current_fname: *const c_char = ptr::null();
static mut inbuf: *const c_char = ptr::null();
static mut data_in_buffer: usize = 0;
static mut namerules: *mut List = ptr::null_mut();
static mut startrules: *mut List = ptr::null_mut();
static mut current_match: *mut re_registers = ptr::null_mut();
static mut current_match_buf: *const c_char = ptr::null();
static mut current_match_node: *mut Node = ptr::null_mut();
static mut ns_prims: *mut HashMap<String, Primitive> = ptr::null_mut();

macro_rules! DEFUN {
    ($prim:ident) => {
        fn $prim(
            prim_name: &str,
            args: &[Node],
            env: &Environment,
            linenum: u32,
        ) -> Node {
            // Implementation
            unimplemented!()
        }
    };
}

macro_rules! NEED_ARG {
    () => {
        if arg.is_null() {
            eprintln!(
                "{}:{}: {}: too few arguments\n",
                unsafe { CStr::from_ptr(defs_file).to_string_lossy() },
                linenum,
                prim_name
            );
            std::process::exit(1);
        }
    };
}

macro_rules! LAST_ARG {
    () => {
        if !arg.is_null() {
            eprintln!(
                "{}:{}: {}: too many arguments\n",
                unsafe { CStr::from_ptr(defs_file).to_string_lossy() },
                linenum,
                prim_name
            );
            std::process::exit(1);
        }
    };
}

macro_rules! MATCH_ARG {
    ($type:expr) => {
        match_arg(prim_name, $type, &mut arg, env, linenum)
    };
}

macro_rules! APPEND {
    ($data:expr, $len:expr) => {
        if result_len < result_pos + $len {
            result_len += $len + 1024;
            result = unsafe { libc::realloc(result as *mut _, result_len) as *mut c_char };
        }
        unsafe {
            libc::memcpy(
                result.add(result_pos) as *mut _,
                $data as *const _,
                $len,
            );
        }
        result_pos += $len;
    };
}

macro_rules! FMTSPECIAL {
    ($ch:expr) => {
        (b'0' <= $ch && $ch <= b'9') || $ch == b'.' || $ch == b'-'
    };
}

DEFUN!(prim_call);
DEFUN!(prim_check_namerules);
DEFUN!(prim_check_startrules);
DEFUN!(prim_concat);
DEFUN!(prim_float);
DEFUN!(prim_getenv);
DEFUN!(prim_int);
DEFUN!(prim_length);
DEFUN!(prim_list);
DEFUN!(prim_panic);
DEFUN!(prim_prereq);
DEFUN!(prim_print);
DEFUN!(prim_range);
DEFUN!(prim_regexp);
DEFUN!(prim_regexp_syntax);
DEFUN!(prim_regmatch);
DEFUN!(prim_regsub);
DEFUN!(prim_regsuball);
DEFUN!(prim_split);
DEFUN!(prim_sprintf);
DEFUN!(prim_strcmp);
DEFUN!(prim_string);
DEFUN!(prim_strncmp);
DEFUN!(prim_substring);

fn match_arg(
    prim_name: &str,
    type_: NodeType,
    arg: &mut *mut ListItem,
    env: &Environment,
    linenum: u32,
) -> Node {
    NEED_ARG!();
    let n = unsafe { eval_expr((*arg).data, env) };
    if type_ != N_VOID && n.type_ != type_ {
        eprintln!(
            "{}:{}: {}: illegal argument type\n",
            unsafe { CStr::from_ptr(defs_file).to_string_lossy() },
            linenum,
            prim_name
        );
        std::process::exit(1);
    }
    unsafe {
        *arg = (*arg).next;
    }
    n
}

fn eval_expr(expr: *mut Expr, env: &Environment) -> Node {
    unimplemented!()
}

fn execute_state(state: &str) -> Node {
    unimplemented!()
}

fn re_search(
    regex: *const RegexData,
    text: *const c_char,
    len: usize,
    start: usize,
    end: usize,
    regs: *mut re_registers,
) -> i32 {
    unimplemented!()
}

fn node_alloc(type_: NodeType) -> Node {
    Node {
        type_,
        u: match type_ {
            N_STRING => NodeUnion {
                str: StringData {
                    data: ptr::null_mut(),
                    len: 0,
                },
            },
            N_REGEXP => NodeUnion {
                re: RegexData {
                    data: ptr::null_mut(),
                    len: 0,
                    matches: ptr::null_mut(),
                },
            },
            N_INTEGER => NodeUnion { integer: 0 },
            N_REAL => NodeUnion { real: 0.0 },
            N_SYMBOL => NodeUnion {
                sym: ptr::null_mut(),
            },
            N_ARRAY => NodeUnion {
                array: ArrayData {
                    array: ptr::null_mut(),
                    allocated: 0,
                    len: 0,
                },
            },
            _ => NodeUnion { integer: 0 },
        },
    }
}

fn node_free(node: Node) {
    unsafe {
        match node.type_ {
            N_STRING => libc::free(node.u.str.data as *mut _),
            N_REGEXP => {
                libc::free(node.u.re.data as *mut _);
                if !node.u.re.matches.is_null() {
                    libc::free(node.u.re.matches as *mut _);
                }
            }
            N_SYMBOL => libc::free(node.u.sym as *mut _),
            N_ARRAY => {
                for i in 0..node.u.array.len {
                    node_free(*node.u.array.array.add(i));
                }
                libc::free(node.u.array.array as *mut _);
            }
            _ => {}
        }
    }
}

fn print_node(node: &Node) {
    unsafe {
        match node.type_ {
            N_STRING => {
                let slice = slice::from_raw_parts(node.u.str.data as *const u8, node.u.str.len);
                ofp.write_all(slice).unwrap();
            }
            N_REGEXP => {
                ofp.write_all(b"/").unwrap();
                let slice = slice::from_raw_parts(node.u.re.data as *const u8, node.u.re.len);
                ofp.write_all(slice).unwrap();
                ofp.write_all(b"/").unwrap();
            }
            N_INTEGER => write!(ofp, "{}", node.u.integer).unwrap(),
            N_REAL => write!(ofp, "{}", node.u.real).unwrap(),
            N_SYMBOL => {
                let s = CStr::from_ptr(node.u.sym).to_string_lossy();
                write!(ofp, "{}", s).unwrap();
            }
            N_ARRAY => {
                for i in 0..node.u.array.len {
                    print_node(&*node.u.array.array.add(i));
                    if i + 1 < node.u.array.len {
                        write!(ofp, " ").unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_regsubsts(
    str_node: Node,
    re_node: Node,
    subst_node: Node,
    allp: bool,
) -> Node {
    unimplemented!()
}

fn init_primitives() {
    let prims = [
        ("call", prim_call as Primitive),
        ("check_namerules", prim_check_namerules),
        ("check_startrules", prim_check_startrules),
        ("concat", prim_concat),
        ("float", prim_float),
        ("getenv", prim_getenv),
        ("int", prim_int),
        ("length", prim_length),
        ("list", prim_list),
        ("panic", prim_panic),
        ("prereq", prim_prereq),
        ("print", prim_print),
        ("range", prim_range),
        ("regexp", prim_regexp),
        ("regexp_syntax", prim_regexp_syntax),
        ("regmatch", prim_regmatch),
        ("regsub", prim_regsub),
        ("regsuball", prim_regsuball),
        ("split", prim_split),
        ("sprintf", prim_sprintf),
        ("strcmp", prim_strcmp),
        ("string", prim_string),
        ("strncmp", prim_strncmp),
        ("substring", prim_substring),
    ];

    unsafe {
        if ns_prims.is_null() {
            ns_prims = Box::into_raw(Box::new(HashMap::new()));
        }
        let ns_prims = &mut *ns_prims;
        for (name, prim) in prims {
            ns_prims.insert(name.to_string(), prim);
        }
    }
}