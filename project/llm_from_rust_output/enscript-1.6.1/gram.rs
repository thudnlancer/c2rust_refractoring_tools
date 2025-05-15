use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::ptr;
use std::mem;
use std::os::raw::{c_long, c_ulong, c_ushort, c_schar};
use std::slice;
use std::collections::LinkedList;

type size_t = c_ulong;
type __off_t = c_long;
type __off64_t = c_long;
type reg_syntax_t = c_ulong;
type regoff_t = c_int;

#[derive(Debug, Clone, Copy)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut c_void,
    pub __pad2: *mut c_void,
    pub __pad3: *mut c_void,
    pub __pad4: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}

#[derive(Debug, Clone, Copy)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: c_int,
}

pub type FILE = _IO_FILE;

#[derive(Debug, Clone, Copy)]
pub struct re_pattern_buffer {
    pub buffer: *mut u8,
    pub allocated: c_ulong,
    pub used: c_ulong,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut c_char,
    pub translate: *mut c_char,
    pub re_nsub: size_t,
    pub can_be_null: bool,
    pub regs_allocated: u8,
    pub fastmap_accurate: bool,
    pub no_sub: bool,
    pub not_bol: bool,
    pub not_eol: bool,
    pub newline_anchor: bool,
}

pub type regex_t = re_pattern_buffer;

#[derive(Debug, Clone, Copy)]
pub struct re_registers {
    pub num_regs: c_uint,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}

#[derive(Debug)]
pub struct ListItem {
    pub next: *mut ListItem,
    pub data: *mut c_void,
}

#[derive(Debug)]
pub struct List {
    pub head: *mut ListItem,
    pub tail: *mut ListItem,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Void = 0,
    String = 1,
    Regexp = 2,
    Integer = 3,
    Real = 4,
    Symbol = 5,
    Array = 6,
}

#[derive(Debug)]
pub struct Node {
    pub type_: NodeType,
    pub refcount: c_uint,
    pub linenum: c_uint,
    pub data: NodeData,
}

#[derive(Debug)]
pub enum NodeData {
    Str { data: *mut c_char, len: c_uint },
    Re {
        data: *mut c_char,
        len: c_uint,
        flags: c_uint,
        compiled: regex_t,
        matches: re_registers,
    },
    Integer(i32),
    Real(f64),
    Sym(*mut c_char),
    Array {
        array: *mut *mut Node,
        len: c_uint,
        allocated: c_uint,
    },
}

#[derive(Debug)]
pub struct Cons {
    pub car: *mut c_void,
    pub cdr: *mut c_void,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExprType {
    String = 0,
    Regexp = 1,
    Integer = 2,
    Real = 3,
    Symbol = 4,
    Not = 5,
    And = 6,
    Or = 7,
    Fcall = 8,
    Assign = 9,
    AddAssign = 10,
    SubAssign = 11,
    MulAssign = 12,
    DivAssign = 13,
    PostfixAdd = 14,
    PostfixSub = 15,
    PrefixAdd = 16,
    PrefixSub = 17,
    ArrayAssign = 18,
    ArrayRef = 19,
    QuestColon = 20,
    Mult = 21,
    Div = 22,
    Plus = 23,
    Minus = 24,
    Lt = 25,
    Gt = 26,
    Eq = 27,
    Ne = 28,
    Ge = 29,
    Le = 30,
}

#[derive(Debug)]
pub struct Expr {
    pub type_: ExprType,
    pub linenum: c_uint,
    pub data: ExprData,
}

#[derive(Debug)]
pub enum ExprData {
    Node(*mut Node),
    Not(*mut Expr),
    Fcall { name: *mut Node, args: *mut List },
    Assign { sym: *mut Node, expr: *mut Expr },
    ArrayAssign {
        expr1: *mut Expr,
        expr2: *mut Expr,
        expr3: *mut Expr,
    },
    ArrayRef {
        expr1: *mut Expr,
        expr2: *mut Expr,
    },
    QuestColon {
        cond: *mut Expr,
        expr1: *mut Expr,
        expr2: *mut Expr,
    },
    Op {
        left: *mut Expr,
        right: *mut Expr,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StmtType {
    Return = 0,
    Defsub = 1,
    Block = 2,
    If = 3,
    Expr = 4,
    While = 5,
    For = 6,
}

#[derive(Debug)]
pub struct Stmt {
    pub type_: StmtType,
    pub linenum: c_uint,
    pub data: StmtData,
}

#[derive(Debug)]
pub enum StmtData {
    Expr(*mut Expr),
    Defsub { name: *mut Node, closure: *mut Cons },
    If {
        expr: *mut Expr,
        then_stmt: *mut Stmt,
        else_stmt: *mut Stmt,
    },
    While {
        expr: *mut Expr,
        body: *mut Stmt,
    },
    For {
        init: *mut Expr,
        cond: *mut Expr,
        incr: *mut Expr,
        body: *mut Stmt,
    },
    Block(*mut List),
}

#[derive(Debug)]
pub enum YYSTYPE {
    List(*mut List),
    Node(*mut Node),
    Cons(*mut Cons),
    Stmt(*mut Stmt),
    Expr(*mut Expr),
}

static mut stderr: *mut FILE = ptr::null_mut();
static mut defs_file: *mut c_char = ptr::null_mut();
static mut linenum: c_uint = 0;
static mut global_stmts: *mut List = ptr::null_mut();
static mut start_stmts: *mut List = ptr::null_mut();
static mut startrules: *mut List = ptr::null_mut();
static mut namerules: *mut List = ptr::null_mut();

pub unsafe fn yyerror(msg: *const c_char) {
    fprintf(
        stderr,
        b"%s:%d: %s\n\0".as_ptr() as *const c_char,
        defs_file,
        linenum,
        msg,
    );
}

pub unsafe fn yyparse() -> c_int {
    // Implementation of yyparse would go here
    // This is a complex function that would need to be carefully translated
    // to Rust while maintaining the same behavior
    0
}

// Helper functions would need to be implemented as well
unsafe fn define_state(sym: *mut Node, rules: *mut List) {
    // Implementation
}

unsafe fn cons(car: *mut c_void, cdr: *mut c_void) -> *mut Cons {
    // Implementation
    ptr::null_mut()
}

unsafe fn mk_stmt(
    type_: StmtType,
    arg1: *mut c_void,
    arg2: *mut c_void,
    arg3: *mut c_void,
    arg4: *mut c_void,
) -> *mut Stmt {
    // Implementation
    ptr::null_mut()
}

unsafe fn mk_expr(
    type_: ExprType,
    arg1: *mut c_void,
    arg2: *mut c_void,
    arg3: *mut c_void,
) -> *mut Expr {
    // Implementation
    ptr::null_mut()
}

unsafe fn list() -> *mut List {
    // Implementation
    ptr::null_mut()
}

unsafe fn list_append(list: *mut List, data: *mut c_void) {
    // Implementation
}

extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn yylex() -> c_int;
}