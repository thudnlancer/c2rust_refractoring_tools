#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn yylex() -> libc::c_int;
    static mut defs_file: *mut libc::c_char;
    static mut linenum: libc::c_uint;
    static mut global_stmts: *mut List;
    static mut start_stmts: *mut List;
    static mut startrules: *mut List;
    static mut namerules: *mut List;
    fn define_state(sym: *mut Node, rules: *mut List);
    fn cons(car: *mut libc::c_void, cdr: *mut libc::c_void) -> *mut Cons;
    fn mk_stmt(
        type_0: StmtType,
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
        arg3: *mut libc::c_void,
        arg4: *mut libc::c_void,
    ) -> *mut Stmt;
    fn mk_expr(
        type_0: ExprType,
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
        arg3: *mut libc::c_void,
    ) -> *mut Expr;
    fn list() -> *mut List;
    fn list_append(list_0: *mut List, data: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut libc::c_uchar,
    pub allocated: libc::c_ulong,
    pub used: libc::c_ulong,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_char,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: libc::c_uint,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_item_st {
    pub next: *mut list_item_st,
    pub data: *mut libc::c_void,
}
pub type ListItem = list_item_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_st {
    pub head: *mut ListItem,
    pub tail: *mut ListItem,
}
pub type List = list_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum NodeType {
    nARRAY = 6,
    nSYMBOL = 5,
    nREAL = 4,
    nINTEGER = 3,
    nREGEXP = 2,
    nSTRING = 1,
    nVOID = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub type_0: NodeType,
    pub refcount: libc::c_uint,
    pub linenum: libc::c_uint,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: C2RustUnnamed_2,
    pub re: C2RustUnnamed_1,
    pub integer: libc::c_int,
    pub real: libc::c_double,
    pub sym: *mut libc::c_char,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub array: *mut *mut node_st,
    pub len: libc::c_uint,
    pub allocated: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
    pub flags: libc::c_uint,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
}
pub type Node = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons_st {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}
pub type Cons = cons_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ExprType {
    eLE = 30,
    eGE = 29,
    eNE = 28,
    eEQ = 27,
    eGT = 26,
    eLT = 25,
    eMINUS = 24,
    ePLUS = 23,
    eDIV = 22,
    eMULT = 21,
    eQUESTCOLON = 20,
    eARRAYREF = 19,
    eARRAYASSIGN = 18,
    ePREFIXSUB = 17,
    ePREFIXADD = 16,
    ePOSTFIXSUB = 15,
    ePOSTFIXADD = 14,
    eDIVASSIGN = 13,
    eMULASSIGN = 12,
    eSUBASSIGN = 11,
    eADDASSIGN = 10,
    eASSIGN = 9,
    eFCALL = 8,
    eOR = 7,
    eAND = 6,
    eNOT = 5,
    eSYMBOL = 4,
    eREAL = 3,
    eINTEGER = 2,
    eREGEXP = 1,
    eSTRING = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct expr_st {
    pub type_0: ExprType,
    pub linenum: libc::c_uint,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub node: *mut Node,
    pub not: *mut expr_st,
    pub fcall: C2RustUnnamed_9,
    pub assign: C2RustUnnamed_8,
    pub arrayassign: C2RustUnnamed_7,
    pub arrayref: C2RustUnnamed_6,
    pub questcolon: C2RustUnnamed_5,
    pub op: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub left: *mut expr_st,
    pub right: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub cond: *mut expr_st,
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
    pub expr3: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub sym: *mut Node,
    pub expr: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub name: *mut Node,
    pub args: *mut List,
}
pub type Expr = expr_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum StmtType {
    sFOR = 6,
    sWHILE = 5,
    sEXPR = 4,
    sIF = 3,
    sBLOCK = 2,
    sDEFSUB = 1,
    sRETURN = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_st {
    pub type_0: StmtType,
    pub linenum: libc::c_uint,
    pub u: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub expr: *mut Expr,
    pub defsub: C2RustUnnamed_14,
    pub stmt_if: C2RustUnnamed_13,
    pub stmt_while: C2RustUnnamed_12,
    pub stmt_for: C2RustUnnamed_11,
    pub block: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub init: *mut Expr,
    pub cond: *mut Expr,
    pub incr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub expr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub expr: *mut Expr,
    pub then_stmt: *mut stmt_st,
    pub else_stmt: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub name: *mut Node,
    pub closure: *mut Cons,
}
pub type Stmt = stmt_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub lst: *mut List,
    pub node: *mut Node,
    pub cons: *mut Cons,
    pub stmt: *mut Stmt,
    pub expr: *mut Expr,
}
static mut yytranslate: [libc::c_char; 289] = [
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    40 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    49 as libc::c_int as libc::c_char,
    50 as libc::c_int as libc::c_char,
    38 as libc::c_int as libc::c_char,
    36 as libc::c_int as libc::c_char,
    48 as libc::c_int as libc::c_char,
    37 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    27 as libc::c_int as libc::c_char,
    47 as libc::c_int as libc::c_char,
    32 as libc::c_int as libc::c_char,
    21 as libc::c_int as libc::c_char,
    33 as libc::c_int as libc::c_char,
    26 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    43 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    44 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    45 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    46 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    16 as libc::c_int as libc::c_char,
    17 as libc::c_int as libc::c_char,
    18 as libc::c_int as libc::c_char,
    19 as libc::c_int as libc::c_char,
    20 as libc::c_int as libc::c_char,
    22 as libc::c_int as libc::c_char,
    23 as libc::c_int as libc::c_char,
    24 as libc::c_int as libc::c_char,
    25 as libc::c_int as libc::c_char,
    28 as libc::c_int as libc::c_char,
    29 as libc::c_int as libc::c_char,
    30 as libc::c_int as libc::c_char,
    31 as libc::c_int as libc::c_char,
    34 as libc::c_int as libc::c_char,
    35 as libc::c_int as libc::c_char,
    39 as libc::c_int as libc::c_char,
    41 as libc::c_int as libc::c_char,
    42 as libc::c_int as libc::c_char,
];
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    lst: 0 as *const List as *mut List,
};
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
static mut yyr1: [libc::c_short; 75] = [
    0 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
];
static mut yyr2: [libc::c_short; 75] = [
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
];
static mut yydefact: [libc::c_short; 163] = [
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    73 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    70 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
];
static mut yydefgoto: [libc::c_short; 16] = [
    1 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    73 as libc::c_int as libc::c_short,
    102 as libc::c_int as libc::c_short,
    120 as libc::c_int as libc::c_short,
    100 as libc::c_int as libc::c_short,
    101 as libc::c_int as libc::c_short,
    138 as libc::c_int as libc::c_short,
    146 as libc::c_int as libc::c_short,
    147 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    81 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
];
static mut yypact: [libc::c_short; 163] = [
    -(32768 as libc::c_int) as libc::c_short,
    51 as libc::c_int as libc::c_short,
    312 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    1 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    -(33 as libc::c_int) as libc::c_short,
    -(22 as libc::c_int) as libc::c_short,
    -(21 as libc::c_int) as libc::c_short,
    281 as libc::c_int as libc::c_short,
    -(24 as libc::c_int) as libc::c_short,
    -(23 as libc::c_int) as libc::c_short,
    -(16 as libc::c_int) as libc::c_short,
    303 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    303 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    380 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    303 as libc::c_int as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    20 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    400 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    115 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    475 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    134 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    334 as libc::c_int as libc::c_short,
    357 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    459 as libc::c_int as libc::c_short,
    490 as libc::c_int as libc::c_short,
    504 as libc::c_int as libc::c_short,
    516 as libc::c_int as libc::c_short,
    516 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    -(36 as libc::c_int) as libc::c_short,
    -(36 as libc::c_int) as libc::c_short,
    10 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    440 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    303 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    38 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    91 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    275 as libc::c_int as libc::c_short,
    275 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    69 as libc::c_int as libc::c_short,
    97 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    420 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    275 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    475 as libc::c_int as libc::c_short,
    126 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    181 as libc::c_int as libc::c_short,
    209 as libc::c_int as libc::c_short,
    228 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    86 as libc::c_int as libc::c_short,
    123 as libc::c_int as libc::c_short,
    -(29 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    256 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    275 as libc::c_int as libc::c_short,
    303 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    126 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    475 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    145 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn yyerror(mut msg: *mut libc::c_char) {
    fprintf(
        stderr,
        b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
        defs_file,
        linenum,
        msg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyssp: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yychar1: libc::c_int = 0 as libc::c_int;
    let mut yyssa: [libc::c_short; 200] = [0; 200];
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        lst: 0 as *const List as *mut List,
    }; 200];
    let mut yyss: *mut libc::c_short = yyssa.as_mut_ptr();
    let mut yyvs: *mut YYSTYPE = yyvsa.as_mut_ptr();
    let mut yystacksize: libc::c_int = 200 as libc::c_int;
    let mut yyval: YYSTYPE = YYSTYPE {
        lst: 0 as *const List as *mut List,
    };
    let mut yylen: libc::c_int = 0;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    yyssp = yyss.offset(-(1 as libc::c_int as isize));
    yyvsp = yyvs;
    '_yynewstate: loop {
        yyssp = yyssp.offset(1);
        *yyssp = yystate as libc::c_short;
        if yyssp
            >= yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
        {
            let mut yyvs1: *mut YYSTYPE = yyvs;
            let mut yyss1: *mut libc::c_short = yyss;
            let mut size: libc::c_int = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int;
            if yystacksize >= 10000 as libc::c_int {
                yyerror(
                    b"parser stack overflow\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return 2 as libc::c_int;
            }
            yystacksize *= 2 as libc::c_int;
            if yystacksize > 10000 as libc::c_int {
                yystacksize = 10000 as libc::c_int;
            }
            let mut fresh0 = ::std::vec::from_elem(
                0,
                (yystacksize as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ) as usize,
            );
            yyss = fresh0.as_mut_ptr() as *mut libc::c_short;
            libc::memcpy(
                yyss as *mut libc::c_char as *mut libc::c_void,
                yyss1 as *mut libc::c_char as *const libc::c_void,
                (size as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ) as libc::size_t,
            );
            let mut fresh1 = ::std::vec::from_elem(
                0,
                (yystacksize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as usize,
            );
            yyvs = fresh1.as_mut_ptr() as *mut YYSTYPE;
            libc::memcpy(
                yyvs as *mut libc::c_char as *mut libc::c_void,
                yyvs1 as *mut libc::c_char as *const libc::c_void,
                (size as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyssp = yyss.offset(size as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(size as isize).offset(-(1 as libc::c_int as isize));
            if yyssp
                >= yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            {
                return 1 as libc::c_int;
            }
        }
        yyn = yypact[yystate as usize] as libc::c_int;
        if yyn == -(32768 as libc::c_int) {
            current_block = 220961348201274598;
        } else {
            if yychar == -(2 as libc::c_int) {
                yychar = yylex();
            }
            if yychar <= 0 as libc::c_int {
                yychar1 = 0 as libc::c_int;
                yychar = 0 as libc::c_int;
            } else {
                yychar1 = if yychar as libc::c_uint <= 288 as libc::c_int as libc::c_uint
                {
                    yytranslate[yychar as usize] as libc::c_int
                } else {
                    67 as libc::c_int
                };
            }
            yyn += yychar1;
            if yyn < 0 as libc::c_int || yyn > 559 as libc::c_int
                || yycheck[yyn as usize] as libc::c_int != yychar1
            {
                current_block = 220961348201274598;
            } else {
                yyn = yytable[yyn as usize] as libc::c_int;
                if yyn < 0 as libc::c_int {
                    if yyn == -(32768 as libc::c_int) {
                        current_block = 14276091566365472518;
                    } else {
                        yyn = -yyn;
                        current_block = 16944121321550454104;
                    }
                } else if yyn == 0 as libc::c_int {
                    current_block = 14276091566365472518;
                } else {
                    if yyn == 162 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    if yychar != 0 as libc::c_int {
                        yychar = -(2 as libc::c_int);
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    if yyerrstatus != 0 {
                        yyerrstatus -= 1;
                        yyerrstatus;
                    }
                    yystate = yyn;
                    continue;
                }
            }
        }
        match current_block {
            220961348201274598 => {
                yyn = yydefact[yystate as usize] as libc::c_int;
                if yyn == 0 as libc::c_int {
                    current_block = 14276091566365472518;
                } else {
                    current_block = 16944121321550454104;
                }
            }
            _ => {}
        }
        match current_block {
            14276091566365472518 => {
                if yyerrstatus == 0 {
                    yynerrs += 1;
                    yynerrs;
                    yyerror(
                        b"parse error\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                if yyerrstatus == 3 as libc::c_int {
                    if yychar == 0 as libc::c_int {
                        return 1 as libc::c_int;
                    }
                    yychar = -(2 as libc::c_int);
                }
                yyerrstatus = 3 as libc::c_int;
                loop {
                    yyn = yypact[yystate as usize] as libc::c_int;
                    if !(yyn == -(32768 as libc::c_int)) {
                        yyn += 1 as libc::c_int;
                        if !(yyn < 0 as libc::c_int || yyn > 559 as libc::c_int
                            || yycheck[yyn as usize] as libc::c_int != 1 as libc::c_int)
                        {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn < 0 as libc::c_int {
                                if !(yyn == -(32768 as libc::c_int)) {
                                    yyn = -yyn;
                                    break;
                                }
                            } else if !(yyn == 0 as libc::c_int) {
                                if yyn == 162 as libc::c_int {
                                    return 0 as libc::c_int;
                                }
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yystate = yyn;
                                continue '_yynewstate;
                            }
                        }
                    }
                    if yyssp == yyss {
                        return 1 as libc::c_int;
                    }
                    yyvsp = yyvsp.offset(-1);
                    yyvsp;
                    yyssp = yyssp.offset(-1);
                    yystate = *yyssp as libc::c_int;
                }
            }
            _ => {}
        }
        yylen = yyr2[yyn as usize] as libc::c_int;
        if yylen > 0 as libc::c_int {
            yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
        }
        match yyn {
            3 => {
                start_stmts = (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst;
            }
            4 => {
                startrules = (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst;
            }
            5 => {
                namerules = (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst;
            }
            6 => {
                define_state(
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).node,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst,
                );
            }
            7 => {
                list_append(
                    global_stmts,
                    (*yyvsp.offset(0 as libc::c_int as isize)).stmt as *mut libc::c_void,
                );
            }
            8 => {
                yyval.lst = list();
            }
            9 => {
                list_append(
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).lst,
                    cons(
                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                            as *mut libc::c_void,
                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).node
                            as *mut libc::c_void,
                    ) as *mut libc::c_void,
                );
            }
            10 => {
                yyval.lst = list();
            }
            11 => {
                list_append(
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).cons as *mut libc::c_void,
                );
            }
            12 => {
                yyval
                    .cons = cons(
                    0 as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                        as *mut libc::c_void,
                );
            }
            13 => {
                yyval
                    .cons = cons(
                    1 as libc::c_int as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                        as *mut libc::c_void,
                );
            }
            14 => {
                yyval
                    .cons = cons(
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                        as *mut libc::c_void,
                );
            }
            15 => {
                yyval
                    .cons = cons(
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                        as *mut libc::c_void,
                );
            }
            16 => {
                yyval.lst = list();
            }
            17 => {
                yyval.lst = (*yyvsp.offset(0 as libc::c_int as isize)).lst;
            }
            18 => {
                yyval.lst = list();
                list_append(
                    yyval.lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                );
            }
            19 => {
                list_append(
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                );
            }
            20 => {
                yyval.lst = list();
            }
            21 => {
                yyval.lst = (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst;
            }
            22 => {
                yyval.lst = list();
                list_append(
                    yyval.lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).cons as *mut libc::c_void,
                );
            }
            23 => {
                list_append(
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).cons as *mut libc::c_void,
                );
            }
            24 => {
                yyval
                    .cons = cons(
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            25 => {
                yyval
                    .cons = cons(
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                );
            }
            26 => {
                yyval.lst = list();
            }
            27 => {
                list_append(
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).stmt as *mut libc::c_void,
                );
            }
            28 => {
                yyval
                    .stmt = mk_stmt(
                    sRETURN,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            29 => {
                yyval
                    .stmt = mk_stmt(
                    sRETURN,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            30 => {
                yyval
                    .stmt = mk_stmt(
                    sDEFSUB,
                    (*yyvsp.offset(-(7 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    cons(
                        cons(
                            (*yyvsp.offset(-(5 as libc::c_int) as isize)).lst
                                as *mut libc::c_void,
                            (*yyvsp.offset(-(2 as libc::c_int) as isize)).lst
                                as *mut libc::c_void,
                        ) as *mut libc::c_void,
                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                            as *mut libc::c_void,
                    ) as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            31 => {
                yyval
                    .stmt = mk_stmt(
                    sBLOCK,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            32 => {
                yyval
                    .stmt = mk_stmt(
                    sIF,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).stmt as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            33 => {
                yyval
                    .stmt = mk_stmt(
                    sIF,
                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).stmt
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).stmt as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            34 => {
                yyval
                    .stmt = mk_stmt(
                    sWHILE,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).stmt as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            35 => {
                yyval
                    .stmt = mk_stmt(
                    sFOR,
                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).stmt as *mut libc::c_void,
                );
            }
            36 => {
                yyval
                    .stmt = mk_stmt(
                    sEXPR,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            37 => {
                yyval
                    .expr = mk_expr(
                    eSTRING,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            38 => {
                yyval
                    .expr = mk_expr(
                    eREGEXP,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            39 => {
                yyval
                    .expr = mk_expr(
                    eINTEGER,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            40 => {
                yyval
                    .expr = mk_expr(
                    eREAL,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            41 => {
                yyval
                    .expr = mk_expr(
                    eSYMBOL,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            42 => {
                yyval
                    .expr = mk_expr(
                    eNOT,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            43 => {
                yyval
                    .expr = mk_expr(
                    eAND,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            44 => {
                yyval
                    .expr = mk_expr(
                    eOR,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            45 => {
                yyval
                    .expr = mk_expr(
                    eFCALL,
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).lst
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            46 => {
                yyval
                    .expr = mk_expr(
                    eASSIGN,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            47 => {
                yyval
                    .expr = mk_expr(
                    eADDASSIGN,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            48 => {
                yyval
                    .expr = mk_expr(
                    eSUBASSIGN,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            49 => {
                yyval
                    .expr = mk_expr(
                    eMULASSIGN,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            50 => {
                yyval
                    .expr = mk_expr(
                    eDIVASSIGN,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            51 => {
                yyval
                    .expr = mk_expr(
                    ePOSTFIXADD,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            52 => {
                yyval
                    .expr = mk_expr(
                    ePOSTFIXSUB,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).node
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            53 => {
                yyval
                    .expr = mk_expr(
                    ePREFIXADD,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            54 => {
                yyval
                    .expr = mk_expr(
                    ePREFIXSUB,
                    (*yyvsp.offset(0 as libc::c_int as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            55 => {
                yyval
                    .expr = mk_expr(
                    eARRAYASSIGN,
                    (*yyvsp.offset(-(5 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                );
            }
            56 => {
                yyval.expr = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
            }
            57 => {
                yyval
                    .expr = mk_expr(
                    eARRAYREF,
                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            58 => {
                yyval
                    .expr = mk_expr(
                    eQUESTCOLON,
                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                );
            }
            59 => {
                yyval
                    .expr = mk_expr(
                    eMULT,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            60 => {
                yyval
                    .expr = mk_expr(
                    eDIV,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            61 => {
                yyval
                    .expr = mk_expr(
                    ePLUS,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            62 => {
                yyval
                    .expr = mk_expr(
                    eMINUS,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            63 => {
                yyval
                    .expr = mk_expr(
                    eLT,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            64 => {
                yyval
                    .expr = mk_expr(
                    eGT,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            65 => {
                yyval
                    .expr = mk_expr(
                    eEQ,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            66 => {
                yyval
                    .expr = mk_expr(
                    eNE,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            67 => {
                yyval
                    .expr = mk_expr(
                    eGE,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            68 => {
                yyval
                    .expr = mk_expr(
                    eLE,
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr
                        as *mut libc::c_void,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            69 => {
                yyval.expr = 0 as *mut Expr;
            }
            70 => {
                yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
            }
            71 => {
                yyval.lst = list();
            }
            72 => {
                yyval.lst = (*yyvsp.offset(0 as libc::c_int as isize)).lst;
            }
            73 => {
                yyval.lst = list();
                list_append(
                    yyval.lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                );
            }
            74 => {
                list_append(
                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).lst,
                    (*yyvsp.offset(0 as libc::c_int as isize)).expr as *mut libc::c_void,
                );
            }
            _ => {}
        }
        yyvsp = yyvsp.offset(-(yylen as isize));
        yyssp = yyssp.offset(-(yylen as isize));
        yyvsp = yyvsp.offset(1);
        *yyvsp = yyval;
        yyn = yyr1[yyn as usize] as libc::c_int;
        yystate = yypgoto[(yyn - 51 as libc::c_int) as usize] as libc::c_int
            + *yyssp as libc::c_int;
        if yystate >= 0 as libc::c_int && yystate <= 559 as libc::c_int
            && yycheck[yystate as usize] as libc::c_int == *yyssp as libc::c_int
        {
            yystate = yytable[yystate as usize] as libc::c_int;
        } else {
            yystate = yydefgoto[(yyn - 51 as libc::c_int) as usize] as libc::c_int;
        }
    };
}
static mut yypgoto: [libc::c_short; 16] = [
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    110 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(9 as libc::c_int) as libc::c_short,
    -(28 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(11 as libc::c_int) as libc::c_short,
    13 as libc::c_int as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
    -(32768 as libc::c_int) as libc::c_short,
];
static mut yytable: [libc::c_short; 560] = [
    22 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    104 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    104 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    155 as libc::c_int as libc::c_short,
    156 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    115 as libc::c_int as libc::c_short,
    116 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    76 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    78 as libc::c_int as libc::c_short,
    117 as libc::c_int as libc::c_short,
    118 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    70 as libc::c_int as libc::c_short,
    83 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    85 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    88 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    90 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    95 as libc::c_int as libc::c_short,
    96 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    161 as libc::c_int as libc::c_short,
    106 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    119 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    97 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    112 as libc::c_int as libc::c_short,
    113 as libc::c_int as libc::c_short,
    114 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    121 as libc::c_int as libc::c_short,
    126 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    82 as libc::c_int as libc::c_short,
    124 as libc::c_int as libc::c_short,
    125 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    139 as libc::c_int as libc::c_short,
    140 as libc::c_int as libc::c_short,
    141 as libc::c_int as libc::c_short,
    142 as libc::c_int as libc::c_short,
    127 as libc::c_int as libc::c_short,
    122 as libc::c_int as libc::c_short,
    123 as libc::c_int as libc::c_short,
    128 as libc::c_int as libc::c_short,
    129 as libc::c_int as libc::c_short,
    148 as libc::c_int as libc::c_short,
    130 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    132 as libc::c_int as libc::c_short,
    134 as libc::c_int as libc::c_short,
    136 as libc::c_int as libc::c_short,
    133 as libc::c_int as libc::c_short,
    137 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    78 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    145 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    143 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    153 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    159 as libc::c_int as libc::c_short,
    154 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    160 as libc::c_int as libc::c_short,
    144 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    158 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    80 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    103 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    149 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    150 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    151 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    152 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    157 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    75 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    135 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
];
static mut yycheck: [libc::c_short; 560] = [
    1 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    129 as libc::c_int as libc::c_short,
    130 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    132 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    138 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    126 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    135 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    134 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    154 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    156 as libc::c_int as libc::c_short,
    135 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    153 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    49 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    45 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    47 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    49 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    21 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    49 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    43 as libc::c_int as libc::c_short,
];
