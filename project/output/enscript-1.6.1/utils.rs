use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type stringhash_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xcalloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut i8) -> *mut i8;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut i8,
        keylen: i32,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_get(
        hash: StringHashPtr,
        key: *const i8,
        keylen: i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    static mut program: *mut i8;
    static mut defs_file: *mut i8;
    static mut linenum: u32;
    static mut warning_level: WarningLevel;
    static mut ns_prims: StringHashPtr;
    static mut ns_vars: StringHashPtr;
    static mut ns_subs: StringHashPtr;
    static mut ns_states: StringHashPtr;
    static mut nvoid: *mut Node;
    static mut current_linenum: u32;
    static mut current_match: *mut re_registers;
    static mut current_match_buf: *mut i8;
    fn yyerror(msg: *mut i8);
    fn re_compile_pattern(
        pattern: *const i8,
        length: size_t,
        buffer: *mut re_pattern_buffer,
    ) -> *const i8;
    fn re_compile_fastmap(buffer: *mut re_pattern_buffer) -> i32;
}
pub type size_t = u64;
pub type __int32_t = i32;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
pub type StringHashPtr = *mut stringhash_st;
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
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub str_0: C2RustUnnamed_3,
    pub re: C2RustUnnamed_2,
    pub integer: i32,
    pub real: libc::c_double,
    pub sym: *mut i8,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub array: *mut *mut node_st,
    pub len: u32,
    pub allocated: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: *mut i8,
    pub len: u32,
    pub flags: u32,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub node: *mut Node,
    pub not: *mut expr_st,
    pub fcall: C2RustUnnamed_10,
    pub assign: C2RustUnnamed_9,
    pub arrayassign: C2RustUnnamed_8,
    pub arrayref: C2RustUnnamed_7,
    pub questcolon: C2RustUnnamed_6,
    pub op: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub left: *mut expr_st,
    pub right: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub cond: *mut expr_st,
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
    pub expr3: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub sym: *mut Node,
    pub expr: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub u: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub expr: *mut Expr,
    pub defsub: C2RustUnnamed_15,
    pub stmt_if: C2RustUnnamed_14,
    pub stmt_while: C2RustUnnamed_13,
    pub stmt_for: C2RustUnnamed_12,
    pub block: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub init: *mut Expr,
    pub cond: *mut Expr,
    pub incr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub expr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub expr: *mut Expr,
    pub then_stmt: *mut stmt_st,
    pub else_stmt: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub name: *mut Node,
    pub closure: *mut Cons,
}
pub type Stmt = stmt_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment_st {
    pub next: *mut environment_st,
    pub name: *mut i8,
    pub val: *mut Node,
}
pub type Environment = environment_st;
pub type Primitive = Option<
    unsafe extern "C" fn(*mut i8, *mut List, *mut Environment, u32) -> *mut Node,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum WarningLevel {
    WARN_LIGHT = 10,
    WARN_ALL = 100,
}
impl WarningLevel {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            WarningLevel::WARN_LIGHT => 10,
            WarningLevel::WARN_ALL => 100,
        }
    }
    fn from_libc_c_uint(value: u32) -> WarningLevel {
        match value {
            10 => WarningLevel::WARN_LIGHT,
            100 => WarningLevel::WARN_ALL,
            _ => panic!("Invalid value for WarningLevel: {}", value),
        }
    }
}
impl AddAssign<u32> for WarningLevel {
    fn add_assign(&mut self, rhs: u32) {
        *self = WarningLevel::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for WarningLevel {
    fn sub_assign(&mut self, rhs: u32) {
        *self = WarningLevel::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for WarningLevel {
    fn mul_assign(&mut self, rhs: u32) {
        *self = WarningLevel::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for WarningLevel {
    fn div_assign(&mut self, rhs: u32) {
        *self = WarningLevel::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for WarningLevel {
    fn rem_assign(&mut self, rhs: u32) {
        *self = WarningLevel::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for WarningLevel {
    type Output = WarningLevel;
    fn add(self, rhs: u32) -> WarningLevel {
        WarningLevel::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for WarningLevel {
    type Output = WarningLevel;
    fn sub(self, rhs: u32) -> WarningLevel {
        WarningLevel::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for WarningLevel {
    type Output = WarningLevel;
    fn mul(self, rhs: u32) -> WarningLevel {
        WarningLevel::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for WarningLevel {
    type Output = WarningLevel;
    fn div(self, rhs: u32) -> WarningLevel {
        WarningLevel::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for WarningLevel {
    type Output = WarningLevel;
    fn rem(self, rhs: u32) -> WarningLevel {
        WarningLevel::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut case_insensitive_translate: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn list() -> *mut List {
    return xcalloc(1 as i32 as size_t, ::core::mem::size_of::<List>() as u64)
        as *mut List;
}
#[no_mangle]
pub unsafe extern "C" fn list_prepend(
    mut list_0: *mut List,
    mut data: *mut libc::c_void,
) {
    let mut item: *mut ListItem = 0 as *mut ListItem;
    item = xmalloc(::core::mem::size_of::<ListItem>() as u64) as *mut ListItem;
    (*item).data = data;
    (*item).next = (*list_0).head;
    (*list_0).head = item;
    if ((*list_0).tail).is_null() {
        (*list_0).tail = item;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_append(
    mut list_0: *mut List,
    mut data: *mut libc::c_void,
) {
    let mut item: *mut ListItem = 0 as *mut ListItem;
    item = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<ListItem>() as u64)
        as *mut ListItem;
    (*item).data = data;
    if !((*list_0).tail).is_null() {
        (*(*list_0).tail).next = item;
    } else {
        (*list_0).head = item;
    }
    (*list_0).tail = item;
}
#[no_mangle]
pub unsafe extern "C" fn node_alloc(mut type_0: NodeType) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    n = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<Node>() as u64) as *mut Node;
    (*n).type_0 = type_0;
    (*n).refcount = 1 as i32 as u32;
    (*n).linenum = linenum;
    if type_0 as u32 == NodeType::nREGEXP as i32 as u32 {
        (*n).u.re.compiled.fastmap = xmalloc(256 as i32 as size_t) as *mut i8;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn node_copy(mut n: *mut Node) -> *mut Node {
    let mut n2: *mut Node = 0 as *mut Node;
    let mut i: i32 = 0;
    n2 = node_alloc((*n).type_0 as u32);
    (*n2).linenum = (*n).linenum;
    match (*n).type_0 as u32 {
        1 => {
            (*n2).u.str_0.len = (*n).u.str_0.len;
            (*n2).u.str_0.data = xmalloc(
                ((*n2).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
            ) as *mut i8;
            memcpy(
                (*n2).u.str_0.data as *mut libc::c_void,
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as u64,
            );
        }
        2 => {
            (*n2).u.re.data = xstrdup((*n).u.re.data);
            (*n2).u.re.len = (*n).u.re.len;
        }
        3 => {
            (*n2).u.integer = (*n).u.integer;
        }
        4 => {
            (*n2).u.real = (*n).u.real;
        }
        5 => {
            (*n2).u.sym = xstrdup((*n).u.sym);
        }
        6 => {
            (*n2).u.array.len = (*n).u.array.len;
            (*n2).u.array.allocated = ((*n2).u.array.len).wrapping_add(1 as i32 as u32);
            (*n2).u.array.array = xcalloc(
                (*n2).u.array.allocated as size_t,
                ::core::mem::size_of::<*mut Node>() as u64,
            ) as *mut *mut Node;
            i = 0 as i32;
            while (i as u32) < (*n).u.array.len {
                let ref mut fresh0 = *((*n2).u.array.array).offset(i as isize);
                *fresh0 = node_copy(*((*n).u.array.array).offset(i as isize));
                i += 1;
                i;
            }
        }
        0 | _ => {}
    }
    return n2;
}
#[no_mangle]
pub unsafe extern "C" fn node_reference(mut node: *mut Node) {
    (*node).refcount = ((*node).refcount).wrapping_add(1);
    (*node).refcount;
}
#[no_mangle]
pub unsafe extern "C" fn node_free(mut node: *mut Node) {
    let mut i: u32 = 0;
    if node.is_null() {
        return;
    }
    (*node).refcount = ((*node).refcount).wrapping_sub(1);
    if (*node).refcount > 0 as i32 as u32 {
        return;
    }
    match (*node).type_0 as u32 {
        0 => return,
        1 => {
            xfree((*node).u.str_0.data as *mut libc::c_void);
        }
        2 => {
            free((*node).u.re.data as *mut libc::c_void);
            xfree((*node).u.re.compiled.fastmap as *mut libc::c_void);
        }
        6 => {
            i = 0 as i32 as u32;
            while i < (*node).u.array.len {
                node_free(*((*node).u.array.array).offset(i as isize));
                i = i.wrapping_add(1);
                i;
            }
            xfree((*node).u.array.array as *mut libc::c_void);
        }
        3 | 4 | 5 | _ => {}
    }
    xfree(node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn enter_system_variable(mut name: *mut i8, mut value: *mut i8) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut old_val: *mut Node = 0 as *mut Node;
    n = node_alloc(NodeType::nSTRING as i32 as u32);
    (*n).u.str_0.len = strlen(value) as u32;
    (*n).u.str_0.data = xstrdup(value);
    if strhash_put(
        ns_vars,
        name,
        strlen(name) as i32,
        n as *mut libc::c_void,
        &mut old_val as *mut *mut Node as *mut *mut libc::c_void,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: out of memory\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program,
        );
        exit(1 as i32);
    }
    node_free(old_val);
}
#[no_mangle]
pub unsafe extern "C" fn compile_regexp(mut re: *mut Node) {
    let mut msg: *const i8 = 0 as *const i8;
    if case_insensitive_translate.is_null() {
        let mut i: i32 = 0;
        case_insensitive_translate = xmalloc(256 as i32 as size_t) as *mut i8;
        i = 0 as i32;
        while i < 256 as i32 {
            if *(*__ctype_b_loc()).offset(i as isize) as i32
                & C2RustUnnamed::_ISupper as i32 as libc::c_ushort as i32 != 0
            {
                *case_insensitive_translate.offset(i as isize) = ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = i;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(i);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(i as isize);
                    }
                    __res
                }) as i8;
            } else {
                *case_insensitive_translate.offset(i as isize) = i as i8;
            }
            i += 1;
            i;
        }
    }
    if (*re).u.re.flags & 1 as i32 as u32 != 0 {
        (*re).u.re.compiled.translate = case_insensitive_translate;
    }
    msg = re_compile_pattern(
        (*re).u.re.data,
        (*re).u.re.len as size_t,
        &mut (*re).u.re.compiled,
    );
    if !msg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: couldn't compile regular expression \"%s\": %s\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            defs_file,
            (*re).linenum,
            (*re).u.re.data,
            msg,
        );
        exit(1 as i32);
    }
    re_compile_fastmap(&mut (*re).u.re.compiled);
}
#[no_mangle]
pub unsafe extern "C" fn mk_stmt(
    mut type_0: StmtType,
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
    mut arg3: *mut libc::c_void,
    mut arg4: *mut libc::c_void,
) -> *mut Stmt {
    let mut stmt: *mut Stmt = 0 as *mut Stmt;
    stmt = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<Stmt>() as u64)
        as *mut Stmt;
    (*stmt).type_0 = type_0;
    (*stmt).linenum = linenum;
    match type_0 as u32 {
        4 | 0 => {
            (*stmt).u.expr = arg1 as *mut Expr;
        }
        1 => {
            (*stmt).u.defsub.name = arg1 as *mut Node;
            (*stmt).u.defsub.closure = arg2 as *mut Cons;
        }
        2 => {
            (*stmt).u.block = arg1 as *mut List;
        }
        3 => {
            (*stmt).u.stmt_if.expr = arg1 as *mut Expr;
            (*stmt).u.stmt_if.then_stmt = arg2 as *mut stmt_st;
            (*stmt).u.stmt_if.else_stmt = arg3 as *mut stmt_st;
        }
        5 => {
            (*stmt).u.stmt_while.expr = arg1 as *mut Expr;
            (*stmt).u.stmt_while.body = arg2 as *mut stmt_st;
        }
        6 => {
            (*stmt).u.stmt_for.init = arg1 as *mut Expr;
            (*stmt).u.stmt_for.cond = arg2 as *mut Expr;
            (*stmt).u.stmt_for.incr = arg3 as *mut Expr;
            (*stmt).u.stmt_for.body = arg4 as *mut stmt_st;
        }
        _ => {}
    }
    return stmt;
}
#[no_mangle]
pub unsafe extern "C" fn mk_expr(
    mut type_0: ExprType,
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
    mut arg3: *mut libc::c_void,
) -> *mut Expr {
    let mut expr: *mut Expr = 0 as *mut Expr;
    expr = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<Expr>() as u64)
        as *mut Expr;
    (*expr).type_0 = type_0;
    (*expr).linenum = linenum;
    match type_0 as u32 {
        0 | 1 | 2 | 3 | 4 => {
            (*expr).u.node = arg1 as *mut Node;
        }
        5 => {
            (*expr).u.not = arg1 as *mut expr_st;
        }
        8 => {
            (*expr).u.fcall.name = arg1 as *mut Node;
            (*expr).u.fcall.args = arg2 as *mut List;
        }
        9 | 10 | 11 | 12 | 13 => {
            (*expr).u.assign.sym = arg1 as *mut Node;
            (*expr).u.assign.expr = arg2 as *mut expr_st;
        }
        14 | 15 | 16 | 17 => {
            (*expr).u.node = arg1 as *mut Node;
        }
        18 => {
            (*expr).u.arrayassign.expr1 = arg1 as *mut expr_st;
            (*expr).u.arrayassign.expr2 = arg2 as *mut expr_st;
            (*expr).u.arrayassign.expr3 = arg3 as *mut expr_st;
        }
        19 => {
            (*expr).u.arrayref.expr1 = arg1 as *mut expr_st;
            (*expr).u.arrayref.expr2 = arg2 as *mut expr_st;
        }
        20 => {
            (*expr).u.questcolon.cond = arg1 as *mut expr_st;
            (*expr).u.questcolon.expr1 = arg2 as *mut expr_st;
            (*expr).u.questcolon.expr2 = arg3 as *mut expr_st;
        }
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 6 | 7 => {
            (*expr).u.op.left = arg1 as *mut expr_st;
            (*expr).u.op.right = arg2 as *mut expr_st;
        }
        _ => {}
    }
    return expr;
}
#[no_mangle]
pub unsafe extern "C" fn cons(
    mut car: *mut libc::c_void,
    mut cdr: *mut libc::c_void,
) -> *mut Cons {
    let mut c: *mut Cons = 0 as *mut Cons;
    c = xmalloc(::core::mem::size_of::<Cons>() as u64) as *mut Cons;
    (*c).car = car;
    (*c).cdr = cdr;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn define_state(mut sym: *mut Node, mut rules: *mut List) {
    let mut old_rules: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut msg: [i8; 512] = [0; 512];
    if strhash_put(
        ns_states,
        (*sym).u.sym,
        strlen((*sym).u.sym) as i32,
        rules as *mut libc::c_void,
        &mut old_rules,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: ouf of memory\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program,
        );
        exit(1 as i32);
    }
    if !old_rules.is_null() {
        sprintf(
            msg.as_mut_ptr(),
            dcgettext(
                0 as *const i8,
                b"warning: redefining state `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*sym).u.sym,
        );
        yyerror(msg.as_mut_ptr());
    }
}
unsafe extern "C" fn define_sub(
    mut sym: *mut Node,
    mut args_body: *mut Cons,
    mut linenum_0: u32,
) {
    let mut old_data: *mut libc::c_void = 0 as *mut libc::c_void;
    if strhash_put(
        ns_subs,
        (*sym).u.sym,
        strlen((*sym).u.sym) as i32,
        args_body as *mut libc::c_void,
        &mut old_data,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: ouf of memory\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program,
        );
        exit(1 as i32);
    }
    if !old_data.is_null()
        && warning_level as u32 >= WarningLevel::WARN_ALL as i32 as u32
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: warning: redefining subroutine `%s'\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum_0,
            (*sym).u.sym,
        );
    }
}
unsafe extern "C" fn lookup_var(
    mut env: *mut Environment,
    mut ns: StringHashPtr,
    mut sym: *mut Node,
    mut linenum_0: u32,
) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    let mut e: *mut Environment = 0 as *mut Environment;
    if *((*sym).u.sym).offset(0 as i32 as isize) as i32 == '$' as i32
        && *((*sym).u.sym).offset(1 as i32 as isize) as i32 != 0
        && *((*sym).u.sym).offset(2 as i32 as isize) as i32 == '\0' as i32
    {
        if *((*sym).u.sym).offset(1 as i32 as isize) as i32 >= '0' as i32
            && *((*sym).u.sym).offset(1 as i32 as isize) as i32 <= '9' as i32
        {
            let mut i: i32 = 0;
            let mut len: i32 = 0;
            i = *((*sym).u.sym).offset(1 as i32 as isize) as i32 - '0' as i32;
            n = node_alloc(NodeType::nSTRING as i32 as u32);
            if current_match.is_null()
                || *((*current_match).start).offset(i as isize) < 0 as i32
                || current_match_buf.is_null()
            {
                (*n).u.str_0.data = xmalloc(1 as i32 as size_t) as *mut i8;
                (*n).u.str_0.len = 0 as i32 as u32;
            } else {
                len = *((*current_match).end).offset(i as isize)
                    - *((*current_match).start).offset(i as isize);
                (*n).u.str_0.data = xmalloc((len + 1 as i32) as size_t) as *mut i8;
                memcpy(
                    (*n).u.str_0.data as *mut libc::c_void,
                    current_match_buf
                        .offset(*((*current_match).start).offset(i as isize) as isize)
                        as *const libc::c_void,
                    len as u64,
                );
                (*n).u.str_0.len = len as u32;
            }
            return n;
        }
        if *((*sym).u.sym).offset(1 as i32 as isize) as i32 == '`' as i32
            || *((*sym).u.sym).offset(1 as i32 as isize) as i32 == 'B' as i32
        {
            n = node_alloc(NodeType::nSTRING as i32 as u32);
            if current_match.is_null()
                || *((*current_match).start).offset(0 as i32 as isize) < 0 as i32
                || current_match_buf.is_null()
            {
                (*n).u.str_0.data = xmalloc(1 as i32 as size_t) as *mut i8;
                (*n).u.str_0.len = 0 as i32 as u32;
            } else {
                (*n).u.str_0.len = *((*current_match).start).offset(0 as i32 as isize)
                    as u32;
                (*n).u.str_0.data = xmalloc(
                    ((*n).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
                ) as *mut i8;
                memcpy(
                    (*n).u.str_0.data as *mut libc::c_void,
                    current_match_buf as *const libc::c_void,
                    (*n).u.str_0.len as u64,
                );
            }
            return n;
        }
        if *((*sym).u.sym).offset(1 as i32 as isize) as i32 == '.' as i32 {
            n = node_alloc(NodeType::nINTEGER as i32 as u32);
            (*n).u.integer = current_linenum as i32;
            return n;
        }
    }
    e = env;
    while !e.is_null() {
        if strcmp((*e).name, (*sym).u.sym) == 0 as i32 {
            return (*e).val;
        }
        e = (*e).next;
    }
    if strhash_get(
        ns,
        (*sym).u.sym,
        strlen((*sym).u.sym) as i32,
        &mut n as *mut *mut Node as *mut *mut libc::c_void,
    ) != 0
    {
        return n;
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const i8,
            b"%s:%d: error: undefined variable `%s'\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        defs_file,
        linenum_0,
        (*sym).u.sym,
    );
    exit(1 as i32);
}
unsafe extern "C" fn set_var(
    mut env: *mut Environment,
    mut ns: StringHashPtr,
    mut sym: *mut Node,
    mut val: *mut Node,
    mut linenum_0: u32,
) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut e: *mut Environment = 0 as *mut Environment;
    e = env;
    while !e.is_null() {
        if strcmp((*e).name, (*sym).u.sym) == 0 as i32 {
            node_free((*e).val);
            (*e).val = val;
            return;
        }
        e = (*e).next;
    }
    if strhash_put(
        ns,
        (*sym).u.sym,
        strlen((*sym).u.sym) as i32,
        val as *mut libc::c_void,
        &mut n as *mut *mut Node as *mut *mut libc::c_void,
    ) != 0
    {
        node_free(n);
        return;
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const i8,
            b"%s:%d: error: couldn't set variable `%s'\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        defs_file,
        linenum_0,
        (*sym).u.sym,
    );
    exit(1 as i32);
}
unsafe extern "C" fn calculate_binary(
    mut l: *mut Node,
    mut r: *mut Node,
    mut type_0: ExprType,
    mut linenum_0: u32,
) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    match type_0 as u32 {
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => {
            if (*l).type_0 as u32 == (*r).type_0 as u32
                && (*l).type_0 as u32 == NodeType::nINTEGER as i32 as u32
            {
                n = node_alloc(NodeType::nINTEGER as i32 as u32);
                match type_0 as u32 {
                    21 => {
                        (*n).u.integer = (*l).u.integer * (*r).u.integer;
                    }
                    22 => {
                        (*n).u.integer = (*l).u.integer / (*r).u.integer;
                    }
                    23 => {
                        (*n).u.integer = (*l).u.integer + (*r).u.integer;
                    }
                    24 => {
                        (*n).u.integer = (*l).u.integer - (*r).u.integer;
                    }
                    25 => {
                        (*n).u.integer = ((*l).u.integer < (*r).u.integer) as i32;
                    }
                    26 => {
                        (*n).u.integer = ((*l).u.integer > (*r).u.integer) as i32;
                    }
                    27 => {
                        (*n).u.integer = ((*l).u.integer == (*r).u.integer) as i32;
                    }
                    28 => {
                        (*n).u.integer = ((*l).u.integer != (*r).u.integer) as i32;
                    }
                    29 => {
                        (*n).u.integer = ((*l).u.integer >= (*r).u.integer) as i32;
                    }
                    30 => {
                        (*n).u.integer = ((*l).u.integer <= (*r).u.integer) as i32;
                    }
                    _ => {}
                }
            } else if ((*l).type_0 as u32 == NodeType::nINTEGER as i32 as u32
                || (*l).type_0 as u32 == NodeType::nREAL as i32 as u32)
                && ((*r).type_0 as u32 == NodeType::nINTEGER as i32 as u32
                    || (*r).type_0 as u32 == NodeType::nREAL as i32 as u32)
            {
                let mut dl: libc::c_double = 0.;
                let mut dr: libc::c_double = 0.;
                if (*l).type_0 as u32 == NodeType::nINTEGER as i32 as u32 {
                    dl = (*l).u.integer as libc::c_double;
                } else {
                    dl = (*l).u.real;
                }
                if (*r).type_0 as u32 == NodeType::nINTEGER as i32 as u32 {
                    dr = (*r).u.integer as libc::c_double;
                } else {
                    dr = (*r).u.real;
                }
                n = node_alloc(NodeType::nREAL as i32 as u32);
                match type_0 as u32 {
                    21 => {
                        (*n).u.real = dl * dr;
                    }
                    22 => {
                        (*n).u.real = dl / dr;
                    }
                    23 => {
                        (*n).u.real = dl + dr;
                    }
                    24 => {
                        (*n).u.real = dl - dr;
                    }
                    25 => {
                        (*n).type_0 = NodeType::nINTEGER;
                        (*n).u.integer = (dl < dr) as i32;
                    }
                    26 => {
                        (*n).type_0 = NodeType::nINTEGER;
                        (*n).u.integer = (dl > dr) as i32;
                    }
                    27 => {
                        (*n).type_0 = NodeType::nINTEGER;
                        (*n).u.integer = (dl == dr) as i32;
                    }
                    28 => {
                        (*n).type_0 = NodeType::nINTEGER;
                        (*n).u.integer = (dl != dr) as i32;
                    }
                    29 => {
                        (*n).type_0 = NodeType::nINTEGER;
                        (*n).u.integer = (dl >= dr) as i32;
                    }
                    30 => {
                        (*n).type_0 = NodeType::nINTEGER;
                        (*n).u.integer = (dl <= dr) as i32;
                    }
                    _ => {}
                }
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: expression between illegal types\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    linenum_0,
                );
                exit(1 as i32);
            }
        }
        _ => {
            abort();
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn eval_expr(
    mut expr: *mut Expr,
    mut env: *mut Environment,
) -> *mut Node {
    let mut n: *mut Node = nvoid;
    let mut n2: *mut Node = 0 as *mut Node;
    let mut l: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut c: *mut Cons = 0 as *mut Cons;
    let mut prim: Primitive = None;
    let mut return_seen: i32 = 0;
    let mut ei: *mut Environment = 0 as *mut Environment;
    let mut ei2: *mut Environment = 0 as *mut Environment;
    let mut i: i32 = 0;
    let mut sn: Node = Node {
        type_0: NodeType::nVOID,
        refcount: 0,
        linenum: 0,
        u: C2RustUnnamed_0 {
            str_0: C2RustUnnamed_3 {
                data: 0 as *mut i8,
                len: 0,
            },
        },
    };
    if expr.is_null() {
        return nvoid;
    }
    match (*expr).type_0 as u32 {
        0 | 1 | 2 | 3 => {
            node_reference((*expr).u.node);
            return (*expr).u.node;
        }
        4 => {
            n = lookup_var(env, ns_vars, (*expr).u.node, (*expr).linenum);
            node_reference(n);
            return n;
        }
        5 => {
            n = eval_expr((*expr).u.not, env);
            i = !((*n).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                || (*n).u.integer != 0 as i32) as i32;
            node_free(n);
            n = node_alloc(NodeType::nINTEGER as i32 as u32);
            (*n).u.integer = i;
            return n;
        }
        8 => {
            n = (*expr).u.fcall.name;
            if strhash_get(
                ns_subs,
                (*n).u.sym,
                strlen((*n).u.sym) as i32,
                &mut c as *mut *mut Cons as *mut *mut libc::c_void,
            ) != 0
            {
                let mut nenv: *mut Environment = 0 as *mut Environment;
                let mut i_0: *mut ListItem = 0 as *mut ListItem;
                let mut e: *mut ListItem = 0 as *mut ListItem;
                let mut stmts: *mut List = 0 as *mut List;
                let mut lst: *mut List = 0 as *mut List;
                let mut args_locals: *mut Cons = 0 as *mut Cons;
                args_locals = (*c).car as *mut Cons;
                stmts = (*c).cdr as *mut List;
                lst = (*args_locals).car as *mut List;
                i_0 = (*lst).head;
                e = (*(*expr).u.fcall.args).head;
                while !i_0.is_null() && !e.is_null() {
                    let mut sym: *mut Node = 0 as *mut Node;
                    sym = (*i_0).data as *mut Node;
                    n = eval_expr((*e).data as *mut Expr, env);
                    ei = xcalloc(
                        1 as i32 as size_t,
                        ::core::mem::size_of::<Environment>() as u64,
                    ) as *mut Environment;
                    (*ei).name = (*sym).u.sym;
                    (*ei).val = n;
                    (*ei).next = nenv;
                    nenv = ei;
                    i_0 = (*i_0).next;
                    e = (*e).next;
                }
                if !i_0.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: too few arguments for subroutine\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        program,
                    );
                    exit(1 as i32);
                }
                if !e.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: too many arguments for subroutine\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        program,
                    );
                    exit(1 as i32);
                }
                lst = (*args_locals).cdr as *mut List;
                i_0 = (*lst).head;
                while !i_0.is_null() {
                    let mut c_0: *mut Cons = 0 as *mut Cons;
                    let mut sym_0: *mut Node = 0 as *mut Node;
                    let mut init: *mut Expr = 0 as *mut Expr;
                    c_0 = (*i_0).data as *mut Cons;
                    sym_0 = (*c_0).car as *mut Node;
                    init = (*c_0).cdr as *mut Expr;
                    ei = xcalloc(
                        1 as i32 as size_t,
                        ::core::mem::size_of::<Environment>() as u64,
                    ) as *mut Environment;
                    (*ei).name = (*sym_0).u.sym;
                    if !init.is_null() {
                        (*ei).val = eval_expr(init, nenv);
                    } else {
                        (*ei).val = nvoid;
                    }
                    (*ei).next = nenv;
                    nenv = ei;
                    i_0 = (*i_0).next;
                }
                return_seen = 0 as i32;
                n = eval_statement_list((*c).cdr as *mut List, nenv, &mut return_seen);
                ei = nenv;
                while !ei.is_null() {
                    ei2 = (*ei).next;
                    node_free((*ei).val);
                    xfree(ei as *mut libc::c_void);
                    ei = ei2;
                }
                return n;
            } else if strhash_get(
                ns_prims,
                (*n).u.sym,
                strlen((*n).u.sym) as i32,
                &mut prim as *mut Primitive as *mut *mut libc::c_void,
            ) != 0
            {
                n = (Some(prim.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )((*n).u.sym, (*expr).u.fcall.args, env, (*expr).linenum);
                return n;
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: undefined procedure `%s'\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    program,
                    (*n).u.sym,
                );
                exit(1 as i32);
            }
        }
        9 => {
            n = eval_expr((*expr).u.assign.expr, env);
            set_var(env, ns_vars, (*expr).u.assign.sym, n, (*expr).linenum);
            node_reference(n);
            return n;
        }
        10 | 11 | 12 | 13 => {
            n = eval_expr((*expr).u.assign.expr, env);
            n2 = lookup_var(env, ns_vars, (*expr).u.assign.sym, (*expr).linenum);
            match (*expr).type_0 as u32 {
                10 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        ExprType::ePLUS as i32 as u32,
                        (*expr).linenum,
                    );
                }
                11 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        ExprType::eMINUS as i32 as u32,
                        (*expr).linenum,
                    );
                }
                12 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        ExprType::eMULT as i32 as u32,
                        (*expr).linenum,
                    );
                }
                13 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        ExprType::eDIV as i32 as u32,
                        (*expr).linenum,
                    );
                }
                _ => {
                    abort();
                }
            }
            set_var(env, ns_vars, (*expr).u.assign.sym, n2, (*expr).linenum);
            node_free(n);
            node_reference(n2);
            return n2;
        }
        14 | 15 => {
            sn.type_0 = NodeType::nINTEGER;
            sn.u.integer = 1 as i32;
            n2 = lookup_var(env, ns_vars, (*expr).u.node, (*expr).linenum);
            node_reference(n2);
            n = calculate_binary(
                n2,
                &mut sn,
                (if (*expr).type_0 as u32 == ExprType::ePOSTFIXADD as i32 as u32 {
                    ExprType::ePLUS as i32
                } else {
                    ExprType::eMINUS as i32
                }) as u32,
                (*expr).linenum,
            );
            set_var(env, ns_vars, (*expr).u.node, n, (*expr).linenum);
            return n2;
        }
        16 | 17 => {
            sn.type_0 = NodeType::nINTEGER;
            sn.u.integer = 1 as i32;
            n = lookup_var(env, ns_vars, (*expr).u.node, (*expr).linenum);
            n = calculate_binary(
                n,
                &mut sn,
                (if (*expr).type_0 as u32 == ExprType::ePREFIXADD as i32 as u32 {
                    ExprType::ePLUS as i32
                } else {
                    ExprType::eMINUS as i32
                }) as u32,
                (*expr).linenum,
            );
            set_var(env, ns_vars, (*expr).u.node, n, (*expr).linenum);
            node_reference(n);
            return n;
        }
        18 => {
            n = eval_expr((*expr).u.arrayassign.expr1, env);
            if (*n).type_0 as u32 != NodeType::nARRAY as i32 as u32
                && (*n).type_0 as u32 != NodeType::nSTRING as i32 as u32
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: illegal lvalue for assignment\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    (*expr).linenum,
                );
                exit(1 as i32);
            }
            n2 = eval_expr((*expr).u.arrayassign.expr2, env);
            if (*n2).type_0 as u32 != NodeType::nINTEGER as i32 as u32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: array reference index is not integer\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    (*expr).linenum,
                );
                exit(1 as i32);
            }
            if (*n2).u.integer < 0 as i32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: negative array reference index\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    (*expr).linenum,
                );
                exit(1 as i32);
            }
            if (*n).type_0 as u32 == NodeType::nARRAY as i32 as u32 {
                if (*n2).u.integer as u32 >= (*n).u.array.len {
                    if (*n2).u.integer as u32 >= (*n).u.array.allocated {
                        (*n).u.array.allocated = ((*n2).u.integer + 100 as i32) as u32;
                        (*n).u.array.array = xrealloc(
                            (*n).u.array.array as *mut libc::c_void,
                            ((*n).u.array.allocated as u64)
                                .wrapping_mul(::core::mem::size_of::<*mut Node>() as u64),
                        ) as *mut *mut Node;
                    }
                    i = (*n).u.array.len as i32;
                    while i <= (*n2).u.integer {
                        let ref mut fresh1 = *((*n).u.array.array).offset(i as isize);
                        *fresh1 = nvoid;
                        i += 1;
                        i;
                    }
                    (*n).u.array.len = ((*n2).u.integer + 1 as i32) as u32;
                }
                node_free(*((*n).u.array.array).offset((*n2).u.integer as isize));
                l = eval_expr((*expr).u.arrayassign.expr3, env);
                node_reference(l);
                let ref mut fresh2 = *((*n).u.array.array)
                    .offset((*n2).u.integer as isize);
                *fresh2 = l;
            } else {
                if (*n2).u.integer as u32 >= (*n).u.str_0.len {
                    i = (*n).u.str_0.len as i32;
                    (*n).u.str_0.len = ((*n2).u.integer + 1 as i32) as u32;
                    (*n).u.str_0.data = xrealloc(
                        (*n).u.str_0.data as *mut libc::c_void,
                        (*n).u.str_0.len as size_t,
                    ) as *mut i8;
                    while (i as u32) < (*n).u.str_0.len {
                        *((*n).u.str_0.data).offset(i as isize) = ' ' as i32 as i8;
                        i += 1;
                        i;
                    }
                }
                l = eval_expr((*expr).u.arrayassign.expr3, env);
                if (*l).type_0 as u32 != NodeType::nINTEGER as i32 as u32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s:%d: error: illegal rvalue for string assignment\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        defs_file,
                        (*expr).linenum,
                    );
                    exit(1 as i32);
                }
                *((*n).u.str_0.data).offset((*n2).u.integer as isize) = (*l).u.integer
                    as i8;
            }
            node_free(n);
            node_free(n2);
            return l;
        }
        19 => {
            n = eval_expr((*expr).u.arrayref.expr1, env);
            if (*n).type_0 as u32 != NodeType::nARRAY as i32 as u32
                && (*n).type_0 as u32 != NodeType::nSTRING as i32 as u32
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: illegal type for array reference\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    (*expr).linenum,
                );
                exit(1 as i32);
            }
            n2 = eval_expr((*expr).u.arrayref.expr2, env);
            if (*n2).type_0 as u32 != NodeType::nINTEGER as i32 as u32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: array reference index is not integer\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    (*expr).linenum,
                );
                exit(1 as i32);
            }
            if (*n2).u.integer < 0 as i32
                || (*n).type_0 as u32 == NodeType::nARRAY as i32 as u32
                    && (*n2).u.integer as u32 >= (*n).u.array.len
                || (*n).type_0 as u32 == NodeType::nSTRING as i32 as u32
                    && (*n2).u.integer as u32 >= (*n).u.str_0.len
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: error: array reference index out of rance\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    (*expr).linenum,
                );
                exit(1 as i32);
            }
            if (*n).type_0 as u32 == NodeType::nARRAY as i32 as u32 {
                l = *((*n).u.array.array).offset((*n2).u.integer as isize);
                node_reference(l);
            } else {
                l = node_alloc(NodeType::nINTEGER as i32 as u32);
                (*l).u.integer = *((*n).u.str_0.data as *mut u8)
                    .offset((*n2).u.integer as isize) as i32;
            }
            node_free(n);
            node_free(n2);
            return l;
        }
        20 => {
            n = eval_expr((*expr).u.questcolon.cond, env);
            i = ((*n).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                || (*n).u.integer != 0 as i32) as i32;
            node_free(n);
            if i != 0 {
                n = eval_expr((*expr).u.questcolon.expr1, env);
            } else {
                n = eval_expr((*expr).u.questcolon.expr2, env);
            }
            return n;
        }
        6 => {
            n = eval_expr((*expr).u.op.left, env);
            if !((*n).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                || (*n).u.integer != 0 as i32)
            {
                return n;
            }
            node_free(n);
            return eval_expr((*expr).u.op.right, env);
        }
        7 => {
            n = eval_expr((*expr).u.op.left, env);
            if (*n).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                || (*n).u.integer != 0 as i32
            {
                return n;
            }
            node_free(n);
            return eval_expr((*expr).u.op.right, env);
        }
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => {
            l = eval_expr((*expr).u.op.left, env);
            r = eval_expr((*expr).u.op.right, env);
            n = calculate_binary(l, r, (*expr).type_0 as u32, (*expr).linenum);
            node_free(l);
            node_free(r);
            return n;
        }
        _ => {}
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn eval_statement(
    mut stmt: *mut Stmt,
    mut env: *mut Environment,
    mut return_seen: *mut i32,
) -> *mut Node {
    let mut n: *mut Node = nvoid;
    let mut n2: *mut Node = 0 as *mut Node;
    let mut i: i32 = 0;
    match (*stmt).type_0 as u32 {
        0 => {
            n = eval_expr((*stmt).u.expr, env);
            *return_seen = 1 as i32;
        }
        1 => {
            define_sub((*stmt).u.defsub.name, (*stmt).u.defsub.closure, (*stmt).linenum);
        }
        2 => {
            n = eval_statement_list((*stmt).u.block, env, return_seen);
        }
        3 => {
            n = eval_expr((*stmt).u.stmt_if.expr, env);
            i = ((*n).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                || (*n).u.integer != 0 as i32) as i32;
            node_free(n);
            if i != 0 {
                n = eval_statement((*stmt).u.stmt_if.then_stmt, env, return_seen);
            } else if !((*stmt).u.stmt_if.else_stmt).is_null() {
                n = eval_statement((*stmt).u.stmt_if.else_stmt, env, return_seen);
            } else {
                n = nvoid;
            }
        }
        5 => {
            loop {
                n2 = eval_expr((*stmt).u.stmt_while.expr, env);
                i = ((*n2).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                    || (*n2).u.integer != 0 as i32) as i32;
                node_free(n2);
                if i == 0 {
                    break;
                }
                node_free(n);
                n = eval_statement((*stmt).u.stmt_while.body, env, return_seen);
                if *return_seen != 0 {
                    break;
                }
            }
        }
        6 => {
            if !((*stmt).u.stmt_for.init).is_null() {
                n2 = eval_expr((*stmt).u.stmt_for.init, env);
                node_free(n2);
            }
            loop {
                n2 = eval_expr((*stmt).u.stmt_for.cond, env);
                i = ((*n2).type_0 as u32 != NodeType::nINTEGER as i32 as u32
                    || (*n2).u.integer != 0 as i32) as i32;
                node_free(n2);
                if i == 0 {
                    break;
                }
                node_free(n);
                n = eval_statement((*stmt).u.stmt_for.body, env, return_seen);
                if *return_seen != 0 {
                    break;
                }
                if !((*stmt).u.stmt_for.incr).is_null() {
                    n2 = eval_expr((*stmt).u.stmt_for.incr, env);
                    node_free(n2);
                }
            }
        }
        4 => {
            n = eval_expr((*stmt).u.expr, env);
        }
        _ => {}
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn eval_statement_list(
    mut lst: *mut List,
    mut env: *mut Environment,
    mut return_seen: *mut i32,
) -> *mut Node {
    let mut i: *mut ListItem = 0 as *mut ListItem;
    let mut stmt: *mut Stmt = 0 as *mut Stmt;
    let mut n: *mut Node = nvoid;
    if lst.is_null() {
        return nvoid;
    }
    i = (*lst).head;
    while !i.is_null() {
        node_free(n);
        stmt = (*i).data as *mut Stmt;
        n = eval_statement(stmt, env, return_seen);
        if *return_seen != 0 {
            return n;
        }
        i = (*i).next;
    }
    return n;
}