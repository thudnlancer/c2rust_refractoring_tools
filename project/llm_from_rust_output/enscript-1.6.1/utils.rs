use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;
use std::mem;
use std::collections::HashMap;
use regex::Regex;

type SizeT = usize;
type StringHashPtr = *mut StringHash;
type WarningLevel = c_uint;

const WARN_ALL: WarningLevel = 100;
const WARN_LIGHT: WarningLevel = 10;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Node {
    type_: NodeType,
    refcount: c_uint,
    linenum: c_uint,
    u: NodeUnion,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum NodeType {
    Void = 0,
    String = 1,
    Regexp = 2,
    Integer = 3,
    Real = 4,
    Symbol = 5,
    Array = 6,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub union NodeUnion {
    str_: StrData,
    re: RegexData,
    integer: c_int,
    real: f64,
    sym: *mut c_char,
    array: ArrayData,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StrData {
    data: *mut c_char,
    len: c_uint,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RegexData {
    data: *mut c_char,
    len: c_uint,
    flags: c_uint,
    compiled: Regex,
    matches: Registers,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ArrayData {
    array: *mut *mut Node,
    len: c_uint,
    allocated: c_uint,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Registers {
    num_regs: c_uint,
    start: *mut c_int,
    end: *mut c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Expr {
    type_: ExprType,
    linenum: c_uint,
    u: ExprUnion,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ExprType {
    String = 0,
    Regexp = 1,
    Integer = 2,
    Real = 3,
    Symbol = 4,
    Not = 5,
    And = 6,
    Or = 7,
    FCall = 8,
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub union ExprUnion {
    node: *mut Node,
    not: *mut Expr,
    fcall: FCallData,
    assign: AssignData,
    arrayassign: ArrayAssignData,
    arrayref: ArrayRefData,
    questcolon: QuestColonData,
    op: OpData,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FCallData {
    name: *mut Node,
    args: *mut List,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AssignData {
    sym: *mut Node,
    expr: *mut Expr,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ArrayAssignData {
    expr1: *mut Expr,
    expr2: *mut Expr,
    expr3: *mut Expr,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ArrayRefData {
    expr1: *mut Expr,
    expr2: *mut Expr,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QuestColonData {
    cond: *mut Expr,
    expr1: *mut Expr,
    expr2: *mut Expr,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OpData {
    left: *mut Expr,
    right: *mut Expr,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Stmt {
    type_: StmtType,
    linenum: c_uint,
    u: StmtUnion,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum StmtType {
    Return = 0,
    DefSub = 1,
    Block = 2,
    If = 3,
    Expr = 4,
    While = 5,
    For = 6,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub union StmtUnion {
    expr: *mut Expr,
    defsub: DefSubData,
    stmt_if: IfData,
    stmt_while: WhileData,
    stmt_for: ForData,
    block: *mut List,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DefSubData {
    name: *mut Node,
    closure: *mut Cons,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IfData {
    expr: *mut Expr,
    then_stmt: *mut Stmt,
    else_stmt: *mut Stmt,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WhileData {
    expr: *mut Expr,
    body: *mut Stmt,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ForData {
    init: *mut Expr,
    cond: *mut Expr,
    incr: *mut Expr,
    body: *mut Stmt,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ListItem {
    next: *mut ListItem,
    data: *mut c_void,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct List {
    head: *mut ListItem,
    tail: *mut ListItem,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Cons {
    car: *mut c_void,
    cdr: *mut c_void,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Environment {
    next: *mut Environment,
    name: *mut c_char,
    val: *mut Node,
}

#[derive(Debug)]
pub struct StringHash {
    map: HashMap<String, *mut c_void>,
}

static mut NVoid: *mut Node = ptr::null_mut();
static mut CurrentLinenum: c_uint = 0;
static mut CurrentMatch: *mut Registers = ptr::null_mut();
static mut CurrentMatchBuf: *mut c_char = ptr::null_mut();
static mut Program: *mut c_char = ptr::null_mut();
static mut DefsFile: *mut c_char = ptr::null_mut();
static mut Linenum: c_uint = 0;
static mut WarningLevel: WarningLevel = WARN_LIGHT;
static mut NsPrims: StringHashPtr = ptr::null_mut();
static mut NsVars: StringHashPtr = ptr::null_mut();
static mut NsSubs: StringHashPtr = ptr::null_mut();
static mut NsStates: StringHashPtr = ptr::null_mut();

pub fn list() -> *mut List {
    Box::into_raw(Box::new(List {
        head: ptr::null_mut(),
        tail: ptr::null_mut(),
    }))
}

pub fn list_prepend(list: *mut List, data: *mut c_void) {
    unsafe {
        let item = Box::into_raw(Box::new(ListItem {
            next: (*list).head,
            data,
        }));
        (*list).head = item;
        if (*list).tail.is_null() {
            (*list).tail = item;
        }
    }
}

pub fn list_append(list: *mut List, data: *mut c_void) {
    unsafe {
        let item = Box::into_raw(Box::new(ListItem {
            next: ptr::null_mut(),
            data,
        }));
        if !(*list).tail.is_null() {
            (*(*list).tail).next = item;
        } else {
            (*list).head = item;
        }
        (*list).tail = item;
    }
}

pub fn node_alloc(type_: NodeType) -> *mut Node {
    let mut n = Box::new(Node {
        type_,
        refcount: 1,
        linenum: unsafe { Linenum },
        u: NodeUnion {
            str_: StrData {
                data: ptr::null_mut(),
                len: 0,
            },
        },
    });

    if type_ == NodeType::Regexp {
        n.u.re.compiled.fastmap = Box::into_raw(vec![0; 256].into_boxed_slice()) as *mut c_char;
    }

    Box::into_raw(n)
}

pub fn node_copy(n: *mut Node) -> *mut Node {
    unsafe {
        let n2 = node_alloc((*n).type_);
        (*n2).linenum = (*n).linenum;

        match (*n).type_ {
            NodeType::String => {
                (*n2).u.str_.len = (*n).u.str_.len;
                (*n2).u.str_.data = Box::into_raw(
                    CString::new(
                        std::slice::from_raw_parts(
                            (*n).u.str_.data,
                            (*n).u.str_.len as usize,
                        )
                    )
                    .unwrap()
                    .into_raw(),
                );
            }
            NodeType::Regexp => {
                (*n2).u.re.data = CStr::from_ptr((*n).u.re.data).to_owned().into_raw();
                (*n2).u.re.len = (*n).u.re.len;
            }
            NodeType::Integer => {
                (*n2).u.integer = (*n).u.integer;
            }
            NodeType::Real => {
                (*n2).u.real = (*n).u.real;
            }
            NodeType::Symbol => {
                (*n2).u.sym = CStr::from_ptr((*n).u.sym).to_owned().into_raw();
            }
            NodeType::Array => {
                (*n2).u.array.len = (*n).u.array.len;
                (*n2).u.array.allocated = (*n2).u.array.len + 1;
                (*n2).u.array.array = Box::into_raw(
                    vec![ptr::null_mut(); (*n2).u.array.allocated as usize].into_boxed_slice(),
                ) as *mut *mut Node;

                for i in 0..(*n).u.array.len as isize {
                    *((*n2).u.array.array).offset(i) = node_copy(*((*n).u.array.array).offset(i));
                }
            }
            _ => {}
        }

        n2
    }
}

pub fn node_reference(node: *mut Node) {
    unsafe {
        (*node).refcount += 1;
    }
}

pub fn node_free(node: *mut Node) {
    if node.is_null() {
        return;
    }

    unsafe {
        (*node).refcount -= 1;
        if (*node).refcount > 0 {
            return;
        }

        match (*node).type_ {
            NodeType::String => {
                CString::from_raw((*node).u.str_.data);
            }
            NodeType::Regexp => {
                CString::from_raw((*node).u.re.data);
                Box::from_raw((*node).u.re.compiled.fastmap as *mut [c_char]);
            }
            NodeType::Array => {
                for i in 0..(*node).u.array.len as isize {
                    node_free(*((*node).u.array.array).offset(i));
                }
                Box::from_raw((*node).u.array.array as *mut [*mut Node]);
            }
            NodeType::Symbol => {
                CString::from_raw((*node).u.sym);
            }
            _ => {}
        }

        Box::from_raw(node);
    }
}

// 其他函数类似地转换为Rust代码...

pub fn eval_expr(expr: *mut Expr, env: *mut Environment) -> *mut Node {
    // 实现表达式求值逻辑
    unsafe { NVoid }
}

pub fn eval_statement(stmt: *mut Stmt, env: *mut Environment, return_seen: *mut c_int) -> *mut Node {
    // 实现语句执行逻辑
    unsafe { NVoid }
}

pub fn eval_statement_list(lst: *mut List, env: *mut Environment, return_seen: *mut c_int) -> *mut Node {
    // 实现语句列表执行逻辑
    unsafe { NVoid }
}