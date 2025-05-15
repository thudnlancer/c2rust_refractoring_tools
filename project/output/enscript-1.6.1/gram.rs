use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut defs_file: *mut i8;
    static mut linenum: u32;
    static mut global_stmts: *mut List;
    static mut start_stmts: *mut List;
    static mut startrules: *mut List;
    static mut namerules: *mut List;
    fn yylex() -> i32;
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
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut u8,
    pub allocated: u64,
    pub used: u64,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut i8,
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
pub type regoff_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: u32,
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
    nVOID,
    nSTRING,
    nREGEXP,
    nINTEGER,
    nREAL,
    nSYMBOL,
    nARRAY,
}
impl NodeType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            NodeType::nVOID => 0,
            NodeType::nSTRING => 1,
            NodeType::nREGEXP => 2,
            NodeType::nINTEGER => 3,
            NodeType::nREAL => 4,
            NodeType::nSYMBOL => 5,
            NodeType::nARRAY => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> NodeType {
        match value {
            0 => NodeType::nVOID,
            1 => NodeType::nSTRING,
            2 => NodeType::nREGEXP,
            3 => NodeType::nINTEGER,
            4 => NodeType::nREAL,
            5 => NodeType::nSYMBOL,
            6 => NodeType::nARRAY,
            _ => panic!("Invalid value for NodeType: {}", value),
        }
    }
}
impl AddAssign<u32> for NodeType {
    fn add_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for NodeType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for NodeType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for NodeType {
    fn div_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for NodeType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = NodeType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for NodeType {
    type Output = NodeType;
    fn add(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for NodeType {
    type Output = NodeType;
    fn sub(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for NodeType {
    type Output = NodeType;
    fn mul(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for NodeType {
    type Output = NodeType;
    fn div(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for NodeType {
    type Output = NodeType;
    fn rem(self, rhs: u32) -> NodeType {
        NodeType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub type_0: NodeType,
    pub refcount: u32,
    pub linenum: u32,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: C2RustUnnamed_2,
    pub re: C2RustUnnamed_1,
    pub integer: i32,
    pub real: libc::c_double,
    pub sym: *mut i8,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub array: *mut *mut node_st,
    pub len: u32,
    pub allocated: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: *mut i8,
    pub len: u32,
    pub flags: u32,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: *mut i8,
    pub len: u32,
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
    eSTRING,
    eREGEXP,
    eINTEGER,
    eREAL,
    eSYMBOL,
    eNOT,
    eAND,
    eOR,
    eFCALL,
    eASSIGN,
    eADDASSIGN,
    eSUBASSIGN,
    eMULASSIGN,
    eDIVASSIGN,
    ePOSTFIXADD,
    ePOSTFIXSUB,
    ePREFIXADD,
    ePREFIXSUB,
    eARRAYASSIGN,
    eARRAYREF,
    eQUESTCOLON,
    eMULT,
    eDIV,
    ePLUS,
    eMINUS,
    eLT,
    eGT,
    eEQ,
    eNE,
    eGE,
    eLE,
}
impl ExprType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ExprType::eSTRING => 0,
            ExprType::eREGEXP => 1,
            ExprType::eINTEGER => 2,
            ExprType::eREAL => 3,
            ExprType::eSYMBOL => 4,
            ExprType::eNOT => 5,
            ExprType::eAND => 6,
            ExprType::eOR => 7,
            ExprType::eFCALL => 8,
            ExprType::eASSIGN => 9,
            ExprType::eADDASSIGN => 10,
            ExprType::eSUBASSIGN => 11,
            ExprType::eMULASSIGN => 12,
            ExprType::eDIVASSIGN => 13,
            ExprType::ePOSTFIXADD => 14,
            ExprType::ePOSTFIXSUB => 15,
            ExprType::ePREFIXADD => 16,
            ExprType::ePREFIXSUB => 17,
            ExprType::eARRAYASSIGN => 18,
            ExprType::eARRAYREF => 19,
            ExprType::eQUESTCOLON => 20,
            ExprType::eMULT => 21,
            ExprType::eDIV => 22,
            ExprType::ePLUS => 23,
            ExprType::eMINUS => 24,
            ExprType::eLT => 25,
            ExprType::eGT => 26,
            ExprType::eEQ => 27,
            ExprType::eNE => 28,
            ExprType::eGE => 29,
            ExprType::eLE => 30,
        }
    }
    fn from_libc_c_uint(value: u32) -> ExprType {
        match value {
            0 => ExprType::eSTRING,
            1 => ExprType::eREGEXP,
            2 => ExprType::eINTEGER,
            3 => ExprType::eREAL,
            4 => ExprType::eSYMBOL,
            5 => ExprType::eNOT,
            6 => ExprType::eAND,
            7 => ExprType::eOR,
            8 => ExprType::eFCALL,
            9 => ExprType::eASSIGN,
            10 => ExprType::eADDASSIGN,
            11 => ExprType::eSUBASSIGN,
            12 => ExprType::eMULASSIGN,
            13 => ExprType::eDIVASSIGN,
            14 => ExprType::ePOSTFIXADD,
            15 => ExprType::ePOSTFIXSUB,
            16 => ExprType::ePREFIXADD,
            17 => ExprType::ePREFIXSUB,
            18 => ExprType::eARRAYASSIGN,
            19 => ExprType::eARRAYREF,
            20 => ExprType::eQUESTCOLON,
            21 => ExprType::eMULT,
            22 => ExprType::eDIV,
            23 => ExprType::ePLUS,
            24 => ExprType::eMINUS,
            25 => ExprType::eLT,
            26 => ExprType::eGT,
            27 => ExprType::eEQ,
            28 => ExprType::eNE,
            29 => ExprType::eGE,
            30 => ExprType::eLE,
            _ => panic!("Invalid value for ExprType: {}", value),
        }
    }
}
impl AddAssign<u32> for ExprType {
    fn add_assign(&mut self, rhs: u32) {
        *self = ExprType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ExprType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ExprType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ExprType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ExprType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ExprType {
    fn div_assign(&mut self, rhs: u32) {
        *self = ExprType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ExprType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ExprType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ExprType {
    type Output = ExprType;
    fn add(self, rhs: u32) -> ExprType {
        ExprType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ExprType {
    type Output = ExprType;
    fn sub(self, rhs: u32) -> ExprType {
        ExprType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ExprType {
    type Output = ExprType;
    fn mul(self, rhs: u32) -> ExprType {
        ExprType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ExprType {
    type Output = ExprType;
    fn div(self, rhs: u32) -> ExprType {
        ExprType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ExprType {
    type Output = ExprType;
    fn rem(self, rhs: u32) -> ExprType {
        ExprType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expr_st {
    pub type_0: ExprType,
    pub linenum: u32,
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
    sRETURN,
    sDEFSUB,
    sBLOCK,
    sIF,
    sEXPR,
    sWHILE,
    sFOR,
}
impl StmtType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            StmtType::sRETURN => 0,
            StmtType::sDEFSUB => 1,
            StmtType::sBLOCK => 2,
            StmtType::sIF => 3,
            StmtType::sEXPR => 4,
            StmtType::sWHILE => 5,
            StmtType::sFOR => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> StmtType {
        match value {
            0 => StmtType::sRETURN,
            1 => StmtType::sDEFSUB,
            2 => StmtType::sBLOCK,
            3 => StmtType::sIF,
            4 => StmtType::sEXPR,
            5 => StmtType::sWHILE,
            6 => StmtType::sFOR,
            _ => panic!("Invalid value for StmtType: {}", value),
        }
    }
}
impl AddAssign<u32> for StmtType {
    fn add_assign(&mut self, rhs: u32) {
        *self = StmtType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for StmtType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = StmtType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for StmtType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = StmtType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for StmtType {
    fn div_assign(&mut self, rhs: u32) {
        *self = StmtType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for StmtType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = StmtType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for StmtType {
    type Output = StmtType;
    fn add(self, rhs: u32) -> StmtType {
        StmtType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for StmtType {
    type Output = StmtType;
    fn sub(self, rhs: u32) -> StmtType {
        StmtType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for StmtType {
    type Output = StmtType;
    fn mul(self, rhs: u32) -> StmtType {
        StmtType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for StmtType {
    type Output = StmtType;
    fn div(self, rhs: u32) -> StmtType {
        StmtType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for StmtType {
    type Output = StmtType;
    fn rem(self, rhs: u32) -> StmtType {
        StmtType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_st {
    pub type_0: StmtType,
    pub linenum: u32,
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
static mut yytranslate: [i8; 289] = [
    0 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    40 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    49 as i32 as i8,
    50 as i32 as i8,
    38 as i32 as i8,
    36 as i32 as i8,
    48 as i32 as i8,
    37 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    27 as i32 as i8,
    47 as i32 as i8,
    32 as i32 as i8,
    21 as i32 as i8,
    33 as i32 as i8,
    26 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    43 as i32 as i8,
    2 as i32 as i8,
    44 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    45 as i32 as i8,
    2 as i32 as i8,
    46 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    2 as i32 as i8,
    1 as i32 as i8,
    2 as i32 as i8,
    3 as i32 as i8,
    4 as i32 as i8,
    5 as i32 as i8,
    6 as i32 as i8,
    7 as i32 as i8,
    8 as i32 as i8,
    9 as i32 as i8,
    10 as i32 as i8,
    11 as i32 as i8,
    12 as i32 as i8,
    13 as i32 as i8,
    14 as i32 as i8,
    15 as i32 as i8,
    16 as i32 as i8,
    17 as i32 as i8,
    18 as i32 as i8,
    19 as i32 as i8,
    20 as i32 as i8,
    22 as i32 as i8,
    23 as i32 as i8,
    24 as i32 as i8,
    25 as i32 as i8,
    28 as i32 as i8,
    29 as i32 as i8,
    30 as i32 as i8,
    31 as i32 as i8,
    34 as i32 as i8,
    35 as i32 as i8,
    39 as i32 as i8,
    41 as i32 as i8,
    42 as i32 as i8,
];
#[no_mangle]
pub static mut yychar: i32 = 0;
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    lst: 0 as *const List as *mut List,
};
#[no_mangle]
pub static mut yynerrs: i32 = 0;
static mut yyr1: [libc::c_short; 75] = [
    0 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
];
static mut yyr2: [libc::c_short; 75] = [
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
];
static mut yydefact: [libc::c_short; 163] = [
    1 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    71 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    73 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    72 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    70 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    44 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    67 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    74 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    11 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    23 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
];
static mut yydefgoto: [libc::c_short; 16] = [
    1 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    73 as i32 as libc::c_short,
    102 as i32 as libc::c_short,
    120 as i32 as libc::c_short,
    100 as i32 as libc::c_short,
    101 as i32 as libc::c_short,
    138 as i32 as libc::c_short,
    146 as i32 as libc::c_short,
    147 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    81 as i32 as libc::c_short,
    23 as i32 as libc::c_short,
    79 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
];
static mut yypact: [libc::c_short; 163] = [
    -(32768 as i32) as libc::c_short,
    51 as i32 as libc::c_short,
    312 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    1 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    -(33 as i32) as libc::c_short,
    -(22 as i32) as libc::c_short,
    -(21 as i32) as libc::c_short,
    281 as i32 as libc::c_short,
    -(24 as i32) as libc::c_short,
    -(23 as i32) as libc::c_short,
    -(16 as i32) as libc::c_short,
    303 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    303 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    380 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    303 as i32 as libc::c_short,
    -(14 as i32) as libc::c_short,
    20 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    400 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    115 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    475 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    134 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    334 as i32 as libc::c_short,
    357 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    459 as i32 as libc::c_short,
    490 as i32 as libc::c_short,
    504 as i32 as libc::c_short,
    516 as i32 as libc::c_short,
    516 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    -(36 as i32) as libc::c_short,
    -(36 as i32) as libc::c_short,
    10 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    440 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    303 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    38 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    91 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    275 as i32 as libc::c_short,
    275 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    74 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    105 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
    67 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    69 as i32 as libc::c_short,
    97 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    420 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    99 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    275 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    475 as i32 as libc::c_short,
    126 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    162 as i32 as libc::c_short,
    181 as i32 as libc::c_short,
    209 as i32 as libc::c_short,
    228 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    86 as i32 as libc::c_short,
    123 as i32 as libc::c_short,
    -(29 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    256 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    275 as i32 as libc::c_short,
    303 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    126 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    475 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    145 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
];
#[no_mangle]
pub unsafe extern "C" fn yyerror(mut msg: *mut i8) {
    fprintf(stderr, b"%s:%d: %s\n\0" as *const u8 as *const i8, defs_file, linenum, msg);
}
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> i32 {
    let mut current_block: u64;
    let mut yystate: i32 = 0;
    let mut yyn: i32 = 0;
    let mut yyssp: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyerrstatus: i32 = 0;
    let mut yychar1: i32 = 0 as i32;
    let mut yyssa: [libc::c_short; 200] = [0; 200];
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        lst: 0 as *const List as *mut List,
    }; 200];
    let mut yyss: *mut libc::c_short = yyssa.as_mut_ptr();
    let mut yyvs: *mut YYSTYPE = yyvsa.as_mut_ptr();
    let mut yystacksize: i32 = 200 as i32;
    let mut yyval: YYSTYPE = YYSTYPE {
        lst: 0 as *const List as *mut List,
    };
    let mut yylen: i32 = 0;
    yystate = 0 as i32;
    yyerrstatus = 0 as i32;
    yynerrs = 0 as i32;
    yychar = -(2 as i32);
    yyssp = yyss.offset(-(1 as i32 as isize));
    yyvsp = yyvs;
    '_yynewstate: loop {
        yyssp = yyssp.offset(1);
        *yyssp = yystate as libc::c_short;
        if yyssp >= yyss.offset(yystacksize as isize).offset(-(1 as i32 as isize)) {
            let mut yyvs1: *mut YYSTYPE = yyvs;
            let mut yyss1: *mut libc::c_short = yyss;
            let mut size: i32 = (yyssp.offset_from(yyss) as i64 + 1 as i32 as i64)
                as i32;
            if yystacksize >= 10000 as i32 {
                yyerror(b"parser stack overflow\0" as *const u8 as *const i8 as *mut i8);
                return 2 as i32;
            }
            yystacksize *= 2 as i32;
            if yystacksize > 10000 as i32 {
                yystacksize = 10000 as i32;
            }
            let mut fresh0 = ::std::vec::from_elem(
                0,
                (yystacksize as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                    as usize,
            );
            yyss = fresh0.as_mut_ptr() as *mut libc::c_short;
            libc::memcpy(
                yyss as *mut i8 as *mut libc::c_void,
                yyss1 as *mut i8 as *const libc::c_void,
                (size as u64)
                    .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                    as libc::size_t,
            );
            let mut fresh1 = ::std::vec::from_elem(
                0,
                (yystacksize as u64)
                    .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as u64) as usize,
            );
            yyvs = fresh1.as_mut_ptr() as *mut YYSTYPE;
            libc::memcpy(
                yyvs as *mut i8 as *mut libc::c_void,
                yyvs1 as *mut i8 as *const libc::c_void,
                (size as u64).wrapping_mul(::core::mem::size_of::<YYSTYPE>() as u64)
                    as libc::size_t,
            );
            yyssp = yyss.offset(size as isize).offset(-(1 as i32 as isize));
            yyvsp = yyvs.offset(size as isize).offset(-(1 as i32 as isize));
            if yyssp >= yyss.offset(yystacksize as isize).offset(-(1 as i32 as isize)) {
                return 1 as i32;
            }
        }
        yyn = yypact[yystate as usize] as i32;
        if yyn == -(32768 as i32) {
            current_block = 16435127319589982136;
        } else {
            if yychar == -(2 as i32) {
                yychar = yylex();
            }
            if yychar <= 0 as i32 {
                yychar1 = 0 as i32;
                yychar = 0 as i32;
            } else {
                yychar1 = if yychar as u32 <= 288 as i32 as u32 {
                    yytranslate[yychar as usize] as i32
                } else {
                    67 as i32
                };
            }
            yyn += yychar1;
            if yyn < 0 as i32 || yyn > 559 as i32
                || yycheck[yyn as usize] as i32 != yychar1
            {
                current_block = 16435127319589982136;
            } else {
                yyn = yytable[yyn as usize] as i32;
                if yyn < 0 as i32 {
                    if yyn == -(32768 as i32) {
                        current_block = 7211306733824838851;
                    } else {
                        yyn = -yyn;
                        current_block = 15475533361646805975;
                    }
                } else if yyn == 0 as i32 {
                    current_block = 7211306733824838851;
                } else {
                    if yyn == 162 as i32 {
                        return 0 as i32;
                    }
                    if yychar != 0 as i32 {
                        yychar = -(2 as i32);
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
            16435127319589982136 => {
                yyn = yydefact[yystate as usize] as i32;
                if yyn == 0 as i32 {
                    current_block = 7211306733824838851;
                } else {
                    current_block = 15475533361646805975;
                }
            }
            _ => {}
        }
        match current_block {
            7211306733824838851 => {
                if yyerrstatus == 0 {
                    yynerrs += 1;
                    yynerrs;
                    yyerror(b"parse error\0" as *const u8 as *const i8 as *mut i8);
                }
                if yyerrstatus == 3 as i32 {
                    if yychar == 0 as i32 {
                        return 1 as i32;
                    }
                    yychar = -(2 as i32);
                }
                yyerrstatus = 3 as i32;
                loop {
                    yyn = yypact[yystate as usize] as i32;
                    if !(yyn == -(32768 as i32)) {
                        yyn += 1 as i32;
                        if !(yyn < 0 as i32 || yyn > 559 as i32
                            || yycheck[yyn as usize] as i32 != 1 as i32)
                        {
                            yyn = yytable[yyn as usize] as i32;
                            if yyn < 0 as i32 {
                                if !(yyn == -(32768 as i32)) {
                                    yyn = -yyn;
                                    break;
                                }
                            } else if !(yyn == 0 as i32) {
                                if yyn == 162 as i32 {
                                    return 0 as i32;
                                }
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yystate = yyn;
                                continue '_yynewstate;
                            }
                        }
                    }
                    if yyssp == yyss {
                        return 1 as i32;
                    }
                    yyvsp = yyvsp.offset(-1);
                    yyvsp;
                    yyssp = yyssp.offset(-1);
                    yystate = *yyssp as i32;
                }
            }
            _ => {}
        }
        yylen = yyr2[yyn as usize] as i32;
        if yylen > 0 as i32 {
            yyval = *yyvsp.offset((1 as i32 - yylen) as isize);
        }
        match yyn {
            3 => {
                start_stmts = (*yyvsp.offset(-(1 as i32) as isize)).lst;
            }
            4 => {
                startrules = (*yyvsp.offset(-(1 as i32) as isize)).lst;
            }
            5 => {
                namerules = (*yyvsp.offset(-(1 as i32) as isize)).lst;
            }
            6 => {
                define_state(
                    (*yyvsp.offset(-(3 as i32) as isize)).node,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst,
                );
            }
            7 => {
                list_append(
                    global_stmts,
                    (*yyvsp.offset(0 as i32 as isize)).stmt as *mut libc::c_void,
                );
            }
            8 => {
                yyval.lst = list();
            }
            9 => {
                list_append(
                    (*yyvsp.offset(-(3 as i32) as isize)).lst,
                    cons(
                        (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                        (*yyvsp.offset(-(1 as i32) as isize)).node as *mut libc::c_void,
                    ) as *mut libc::c_void,
                );
            }
            10 => {
                yyval.lst = list();
            }
            11 => {
                list_append(
                    (*yyvsp.offset(-(1 as i32) as isize)).lst,
                    (*yyvsp.offset(0 as i32 as isize)).cons as *mut libc::c_void,
                );
            }
            12 => {
                yyval.cons = cons(
                    0 as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                );
            }
            13 => {
                yyval.cons = cons(
                    1 as i32 as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                );
            }
            14 => {
                yyval.cons = cons(
                    (*yyvsp.offset(-(3 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                );
            }
            15 => {
                yyval.cons = cons(
                    (*yyvsp.offset(-(3 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                );
            }
            16 => {
                yyval.lst = list();
            }
            17 => {
                yyval.lst = (*yyvsp.offset(0 as i32 as isize)).lst;
            }
            18 => {
                yyval.lst = list();
                list_append(
                    yyval.lst,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                );
            }
            19 => {
                list_append(
                    (*yyvsp.offset(-(2 as i32) as isize)).lst,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                );
            }
            20 => {
                yyval.lst = list();
            }
            21 => {
                yyval.lst = (*yyvsp.offset(-(1 as i32) as isize)).lst;
            }
            22 => {
                yyval.lst = list();
                list_append(
                    yyval.lst,
                    (*yyvsp.offset(0 as i32 as isize)).cons as *mut libc::c_void,
                );
            }
            23 => {
                list_append(
                    (*yyvsp.offset(-(2 as i32) as isize)).lst,
                    (*yyvsp.offset(0 as i32 as isize)).cons as *mut libc::c_void,
                );
            }
            24 => {
                yyval.cons = cons(
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            25 => {
                yyval.cons = cons(
                    (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                );
            }
            26 => {
                yyval.lst = list();
            }
            27 => {
                list_append(
                    (*yyvsp.offset(-(1 as i32) as isize)).lst,
                    (*yyvsp.offset(0 as i32 as isize)).stmt as *mut libc::c_void,
                );
            }
            28 => {
                yyval.stmt = mk_stmt(
                    StmtType::sRETURN,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            29 => {
                yyval.stmt = mk_stmt(
                    StmtType::sRETURN,
                    (*yyvsp.offset(-(1 as i32) as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            30 => {
                yyval.stmt = mk_stmt(
                    StmtType::sDEFSUB,
                    (*yyvsp.offset(-(7 as i32) as isize)).node as *mut libc::c_void,
                    cons(
                        cons(
                            (*yyvsp.offset(-(5 as i32) as isize)).lst
                                as *mut libc::c_void,
                            (*yyvsp.offset(-(2 as i32) as isize)).lst
                                as *mut libc::c_void,
                        ) as *mut libc::c_void,
                        (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                    ) as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            31 => {
                yyval.stmt = mk_stmt(
                    StmtType::sBLOCK,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            32 => {
                yyval.stmt = mk_stmt(
                    StmtType::sIF,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).stmt as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            33 => {
                yyval.stmt = mk_stmt(
                    StmtType::sIF,
                    (*yyvsp.offset(-(4 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(-(2 as i32) as isize)).stmt as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).stmt as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            34 => {
                yyval.stmt = mk_stmt(
                    StmtType::sWHILE,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).stmt as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            35 => {
                yyval.stmt = mk_stmt(
                    StmtType::sFOR,
                    (*yyvsp.offset(-(6 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(-(4 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).stmt as *mut libc::c_void,
                );
            }
            36 => {
                yyval.stmt = mk_stmt(
                    StmtType::sEXPR,
                    (*yyvsp.offset(-(1 as i32) as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            37 => {
                yyval.expr = mk_expr(
                    ExprType::eSTRING,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            38 => {
                yyval.expr = mk_expr(
                    ExprType::eREGEXP,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            39 => {
                yyval.expr = mk_expr(
                    ExprType::eINTEGER,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            40 => {
                yyval.expr = mk_expr(
                    ExprType::eREAL,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            41 => {
                yyval.expr = mk_expr(
                    ExprType::eSYMBOL,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            42 => {
                yyval.expr = mk_expr(
                    ExprType::eNOT,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            43 => {
                yyval.expr = mk_expr(
                    ExprType::eAND,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            44 => {
                yyval.expr = mk_expr(
                    ExprType::eOR,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            45 => {
                yyval.expr = mk_expr(
                    ExprType::eFCALL,
                    (*yyvsp.offset(-(3 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as i32) as isize)).lst as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            46 => {
                yyval.expr = mk_expr(
                    ExprType::eASSIGN,
                    (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            47 => {
                yyval.expr = mk_expr(
                    ExprType::eADDASSIGN,
                    (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            48 => {
                yyval.expr = mk_expr(
                    ExprType::eSUBASSIGN,
                    (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            49 => {
                yyval.expr = mk_expr(
                    ExprType::eMULASSIGN,
                    (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            50 => {
                yyval.expr = mk_expr(
                    ExprType::eDIVASSIGN,
                    (*yyvsp.offset(-(2 as i32) as isize)).node as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            51 => {
                yyval.expr = mk_expr(
                    ExprType::ePOSTFIXADD,
                    (*yyvsp.offset(-(1 as i32) as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            52 => {
                yyval.expr = mk_expr(
                    ExprType::ePOSTFIXSUB,
                    (*yyvsp.offset(-(1 as i32) as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            53 => {
                yyval.expr = mk_expr(
                    ExprType::ePREFIXADD,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            54 => {
                yyval.expr = mk_expr(
                    ExprType::ePREFIXSUB,
                    (*yyvsp.offset(0 as i32 as isize)).node as *mut libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            55 => {
                yyval.expr = mk_expr(
                    ExprType::eARRAYASSIGN,
                    (*yyvsp.offset(-(5 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(-(3 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                );
            }
            56 => {
                yyval.expr = (*yyvsp.offset(-(1 as i32) as isize)).expr;
            }
            57 => {
                yyval.expr = mk_expr(
                    ExprType::eARRAYREF,
                    (*yyvsp.offset(-(3 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(-(1 as i32) as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            58 => {
                yyval.expr = mk_expr(
                    ExprType::eQUESTCOLON,
                    (*yyvsp.offset(-(4 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                );
            }
            59 => {
                yyval.expr = mk_expr(
                    ExprType::eMULT,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            60 => {
                yyval.expr = mk_expr(
                    ExprType::eDIV,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            61 => {
                yyval.expr = mk_expr(
                    ExprType::ePLUS,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            62 => {
                yyval.expr = mk_expr(
                    ExprType::eMINUS,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            63 => {
                yyval.expr = mk_expr(
                    ExprType::eLT,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            64 => {
                yyval.expr = mk_expr(
                    ExprType::eGT,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            65 => {
                yyval.expr = mk_expr(
                    ExprType::eEQ,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            66 => {
                yyval.expr = mk_expr(
                    ExprType::eNE,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            67 => {
                yyval.expr = mk_expr(
                    ExprType::eGE,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            68 => {
                yyval.expr = mk_expr(
                    ExprType::eLE,
                    (*yyvsp.offset(-(2 as i32) as isize)).expr as *mut libc::c_void,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
            }
            69 => {
                yyval.expr = 0 as *mut Expr;
            }
            70 => {
                yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
            }
            71 => {
                yyval.lst = list();
            }
            72 => {
                yyval.lst = (*yyvsp.offset(0 as i32 as isize)).lst;
            }
            73 => {
                yyval.lst = list();
                list_append(
                    yyval.lst,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                );
            }
            74 => {
                list_append(
                    (*yyvsp.offset(-(2 as i32) as isize)).lst,
                    (*yyvsp.offset(0 as i32 as isize)).expr as *mut libc::c_void,
                );
            }
            _ => {}
        }
        yyvsp = yyvsp.offset(-(yylen as isize));
        yyssp = yyssp.offset(-(yylen as isize));
        yyvsp = yyvsp.offset(1);
        *yyvsp = yyval;
        yyn = yyr1[yyn as usize] as i32;
        yystate = yypgoto[(yyn - 51 as i32) as usize] as i32 + *yyssp as i32;
        if yystate >= 0 as i32 && yystate <= 559 as i32
            && yycheck[yystate as usize] as i32 == *yyssp as i32
        {
            yystate = yytable[yystate as usize] as i32;
        } else {
            yystate = yydefgoto[(yyn - 51 as i32) as usize] as i32;
        }
    };
}
static mut yypgoto: [libc::c_short; 16] = [
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    110 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(9 as i32) as libc::c_short,
    -(28 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(11 as i32) as libc::c_short,
    13 as i32 as libc::c_short,
    -(32768 as i32) as libc::c_short,
    -(32768 as i32) as libc::c_short,
];
static mut yytable: [libc::c_short; 560] = [
    22 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    72 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    104 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    104 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
    155 as i32 as libc::c_short,
    156 as i32 as libc::c_short,
    67 as i32 as libc::c_short,
    115 as i32 as libc::c_short,
    116 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    76 as i32 as libc::c_short,
    77 as i32 as libc::c_short,
    78 as i32 as libc::c_short,
    117 as i32 as libc::c_short,
    118 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    44 as i32 as libc::c_short,
    70 as i32 as libc::c_short,
    83 as i32 as libc::c_short,
    84 as i32 as libc::c_short,
    85 as i32 as libc::c_short,
    86 as i32 as libc::c_short,
    87 as i32 as libc::c_short,
    88 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    90 as i32 as libc::c_short,
    91 as i32 as libc::c_short,
    92 as i32 as libc::c_short,
    93 as i32 as libc::c_short,
    94 as i32 as libc::c_short,
    95 as i32 as libc::c_short,
    96 as i32 as libc::c_short,
    105 as i32 as libc::c_short,
    161 as i32 as libc::c_short,
    106 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    11 as i32 as libc::c_short,
    119 as i32 as libc::c_short,
    71 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    99 as i32 as libc::c_short,
    97 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    109 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    98 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    112 as i32 as libc::c_short,
    113 as i32 as libc::c_short,
    114 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    121 as i32 as libc::c_short,
    126 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    82 as i32 as libc::c_short,
    124 as i32 as libc::c_short,
    125 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    139 as i32 as libc::c_short,
    140 as i32 as libc::c_short,
    141 as i32 as libc::c_short,
    142 as i32 as libc::c_short,
    127 as i32 as libc::c_short,
    122 as i32 as libc::c_short,
    123 as i32 as libc::c_short,
    128 as i32 as libc::c_short,
    129 as i32 as libc::c_short,
    148 as i32 as libc::c_short,
    130 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    132 as i32 as libc::c_short,
    134 as i32 as libc::c_short,
    136 as i32 as libc::c_short,
    133 as i32 as libc::c_short,
    137 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    78 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    145 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    143 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    153 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    159 as i32 as libc::c_short,
    154 as i32 as libc::c_short,
    162 as i32 as libc::c_short,
    74 as i32 as libc::c_short,
    160 as i32 as libc::c_short,
    144 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    158 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    80 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    103 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    149 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    150 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    151 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    152 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    157 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    108 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    75 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    135 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    111 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
];
static mut yycheck: [libc::c_short; 560] = [
    1 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    11 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    98 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    109 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    129 as i32 as libc::c_short,
    130 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    132 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    108 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    138 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    126 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    135 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    134 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    154 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    156 as i32 as libc::c_short,
    135 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    153 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    45 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    49 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    45 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    47 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    49 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    21 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    23 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    40 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    49 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    44 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    26 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    -(1 as i32) as libc::c_short,
    43 as i32 as libc::c_short,
];