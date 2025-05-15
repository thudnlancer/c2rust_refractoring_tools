use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type stringhash_st;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn re_set_character_syntax(ch: u8, syntax: i8);
    fn re_search(
        buffer: *mut re_pattern_buffer,
        string: *const i8,
        length: i32,
        start: i32,
        range: i32,
        regs: *mut re_registers,
    ) -> i32;
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
    static mut program: *mut i8;
    static mut ofp: *mut FILE;
    static mut defs_file: *mut i8;
    static mut ns_prims: StringHashPtr;
    static mut startrules: *mut List;
    static mut namerules: *mut List;
    static mut nvoid: *mut Node;
    static mut inbuf: *mut i8;
    static mut data_in_buffer: u32;
    static mut current_fname: *mut i8;
    static mut current_match: *mut re_registers;
    static mut current_match_buf: *mut i8;
    static mut start_state: *mut i8;
    fn node_free(node: *mut Node);
    fn node_alloc(type_0: NodeType) -> *mut Node;
    fn eval_expr(expr: *mut Expr, env: *mut Environment) -> *mut Node;
    fn execute_state(name: *mut i8) -> *mut Node;
    fn compile_regexp(regexp: *mut Node);
    fn node_copy(node: *mut Node) -> *mut Node;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub name: *mut i8,
    pub prim: Primitive,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
