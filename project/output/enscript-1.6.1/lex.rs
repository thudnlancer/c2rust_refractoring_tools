use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn isatty(__fd: i32) -> i32;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn yyerror(msg: *mut i8);
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn node_alloc(type_0: NodeType) -> *mut Node;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    static mut linenum: u32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut i8) -> *mut i8;
    static mut yylval: YYSTYPE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut i8,
    pub yy_buf_pos: *mut i8,
    pub yy_buf_size: yy_size_t,
    pub yy_n_chars: i32,
    pub yy_is_our_buffer: i32,
    pub yy_is_interactive: i32,
    pub yy_at_bol: i32,
    pub yy_fill_buffer: i32,
    pub yy_buffer_status: i32,
}
pub type yy_size_t = u32;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type YY_CHAR = u8;
pub type yy_state_type = i32;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
unsafe extern "C" fn eat_comment() {
    let mut c: i32 = 0;
    loop {
        c = input();
        if !(c != -(1 as i32)) {
            break;
        }
        if c == '\n' as i32 {
            linenum = linenum.wrapping_add(1);
            linenum;
        } else {
            if !(c == '*' as i32) {
                continue;
            }
            c = input();
            if c == '/' as i32 {
                return;
            }
            if c == -(1 as i32) {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"error: EOF in comment\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                break;
            } else {
                yyunput(c, yytext);
            }
        }
    }
    yyerror(
        dcgettext(
            0 as *const i8,
            b"error: EOF in comment\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn yywrap() -> i32 {
    return 1 as i32;
}
unsafe extern "C" fn read_string(mut len_return: *mut u32) -> *mut i8 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buf2: *mut i8 = 0 as *mut i8;
    let mut buflen: i32 = 0 as i32;
    let mut bufpos: i32 = 0 as i32;
    let mut ch: i32 = 0;
    let mut done: i32 = 0 as i32;
    while done == 0 {
        ch = input();
        if ch == '\n' as i32 {
            linenum = linenum.wrapping_add(1);
            linenum;
        }
        let mut current_block_26: u64;
        match ch {
            -1 => {
                current_block_26 = 6394203536652651481;
            }
            34 => {
                done = 1 as i32;
                current_block_26 = 6417057564578538666;
            }
            92 => {
                ch = input();
                match ch {
                    110 => {
                        current_block_26 = 15050610111240922756;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    116 => {
                        current_block_26 = 11053768700258126603;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    118 => {
                        current_block_26 = 18401480588297710244;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    98 => {
                        current_block_26 = 13624143383088606119;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    114 => {
                        current_block_26 = 3142581409041571743;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    102 => {
                        current_block_26 = 2346692697102523595;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    97 => {
                        current_block_26 = 3049831026159303899;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                    -1 => {
                        current_block_26 = 6394203536652651481;
                    }
                    _ => {
                        current_block_26 = 6496932521762800249;
                        match current_block_26 {
                            6496932521762800249 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            15050610111240922756 => {
                                ch = '\n' as i32;
                            }
                            18401480588297710244 => {
                                ch = '\u{b}' as i32;
                            }
                            13624143383088606119 => {
                                ch = '\u{8}' as i32;
                            }
                            3142581409041571743 => {
                                ch = '\r' as i32;
                            }
                            2346692697102523595 => {
                                ch = '\u{c}' as i32;
                            }
                            3049831026159303899 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 11520128967991814264;
                    }
                }
            }
            _ => {
                current_block_26 = 11520128967991814264;
            }
        }
        match current_block_26 {
            11520128967991814264 => {
                if bufpos >= buflen {
                    buflen += 1024 as i32;
                    buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                        as *mut i8;
                }
                let fresh0 = bufpos;
                bufpos = bufpos + 1;
                *buf.offset(fresh0 as isize) = ch as i8;
            }
            6394203536652651481 => {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"error: EOF in string constant\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                done = 1 as i32;
            }
            _ => {}
        }
    }
    buf2 = xmalloc((bufpos + 1 as i32) as size_t) as *mut i8;
    memcpy(buf2 as *mut libc::c_void, buf as *const libc::c_void, bufpos as u64);
    *buf2.offset(bufpos as isize) = '\0' as i32 as i8;
    xfree(buf as *mut libc::c_void);
    *len_return = bufpos as u32;
    return buf2;
}
static mut yy_current_buffer: YY_BUFFER_STATE = 0 as *const yy_buffer_state
    as YY_BUFFER_STATE;
static mut yy_hold_char: i8 = 0;
static mut yy_n_chars: i32 = 0;
#[no_mangle]
pub static mut yyleng: i32 = 0;
static mut yy_c_buf_p: *mut i8 = 0 as *const i8 as *mut i8;
static mut yy_init: i32 = 1 as i32;
static mut yy_start: i32 = 0 as i32;
static mut yy_did_buffer_switch_on_eof: i32 = 0;
#[no_mangle]
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut yy_accept: [libc::c_short; 108] = [
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    2 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    23 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    11 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
];
unsafe extern "C" fn read_regexp(mut node: *mut Node) {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buf2: *mut i8 = 0 as *mut i8;
    let mut buflen: i32 = 0 as i32;
    let mut bufpos: i32 = 0 as i32;
    let mut ch: i32 = 0;
    let mut done: i32 = 0 as i32;
    let mut current_block_25: u64;
    while done == 0 {
        ch = input();
        match ch {
            -1 => {
                current_block_25 = 4217509901795835879;
            }
            47 => {
                done = 1 as i32;
                continue;
            }
            92 => {
                ch = input();
                match ch {
                    10 => {
                        current_block_25 = 5321835445146156721;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                    110 => {
                        current_block_25 = 6083747870733027739;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                    114 => {
                        current_block_25 = 709216604931675506;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                    102 => {
                        current_block_25 = 11933556806527724267;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                    116 => {
                        current_block_25 = 1732048935134695626;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                    47 | 92 => {
                        current_block_25 = 16793976072624171809;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                    -1 => {
                        current_block_25 = 4217509901795835879;
                    }
                    _ => {
                        current_block_25 = 3196496897725958382;
                        match current_block_25 {
                            3196496897725958382 => {
                                if ch == '0' as i32 {
                                    let mut i: i32 = 0;
                                    let mut val: i32 = 0 as i32;
                                    i = 0 as i32;
                                    while i < 3 as i32 {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as i32 + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            6083747870733027739 => {
                                ch = '\n' as i32;
                            }
                            709216604931675506 => {
                                ch = '\r' as i32;
                            }
                            11933556806527724267 => {
                                ch = '\u{c}' as i32;
                            }
                            1732048935134695626 => {
                                ch = '\t' as i32;
                            }
                            16793976072624171809 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 6225258385525537431;
                    }
                }
            }
            _ => {
                current_block_25 = 6225258385525537431;
            }
        }
        match current_block_25 {
            6225258385525537431 => {
                if bufpos >= buflen {
                    buflen += 1024 as i32;
                    buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                        as *mut i8;
                }
                let fresh1 = bufpos;
                bufpos = bufpos + 1;
                *buf.offset(fresh1 as isize) = ch as i8;
            }
            _ => {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"error: EOF in regular expression\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                done = 1 as i32;
            }
        }
    }
    done = 0 as i32;
    while done == 0 {
        ch = input();
        match ch {
            105 => {
                (*node).u.re.flags |= 1 as i32 as u32;
            }
            _ => {
                yyunput(ch, yytext);
                done = 1 as i32;
            }
        }
    }
    buf2 = xmalloc((bufpos + 1 as i32) as size_t) as *mut i8;
    memcpy(buf2 as *mut libc::c_void, buf as *const libc::c_void, bufpos as u64);
    *buf2.offset(bufpos as isize) = '\0' as i32 as i8;
    xfree(buf as *mut libc::c_void);
    (*node).u.re.data = buf2;
    (*node).u.re.len = bufpos as u32;
}
static mut yy_ec: [i32; 256] = [
    0 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    1 as i32,
    2 as i32,
    2 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    4 as i32,
    5 as i32,
    1 as i32,
    6 as i32,
    1 as i32,
    7 as i32,
    8 as i32,
    1 as i32,
    1 as i32,
    9 as i32,
    10 as i32,
    1 as i32,
    11 as i32,
    12 as i32,
    13 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    14 as i32,
    1 as i32,
    1 as i32,
    15 as i32,
    16 as i32,
    17 as i32,
    1 as i32,
    1 as i32,
    18 as i32,
    19 as i32,
    18 as i32,
    20 as i32,
    21 as i32,
    18 as i32,
    22 as i32,
    18 as i32,
    23 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    24 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    1 as i32,
    25 as i32,
    1 as i32,
    1 as i32,
    18 as i32,
    1 as i32,
    26 as i32,
    27 as i32,
    28 as i32,
    29 as i32,
    30 as i32,
    31 as i32,
    18 as i32,
    32 as i32,
    33 as i32,
    18 as i32,
    18 as i32,
    34 as i32,
    35 as i32,
    36 as i32,
    37 as i32,
    18 as i32,
    18 as i32,
    38 as i32,
    39 as i32,
    40 as i32,
    41 as i32,
    42 as i32,
    43 as i32,
    18 as i32,
    18 as i32,
    18 as i32,
    1 as i32,
    44 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
];
static mut yy_meta: [i32; 45] = [
    0 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    3 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    1 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    1 as i32,
];
static mut yy_base: [libc::c_short; 112] = [
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    130 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    113 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    121 as i32 as libc::c_short,
    102 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    111 as i32 as libc::c_short,
    115 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    106 as i32 as libc::c_short,
    105 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    99 as i32 as libc::c_short,
    95 as i32 as libc::c_short,
    85 as i32 as libc::c_short,
    83 as i32 as libc::c_short,
    79 as i32 as libc::c_short,
    84 as i32 as libc::c_short,
    77 as i32 as libc::c_short,
    87 as i32 as libc::c_short,
    82 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    79 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    101 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    94 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    93 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    92 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    83 as i32 as libc::c_short,
    84 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    72 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    71 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    86 as i32 as libc::c_short,
    79 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    75 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    44 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    131 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
];
static mut yy_def: [libc::c_short; 112] = [
    0 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    108 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    109 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    111 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    0 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
];
static mut yy_nxt: [libc::c_short; 176] = [
    0 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    5 as i32 as libc::c_short,
    6 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    8 as i32 as libc::c_short,
    9 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    11 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    23 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    4 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    21 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    40 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    52 as i32 as libc::c_short,
    43 as i32 as libc::c_short,
    44 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    45 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    87 as i32 as libc::c_short,
    106 as i32 as libc::c_short,
    88 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    105 as i32 as libc::c_short,
    35 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    67 as i32 as libc::c_short,
    104 as i32 as libc::c_short,
    67 as i32 as libc::c_short,
    103 as i32 as libc::c_short,
    102 as i32 as libc::c_short,
    101 as i32 as libc::c_short,
    100 as i32 as libc::c_short,
    99 as i32 as libc::c_short,
    98 as i32 as libc::c_short,
    97 as i32 as libc::c_short,
    96 as i32 as libc::c_short,
    95 as i32 as libc::c_short,
    94 as i32 as libc::c_short,
    93 as i32 as libc::c_short,
    92 as i32 as libc::c_short,
    91 as i32 as libc::c_short,
    90 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    86 as i32 as libc::c_short,
    85 as i32 as libc::c_short,
    84 as i32 as libc::c_short,
    83 as i32 as libc::c_short,
    82 as i32 as libc::c_short,
    81 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
    80 as i32 as libc::c_short,
    79 as i32 as libc::c_short,
    78 as i32 as libc::c_short,
    77 as i32 as libc::c_short,
    76 as i32 as libc::c_short,
    75 as i32 as libc::c_short,
    74 as i32 as libc::c_short,
    73 as i32 as libc::c_short,
    72 as i32 as libc::c_short,
    71 as i32 as libc::c_short,
    70 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    66 as i32 as libc::c_short,
    65 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    58 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    51 as i32 as libc::c_short,
    50 as i32 as libc::c_short,
    49 as i32 as libc::c_short,
    47 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    39 as i32 as libc::c_short,
    38 as i32 as libc::c_short,
    36 as i32 as libc::c_short,
    34 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
];
static mut yy_chk: [libc::c_short; 176] = [
    0 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    1 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    17 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    110 as i32 as libc::c_short,
    13 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    42 as i32 as libc::c_short,
    14 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    31 as i32 as libc::c_short,
    77 as i32 as libc::c_short,
    105 as i32 as libc::c_short,
    77 as i32 as libc::c_short,
    108 as i32 as libc::c_short,
    103 as i32 as libc::c_short,
    108 as i32 as libc::c_short,
    109 as i32 as libc::c_short,
    109 as i32 as libc::c_short,
    109 as i32 as libc::c_short,
    111 as i32 as libc::c_short,
    102 as i32 as libc::c_short,
    111 as i32 as libc::c_short,
    101 as i32 as libc::c_short,
    100 as i32 as libc::c_short,
    99 as i32 as libc::c_short,
    97 as i32 as libc::c_short,
    94 as i32 as libc::c_short,
    93 as i32 as libc::c_short,
    92 as i32 as libc::c_short,
    89 as i32 as libc::c_short,
    88 as i32 as libc::c_short,
    87 as i32 as libc::c_short,
    86 as i32 as libc::c_short,
    85 as i32 as libc::c_short,
    84 as i32 as libc::c_short,
    81 as i32 as libc::c_short,
    79 as i32 as libc::c_short,
    76 as i32 as libc::c_short,
    75 as i32 as libc::c_short,
    74 as i32 as libc::c_short,
    72 as i32 as libc::c_short,
    71 as i32 as libc::c_short,
    69 as i32 as libc::c_short,
    68 as i32 as libc::c_short,
    67 as i32 as libc::c_short,
    64 as i32 as libc::c_short,
    63 as i32 as libc::c_short,
    62 as i32 as libc::c_short,
    61 as i32 as libc::c_short,
    60 as i32 as libc::c_short,
    59 as i32 as libc::c_short,
    57 as i32 as libc::c_short,
    56 as i32 as libc::c_short,
    55 as i32 as libc::c_short,
    54 as i32 as libc::c_short,
    53 as i32 as libc::c_short,
    48 as i32 as libc::c_short,
    46 as i32 as libc::c_short,
    41 as i32 as libc::c_short,
    37 as i32 as libc::c_short,
    33 as i32 as libc::c_short,
    32 as i32 as libc::c_short,
    30 as i32 as libc::c_short,
    29 as i32 as libc::c_short,
    28 as i32 as libc::c_short,
    27 as i32 as libc::c_short,
    26 as i32 as libc::c_short,
    25 as i32 as libc::c_short,
    24 as i32 as libc::c_short,
    23 as i32 as libc::c_short,
    22 as i32 as libc::c_short,
    20 as i32 as libc::c_short,
    19 as i32 as libc::c_short,
    18 as i32 as libc::c_short,
    16 as i32 as libc::c_short,
    15 as i32 as libc::c_short,
    12 as i32 as libc::c_short,
    11 as i32 as libc::c_short,
    10 as i32 as libc::c_short,
    7 as i32 as libc::c_short,
    3 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
    107 as i32 as libc::c_short,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut yytext: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn yylex() -> i32 {
    let mut yy_amount_of_matched_text: i32 = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut i8 = 0 as *mut i8;
    let mut yy_bp: *mut i8 = 0 as *mut i8;
    let mut yy_act: i32 = 0;
    if yy_init != 0 {
        yy_init = 0 as i32;
        if yy_start == 0 {
            yy_start = 1 as i32;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if yy_current_buffer.is_null() {
            yy_current_buffer = yy_create_buffer(yyin, 16384 as i32);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        '_yy_match: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as u8 as u32 as usize] as YY_CHAR;
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as i32 + yy_c as i32)
                    as usize] as i32 != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as i32;
                    if yy_current_state >= 108 as i32 {
                        yy_c = yy_meta[yy_c as u32 as usize] as YY_CHAR;
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as u32)
                    .wrapping_add(yy_c as u32) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as i32 != 131 as i32) {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as i32;
                if yy_act == 0 as i32 {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as i32;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as i64 as i32;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as i8;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => {
                            eat_comment();
                            break '_yy_match;
                        }
                        2 => {
                            break '_yy_match;
                        }
                        3 => {
                            linenum = linenum.wrapping_add(1);
                            linenum;
                            break '_yy_match;
                        }
                        4 => {
                            yylval.node = node_alloc(NodeType::nSTRING);
                            (*yylval.node).u.str_0.data = read_string(
                                &mut (*yylval.node).u.str_0.len,
                            );
                            return 260 as i32;
                        }
                        5 => {
                            yylval.node = node_alloc(NodeType::nINTEGER);
                            (*yylval.node).u.integer = *yytext.offset(1 as i32 as isize)
                                as i32;
                            return 261 as i32;
                        }
                        6 => {
                            yylval.node = node_alloc(NodeType::nINTEGER);
                            match *yytext.offset(2 as i32 as isize) as i32 {
                                110 => {
                                    (*yylval.node).u.integer = '\n' as i32;
                                }
                                116 => {
                                    (*yylval.node).u.integer = '\t' as i32;
                                }
                                118 => {
                                    (*yylval.node).u.integer = '\u{b}' as i32;
                                }
                                98 => {
                                    (*yylval.node).u.integer = '\u{8}' as i32;
                                }
                                114 => {
                                    (*yylval.node).u.integer = '\r' as i32;
                                }
                                102 => {
                                    (*yylval.node).u.integer = '\u{c}' as i32;
                                }
                                97 => {
                                    (*yylval.node).u.integer = '\u{7}' as i32;
                                }
                                _ => {
                                    (*yylval.node).u.integer = *yytext.offset(2 as i32 as isize)
                                        as i32;
                                }
                            }
                            return 261 as i32;
                        }
                        7 => {
                            yylval.node = node_alloc(NodeType::nREGEXP);
                            read_regexp(yylval.node);
                            return 259 as i32;
                        }
                        8 => return 268 as i32,
                        9 => return 269 as i32,
                        10 => return 286 as i32,
                        11 => return 272 as i32,
                        12 => return 275 as i32,
                        13 => return 271 as i32,
                        14 => return 273 as i32,
                        15 => return 267 as i32,
                        16 => return 270 as i32,
                        17 => return 265 as i32,
                        18 => return 266 as i32,
                        19 => return 264 as i32,
                        20 => return 263 as i32,
                        21 => return 274 as i32,
                        22 => return 282 as i32,
                        23 => return 283 as i32,
                        24 => return 285 as i32,
                        25 => return 284 as i32,
                        26 => return 281 as i32,
                        27 => return 280 as i32,
                        28 => return 287 as i32,
                        29 => return 288 as i32,
                        30 => return 276 as i32,
                        31 => return 277 as i32,
                        32 => return 278 as i32,
                        33 => return 279 as i32,
                        34 => {
                            yylval.node = node_alloc(NodeType::nREAL);
                            (*yylval.node).u.real = atof(yytext);
                            return 262 as i32;
                        }
                        35 => {
                            yylval.node = node_alloc(NodeType::nINTEGER);
                            (*yylval.node).u.integer = atoi(yytext);
                            return 261 as i32;
                        }
                        36 => {
                            yylval.node = node_alloc(NodeType::nSYMBOL);
                            (*yylval.node).u.sym = xstrdup(yytext);
                            return 258 as i32;
                        }
                        37 => {
                            return *yytext.offset(0 as i32 as isize) as i32;
                        }
                        38 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as size_t,
                                1 as i32 as size_t,
                                yyout,
                            );
                            break '_yy_match;
                        }
                        40 => return 0 as i32,
                        39 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext) as i64
                                as i32 - 1 as i32;
                            *yy_cp = yy_hold_char;
                            if (*yy_current_buffer).yy_buffer_status == 0 as i32 {
                                yy_n_chars = (*yy_current_buffer).yy_n_chars;
                                (*yy_current_buffer).yy_input_file = yyin;
                                (*yy_current_buffer).yy_buffer_status = 1 as i32;
                            }
                            if yy_c_buf_p
                                <= &mut *((*yy_current_buffer).yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut i8
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as i32 as isize);
                                if yy_next_state != 0 {
                                    current_block = 12608488225262500095;
                                    break;
                                } else {
                                    current_block = 7293850626974290116;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as i32;
                                        if yywrap() != 0 {
                                            yy_c_buf_p = yytext.offset(0 as i32 as isize);
                                            yy_act = 39 as i32 + (yy_start - 1 as i32) / 2 as i32
                                                + 1 as i32;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                yyrestart(yyin);
                                            }
                                            break '_yy_match;
                                        }
                                    }
                                    0 => {
                                        yy_c_buf_p = yytext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as i32 as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((*yy_current_buffer).yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut i8;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as i32 as isize);
                                        continue '_yy_find_action;
                                    }
                                    _ => {
                                        break '_yy_match;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const i8,
                            );
                            break '_yy_match;
                        }
                    }
                }
                match current_block {
                    7293850626974290116 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_get_next_buffer() -> i32 {
    let mut dest: *mut i8 = (*yy_current_buffer).yy_ch_buf;
    let mut source: *mut i8 = yytext;
    let mut number_to_move: i32 = 0;
    let mut i: i32 = 0;
    let mut ret_val: i32 = 0;
    if yy_c_buf_p
        > &mut *((*yy_current_buffer).yy_ch_buf).offset((yy_n_chars + 1 as i32) as isize)
            as *mut i8
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const i8,
        );
    }
    if (*yy_current_buffer).yy_fill_buffer == 0 as i32 {
        if yy_c_buf_p.offset_from(yytext) as i64 - 0 as i32 as i64 == 1 as i32 as i64 {
            return 1 as i32
        } else {
            return 2 as i32
        }
    }
    number_to_move = yy_c_buf_p.offset_from(yytext) as i64 as i32 - 1 as i32;
    i = 0 as i32;
    while i < number_to_move {
        let fresh2 = source;
        source = source.offset(1);
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 = *fresh2;
        i += 1;
        i;
    }
    if (*yy_current_buffer).yy_buffer_status == 2 as i32 {
        yy_n_chars = 0 as i32;
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: i32 = ((*yy_current_buffer).yy_buf_size)
            .wrapping_sub(number_to_move as u32)
            .wrapping_sub(1 as i32 as u32) as i32;
        while num_to_read <= 0 as i32 {
            let mut b: YY_BUFFER_STATE = yy_current_buffer;
            let mut yy_c_buf_p_offset: i32 = yy_c_buf_p.offset_from((*b).yy_ch_buf)
                as i64 as i32;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: i32 = ((*b).yy_buf_size).wrapping_mul(2 as i32 as u32)
                    as i32;
                if new_size <= 0 as i32 {
                    (*b).yy_buf_size = ((*b).yy_buf_size as u32)
                        .wrapping_add(((*b).yy_buf_size).wrapping_div(8 as i32 as u32))
                        as yy_size_t as yy_size_t;
                } else {
                    (*b).yy_buf_size = ((*b).yy_buf_size as u32)
                        .wrapping_mul(2 as i32 as u32) as yy_size_t as yy_size_t;
                }
                (*b).yy_ch_buf = yy_flex_realloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size).wrapping_add(2 as i32 as u32),
                ) as *mut i8;
            } else {
                (*b).yy_ch_buf = 0 as *mut i8;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const i8,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut i8;
            num_to_read = ((*yy_current_buffer).yy_buf_size)
                .wrapping_sub(number_to_move as u32)
                .wrapping_sub(1 as i32 as u32) as i32;
        }
        if num_to_read > 8192 as i32 {
            num_to_read = 8192 as i32;
        }
        if (*yy_current_buffer).yy_is_interactive != 0 {
            let mut c: i32 = '*' as i32;
            let mut n: i32 = 0;
            n = 0 as i32;
            while n < num_to_read
                && {
                    c = _IO_getc(yyin);
                    c != -(1 as i32)
                } && c != '\n' as i32
            {
                *(&mut *((*yy_current_buffer).yy_ch_buf).offset(number_to_move as isize)
                    as *mut i8)
                    .offset(n as isize) = c as i8;
                n += 1;
                n;
            }
            if c == '\n' as i32 {
                let fresh4 = n;
                n = n + 1;
                *(&mut *((*yy_current_buffer).yy_ch_buf).offset(number_to_move as isize)
                    as *mut i8)
                    .offset(fresh4 as isize) = c as i8;
            }
            if c == -(1 as i32) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const i8,
                );
            }
            yy_n_chars = n;
        } else {
            yy_n_chars = fread(
                &mut *((*yy_current_buffer).yy_ch_buf).offset(number_to_move as isize)
                    as *mut i8 as *mut libc::c_void,
                1 as i32 as size_t,
                num_to_read as size_t,
                yyin,
            ) as i32;
            if yy_n_chars == 0 as i32 && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const i8,
                );
            }
        }
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as i32 {
        if number_to_move == 0 as i32 {
            ret_val = 1 as i32;
            yyrestart(yyin);
        } else {
            ret_val = 2 as i32;
            (*yy_current_buffer).yy_buffer_status = 2 as i32;
        }
    } else {
        ret_val = 0 as i32;
    }
    yy_n_chars += number_to_move;
    *((*yy_current_buffer).yy_ch_buf).offset(yy_n_chars as isize) = 0 as i32 as i8;
    *((*yy_current_buffer).yy_ch_buf).offset((yy_n_chars + 1 as i32) as isize) = 0 as i32
        as i8;
    yytext = &mut *((*yy_current_buffer).yy_ch_buf).offset(0 as i32 as isize) as *mut i8;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut i8 = 0 as *mut i8;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as i32 as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as i32 != 0 {
            yy_ec[*yy_cp as u8 as u32 as usize]
        } else {
            1 as i32
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as i32 + yy_c as i32) as usize]
            as i32 != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as i32;
            if yy_current_state >= 108 as i32 {
                yy_c = yy_meta[yy_c as u32 as usize] as YY_CHAR;
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as u32)
            .wrapping_add(yy_c as u32) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: i32 = 0;
    let mut yy_cp: *mut i8 = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as i32 as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as i32 + yy_c as i32) as usize]
        as i32 != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as i32;
        if yy_current_state >= 108 as i32 {
            yy_c = yy_meta[yy_c as u32 as usize] as YY_CHAR;
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as u32)
        .wrapping_add(yy_c as u32) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 107 as i32) as i32;
    return if yy_is_jam != 0 { 0 as i32 } else { yy_current_state };
}
unsafe extern "C" fn yyunput(mut c: i32, mut yy_bp: *mut i8) {
    let mut yy_cp: *mut i8 = yy_c_buf_p;
    *yy_cp = yy_hold_char;
    if yy_cp < ((*yy_current_buffer).yy_ch_buf).offset(2 as i32 as isize) {
        let mut number_to_move: i32 = yy_n_chars + 2 as i32;
        let mut dest: *mut i8 = &mut *((*yy_current_buffer).yy_ch_buf)
            .offset(
                ((*yy_current_buffer).yy_buf_size).wrapping_add(2 as i32 as u32) as isize,
            ) as *mut i8;
        let mut source: *mut i8 = &mut *((*yy_current_buffer).yy_ch_buf)
            .offset(number_to_move as isize) as *mut i8;
        while source > (*yy_current_buffer).yy_ch_buf {
            source = source.offset(-1);
            dest = dest.offset(-1);
            *dest = *source;
        }
        yy_cp = yy_cp.offset(dest.offset_from(source) as i64 as i32 as isize);
        yy_bp = yy_bp.offset(dest.offset_from(source) as i64 as i32 as isize);
        yy_n_chars = (*yy_current_buffer).yy_buf_size as i32;
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
        if yy_cp < ((*yy_current_buffer).yy_ch_buf).offset(2 as i32 as isize) {
            yy_fatal_error(
                b"flex scanner push-back overflow\0" as *const u8 as *const i8,
            );
        }
    }
    yy_cp = yy_cp.offset(-1);
    *yy_cp = c as i8;
    yytext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
unsafe extern "C" fn input() -> i32 {
    let mut c: i32 = 0;
    *yy_c_buf_p = yy_hold_char;
    if *yy_c_buf_p as i32 == 0 as i32 {
        if yy_c_buf_p
            < &mut *((*yy_current_buffer).yy_ch_buf).offset(yy_n_chars as isize)
                as *mut i8
        {
            *yy_c_buf_p = '\0' as i32 as i8;
        } else {
            let mut offset: i32 = yy_c_buf_p.offset_from(yytext) as i64 as i32;
            yy_c_buf_p = yy_c_buf_p.offset(1);
            yy_c_buf_p;
            let mut current_block_10: u64;
            match yy_get_next_buffer() {
                2 => {
                    yyrestart(yyin);
                    current_block_10 = 7609812572866202773;
                }
                1 => {
                    current_block_10 = 7609812572866202773;
                }
                0 => {
                    yy_c_buf_p = yytext.offset(offset as isize);
                    current_block_10 = 7746791466490516765;
                }
                _ => {
                    current_block_10 = 7746791466490516765;
                }
            }
            match current_block_10 {
                7746791466490516765 => {}
                _ => {
                    if yywrap() != 0 {
                        return -(1 as i32);
                    }
                    if yy_did_buffer_switch_on_eof == 0 {
                        yyrestart(yyin);
                    }
                    return input();
                }
            }
        }
    }
    c = *(yy_c_buf_p as *mut u8) as i32;
    *yy_c_buf_p = '\0' as i32 as i8;
    yy_c_buf_p = yy_c_buf_p.offset(1);
    yy_hold_char = *yy_c_buf_p;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if yy_current_buffer.is_null() {
        yy_current_buffer = yy_create_buffer(yyin, 16384 as i32);
    }
    yy_init_buffer(yy_current_buffer, input_file);
    yy_load_buffer_state();
}
#[no_mangle]
pub unsafe extern "C" fn yy_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    if yy_current_buffer == new_buffer {
        return;
    }
    if !yy_current_buffer.is_null() {
        *yy_c_buf_p = yy_hold_char;
        (*yy_current_buffer).yy_buf_pos = yy_c_buf_p;
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
    }
    yy_current_buffer = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (*yy_current_buffer).yy_n_chars;
    yy_c_buf_p = (*yy_current_buffer).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (*yy_current_buffer).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
#[no_mangle]
pub unsafe extern "C" fn yy_create_buffer(
    mut file: *mut FILE,
    mut size: i32,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yy_flex_alloc(::core::mem::size_of::<yy_buffer_state>() as u64 as yy_size_t)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8 as *const i8,
        );
    }
    (*b).yy_buf_size = size as yy_size_t;
    (*b).yy_ch_buf = yy_flex_alloc(((*b).yy_buf_size).wrapping_add(2 as i32 as u32))
        as *mut i8;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8 as *const i8,
        );
    }
    (*b).yy_is_our_buffer = 1 as i32;
    yy_init_buffer(b, file);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yy_delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    if b == yy_current_buffer {
        yy_current_buffer = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yy_flex_free((*b).yy_ch_buf as *mut libc::c_void);
    }
    yy_flex_free(b as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn yy_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    yy_flush_buffer(b);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as i32;
    (*b).yy_is_interactive = if !file.is_null() {
        (isatty(fileno(file)) > 0 as i32) as i32
    } else {
        0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as i32;
    *((*b).yy_ch_buf).offset(0 as i32 as isize) = 0 as i32 as i8;
    *((*b).yy_ch_buf).offset(1 as i32 as isize) = 0 as i32 as i8;
    (*b).yy_buf_pos = &mut *((*b).yy_ch_buf).offset(0 as i32 as isize) as *mut i8;
    (*b).yy_at_bol = 1 as i32;
    (*b).yy_buffer_status = 0 as i32;
    if b == yy_current_buffer {
        yy_load_buffer_state();
    }
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut i8,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as i32 as u32
        || *base.offset(size.wrapping_sub(2 as i32 as u32) as isize) as i32 != 0 as i32
        || *base.offset(size.wrapping_sub(1 as i32 as u32) as isize) as i32 != 0 as i32
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = yy_flex_alloc(::core::mem::size_of::<yy_buffer_state>() as u64 as yy_size_t)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8 as *const i8,
        );
    }
    (*b).yy_buf_size = size.wrapping_sub(2 as i32 as u32);
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as i32;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size as i32;
    (*b).yy_is_interactive = 0 as i32;
    (*b).yy_at_bol = 1 as i32;
    (*b).yy_fill_buffer = 0 as i32;
    (*b).yy_buffer_status = 0 as i32;
    yy_switch_to_buffer(b);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_string(mut yy_str: *const i8) -> YY_BUFFER_STATE {
    let mut len: i32 = 0;
    len = 0 as i32;
    while *yy_str.offset(len as isize) != 0 {
        len += 1;
        len;
    }
    return yy_scan_bytes(yy_str, len);
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_bytes(
    mut bytes: *const i8,
    mut len: i32,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut n: yy_size_t = 0;
    let mut i: i32 = 0;
    n = (len + 2 as i32) as yy_size_t;
    buf = yy_flex_alloc(n) as *mut i8;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8 as *const i8,
        );
    }
    i = 0 as i32;
    while i < len {
        *buf.offset(i as isize) = *bytes.offset(i as isize);
        i += 1;
        i;
    }
    let ref mut fresh5 = *buf.offset((len + 1 as i32) as isize);
    *fresh5 = 0 as i32 as i8;
    *buf.offset(len as isize) = *fresh5;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const i8);
    }
    (*b).yy_is_our_buffer = 1 as i32;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const i8) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, msg);
    exit(2 as i32);
}
unsafe extern "C" fn yy_flex_alloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size as u64);
}
unsafe extern "C" fn yy_flex_realloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr as *mut i8 as *mut libc::c_void, size as u64);
}
unsafe extern "C" fn yy_flex_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}