use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_uint, c_void, c_double, c_long};
use std::ptr;
use std::mem;
use std::slice;
use libc::{size_t, strtod, strtol, exit, fprintf, sprintf, sscanf, fputc, fwrite, getenv, memcpy, strlen};
use regex_syntax::hir::Hir;
use regex_syntax::Parser;

type StringHashPtr = *mut c_void;
type NodeType = c_uint;
type ExprType = c_uint;

const nVOID: NodeType = 0;
const nSTRING: NodeType = 1;
const nREGEXP: NodeType = 2;
const nINTEGER: NodeType = 3;
const nREAL: NodeType = 4;
const nSYMBOL: NodeType = 5;
const nARRAY: NodeType = 6;

struct Node {
    type_: NodeType,
    refcount: c_uint,
    linenum: c_uint,
    u: NodeUnion,
}

union NodeUnion {
    str_: NodeString,
    re: NodeRegex,
    integer: c_int,
    real: c_double,
    sym: *mut c_char,
    array: NodeArray,
}

struct NodeString {
    data: *mut c_char,
    len: c_uint,
}

struct NodeRegex {
    data: *mut c_char,
    len: c_uint,
    flags: c_uint,
    compiled: regex_t,
    matches: re_registers,
}

struct NodeArray {
    array: *mut *mut Node,
    len: c_uint,
    allocated: c_uint,
}

struct regex_t {
    buffer: *mut u8,
    allocated: c_ulong,
    used: c_ulong,
    syntax: c_ulong,
    fastmap: *mut c_char,
    translate: *mut c_char,
    re_nsub: size_t,
    flags: u8,
}

struct re_registers {
    num_regs: c_uint,
    start: *mut c_int,
    end: *mut c_int,
}

struct ListItem {
    next: *mut ListItem,
    data: *mut c_void,
}

struct List {
    head: *mut ListItem,
    tail: *mut ListItem,
}

struct Environment {
    next: *mut Environment,
    name: *mut c_char,
    val: *mut Node,
}

type Primitive = Option<unsafe extern "C" fn(*mut c_char, *mut List, *mut Environment, c_uint) -> *mut Node>;

struct PrimitiveDef {
    name: *mut c_char,
    prim: Primitive,
}

static mut nvoid: *mut Node = ptr::null_mut();
static mut program: *mut c_char = ptr::null_mut();
static mut ofp: *mut FILE = ptr::null_mut();
static mut defs_file: *mut c_char = ptr::null_mut();
static mut ns_prims: StringHashPtr = ptr::null_mut();
static mut startrules: *mut List = ptr::null_mut();
static mut namerules: *mut List = ptr::null_mut();
static mut inbuf: *mut c_char = ptr::null_mut();
static mut data_in_buffer: c_uint = 0;
static mut current_fname: *mut c_char = ptr::null_mut();
static mut current_match: *mut re_registers = ptr::null_mut();
static mut current_match_buf: *mut c_char = ptr::null_mut();
static mut start_state: *mut c_char = ptr::null_mut();

unsafe fn match_arg(
    prim_name: *mut c_char,
    type_: NodeType,
    argp: &mut *mut ListItem,
    env: *mut Environment,
    linenum: c_uint,
) -> *mut Node {
    let arg = *argp;
    if arg.is_null() {
        fprintf(
            stderr,
            b"%s:%d: %s: too few arguments\n\0".as_ptr() as *const c_char,
            defs_file,
            linenum,
            prim_name,
        );
        exit(1);
    }

    let n = eval_expr((*arg).data as *mut Expr, env);
    if type_ != nVOID && (*n).type_ != type_ {
        fprintf(
            stderr,
            b"%s:%d: %s: illegal argument type\n\0".as_ptr() as *const c_char,
            defs_file,
            linenum,
            prim_name,
        );
        exit(1);
    }

    *argp = (*arg).next;
    n
}

unsafe fn prim_call(
    prim_name: *mut c_char,
    args: *mut List,
    env: *mut Environment,
    linenum: c_uint,
) -> *mut Node {
    let mut arg = (*args).head;
    let e = (*arg).data as *mut Expr;
    
    if (*e).type_ != eSYMBOL {
        fprintf(
            stderr,
            b"%s:%d: %s: illegal argument type\n\0".as_ptr() as *const c_char,
            defs_file,
            linenum,
            prim_name,
        );
        exit(1);
    }

    let cp = (*(*e).u.node).u.sym;
    arg = (*arg).next;

    if !arg.is_null() {
        fprintf(
            stderr,
            b"%s:%d: %s: too many arguments\n\0".as_ptr() as *const c_char,
            defs_file,
            linenum,
            prim_name,
        );
        exit(1);
    }

    execute_state(cp)
}

// Other primitive functions would follow similar patterns...

static mut prims: [PrimitiveDef; 25] = [
    PrimitiveDef {
        name: b"call\0".as_ptr() as *mut c_char,
        prim: Some(prim_call),
    },
    // Other primitives would be listed here...
    PrimitiveDef {
        name: ptr::null_mut(),
        prim: None,
    },
];

pub unsafe extern "C" fn init_primitives() {
    let mut old: *mut c_void = ptr::null_mut();
    let mut i = 0;
    
    while !prims[i].name.is_null() {
        if strhash_put(
            ns_prims,
            prims[i].name,
            strlen(prims[i].name) as c_int,
            mem::transmute(prims[i].prim),
            &mut old,
        ) == 0
        {
            fprintf(
                stderr,
                b"%s: out of memory\n\0".as_ptr() as *const c_char,
                program,
            );
            exit(1);
        }
        i += 1;
    }
}