unsafe extern "C" fn match_arg(
    mut prim_name: *mut i8,
    mut type_0: NodeType,
    mut argp: *mut *mut ListItem,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = *argp;
    let mut n: *mut Node = 0 as *mut Node;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    n = eval_expr((*arg).data as *mut Expr, env);
    if type_0 as u32 != NodeType::nVOID as i32 as u32
        && (*n).type_0 as u32 != type_0 as u32
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: illegal argument type\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    *argp = (*arg).next;
    return n;
}
unsafe extern "C" fn prim_call(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut e: *mut Expr = 0 as *mut Expr;
    let mut cp: *mut i8 = 0 as *mut i8;
    e = (*arg).data as *mut Expr;
    if (*e).type_0 as u32 != ExprType::eSYMBOL as i32 as u32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: illegal argument type\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    cp = (*(*e).u.node).u.sym;
    arg = (*arg).next;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    return execute_state(cp);
}
unsafe extern "C" fn prim_check_namerules(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut i: *mut ListItem = 0 as *mut ListItem;
    let mut c: *mut Cons = 0 as *mut Cons;
    let mut n: *mut Node = 0 as *mut Node;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    if start_state.is_null() {
        i = (*namerules).head;
        while !i.is_null() {
            c = (*i).data as *mut Cons;
            n = (*c).car as *mut Node;
            if re_search(
                (if ((*n).u.re.compiled).fastmap_accurate() as i32 != 0 {
                    &mut (*n).u.re.compiled
                } else {
                    compile_regexp(n);
                    &mut (*n).u.re.compiled
                }),
                current_fname,
                strlen(current_fname) as i32,
                0 as i32,
                strlen(current_fname) as i32,
                0 as *mut re_registers,
            ) >= 0 as i32
            {
                n = (*c).cdr as *mut Node;
                start_state = (*n).u.sym;
                n = node_alloc(NodeType::nINTEGER);
                (*n).u.integer = 1 as i32;
                return n;
            }
            i = (*i).next;
        }
    }
    n = node_alloc(NodeType::nINTEGER);
    (*n).u.integer = 0 as i32;
    return n;
}
unsafe extern "C" fn prim_check_startrules(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut i: *mut ListItem = 0 as *mut ListItem;
    let mut c: *mut Cons = 0 as *mut Cons;
    let mut n: *mut Node = 0 as *mut Node;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    if start_state.is_null() {
        i = (*startrules).head;
        while !i.is_null() {
            c = (*i).data as *mut Cons;
            n = (*c).car as *mut Node;
            if re_search(
                (if ((*n).u.re.compiled).fastmap_accurate() as i32 != 0 {
                    &mut (*n).u.re.compiled
                } else {
                    compile_regexp(n);
                    &mut (*n).u.re.compiled
                }),
                inbuf,
                data_in_buffer as i32,
                0 as i32,
                data_in_buffer as i32,
                0 as *mut re_registers,
            ) >= 0 as i32
            {
                n = (*c).cdr as *mut Node;
                start_state = (*n).u.sym;
                n = node_alloc(NodeType::nINTEGER);
                (*n).u.integer = 1 as i32;
                return n;
            }
            i = (*i).next;
        }
    }
    n = node_alloc(NodeType::nINTEGER);
    (*n).u.integer = 0 as i32;
    return n;
}
unsafe extern "C" fn prim_concat(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut len: i32 = 0 as i32;
    let mut data: *mut i8 = 0 as *mut i8;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    while !arg.is_null() {
        n = eval_expr((*arg).data as *mut Expr, env);
        if (*n).type_0 as u32 != NodeType::nSTRING as i32 as u32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s:%d: %s: illegal argument type\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                defs_file,
                linenum,
                prim_name,
            );
            exit(1 as i32);
        }
        if (*n).u.str_0.len > 0 as i32 as u32 {
            data = xrealloc(
                data as *mut libc::c_void,
                (len as u32).wrapping_add((*n).u.str_0.len) as size_t,
            ) as *mut i8;
            memcpy(
                data.offset(len as isize) as *mut libc::c_void,
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as u64,
            );
            len = (len as u32).wrapping_add((*n).u.str_0.len) as i32 as i32;
        }
        node_free(n);
        arg = (*arg).next;
    }
    n = node_alloc(NodeType::nSTRING);
    (*n).u.str_0.data = data;
    (*n).u.str_0.len = len as u32;
    return n;
}
unsafe extern "C" fn prim_float(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut buf: [i8; 512] = [0; 512];
    n = match_arg(prim_name, NodeType::nVOID as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    r = node_alloc(NodeType::nREAL);
    match (*n).type_0 as u32 {
        0 | 2 | 5 => {
            (*r).u.real = 0.0f64;
        }
        6 => {
            (*r).u.real = (*n).u.array.len as libc::c_double;
        }
        1 => {
            if (*n).u.str_0.len as u64
                > (::core::mem::size_of::<[i8; 512]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
            {
                (*r).u.real = 0.0f64;
            } else {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    (*n).u.str_0.data as *const libc::c_void,
                    (*n).u.str_0.len as u64,
                );
                buf[(*n).u.str_0.len as usize] = '\0' as i32 as i8;
                (*r).u.real = atof(buf.as_mut_ptr());
            }
        }
        3 => {
            (*r).u.real = (*n).u.integer as libc::c_double;
        }
        4 => {
            (*r).u.real = (*n).u.real;
        }
        _ => {}
    }
    node_free(n);
    return r;
}
unsafe extern "C" fn prim_getenv(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut var: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut key: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    var = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    key = xcalloc(
        1 as i32 as size_t,
        ((*var).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
    ) as *mut i8;
    memcpy(
        key as *mut libc::c_void,
        (*var).u.str_0.data as *const libc::c_void,
        (*var).u.str_0.len as u64,
    );
    cp = getenv(key);
    node_free(var);
    xfree(key as *mut libc::c_void);
    n = node_alloc(NodeType::nSTRING);
    if cp.is_null() {
        (*n).u.str_0.data = xmalloc(1 as i32 as size_t) as *mut i8;
        (*n).u.str_0.len = 0 as i32 as u32;
    } else {
        (*n).u.str_0.data = xstrdup(cp);
        (*n).u.str_0.len = strlen(cp) as u32;
    }
    return n;
}
unsafe extern "C" fn prim_int(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut buf: [i8; 512] = [0; 512];
    n = match_arg(prim_name, NodeType::nVOID as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    r = node_alloc(NodeType::nINTEGER);
    match (*n).type_0 as u32 {
        0 | 2 | 5 => {
            (*r).u.integer = 0 as i32;
        }
        6 => {
            (*r).u.integer = (*n).u.array.len as i32;
        }
        1 => {
            if (*n).u.str_0.len as u64
                > (::core::mem::size_of::<[i8; 512]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
            {
                (*r).u.integer = 0 as i32;
            } else {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    (*n).u.str_0.data as *const libc::c_void,
                    (*n).u.str_0.len as u64,
                );
                buf[(*n).u.str_0.len as usize] = '\0' as i32 as i8;
                (*r).u.integer = atoi(buf.as_mut_ptr());
            }
        }
        3 => {
            (*r).u.integer = (*n).u.integer;
        }
        4 => {
            (*r).u.integer = (*n).u.real as i32;
        }
        _ => {}
    }
    node_free(n);
    return r;
}
unsafe extern "C" fn prim_length(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut result: i32 = 0 as i32;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    while !arg.is_null() {
        n = eval_expr((*arg).data as *mut Expr, env);
        match (*n).type_0 as u32 {
            1 => {
                result = (result as u32).wrapping_add((*n).u.str_0.len) as i32 as i32;
            }
            6 => {
                result = (result as u32).wrapping_add((*n).u.array.len) as i32 as i32;
            }
            _ => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%d: %s: illegal argument type\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    defs_file,
                    linenum,
                    prim_name,
                );
                exit(1 as i32);
            }
        }
        node_free(n);
        arg = (*arg).next;
    }
    n = node_alloc(NodeType::nINTEGER);
    (*n).u.integer = result;
    return n;
}
unsafe extern "C" fn prim_list(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut len: u32 = 0;
    let mut n: *mut Node = 0 as *mut Node;
    len = 0 as i32 as u32;
    while !arg.is_null() {
        len = len.wrapping_add(1);
        len;
        arg = (*arg).next;
    }
    arg = (*args).head;
    n = node_alloc(NodeType::nARRAY);
    (*n).u.array.array = xcalloc(
        len.wrapping_add(1 as i32 as u32) as size_t,
        ::core::mem::size_of::<*mut Node>() as u64,
    ) as *mut *mut Node;
    (*n).u.array.allocated = len.wrapping_add(1 as i32 as u32);
    (*n).u.array.len = len;
    len = 0 as i32 as u32;
    while !arg.is_null() {
        let ref mut fresh0 = *((*n).u.array.array).offset(len as isize);
        *fresh0 = eval_expr((*arg).data as *mut Expr, env);
        len = len.wrapping_add(1);
        len;
        arg = (*arg).next;
    }
    return n;
}
unsafe extern "C" fn prim_panic(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    fprintf(
        stderr,
        dcgettext(0 as *const i8, b"%s: panic: \0" as *const u8 as *const i8, 5 as i32),
        program,
    );
    ofp = stderr;
    prim_print(prim_name, args, env, linenum);
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    exit(1 as i32);
}
unsafe extern "C" fn prim_prereq(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut s: *mut Node = 0 as *mut Node;
    let mut over: [i32; 3] = [0; 3];
    let mut rver: [i32; 3] = [0; 3];
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    s = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    sscanf(
        b"1.6.1\0" as *const u8 as *const i8,
        b"%d.%d.%d\0" as *const u8 as *const i8,
        &mut *over.as_mut_ptr().offset(0 as i32 as isize) as *mut i32,
        &mut *over.as_mut_ptr().offset(1 as i32 as isize) as *mut i32,
        &mut *over.as_mut_ptr().offset(2 as i32 as isize) as *mut i32,
    );
    cp = xcalloc(
        1 as i32 as size_t,
        ((*s).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
    ) as *mut i8;
    memcpy(
        cp as *mut libc::c_void,
        (*s).u.str_0.data as *const libc::c_void,
        (*s).u.str_0.len as u64,
    );
    if sscanf(
        cp,
        b"%d.%d.%d\0" as *const u8 as *const i8,
        &mut *rver.as_mut_ptr().offset(0 as i32 as isize) as *mut i32,
        &mut *rver.as_mut_ptr().offset(1 as i32 as isize) as *mut i32,
        &mut *rver.as_mut_ptr().offset(2 as i32 as isize) as *mut i32,
    ) != 3 as i32
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: malformed version string `%s'\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
            cp,
        );
        exit(1 as i32);
    }
    i = 0 as i32;
    while i < 3 as i32 {
        if over[i as usize] > rver[i as usize] {
            break;
        }
        if over[i as usize] < rver[i as usize] {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: FATAL ERROR: States version %s or higher is required for this script\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                program,
                cp,
            );
            exit(1 as i32);
        }
        i += 1;
        i;
    }
    xfree(cp as *mut libc::c_void);
    return nvoid;
}
unsafe extern "C" fn print_node(mut n: *mut Node) {
    let mut i: u32 = 0;
    match (*n).type_0 as u32 {
        1 => {
            fwrite(
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as size_t,
                1 as i32 as size_t,
                ofp,
            );
        }
        2 => {
            fputc('/' as i32, ofp);
            fwrite(
                (*n).u.re.data as *const libc::c_void,
                (*n).u.re.len as size_t,
                1 as i32 as size_t,
                ofp,
            );
            fputc('/' as i32, ofp);
        }
        3 => {
            fprintf(ofp, b"%d\0" as *const u8 as *const i8, (*n).u.integer);
        }
        4 => {
            fprintf(ofp, b"%f\0" as *const u8 as *const i8, (*n).u.real);
        }
        5 => {
            fprintf(ofp, b"%s\0" as *const u8 as *const i8, (*n).u.sym);
        }
        6 => {
            i = 0 as i32 as u32;
            while i < (*n).u.array.len {
                print_node(*((*n).u.array.array).offset(i as isize));
                if i.wrapping_add(1 as i32 as u32) < (*n).u.array.len {
                    fprintf(ofp, b" \0" as *const u8 as *const i8);
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn prim_print(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    while !arg.is_null() {
        n = eval_expr((*arg).data as *mut Expr, env);
        print_node(n);
        node_free(n);
        arg = (*arg).next;
    }
    return nvoid;
}
unsafe extern "C" fn prim_range(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut from: *mut Node = 0 as *mut Node;
    let mut start: *mut Node = 0 as *mut Node;
    let mut end: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: i32 = 0;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    from = eval_expr((*arg).data as *mut Expr, env);
    arg = (*arg).next;
    start = match_arg(
        prim_name,
        NodeType::nINTEGER as i32 as u32,
        &mut arg,
        env,
        linenum,
    );
    end = match_arg(prim_name, NodeType::nINTEGER as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    if (*start).u.integer > (*end).u.integer {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: start offset is bigger than end offset\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    if (*from).type_0 as u32 == NodeType::nSTRING as i32 as u32 {
        if (*end).u.integer as u32 > (*from).u.str_0.len {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s:%d: %s: offset out of range\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                defs_file,
                linenum,
                prim_name,
            );
            exit(1 as i32);
        }
        n = node_alloc(NodeType::nSTRING);
        (*n).u.str_0.len = ((*end).u.integer - (*start).u.integer) as u32;
        (*n).u.str_0.data = xmalloc(
            ((*n).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
        ) as *mut i8;
        memcpy(
            (*n).u.str_0.data as *mut libc::c_void,
            ((*from).u.str_0.data).offset((*start).u.integer as isize)
                as *const libc::c_void,
            (*n).u.str_0.len as u64,
        );
    } else if (*from).type_0 as u32 == NodeType::nARRAY as i32 as u32 {
        if (*end).u.integer as u32 > (*from).u.array.len {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s:%d: %s: offset out of range\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                defs_file,
                linenum,
                prim_name,
            );
            exit(1 as i32);
        }
        n = node_alloc(NodeType::nARRAY);
        (*n).u.array.len = ((*end).u.integer - (*start).u.integer) as u32;
        (*n).u.array.allocated = ((*n).u.array.len).wrapping_add(1 as i32 as u32);
        (*n).u.array.array = xcalloc(
            (*n).u.array.allocated as size_t,
            ::core::mem::size_of::<*mut Node>() as u64,
        ) as *mut *mut Node;
        i = 0 as i32;
        while (i as u32) < (*n).u.array.len {
            let ref mut fresh1 = *((*n).u.array.array).offset(i as isize);
            *fresh1 = node_copy(
                *((*from).u.array.array).offset((i + (*start).u.integer) as isize),
            );
            i += 1;
            i;
        }
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: illegal argument\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    node_free(from);
    node_free(start);
    node_free(end);
    return n;
}
unsafe extern "C" fn prim_regexp(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    str = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    n = node_alloc(NodeType::nREGEXP);
    (*n).u.re.data = xmalloc(
        ((*str).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
    ) as *mut i8;
    (*n).u.re.len = (*str).u.str_0.len;
    memcpy(
        (*n).u.re.data as *mut libc::c_void,
        (*str).u.str_0.data as *const libc::c_void,
        (*str).u.str_0.len as u64,
    );
    *((*n).u.re.data).offset((*str).u.str_0.len as isize) = '\0' as i32 as i8;
    return n;
}
unsafe extern "C" fn prim_regexp_syntax(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut ch: *mut Node = 0 as *mut Node;
    let mut st: *mut Node = 0 as *mut Node;
    let mut syntax: i8 = 0;
    ch = match_arg(prim_name, NodeType::nINTEGER as i32 as u32, &mut arg, env, linenum);
    st = match_arg(prim_name, NodeType::nINTEGER as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    syntax = (*st).u.integer as i8;
    if syntax as i32 != 'w' as i32 && syntax as i32 != ' ' as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: illegal regexp character syntax: %c\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
            syntax as i32,
        );
        exit(1 as i32);
    }
    re_set_character_syntax((*ch).u.integer as u8, syntax);
    return nvoid;
}
unsafe extern "C" fn prim_regmatch(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut re: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    static mut matches: re_registers = {
        let mut init = re_registers {
            num_regs: 0 as i32 as u32,
            start: 0 as *const regoff_t as *mut regoff_t,
            end: 0 as *const regoff_t as *mut regoff_t,
        };
        init
    };
    static mut current_match_node: *mut Node = 0 as *const Node as *mut Node;
    let mut i: i32 = 0;
    str = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    re = match_arg(prim_name, NodeType::nREGEXP as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    i = re_search(
        if ((*re).u.re.compiled).fastmap_accurate() as i32 != 0 {
            &mut (*re).u.re.compiled
        } else {
            compile_regexp(re);
            &mut (*re).u.re.compiled
        },
        (*str).u.str_0.data,
        (*str).u.str_0.len as i32,
        0 as i32,
        (*str).u.str_0.len as i32,
        &mut matches,
    );
    if i < 0 as i32 {
        current_match = 0 as *mut re_registers;
        node_free(str);
    } else {
        node_free(current_match_node);
        current_match_node = str;
        current_match = &mut matches;
        current_match_buf = (*str).u.str_0.data;
    }
    node_free(re);
    n = node_alloc(NodeType::nINTEGER);
    (*n).u.integer = (i >= 0 as i32) as i32;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn do_regsubsts(
    mut str: *mut Node,
    mut re: *mut Node,
    mut subst: *mut Node,
    mut allp: i32,
) -> *mut Node {
    let mut i: i32 = 0;
    let mut pos: i32 = 0;
    let mut j: i32 = 0;
    static mut matches: re_registers = {
        let mut init = re_registers {
            num_regs: 0 as i32 as u32,
            start: 0 as *const regoff_t as *mut regoff_t,
            end: 0 as *const regoff_t as *mut regoff_t,
        };
        init
    };
    static mut result: *mut i8 = 0 as *const i8 as *mut i8;
    static mut result_len: u32 = 0 as i32 as u32;
    let mut result_pos: u32 = 0 as i32 as u32;
    let mut num_matches: i32 = 0 as i32;
    let mut do_expansions_in_substs: i32 = 0 as i32;
    i = 0 as i32;
    while (i as u32) < (*subst).u.str_0.len {
        if *((*subst).u.str_0.data).offset(i as isize) as i32 == '$' as i32 {
            do_expansions_in_substs = 1 as i32;
            break;
        } else {
            i += 1;
            i;
        }
    }
    pos = 0 as i32;
    loop {
        i = re_search(
            if ((*re).u.re.compiled).fastmap_accurate() as i32 != 0 {
                &mut (*re).u.re.compiled
            } else {
                compile_regexp(re);
                &mut (*re).u.re.compiled
            },
            (*str).u.str_0.data,
            (*str).u.str_0.len as i32,
            pos,
            ((*str).u.str_0.len).wrapping_sub(pos as u32) as i32,
            &mut matches,
        );
        if i < 0 as i32 {
            break;
        }
        num_matches += 1;
        num_matches;
        if result_len
            < result_pos
                .wrapping_add((*(matches.start).offset(0 as i32 as isize) - pos) as u32)
        {
            result_len = result_len
                .wrapping_add(
                    (*(matches.start).offset(0 as i32 as isize) - pos + 1024 as i32)
                        as u32,
                );
            result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                as *mut i8;
        }
        memcpy(
            result.offset(result_pos as isize) as *mut libc::c_void,
            ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
            (*(matches.start).offset(0 as i32 as isize) - pos) as u64,
        );
        result_pos = result_pos
            .wrapping_add((*(matches.start).offset(0 as i32 as isize) - pos) as u32);
        if do_expansions_in_substs == 0 {
            if result_len < result_pos.wrapping_add((*subst).u.str_0.len) {
                result_len = result_len
                    .wrapping_add(
                        ((*subst).u.str_0.len).wrapping_add(1024 as i32 as u32),
                    );
                result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                    as *mut i8;
            }
            memcpy(
                result.offset(result_pos as isize) as *mut libc::c_void,
                (*subst).u.str_0.data as *const libc::c_void,
                (*subst).u.str_0.len as u64,
            );
            result_pos = result_pos.wrapping_add((*subst).u.str_0.len);
        } else {
            i = 0 as i32;
            while (i as u32) < (*subst).u.str_0.len {
                if *((*subst).u.str_0.data).offset(i as isize) as i32 == '$' as i32
                    && ((i + 1 as i32) as u32) < (*subst).u.str_0.len
                {
                    i += 1;
                    i;
                    match *((*subst).u.str_0.data).offset(i as isize) as i32 {
                        36 => {
                            if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                                result_len = result_len
                                    .wrapping_add((1 as i32 + 1024 as i32) as u32);
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut i8;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                b"$\0" as *const u8 as *const i8 as *const libc::c_void,
                                1 as i32 as u64,
                            );
                            result_pos = result_pos.wrapping_add(1 as i32 as u32);
                        }
                        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                            j = *((*subst).u.str_0.data).offset(i as isize) as i32
                                - '0' as i32;
                            if *(matches.start).offset(j as isize) >= 0 as i32 {
                                if result_len
                                    < result_pos
                                        .wrapping_add(
                                            (*(matches.end).offset(j as isize)
                                                - *(matches.start).offset(j as isize)) as u32,
                                        )
                                {
                                    result_len = result_len
                                        .wrapping_add(
                                            (*(matches.end).offset(j as isize)
                                                - *(matches.start).offset(j as isize) + 1024 as i32) as u32,
                                        );
                                    result = xrealloc(
                                        result as *mut libc::c_void,
                                        result_len as size_t,
                                    ) as *mut i8;
                                }
                                memcpy(
                                    result.offset(result_pos as isize) as *mut libc::c_void,
                                    ((*str).u.str_0.data)
                                        .offset(*(matches.start).offset(j as isize) as isize)
                                        as *const libc::c_void,
                                    (*(matches.end).offset(j as isize)
                                        - *(matches.start).offset(j as isize)) as u64,
                                );
                                result_pos = result_pos
                                    .wrapping_add(
                                        (*(matches.end).offset(j as isize)
                                            - *(matches.start).offset(j as isize)) as u32,
                                    );
                            }
                        }
                        _ => {
                            if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                                result_len = result_len
                                    .wrapping_add((1 as i32 + 1024 as i32) as u32);
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut i8;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                b"$\0" as *const u8 as *const i8 as *const libc::c_void,
                                1 as i32 as u64,
                            );
                            result_pos = result_pos.wrapping_add(1 as i32 as u32);
                            if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                                result_len = result_len
                                    .wrapping_add((1 as i32 + 1024 as i32) as u32);
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut i8;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                ((*subst).u.str_0.data).offset(i as isize)
                                    as *const libc::c_void,
                                1 as i32 as u64,
                            );
                            result_pos = result_pos.wrapping_add(1 as i32 as u32);
                        }
                    }
                } else {
                    if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                        result_len = result_len
                            .wrapping_add((1 as i32 + 1024 as i32) as u32);
                        result = xrealloc(
                            result as *mut libc::c_void,
                            result_len as size_t,
                        ) as *mut i8;
                    }
                    memcpy(
                        result.offset(result_pos as isize) as *mut libc::c_void,
                        ((*subst).u.str_0.data).offset(i as isize)
                            as *const libc::c_void,
                        1 as i32 as u64,
                    );
                    result_pos = result_pos.wrapping_add(1 as i32 as u32);
                }
                i += 1;
                i;
            }
        }
        pos = *(matches.end).offset(0 as i32 as isize);
        if allp == 0 {
            break;
        }
    }
    if num_matches == 0 as i32 {
        node_free(re);
        node_free(subst);
        return str;
    }
    if result_len
        < result_pos.wrapping_add(((*str).u.str_0.len).wrapping_sub(pos as u32))
    {
        result_len = result_len
            .wrapping_add(
                ((*str).u.str_0.len)
                    .wrapping_sub(pos as u32)
                    .wrapping_add(1024 as i32 as u32),
            );
        result = xrealloc(result as *mut libc::c_void, result_len as size_t) as *mut i8;
    }
    memcpy(
        result.offset(result_pos as isize) as *mut libc::c_void,
        ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
        ((*str).u.str_0.len).wrapping_sub(pos as u32) as u64,
    );
    result_pos = result_pos.wrapping_add(((*str).u.str_0.len).wrapping_sub(pos as u32));
    node_free(str);
    node_free(re);
    node_free(subst);
    str = node_alloc(NodeType::nSTRING);
    (*str).u.str_0.len = result_pos;
    (*str).u.str_0.data = xmalloc(result_pos as size_t) as *mut i8;
    memcpy(
        (*str).u.str_0.data as *mut libc::c_void,
        result as *const libc::c_void,
        result_pos as u64,
    );
    return str;
}
unsafe extern "C" fn prim_regsub(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut re: *mut Node = 0 as *mut Node;
    let mut subst: *mut Node = 0 as *mut Node;
    str = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    re = match_arg(prim_name, NodeType::nREGEXP as i32 as u32, &mut arg, env, linenum);
    subst = match_arg(
        prim_name,
        NodeType::nSTRING as i32 as u32,
        &mut arg,
        env,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    return do_regsubsts(str, re, subst, 0 as i32);
}
unsafe extern "C" fn prim_regsuball(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut re: *mut Node = 0 as *mut Node;
    let mut subst: *mut Node = 0 as *mut Node;
    str = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    re = match_arg(prim_name, NodeType::nREGEXP as i32 as u32, &mut arg, env, linenum);
    subst = match_arg(
        prim_name,
        NodeType::nSTRING as i32 as u32,
        &mut arg,
        env,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    return do_regsubsts(str, re, subst, 1 as i32);
}
unsafe extern "C" fn prim_split(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut re: *mut Node = 0 as *mut Node;
    let mut str: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut n2: *mut Node = 0 as *mut Node;
    let mut pos: i32 = 0;
    let mut i: i32 = 0;
    re = match_arg(prim_name, NodeType::nREGEXP as i32 as u32, &mut arg, env, linenum);
    str = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    n = node_alloc(NodeType::nARRAY);
    (*n).u.array.allocated = 100 as i32 as u32;
    (*n).u.array.array = xcalloc(
        (*n).u.array.allocated as size_t,
        ::core::mem::size_of::<*mut Node>() as u64,
    ) as *mut *mut Node;
    pos = 0 as i32;
    while (pos as u32) < (*str).u.str_0.len {
        i = re_search(
            if ((*re).u.re.compiled).fastmap_accurate() as i32 != 0 {
                &mut (*re).u.re.compiled
            } else {
                compile_regexp(re);
                &mut (*re).u.re.compiled
            },
            (*str).u.str_0.data,
            (*str).u.str_0.len as i32,
            pos,
            ((*str).u.str_0.len).wrapping_sub(pos as u32) as i32,
            &mut (*re).u.re.matches,
        );
        if i < 0 as i32 {
            break;
        }
        n2 = node_alloc(NodeType::nSTRING);
        (*n2).u.str_0.len = (i - pos) as u32;
        (*n2).u.str_0.data = xmalloc(
            ((*n2).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
        ) as *mut i8;
        memcpy(
            (*n2).u.str_0.data as *mut libc::c_void,
            ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
            (*n2).u.str_0.len as u64,
        );
        pos = *((*re).u.re.matches.end).offset(0 as i32 as isize);
        if ((*n).u.array.len).wrapping_add(1 as i32 as u32) >= (*n).u.array.allocated {
            (*n).u.array.allocated = ((*n).u.array.allocated)
                .wrapping_add(100 as i32 as u32);
            (*n).u.array.array = xrealloc(
                (*n).u.array.array as *mut libc::c_void,
                ((*n).u.array.allocated as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut Node>() as u64),
            ) as *mut *mut Node;
        }
        let fresh2 = (*n).u.array.len;
        (*n).u.array.len = ((*n).u.array.len).wrapping_add(1);
        let ref mut fresh3 = *((*n).u.array.array).offset(fresh2 as isize);
        *fresh3 = n2;
    }
    n2 = node_alloc(NodeType::nSTRING);
    (*n2).u.str_0.len = ((*str).u.str_0.len).wrapping_sub(pos as u32);
    (*n2).u.str_0.data = xmalloc(
        ((*n2).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
    ) as *mut i8;
    memcpy(
        (*n2).u.str_0.data as *mut libc::c_void,
        ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
        (*n2).u.str_0.len as u64,
    );
    let fresh4 = (*n).u.array.len;
    (*n).u.array.len = ((*n).u.array.len).wrapping_add(1);
    let ref mut fresh5 = *((*n).u.array.array).offset(fresh4 as isize);
    *fresh5 = n2;
    return n;
}
unsafe extern "C" fn prim_sprintf(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut current_block: u64;
    let mut arg: *mut ListItem = (*args).head;
    let mut fmt: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut buf: [i8; 512] = [0; 512];
    let mut ifmt: [i8; 256] = [0; 256];
    let mut ifmtopts: [i8; 256] = [0; 256];
    let mut result: *mut i8 = 0 as *mut i8;
    let mut result_pos: u32 = 0 as i32 as u32;
    let mut result_len: u32 = 0 as i32 as u32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut argument_count: i32 = 0 as i32;
    let mut cp: *mut i8 = 0 as *mut i8;
    fmt = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    cp = (*fmt).u.str_0.data;
    i = 0 as i32;
    while (i as u32) < (*fmt).u.str_0.len {
        if *cp.offset(i as isize) as i32 == '%' as i32
            && ((i + 1 as i32) as u32 >= (*fmt).u.str_0.len
                || *cp.offset((i + 1 as i32) as isize) as i32 == '%' as i32)
        {
            i += 1;
            i;
            if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                result_len = result_len.wrapping_add((1 as i32 + 1024 as i32) as u32);
                result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                    as *mut i8;
            }
            memcpy(
                result.offset(result_pos as isize) as *mut libc::c_void,
                cp.offset(i as isize) as *const libc::c_void,
                1 as i32 as u64,
            );
            result_pos = result_pos.wrapping_add(1 as i32 as u32);
        } else if *cp.offset(i as isize) as i32 == '%' as i32 {
            argument_count += 1;
            argument_count;
            if arg.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: primitive `%s': too few arguments for format\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    program,
                    prim_name,
                );
                exit(1 as i32);
            }
            n = eval_expr((*arg).data as *mut Expr, env);
            arg = (*arg).next;
            i += 1;
            i;
            j = 0 as i32;
            while (i as u32) < (*fmt).u.str_0.len
                && ('0' as i32 <= *cp.offset(i as isize) as i32
                    && *cp.offset(i as isize) as i32 <= '9' as i32
                    || *cp.offset(i as isize) as i32 == '.' as i32
                    || *cp.offset(i as isize) as i32 == '-' as i32)
            {
                ifmtopts[j as usize] = *cp.offset(i as isize);
                i += 1;
                i;
                j += 1;
                j;
            }
            ifmtopts[j as usize] = '\0' as i32 as i8;
            if i as u32 >= (*fmt).u.str_0.len {
                if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                    result_len = result_len
                        .wrapping_add((1 as i32 + 1024 as i32) as u32);
                    result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                        as *mut i8;
                }
                memcpy(
                    result.offset(result_pos as isize) as *mut libc::c_void,
                    b"%\0" as *const u8 as *const i8 as *const libc::c_void,
                    1 as i32 as u64,
                );
                result_pos = result_pos.wrapping_add(1 as i32 as u32);
                if result_len < result_pos.wrapping_add(j as u32) {
                    result_len = result_len.wrapping_add((j + 1024 as i32) as u32);
                    result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                        as *mut i8;
                }
                memcpy(
                    result.offset(result_pos as isize) as *mut libc::c_void,
                    ifmtopts.as_mut_ptr() as *const libc::c_void,
                    j as u64,
                );
                result_pos = result_pos.wrapping_add(j as u32);
            } else {
                match *cp.offset(i as isize) as i32 {
                    120 | 88 | 100 => {
                        if (*n).type_0 as u32 != NodeType::nINTEGER as i32 as u32 {
                            current_block = 4675114907230213655;
                        } else {
                            sprintf(
                                ifmt.as_mut_ptr(),
                                b"%%%s%c\0" as *const u8 as *const i8,
                                ifmtopts.as_mut_ptr(),
                                *cp.offset(i as isize) as i32,
                            );
                            sprintf(buf.as_mut_ptr(), ifmt.as_mut_ptr(), (*n).u.integer);
                            if (result_len as u64)
                                < (result_pos as u64).wrapping_add(strlen(buf.as_mut_ptr()))
                            {
                                result_len = (result_len as u64)
                                    .wrapping_add(
                                        (strlen(buf.as_mut_ptr())).wrapping_add(1024 as i32 as u64),
                                    ) as u32 as u32;
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut i8;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                strlen(buf.as_mut_ptr()),
                            );
                            result_pos = (result_pos as u64)
                                .wrapping_add(strlen(buf.as_mut_ptr())) as u32 as u32;
                            current_block = 15619007995458559411;
                        }
                    }
                    102 | 103 | 101 | 69 => {
                        if (*n).type_0 as u32 != NodeType::nREAL as i32 as u32 {
                            current_block = 4675114907230213655;
                        } else {
                            sprintf(
                                ifmt.as_mut_ptr(),
                                b"%%%s%c\0" as *const u8 as *const i8,
                                ifmtopts.as_mut_ptr(),
                                *cp.offset(i as isize) as i32,
                            );
                            sprintf(buf.as_mut_ptr(), ifmt.as_mut_ptr(), (*n).u.real);
                            if (result_len as u64)
                                < (result_pos as u64).wrapping_add(strlen(buf.as_mut_ptr()))
                            {
                                result_len = (result_len as u64)
                                    .wrapping_add(
                                        (strlen(buf.as_mut_ptr())).wrapping_add(1024 as i32 as u64),
                                    ) as u32 as u32;
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut i8;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                strlen(buf.as_mut_ptr()),
                            );
                            result_pos = (result_pos as u64)
                                .wrapping_add(strlen(buf.as_mut_ptr())) as u32 as u32;
                            current_block = 15619007995458559411;
                        }
                    }
                    115 => {
                        if (*n).type_0 as u32 != NodeType::nSTRING as i32 as u32 {
                            current_block = 4675114907230213655;
                        } else {
                            if ifmtopts[0 as i32 as usize] as i32 != '\0' as i32 {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const i8,
                                        b"%s:%d: %s: no extra options can be specified for %%s\n\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    defs_file,
                                    linenum,
                                    prim_name,
                                );
                                exit(1 as i32);
                            }
                            if result_len < result_pos.wrapping_add((*n).u.str_0.len) {
                                result_len = result_len
                                    .wrapping_add(
                                        ((*n).u.str_0.len).wrapping_add(1024 as i32 as u32),
                                    );
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut i8;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                (*n).u.str_0.data as *const libc::c_void,
                                (*n).u.str_0.len as u64,
                            );
                            result_pos = result_pos.wrapping_add((*n).u.str_0.len);
                            current_block = 15619007995458559411;
                        }
                    }
                    _ => {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"%s:%d: %s: illegal type specifier `%c'\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            defs_file,
                            linenum,
                            prim_name,
                            *cp.offset(i as isize) as i32,
                        );
                        exit(1 as i32);
                    }
                }
                match current_block {
                    15619007995458559411 => {}
                    _ => {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"%s:%d: %s: argument %d doesn't match format\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            defs_file,
                            linenum,
                            prim_name,
                            argument_count,
                        );
                        exit(1 as i32);
                    }
                }
            }
        } else {
            if result_len < result_pos.wrapping_add(1 as i32 as u32) {
                result_len = result_len.wrapping_add((1 as i32 + 1024 as i32) as u32);
                result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                    as *mut i8;
            }
            memcpy(
                result.offset(result_pos as isize) as *mut libc::c_void,
                cp.offset(i as isize) as *const libc::c_void,
                1 as i32 as u64,
            );
            result_pos = result_pos.wrapping_add(1 as i32 as u32);
        }
        i += 1;
        i;
    }
    node_free(fmt);
    n = node_alloc(NodeType::nSTRING);
    (*n).u.str_0.len = result_pos;
    (*n).u.str_0.data = result;
    return n;
}
unsafe extern "C" fn prim_strcmp(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut current_block: u64;
    let mut arg: *mut ListItem = (*args).head;
    let mut s1: *mut Node = 0 as *mut Node;
    let mut s2: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    let mut cp1: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    s1 = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    s2 = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    cp1 = (*s1).u.str_0.data;
    cp2 = (*s2).u.str_0.data;
    i = 0 as i32;
    loop {
        if !((i as u32) < (*s1).u.str_0.len && (i as u32) < (*s2).u.str_0.len) {
            current_block = 9606288038608642794;
            break;
        }
        if (*cp1.offset(i as isize) as i32) < *cp2.offset(i as isize) as i32 {
            result = -(1 as i32);
            current_block = 341154758773065815;
            break;
        } else if *cp1.offset(i as isize) as i32 > *cp2.offset(i as isize) as i32 {
            result = 1 as i32;
            current_block = 341154758773065815;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        9606288038608642794 => {
            if (*s1).u.str_0.len < (*s2).u.str_0.len {
                result = -(1 as i32);
            } else if (*s1).u.str_0.len > (*s2).u.str_0.len {
                result = 1 as i32;
            } else {
                result = 0 as i32;
            }
        }
        _ => {}
    }
    node_free(s1);
    node_free(s2);
    n = node_alloc(NodeType::nINTEGER);
    (*n).u.integer = result;
    return n;
}
unsafe extern "C" fn prim_string(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut buf: [i8; 512] = [0; 512];
    n = match_arg(prim_name, NodeType::nVOID as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    r = node_alloc(NodeType::nSTRING);
    match (*n).type_0 as u32 {
        0 | 2 | 6 => {
            (*r).u.str_0.data = xcalloc(1 as i32 as size_t, 1 as i32 as size_t)
                as *mut i8;
            (*r).u.str_0.len = 0 as i32 as u32;
        }
        5 => {
            (*r).u.str_0.len = strlen((*n).u.sym) as u32;
            (*r).u.str_0.data = xmalloc((*r).u.str_0.len as size_t) as *mut i8;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                (*n).u.sym as *const libc::c_void,
                (*r).u.str_0.len as u64,
            );
        }
        1 => {
            (*r).u.str_0.len = (*n).u.str_0.len;
            (*r).u.str_0.data = xmalloc((*n).u.str_0.len as size_t) as *mut i8;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as u64,
            );
        }
        3 => {
            sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const i8, (*n).u.integer);
            (*r).u.str_0.len = strlen(buf.as_mut_ptr()) as u32;
            (*r).u.str_0.data = xmalloc((*r).u.str_0.len as size_t) as *mut i8;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                (*r).u.str_0.len as u64,
            );
        }
        4 => {
            sprintf(buf.as_mut_ptr(), b"%f\0" as *const u8 as *const i8, (*n).u.real);
            (*r).u.str_0.len = strlen(buf.as_mut_ptr()) as u32;
            (*r).u.str_0.data = xmalloc((*r).u.str_0.len as size_t) as *mut i8;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                (*r).u.str_0.len as u64,
            );
        }
        _ => {}
    }
    node_free(n);
    return r;
}
unsafe extern "C" fn prim_strncmp(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut current_block: u64;
    let mut arg: *mut ListItem = (*args).head;
    let mut s1: *mut Node = 0 as *mut Node;
    let mut s2: *mut Node = 0 as *mut Node;
    let mut len: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    let mut cp1: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    s1 = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    s2 = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    len = match_arg(prim_name, NodeType::nINTEGER as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    cp1 = (*s1).u.str_0.data;
    cp2 = (*s2).u.str_0.data;
    i = 0 as i32;
    loop {
        if !((i as u32) < (*s1).u.str_0.len && (i as u32) < (*s2).u.str_0.len
            && i < (*len).u.integer)
        {
            current_block = 1054647088692577877;
            break;
        }
        if (*cp1.offset(i as isize) as i32) < *cp2.offset(i as isize) as i32 {
            result = -(1 as i32);
            current_block = 15682296595439280359;
            break;
        } else if *cp1.offset(i as isize) as i32 > *cp2.offset(i as isize) as i32 {
            result = 1 as i32;
            current_block = 15682296595439280359;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        1054647088692577877 => {
            if i >= (*len).u.integer {
                result = 0 as i32;
            } else if (*s1).u.str_0.len < (*s2).u.str_0.len {
                result = -(1 as i32);
            } else if (*s1).u.str_0.len > (*s2).u.str_0.len {
                result = 1 as i32;
            } else {
                result = 0 as i32;
            }
        }
        _ => {}
    }
    node_free(s1);
    node_free(s2);
    node_free(len);
    n = node_alloc(NodeType::nINTEGER);
    (*n).u.integer = result;
    return n;
}
unsafe extern "C" fn prim_substring(
    mut prim_name: *mut i8,
    mut args: *mut List,
    mut env: *mut Environment,
    mut linenum: u32,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut start: *mut Node = 0 as *mut Node;
    let mut end: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    str = match_arg(prim_name, NodeType::nSTRING as i32 as u32, &mut arg, env, linenum);
    start = match_arg(
        prim_name,
        NodeType::nINTEGER as i32 as u32,
        &mut arg,
        env,
        linenum,
    );
    end = match_arg(prim_name, NodeType::nINTEGER as i32 as u32, &mut arg, env, linenum);
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    if (*start).u.integer > (*end).u.integer {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: start offset is bigger than end offset\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    if (*end).u.integer as u32 > (*str).u.str_0.len {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s: offset out of range\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            defs_file,
            linenum,
            prim_name,
        );
        exit(1 as i32);
    }
    n = node_alloc(NodeType::nSTRING);
    (*n).u.str_0.len = ((*end).u.integer - (*start).u.integer) as u32;
    (*n).u.str_0.data = xmalloc(
        ((*n).u.str_0.len).wrapping_add(1 as i32 as u32) as size_t,
    ) as *mut i8;
    memcpy(
        (*n).u.str_0.data as *mut libc::c_void,
        ((*str).u.str_0.data).offset((*start).u.integer as isize) as *const libc::c_void,
        (*n).u.str_0.len as u64,
    );
    node_free(str);
    node_free(start);
    node_free(end);
    return n;
}
static mut prims: [C2RustUnnamed_10; 25] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_10 {
                name: b"call\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_call),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"check_namerules\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_check_namerules),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"check_startrules\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_check_startrules),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"concat\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_concat),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"float\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_float),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"getenv\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_getenv),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"int\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_int),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"length\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_length),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"list\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_list),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"panic\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_panic),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"prereq\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_prereq),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"print\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_print),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"range\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_range),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regexp\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regexp),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regexp_syntax\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regexp_syntax),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regmatch\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regmatch),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regsub\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regsub),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regsuball\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regsuball),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"split\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_split),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"sprintf\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_sprintf),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"strcmp\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_strcmp),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"string\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_string),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"strncmp\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_strncmp),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"substring\0" as *const u8 as *const i8 as *mut i8,
                prim: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut i8,
                                *mut List,
                                *mut Environment,
                                u32,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_substring),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: 0 as *const i8 as *mut i8,
                prim: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn init_primitives() {
    let mut old: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: i32 = 0;
    i = 0 as i32;
    while !(prims[i as usize].name).is_null() {
        if strhash_put(
            ns_prims,
            prims[i as usize].name,
            strlen(prims[i as usize].name) as i32,
            ::core::mem::transmute::<
                Primitive,
                *mut libc::c_void,
            >(prims[i as usize].prim),
            &mut old,
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
        i += 1;
        i;
    }
}