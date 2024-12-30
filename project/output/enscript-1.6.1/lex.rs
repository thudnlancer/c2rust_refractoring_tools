#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
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
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strtod(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn yyerror(msg: *mut libc::c_char);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn node_alloc(type_0: NodeType) -> *mut Node;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    static mut linenum: libc::c_uint;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut yylval: YYSTYPE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: yy_size_t,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type yy_size_t = libc::c_uint;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type YY_CHAR = libc::c_uchar;
pub type yy_state_type = libc::c_int;
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
    nVOID,
    nSTRING,
    nREGEXP,
    nINTEGER,
    nREAL,
    nSYMBOL,
    nARRAY,
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
    sRETURN,
    sDEFSUB,
    sBLOCK,
    sIF,
    sEXPR,
    sWHILE,
    sFOR,
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn eat_comment() {
    let mut c: libc::c_int = 0;
    loop {
        c = input();
        if !(c != -(1 as libc::c_int)) {
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
            if c == -(1 as libc::c_int) {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: EOF in comment\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
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
            0 as *const libc::c_char,
            b"error: EOF in comment\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn yywrap() -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_string(
    mut len_return: *mut libc::c_uint,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: libc::c_int = 0 as libc::c_int;
    let mut bufpos: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    while done == 0 {
        ch = input();
        if ch == '\n' as i32 {
            linenum = linenum.wrapping_add(1);
            linenum;
        }
        let mut current_block_26: u64;
        match ch {
            -1 => {
                current_block_26 = 13509395632502472069;
            }
            34 => {
                done = 1 as libc::c_int;
                current_block_26 = 6417057564578538666;
            }
            92 => {
                ch = input();
                match ch {
                    110 => {
                        current_block_26 = 16598926681669342143;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    116 => {
                        current_block_26 = 13262478452272424093;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    118 => {
                        current_block_26 = 7369904960316564698;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    98 => {
                        current_block_26 = 15109432795376173018;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    114 => {
                        current_block_26 = 7103576243689391462;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    102 => {
                        current_block_26 = 16626929343264605553;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    97 => {
                        current_block_26 = 18272008333722400074;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                    -1 => {
                        current_block_26 = 13509395632502472069;
                    }
                    _ => {
                        current_block_26 = 11665363508214832611;
                        match current_block_26 {
                            11665363508214832611 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            16598926681669342143 => {
                                ch = '\n' as i32;
                            }
                            7369904960316564698 => {
                                ch = '\u{b}' as i32;
                            }
                            15109432795376173018 => {
                                ch = '\u{8}' as i32;
                            }
                            7103576243689391462 => {
                                ch = '\r' as i32;
                            }
                            16626929343264605553 => {
                                ch = '\u{c}' as i32;
                            }
                            18272008333722400074 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\t' as i32;
                            }
                        }
                        current_block_26 = 9606193718916044350;
                    }
                }
            }
            _ => {
                current_block_26 = 9606193718916044350;
            }
        }
        match current_block_26 {
            9606193718916044350 => {
                if bufpos >= buflen {
                    buflen += 1024 as libc::c_int;
                    buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                        as *mut libc::c_char;
                }
                let fresh0 = bufpos;
                bufpos = bufpos + 1;
                *buf.offset(fresh0 as isize) = ch as libc::c_char;
            }
            13509395632502472069 => {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: EOF in string constant\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                done = 1 as libc::c_int;
            }
            _ => {}
        }
    }
    buf2 = xmalloc((bufpos + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        buf2 as *mut libc::c_void,
        buf as *const libc::c_void,
        bufpos as libc::c_ulong,
    );
    *buf2.offset(bufpos as isize) = '\0' as i32 as libc::c_char;
    xfree(buf as *mut libc::c_void);
    *len_return = bufpos as libc::c_uint;
    return buf2;
}
static mut yy_current_buffer: YY_BUFFER_STATE = 0 as *const yy_buffer_state
    as YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
#[no_mangle]
pub static mut yyleng: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 1 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
#[no_mangle]
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut yy_accept: [libc::c_short; 108] = [
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
];
unsafe extern "C" fn read_regexp(mut node: *mut Node) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: libc::c_int = 0 as libc::c_int;
    let mut bufpos: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut current_block_25: u64;
    while done == 0 {
        ch = input();
        match ch {
            -1 => {
                current_block_25 = 17481033863940419519;
            }
            47 => {
                done = 1 as libc::c_int;
                continue;
            }
            92 => {
                ch = input();
                match ch {
                    10 => {
                        current_block_25 = 14279995577508924856;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                    110 => {
                        current_block_25 = 7350512134151601729;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                    114 => {
                        current_block_25 = 465876046698638192;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                    102 => {
                        current_block_25 = 7264131456114942388;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                    116 => {
                        current_block_25 = 16301682354449893158;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                    47 | 92 => {
                        current_block_25 = 1830506787698777107;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                    -1 => {
                        current_block_25 = 17481033863940419519;
                    }
                    _ => {
                        current_block_25 = 4108458233072306692;
                        match current_block_25 {
                            4108458233072306692 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
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
                            7350512134151601729 => {
                                ch = '\n' as i32;
                            }
                            465876046698638192 => {
                                ch = '\r' as i32;
                            }
                            7264131456114942388 => {
                                ch = '\u{c}' as i32;
                            }
                            16301682354449893158 => {
                                ch = '\t' as i32;
                            }
                            1830506787698777107 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 809521523908297035;
                    }
                }
            }
            _ => {
                current_block_25 = 809521523908297035;
            }
        }
        match current_block_25 {
            809521523908297035 => {
                if bufpos >= buflen {
                    buflen += 1024 as libc::c_int;
                    buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                        as *mut libc::c_char;
                }
                let fresh1 = bufpos;
                bufpos = bufpos + 1;
                *buf.offset(fresh1 as isize) = ch as libc::c_char;
            }
            _ => {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: EOF in regular expression\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                done = 1 as libc::c_int;
            }
        }
    }
    done = 0 as libc::c_int;
    while done == 0 {
        ch = input();
        match ch {
            105 => {
                (*node).u.re.flags |= 1 as libc::c_int as libc::c_uint;
            }
            _ => {
                yyunput(ch, yytext);
                done = 1 as libc::c_int;
            }
        }
    }
    buf2 = xmalloc((bufpos + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        buf2 as *mut libc::c_void,
        buf as *const libc::c_void,
        bufpos as libc::c_ulong,
    );
    *buf2.offset(bufpos as isize) = '\0' as i32 as libc::c_char;
    xfree(buf as *mut libc::c_void);
    (*node).u.re.data = buf2;
    (*node).u.re.len = bufpos as libc::c_uint;
}
static mut yy_ec: [libc::c_int; 256] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    1 as libc::c_int,
    6 as libc::c_int,
    1 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    1 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    15 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    18 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    18 as libc::c_int,
    22 as libc::c_int,
    18 as libc::c_int,
    23 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    24 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    1 as libc::c_int,
    25 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    18 as libc::c_int,
    1 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    18 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    1 as libc::c_int,
    44 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
];
static mut yy_meta: [libc::c_int; 45] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
];
static mut yy_base: [libc::c_short; 112] = [
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    130 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    113 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    121 as libc::c_int as libc::c_short,
    102 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    115 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    106 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    95 as libc::c_int as libc::c_short,
    85 as libc::c_int as libc::c_short,
    83 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    82 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    101 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    83 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    75 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
];
static mut yy_def: [libc::c_short; 112] = [
    0 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
];
static mut yy_nxt: [libc::c_short; 176] = [
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    106 as libc::c_int as libc::c_short,
    88 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    104 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    103 as libc::c_int as libc::c_short,
    102 as libc::c_int as libc::c_short,
    101 as libc::c_int as libc::c_short,
    100 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    97 as libc::c_int as libc::c_short,
    96 as libc::c_int as libc::c_short,
    95 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    90 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    85 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    83 as libc::c_int as libc::c_short,
    82 as libc::c_int as libc::c_short,
    81 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    80 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    78 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    76 as libc::c_int as libc::c_short,
    75 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    73 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    70 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
];
static mut yy_chk: [libc::c_short; 176] = [
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    103 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    102 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    101 as libc::c_int as libc::c_short,
    100 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    97 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    88 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    85 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    81 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    76 as libc::c_int as libc::c_short,
    75 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut yytext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn yylex() -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    if yy_init != 0 {
        yy_init = 0 as libc::c_int;
        if yy_start == 0 {
            yy_start = 1 as libc::c_int;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if yy_current_buffer.is_null() {
            yy_current_buffer = yy_create_buffer(yyin, 16384 as libc::c_int);
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
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as libc::c_uchar as libc::c_uint
                    as usize] as YY_CHAR;
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 108 as libc::c_int {
                        yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_uint)
                    .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 131 as libc::c_int)
                {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
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
                            yylval.node = node_alloc(nSTRING);
                            (*yylval.node)
                                .u
                                .str_0
                                .data = read_string(&mut (*yylval.node).u.str_0.len);
                            return 260 as libc::c_int;
                        }
                        5 => {
                            yylval.node = node_alloc(nINTEGER);
                            (*yylval.node)
                                .u
                                .integer = *yytext.offset(1 as libc::c_int as isize)
                                as libc::c_int;
                            return 261 as libc::c_int;
                        }
                        6 => {
                            yylval.node = node_alloc(nINTEGER);
                            match *yytext.offset(2 as libc::c_int as isize)
                                as libc::c_int
                            {
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
                                    (*yylval.node)
                                        .u
                                        .integer = *yytext.offset(2 as libc::c_int as isize)
                                        as libc::c_int;
                                }
                            }
                            return 261 as libc::c_int;
                        }
                        7 => {
                            yylval.node = node_alloc(nREGEXP);
                            read_regexp(yylval.node);
                            return 259 as libc::c_int;
                        }
                        8 => return 268 as libc::c_int,
                        9 => return 269 as libc::c_int,
                        10 => return 286 as libc::c_int,
                        11 => return 272 as libc::c_int,
                        12 => return 275 as libc::c_int,
                        13 => return 271 as libc::c_int,
                        14 => return 273 as libc::c_int,
                        15 => return 267 as libc::c_int,
                        16 => return 270 as libc::c_int,
                        17 => return 265 as libc::c_int,
                        18 => return 266 as libc::c_int,
                        19 => return 264 as libc::c_int,
                        20 => return 263 as libc::c_int,
                        21 => return 274 as libc::c_int,
                        22 => return 282 as libc::c_int,
                        23 => return 283 as libc::c_int,
                        24 => return 285 as libc::c_int,
                        25 => return 284 as libc::c_int,
                        26 => return 281 as libc::c_int,
                        27 => return 280 as libc::c_int,
                        28 => return 287 as libc::c_int,
                        29 => return 288 as libc::c_int,
                        30 => return 276 as libc::c_int,
                        31 => return 277 as libc::c_int,
                        32 => return 278 as libc::c_int,
                        33 => return 279 as libc::c_int,
                        34 => {
                            yylval.node = node_alloc(nREAL);
                            (*yylval.node).u.real = atof(yytext);
                            return 262 as libc::c_int;
                        }
                        35 => {
                            yylval.node = node_alloc(nINTEGER);
                            (*yylval.node).u.integer = atoi(yytext);
                            return 261 as libc::c_int;
                        }
                        36 => {
                            yylval.node = node_alloc(nSYMBOL);
                            (*yylval.node).u.sym = xstrdup(yytext);
                            return 258 as libc::c_int;
                        }
                        37 => {
                            return *yytext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        38 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as size_t,
                                1 as libc::c_int as size_t,
                                yyout,
                            );
                            break '_yy_match;
                        }
                        40 => return 0 as libc::c_int,
                        39 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (*yy_current_buffer).yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (*yy_current_buffer).yy_n_chars;
                                (*yy_current_buffer).yy_input_file = yyin;
                                (*yy_current_buffer).yy_buffer_status = 1 as libc::c_int;
                            }
                            if yy_c_buf_p
                                <= &mut *((*yy_current_buffer).yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut libc::c_char
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as libc::c_int as isize);
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
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        if yywrap() != 0 {
                                            yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                            yy_act = 39 as libc::c_int
                                                + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                                + 1 as libc::c_int;
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
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((*yy_current_buffer).yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut libc::c_char;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
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
                                    as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char = (*yy_current_buffer).yy_ch_buf;
    let mut source: *mut libc::c_char = yytext;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p
        > &mut *((*yy_current_buffer).yy_ch_buf)
            .offset((yy_n_chars + 1 as libc::c_int) as isize) as *mut libc::c_char
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*yy_current_buffer).yy_fill_buffer == 0 as libc::c_int {
        if yy_c_buf_p.offset_from(yytext) as libc::c_long
            - 0 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = yy_c_buf_p.offset_from(yytext) as libc::c_long as libc::c_int
        - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh2 = source;
        source = source.offset(1);
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 = *fresh2;
        i += 1;
        i;
    }
    if (*yy_current_buffer).yy_buffer_status == 2 as libc::c_int {
        yy_n_chars = 0 as libc::c_int;
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: libc::c_int = ((*yy_current_buffer).yy_buf_size)
            .wrapping_sub(number_to_move as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            let mut b: YY_BUFFER_STATE = yy_current_buffer;
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int = ((*b).yy_buf_size)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b)
                        .yy_buf_size = ((*b).yy_buf_size as libc::c_uint)
                        .wrapping_add(
                            ((*b).yy_buf_size)
                                .wrapping_div(8 as libc::c_int as libc::c_uint),
                        ) as yy_size_t as yy_size_t;
                } else {
                    (*b)
                        .yy_buf_size = ((*b).yy_buf_size as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as yy_size_t
                        as yy_size_t;
                }
                (*b)
                    .yy_ch_buf = yy_flex_realloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size).wrapping_add(2 as libc::c_int as libc::c_uint),
                ) as *mut libc::c_char;
            } else {
                (*b).yy_ch_buf = 0 as *mut libc::c_char;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const libc::c_char,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut libc::c_char;
            num_to_read = ((*yy_current_buffer).yy_buf_size)
                .wrapping_sub(number_to_move as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        if (*yy_current_buffer).yy_is_interactive != 0 {
            let mut c: libc::c_int = '*' as i32;
            let mut n: libc::c_int = 0;
            n = 0 as libc::c_int;
            while n < num_to_read
                && {
                    c = getc(yyin);
                    c != -(1 as libc::c_int)
                } && c != '\n' as i32
            {
                *(&mut *((*yy_current_buffer).yy_ch_buf).offset(number_to_move as isize)
                    as *mut libc::c_char)
                    .offset(n as isize) = c as libc::c_char;
                n += 1;
                n;
            }
            if c == '\n' as i32 {
                let fresh4 = n;
                n = n + 1;
                *(&mut *((*yy_current_buffer).yy_ch_buf).offset(number_to_move as isize)
                    as *mut libc::c_char)
                    .offset(fresh4 as isize) = c as libc::c_char;
            }
            if c == -(1 as libc::c_int) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
                );
            }
            yy_n_chars = n;
        } else {
            yy_n_chars = fread(
                &mut *((*yy_current_buffer).yy_ch_buf).offset(number_to_move as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
                num_to_read as size_t,
                yyin,
            ) as libc::c_int;
            if yy_n_chars == 0 as libc::c_int && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            yyrestart(yyin);
        } else {
            ret_val = 2 as libc::c_int;
            (*yy_current_buffer).yy_buffer_status = 2 as libc::c_int;
        }
    } else {
        ret_val = 0 as libc::c_int;
    }
    yy_n_chars += number_to_move;
    *((*yy_current_buffer).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((*yy_current_buffer).yy_ch_buf)
        .offset(
            (yy_n_chars + 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    yytext = &mut *((*yy_current_buffer).yy_ch_buf).offset(0 as libc::c_int as isize)
        as *mut libc::c_char;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as libc::c_uchar as libc::c_uint as usize]
        } else {
            1 as libc::c_int
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 108 as libc::c_int {
                yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
            .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 108 as libc::c_int {
            yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
        .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 107 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
unsafe extern "C" fn yyunput(mut c: libc::c_int, mut yy_bp: *mut libc::c_char) {
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    *yy_cp = yy_hold_char;
    if yy_cp < ((*yy_current_buffer).yy_ch_buf).offset(2 as libc::c_int as isize) {
        let mut number_to_move: libc::c_int = yy_n_chars + 2 as libc::c_int;
        let mut dest: *mut libc::c_char = &mut *((*yy_current_buffer).yy_ch_buf)
            .offset(
                ((*yy_current_buffer).yy_buf_size)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) as *mut libc::c_char;
        let mut source: *mut libc::c_char = &mut *((*yy_current_buffer).yy_ch_buf)
            .offset(number_to_move as isize) as *mut libc::c_char;
        while source > (*yy_current_buffer).yy_ch_buf {
            source = source.offset(-1);
            dest = dest.offset(-1);
            *dest = *source;
        }
        yy_cp = yy_cp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_bp = yy_bp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_n_chars = (*yy_current_buffer).yy_buf_size as libc::c_int;
        (*yy_current_buffer).yy_n_chars = yy_n_chars;
        if yy_cp < ((*yy_current_buffer).yy_ch_buf).offset(2 as libc::c_int as isize) {
            yy_fatal_error(
                b"flex scanner push-back overflow\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    yy_cp = yy_cp.offset(-1);
    *yy_cp = c as libc::c_char;
    yytext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
unsafe extern "C" fn input() -> libc::c_int {
    let mut c: libc::c_int = 0;
    *yy_c_buf_p = yy_hold_char;
    if *yy_c_buf_p as libc::c_int == 0 as libc::c_int {
        if yy_c_buf_p
            < &mut *((*yy_current_buffer).yy_ch_buf).offset(yy_n_chars as isize)
                as *mut libc::c_char
        {
            *yy_c_buf_p = '\0' as i32 as libc::c_char;
        } else {
            let mut offset: libc::c_int = yy_c_buf_p.offset_from(yytext) as libc::c_long
                as libc::c_int;
            yy_c_buf_p = yy_c_buf_p.offset(1);
            yy_c_buf_p;
            let mut current_block_10: u64;
            match yy_get_next_buffer() {
                2 => {
                    yyrestart(yyin);
                    current_block_10 = 1409304843480750738;
                }
                1 => {
                    current_block_10 = 1409304843480750738;
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
                        return -(1 as libc::c_int);
                    }
                    if yy_did_buffer_switch_on_eof == 0 {
                        yyrestart(yyin);
                    }
                    return input();
                }
            }
        }
    }
    c = *(yy_c_buf_p as *mut libc::c_uchar) as libc::c_int;
    *yy_c_buf_p = '\0' as i32 as libc::c_char;
    yy_c_buf_p = yy_c_buf_p.offset(1);
    yy_hold_char = *yy_c_buf_p;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if yy_current_buffer.is_null() {
        yy_current_buffer = yy_create_buffer(yyin, 16384 as libc::c_int);
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
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
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
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yy_flex_alloc(
        ::core::mem::size_of::<yy_buffer_state>() as libc::c_ulong as yy_size_t,
    ) as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size as yy_size_t;
    (*b)
        .yy_ch_buf = yy_flex_alloc(
        ((*b).yy_buf_size).wrapping_add(2 as libc::c_int as libc::c_uint),
    ) as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
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
    (*b).yy_fill_buffer = 1 as libc::c_int;
    (*b)
        .yy_is_interactive = if !file.is_null() {
        (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*b)
        .yy_buf_pos = &mut *((*b).yy_ch_buf).offset(0 as libc::c_int as isize)
        as *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b == yy_current_buffer {
        yy_load_buffer_state();
    }
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut libc::c_char,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_uint
        || *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int != 0 as libc::c_int
        || *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int != 0 as libc::c_int
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = yy_flex_alloc(
        ::core::mem::size_of::<yy_buffer_state>() as libc::c_ulong as yy_size_t,
    ) as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size.wrapping_sub(2 as libc::c_int as libc::c_uint);
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size as libc::c_int;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    yy_switch_to_buffer(b);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_string(
    mut yy_str: *const libc::c_char,
) -> YY_BUFFER_STATE {
    let mut len: libc::c_int = 0;
    len = 0 as libc::c_int;
    while *yy_str.offset(len as isize) != 0 {
        len += 1;
        len;
    }
    return yy_scan_bytes(yy_str, len);
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_bytes(
    mut bytes: *const libc::c_char,
    mut len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    n = (len + 2 as libc::c_int) as yy_size_t;
    buf = yy_flex_alloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < len {
        *buf.offset(i as isize) = *bytes.offset(i as isize);
        i += 1;
        i;
    }
    let ref mut fresh5 = *buf.offset((len + 1 as libc::c_int) as isize);
    *fresh5 = 0 as libc::c_int as libc::c_char;
    *buf.offset(len as isize) = *fresh5;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
unsafe extern "C" fn yy_flex_alloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size as libc::c_ulong);
}
unsafe extern "C" fn yy_flex_realloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr as *mut libc::c_char as *mut libc::c_void, size as libc::c_ulong);
}
unsafe extern "C" fn yy_flex_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
