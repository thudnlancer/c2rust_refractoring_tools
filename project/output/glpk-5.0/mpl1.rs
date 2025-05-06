#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type DMP;
    pub type glp_file;
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_type(node: *mut AVLNODE, type_0: i32);
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_find_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_get_node_type(node: *mut AVLNODE) -> i32;
    fn _glp_avl_get_node_link(node: *mut AVLNODE) -> *mut libc::c_void;
    fn _glp_avl_delete_node(tree: *mut AVL, node: *mut AVLNODE);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_str2int(str: *const i8, val: *mut i32) -> i32;
    fn _glp_str2num(str: *const i8, val: *mut libc::c_double) -> i32;
    fn _glp_mpl_is_literal(mpl: *mut MPL, literal: *mut i8) -> i32;
    fn _glp_mpl_error(mpl: *mut MPL, fmt: *mut i8, _: ...);
    fn _glp_mpl_warning(mpl: *mut MPL, fmt: *mut i8, _: ...);
    fn _glp_mpl_read_char(mpl: *mut MPL) -> i32;
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: i32,
    pub c: i32,
    pub token: i32,
    pub imlen: i32,
    pub image: *mut i8,
    pub value: libc::c_double,
    pub b_token: i32,
    pub b_imlen: i32,
    pub b_image: *mut i8,
    pub b_value: libc::c_double,
    pub f_dots: i32,
    pub f_scan: i32,
    pub f_token: i32,
    pub f_imlen: i32,
    pub f_image: *mut i8,
    pub f_value: libc::c_double,
    pub context: *mut i8,
    pub c_ptr: i32,
    pub flag_d: i32,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: i32,
    pub as_within: i32,
    pub as_in: i32,
    pub as_binary: i32,
    pub flag_s: i32,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut i8,
    pub tup_buf: *mut i8,
    pub rand: *mut RNG,
    pub flag_p: i32,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: i32,
    pub n: i32,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut i8,
    pub out_fp: *mut glp_file,
    pub out_file: *mut i8,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut i8,
    pub jump: jmp_buf,
    pub phase: i32,
    pub mod_file: *mut i8,
    pub mpl_buf: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: i32,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: i32,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: i32,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: i32,
    pub dim: i32,
    pub size: i32,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: i32,
    pub arg: OPERANDS,
    pub type_0: i32,
    pub dim: i32,
    pub up: *mut CODE,
    pub vflag: i32,
    pub valid: i32,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut i8,
    pub index: C2RustUnnamed_6,
    pub par: C2RustUnnamed_5,
    pub set: C2RustUnnamed_4,
    pub var: C2RustUnnamed_3,
    pub con: C2RustUnnamed_2,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_1,
    pub loop_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut i8,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub dimen: i32,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: i32,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [i32; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: i32,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: i32,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: i32,
    pub link: *mut libc::c_void,
    pub na: i32,
    pub arg: *mut *mut i8,
    pub nf: i32,
    pub name: *mut *mut i8,
    pub type_0: *mut i32,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: i32,
    pub type_0: i32,
    pub u: C2RustUnnamed_7,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: i32,
    pub u: C2RustUnnamed_8,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub type_0: i32,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub in_0: C2RustUnnamed_11,
    pub out: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut i8,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut i8,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut i8,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub name: *mut i8,
    pub code: *mut CODE,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_enter_context(mut mpl: *mut MPL) {
    let mut image: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    if (*mpl).token == 201 as i32 {
        image = b"_|_\0" as *const u8 as *const i8 as *mut i8;
    } else if (*mpl).token == 205 as i32 {
        image = b"'...'\0" as *const u8 as *const i8 as *mut i8;
    } else {
        image = (*mpl).image;
    }
    (0 as i32 <= (*mpl).c_ptr && (*mpl).c_ptr < 60 as i32
        || {
            glp_assert_(
                b"0 <= mpl->c_ptr && mpl->c_ptr < CONTEXT_SIZE\0" as *const u8
                    as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                43 as i32,
            );
            1 as i32 != 0
        }) as i32;
    let fresh0 = (*mpl).c_ptr;
    (*mpl).c_ptr = (*mpl).c_ptr + 1;
    *((*mpl).context).offset(fresh0 as isize) = ' ' as i32 as i8;
    if (*mpl).c_ptr == 60 as i32 {
        (*mpl).c_ptr = 0 as i32;
    }
    s = image;
    while *s as i32 != '\0' as i32 {
        let fresh1 = (*mpl).c_ptr;
        (*mpl).c_ptr = (*mpl).c_ptr + 1;
        *((*mpl).context).offset(fresh1 as isize) = *s;
        if (*mpl).c_ptr == 60 as i32 {
            (*mpl).c_ptr = 0 as i32;
        }
        s = s.offset(1);
        s;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_print_context(mut mpl: *mut MPL) {
    let mut c: i32 = 0;
    while (*mpl).c_ptr > 0 as i32 {
        (*mpl).c_ptr -= 1;
        (*mpl).c_ptr;
        c = *((*mpl).context).offset(0 as i32 as isize) as i32;
        memmove(
            (*mpl).context as *mut libc::c_void,
            ((*mpl).context).offset(1 as i32 as isize) as *const libc::c_void,
            (60 as i32 - 1 as i32) as u64,
        );
        *((*mpl).context).offset((60 as i32 - 1 as i32) as isize) = c as i8;
    }
    glp_printf(
        b"Context: %s%.*s\n\0" as *const u8 as *const i8,
        if *((*mpl).context).offset(0 as i32 as isize) as i32 == ' ' as i32 {
            b"\0" as *const u8 as *const i8
        } else {
            b"...\0" as *const u8 as *const i8
        },
        60 as i32,
        (*mpl).context,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_char(mut mpl: *mut MPL) {
    let mut c: i32 = 0;
    if !((*mpl).c == -(1 as i32)) {
        if (*mpl).c == '\n' as i32 {
            (*mpl).line += 1;
            (*mpl).line;
        }
        c = _glp_mpl_read_char(mpl);
        if c == -(1 as i32) {
            if (*mpl).c == '\n' as i32 {
                (*mpl).line -= 1;
                (*mpl).line;
            } else {
                _glp_mpl_warning(
                    mpl,
                    b"final NL missing before end of file\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
        } else if !(c == '\n' as i32) {
            if *(*__ctype_b_loc()).offset(c as isize) as i32
                & _ISspace as i32 as libc::c_ushort as i32 != 0
            {
                c = ' ' as i32;
            } else if *(*__ctype_b_loc()).offset(c as isize) as i32
                & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                _glp_mpl_enter_context(mpl);
                _glp_mpl_error(
                    mpl,
                    b"control character 0x%02X not allowed\0" as *const u8 as *const i8
                        as *mut i8,
                    c,
                );
            }
        }
        (*mpl).c = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_append_char(mut mpl: *mut MPL) {
    (0 as i32 <= (*mpl).imlen && (*mpl).imlen <= 100 as i32
        || {
            glp_assert_(
                b"0 <= mpl->imlen && mpl->imlen <= MAX_LENGTH\0" as *const u8
                    as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                107 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).imlen == 100 as i32 {
        let mut current_block_9: u64;
        match (*mpl).token {
            202 => {
                _glp_mpl_enter_context(mpl);
                _glp_mpl_error(
                    mpl,
                    b"symbolic name %s... too long\0" as *const u8 as *const i8
                        as *mut i8,
                    (*mpl).image,
                );
                current_block_9 = 2583733423556193169;
            }
            203 => {
                current_block_9 = 2583733423556193169;
            }
            204 => {
                current_block_9 = 9079348504216822785;
            }
            205 => {
                current_block_9 = 13147206449030426259;
            }
            _ => {
                current_block_9 = 16186633624177175477;
            }
        }
        match current_block_9 {
            2583733423556193169 => {
                _glp_mpl_enter_context(mpl);
                _glp_mpl_error(
                    mpl,
                    b"symbol %s... too long\0" as *const u8 as *const i8 as *mut i8,
                    (*mpl).image,
                );
                current_block_9 = 9079348504216822785;
            }
            _ => {}
        }
        match current_block_9 {
            9079348504216822785 => {
                _glp_mpl_enter_context(mpl);
                _glp_mpl_error(
                    mpl,
                    b"numeric literal %s... too long\0" as *const u8 as *const i8
                        as *mut i8,
                    (*mpl).image,
                );
                current_block_9 = 13147206449030426259;
            }
            _ => {}
        }
        match current_block_9 {
            13147206449030426259 => {
                _glp_mpl_enter_context(mpl);
                _glp_mpl_error(
                    mpl,
                    b"string literal too long\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _ => {}
        }
        (mpl != mpl
            || {
                glp_assert_(
                    b"mpl != mpl\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    123 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    let fresh2 = (*mpl).imlen;
    (*mpl).imlen = (*mpl).imlen + 1;
    *((*mpl).image).offset(fresh2 as isize) = (*mpl).c as i8;
    *((*mpl).image).offset((*mpl).imlen as isize) = '\0' as i32 as i8;
    _glp_mpl_get_char(mpl);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_token(mut mpl: *mut MPL) {
    let mut current_block: u64;
    (*mpl).b_token = (*mpl).token;
    (*mpl).b_imlen = (*mpl).imlen;
    strcpy((*mpl).b_image, (*mpl).image);
    (*mpl).b_value = (*mpl).value;
    if (*mpl).f_scan != 0 {
        (*mpl).f_scan = 0 as i32;
        (*mpl).token = (*mpl).f_token;
        (*mpl).imlen = (*mpl).f_imlen;
        strcpy((*mpl).image, (*mpl).f_image);
        (*mpl).value = (*mpl).f_value;
    } else {
        loop {
            (*mpl).token = 0 as i32;
            (*mpl).imlen = 0 as i32;
            *((*mpl).image).offset(0 as i32 as isize) = '\0' as i32 as i8;
            (*mpl).value = 0.0f64;
            while (*mpl).c == ' ' as i32 || (*mpl).c == '\n' as i32 {
                _glp_mpl_get_char(mpl);
            }
            if (*mpl).c == -(1 as i32) {
                (*mpl).token = 201 as i32;
                current_block = 4466262843398566590;
                break;
            } else if (*mpl).c == '#' as i32 {
                while (*mpl).c != '\n' as i32 && (*mpl).c != -(1 as i32) {
                    _glp_mpl_get_char(mpl);
                }
            } else if (*mpl).flag_d == 0
                && (*(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISalpha as i32 as libc::c_ushort as i32 != 0
                    || (*mpl).c == '_' as i32)
            {
                (*mpl).token = 202 as i32;
                while *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISalnum as i32 as libc::c_ushort as i32 != 0
                    || (*mpl).c == '_' as i32
                {
                    _glp_mpl_append_char(mpl);
                }
                if strcmp((*mpl).image, b"and\0" as *const u8 as *const i8) == 0 as i32 {
                    (*mpl).token = 206 as i32;
                } else if strcmp((*mpl).image, b"by\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 207 as i32;
                } else if strcmp((*mpl).image, b"cross\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 208 as i32;
                } else if strcmp((*mpl).image, b"diff\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 209 as i32;
                } else if strcmp((*mpl).image, b"div\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 210 as i32;
                } else if strcmp((*mpl).image, b"else\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 211 as i32;
                } else if strcmp((*mpl).image, b"if\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 212 as i32;
                } else if strcmp((*mpl).image, b"in\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 213 as i32;
                } else if strcmp((*mpl).image, b"Infinity\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 214 as i32;
                } else if strcmp((*mpl).image, b"inter\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 215 as i32;
                } else if strcmp((*mpl).image, b"less\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 216 as i32;
                } else if strcmp((*mpl).image, b"mod\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 217 as i32;
                } else if strcmp((*mpl).image, b"not\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 218 as i32;
                } else if strcmp((*mpl).image, b"or\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 219 as i32;
                } else if strcmp((*mpl).image, b"s\0" as *const u8 as *const i8)
                    == 0 as i32 && (*mpl).c == '.' as i32
                {
                    let mut current_block_42: u64;
                    (*mpl).token = 220 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c != 't' as i32 {
                        current_block_42 = 10785700016822311470;
                    } else {
                        current_block_42 = 17784502470059252271;
                    }
                    loop {
                        match current_block_42 {
                            10785700016822311470 => {
                                _glp_mpl_enter_context(mpl);
                                _glp_mpl_error(
                                    mpl,
                                    b"keyword s.t. incomplete\0" as *const u8 as *const i8
                                        as *mut i8,
                                );
                                current_block_42 = 17784502470059252271;
                            }
                            _ => {
                                _glp_mpl_append_char(mpl);
                                if (*mpl).c != '.' as i32 {
                                    current_block_42 = 10785700016822311470;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    _glp_mpl_append_char(mpl);
                } else if strcmp((*mpl).image, b"symdiff\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 221 as i32;
                } else if strcmp((*mpl).image, b"then\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 222 as i32;
                } else if strcmp((*mpl).image, b"union\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 223 as i32;
                } else if strcmp((*mpl).image, b"within\0" as *const u8 as *const i8)
                    == 0 as i32
                {
                    (*mpl).token = 224 as i32;
                }
                current_block = 4466262843398566590;
                break;
            } else if (*mpl).flag_d == 0
                && *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0
            {
                (*mpl).token = 204 as i32;
                while *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    _glp_mpl_append_char(mpl);
                }
                if (*mpl).c == '.' as i32 {
                    current_block = 15594839951440953787;
                    break;
                } else {
                    current_block = 5684854171168229155;
                    break;
                }
            } else if (*mpl).c == '\'' as i32 || (*mpl).c == '"' as i32 {
                let mut quote: i32 = (*mpl).c;
                (*mpl).token = 205 as i32;
                _glp_mpl_get_char(mpl);
                loop {
                    if (*mpl).c == '\n' as i32 || (*mpl).c == -(1 as i32) {
                        _glp_mpl_enter_context(mpl);
                        _glp_mpl_error(
                            mpl,
                            b"unexpected end of line; string literal incomplete\0"
                                as *const u8 as *const i8 as *mut i8,
                        );
                    }
                    if (*mpl).c == quote {
                        _glp_mpl_get_char(mpl);
                        if (*mpl).c != quote {
                            break;
                        }
                    }
                    _glp_mpl_append_char(mpl);
                }
                current_block = 4466262843398566590;
                break;
            } else if (*mpl).flag_d == 0 && (*mpl).c == '+' as i32 {
                (*mpl).token = 225 as i32;
                _glp_mpl_append_char(mpl);
                current_block = 4466262843398566590;
                break;
            } else if (*mpl).flag_d == 0 && (*mpl).c == '-' as i32 {
                (*mpl).token = 226 as i32;
                _glp_mpl_append_char(mpl);
                current_block = 4466262843398566590;
                break;
            } else if (*mpl).c == '*' as i32 {
                (*mpl).token = 227 as i32;
                _glp_mpl_append_char(mpl);
                if (*mpl).c == '*' as i32 {
                    (*mpl).token = 229 as i32;
                    _glp_mpl_append_char(mpl);
                }
                current_block = 4466262843398566590;
                break;
            } else if (*mpl).c == '/' as i32 {
                (*mpl).token = 228 as i32;
                _glp_mpl_append_char(mpl);
                if !((*mpl).c == '*' as i32) {
                    current_block = 4466262843398566590;
                    break;
                }
                _glp_mpl_get_char(mpl);
                loop {
                    if (*mpl).c == -(1 as i32) {
                        _glp_mpl_error(
                            mpl,
                            b"unexpected end of file; comment sequence incomplete\0"
                                as *const u8 as *const i8 as *mut i8,
                        );
                    } else if (*mpl).c == '*' as i32 {
                        _glp_mpl_get_char(mpl);
                        if (*mpl).c == '/' as i32 {
                            break;
                        }
                    } else {
                        _glp_mpl_get_char(mpl);
                    }
                }
                _glp_mpl_get_char(mpl);
            } else if (*mpl).c == '^' as i32 {
                current_block = 13740693533991687037;
                break;
            } else {
                current_block = 14027225908442187354;
                break;
            }
        }
        match current_block {
            14027225908442187354 => {
                if (*mpl).c == '<' as i32 {
                    (*mpl).token = 230 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '=' as i32 {
                        (*mpl).token = 231 as i32;
                        _glp_mpl_append_char(mpl);
                    } else if (*mpl).c == '>' as i32 {
                        (*mpl).token = 235 as i32;
                        _glp_mpl_append_char(mpl);
                    } else if (*mpl).c == '-' as i32 {
                        (*mpl).token = 252 as i32;
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '=' as i32 {
                    (*mpl).token = 232 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '=' as i32 {
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '>' as i32 {
                    (*mpl).token = 234 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '=' as i32 {
                        (*mpl).token = 233 as i32;
                        _glp_mpl_append_char(mpl);
                    } else if (*mpl).c == '>' as i32 {
                        (*mpl).token = 250 as i32;
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '!' as i32 {
                    (*mpl).token = 218 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '=' as i32 {
                        (*mpl).token = 235 as i32;
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '&' as i32 {
                    (*mpl).token = 236 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '&' as i32 {
                        (*mpl).token = 206 as i32;
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '|' as i32 {
                    (*mpl).token = 237 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '|' as i32 {
                        (*mpl).token = 219 as i32;
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).flag_d == 0 && (*mpl).c == '.' as i32 {
                    (*mpl).token = 238 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).f_dots != 0 {
                        (*mpl).token = 243 as i32;
                        (*mpl).imlen = 2 as i32;
                        strcpy((*mpl).image, b"..\0" as *const u8 as *const i8);
                        (*mpl).f_dots = 0 as i32;
                        current_block = 4466262843398566590;
                    } else if (*mpl).c == '.' as i32 {
                        (*mpl).token = 243 as i32;
                        _glp_mpl_append_char(mpl);
                        current_block = 4466262843398566590;
                    } else if *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        (*mpl).token = 204 as i32;
                        _glp_mpl_append_char(mpl);
                        current_block = 2960454738949053742;
                    } else {
                        current_block = 4466262843398566590;
                    }
                } else if (*mpl).c == ',' as i32 {
                    (*mpl).token = 239 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == ':' as i32 {
                    (*mpl).token = 240 as i32;
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '=' as i32 {
                        (*mpl).token = 242 as i32;
                        _glp_mpl_append_char(mpl);
                    }
                    current_block = 4466262843398566590;
                } else if (*mpl).c == ';' as i32 {
                    (*mpl).token = 241 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '(' as i32 {
                    (*mpl).token = 244 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == ')' as i32 {
                    (*mpl).token = 245 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '[' as i32 {
                    (*mpl).token = 246 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == ']' as i32 {
                    (*mpl).token = 247 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '{' as i32 {
                    (*mpl).token = 248 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '}' as i32 {
                    (*mpl).token = 249 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if (*mpl).c == '~' as i32 {
                    (*mpl).token = 251 as i32;
                    _glp_mpl_append_char(mpl);
                    current_block = 4466262843398566590;
                } else if *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISalnum as i32 as libc::c_ushort as i32 != 0
                    || !(strchr(b"+-._\0" as *const u8 as *const i8, (*mpl).c)).is_null()
                {
                    ((*mpl).flag_d != 0
                        || {
                            glp_assert_(
                                b"mpl->flag_d\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                400 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*mpl).token = 203 as i32;
                    while *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                        & _ISalnum as i32 as libc::c_ushort as i32 != 0
                        || !(strchr(b"+-._\0" as *const u8 as *const i8, (*mpl).c))
                            .is_null()
                    {
                        _glp_mpl_append_char(mpl);
                    }
                    match _glp_str2num((*mpl).image, &mut (*mpl).value) {
                        0 => {
                            current_block = 12475473930704869784;
                            match current_block {
                                9433651810864827314 => {
                                    (mpl != mpl
                                        || {
                                            glp_assert_(
                                                b"mpl != mpl\0" as *const u8 as *const i8,
                                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                                413 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                                _ => {
                                    (*mpl).token = 204 as i32;
                                }
                            }
                            current_block = 4466262843398566590;
                        }
                        1 => {
                            current_block = 14565039439201253473;
                        }
                        2 => {
                            current_block = 4466262843398566590;
                        }
                        _ => {
                            current_block = 9433651810864827314;
                            match current_block {
                                9433651810864827314 => {
                                    (mpl != mpl
                                        || {
                                            glp_assert_(
                                                b"mpl != mpl\0" as *const u8 as *const i8,
                                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                                413 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                                _ => {
                                    (*mpl).token = 204 as i32;
                                }
                            }
                            current_block = 4466262843398566590;
                        }
                    }
                } else {
                    _glp_mpl_enter_context(mpl);
                    _glp_mpl_error(
                        mpl,
                        b"character %c not allowed\0" as *const u8 as *const i8
                            as *mut i8,
                        (*mpl).c,
                    );
                    current_block = 4466262843398566590;
                }
            }
            15594839951440953787 => {
                _glp_mpl_append_char(mpl);
                if (*mpl).c == '.' as i32 {
                    (*mpl).imlen -= 1;
                    (*mpl).imlen;
                    *((*mpl).image).offset((*mpl).imlen as isize) = '\0' as i32 as i8;
                    (*mpl).f_dots = 1 as i32;
                    current_block = 1288800219294318738;
                } else {
                    current_block = 2960454738949053742;
                }
            }
            13740693533991687037 => {
                (*mpl).token = 229 as i32;
                _glp_mpl_append_char(mpl);
                current_block = 4466262843398566590;
            }
            _ => {}
        }
        match current_block {
            2960454738949053742 => {
                while *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    _glp_mpl_append_char(mpl);
                }
                current_block = 5684854171168229155;
            }
            _ => {}
        }
        match current_block {
            5684854171168229155 => {
                if (*mpl).c == 'e' as i32 || (*mpl).c == 'E' as i32 {
                    _glp_mpl_append_char(mpl);
                    if (*mpl).c == '+' as i32 || (*mpl).c == '-' as i32 {
                        _glp_mpl_append_char(mpl);
                    }
                    if *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 == 0
                    {
                        _glp_mpl_enter_context(mpl);
                        _glp_mpl_error(
                            mpl,
                            b"numeric literal %s incomplete\0" as *const u8 as *const i8
                                as *mut i8,
                            (*mpl).image,
                        );
                    }
                    while *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        _glp_mpl_append_char(mpl);
                    }
                }
                if *(*__ctype_b_loc()).offset((*mpl).c as isize) as i32
                    & _ISalpha as i32 as libc::c_ushort as i32 != 0
                    || (*mpl).c == '_' as i32
                {
                    _glp_mpl_enter_context(mpl);
                    _glp_mpl_error(
                        mpl,
                        b"symbol %s%c... should be enclosed in quotes\0" as *const u8
                            as *const i8 as *mut i8,
                        (*mpl).image,
                        (*mpl).c,
                    );
                }
                current_block = 1288800219294318738;
            }
            _ => {}
        }
        match current_block {
            1288800219294318738 => {
                if _glp_str2num((*mpl).image, &mut (*mpl).value) != 0 {
                    current_block = 14565039439201253473;
                } else {
                    current_block = 4466262843398566590;
                }
            }
            _ => {}
        }
        match current_block {
            14565039439201253473 => {
                _glp_mpl_enter_context(mpl);
                _glp_mpl_error(
                    mpl,
                    b"cannot convert numeric literal %s to floating-point number\0"
                        as *const u8 as *const i8 as *mut i8,
                    (*mpl).image,
                );
            }
            _ => {}
        }
        _glp_mpl_enter_context(mpl);
        (*mpl).flag_x = 0 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_unget_token(mut mpl: *mut MPL) {
    ((*mpl).f_scan == 0
        || {
            glp_assert_(
                b"!mpl->f_scan\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                436 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*mpl).f_scan = 1 as i32;
    (*mpl).f_token = (*mpl).token;
    (*mpl).f_imlen = (*mpl).imlen;
    strcpy((*mpl).f_image, (*mpl).image);
    (*mpl).f_value = (*mpl).value;
    (*mpl).token = (*mpl).b_token;
    (*mpl).imlen = (*mpl).b_imlen;
    strcpy((*mpl).image, (*mpl).b_image);
    (*mpl).value = (*mpl).b_value;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_is_keyword(
    mut mpl: *mut MPL,
    mut keyword: *mut i8,
) -> i32 {
    return ((*mpl).token == 202 as i32 && strcmp((*mpl).image, keyword) == 0 as i32)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_is_reserved(mut mpl: *mut MPL) -> i32 {
    return ((*mpl).token == 206 as i32
        && *((*mpl).image).offset(0 as i32 as isize) as i32 == 'a' as i32
        || (*mpl).token == 207 as i32 || (*mpl).token == 208 as i32
        || (*mpl).token == 209 as i32 || (*mpl).token == 210 as i32
        || (*mpl).token == 211 as i32 || (*mpl).token == 212 as i32
        || (*mpl).token == 213 as i32 || (*mpl).token == 215 as i32
        || (*mpl).token == 216 as i32 || (*mpl).token == 217 as i32
        || (*mpl).token == 218 as i32
            && *((*mpl).image).offset(0 as i32 as isize) as i32 == 'n' as i32
        || (*mpl).token == 219 as i32
            && *((*mpl).image).offset(0 as i32 as isize) as i32 == 'o' as i32
        || (*mpl).token == 221 as i32 || (*mpl).token == 222 as i32
        || (*mpl).token == 223 as i32 || (*mpl).token == 224 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_make_code(
    mut mpl: *mut MPL,
    mut op: i32,
    mut arg: *mut OPERANDS,
    mut type_0: i32,
    mut dim: i32,
) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut domain: *mut DOMAIN1 = 0 as *mut DOMAIN1;
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
    code = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<CODE>() as u64 as i32)
        as *mut CODE;
    (*code).op = op;
    (*code).vflag = 0 as i32;
    memset(
        &mut (*code).arg as *mut OPERANDS as *mut libc::c_void,
        '?' as i32,
        ::core::mem::size_of::<OPERANDS>() as u64,
    );
    let mut current_block_121: u64;
    match op {
        301 => {
            (*code).arg.num = (*arg).num;
            current_block_121 = 7337917895049117968;
        }
        302 => {
            (*code).arg.str_0 = (*arg).str_0;
            current_block_121 = 7337917895049117968;
        }
        303 => {
            (*code).arg.index.slot = (*arg).index.slot;
            (*code).arg.index.next = (*arg).index.next;
            current_block_121 = 7337917895049117968;
        }
        304 | 305 => {
            e = (*arg).par.list;
            while !e.is_null() {
                (!((*e).x).is_null()
                    || {
                        glp_assert_(
                            b"e->x != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            521 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*e).x).up).is_null()
                    || {
                        glp_assert_(
                            b"e->x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            522 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*e).x).up = code;
                (*code).vflag |= (*(*e).x).vflag;
                e = (*e).next;
            }
            (*code).arg.par.par = (*arg).par.par;
            (*code).arg.par.list = (*arg).par.list;
            current_block_121 = 7337917895049117968;
        }
        306 => {
            e = (*arg).set.list;
            while !e.is_null() {
                (!((*e).x).is_null()
                    || {
                        glp_assert_(
                            b"e->x != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            531 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*e).x).up).is_null()
                    || {
                        glp_assert_(
                            b"e->x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            532 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*e).x).up = code;
                (*code).vflag |= (*(*e).x).vflag;
                e = (*e).next;
            }
            (*code).arg.set.set = (*arg).set.set;
            (*code).arg.set.list = (*arg).set.list;
            current_block_121 = 7337917895049117968;
        }
        307 => {
            e = (*arg).var.list;
            while !e.is_null() {
                (!((*e).x).is_null()
                    || {
                        glp_assert_(
                            b"e->x != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            541 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*e).x).up).is_null()
                    || {
                        glp_assert_(
                            b"e->x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            542 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*e).x).up = code;
                (*code).vflag |= (*(*e).x).vflag;
                e = (*e).next;
            }
            (*code).arg.var.var = (*arg).var.var;
            (*code).arg.var.list = (*arg).var.list;
            (*code).arg.var.suff = (*arg).var.suff;
            current_block_121 = 7337917895049117968;
        }
        308 => {
            e = (*arg).con.list;
            while !e.is_null() {
                (!((*e).x).is_null()
                    || {
                        glp_assert_(
                            b"e->x != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            555 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*e).x).up).is_null()
                    || {
                        glp_assert_(
                            b"e->x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            556 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*e).x).up = code;
                (*code).vflag |= (*(*e).x).vflag;
                e = (*e).next;
            }
            (*code).arg.con.con = (*arg).con.con;
            (*code).arg.con.list = (*arg).con.list;
            (*code).arg.con.suff = (*arg).con.suff;
            current_block_121 = 7337917895049117968;
        }
        309 | 310 => {
            e = (*arg).list;
            while !e.is_null() {
                (!((*e).x).is_null()
                    || {
                        glp_assert_(
                            b"e->x != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            568 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*e).x).up).is_null()
                    || {
                        glp_assert_(
                            b"e->x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            569 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*e).x).up = code;
                (*code).vflag |= (*(*e).x).vflag;
                e = (*e).next;
            }
            (*code).arg.list = (*arg).list;
            current_block_121 = 7337917895049117968;
        }
        311 => {
            (!((*arg).slice).is_null()
                || {
                    glp_assert_(
                        b"arg->slice != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        576 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*code).arg.slice = (*arg).slice;
            current_block_121 = 7337917895049117968;
        }
        312 | 313 | 314 | 315 => {
            (*code).vflag = 1 as i32;
            current_block_121 = 7337917895049117968;
        }
        316 | 317 | 318 | 319 | 320 | 321 | 322 | 323 | 324 | 325 | 326 | 327 | 328 | 329
        | 330 | 331 | 332 | 333 | 334 | 335 | 336 | 337 | 338 => {
            (!((*arg).arg.x).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.x != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        609 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (((*(*arg).arg.x).up).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.x->up == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        610 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*arg).arg.x).up = code;
            (*code).vflag |= (*(*arg).arg.x).vflag;
            (*code).arg.arg.x = (*arg).arg.x;
            current_block_121 = 7337917895049117968;
        }
        339 | 340 | 341 | 342 | 343 | 344 | 345 | 346 | 347 | 348 | 349 | 350 => {
            if op == 350 as i32 {
                (*code).vflag = 1 as i32;
            }
            current_block_121 = 6631021144877404942;
        }
        351 => {
            current_block_121 = 6631021144877404942;
        }
        352 | 353 | 354 | 355 | 356 | 357 | 358 | 359 | 360 | 361 | 362 | 363 | 364 | 365
        | 366 | 367 | 368 | 369 | 370 | 371 | 372 => {
            current_block_121 = 13104498352272130478;
        }
        373 | 374 | 375 => {
            (!((*arg).arg.x).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.x != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        667 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (((*(*arg).arg.x).up).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.x->up == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        668 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*arg).arg.x).up = code;
            (*code).vflag |= (*(*arg).arg.x).vflag;
            (!((*arg).arg.y).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.y != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        671 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (((*(*arg).arg.y).up).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.y->up == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        672 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*arg).arg.y).up = code;
            (*code).vflag |= (*(*arg).arg.y).vflag;
            if !((*arg).arg.z).is_null() {
                (((*(*arg).arg.z).up).is_null()
                    || {
                        glp_assert_(
                            b"arg->arg.z->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            676 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*arg).arg.z).up = code;
                (*code).vflag |= (*(*arg).arg.z).vflag;
            }
            (*code).arg.arg.x = (*arg).arg.x;
            (*code).arg.arg.y = (*arg).arg.y;
            (*code).arg.arg.z = (*arg).arg.z;
            current_block_121 = 7337917895049117968;
        }
        376 | 377 => {
            e = (*arg).list;
            while !e.is_null() {
                (!((*e).x).is_null()
                    || {
                        glp_assert_(
                            b"e->x != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            688 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*e).x).up).is_null()
                    || {
                        glp_assert_(
                            b"e->x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            689 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*e).x).up = code;
                (*code).vflag |= (*(*e).x).vflag;
                e = (*e).next;
            }
            (*code).arg.list = (*arg).list;
            current_block_121 = 7337917895049117968;
        }
        378 | 379 | 380 | 381 | 382 | 383 | 384 | 385 => {
            domain = (*arg).loop_0.domain;
            (!domain.is_null()
                || {
                    glp_assert_(
                        b"domain != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        705 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !((*domain).code).is_null() {
                (((*(*domain).code).up).is_null()
                    || {
                        glp_assert_(
                            b"domain->code->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            707 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*domain).code).up = code;
                (*code).vflag |= (*(*domain).code).vflag;
            }
            block = (*domain).list;
            while !block.is_null() {
                (!((*block).code).is_null()
                    || {
                        glp_assert_(
                            b"block->code != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            713 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (((*(*block).code).up).is_null()
                    || {
                        glp_assert_(
                            b"block->code->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            714 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*block).code).up = code;
                (*code).vflag |= (*(*block).code).vflag;
                block = (*block).next;
            }
            if !((*arg).loop_0.x).is_null() {
                (((*(*arg).loop_0.x).up).is_null()
                    || {
                        glp_assert_(
                            b"arg->loop.x->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            719 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*arg).loop_0.x).up = code;
                (*code).vflag |= (*(*arg).loop_0.x).vflag;
            }
            (*code).arg.loop_0.domain = (*arg).loop_0.domain;
            (*code).arg.loop_0.x = (*arg).loop_0.x;
            current_block_121 = 7337917895049117968;
        }
        _ => {
            (op != op
                || {
                    glp_assert_(
                        b"op != op\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        727 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block_121 = 7337917895049117968;
        }
    }
    match current_block_121 {
        6631021144877404942 => {
            if op == 351 as i32 {
                (*code).vflag = 1 as i32;
            }
            current_block_121 = 13104498352272130478;
        }
        _ => {}
    }
    match current_block_121 {
        13104498352272130478 => {
            (!((*arg).arg.x).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.x != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        652 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (((*(*arg).arg.x).up).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.x->up == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        653 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*arg).arg.x).up = code;
            (*code).vflag |= (*(*arg).arg.x).vflag;
            (!((*arg).arg.y).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.y != NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        656 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (((*(*arg).arg.y).up).is_null()
                || {
                    glp_assert_(
                        b"arg->arg.y->up == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        657 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*arg).arg.y).up = code;
            (*code).vflag |= (*(*arg).arg.y).vflag;
            (*code).arg.arg.x = (*arg).arg.x;
            (*code).arg.arg.y = (*arg).arg.y;
        }
        _ => {}
    }
    (*code).type_0 = type_0;
    (*code).dim = dim;
    (*code).up = 0 as *mut CODE;
    (*code).valid = 0 as i32;
    memset(
        &mut (*code).value as *mut VALUE as *mut libc::c_void,
        '?' as i32,
        ::core::mem::size_of::<VALUE>() as u64,
    );
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_make_unary(
    mut mpl: *mut MPL,
    mut op: i32,
    mut x: *mut CODE,
    mut type_0: i32,
    mut dim: i32,
) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    (!x.is_null()
        || {
            glp_assert_(
                b"x != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                746 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.arg.x = x;
    code = _glp_mpl_make_code(mpl, op, &mut arg, type_0, dim);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_make_binary(
    mut mpl: *mut MPL,
    mut op: i32,
    mut x: *mut CODE,
    mut y: *mut CODE,
    mut type_0: i32,
    mut dim: i32,
) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    (!x.is_null()
        || {
            glp_assert_(
                b"x != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                761 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!y.is_null()
        || {
            glp_assert_(
                b"y != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                762 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.arg.x = x;
    arg.arg.y = y;
    code = _glp_mpl_make_code(mpl, op, &mut arg, type_0, dim);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_make_ternary(
    mut mpl: *mut MPL,
    mut op: i32,
    mut x: *mut CODE,
    mut y: *mut CODE,
    mut z: *mut CODE,
    mut type_0: i32,
    mut dim: i32,
) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    (!x.is_null()
        || {
            glp_assert_(
                b"x != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                778 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!y.is_null()
        || {
            glp_assert_(
                b"y != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                779 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.arg.x = x;
    arg.arg.y = y;
    arg.arg.z = z;
    code = _glp_mpl_make_code(mpl, op, &mut arg, type_0, dim);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_numeric_literal(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    ((*mpl).token == 204 as i32
        || {
            glp_assert_(
                b"mpl->token == T_NUMBER\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                798 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.num = (*mpl).value;
    code = _glp_mpl_make_code(mpl, 301 as i32, &mut arg, 118 as i32, 0 as i32);
    _glp_mpl_get_token(mpl);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_string_literal(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    ((*mpl).token == 205 as i32
        || {
            glp_assert_(
                b"mpl->token == T_STRING\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                815 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.str_0 = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy(arg.str_0, (*mpl).image);
    code = _glp_mpl_make_code(mpl, 302 as i32, &mut arg, 124 as i32, 0 as i32);
    _glp_mpl_get_token(mpl);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_create_arg_list(mut mpl: *mut MPL) -> *mut ARG_LIST {
    let mut list: *mut ARG_LIST = 0 as *mut ARG_LIST;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                830 as i32,
            );
            1 as i32 != 0
        }) as i32;
    list = 0 as *mut ARG_LIST;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expand_arg_list(
    mut mpl: *mut MPL,
    mut list: *mut ARG_LIST,
    mut x: *mut CODE,
) -> *mut ARG_LIST {
    let mut tail: *mut ARG_LIST = 0 as *mut ARG_LIST;
    let mut temp: *mut ARG_LIST = 0 as *mut ARG_LIST;
    (!x.is_null()
        || {
            glp_assert_(
                b"x != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                842 as i32,
            );
            1 as i32 != 0
        }) as i32;
    tail = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<ARG_LIST>() as u64 as i32,
    ) as *mut ARG_LIST;
    (*tail).x = x;
    (*tail).next = 0 as *mut ARG_LIST;
    if list.is_null() {
        list = tail;
    } else {
        temp = list;
        while !((*temp).next).is_null() {
            temp = (*temp).next;
        }
        (*temp).next = tail;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_arg_list_len(
    mut mpl: *mut MPL,
    mut list: *mut ARG_LIST,
) -> i32 {
    let mut temp: *mut ARG_LIST = 0 as *mut ARG_LIST;
    let mut len: i32 = 0;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                865 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = 0 as i32;
    temp = list;
    while !temp.is_null() {
        len += 1;
        len;
        temp = (*temp).next;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_subscript_list(mut mpl: *mut MPL) -> *mut ARG_LIST {
    let mut list: *mut ARG_LIST = 0 as *mut ARG_LIST;
    let mut x: *mut CODE = 0 as *mut CODE;
    list = _glp_mpl_create_arg_list(mpl);
    loop {
        x = _glp_mpl_expression_5(mpl);
        if (*x).type_0 == 118 as i32 {
            x = _glp_mpl_make_unary(mpl, 317 as i32, x, 124 as i32, 0 as i32);
        }
        if (*x).type_0 != 124 as i32 {
            _glp_mpl_error(
                mpl,
                b"subscript expression has invalid type\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        ((*x).dim == 0 as i32
            || {
                glp_assert_(
                    b"x->dim == 0\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    893 as i32,
                );
                1 as i32 != 0
            }) as i32;
        list = _glp_mpl_expand_arg_list(mpl, list, x);
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else {
            if (*mpl).token == 247 as i32 {
                break;
            }
            _glp_mpl_error(
                mpl,
                b"syntax error in subscript list\0" as *const u8 as *const i8 as *mut i8,
            );
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_object_reference(mut mpl: *mut MPL) -> *mut CODE {
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut set: *mut SET = 0 as *mut SET;
    let mut par: *mut PARAMETER = 0 as *mut PARAMETER;
    let mut var: *mut VARIABLE = 0 as *mut VARIABLE;
    let mut con: *mut CONSTRAINT = 0 as *mut CONSTRAINT;
    let mut list: *mut ARG_LIST = 0 as *mut ARG_LIST;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut dim: i32 = 0;
    let mut suff: i32 = 0;
    ((*mpl).token == 202 as i32
        || {
            glp_assert_(
                b"mpl->token == T_NAME\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                944 as i32,
            );
            1 as i32 != 0
        }) as i32;
    node = _glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void);
    if node.is_null() {
        _glp_mpl_error(
            mpl,
            b"%s not defined\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    match _glp_avl_get_node_type(node) {
        111 => {
            slot = _glp_avl_get_node_link(node) as *mut DOMAIN_SLOT;
            name = (*slot).name;
            dim = 0 as i32;
        }
        122 => {
            set = _glp_avl_get_node_link(node) as *mut SET;
            name = (*set).name;
            dim = (*set).dim;
            if (*set).dimen == 0 as i32 {
                (*set).dimen = 1 as i32;
            }
        }
        120 => {
            par = _glp_avl_get_node_link(node) as *mut PARAMETER;
            name = (*par).name;
            dim = (*par).dim;
        }
        127 => {
            var = _glp_avl_get_node_link(node) as *mut VARIABLE;
            name = (*var).name;
            dim = (*var).dim;
        }
        103 => {
            con = _glp_avl_get_node_link(node) as *mut CONSTRAINT;
            name = (*con).name;
            dim = (*con).dim;
        }
        _ => {
            (node != node
                || {
                    glp_assert_(
                        b"node != node\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        985 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 246 as i32 {
        if dim == 0 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s cannot be subscripted\0" as *const u8 as *const i8 as *mut i8,
                name,
            );
        }
        _glp_mpl_get_token(mpl);
        list = _glp_mpl_subscript_list(mpl);
        if dim != _glp_mpl_arg_list_len(mpl, list) {
            _glp_mpl_error(
                mpl,
                b"%s must have %d subscript%s rather than %d\0" as *const u8 as *const i8
                    as *mut i8,
                name,
                dim,
                if dim == 1 as i32 {
                    b"\0" as *const u8 as *const i8
                } else {
                    b"s\0" as *const u8 as *const i8
                },
                _glp_mpl_arg_list_len(mpl, list),
            );
        }
        ((*mpl).token == 247 as i32
            || {
                glp_assert_(
                    b"mpl->token == T_RBRACKET\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    998 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_mpl_get_token(mpl);
    } else {
        if dim != 0 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s must be subscripted\0" as *const u8 as *const i8 as *mut i8,
                name,
            );
        }
        list = _glp_mpl_create_arg_list(mpl);
    }
    if (*mpl).flag_s == 0 && _glp_avl_get_node_type(node) == 127 as i32 {
        suff = 0 as i32;
    } else {
        suff = 0x4 as i32;
    }
    if (*mpl).token == 238 as i32 {
        _glp_mpl_get_token(mpl);
        if (*mpl).token != 202 as i32 {
            _glp_mpl_error(
                mpl,
                b"invalid use of period\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        if !(_glp_avl_get_node_type(node) == 127 as i32
            || _glp_avl_get_node_type(node) == 103 as i32)
        {
            _glp_mpl_error(
                mpl,
                b"%s cannot have a suffix\0" as *const u8 as *const i8 as *mut i8,
                name,
            );
        }
        if strcmp((*mpl).image, b"lb\0" as *const u8 as *const i8) == 0 as i32 {
            suff = 0x1 as i32;
        } else if strcmp((*mpl).image, b"ub\0" as *const u8 as *const i8) == 0 as i32 {
            suff = 0x2 as i32;
        } else if strcmp((*mpl).image, b"status\0" as *const u8 as *const i8) == 0 as i32
        {
            suff = 0x3 as i32;
        } else if strcmp((*mpl).image, b"val\0" as *const u8 as *const i8) == 0 as i32 {
            suff = 0x4 as i32;
        } else if strcmp((*mpl).image, b"dual\0" as *const u8 as *const i8) == 0 as i32 {
            suff = 0x5 as i32;
        } else {
            _glp_mpl_error(
                mpl,
                b"suffix .%s invalid\0" as *const u8 as *const i8 as *mut i8,
                (*mpl).image,
            );
        }
        _glp_mpl_get_token(mpl);
    }
    match _glp_avl_get_node_type(node) {
        111 => {
            arg.index.slot = slot;
            arg.index.next = (*slot).list;
            code = _glp_mpl_make_code(mpl, 303 as i32, &mut arg, 124 as i32, 0 as i32);
            (*slot).list = code;
        }
        122 => {
            arg.set.set = set;
            arg.set.list = list;
            code = _glp_mpl_make_code(
                mpl,
                306 as i32,
                &mut arg,
                106 as i32,
                (*set).dimen,
            );
        }
        120 => {
            arg.par.par = par;
            arg.par.list = list;
            if (*par).type_0 == 124 as i32 {
                code = _glp_mpl_make_code(
                    mpl,
                    305 as i32,
                    &mut arg,
                    124 as i32,
                    0 as i32,
                );
            } else {
                code = _glp_mpl_make_code(
                    mpl,
                    304 as i32,
                    &mut arg,
                    118 as i32,
                    0 as i32,
                );
            }
        }
        127 => {
            if (*mpl).flag_s == 0
                && (suff == 0x3 as i32 || suff == 0x4 as i32 || suff == 0x5 as i32)
            {
                _glp_mpl_error(
                    mpl,
                    b"invalid reference to status, primal value, or dual value of variable %s above solve statement\0"
                        as *const u8 as *const i8 as *mut i8,
                    (*var).name,
                );
            }
            arg.var.var = var;
            arg.var.list = list;
            arg.var.suff = suff;
            code = _glp_mpl_make_code(
                mpl,
                307 as i32,
                &mut arg,
                if suff == 0 as i32 { 110 as i32 } else { 118 as i32 },
                0 as i32,
            );
        }
        103 => {
            if (*mpl).flag_s == 0
                && (suff == 0x3 as i32 || suff == 0x4 as i32 || suff == 0x5 as i32)
            {
                _glp_mpl_error(
                    mpl,
                    b"invalid reference to status, primal value, or dual value of %s %s above solve statement\0"
                        as *const u8 as *const i8 as *mut i8,
                    if (*con).type_0 == 103 as i32 {
                        b"constraint\0" as *const u8 as *const i8
                    } else {
                        b"objective\0" as *const u8 as *const i8
                    },
                    (*con).name,
                );
            }
            arg.con.con = con;
            arg.con.list = list;
            arg.con.suff = suff;
            code = _glp_mpl_make_code(mpl, 308 as i32, &mut arg, 118 as i32, 0 as i32);
        }
        _ => {
            (node != node
                || {
                    glp_assert_(
                        b"node != node\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        1080 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_numeric_argument(
    mut mpl: *mut MPL,
    mut func: *mut i8,
) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_5(mpl);
    if (*x).type_0 == 124 as i32 {
        x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
    }
    if (*x).type_0 != 118 as i32 {
        _glp_mpl_error(
            mpl,
            b"argument for %s has invalid type\0" as *const u8 as *const i8 as *mut i8,
            func,
        );
    }
    ((*x).dim == 0 as i32
        || {
            glp_assert_(
                b"x->dim == 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1103 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_symbolic_argument(
    mut mpl: *mut MPL,
    mut func: *mut i8,
) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_5(mpl);
    if (*x).type_0 == 118 as i32 {
        x = _glp_mpl_make_unary(mpl, 317 as i32, x, 124 as i32, 0 as i32);
    }
    if (*x).type_0 != 124 as i32 {
        _glp_mpl_error(
            mpl,
            b"argument for %s has invalid type\0" as *const u8 as *const i8 as *mut i8,
            func,
        );
    }
    ((*x).dim == 0 as i32
        || {
            glp_assert_(
                b"x->dim == 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1117 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_elemset_argument(
    mut mpl: *mut MPL,
    mut func: *mut i8,
) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_9(mpl);
    if (*x).type_0 != 106 as i32 {
        _glp_mpl_error(
            mpl,
            b"argument for %s has invalid type\0" as *const u8 as *const i8 as *mut i8,
            func,
        );
    }
    ((*x).dim > 0 as i32
        || {
            glp_assert_(
                b"x->dim > 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1128 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_function_reference(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    let mut op: i32 = 0;
    let mut func: [i8; 16] = [0; 16];
    ((*mpl).token == 202 as i32
        || {
            glp_assert_(
                b"mpl->token == T_NAME\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1177 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if strcmp((*mpl).image, b"abs\0" as *const u8 as *const i8) == 0 as i32 {
        op = 324 as i32;
    } else if strcmp((*mpl).image, b"ceil\0" as *const u8 as *const i8) == 0 as i32 {
        op = 325 as i32;
    } else if strcmp((*mpl).image, b"floor\0" as *const u8 as *const i8) == 0 as i32 {
        op = 326 as i32;
    } else if strcmp((*mpl).image, b"exp\0" as *const u8 as *const i8) == 0 as i32 {
        op = 327 as i32;
    } else if strcmp((*mpl).image, b"log\0" as *const u8 as *const i8) == 0 as i32 {
        op = 328 as i32;
    } else if strcmp((*mpl).image, b"log10\0" as *const u8 as *const i8) == 0 as i32 {
        op = 329 as i32;
    } else if strcmp((*mpl).image, b"sqrt\0" as *const u8 as *const i8) == 0 as i32 {
        op = 330 as i32;
    } else if strcmp((*mpl).image, b"sin\0" as *const u8 as *const i8) == 0 as i32 {
        op = 331 as i32;
    } else if strcmp((*mpl).image, b"cos\0" as *const u8 as *const i8) == 0 as i32 {
        op = 332 as i32;
    } else if strcmp((*mpl).image, b"tan\0" as *const u8 as *const i8) == 0 as i32 {
        op = 333 as i32;
    } else if strcmp((*mpl).image, b"atan\0" as *const u8 as *const i8) == 0 as i32 {
        op = 334 as i32;
    } else if strcmp((*mpl).image, b"min\0" as *const u8 as *const i8) == 0 as i32 {
        op = 376 as i32;
    } else if strcmp((*mpl).image, b"max\0" as *const u8 as *const i8) == 0 as i32 {
        op = 377 as i32;
    } else if strcmp((*mpl).image, b"round\0" as *const u8 as *const i8) == 0 as i32 {
        op = 335 as i32;
    } else if strcmp((*mpl).image, b"trunc\0" as *const u8 as *const i8) == 0 as i32 {
        op = 336 as i32;
    } else if strcmp((*mpl).image, b"Irand224\0" as *const u8 as *const i8) == 0 as i32 {
        op = 312 as i32;
    } else if strcmp((*mpl).image, b"Uniform01\0" as *const u8 as *const i8) == 0 as i32
    {
        op = 313 as i32;
    } else if strcmp((*mpl).image, b"Uniform\0" as *const u8 as *const i8) == 0 as i32 {
        op = 350 as i32;
    } else if strcmp((*mpl).image, b"Normal01\0" as *const u8 as *const i8) == 0 as i32 {
        op = 314 as i32;
    } else if strcmp((*mpl).image, b"Normal\0" as *const u8 as *const i8) == 0 as i32 {
        op = 351 as i32;
    } else if strcmp((*mpl).image, b"card\0" as *const u8 as *const i8) == 0 as i32 {
        op = 337 as i32;
    } else if strcmp((*mpl).image, b"length\0" as *const u8 as *const i8) == 0 as i32 {
        op = 338 as i32;
    } else if strcmp((*mpl).image, b"substr\0" as *const u8 as *const i8) == 0 as i32 {
        op = 370 as i32;
    } else if strcmp((*mpl).image, b"str2time\0" as *const u8 as *const i8) == 0 as i32 {
        op = 371 as i32;
    } else if strcmp((*mpl).image, b"time2str\0" as *const u8 as *const i8) == 0 as i32 {
        op = 372 as i32;
    } else if strcmp((*mpl).image, b"gmtime\0" as *const u8 as *const i8) == 0 as i32 {
        op = 315 as i32;
    } else {
        _glp_mpl_error(
            mpl,
            b"function %s unknown\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    strcpy(func.as_mut_ptr(), (*mpl).image);
    (strlen(func.as_mut_ptr()) < ::core::mem::size_of::<[i8; 16]>() as u64
        || {
            glp_assert_(
                b"strlen(func) < sizeof(func)\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1234 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    ((*mpl).token == 244 as i32
        || {
            glp_assert_(
                b"mpl->token == T_LEFT\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1237 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    if op == 376 as i32 || op == 377 as i32 {
        arg.list = _glp_mpl_create_arg_list(mpl);
        loop {
            arg.list = _glp_mpl_expand_arg_list(
                mpl,
                arg.list,
                _glp_mpl_numeric_argument(mpl, func.as_mut_ptr()),
            );
            if (*mpl).token == 239 as i32 {
                _glp_mpl_get_token(mpl);
            } else {
                if (*mpl).token == 245 as i32 {
                    break;
                }
                _glp_mpl_error(
                    mpl,
                    b"syntax error in argument list for %s\0" as *const u8 as *const i8
                        as *mut i8,
                    func.as_mut_ptr(),
                );
            }
        }
    } else if op == 312 as i32 || op == 313 as i32 || op == 314 as i32
        || op == 315 as i32
    {
        if (*mpl).token != 245 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs no arguments\0" as *const u8 as *const i8 as *mut i8,
                func.as_mut_ptr(),
            );
        }
    } else if op == 350 as i32 || op == 351 as i32 {
        arg.arg.x = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        if !((*mpl).token == 239 as i32) {
            if (*mpl).token == 245 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"%s needs two arguments\0" as *const u8 as *const i8 as *mut i8,
                    func.as_mut_ptr(),
                );
            } else {
                _glp_mpl_error(
                    mpl,
                    b"syntax error in argument for %s\0" as *const u8 as *const i8
                        as *mut i8,
                    func.as_mut_ptr(),
                );
            }
        }
        _glp_mpl_get_token(mpl);
        arg.arg.y = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        if (*mpl).token == 239 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs two argument\0" as *const u8 as *const i8 as *mut i8,
                func.as_mut_ptr(),
            );
        } else if !((*mpl).token == 245 as i32) {
            _glp_mpl_error(
                mpl,
                b"syntax error in argument for %s\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        }
    } else if op == 334 as i32 || op == 335 as i32 || op == 336 as i32 {
        arg.arg.x = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        if (*mpl).token == 239 as i32 {
            match op {
                334 => {
                    op = 347 as i32;
                }
                335 => {
                    op = 348 as i32;
                }
                336 => {
                    op = 349 as i32;
                }
                _ => {
                    (op != op
                        || {
                            glp_assert_(
                                b"op != op\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                1295 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            _glp_mpl_get_token(mpl);
            arg.arg.y = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        }
        if (*mpl).token == 239 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs one or two arguments\0" as *const u8 as *const i8 as *mut i8,
                func.as_mut_ptr(),
            );
        } else if !((*mpl).token == 245 as i32) {
            _glp_mpl_error(
                mpl,
                b"syntax error in argument for %s\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        }
    } else if op == 370 as i32 {
        arg.arg.x = _glp_mpl_symbolic_argument(mpl, func.as_mut_ptr());
        if !((*mpl).token == 239 as i32) {
            if (*mpl).token == 245 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"%s needs two or three arguments\0" as *const u8 as *const i8
                        as *mut i8,
                    func.as_mut_ptr(),
                );
            } else {
                _glp_mpl_error(
                    mpl,
                    b"syntax error in argument for %s\0" as *const u8 as *const i8
                        as *mut i8,
                    func.as_mut_ptr(),
                );
            }
        }
        _glp_mpl_get_token(mpl);
        arg.arg.y = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        if (*mpl).token == 239 as i32 {
            op = 375 as i32;
            _glp_mpl_get_token(mpl);
            arg.arg.z = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        }
        if (*mpl).token == 239 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs two or three arguments\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        } else if !((*mpl).token == 245 as i32) {
            _glp_mpl_error(
                mpl,
                b"syntax error in argument for %s\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        }
    } else if op == 371 as i32 {
        arg.arg.x = _glp_mpl_symbolic_argument(mpl, func.as_mut_ptr());
        if !((*mpl).token == 239 as i32) {
            if (*mpl).token == 245 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"%s needs two arguments\0" as *const u8 as *const i8 as *mut i8,
                    func.as_mut_ptr(),
                );
            } else {
                _glp_mpl_error(
                    mpl,
                    b"syntax error in argument for %s\0" as *const u8 as *const i8
                        as *mut i8,
                    func.as_mut_ptr(),
                );
            }
        }
        _glp_mpl_get_token(mpl);
        arg.arg.y = _glp_mpl_symbolic_argument(mpl, func.as_mut_ptr());
        if (*mpl).token == 239 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs two argument\0" as *const u8 as *const i8 as *mut i8,
                func.as_mut_ptr(),
            );
        } else if !((*mpl).token == 245 as i32) {
            _glp_mpl_error(
                mpl,
                b"syntax error in argument for %s\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        }
    } else if op == 372 as i32 {
        arg.arg.x = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        if !((*mpl).token == 239 as i32) {
            if (*mpl).token == 245 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"%s needs two arguments\0" as *const u8 as *const i8 as *mut i8,
                    func.as_mut_ptr(),
                );
            } else {
                _glp_mpl_error(
                    mpl,
                    b"syntax error in argument for %s\0" as *const u8 as *const i8
                        as *mut i8,
                    func.as_mut_ptr(),
                );
            }
        }
        _glp_mpl_get_token(mpl);
        arg.arg.y = _glp_mpl_symbolic_argument(mpl, func.as_mut_ptr());
        if (*mpl).token == 239 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs two argument\0" as *const u8 as *const i8 as *mut i8,
                func.as_mut_ptr(),
            );
        } else if !((*mpl).token == 245 as i32) {
            _glp_mpl_error(
                mpl,
                b"syntax error in argument for %s\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        }
    } else {
        if op == 337 as i32 {
            arg.arg.x = _glp_mpl_elemset_argument(mpl, func.as_mut_ptr());
        } else if op == 338 as i32 {
            arg.arg.x = _glp_mpl_symbolic_argument(mpl, func.as_mut_ptr());
        } else {
            arg.arg.x = _glp_mpl_numeric_argument(mpl, func.as_mut_ptr());
        }
        if (*mpl).token == 239 as i32 {
            _glp_mpl_error(
                mpl,
                b"%s needs one argument\0" as *const u8 as *const i8 as *mut i8,
                func.as_mut_ptr(),
            );
        } else if !((*mpl).token == 245 as i32) {
            _glp_mpl_error(
                mpl,
                b"syntax error in argument for %s\0" as *const u8 as *const i8
                    as *mut i8,
                func.as_mut_ptr(),
            );
        }
    }
    if op == 370 as i32 || op == 375 as i32 || op == 372 as i32 {
        code = _glp_mpl_make_code(mpl, op, &mut arg, 124 as i32, 0 as i32);
    } else {
        code = _glp_mpl_make_code(mpl, op, &mut arg, 118 as i32, 0 as i32);
    }
    ((*mpl).token == 245 as i32
        || {
            glp_assert_(
                b"mpl->token == T_RIGHT\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1402 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_create_domain(mut mpl: *mut MPL) -> *mut DOMAIN1 {
    let mut domain: *mut DOMAIN1 = 0 as *mut DOMAIN1;
    domain = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<DOMAIN1>() as u64 as i32,
    ) as *mut DOMAIN1;
    (*domain).list = 0 as *mut DOMAIN_BLOCK;
    (*domain).code = 0 as *mut CODE;
    return domain;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_create_block(mut mpl: *mut MPL) -> *mut DOMAIN_BLOCK {
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    block = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<DOMAIN_BLOCK>() as u64 as i32,
    ) as *mut DOMAIN_BLOCK;
    (*block).list = 0 as *mut DOMAIN_SLOT;
    (*block).code = 0 as *mut CODE;
    (*block).backup = 0 as *mut TUPLE;
    (*block).next = 0 as *mut DOMAIN_BLOCK;
    return block;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_append_block(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
    mut block: *mut DOMAIN_BLOCK,
) {
    let mut temp: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1445 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!domain.is_null()
        || {
            glp_assert_(
                b"domain != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1446 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!block.is_null()
        || {
            glp_assert_(
                b"block != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1447 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (((*block).next).is_null()
        || {
            glp_assert_(
                b"block->next == NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1448 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ((*domain).list).is_null() {
        (*domain).list = block;
    } else {
        temp = (*domain).list;
        while !((*temp).next).is_null() {
            temp = (*temp).next;
        }
        (*temp).next = block;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_append_slot(
    mut mpl: *mut MPL,
    mut block: *mut DOMAIN_BLOCK,
    mut name: *mut i8,
    mut code: *mut CODE,
) -> *mut DOMAIN_SLOT {
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut temp: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    (!block.is_null()
        || {
            glp_assert_(
                b"block != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1476 as i32,
            );
            1 as i32 != 0
        }) as i32;
    slot = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<DOMAIN_SLOT>() as u64 as i32,
    ) as *mut DOMAIN_SLOT;
    (*slot).name = name;
    (*slot).code = code;
    (*slot).value = 0 as *mut SYMBOL;
    (*slot).list = 0 as *mut CODE;
    (*slot).next = 0 as *mut DOMAIN_SLOT;
    if ((*block).list).is_null() {
        (*block).list = slot;
    } else {
        temp = (*block).list;
        while !((*temp).next).is_null() {
            temp = (*temp).next;
        }
        (*temp).next = slot;
    }
    return slot;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_list(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    let mut list: [C2RustUnnamed_12; 21] = [C2RustUnnamed_12 {
        name: 0 as *mut i8,
        code: 0 as *mut CODE,
    }; 21];
    let mut flag_x: i32 = 0;
    let mut next_token: i32 = 0;
    let mut dim: i32 = 0;
    let mut j: i32 = 0;
    let mut slice: i32 = 0 as i32;
    ((*mpl).token == 244 as i32
        || {
            glp_assert_(
                b"mpl->token == T_LEFT\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1526 as i32,
            );
            1 as i32 != 0
        }) as i32;
    flag_x = (*mpl).flag_x;
    _glp_mpl_get_token(mpl);
    dim = 1 as i32;
    loop {
        if dim > 20 as i32 {
            _glp_mpl_error(
                mpl,
                b"too many components within parentheses\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        let mut current_block_31: u64;
        if (*mpl).token == 202 as i32 {
            _glp_mpl_get_token(mpl);
            next_token = (*mpl).token;
            _glp_mpl_unget_token(mpl);
            if !(flag_x != 0 && (next_token == 239 as i32 || next_token == 245 as i32)
                && (_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void))
                    .is_null())
            {
                current_block_31 = 133275390965565419;
            } else {
                j = 1 as i32;
                while j < dim {
                    if !(list[j as usize].name).is_null()
                        && strcmp(list[j as usize].name, (*mpl).image) == 0 as i32
                    {
                        _glp_mpl_error(
                            mpl,
                            b"duplicate dummy index %s not allowed\0" as *const u8
                                as *const i8 as *mut i8,
                            (*mpl).image,
                        );
                    }
                    j += 1;
                    j;
                }
                list[dim as usize].name = _glp_dmp_get_atom(
                    (*mpl).pool,
                    (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
                ) as *mut i8;
                strcpy(list[dim as usize].name, (*mpl).image);
                list[dim as usize].code = 0 as *mut CODE;
                _glp_mpl_get_token(mpl);
                slice = 1 as i32;
                if dim == 1 as i32 && (*mpl).token == 245 as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"%s not defined\0" as *const u8 as *const i8 as *mut i8,
                        list[dim as usize].name,
                    );
                }
                current_block_31 = 16924917904204750491;
            }
        } else {
            current_block_31 = 133275390965565419;
        }
        match current_block_31 {
            133275390965565419 => {
                code = _glp_mpl_expression_13(mpl);
                if (*mpl).token == 239 as i32 || dim > 1 as i32 {
                    if (*code).type_0 == 118 as i32 {
                        code = _glp_mpl_make_unary(
                            mpl,
                            317 as i32,
                            code,
                            124 as i32,
                            0 as i32,
                        );
                    }
                    if (*code).type_0 != 124 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"component expression has invalid type\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                    ((*code).dim == 0 as i32
                        || {
                            glp_assert_(
                                b"code->dim == 0\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                1588 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                list[dim as usize].name = 0 as *mut i8;
                list[dim as usize].code = code;
            }
            _ => {}
        }
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else {
            if (*mpl).token == 245 as i32 {
                break;
            }
            _glp_mpl_error(
                mpl,
                b"right parenthesis missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        dim += 1;
        dim;
    }
    if dim == 1 as i32 && slice == 0 {
        code = list[1 as i32 as usize].code;
    } else if slice == 0 {
        arg.list = _glp_mpl_create_arg_list(mpl);
        j = 1 as i32;
        while j <= dim {
            arg.list = _glp_mpl_expand_arg_list(mpl, arg.list, list[j as usize].code);
            j += 1;
            j;
        }
        code = _glp_mpl_make_code(mpl, 309 as i32, &mut arg, 126 as i32, dim);
    } else {
        arg.slice = _glp_mpl_create_block(mpl);
        j = 1 as i32;
        while j <= dim {
            _glp_mpl_append_slot(
                mpl,
                arg.slice,
                list[j as usize].name,
                list[j as usize].code,
            );
            j += 1;
            j;
        }
        code = _glp_mpl_make_code(mpl, 311 as i32, &mut arg, 126 as i32, dim);
    }
    _glp_mpl_get_token(mpl);
    if slice != 0 && (*mpl).token != 213 as i32 {
        _glp_mpl_error(
            mpl,
            b"keyword in missing where expected\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    if flag_x != 0 && (*mpl).token == 213 as i32 && slice == 0 {
        if dim == 1 as i32 {
            _glp_mpl_error(
                mpl,
                b"syntax error in indexing expression\0" as *const u8 as *const i8
                    as *mut i8,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"0-ary slice not allowed\0" as *const u8 as *const i8 as *mut i8,
            );
        }
    }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_literal_set(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> *mut CODE {
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    let mut j: i32 = 0;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1655 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.list = _glp_mpl_create_arg_list(mpl);
    j = 1 as i32;
    loop {
        if (*code).type_0 == 118 as i32 {
            code = _glp_mpl_make_unary(mpl, 317 as i32, code, 124 as i32, 0 as i32);
        }
        if (*code).type_0 == 124 as i32 {
            code = _glp_mpl_make_unary(mpl, 319 as i32, code, 126 as i32, 1 as i32);
        }
        if (*code).type_0 != 126 as i32 {
            _glp_mpl_error(
                mpl,
                b"member expression has invalid type\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        if !(arg.list).is_null() && (*(*arg.list).x).dim != (*code).dim {
            _glp_mpl_error(
                mpl,
                b"member %d has %d component%s while member %d has %d component%s\0"
                    as *const u8 as *const i8 as *mut i8,
                j - 1 as i32,
                (*(*arg.list).x).dim,
                if (*(*arg.list).x).dim == 1 as i32 {
                    b"\0" as *const u8 as *const i8
                } else {
                    b"s\0" as *const u8 as *const i8
                },
                j,
                (*code).dim,
                if (*code).dim == 1 as i32 {
                    b"\0" as *const u8 as *const i8
                } else {
                    b"s\0" as *const u8 as *const i8
                },
            );
        }
        arg.list = _glp_mpl_expand_arg_list(mpl, arg.list, code);
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else {
            if (*mpl).token == 249 as i32 {
                break;
            }
            _glp_mpl_error(
                mpl,
                b"syntax error in literal set\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        code = _glp_mpl_expression_5(mpl);
        j += 1;
        j;
    }
    code = _glp_mpl_make_code(
        mpl,
        310 as i32,
        &mut arg,
        106 as i32,
        (*(*arg.list).x).dim,
    );
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_indexing_expression(
    mut mpl: *mut MPL,
) -> *mut DOMAIN1 {
    let mut domain: *mut DOMAIN1 = 0 as *mut DOMAIN1;
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut code: *mut CODE = 0 as *mut CODE;
    ((*mpl).token == 248 as i32
        || {
            glp_assert_(
                b"mpl->token == T_LBRACE\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1718 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 249 as i32 {
        _glp_mpl_error(
            mpl,
            b"empty indexing expression not allowed\0" as *const u8 as *const i8
                as *mut i8,
        );
    }
    domain = _glp_mpl_create_domain(mpl);
    loop {
        block = 0 as *mut DOMAIN_BLOCK;
        code = 0 as *mut CODE;
        if (*mpl).token == 202 as i32 {
            let mut next_token: i32 = 0;
            let mut name: *mut i8 = 0 as *mut i8;
            _glp_mpl_get_token(mpl);
            next_token = (*mpl).token;
            _glp_mpl_unget_token(mpl);
            if next_token == 213 as i32
                && (_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void))
                    .is_null()
            {
                block = _glp_mpl_create_block(mpl);
                name = _glp_dmp_get_atom(
                    (*mpl).pool,
                    (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
                ) as *mut i8;
                strcpy(name, (*mpl).image);
                _glp_mpl_append_slot(mpl, block, name, 0 as *mut CODE);
                _glp_mpl_get_token(mpl);
                ((*mpl).token == 213 as i32
                    || {
                        glp_assert_(
                            b"mpl->token == T_IN\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            1756 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                _glp_mpl_get_token(mpl);
            }
        } else if (*mpl).token == 244 as i32 {
            (*mpl).flag_x = 1 as i32;
            code = _glp_mpl_expression_9(mpl);
            if !((*code).op != 311 as i32) {
                block = (*code).arg.slice;
                code = 0 as *mut CODE;
                ((*mpl).token == 213 as i32
                    || {
                        glp_assert_(
                            b"mpl->token == T_IN\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            1778 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                _glp_mpl_get_token(mpl);
            }
        }
        if code.is_null() {
            code = _glp_mpl_expression_9(mpl);
        }
        if (*code).type_0 != 106 as i32 {
            if !block.is_null() {
                _glp_mpl_error(
                    mpl,
                    b"domain expression has invalid type\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
            code = _glp_mpl_literal_set(mpl, code);
        }
        (!code.is_null()
            || {
                glp_assert_(
                    b"code != NULL\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    1804 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*code).type_0 == 106 as i32
            || {
                glp_assert_(
                    b"code->type == A_ELEMSET\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    1805 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*code).dim > 0 as i32
            || {
                glp_assert_(
                    b"code->dim > 0\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    1806 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if block.is_null() {
            let mut j: i32 = 0;
            block = _glp_mpl_create_block(mpl);
            j = 1 as i32;
            while j <= (*code).dim {
                _glp_mpl_append_slot(mpl, block, 0 as *mut i8, 0 as *mut CODE);
                j += 1;
                j;
            }
        }
        let mut dim: i32 = 0 as i32;
        slot = (*block).list;
        while !slot.is_null() {
            dim += 1;
            dim;
            slot = (*slot).next;
        }
        if dim != (*code).dim {
            _glp_mpl_error(
                mpl,
                b"%d %s specified for set of dimension %d\0" as *const u8 as *const i8
                    as *mut i8,
                dim,
                if dim == 1 as i32 {
                    b"index\0" as *const u8 as *const i8
                } else {
                    b"indices\0" as *const u8 as *const i8
                },
                (*code).dim,
            );
        }
        (((*block).code).is_null()
            || {
                glp_assert_(
                    b"block->code == NULL\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    1826 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*block).code = code;
        _glp_mpl_append_block(mpl, domain, block);
        slot = (*block).list;
        while !slot.is_null() {
            if !((*slot).name).is_null() {
                let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
                ((_glp_avl_find_node((*mpl).tree, (*slot).name as *const libc::c_void))
                    .is_null()
                    || {
                        glp_assert_(
                            b"avl_find_node(mpl->tree, slot->name) == NULL\0"
                                as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            1838 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                node = _glp_avl_insert_node(
                    (*mpl).tree,
                    (*slot).name as *const libc::c_void,
                );
                _glp_avl_set_node_type(node, 111 as i32);
                _glp_avl_set_node_link(node, slot as *mut libc::c_void);
            }
            slot = (*slot).next;
        }
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else {
            if (*mpl).token == 240 as i32 || (*mpl).token == 249 as i32 {
                break;
            }
            _glp_mpl_error(
                mpl,
                b"syntax error in indexing expression\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if (*mpl).token == 240 as i32 {
        _glp_mpl_get_token(mpl);
        code = _glp_mpl_expression_13(mpl);
        if (*code).type_0 == 124 as i32 {
            code = _glp_mpl_make_unary(mpl, 316 as i32, code, 118 as i32, 0 as i32);
        }
        if (*code).type_0 == 118 as i32 {
            code = _glp_mpl_make_unary(mpl, 318 as i32, code, 114 as i32, 0 as i32);
        }
        if (*code).type_0 != 114 as i32 {
            _glp_mpl_error(
                mpl,
                b"expression following colon has invalid type\0" as *const u8
                    as *const i8 as *mut i8,
            );
        }
        ((*code).dim == 0 as i32
            || {
                glp_assert_(
                    b"code->dim == 0\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    1863 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*domain).code = code;
        if (*mpl).token != 249 as i32 {
            _glp_mpl_error(
                mpl,
                b"syntax error in indexing expression\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    _glp_mpl_get_token(mpl);
    return domain;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_close_scope(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
) {
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    (!domain.is_null()
        || {
            glp_assert_(
                b"domain != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1884 as i32,
            );
            1 as i32 != 0
        }) as i32;
    block = (*domain).list;
    while !block.is_null() {
        slot = (*block).list;
        while !slot.is_null() {
            if !((*slot).name).is_null() {
                node = _glp_avl_find_node(
                    (*mpl).tree,
                    (*slot).name as *const libc::c_void,
                );
                (!node.is_null()
                    || {
                        glp_assert_(
                            b"node != NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            1890 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (_glp_avl_get_node_type(node) == 111 as i32
                    || {
                        glp_assert_(
                            b"avl_get_node_type(node) == A_INDEX\0" as *const u8
                                as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            1891 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                _glp_avl_delete_node((*mpl).tree, node);
            }
            slot = (*slot).next;
        }
        block = (*block).next;
    }
}
unsafe extern "C" fn link_up(mut code: *mut CODE) {
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    block = (*(*code).arg.loop_0.domain).list;
    while !block.is_null() {
        slot = (*block).list;
        while !slot.is_null() {
            if !((*slot).code).is_null() {
                (((*(*slot).code).up).is_null()
                    || {
                        glp_assert_(
                            b"slot->code->up == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            1931 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*slot).code).up = code;
            }
            slot = (*slot).next;
        }
        block = (*block).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_iterated_expression(mut mpl: *mut MPL) -> *mut CODE {
    let mut current_block: u64;
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    let mut op: i32 = 0;
    let mut opstr: [i8; 8] = [0; 8];
    ((*mpl).token == 202 as i32
        || {
            glp_assert_(
                b"mpl->token == T_NAME\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1946 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if strcmp((*mpl).image, b"sum\0" as *const u8 as *const i8) == 0 as i32 {
        op = 378 as i32;
    } else if strcmp((*mpl).image, b"prod\0" as *const u8 as *const i8) == 0 as i32 {
        op = 379 as i32;
    } else if strcmp((*mpl).image, b"min\0" as *const u8 as *const i8) == 0 as i32 {
        op = 380 as i32;
    } else if strcmp((*mpl).image, b"max\0" as *const u8 as *const i8) == 0 as i32 {
        op = 381 as i32;
    } else if strcmp((*mpl).image, b"forall\0" as *const u8 as *const i8) == 0 as i32 {
        op = 382 as i32;
    } else if strcmp((*mpl).image, b"exists\0" as *const u8 as *const i8) == 0 as i32 {
        op = 383 as i32;
    } else if strcmp((*mpl).image, b"setof\0" as *const u8 as *const i8) == 0 as i32 {
        op = 384 as i32;
    } else {
        _glp_mpl_error(
            mpl,
            b"operator %s unknown\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    strcpy(opstr.as_mut_ptr(), (*mpl).image);
    (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 8]>() as u64
        || {
            glp_assert_(
                b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1964 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    ((*mpl).token == 248 as i32
        || {
            glp_assert_(
                b"mpl->token == T_LBRACE\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                1967 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arg.loop_0.domain = _glp_mpl_indexing_expression(mpl);
    match op {
        378 | 379 | 380 | 381 => {
            arg.loop_0.x = _glp_mpl_expression_3(mpl);
            if (*arg.loop_0.x).type_0 == 124 as i32 {
                arg.loop_0.x = _glp_mpl_make_unary(
                    mpl,
                    316 as i32,
                    arg.loop_0.x,
                    118 as i32,
                    0 as i32,
                );
            }
            if !((*arg.loop_0.x).type_0 == 118 as i32
                || op == 378 as i32 && (*arg.loop_0.x).type_0 == 110 as i32)
            {
                current_block = 411102616872665082;
            } else {
                current_block = 13472856163611868459;
            }
        }
        382 | 383 => {
            arg.loop_0.x = _glp_mpl_expression_12(mpl);
            if (*arg.loop_0.x).type_0 == 124 as i32 {
                arg.loop_0.x = _glp_mpl_make_unary(
                    mpl,
                    316 as i32,
                    arg.loop_0.x,
                    118 as i32,
                    0 as i32,
                );
            }
            if (*arg.loop_0.x).type_0 == 118 as i32 {
                arg.loop_0.x = _glp_mpl_make_unary(
                    mpl,
                    318 as i32,
                    arg.loop_0.x,
                    114 as i32,
                    0 as i32,
                );
            }
            if (*arg.loop_0.x).type_0 != 114 as i32 {
                current_block = 411102616872665082;
            } else {
                ((*arg.loop_0.x).dim == 0 as i32
                    || {
                        glp_assert_(
                            b"arg.loop.x->dim == 0\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            2003 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                code = _glp_mpl_make_code(mpl, op, &mut arg, 114 as i32, 0 as i32);
                current_block = 12199444798915819164;
            }
        }
        384 => {
            arg.loop_0.x = _glp_mpl_expression_5(mpl);
            if (*arg.loop_0.x).type_0 == 118 as i32 {
                arg.loop_0.x = _glp_mpl_make_unary(
                    mpl,
                    317 as i32,
                    arg.loop_0.x,
                    124 as i32,
                    0 as i32,
                );
            }
            if (*arg.loop_0.x).type_0 == 124 as i32 {
                arg.loop_0.x = _glp_mpl_make_unary(
                    mpl,
                    319 as i32,
                    arg.loop_0.x,
                    126 as i32,
                    1 as i32,
                );
            }
            if (*arg.loop_0.x).type_0 != 126 as i32 {
                current_block = 411102616872665082;
            } else {
                ((*arg.loop_0.x).dim > 0 as i32
                    || {
                        glp_assert_(
                            b"arg.loop.x->dim > 0\0" as *const u8 as *const i8,
                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                            2018 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                code = _glp_mpl_make_code(
                    mpl,
                    op,
                    &mut arg,
                    106 as i32,
                    (*arg.loop_0.x).dim,
                );
                current_block = 12199444798915819164;
            }
        }
        _ => {
            (op != op
                || {
                    glp_assert_(
                        b"op != op\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        2023 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block = 12199444798915819164;
        }
    }
    match current_block {
        411102616872665082 => {
            _glp_mpl_error(
                mpl,
                b"integrand following %s{...} has invalid type\0" as *const u8
                    as *const i8 as *mut i8,
                opstr.as_mut_ptr(),
            );
            current_block = 13472856163611868459;
        }
        _ => {}
    }
    match current_block {
        13472856163611868459 => {
            ((*arg.loop_0.x).dim == 0 as i32
                || {
                    glp_assert_(
                        b"arg.loop.x->dim == 0\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        1987 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            code = _glp_mpl_make_code(
                mpl,
                op,
                &mut arg,
                (*arg.loop_0.x).type_0,
                0 as i32,
            );
        }
        _ => {}
    }
    _glp_mpl_close_scope(mpl, arg.loop_0.domain);
    link_up(code);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_domain_arity(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
) -> i32 {
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut arity: i32 = 0;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                2043 as i32,
            );
            1 as i32 != 0
        }) as i32;
    arity = 0 as i32;
    block = (*domain).list;
    while !block.is_null() {
        slot = (*block).list;
        while !slot.is_null() {
            if ((*slot).code).is_null() {
                arity += 1;
                arity;
            }
            slot = (*slot).next;
        }
        block = (*block).next;
    }
    return arity;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_set_expression(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut arg: OPERANDS = OPERANDS { num: 0. };
    ((*mpl).token == 248 as i32
        || {
            glp_assert_(
                b"mpl->token == T_LBRACE\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                2062 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 249 as i32 {
        arg.list = 0 as *mut ARG_LIST;
        code = _glp_mpl_make_code(mpl, 310 as i32, &mut arg, 106 as i32, 1 as i32);
        _glp_mpl_get_token(mpl);
    } else {
        _glp_mpl_unget_token(mpl);
        arg.loop_0.domain = _glp_mpl_indexing_expression(mpl);
        arg.loop_0.x = 0 as *mut CODE;
        _glp_mpl_close_scope(mpl, arg.loop_0.domain);
        code = _glp_mpl_make_code(
            mpl,
            385 as i32,
            &mut arg,
            106 as i32,
            _glp_mpl_domain_arity(mpl, arg.loop_0.domain),
        );
        link_up(code);
    }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_branched_expression(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    let mut z: *mut CODE = 0 as *mut CODE;
    ((*mpl).token == 212 as i32
        || {
            glp_assert_(
                b"mpl->token == T_IF\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                2103 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    x = _glp_mpl_expression_13(mpl);
    if (*x).type_0 == 124 as i32 {
        x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
    }
    if (*x).type_0 == 118 as i32 {
        x = _glp_mpl_make_unary(mpl, 318 as i32, x, 114 as i32, 0 as i32);
    }
    if (*x).type_0 != 114 as i32 {
        _glp_mpl_error(
            mpl,
            b"expression following if has invalid type\0" as *const u8 as *const i8
                as *mut i8,
        );
    }
    ((*x).dim == 0 as i32
        || {
            glp_assert_(
                b"x->dim == 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                2115 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).token != 222 as i32 {
        _glp_mpl_error(
            mpl,
            b"keyword then missing where expected\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    y = _glp_mpl_expression_9(mpl);
    if !((*y).type_0 == 118 as i32 || (*y).type_0 == 124 as i32
        || (*y).type_0 == 106 as i32 || (*y).type_0 == 110 as i32)
    {
        _glp_mpl_error(
            mpl,
            b"expression following then has invalid type\0" as *const u8 as *const i8
                as *mut i8,
        );
    }
    if (*mpl).token != 211 as i32 {
        if (*y).type_0 == 106 as i32 {
            _glp_mpl_error(
                mpl,
                b"keyword else missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        z = 0 as *mut CODE;
    } else {
        _glp_mpl_get_token(mpl);
        z = _glp_mpl_expression_9(mpl);
        if !((*z).type_0 == 118 as i32 || (*z).type_0 == 124 as i32
            || (*z).type_0 == 106 as i32 || (*z).type_0 == 110 as i32)
        {
            _glp_mpl_error(
                mpl,
                b"expression following else has invalid type\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        if (*y).type_0 == 110 as i32 || (*z).type_0 == 110 as i32 {
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if (*y).type_0 == 118 as i32 {
                y = _glp_mpl_make_unary(mpl, 320 as i32, y, 110 as i32, 0 as i32);
            }
            if (*z).type_0 == 124 as i32 {
                z = _glp_mpl_make_unary(mpl, 316 as i32, z, 118 as i32, 0 as i32);
            }
            if (*z).type_0 == 118 as i32 {
                z = _glp_mpl_make_unary(mpl, 320 as i32, z, 110 as i32, 0 as i32);
            }
        }
        if (*y).type_0 == 124 as i32 || (*z).type_0 == 124 as i32 {
            if (*y).type_0 == 118 as i32 {
                y = _glp_mpl_make_unary(mpl, 317 as i32, y, 124 as i32, 0 as i32);
            }
            if (*z).type_0 == 118 as i32 {
                z = _glp_mpl_make_unary(mpl, 317 as i32, z, 124 as i32, 0 as i32);
            }
        }
        if (*y).type_0 != (*z).type_0 {
            _glp_mpl_error(
                mpl,
                b"expressions following then and else have incompatible types\0"
                    as *const u8 as *const i8 as *mut i8,
            );
        }
        if (*y).dim != (*z).dim {
            _glp_mpl_error(
                mpl,
                b"expressions following then and else have different dimensions %d and %d, respectively\0"
                    as *const u8 as *const i8 as *mut i8,
                (*y).dim,
                (*z).dim,
            );
        }
    }
    code = _glp_mpl_make_ternary(mpl, 374 as i32, x, y, z, (*y).type_0, (*y).dim);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_primary_expression(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    if (*mpl).token == 204 as i32 {
        code = _glp_mpl_numeric_literal(mpl);
    } else if (*mpl).token == 214 as i32 {
        let mut arg: OPERANDS = OPERANDS { num: 0. };
        arg.num = 1.7976931348623157e+308f64;
        code = _glp_mpl_make_code(mpl, 301 as i32, &mut arg, 118 as i32, 0 as i32);
        _glp_mpl_get_token(mpl);
    } else if (*mpl).token == 205 as i32 {
        code = _glp_mpl_string_literal(mpl);
    } else if (*mpl).token == 202 as i32 {
        let mut next_token: i32 = 0;
        _glp_mpl_get_token(mpl);
        next_token = (*mpl).token;
        _glp_mpl_unget_token(mpl);
        match next_token {
            246 => {
                code = _glp_mpl_object_reference(mpl);
            }
            244 => {
                code = _glp_mpl_function_reference(mpl);
            }
            248 => {
                code = _glp_mpl_iterated_expression(mpl);
            }
            _ => {
                code = _glp_mpl_object_reference(mpl);
            }
        }
    } else if (*mpl).token == 244 as i32 {
        code = _glp_mpl_expression_list(mpl);
    } else if (*mpl).token == 248 as i32 {
        code = _glp_mpl_set_expression(mpl);
    } else if (*mpl).token == 212 as i32 {
        code = _glp_mpl_branched_expression(mpl);
    } else if _glp_mpl_is_reserved(mpl) != 0 {
        _glp_mpl_error(
            mpl,
            b"invalid use of reserved keyword %s\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    } else {
        _glp_mpl_error(
            mpl,
            b"syntax error in expression\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_error_preceding(
    mut mpl: *mut MPL,
    mut opstr: *mut i8,
) {
    _glp_mpl_error(
        mpl,
        b"operand preceding %s has invalid type\0" as *const u8 as *const i8 as *mut i8,
        opstr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_error_following(
    mut mpl: *mut MPL,
    mut opstr: *mut i8,
) {
    _glp_mpl_error(
        mpl,
        b"operand following %s has invalid type\0" as *const u8 as *const i8 as *mut i8,
        opstr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_error_dimension(
    mut mpl: *mut MPL,
    mut opstr: *mut i8,
    mut dim1: i32,
    mut dim2: i32,
) {
    _glp_mpl_error(
        mpl,
        b"operands preceding and following %s have different dimensions %d and %d, respectively\0"
            as *const u8 as *const i8 as *mut i8,
        opstr,
        dim1,
        dim2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_0(mut mpl: *mut MPL) -> *mut CODE {
    let mut code: *mut CODE = 0 as *mut CODE;
    code = _glp_mpl_primary_expression(mpl);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_1(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    let mut opstr: [i8; 8] = [0; 8];
    x = _glp_mpl_expression_0(mpl);
    if (*mpl).token == 229 as i32 {
        strcpy(opstr.as_mut_ptr(), (*mpl).image);
        (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 8]>() as u64
            || {
                glp_assert_(
                    b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    2323 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if (*x).type_0 != 118 as i32 {
            _glp_mpl_error_preceding(mpl, opstr.as_mut_ptr());
        }
        _glp_mpl_get_token(mpl);
        if (*mpl).token == 225 as i32 || (*mpl).token == 226 as i32 {
            y = _glp_mpl_expression_2(mpl);
        } else {
            y = _glp_mpl_expression_1(mpl);
        }
        if (*y).type_0 == 124 as i32 {
            y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
        }
        if (*y).type_0 != 118 as i32 {
            _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
        }
        x = _glp_mpl_make_binary(mpl, 346 as i32, x, y, 118 as i32, 0 as i32);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_2(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    if (*mpl).token == 225 as i32 {
        _glp_mpl_get_token(mpl);
        x = _glp_mpl_expression_1(mpl);
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if !((*x).type_0 == 118 as i32 || (*x).type_0 == 110 as i32) {
            _glp_mpl_error_following(mpl, b"+\0" as *const u8 as *const i8 as *mut i8);
        }
        x = _glp_mpl_make_unary(mpl, 321 as i32, x, (*x).type_0, 0 as i32);
    } else if (*mpl).token == 226 as i32 {
        _glp_mpl_get_token(mpl);
        x = _glp_mpl_expression_1(mpl);
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if !((*x).type_0 == 118 as i32 || (*x).type_0 == 110 as i32) {
            _glp_mpl_error_following(mpl, b"-\0" as *const u8 as *const i8 as *mut i8);
        }
        x = _glp_mpl_make_unary(mpl, 322 as i32, x, (*x).type_0, 0 as i32);
    } else {
        x = _glp_mpl_expression_1(mpl);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_3(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_2(mpl);
    loop {
        if (*mpl).token == 227 as i32 {
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if !((*x).type_0 == 118 as i32 || (*x).type_0 == 110 as i32) {
                _glp_mpl_error_preceding(
                    mpl,
                    b"*\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_2(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if !((*y).type_0 == 118 as i32 || (*y).type_0 == 110 as i32) {
                _glp_mpl_error_following(
                    mpl,
                    b"*\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).type_0 == 110 as i32 && (*y).type_0 == 110 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"multiplication of linear forms not allowed\0" as *const u8
                        as *const i8 as *mut i8,
                );
            }
            if (*x).type_0 == 118 as i32 && (*y).type_0 == 118 as i32 {
                x = _glp_mpl_make_binary(mpl, 342 as i32, x, y, 118 as i32, 0 as i32);
            } else {
                x = _glp_mpl_make_binary(mpl, 342 as i32, x, y, 110 as i32, 0 as i32);
            }
        } else if (*mpl).token == 228 as i32 {
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if !((*x).type_0 == 118 as i32 || (*x).type_0 == 110 as i32) {
                _glp_mpl_error_preceding(
                    mpl,
                    b"/\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_2(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if (*y).type_0 != 118 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"/\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).type_0 == 118 as i32 {
                x = _glp_mpl_make_binary(mpl, 343 as i32, x, y, 118 as i32, 0 as i32);
            } else {
                x = _glp_mpl_make_binary(mpl, 343 as i32, x, y, 110 as i32, 0 as i32);
            }
        } else if (*mpl).token == 210 as i32 {
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if (*x).type_0 != 118 as i32 {
                _glp_mpl_error_preceding(
                    mpl,
                    b"div\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_2(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if (*y).type_0 != 118 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"div\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            x = _glp_mpl_make_binary(mpl, 344 as i32, x, y, 118 as i32, 0 as i32);
        } else {
            if !((*mpl).token == 217 as i32) {
                break;
            }
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if (*x).type_0 != 118 as i32 {
                _glp_mpl_error_preceding(
                    mpl,
                    b"mod\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_2(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if (*y).type_0 != 118 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"mod\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            x = _glp_mpl_make_binary(mpl, 345 as i32, x, y, 118 as i32, 0 as i32);
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_4(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_3(mpl);
    loop {
        if (*mpl).token == 225 as i32 {
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if !((*x).type_0 == 118 as i32 || (*x).type_0 == 110 as i32) {
                _glp_mpl_error_preceding(
                    mpl,
                    b"+\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_3(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if !((*y).type_0 == 118 as i32 || (*y).type_0 == 110 as i32) {
                _glp_mpl_error_following(
                    mpl,
                    b"+\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).type_0 == 118 as i32 && (*y).type_0 == 110 as i32 {
                x = _glp_mpl_make_unary(mpl, 320 as i32, x, 110 as i32, 0 as i32);
            }
            if (*x).type_0 == 110 as i32 && (*y).type_0 == 118 as i32 {
                y = _glp_mpl_make_unary(mpl, 320 as i32, y, 110 as i32, 0 as i32);
            }
            x = _glp_mpl_make_binary(mpl, 339 as i32, x, y, (*x).type_0, 0 as i32);
        } else if (*mpl).token == 226 as i32 {
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if !((*x).type_0 == 118 as i32 || (*x).type_0 == 110 as i32) {
                _glp_mpl_error_preceding(
                    mpl,
                    b"-\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_3(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if !((*y).type_0 == 118 as i32 || (*y).type_0 == 110 as i32) {
                _glp_mpl_error_following(
                    mpl,
                    b"-\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).type_0 == 118 as i32 && (*y).type_0 == 110 as i32 {
                x = _glp_mpl_make_unary(mpl, 320 as i32, x, 110 as i32, 0 as i32);
            }
            if (*x).type_0 == 110 as i32 && (*y).type_0 == 118 as i32 {
                y = _glp_mpl_make_unary(mpl, 320 as i32, y, 110 as i32, 0 as i32);
            }
            x = _glp_mpl_make_binary(mpl, 340 as i32, x, y, (*x).type_0, 0 as i32);
        } else {
            if !((*mpl).token == 216 as i32) {
                break;
            }
            if (*x).type_0 == 124 as i32 {
                x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
            }
            if (*x).type_0 != 118 as i32 {
                _glp_mpl_error_preceding(
                    mpl,
                    b"less\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_3(mpl);
            if (*y).type_0 == 124 as i32 {
                y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
            }
            if (*y).type_0 != 118 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"less\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            x = _glp_mpl_make_binary(mpl, 341 as i32, x, y, 118 as i32, 0 as i32);
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_5(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_4(mpl);
    while (*mpl).token == 236 as i32 {
        if (*x).type_0 == 118 as i32 {
            x = _glp_mpl_make_unary(mpl, 317 as i32, x, 124 as i32, 0 as i32);
        }
        if (*x).type_0 != 124 as i32 {
            _glp_mpl_error_preceding(mpl, b"&\0" as *const u8 as *const i8 as *mut i8);
        }
        _glp_mpl_get_token(mpl);
        y = _glp_mpl_expression_4(mpl);
        if (*y).type_0 == 118 as i32 {
            y = _glp_mpl_make_unary(mpl, 317 as i32, y, 124 as i32, 0 as i32);
        }
        if (*y).type_0 != 124 as i32 {
            _glp_mpl_error_following(mpl, b"&\0" as *const u8 as *const i8 as *mut i8);
        }
        x = _glp_mpl_make_binary(mpl, 352 as i32, x, y, 124 as i32, 0 as i32);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_6(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    let mut z: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_5(mpl);
    if (*mpl).token == 243 as i32 {
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if (*x).type_0 != 118 as i32 {
            _glp_mpl_error_preceding(mpl, b"..\0" as *const u8 as *const i8 as *mut i8);
        }
        _glp_mpl_get_token(mpl);
        y = _glp_mpl_expression_5(mpl);
        if (*y).type_0 == 124 as i32 {
            y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
        }
        if (*y).type_0 != 118 as i32 {
            _glp_mpl_error_following(mpl, b"..\0" as *const u8 as *const i8 as *mut i8);
        }
        if (*mpl).token == 207 as i32 {
            _glp_mpl_get_token(mpl);
            z = _glp_mpl_expression_5(mpl);
            if (*z).type_0 == 124 as i32 {
                z = _glp_mpl_make_unary(mpl, 316 as i32, z, 118 as i32, 0 as i32);
            }
            if (*z).type_0 != 118 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"by\0" as *const u8 as *const i8 as *mut i8,
                );
            }
        } else {
            z = 0 as *mut CODE;
        }
        x = _glp_mpl_make_ternary(mpl, 373 as i32, x, y, z, 106 as i32, 1 as i32);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_7(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_6(mpl);
    while (*mpl).token == 208 as i32 {
        if (*x).type_0 != 106 as i32 {
            _glp_mpl_error_preceding(
                mpl,
                b"cross\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        _glp_mpl_get_token(mpl);
        y = _glp_mpl_expression_6(mpl);
        if (*y).type_0 != 106 as i32 {
            _glp_mpl_error_following(
                mpl,
                b"cross\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        x = _glp_mpl_make_binary(mpl, 365 as i32, x, y, 106 as i32, (*x).dim + (*y).dim);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_8(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_7(mpl);
    while (*mpl).token == 215 as i32 {
        if (*x).type_0 != 106 as i32 {
            _glp_mpl_error_preceding(
                mpl,
                b"inter\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        _glp_mpl_get_token(mpl);
        y = _glp_mpl_expression_7(mpl);
        if (*y).type_0 != 106 as i32 {
            _glp_mpl_error_following(
                mpl,
                b"inter\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        if (*x).dim != (*y).dim {
            _glp_mpl_error_dimension(
                mpl,
                b"inter\0" as *const u8 as *const i8 as *mut i8,
                (*x).dim,
                (*y).dim,
            );
        }
        x = _glp_mpl_make_binary(mpl, 364 as i32, x, y, 106 as i32, (*x).dim);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_9(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    x = _glp_mpl_expression_8(mpl);
    loop {
        if (*mpl).token == 223 as i32 {
            if (*x).type_0 != 106 as i32 {
                _glp_mpl_error_preceding(
                    mpl,
                    b"union\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_8(mpl);
            if (*y).type_0 != 106 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"union\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).dim != (*y).dim {
                _glp_mpl_error_dimension(
                    mpl,
                    b"union\0" as *const u8 as *const i8 as *mut i8,
                    (*x).dim,
                    (*y).dim,
                );
            }
            x = _glp_mpl_make_binary(mpl, 361 as i32, x, y, 106 as i32, (*x).dim);
        } else if (*mpl).token == 209 as i32 {
            if (*x).type_0 != 106 as i32 {
                _glp_mpl_error_preceding(
                    mpl,
                    b"diff\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_8(mpl);
            if (*y).type_0 != 106 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"diff\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).dim != (*y).dim {
                _glp_mpl_error_dimension(
                    mpl,
                    b"diff\0" as *const u8 as *const i8 as *mut i8,
                    (*x).dim,
                    (*y).dim,
                );
            }
            x = _glp_mpl_make_binary(mpl, 362 as i32, x, y, 106 as i32, (*x).dim);
        } else {
            if !((*mpl).token == 221 as i32) {
                break;
            }
            if (*x).type_0 != 106 as i32 {
                _glp_mpl_error_preceding(
                    mpl,
                    b"symdiff\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
            y = _glp_mpl_expression_8(mpl);
            if (*y).type_0 != 106 as i32 {
                _glp_mpl_error_following(
                    mpl,
                    b"symdiff\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*x).dim != (*y).dim {
                _glp_mpl_error_dimension(
                    mpl,
                    b"symdiff\0" as *const u8 as *const i8 as *mut i8,
                    (*x).dim,
                    (*y).dim,
                );
            }
            x = _glp_mpl_make_binary(mpl, 363 as i32, x, y, 106 as i32, (*x).dim);
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_10(mut mpl: *mut MPL) -> *mut CODE {
    let mut current_block: u64;
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    let mut op: i32 = -(1 as i32);
    let mut opstr: [i8; 16] = [0; 16];
    x = _glp_mpl_expression_9(mpl);
    strcpy(opstr.as_mut_ptr(), b"\0" as *const u8 as *const i8);
    match (*mpl).token {
        230 => {
            op = 353 as i32;
            current_block = 5783071609795492627;
        }
        231 => {
            op = 354 as i32;
            current_block = 5783071609795492627;
        }
        232 => {
            op = 355 as i32;
            current_block = 5783071609795492627;
        }
        233 => {
            op = 356 as i32;
            current_block = 5783071609795492627;
        }
        234 => {
            op = 357 as i32;
            current_block = 5783071609795492627;
        }
        235 => {
            op = 358 as i32;
            current_block = 5783071609795492627;
        }
        213 => {
            op = 366 as i32;
            current_block = 5783071609795492627;
        }
        224 => {
            op = 368 as i32;
            current_block = 5783071609795492627;
        }
        218 => {
            strcpy(opstr.as_mut_ptr(), (*mpl).image);
            _glp_mpl_get_token(mpl);
            if (*mpl).token == 213 as i32 {
                op = 367 as i32;
            } else if (*mpl).token == 224 as i32 {
                op = 369 as i32;
            } else {
                _glp_mpl_error(
                    mpl,
                    b"invalid use of %s\0" as *const u8 as *const i8 as *mut i8,
                    opstr.as_mut_ptr(),
                );
            }
            strcat(opstr.as_mut_ptr(), b" \0" as *const u8 as *const i8);
            current_block = 5783071609795492627;
        }
        _ => {
            current_block = 962402290960692374;
        }
    }
    match current_block {
        5783071609795492627 => {
            strcat(opstr.as_mut_ptr(), (*mpl).image);
            (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 16]>() as u64
                || {
                    glp_assert_(
                        b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        2752 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            match op {
                355 | 358 | 353 | 354 | 357 | 356 => {
                    if !((*x).type_0 == 118 as i32 || (*x).type_0 == 124 as i32) {
                        _glp_mpl_error_preceding(mpl, opstr.as_mut_ptr());
                    }
                    _glp_mpl_get_token(mpl);
                    y = _glp_mpl_expression_9(mpl);
                    if !((*y).type_0 == 118 as i32 || (*y).type_0 == 124 as i32) {
                        _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
                    }
                    if (*x).type_0 == 118 as i32 && (*y).type_0 == 124 as i32 {
                        x = _glp_mpl_make_unary(
                            mpl,
                            317 as i32,
                            x,
                            124 as i32,
                            0 as i32,
                        );
                    }
                    if (*x).type_0 == 124 as i32 && (*y).type_0 == 118 as i32 {
                        y = _glp_mpl_make_unary(
                            mpl,
                            317 as i32,
                            y,
                            124 as i32,
                            0 as i32,
                        );
                    }
                    x = _glp_mpl_make_binary(mpl, op, x, y, 114 as i32, 0 as i32);
                }
                366 | 367 => {
                    if (*x).type_0 == 118 as i32 {
                        x = _glp_mpl_make_unary(
                            mpl,
                            317 as i32,
                            x,
                            124 as i32,
                            0 as i32,
                        );
                    }
                    if (*x).type_0 == 124 as i32 {
                        x = _glp_mpl_make_unary(
                            mpl,
                            319 as i32,
                            x,
                            126 as i32,
                            1 as i32,
                        );
                    }
                    if (*x).type_0 != 126 as i32 {
                        _glp_mpl_error_preceding(mpl, opstr.as_mut_ptr());
                    }
                    _glp_mpl_get_token(mpl);
                    y = _glp_mpl_expression_9(mpl);
                    if (*y).type_0 != 106 as i32 {
                        _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
                    }
                    if (*x).dim != (*y).dim {
                        _glp_mpl_error_dimension(
                            mpl,
                            opstr.as_mut_ptr(),
                            (*x).dim,
                            (*y).dim,
                        );
                    }
                    x = _glp_mpl_make_binary(mpl, op, x, y, 114 as i32, 0 as i32);
                }
                368 | 369 => {
                    if (*x).type_0 != 106 as i32 {
                        _glp_mpl_error_preceding(mpl, opstr.as_mut_ptr());
                    }
                    _glp_mpl_get_token(mpl);
                    y = _glp_mpl_expression_9(mpl);
                    if (*y).type_0 != 106 as i32 {
                        _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
                    }
                    if (*x).dim != (*y).dim {
                        _glp_mpl_error_dimension(
                            mpl,
                            opstr.as_mut_ptr(),
                            (*x).dim,
                            (*y).dim,
                        );
                    }
                    x = _glp_mpl_make_binary(mpl, op, x, y, 114 as i32, 0 as i32);
                }
                _ => {
                    (op != op
                        || {
                            glp_assert_(
                                b"op != op\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                2821 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        _ => {}
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_11(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut opstr: [i8; 8] = [0; 8];
    if (*mpl).token == 218 as i32 {
        strcpy(opstr.as_mut_ptr(), (*mpl).image);
        (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 8]>() as u64
            || {
                glp_assert_(
                    b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    2840 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_mpl_get_token(mpl);
        x = _glp_mpl_expression_10(mpl);
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if (*x).type_0 == 118 as i32 {
            x = _glp_mpl_make_unary(mpl, 318 as i32, x, 114 as i32, 0 as i32);
        }
        if (*x).type_0 != 114 as i32 {
            _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
        }
        x = _glp_mpl_make_unary(mpl, 323 as i32, x, 114 as i32, 0 as i32);
    } else {
        x = _glp_mpl_expression_10(mpl);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_12(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    let mut opstr: [i8; 8] = [0; 8];
    x = _glp_mpl_expression_11(mpl);
    while (*mpl).token == 206 as i32 {
        strcpy(opstr.as_mut_ptr(), (*mpl).image);
        (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 8]>() as u64
            || {
                glp_assert_(
                    b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    2872 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if (*x).type_0 == 118 as i32 {
            x = _glp_mpl_make_unary(mpl, 318 as i32, x, 114 as i32, 0 as i32);
        }
        if (*x).type_0 != 114 as i32 {
            _glp_mpl_error_preceding(mpl, opstr.as_mut_ptr());
        }
        _glp_mpl_get_token(mpl);
        y = _glp_mpl_expression_11(mpl);
        if (*y).type_0 == 124 as i32 {
            y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
        }
        if (*y).type_0 == 118 as i32 {
            y = _glp_mpl_make_unary(mpl, 318 as i32, y, 114 as i32, 0 as i32);
        }
        if (*y).type_0 != 114 as i32 {
            _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
        }
        x = _glp_mpl_make_binary(mpl, 359 as i32, x, y, 114 as i32, 0 as i32);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_expression_13(mut mpl: *mut MPL) -> *mut CODE {
    let mut x: *mut CODE = 0 as *mut CODE;
    let mut y: *mut CODE = 0 as *mut CODE;
    let mut opstr: [i8; 8] = [0; 8];
    x = _glp_mpl_expression_12(mpl);
    while (*mpl).token == 219 as i32 {
        strcpy(opstr.as_mut_ptr(), (*mpl).image);
        (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 8]>() as u64
            || {
                glp_assert_(
                    b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    2911 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if (*x).type_0 == 124 as i32 {
            x = _glp_mpl_make_unary(mpl, 316 as i32, x, 118 as i32, 0 as i32);
        }
        if (*x).type_0 == 118 as i32 {
            x = _glp_mpl_make_unary(mpl, 318 as i32, x, 114 as i32, 0 as i32);
        }
        if (*x).type_0 != 114 as i32 {
            _glp_mpl_error_preceding(mpl, opstr.as_mut_ptr());
        }
        _glp_mpl_get_token(mpl);
        y = _glp_mpl_expression_12(mpl);
        if (*y).type_0 == 124 as i32 {
            y = _glp_mpl_make_unary(mpl, 316 as i32, y, 118 as i32, 0 as i32);
        }
        if (*y).type_0 == 118 as i32 {
            y = _glp_mpl_make_unary(mpl, 318 as i32, y, 114 as i32, 0 as i32);
        }
        if (*y).type_0 != 114 as i32 {
            _glp_mpl_error_following(mpl, opstr.as_mut_ptr());
        }
        x = _glp_mpl_make_binary(mpl, 360 as i32, x, y, 114 as i32, 0 as i32);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_set_statement(mut mpl: *mut MPL) -> *mut SET {
    let mut current_block: u64;
    let mut set: *mut SET = 0 as *mut SET;
    let mut dimen_used: i32 = 0 as i32;
    (_glp_mpl_is_keyword(mpl, b"set\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"set\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                2956 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    if !((*mpl).token == 202 as i32) {
        if _glp_mpl_is_reserved(mpl) != 0 {
            _glp_mpl_error(
                mpl,
                b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                    as *mut i8,
                (*mpl).image,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"symbolic name missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if !(_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void)).is_null()
    {
        _glp_mpl_error(
            mpl,
            b"%s multiply declared\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    set = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<SET>() as u64 as i32)
        as *mut SET;
    (*set).name = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*set).name, (*mpl).image);
    (*set).alias = 0 as *mut i8;
    (*set).dim = 0 as i32;
    (*set).domain = 0 as *mut DOMAIN1;
    (*set).dimen = 0 as i32;
    (*set).within = 0 as *mut WITHIN;
    (*set).assign = 0 as *mut CODE;
    (*set).option = 0 as *mut CODE;
    (*set).gadget = 0 as *mut GADGET;
    (*set).data = 0 as i32;
    (*set).array = 0 as *mut ARRAY;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 205 as i32 {
        (*set).alias = _glp_dmp_get_atom(
            (*mpl).pool,
            (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*set).alias, (*mpl).image);
        _glp_mpl_get_token(mpl);
    }
    if (*mpl).token == 248 as i32 {
        (*set).domain = _glp_mpl_indexing_expression(mpl);
        (*set).dim = _glp_mpl_domain_arity(mpl, (*set).domain);
    }
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    node = _glp_avl_insert_node((*mpl).tree, (*set).name as *const libc::c_void);
    _glp_avl_set_node_type(node, 122 as i32);
    _glp_avl_set_node_link(node, set as *mut libc::c_void);
    loop {
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else if (*mpl).token == 241 as i32 {
            break;
        }
        if _glp_mpl_is_keyword(mpl, b"dimen\0" as *const u8 as *const i8 as *mut i8) != 0
        {
            let mut dimen: i32 = 0;
            _glp_mpl_get_token(mpl);
            if !((*mpl).token == 204 as i32 && 1.0f64 <= (*mpl).value
                && (*mpl).value <= 20.0f64 && floor((*mpl).value) == (*mpl).value)
            {
                _glp_mpl_error(
                    mpl,
                    b"dimension must be integer between 1 and 20\0" as *const u8
                        as *const i8 as *mut i8,
                );
            }
            dimen = ((*mpl).value + 0.5f64) as i32;
            if dimen_used != 0 {
                _glp_mpl_error(
                    mpl,
                    b"at most one dimension attribute allowed\0" as *const u8
                        as *const i8 as *mut i8,
                );
            }
            if (*set).dimen > 0 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"dimension %d conflicts with dimension %d already determined\0"
                        as *const u8 as *const i8 as *mut i8,
                    dimen,
                    (*set).dimen,
                );
            }
            (*set).dimen = dimen;
            dimen_used = 1 as i32;
            _glp_mpl_get_token(mpl);
        } else if (*mpl).token == 224 as i32 || (*mpl).token == 213 as i32 {
            let mut within: *mut WITHIN = 0 as *mut WITHIN;
            let mut temp: *mut WITHIN = 0 as *mut WITHIN;
            if (*mpl).token == 213 as i32 && (*mpl).as_within == 0 {
                _glp_mpl_warning(
                    mpl,
                    b"keyword in understood as within\0" as *const u8 as *const i8
                        as *mut i8,
                );
                (*mpl).as_within = 1 as i32;
            }
            _glp_mpl_get_token(mpl);
            within = _glp_dmp_get_atom(
                (*mpl).pool,
                ::core::mem::size_of::<WITHIN>() as u64 as i32,
            ) as *mut WITHIN;
            (*within).code = 0 as *mut CODE;
            (*within).next = 0 as *mut WITHIN;
            if ((*set).within).is_null() {
                (*set).within = within;
            } else {
                temp = (*set).within;
                while !((*temp).next).is_null() {
                    temp = (*temp).next;
                }
                (*temp).next = within;
            }
            (*within).code = _glp_mpl_expression_9(mpl);
            if (*(*within).code).type_0 != 106 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"expression following within has invalid type\0" as *const u8
                        as *const i8 as *mut i8,
                );
            }
            ((*(*within).code).dim > 0 as i32
                || {
                    glp_assert_(
                        b"within->code->dim > 0\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        3049 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*set).dimen == 0 as i32 {
                (*set).dimen = (*(*within).code).dim;
            }
            if (*set).dimen != (*(*within).code).dim {
                _glp_mpl_error(
                    mpl,
                    b"set expression following within must have dimension %d rather than %d\0"
                        as *const u8 as *const i8 as *mut i8,
                    (*set).dimen,
                    (*(*within).code).dim,
                );
            }
        } else {
            if (*mpl).token == 242 as i32 {
                if !(((*set).assign).is_null() && ((*set).option).is_null()
                    && ((*set).gadget).is_null())
                {
                    current_block = 14039549852302920099;
                } else {
                    current_block = 8835654301469918283;
                }
            } else {
                if _glp_mpl_is_keyword(
                    mpl,
                    b"default\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    if ((*set).assign).is_null() && ((*set).option).is_null() {
                        _glp_mpl_get_token(mpl);
                        (*set).option = _glp_mpl_expression_9(mpl);
                        if (*(*set).option).type_0 != 106 as i32 {
                            _glp_mpl_error(
                                mpl,
                                b"expression following default has invalid type\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        }
                        ((*(*set).option).dim > 0 as i32
                            || {
                                glp_assert_(
                                    b"set->option->dim > 0\0" as *const u8 as *const i8,
                                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                    3084 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        if (*set).dimen == 0 as i32 {
                            (*set).dimen = (*(*set).option).dim;
                        }
                        if (*set).dimen != (*(*set).option).dim {
                            _glp_mpl_error(
                                mpl,
                                b"set expression following default must have dimension %d rather than %d\0"
                                    as *const u8 as *const i8 as *mut i8,
                                (*set).dimen,
                                (*(*set).option).dim,
                            );
                        }
                        continue;
                    }
                } else if _glp_mpl_is_keyword(
                    mpl,
                    b"data\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    let mut gadget: *mut GADGET = 0 as *mut GADGET;
                    let mut node_0: *mut AVLNODE = 0 as *mut AVLNODE;
                    let mut i: i32 = 0;
                    let mut k: i32 = 0;
                    let mut fff: [i32; 20] = [0; 20];
                    if ((*set).assign).is_null() && ((*set).gadget).is_null() {
                        _glp_mpl_get_token(mpl);
                        gadget = _glp_dmp_get_atom(
                            (*mpl).pool,
                            ::core::mem::size_of::<GADGET>() as u64 as i32,
                        ) as *mut GADGET;
                        (*set).gadget = gadget;
                        if !((*mpl).token == 202 as i32) {
                            if _glp_mpl_is_reserved(mpl) != 0 {
                                _glp_mpl_error(
                                    mpl,
                                    b"invalid use of reserved keyword %s\0" as *const u8
                                        as *const i8 as *mut i8,
                                    (*mpl).image,
                                );
                            } else {
                                _glp_mpl_error(
                                    mpl,
                                    b"set name missing where expected\0" as *const u8
                                        as *const i8 as *mut i8,
                                );
                            }
                        }
                        node_0 = _glp_avl_find_node(
                            (*mpl).tree,
                            (*mpl).image as *const libc::c_void,
                        );
                        if node_0.is_null() {
                            _glp_mpl_error(
                                mpl,
                                b"%s not defined\0" as *const u8 as *const i8 as *mut i8,
                                (*mpl).image,
                            );
                        }
                        if _glp_avl_get_node_type(node_0) != 122 as i32 {
                            current_block = 15075657259964552610;
                        } else {
                            current_block = 16791665189521845338;
                        }
                        loop {
                            match current_block {
                                15075657259964552610 => {
                                    _glp_mpl_error(
                                        mpl,
                                        b"%s not a plain set\0" as *const u8 as *const i8
                                            as *mut i8,
                                        (*mpl).image,
                                    );
                                    current_block = 16791665189521845338;
                                }
                                _ => {
                                    (*gadget).set = _glp_avl_get_node_link(node_0) as *mut SET;
                                    if (*(*gadget).set).dim != 0 as i32 {
                                        current_block = 15075657259964552610;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                        if (*gadget).set == set {
                            _glp_mpl_error(
                                mpl,
                                b"set cannot be initialized by itself\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        }
                        if (*set).dim >= (*(*gadget).set).dimen {
                            current_block = 16838577200577566215;
                        } else {
                            current_block = 16313536926714486912;
                        }
                        loop {
                            match current_block {
                                16838577200577566215 => {
                                    _glp_mpl_error(
                                        mpl,
                                        b"dimension of %s too small\0" as *const u8 as *const i8
                                            as *mut i8,
                                        (*mpl).image,
                                    );
                                    current_block = 16313536926714486912;
                                }
                                _ => {
                                    if (*set).dimen == 0 as i32 {
                                        (*set).dimen = (*(*gadget).set).dimen - (*set).dim;
                                    }
                                    if (*set).dim + (*set).dimen > (*(*gadget).set).dimen {
                                        current_block = 16838577200577566215;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                        if (*set).dim + (*set).dimen < (*(*gadget).set).dimen {
                            _glp_mpl_error(
                                mpl,
                                b"dimension of %s too big\0" as *const u8 as *const i8
                                    as *mut i8,
                                (*mpl).image,
                            );
                        }
                        _glp_mpl_get_token(mpl);
                        if (*mpl).token == 244 as i32 {
                            _glp_mpl_get_token(mpl);
                        } else {
                            _glp_mpl_error(
                                mpl,
                                b"left parenthesis missing where expected\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        }
                        k = 0 as i32;
                        while k < (*(*gadget).set).dimen {
                            fff[k as usize] = 0 as i32;
                            k += 1;
                            k;
                        }
                        k = 0 as i32;
                        let mut current_block_128: u64;
                        loop {
                            if (*mpl).token != 204 as i32 {
                                _glp_mpl_error(
                                    mpl,
                                    b"component number missing where expected\0" as *const u8
                                        as *const i8 as *mut i8,
                                );
                            }
                            if _glp_str2int((*mpl).image, &mut i) != 0 as i32 {
                                current_block_128 = 14210507252782174831;
                            } else {
                                current_block_128 = 12065775993741208975;
                            }
                            loop {
                                match current_block_128 {
                                    12065775993741208975 => {
                                        if !(1 as i32 <= i && i <= (*(*gadget).set).dimen) {
                                            current_block_128 = 14210507252782174831;
                                        } else {
                                            break;
                                        }
                                    }
                                    _ => {
                                        _glp_mpl_error(
                                            mpl,
                                            b"component number must be integer between 1 and %d\0"
                                                as *const u8 as *const i8 as *mut i8,
                                            (*(*gadget).set).dimen,
                                        );
                                        current_block_128 = 12065775993741208975;
                                    }
                                }
                            }
                            if fff[(i - 1 as i32) as usize] != 0 as i32 {
                                _glp_mpl_error(
                                    mpl,
                                    b"component %d multiply specified\0" as *const u8
                                        as *const i8 as *mut i8,
                                    i,
                                );
                            }
                            let fresh3 = k;
                            k = k + 1;
                            (*gadget).ind[fresh3 as usize] = i;
                            fff[(i - 1 as i32) as usize] = 1 as i32;
                            (k <= (*(*gadget).set).dimen
                                || {
                                    glp_assert_(
                                        b"k <= gadget->set->dimen\0" as *const u8 as *const i8,
                                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                        3147 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            _glp_mpl_get_token(mpl);
                            if (*mpl).token == 239 as i32 {
                                _glp_mpl_get_token(mpl);
                            } else {
                                if (*mpl).token == 245 as i32 {
                                    break;
                                }
                                _glp_mpl_error(
                                    mpl,
                                    b"syntax error in data attribute\0" as *const u8
                                        as *const i8 as *mut i8,
                                );
                            }
                        }
                        if k < (*(*gadget).set).dimen {
                            _glp_mpl_error(
                                mpl,
                                b"there are must be %d components rather than %d\0"
                                    as *const u8 as *const i8 as *mut i8,
                                (*(*gadget).set).dimen,
                                k,
                            );
                        }
                        _glp_mpl_get_token(mpl);
                        continue;
                    }
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"syntax error in set statement\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                    continue;
                }
                current_block = 14039549852302920099;
            }
            match current_block {
                14039549852302920099 => {
                    _glp_mpl_error(
                        mpl,
                        b"at most one := or default/data allowed\0" as *const u8
                            as *const i8 as *mut i8,
                    );
                }
                _ => {}
            }
            _glp_mpl_get_token(mpl);
            (*set).assign = _glp_mpl_expression_9(mpl);
            if (*(*set).assign).type_0 != 106 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"expression following := has invalid type\0" as *const u8
                        as *const i8 as *mut i8,
                );
            }
            ((*(*set).assign).dim > 0 as i32
                || {
                    glp_assert_(
                        b"set->assign->dim > 0\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        3067 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*set).dimen == 0 as i32 {
                (*set).dimen = (*(*set).assign).dim;
            }
            if (*set).dimen != (*(*set).assign).dim {
                _glp_mpl_error(
                    mpl,
                    b"set expression following := must have dimension %d rather than %d\0"
                        as *const u8 as *const i8 as *mut i8,
                    (*set).dimen,
                    (*(*set).assign).dim,
                );
            }
        }
    }
    if !((*set).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*set).domain);
    }
    if (*set).dimen == 0 as i32 {
        (*set).dimen = 1 as i32;
    }
    ((*mpl).token == 241 as i32
        || {
            glp_assert_(
                b"mpl->token == T_SEMICOLON\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3170 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_parameter_statement(
    mut mpl: *mut MPL,
) -> *mut PARAMETER {
    let mut par: *mut PARAMETER = 0 as *mut PARAMETER;
    let mut integer_used: i32 = 0 as i32;
    let mut binary_used: i32 = 0 as i32;
    let mut symbolic_used: i32 = 0 as i32;
    (_glp_mpl_is_keyword(mpl, b"param\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"param\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3201 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    if !((*mpl).token == 202 as i32) {
        if _glp_mpl_is_reserved(mpl) != 0 {
            _glp_mpl_error(
                mpl,
                b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                    as *mut i8,
                (*mpl).image,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"symbolic name missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if !(_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void)).is_null()
    {
        _glp_mpl_error(
            mpl,
            b"%s multiply declared\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    par = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<PARAMETER>() as u64 as i32,
    ) as *mut PARAMETER;
    (*par).name = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*par).name, (*mpl).image);
    (*par).alias = 0 as *mut i8;
    (*par).dim = 0 as i32;
    (*par).domain = 0 as *mut DOMAIN1;
    (*par).type_0 = 118 as i32;
    (*par).cond = 0 as *mut CONDITION;
    (*par).in_0 = 0 as *mut WITHIN;
    (*par).assign = 0 as *mut CODE;
    (*par).option = 0 as *mut CODE;
    (*par).data = 0 as i32;
    (*par).defval = 0 as *mut SYMBOL;
    (*par).array = 0 as *mut ARRAY;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 205 as i32 {
        (*par).alias = _glp_dmp_get_atom(
            (*mpl).pool,
            (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*par).alias, (*mpl).image);
        _glp_mpl_get_token(mpl);
    }
    if (*mpl).token == 248 as i32 {
        (*par).domain = _glp_mpl_indexing_expression(mpl);
        (*par).dim = _glp_mpl_domain_arity(mpl, (*par).domain);
    }
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    node = _glp_avl_insert_node((*mpl).tree, (*par).name as *const libc::c_void);
    _glp_avl_set_node_type(node, 120 as i32);
    _glp_avl_set_node_link(node, par as *mut libc::c_void);
    loop {
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else if (*mpl).token == 241 as i32 {
            break;
        }
        if _glp_mpl_is_keyword(mpl, b"integer\0" as *const u8 as *const i8 as *mut i8)
            != 0
        {
            if integer_used != 0 {
                _glp_mpl_error(
                    mpl,
                    b"at most one integer allowed\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*par).type_0 == 124 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"symbolic parameter cannot be integer\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
            if (*par).type_0 != 101 as i32 {
                (*par).type_0 = 113 as i32;
            }
            integer_used = 1 as i32;
            _glp_mpl_get_token(mpl);
        } else {
            let mut current_block_143: u64;
            if _glp_mpl_is_keyword(mpl, b"binary\0" as *const u8 as *const i8 as *mut i8)
                != 0
            {
                current_block_143 = 10292674526205761768;
            } else if _glp_mpl_is_keyword(
                mpl,
                b"logical\0" as *const u8 as *const i8 as *mut i8,
            ) != 0
            {
                if (*mpl).as_binary == 0 {
                    _glp_mpl_warning(
                        mpl,
                        b"keyword logical understood as binary\0" as *const u8
                            as *const i8 as *mut i8,
                    );
                    (*mpl).as_binary = 1 as i32;
                }
                current_block_143 = 10292674526205761768;
            } else {
                if _glp_mpl_is_keyword(
                    mpl,
                    b"symbolic\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    if symbolic_used != 0 {
                        _glp_mpl_error(
                            mpl,
                            b"at most one symbolic allowed\0" as *const u8 as *const i8
                                as *mut i8,
                        );
                    }
                    if (*par).type_0 != 118 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"integer or binary parameter cannot be symbolic\0"
                                as *const u8 as *const i8 as *mut i8,
                        );
                    }
                    if !(((*par).cond).is_null() && ((*par).in_0).is_null()
                        && ((*par).assign).is_null() && ((*par).option).is_null())
                    {
                        _glp_mpl_error(
                            mpl,
                            b"keyword symbolic must precede any other parameter attributes\0"
                                as *const u8 as *const i8 as *mut i8,
                        );
                    }
                    (*par).type_0 = 124 as i32;
                    symbolic_used = 1 as i32;
                    _glp_mpl_get_token(mpl);
                } else if (*mpl).token == 230 as i32 || (*mpl).token == 231 as i32
                    || (*mpl).token == 232 as i32 || (*mpl).token == 233 as i32
                    || (*mpl).token == 234 as i32 || (*mpl).token == 235 as i32
                {
                    let mut cond: *mut CONDITION = 0 as *mut CONDITION;
                    let mut temp: *mut CONDITION = 0 as *mut CONDITION;
                    let mut opstr: [i8; 8] = [0; 8];
                    cond = _glp_dmp_get_atom(
                        (*mpl).pool,
                        ::core::mem::size_of::<CONDITION>() as u64 as i32,
                    ) as *mut CONDITION;
                    match (*mpl).token {
                        230 => {
                            (*cond).rho = 353 as i32;
                            strcpy(opstr.as_mut_ptr(), (*mpl).image);
                        }
                        231 => {
                            (*cond).rho = 354 as i32;
                            strcpy(opstr.as_mut_ptr(), (*mpl).image);
                        }
                        232 => {
                            (*cond).rho = 355 as i32;
                            strcpy(opstr.as_mut_ptr(), (*mpl).image);
                        }
                        233 => {
                            (*cond).rho = 356 as i32;
                            strcpy(opstr.as_mut_ptr(), (*mpl).image);
                        }
                        234 => {
                            (*cond).rho = 357 as i32;
                            strcpy(opstr.as_mut_ptr(), (*mpl).image);
                        }
                        235 => {
                            (*cond).rho = 358 as i32;
                            strcpy(opstr.as_mut_ptr(), (*mpl).image);
                        }
                        _ => {
                            ((*mpl).token != (*mpl).token
                                || {
                                    glp_assert_(
                                        b"mpl->token != mpl->token\0" as *const u8 as *const i8,
                                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                        3317 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                        }
                    }
                    (strlen(opstr.as_mut_ptr())
                        < ::core::mem::size_of::<[i8; 8]>() as u64
                        || {
                            glp_assert_(
                                b"strlen(opstr) < sizeof(opstr)\0" as *const u8
                                    as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3319 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*cond).code = 0 as *mut CODE;
                    (*cond).next = 0 as *mut CONDITION;
                    if ((*par).cond).is_null() {
                        (*par).cond = cond;
                    } else {
                        temp = (*par).cond;
                        while !((*temp).next).is_null() {
                            temp = (*temp).next;
                        }
                        (*temp).next = cond;
                    }
                    _glp_mpl_get_token(mpl);
                    (*cond).code = _glp_mpl_expression_5(mpl);
                    if !((*(*cond).code).type_0 == 118 as i32
                        || (*(*cond).code).type_0 == 124 as i32)
                    {
                        _glp_mpl_error(
                            mpl,
                            b"expression following %s has invalid type\0" as *const u8
                                as *const i8 as *mut i8,
                            opstr.as_mut_ptr(),
                        );
                    }
                    ((*(*cond).code).dim == 0 as i32
                        || {
                            glp_assert_(
                                b"cond->code->dim == 0\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3341 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*par).type_0 != 124 as i32
                        && (*(*cond).code).type_0 == 124 as i32
                    {
                        (*cond).code = _glp_mpl_make_unary(
                            mpl,
                            316 as i32,
                            (*cond).code,
                            118 as i32,
                            0 as i32,
                        );
                    }
                    if (*par).type_0 == 124 as i32
                        && (*(*cond).code).type_0 != 124 as i32
                    {
                        (*cond).code = _glp_mpl_make_unary(
                            mpl,
                            317 as i32,
                            (*cond).code,
                            124 as i32,
                            0 as i32,
                        );
                    }
                } else if (*mpl).token == 213 as i32 || (*mpl).token == 224 as i32 {
                    let mut in_0: *mut WITHIN = 0 as *mut WITHIN;
                    let mut temp_0: *mut WITHIN = 0 as *mut WITHIN;
                    if (*mpl).token == 224 as i32 && (*mpl).as_in == 0 {
                        _glp_mpl_warning(
                            mpl,
                            b"keyword within understood as in\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                        (*mpl).as_in = 1 as i32;
                    }
                    _glp_mpl_get_token(mpl);
                    in_0 = _glp_dmp_get_atom(
                        (*mpl).pool,
                        ::core::mem::size_of::<WITHIN>() as u64 as i32,
                    ) as *mut WITHIN;
                    (*in_0).code = 0 as *mut CODE;
                    (*in_0).next = 0 as *mut WITHIN;
                    if ((*par).in_0).is_null() {
                        (*par).in_0 = in_0;
                    } else {
                        temp_0 = (*par).in_0;
                        while !((*temp_0).next).is_null() {
                            temp_0 = (*temp_0).next;
                        }
                        (*temp_0).next = in_0;
                    }
                    (*in_0).code = _glp_mpl_expression_9(mpl);
                    if (*(*in_0).code).type_0 != 106 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"expression following in has invalid type\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                    ((*(*in_0).code).dim > 0 as i32
                        || {
                            glp_assert_(
                                b"in->code->dim > 0\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3376 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*(*in_0).code).dim != 1 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"set expression following in must have dimension 1 rather than %d\0"
                                as *const u8 as *const i8 as *mut i8,
                            (*(*in_0).code).dim,
                        );
                    }
                } else {
                    let mut current_block_139: u64;
                    if (*mpl).token == 242 as i32 {
                        if !(((*par).assign).is_null() && ((*par).option).is_null()) {
                            current_block_139 = 12786716210571394612;
                        } else {
                            current_block_139 = 5265702136860997526;
                        }
                    } else if _glp_mpl_is_keyword(
                        mpl,
                        b"default\0" as *const u8 as *const i8 as *mut i8,
                    ) != 0
                    {
                        if !(((*par).assign).is_null() && ((*par).option).is_null()) {
                            current_block_139 = 12786716210571394612;
                        } else {
                            _glp_mpl_get_token(mpl);
                            (*par).option = _glp_mpl_expression_5(mpl);
                            if !((*(*par).option).type_0 == 118 as i32
                                || (*(*par).option).type_0 == 124 as i32)
                            {
                                _glp_mpl_error(
                                    mpl,
                                    b"expression following default has invalid type\0"
                                        as *const u8 as *const i8 as *mut i8,
                                );
                            }
                            ((*(*par).option).dim == 0 as i32
                                || {
                                    glp_assert_(
                                        b"par->option->dim == 0\0" as *const u8 as *const i8,
                                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                        3413 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            if (*par).type_0 != 124 as i32
                                && (*(*par).option).type_0 == 124 as i32
                            {
                                (*par).option = _glp_mpl_make_unary(
                                    mpl,
                                    316 as i32,
                                    (*par).option,
                                    118 as i32,
                                    0 as i32,
                                );
                            }
                            if (*par).type_0 == 124 as i32
                                && (*(*par).option).type_0 != 124 as i32
                            {
                                (*par).option = _glp_mpl_make_unary(
                                    mpl,
                                    317 as i32,
                                    (*par).option,
                                    124 as i32,
                                    0 as i32,
                                );
                            }
                            current_block_139 = 17917672080766325409;
                        }
                    } else {
                        _glp_mpl_error(
                            mpl,
                            b"syntax error in parameter statement\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                        current_block_139 = 17917672080766325409;
                    }
                    match current_block_139 {
                        12786716210571394612 => {
                            _glp_mpl_error(
                                mpl,
                                b"at most one := or default allowed\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                            current_block_139 = 5265702136860997526;
                        }
                        _ => {}
                    }
                    match current_block_139 {
                        5265702136860997526 => {
                            _glp_mpl_get_token(mpl);
                            (*par).assign = _glp_mpl_expression_5(mpl);
                            if !((*(*par).assign).type_0 == 118 as i32
                                || (*(*par).assign).type_0 == 124 as i32)
                            {
                                _glp_mpl_error(
                                    mpl,
                                    b"expression following := has invalid type\0" as *const u8
                                        as *const i8 as *mut i8,
                                );
                            }
                            ((*(*par).assign).dim == 0 as i32
                                || {
                                    glp_assert_(
                                        b"par->assign->dim == 0\0" as *const u8 as *const i8,
                                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                        3392 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            if (*par).type_0 != 124 as i32
                                && (*(*par).assign).type_0 == 124 as i32
                            {
                                (*par).assign = _glp_mpl_make_unary(
                                    mpl,
                                    316 as i32,
                                    (*par).assign,
                                    118 as i32,
                                    0 as i32,
                                );
                            }
                            if (*par).type_0 == 124 as i32
                                && (*(*par).assign).type_0 != 124 as i32
                            {
                                (*par).assign = _glp_mpl_make_unary(
                                    mpl,
                                    317 as i32,
                                    (*par).assign,
                                    124 as i32,
                                    0 as i32,
                                );
                            }
                        }
                        _ => {}
                    }
                }
                current_block_143 = 16696653877814833746;
            }
            match current_block_143 {
                10292674526205761768 => {
                    if binary_used != 0 {
                        _glp_mpl_error(
                            mpl,
                            b"at most one binary allowed\0" as *const u8 as *const i8
                                as *mut i8,
                        );
                    }
                    if (*par).type_0 == 124 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"symbolic parameter cannot be binary\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                    (*par).type_0 = 101 as i32;
                    binary_used = 1 as i32;
                    _glp_mpl_get_token(mpl);
                }
                _ => {}
            }
        }
    }
    if !((*par).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*par).domain);
    }
    ((*mpl).token == 241 as i32
        || {
            glp_assert_(
                b"mpl->token == T_SEMICOLON\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3430 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    return par;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_variable_statement(
    mut mpl: *mut MPL,
) -> *mut VARIABLE {
    let mut var: *mut VARIABLE = 0 as *mut VARIABLE;
    let mut integer_used: i32 = 0 as i32;
    let mut binary_used: i32 = 0 as i32;
    (_glp_mpl_is_keyword(mpl, b"var\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"var\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3457 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).flag_s != 0 {
        _glp_mpl_error(
            mpl,
            b"variable statement must precede solve statement\0" as *const u8
                as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    if !((*mpl).token == 202 as i32) {
        if _glp_mpl_is_reserved(mpl) != 0 {
            _glp_mpl_error(
                mpl,
                b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                    as *mut i8,
                (*mpl).image,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"symbolic name missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if !(_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void)).is_null()
    {
        _glp_mpl_error(
            mpl,
            b"%s multiply declared\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    var = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<VARIABLE>() as u64 as i32,
    ) as *mut VARIABLE;
    (*var).name = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*var).name, (*mpl).image);
    (*var).alias = 0 as *mut i8;
    (*var).dim = 0 as i32;
    (*var).domain = 0 as *mut DOMAIN1;
    (*var).type_0 = 118 as i32;
    (*var).lbnd = 0 as *mut CODE;
    (*var).ubnd = 0 as *mut CODE;
    (*var).array = 0 as *mut ARRAY;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 205 as i32 {
        (*var).alias = _glp_dmp_get_atom(
            (*mpl).pool,
            (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*var).alias, (*mpl).image);
        _glp_mpl_get_token(mpl);
    }
    if (*mpl).token == 248 as i32 {
        (*var).domain = _glp_mpl_indexing_expression(mpl);
        (*var).dim = _glp_mpl_domain_arity(mpl, (*var).domain);
    }
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    node = _glp_avl_insert_node((*mpl).tree, (*var).name as *const libc::c_void);
    _glp_avl_set_node_type(node, 127 as i32);
    _glp_avl_set_node_link(node, var as *mut libc::c_void);
    loop {
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else if (*mpl).token == 241 as i32 {
            break;
        }
        if _glp_mpl_is_keyword(mpl, b"integer\0" as *const u8 as *const i8 as *mut i8)
            != 0
        {
            if integer_used != 0 {
                _glp_mpl_error(
                    mpl,
                    b"at most one integer allowed\0" as *const u8 as *const i8 as *mut i8,
                );
            }
            if (*var).type_0 != 101 as i32 {
                (*var).type_0 = 113 as i32;
            }
            integer_used = 1 as i32;
            _glp_mpl_get_token(mpl);
        } else {
            let mut current_block_104: u64;
            if _glp_mpl_is_keyword(mpl, b"binary\0" as *const u8 as *const i8 as *mut i8)
                != 0
            {
                current_block_104 = 18382494810155246110;
            } else if _glp_mpl_is_keyword(
                mpl,
                b"logical\0" as *const u8 as *const i8 as *mut i8,
            ) != 0
            {
                if (*mpl).as_binary == 0 {
                    _glp_mpl_warning(
                        mpl,
                        b"keyword logical understood as binary\0" as *const u8
                            as *const i8 as *mut i8,
                    );
                    (*mpl).as_binary = 1 as i32;
                }
                current_block_104 = 18382494810155246110;
            } else {
                if _glp_mpl_is_keyword(
                    mpl,
                    b"symbolic\0" as *const u8 as *const i8 as *mut i8,
                ) != 0
                {
                    _glp_mpl_error(
                        mpl,
                        b"variable cannot be symbolic\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                } else if (*mpl).token == 233 as i32 {
                    if !((*var).lbnd).is_null() {
                        if (*var).lbnd == (*var).ubnd {
                            _glp_mpl_error(
                                mpl,
                                b"both fixed value and lower bound not allowed\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        } else {
                            _glp_mpl_error(
                                mpl,
                                b"at most one lower bound allowed\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        }
                    }
                    _glp_mpl_get_token(mpl);
                    (*var).lbnd = _glp_mpl_expression_5(mpl);
                    if (*(*var).lbnd).type_0 == 124 as i32 {
                        (*var).lbnd = _glp_mpl_make_unary(
                            mpl,
                            316 as i32,
                            (*var).lbnd,
                            118 as i32,
                            0 as i32,
                        );
                    }
                    if (*(*var).lbnd).type_0 != 118 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"expression following >= has invalid type\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                    ((*(*var).lbnd).dim == 0 as i32
                        || {
                            glp_assert_(
                                b"var->lbnd->dim == 0\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3546 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                } else if (*mpl).token == 231 as i32 {
                    if !((*var).ubnd).is_null() {
                        if (*var).ubnd == (*var).lbnd {
                            _glp_mpl_error(
                                mpl,
                                b"both fixed value and upper bound not allowed\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        } else {
                            _glp_mpl_error(
                                mpl,
                                b"at most one upper bound allowed\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        }
                    }
                    _glp_mpl_get_token(mpl);
                    (*var).ubnd = _glp_mpl_expression_5(mpl);
                    if (*(*var).ubnd).type_0 == 124 as i32 {
                        (*var).ubnd = _glp_mpl_make_unary(
                            mpl,
                            316 as i32,
                            (*var).ubnd,
                            118 as i32,
                            0 as i32,
                        );
                    }
                    if (*(*var).ubnd).type_0 != 118 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"expression following <= has invalid type\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                    ((*(*var).ubnd).dim == 0 as i32
                        || {
                            glp_assert_(
                                b"var->ubnd->dim == 0\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3565 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                } else if (*mpl).token == 232 as i32 {
                    let mut opstr: [i8; 8] = [0; 8];
                    if !(((*var).lbnd).is_null() && ((*var).ubnd).is_null()) {
                        if (*var).lbnd == (*var).ubnd {
                            _glp_mpl_error(
                                mpl,
                                b"at most one fixed value allowed\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        } else if !((*var).lbnd).is_null() {
                            _glp_mpl_error(
                                mpl,
                                b"both lower bound and fixed value not allowed\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        } else {
                            _glp_mpl_error(
                                mpl,
                                b"both upper bound and fixed value not allowed\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        }
                    }
                    strcpy(opstr.as_mut_ptr(), (*mpl).image);
                    (strlen(opstr.as_mut_ptr())
                        < ::core::mem::size_of::<[i8; 8]>() as u64
                        || {
                            glp_assert_(
                                b"strlen(opstr) < sizeof(opstr)\0" as *const u8
                                    as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3581 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    _glp_mpl_get_token(mpl);
                    (*var).lbnd = _glp_mpl_expression_5(mpl);
                    if (*(*var).lbnd).type_0 == 124 as i32 {
                        (*var).lbnd = _glp_mpl_make_unary(
                            mpl,
                            316 as i32,
                            (*var).lbnd,
                            118 as i32,
                            0 as i32,
                        );
                    }
                    if (*(*var).lbnd).type_0 != 118 as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"expression following %s has invalid type\0" as *const u8
                                as *const i8 as *mut i8,
                            opstr.as_mut_ptr(),
                        );
                    }
                    ((*(*var).lbnd).dim == 0 as i32
                        || {
                            glp_assert_(
                                b"var->lbnd->dim == 0\0" as *const u8 as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                3591 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*var).ubnd = (*var).lbnd;
                } else if (*mpl).token == 230 as i32 || (*mpl).token == 234 as i32
                    || (*mpl).token == 235 as i32
                {
                    _glp_mpl_error(
                        mpl,
                        b"strict bound not allowed\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"syntax error in variable statement\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                }
                current_block_104 = 17688141731389699982;
            }
            match current_block_104 {
                18382494810155246110 => {
                    if binary_used != 0 {
                        _glp_mpl_error(
                            mpl,
                            b"at most one binary allowed\0" as *const u8 as *const i8
                                as *mut i8,
                        );
                    }
                    (*var).type_0 = 101 as i32;
                    binary_used = 1 as i32;
                    _glp_mpl_get_token(mpl);
                }
                _ => {}
            }
        }
    }
    if !((*var).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*var).domain);
    }
    ((*mpl).token == 241 as i32
        || {
            glp_assert_(
                b"mpl->token == T_SEMICOLON\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3604 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    return var;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_constraint_statement(
    mut mpl: *mut MPL,
) -> *mut CONSTRAINT {
    let mut current_block: u64;
    let mut con: *mut CONSTRAINT = 0 as *mut CONSTRAINT;
    let mut first: *mut CODE = 0 as *mut CODE;
    let mut second: *mut CODE = 0 as *mut CODE;
    let mut third: *mut CODE = 0 as *mut CODE;
    let mut rho: i32 = 0;
    let mut opstr: [i8; 8] = [0; 8];
    if (*mpl).flag_s != 0 {
        _glp_mpl_error(
            mpl,
            b"constraint statement must precede solve statement\0" as *const u8
                as *const i8 as *mut i8,
        );
    }
    if _glp_mpl_is_keyword(mpl, b"subject\0" as *const u8 as *const i8 as *mut i8) != 0 {
        _glp_mpl_get_token(mpl);
        if _glp_mpl_is_keyword(mpl, b"to\0" as *const u8 as *const i8 as *mut i8) == 0 {
            _glp_mpl_error(
                mpl,
                b"keyword subject to incomplete\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        _glp_mpl_get_token(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"subj\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        _glp_mpl_get_token(mpl);
        if _glp_mpl_is_keyword(mpl, b"to\0" as *const u8 as *const i8 as *mut i8) == 0 {
            _glp_mpl_error(
                mpl,
                b"keyword subj to incomplete\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        _glp_mpl_get_token(mpl);
    } else if (*mpl).token == 220 as i32 {
        _glp_mpl_get_token(mpl);
    }
    if !((*mpl).token == 202 as i32) {
        if _glp_mpl_is_reserved(mpl) != 0 {
            _glp_mpl_error(
                mpl,
                b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                    as *mut i8,
                (*mpl).image,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"symbolic name missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if !(_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void)).is_null()
    {
        _glp_mpl_error(
            mpl,
            b"%s multiply declared\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    con = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<CONSTRAINT>() as u64 as i32,
    ) as *mut CONSTRAINT;
    (*con).name = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*con).name, (*mpl).image);
    (*con).alias = 0 as *mut i8;
    (*con).dim = 0 as i32;
    (*con).domain = 0 as *mut DOMAIN1;
    (*con).type_0 = 103 as i32;
    (*con).code = 0 as *mut CODE;
    (*con).lbnd = 0 as *mut CODE;
    (*con).ubnd = 0 as *mut CODE;
    (*con).array = 0 as *mut ARRAY;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 205 as i32 {
        (*con).alias = _glp_dmp_get_atom(
            (*mpl).pool,
            (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*con).alias, (*mpl).image);
        _glp_mpl_get_token(mpl);
    }
    if (*mpl).token == 248 as i32 {
        (*con).domain = _glp_mpl_indexing_expression(mpl);
        (*con).dim = _glp_mpl_domain_arity(mpl, (*con).domain);
    }
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    node = _glp_avl_insert_node((*mpl).tree, (*con).name as *const libc::c_void);
    _glp_avl_set_node_type(node, 103 as i32);
    _glp_avl_set_node_link(node, con as *mut libc::c_void);
    if (*mpl).token != 240 as i32 {
        _glp_mpl_error(
            mpl,
            b"colon missing where expected\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    first = _glp_mpl_expression_5(mpl);
    if (*first).type_0 == 124 as i32 {
        first = _glp_mpl_make_unary(mpl, 316 as i32, first, 118 as i32, 0 as i32);
    }
    if !((*first).type_0 == 118 as i32 || (*first).type_0 == 110 as i32) {
        _glp_mpl_error(
            mpl,
            b"expression following colon has invalid type\0" as *const u8 as *const i8
                as *mut i8,
        );
    }
    ((*first).dim == 0 as i32
        || {
            glp_assert_(
                b"first->dim == 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3705 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).token == 239 as i32 {
        _glp_mpl_get_token(mpl);
    }
    match (*mpl).token {
        231 | 233 | 232 => {
            rho = (*mpl).token;
            strcpy(opstr.as_mut_ptr(), (*mpl).image);
            (strlen(opstr.as_mut_ptr()) < ::core::mem::size_of::<[i8; 8]>() as u64
                || {
                    glp_assert_(
                        b"strlen(opstr) < sizeof(opstr)\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        3724 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_mpl_get_token(mpl);
            second = _glp_mpl_expression_5(mpl);
            if (*second).type_0 == 124 as i32 {
                second = _glp_mpl_make_unary(
                    mpl,
                    316 as i32,
                    second,
                    118 as i32,
                    0 as i32,
                );
            }
            if !((*second).type_0 == 118 as i32 || (*second).type_0 == 110 as i32) {
                _glp_mpl_error(
                    mpl,
                    b"expression following %s has invalid type\0" as *const u8
                        as *const i8 as *mut i8,
                    opstr.as_mut_ptr(),
                );
            }
            ((*second).dim == 0 as i32
                || {
                    glp_assert_(
                        b"second->dim == 0\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        3732 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*mpl).token == 239 as i32 {
                _glp_mpl_get_token(mpl);
                if (*mpl).token == 241 as i32 {
                    current_block = 4662060382018876917;
                } else {
                    current_block = 13303144130133872306;
                }
            } else {
                current_block = 13303144130133872306;
            }
            match current_block {
                4662060382018876917 => {}
                _ => {
                    if (*mpl).token == 230 as i32 || (*mpl).token == 231 as i32
                        || (*mpl).token == 232 as i32 || (*mpl).token == 233 as i32
                        || (*mpl).token == 234 as i32 || (*mpl).token == 235 as i32
                    {
                        if rho == 232 as i32 || (*mpl).token != rho {
                            _glp_mpl_error(
                                mpl,
                                b"double inequality must be ... <= ... <= ... or ... >= ... >= ...\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        }
                        if (*first).type_0 == 110 as i32 {
                            _glp_mpl_error(
                                mpl,
                                b"leftmost expression in double inequality cannot be linear form\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        }
                        _glp_mpl_get_token(mpl);
                        third = _glp_mpl_expression_5(mpl);
                        if (*third).type_0 == 124 as i32 {
                            third = _glp_mpl_make_unary(
                                mpl,
                                316 as i32,
                                second,
                                118 as i32,
                                0 as i32,
                            );
                        }
                        if !((*third).type_0 == 118 as i32
                            || (*third).type_0 == 110 as i32)
                        {
                            _glp_mpl_error(
                                mpl,
                                b"rightmost expression in double inequality constraint has invalid type\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        }
                        ((*third).dim == 0 as i32
                            || {
                                glp_assert_(
                                    b"third->dim == 0\0" as *const u8 as *const i8,
                                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                    3758 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        if (*third).type_0 == 110 as i32 {
                            _glp_mpl_error(
                                mpl,
                                b"rightmost expression in double inequality cannot be linear form\0"
                                    as *const u8 as *const i8 as *mut i8,
                            );
                        }
                    } else {
                        third = 0 as *mut CODE;
                    }
                    if !((*con).domain).is_null() {
                        _glp_mpl_close_scope(mpl, (*con).domain);
                    }
                    if (*first).type_0 != 110 as i32 {
                        first = _glp_mpl_make_unary(
                            mpl,
                            320 as i32,
                            first,
                            110 as i32,
                            0 as i32,
                        );
                    }
                    if (*second).type_0 != 110 as i32 {
                        second = _glp_mpl_make_unary(
                            mpl,
                            320 as i32,
                            second,
                            110 as i32,
                            0 as i32,
                        );
                    }
                    if !third.is_null() {
                        third = _glp_mpl_make_unary(
                            mpl,
                            320 as i32,
                            third,
                            110 as i32,
                            0 as i32,
                        );
                    }
                    if third.is_null() {
                        match rho {
                            231 => {
                                (*con).code = first;
                                (*con).lbnd = 0 as *mut CODE;
                                (*con).ubnd = second;
                            }
                            233 => {
                                (*con).code = first;
                                (*con).lbnd = second;
                                (*con).ubnd = 0 as *mut CODE;
                            }
                            232 => {
                                (*con).code = first;
                                (*con).lbnd = second;
                                (*con).ubnd = second;
                            }
                            _ => {
                                (rho != rho
                                    || {
                                        glp_assert_(
                                            b"rho != rho\0" as *const u8 as *const i8,
                                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                            3800 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                        }
                    } else {
                        match rho {
                            231 => {
                                (*con).code = second;
                                (*con).lbnd = first;
                                (*con).ubnd = third;
                            }
                            233 => {
                                (*con).code = second;
                                (*con).lbnd = third;
                                (*con).ubnd = first;
                            }
                            _ => {
                                (rho != rho
                                    || {
                                        glp_assert_(
                                            b"rho != rho\0" as *const u8 as *const i8,
                                            b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                            3819 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                        }
                    }
                    if (*mpl).token != 241 as i32 {
                        current_block = 4662060382018876917;
                    } else {
                        current_block = 2220405792722996547;
                    }
                }
            }
        }
        230 | 234 | 235 => {
            _glp_mpl_error(
                mpl,
                b"strict inequality not allowed\0" as *const u8 as *const i8 as *mut i8,
            );
            current_block = 10879965251760104617;
        }
        241 => {
            current_block = 10879965251760104617;
        }
        _ => {
            current_block = 4662060382018876917;
        }
    }
    match current_block {
        10879965251760104617 => {
            _glp_mpl_error(
                mpl,
                b"constraint must be equality or inequality\0" as *const u8 as *const i8
                    as *mut i8,
            );
            current_block = 4662060382018876917;
        }
        _ => {}
    }
    match current_block {
        4662060382018876917 => {
            _glp_mpl_error(
                mpl,
                b"syntax error in constraint statement\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        _ => {}
    }
    _glp_mpl_get_token(mpl);
    return con;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_objective_statement(
    mut mpl: *mut MPL,
) -> *mut CONSTRAINT {
    let mut obj: *mut CONSTRAINT = 0 as *mut CONSTRAINT;
    let mut type_0: i32 = 0;
    if _glp_mpl_is_keyword(mpl, b"minimize\0" as *const u8 as *const i8 as *mut i8) != 0
    {
        type_0 = 116 as i32;
    } else if _glp_mpl_is_keyword(
        mpl,
        b"maximize\0" as *const u8 as *const i8 as *mut i8,
    ) != 0
    {
        type_0 = 115 as i32;
    } else {
        (mpl != mpl
            || {
                glp_assert_(
                    b"mpl != mpl\0" as *const u8 as *const i8,
                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                    3852 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    if (*mpl).flag_s != 0 {
        _glp_mpl_error(
            mpl,
            b"objective statement must precede solve statement\0" as *const u8
                as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    if !((*mpl).token == 202 as i32) {
        if _glp_mpl_is_reserved(mpl) != 0 {
            _glp_mpl_error(
                mpl,
                b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                    as *mut i8,
                (*mpl).image,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"symbolic name missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if !(_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void)).is_null()
    {
        _glp_mpl_error(
            mpl,
            b"%s multiply declared\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    obj = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<CONSTRAINT>() as u64 as i32,
    ) as *mut CONSTRAINT;
    (*obj).name = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*obj).name, (*mpl).image);
    (*obj).alias = 0 as *mut i8;
    (*obj).dim = 0 as i32;
    (*obj).domain = 0 as *mut DOMAIN1;
    (*obj).type_0 = type_0;
    (*obj).code = 0 as *mut CODE;
    (*obj).lbnd = 0 as *mut CODE;
    (*obj).ubnd = 0 as *mut CODE;
    (*obj).array = 0 as *mut ARRAY;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 205 as i32 {
        (*obj).alias = _glp_dmp_get_atom(
            (*mpl).pool,
            (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*obj).alias, (*mpl).image);
        _glp_mpl_get_token(mpl);
    }
    if (*mpl).token == 248 as i32 {
        (*obj).domain = _glp_mpl_indexing_expression(mpl);
        (*obj).dim = _glp_mpl_domain_arity(mpl, (*obj).domain);
    }
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    node = _glp_avl_insert_node((*mpl).tree, (*obj).name as *const libc::c_void);
    _glp_avl_set_node_type(node, 103 as i32);
    _glp_avl_set_node_link(node, obj as *mut libc::c_void);
    if (*mpl).token != 240 as i32 {
        _glp_mpl_error(
            mpl,
            b"colon missing where expected\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    (*obj).code = _glp_mpl_expression_5(mpl);
    if (*(*obj).code).type_0 == 124 as i32 {
        (*obj).code = _glp_mpl_make_unary(
            mpl,
            316 as i32,
            (*obj).code,
            118 as i32,
            0 as i32,
        );
    }
    if (*(*obj).code).type_0 == 118 as i32 {
        (*obj).code = _glp_mpl_make_unary(
            mpl,
            320 as i32,
            (*obj).code,
            110 as i32,
            0 as i32,
        );
    }
    if (*(*obj).code).type_0 != 110 as i32 {
        _glp_mpl_error(
            mpl,
            b"expression following colon has invalid type\0" as *const u8 as *const i8
                as *mut i8,
        );
    }
    ((*(*obj).code).dim == 0 as i32
        || {
            glp_assert_(
                b"obj->code->dim == 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3908 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*obj).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*obj).domain);
    }
    if (*mpl).token != 241 as i32 {
        _glp_mpl_error(
            mpl,
            b"syntax error in objective statement\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_table_statement(mut mpl: *mut MPL) -> *mut TABLE {
    let mut current_block: u64;
    let mut tab: *mut TABLE = 0 as *mut TABLE;
    let mut last_arg: *mut TABARG = 0 as *mut TABARG;
    let mut arg: *mut TABARG = 0 as *mut TABARG;
    let mut last_fld: *mut TABFLD = 0 as *mut TABFLD;
    let mut fld: *mut TABFLD = 0 as *mut TABFLD;
    let mut last_in: *mut TABIN = 0 as *mut TABIN;
    let mut in_0: *mut TABIN = 0 as *mut TABIN;
    let mut last_out: *mut TABOUT = 0 as *mut TABOUT;
    let mut out: *mut TABOUT = 0 as *mut TABOUT;
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut nflds: i32 = 0;
    let mut name: [i8; 101] = [0; 101];
    (_glp_mpl_is_keyword(mpl, b"table\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"table\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                3962 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_get_token(mpl);
    if !((*mpl).token == 202 as i32) {
        if _glp_mpl_is_reserved(mpl) != 0 {
            _glp_mpl_error(
                mpl,
                b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                    as *mut i8,
                (*mpl).image,
            );
        } else {
            _glp_mpl_error(
                mpl,
                b"symbolic name missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if !(_glp_avl_find_node((*mpl).tree, (*mpl).image as *const libc::c_void)).is_null()
    {
        _glp_mpl_error(
            mpl,
            b"%s multiply declared\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    }
    tab = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<TABLE>() as u64 as i32)
        as *mut TABLE;
    (*tab).name = _glp_dmp_get_atom(
        (*mpl).pool,
        (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*tab).name, (*mpl).image);
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 205 as i32 {
        (*tab).alias = _glp_dmp_get_atom(
            (*mpl).pool,
            (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*tab).alias, (*mpl).image);
        _glp_mpl_get_token(mpl);
    } else {
        (*tab).alias = 0 as *mut i8;
    }
    if (*mpl).token == 248 as i32 {
        (*tab).type_0 = 119 as i32;
        (*tab).u.out.domain = _glp_mpl_indexing_expression(mpl);
        if _glp_mpl_is_keyword(mpl, b"OUT\0" as *const u8 as *const i8 as *mut i8) == 0 {
            _glp_mpl_error(
                mpl,
                b"keyword OUT missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        _glp_mpl_get_token(mpl);
    } else {
        (*tab).type_0 = 112 as i32;
        if _glp_mpl_is_keyword(mpl, b"IN\0" as *const u8 as *const i8 as *mut i8) == 0 {
            _glp_mpl_error(
                mpl,
                b"keyword IN missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        _glp_mpl_get_token(mpl);
    }
    last_arg = 0 as *mut TABARG;
    (*tab).arg = last_arg;
    loop {
        arg = _glp_dmp_get_atom(
            (*mpl).pool,
            ::core::mem::size_of::<TABARG>() as u64 as i32,
        ) as *mut TABARG;
        if (*mpl).token == 239 as i32 || (*mpl).token == 240 as i32
            || (*mpl).token == 241 as i32
        {
            _glp_mpl_error(
                mpl,
                b"argument expression missing where expected\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*arg).code = _glp_mpl_expression_5(mpl);
        if (*(*arg).code).type_0 == 118 as i32 {
            (*arg).code = _glp_mpl_make_unary(
                mpl,
                317 as i32,
                (*arg).code,
                124 as i32,
                0 as i32,
            );
        }
        if (*(*arg).code).type_0 != 124 as i32 {
            _glp_mpl_error(
                mpl,
                b"argument expression has invalid type\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*arg).next = 0 as *mut TABARG;
        if last_arg.is_null() {
            (*tab).arg = arg;
        } else {
            (*last_arg).next = arg;
        }
        last_arg = arg;
        if (*mpl).token == 239 as i32 {
            _glp_mpl_get_token(mpl);
        } else if (*mpl).token == 240 as i32 || (*mpl).token == 241 as i32 {
            break;
        }
    }
    (!((*tab).arg).is_null()
        || {
            glp_assert_(
                b"tab->arg != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4033 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).token == 240 as i32 {
        _glp_mpl_get_token(mpl);
    } else {
        _glp_mpl_error(
            mpl,
            b"colon missing where expected\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    match (*tab).type_0 {
        112 => {
            current_block = 13581483217438886834;
        }
        119 => {
            last_out = 0 as *mut TABOUT;
            (*tab).u.out.list = last_out;
            loop {
                out = _glp_dmp_get_atom(
                    (*mpl).pool,
                    ::core::mem::size_of::<TABOUT>() as u64 as i32,
                ) as *mut TABOUT;
                if (*mpl).token == 239 as i32 || (*mpl).token == 241 as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"expression missing where expected\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                }
                if (*mpl).token == 202 as i32 {
                    (strlen((*mpl).image) < ::core::mem::size_of::<[i8; 101]>() as u64
                        || {
                            glp_assert_(
                                b"strlen(mpl->image) < sizeof(name)\0" as *const u8
                                    as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                4179 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    strcpy(name.as_mut_ptr(), (*mpl).image);
                } else {
                    name[0 as i32 as usize] = '\0' as i32 as i8;
                }
                (*out).code = _glp_mpl_expression_5(mpl);
                if (*mpl).token == 251 as i32 {
                    _glp_mpl_get_token(mpl);
                    if !((*mpl).token == 202 as i32) {
                        if _glp_mpl_is_reserved(mpl) != 0 {
                            _glp_mpl_error(
                                mpl,
                                b"invalid use of reserved keyword %s\0" as *const u8
                                    as *const i8 as *mut i8,
                                (*mpl).image,
                            );
                        } else {
                            _glp_mpl_error(
                                mpl,
                                b"field name missing where expected\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        }
                    }
                    (strlen((*mpl).image) < ::core::mem::size_of::<[i8; 101]>() as u64
                        || {
                            glp_assert_(
                                b"strlen(mpl->image) < sizeof(name)\0" as *const u8
                                    as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                4196 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    strcpy(name.as_mut_ptr(), (*mpl).image);
                    _glp_mpl_get_token(mpl);
                }
                if name[0 as i32 as usize] as i32 == '\0' as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"field name required\0" as *const u8 as *const i8 as *mut i8,
                    );
                }
                (*out).name = _glp_dmp_get_atom(
                    (*mpl).pool,
                    (strlen(name.as_mut_ptr())).wrapping_add(1 as i32 as u64) as i32,
                ) as *mut i8;
                strcpy((*out).name, name.as_mut_ptr());
                (*out).next = 0 as *mut TABOUT;
                if last_out.is_null() {
                    (*tab).u.out.list = out;
                } else {
                    (*last_out).next = out;
                }
                last_out = out;
                if (*mpl).token == 239 as i32 {
                    _glp_mpl_get_token(mpl);
                } else {
                    if (*mpl).token == 241 as i32 {
                        break;
                    }
                    _glp_mpl_error(
                        mpl,
                        b"syntax error in output list\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                }
            }
            _glp_mpl_close_scope(mpl, (*tab).u.out.domain);
            current_block = 12431930673972400805;
        }
        _ => {
            (tab != tab
                || {
                    glp_assert_(
                        b"tab != tab\0" as *const u8 as *const i8,
                        b"mpl/mpl1.c\0" as *const u8 as *const i8,
                        4043 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block = 13581483217438886834;
        }
    }
    match current_block {
        13581483217438886834 => {
            if (*mpl).token == 202 as i32 {
                node = _glp_avl_find_node(
                    (*mpl).tree,
                    (*mpl).image as *const libc::c_void,
                );
                if node.is_null() {
                    _glp_mpl_error(
                        mpl,
                        b"%s not defined\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                if _glp_avl_get_node_type(node) != 122 as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"%s not a set\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                (*tab).u.in_0.set = _glp_avl_get_node_link(node) as *mut SET;
                if !((*(*tab).u.in_0.set).assign).is_null() {
                    _glp_mpl_error(
                        mpl,
                        b"%s needs no data\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                if (*(*tab).u.in_0.set).dim != 0 as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"%s must be a simple set\0" as *const u8 as *const i8
                            as *mut i8,
                        (*mpl).image,
                    );
                }
                _glp_mpl_get_token(mpl);
                if (*mpl).token == 252 as i32 {
                    _glp_mpl_get_token(mpl);
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"delimiter <- missing where expected\0" as *const u8
                            as *const i8 as *mut i8,
                    );
                }
            } else if _glp_mpl_is_reserved(mpl) != 0 {
                _glp_mpl_error(
                    mpl,
                    b"invalid use of reserved keyword %s\0" as *const u8 as *const i8
                        as *mut i8,
                    (*mpl).image,
                );
            } else {
                (*tab).u.in_0.set = 0 as *mut SET;
            }
            last_fld = 0 as *mut TABFLD;
            (*tab).u.in_0.fld = last_fld;
            nflds = 0 as i32;
            if (*mpl).token == 246 as i32 {
                _glp_mpl_get_token(mpl);
            } else {
                _glp_mpl_error(
                    mpl,
                    b"field list missing where expected\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
            loop {
                fld = _glp_dmp_get_atom(
                    (*mpl).pool,
                    ::core::mem::size_of::<TABFLD>() as u64 as i32,
                ) as *mut TABFLD;
                if !((*mpl).token == 202 as i32) {
                    if _glp_mpl_is_reserved(mpl) != 0 {
                        _glp_mpl_error(
                            mpl,
                            b"invalid use of reserved keyword %s\0" as *const u8
                                as *const i8 as *mut i8,
                            (*mpl).image,
                        );
                    } else {
                        _glp_mpl_error(
                            mpl,
                            b"field name missing where expected\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                }
                (*fld).name = _glp_dmp_get_atom(
                    (*mpl).pool,
                    (strlen((*mpl).image)).wrapping_add(1 as i32 as u64) as i32,
                ) as *mut i8;
                strcpy((*fld).name, (*mpl).image);
                _glp_mpl_get_token(mpl);
                (*fld).next = 0 as *mut TABFLD;
                if last_fld.is_null() {
                    (*tab).u.in_0.fld = fld;
                } else {
                    (*last_fld).next = fld;
                }
                last_fld = fld;
                nflds += 1;
                nflds;
                if (*mpl).token == 239 as i32 {
                    _glp_mpl_get_token(mpl);
                } else {
                    if (*mpl).token == 247 as i32 {
                        break;
                    }
                    _glp_mpl_error(
                        mpl,
                        b"syntax error in field list\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                }
            }
            if !((*tab).u.in_0.set).is_null() && (*(*tab).u.in_0.set).dimen != nflds {
                _glp_mpl_error(
                    mpl,
                    b"there must be %d field%s rather than %d\0" as *const u8
                        as *const i8 as *mut i8,
                    (*(*tab).u.in_0.set).dimen,
                    if (*(*tab).u.in_0.set).dimen == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                    nflds,
                );
            }
            _glp_mpl_get_token(mpl);
            last_in = 0 as *mut TABIN;
            (*tab).u.in_0.list = last_in;
            while (*mpl).token == 239 as i32 {
                _glp_mpl_get_token(mpl);
                in_0 = _glp_dmp_get_atom(
                    (*mpl).pool,
                    ::core::mem::size_of::<TABIN>() as u64 as i32,
                ) as *mut TABIN;
                if !((*mpl).token == 202 as i32) {
                    if _glp_mpl_is_reserved(mpl) != 0 {
                        _glp_mpl_error(
                            mpl,
                            b"invalid use of reserved keyword %s\0" as *const u8
                                as *const i8 as *mut i8,
                            (*mpl).image,
                        );
                    } else {
                        _glp_mpl_error(
                            mpl,
                            b"parameter name missing where expected\0" as *const u8
                                as *const i8 as *mut i8,
                        );
                    }
                }
                node = _glp_avl_find_node(
                    (*mpl).tree,
                    (*mpl).image as *const libc::c_void,
                );
                if node.is_null() {
                    _glp_mpl_error(
                        mpl,
                        b"%s not defined\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                if _glp_avl_get_node_type(node) != 120 as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"%s not a parameter\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                (*in_0).par = _glp_avl_get_node_link(node) as *mut PARAMETER;
                if (*(*in_0).par).dim != nflds {
                    _glp_mpl_error(
                        mpl,
                        b"%s must have %d subscript%s rather than %d\0" as *const u8
                            as *const i8 as *mut i8,
                        (*mpl).image,
                        nflds,
                        if nflds == 1 as i32 {
                            b"\0" as *const u8 as *const i8
                        } else {
                            b"s\0" as *const u8 as *const i8
                        },
                        (*(*in_0).par).dim,
                    );
                }
                if !((*(*in_0).par).assign).is_null() {
                    _glp_mpl_error(
                        mpl,
                        b"%s needs no data\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                _glp_mpl_get_token(mpl);
                if (*mpl).token == 251 as i32 {
                    _glp_mpl_get_token(mpl);
                    if !((*mpl).token == 202 as i32) {
                        if _glp_mpl_is_reserved(mpl) != 0 {
                            _glp_mpl_error(
                                mpl,
                                b"invalid use of reserved keyword %s\0" as *const u8
                                    as *const i8 as *mut i8,
                                (*mpl).image,
                            );
                        } else {
                            _glp_mpl_error(
                                mpl,
                                b"field name missing where expected\0" as *const u8
                                    as *const i8 as *mut i8,
                            );
                        }
                    }
                    (strlen((*mpl).image) < ::core::mem::size_of::<[i8; 101]>() as u64
                        || {
                            glp_assert_(
                                b"strlen(mpl->image) < sizeof(name)\0" as *const u8
                                    as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                4148 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    strcpy(name.as_mut_ptr(), (*mpl).image);
                    _glp_mpl_get_token(mpl);
                } else {
                    (strlen((*(*in_0).par).name)
                        < ::core::mem::size_of::<[i8; 101]>() as u64
                        || {
                            glp_assert_(
                                b"strlen(in->par->name) < sizeof(name)\0" as *const u8
                                    as *const i8,
                                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                4154 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    strcpy(name.as_mut_ptr(), (*(*in_0).par).name);
                }
                (*in_0).name = _glp_dmp_get_atom(
                    (*mpl).pool,
                    (strlen(name.as_mut_ptr())).wrapping_add(1 as i32 as u64) as i32,
                ) as *mut i8;
                strcpy((*in_0).name, name.as_mut_ptr());
                (*in_0).next = 0 as *mut TABIN;
                if last_in.is_null() {
                    (*tab).u.in_0.list = in_0;
                } else {
                    (*last_in).next = in_0;
                }
                last_in = in_0;
            }
        }
        _ => {}
    }
    if (*mpl).token != 241 as i32 {
        _glp_mpl_error(
            mpl,
            b"syntax error in table statement\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    return tab;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_solve_statement(
    mut mpl: *mut MPL,
) -> *mut libc::c_void {
    (_glp_mpl_is_keyword(mpl, b"solve\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"solve\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4241 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).flag_s != 0 {
        _glp_mpl_error(
            mpl,
            b"at most one solve statement allowed\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    (*mpl).flag_s = 1 as i32;
    _glp_mpl_get_token(mpl);
    if (*mpl).token != 241 as i32 {
        _glp_mpl_error(
            mpl,
            b"syntax error in solve statement\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_check_statement(mut mpl: *mut MPL) -> *mut CHECK {
    let mut chk: *mut CHECK = 0 as *mut CHECK;
    (_glp_mpl_is_keyword(mpl, b"check\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"check\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4266 as i32,
            );
            1 as i32 != 0
        }) as i32;
    chk = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<CHECK>() as u64 as i32)
        as *mut CHECK;
    (*chk).domain = 0 as *mut DOMAIN1;
    (*chk).code = 0 as *mut CODE;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 248 as i32 {
        (*chk).domain = _glp_mpl_indexing_expression(mpl);
    }
    if (*mpl).token == 240 as i32 {
        _glp_mpl_get_token(mpl);
    }
    (*chk).code = _glp_mpl_expression_13(mpl);
    if (*(*chk).code).type_0 != 114 as i32 {
        _glp_mpl_error(
            mpl,
            b"expression has invalid type\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    ((*(*chk).code).dim == 0 as i32
        || {
            glp_assert_(
                b"chk->code->dim == 0\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4286 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*chk).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*chk).domain);
    }
    if (*mpl).token != 241 as i32 {
        _glp_mpl_error(
            mpl,
            b"syntax error in check statement\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    return chk;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_display_statement(mut mpl: *mut MPL) -> *mut DISPLAY {
    let mut dpy: *mut DISPLAY = 0 as *mut DISPLAY;
    let mut entry: *mut DISPLAY1 = 0 as *mut DISPLAY1;
    let mut last_entry: *mut DISPLAY1 = 0 as *mut DISPLAY1;
    (_glp_mpl_is_keyword(mpl, b"display\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"display\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4322 as i32,
            );
            1 as i32 != 0
        }) as i32;
    dpy = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<DISPLAY>() as u64 as i32)
        as *mut DISPLAY;
    (*dpy).domain = 0 as *mut DOMAIN1;
    last_entry = 0 as *mut DISPLAY1;
    (*dpy).list = last_entry;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 248 as i32 {
        (*dpy).domain = _glp_mpl_indexing_expression(mpl);
    }
    if (*mpl).token == 240 as i32 {
        _glp_mpl_get_token(mpl);
    }
    loop {
        entry = _glp_dmp_get_atom(
            (*mpl).pool,
            ::core::mem::size_of::<DISPLAY1>() as u64 as i32,
        ) as *mut DISPLAY1;
        (*entry).type_0 = 0 as i32;
        (*entry).next = 0 as *mut DISPLAY1;
        if ((*dpy).list).is_null() {
            (*dpy).list = entry;
        } else {
            (*last_entry).next = entry;
        }
        last_entry = entry;
        let mut current_block_37: u64;
        if (*mpl).token == 202 as i32 {
            let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
            let mut next_token: i32 = 0;
            _glp_mpl_get_token(mpl);
            next_token = (*mpl).token;
            _glp_mpl_unget_token(mpl);
            if !(next_token == 239 as i32 || next_token == 241 as i32) {
                current_block_37 = 9774309220039542226;
            } else {
                node = _glp_avl_find_node(
                    (*mpl).tree,
                    (*mpl).image as *const libc::c_void,
                );
                if node.is_null() {
                    _glp_mpl_error(
                        mpl,
                        b"%s not defined\0" as *const u8 as *const i8 as *mut i8,
                        (*mpl).image,
                    );
                }
                (*entry).type_0 = _glp_avl_get_node_type(node);
                match _glp_avl_get_node_type(node) {
                    111 => {
                        (*entry).u.slot = _glp_avl_get_node_link(node)
                            as *mut DOMAIN_SLOT;
                    }
                    122 => {
                        (*entry).u.set = _glp_avl_get_node_link(node) as *mut SET;
                    }
                    120 => {
                        (*entry).u.par = _glp_avl_get_node_link(node) as *mut PARAMETER;
                    }
                    127 => {
                        (*entry).u.var = _glp_avl_get_node_link(node) as *mut VARIABLE;
                        if (*mpl).flag_s == 0 {
                            _glp_mpl_error(
                                mpl,
                                b"invalid reference to variable %s above solve statement\0"
                                    as *const u8 as *const i8 as *mut i8,
                                (*(*entry).u.var).name,
                            );
                        }
                    }
                    103 => {
                        (*entry).u.con = _glp_avl_get_node_link(node) as *mut CONSTRAINT;
                        if (*mpl).flag_s == 0 {
                            _glp_mpl_error(
                                mpl,
                                b"invalid reference to %s %s above solve statement\0"
                                    as *const u8 as *const i8 as *mut i8,
                                if (*(*entry).u.con).type_0 == 103 as i32 {
                                    b"constraint\0" as *const u8 as *const i8
                                } else {
                                    b"objective\0" as *const u8 as *const i8
                                },
                                (*(*entry).u.con).name,
                            );
                        }
                    }
                    _ => {
                        (node != node
                            || {
                                glp_assert_(
                                    b"node != node\0" as *const u8 as *const i8,
                                    b"mpl/mpl1.c\0" as *const u8 as *const i8,
                                    4387 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                _glp_mpl_get_token(mpl);
                current_block_37 = 11048769245176032998;
            }
        } else {
            current_block_37 = 9774309220039542226;
        }
        match current_block_37 {
            9774309220039542226 => {
                (*entry).type_0 = 108 as i32;
                (*entry).u.code = _glp_mpl_expression_13(mpl);
            }
            _ => {}
        }
        if !((*mpl).token == 239 as i32) {
            break;
        }
        _glp_mpl_get_token(mpl);
    }
    if !((*dpy).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*dpy).domain);
    }
    if (*mpl).token != 241 as i32 {
        _glp_mpl_error(
            mpl,
            b"syntax error in display statement\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    return dpy;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_printf_statement(mut mpl: *mut MPL) -> *mut PRINTF {
    let mut prt: *mut PRINTF = 0 as *mut PRINTF;
    let mut entry: *mut PRINTF1 = 0 as *mut PRINTF1;
    let mut last_entry: *mut PRINTF1 = 0 as *mut PRINTF1;
    (_glp_mpl_is_keyword(mpl, b"printf\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"printf\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4433 as i32,
            );
            1 as i32 != 0
        }) as i32;
    prt = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<PRINTF>() as u64 as i32)
        as *mut PRINTF;
    (*prt).domain = 0 as *mut DOMAIN1;
    (*prt).fmt = 0 as *mut CODE;
    last_entry = 0 as *mut PRINTF1;
    (*prt).list = last_entry;
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 248 as i32 {
        (*prt).domain = _glp_mpl_indexing_expression(mpl);
    }
    if (*mpl).token == 240 as i32 {
        _glp_mpl_get_token(mpl);
    }
    (*prt).fmt = _glp_mpl_expression_5(mpl);
    if (*(*prt).fmt).type_0 == 118 as i32 {
        (*prt).fmt = _glp_mpl_make_unary(
            mpl,
            317 as i32,
            (*prt).fmt,
            124 as i32,
            0 as i32,
        );
    }
    if (*(*prt).fmt).type_0 != 124 as i32 {
        _glp_mpl_error(
            mpl,
            b"format expression has invalid type\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    while (*mpl).token == 239 as i32 {
        _glp_mpl_get_token(mpl);
        entry = _glp_dmp_get_atom(
            (*mpl).pool,
            ::core::mem::size_of::<PRINTF1>() as u64 as i32,
        ) as *mut PRINTF1;
        (*entry).code = 0 as *mut CODE;
        (*entry).next = 0 as *mut PRINTF1;
        if ((*prt).list).is_null() {
            (*prt).list = entry;
        } else {
            (*last_entry).next = entry;
        }
        last_entry = entry;
        (*entry).code = _glp_mpl_expression_9(mpl);
        if !((*(*entry).code).type_0 == 118 as i32
            || (*(*entry).code).type_0 == 124 as i32
            || (*(*entry).code).type_0 == 114 as i32)
        {
            _glp_mpl_error(
                mpl,
                b"only numeric, symbolic, or logical expression allowed\0" as *const u8
                    as *const i8 as *mut i8,
            );
        }
    }
    if !((*prt).domain).is_null() {
        _glp_mpl_close_scope(mpl, (*prt).domain);
    }
    (*prt).fname = 0 as *mut CODE;
    (*prt).app = 0 as i32;
    if (*mpl).token == 234 as i32 || (*mpl).token == 250 as i32 {
        (*prt).app = ((*mpl).token == 250 as i32) as i32;
        _glp_mpl_get_token(mpl);
        (*prt).fname = _glp_mpl_expression_5(mpl);
        if (*(*prt).fname).type_0 == 118 as i32 {
            (*prt).fname = _glp_mpl_make_unary(
                mpl,
                317 as i32,
                (*prt).fname,
                124 as i32,
                0 as i32,
            );
        }
        if (*(*prt).fname).type_0 != 124 as i32 {
            _glp_mpl_error(
                mpl,
                b"file name expression has invalid type\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
    }
    if (*mpl).token != 241 as i32 {
        _glp_mpl_error(
            mpl,
            b"syntax error in printf statement\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    _glp_mpl_get_token(mpl);
    return prt;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_for_statement(mut mpl: *mut MPL) -> *mut FOR {
    let mut fur: *mut FOR = 0 as *mut FOR;
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    let mut last_stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    (_glp_mpl_is_keyword(mpl, b"for\0" as *const u8 as *const i8 as *mut i8) != 0
        || {
            glp_assert_(
                b"is_keyword(mpl, \"for\")\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4523 as i32,
            );
            1 as i32 != 0
        }) as i32;
    fur = _glp_dmp_get_atom((*mpl).pool, ::core::mem::size_of::<FOR>() as u64 as i32)
        as *mut FOR;
    (*fur).domain = 0 as *mut DOMAIN1;
    last_stmt = 0 as *mut STATEMENT;
    (*fur).list = last_stmt;
    _glp_mpl_get_token(mpl);
    if (*mpl).token != 248 as i32 {
        _glp_mpl_error(
            mpl,
            b"indexing expression missing where expected\0" as *const u8 as *const i8
                as *mut i8,
        );
    }
    (*fur).domain = _glp_mpl_indexing_expression(mpl);
    if (*mpl).token == 240 as i32 {
        _glp_mpl_get_token(mpl);
    }
    if (*mpl).token != 248 as i32 {
        (*fur).list = _glp_mpl_simple_statement(mpl, 1 as i32);
    } else {
        _glp_mpl_get_token(mpl);
        while (*mpl).token != 249 as i32 {
            stmt = _glp_mpl_simple_statement(mpl, 1 as i32);
            if last_stmt.is_null() {
                (*fur).list = stmt;
            } else {
                (*last_stmt).next = stmt;
            }
            last_stmt = stmt;
        }
        _glp_mpl_get_token(mpl);
    }
    (!((*fur).domain).is_null()
        || {
            glp_assert_(
                b"fur->domain != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4556 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpl_close_scope(mpl, (*fur).domain);
    return fur;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_end_statement(mut mpl: *mut MPL) {
    if (*mpl).flag_d == 0
        && _glp_mpl_is_keyword(mpl, b"end\0" as *const u8 as *const i8 as *mut i8) != 0
        || (*mpl).flag_d != 0
            && _glp_mpl_is_literal(mpl, b"end\0" as *const u8 as *const i8 as *mut i8)
                != 0
    {
        _glp_mpl_get_token(mpl);
        if (*mpl).token == 241 as i32 {
            _glp_mpl_get_token(mpl);
        } else {
            _glp_mpl_warning(
                mpl,
                b"no semicolon following end statement; missing semicolon inserted\0"
                    as *const u8 as *const i8 as *mut i8,
            );
        }
    } else {
        _glp_mpl_warning(
            mpl,
            b"unexpected end of file; missing end statement inserted\0" as *const u8
                as *const i8 as *mut i8,
        );
    }
    if (*mpl).token != 201 as i32 {
        _glp_mpl_warning(
            mpl,
            b"some text detected beyond end statement; text ignored\0" as *const u8
                as *const i8 as *mut i8,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_simple_statement(
    mut mpl: *mut MPL,
    mut spec: i32,
) -> *mut STATEMENT {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    stmt = _glp_dmp_get_atom(
        (*mpl).pool,
        ::core::mem::size_of::<STATEMENT>() as u64 as i32,
    ) as *mut STATEMENT;
    (*stmt).line = (*mpl).line;
    (*stmt).next = 0 as *mut STATEMENT;
    if _glp_mpl_is_keyword(mpl, b"set\0" as *const u8 as *const i8 as *mut i8) != 0 {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"set statement not allowed here\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        (*stmt).type_0 = 122 as i32;
        (*stmt).u.set = _glp_mpl_set_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"param\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"parameter statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 120 as i32;
        (*stmt).u.par = _glp_mpl_parameter_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"var\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"variable statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 127 as i32;
        (*stmt).u.var = _glp_mpl_variable_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"subject\0" as *const u8 as *const i8 as *mut i8)
        != 0
        || _glp_mpl_is_keyword(mpl, b"subj\0" as *const u8 as *const i8 as *mut i8) != 0
        || (*mpl).token == 220 as i32
    {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"constraint statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 103 as i32;
        (*stmt).u.con = _glp_mpl_constraint_statement(mpl);
    } else if _glp_mpl_is_keyword(
        mpl,
        b"minimize\0" as *const u8 as *const i8 as *mut i8,
    ) != 0
        || _glp_mpl_is_keyword(mpl, b"maximize\0" as *const u8 as *const i8 as *mut i8)
            != 0
    {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"objective statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 103 as i32;
        (*stmt).u.con = _glp_mpl_objective_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"table\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"table statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 125 as i32;
        (*stmt).u.tab = _glp_mpl_table_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"solve\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"solve statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 123 as i32;
        (*stmt).u.slv = _glp_mpl_solve_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"check\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        (*stmt).type_0 = 102 as i32;
        (*stmt).u.chk = _glp_mpl_check_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"display\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        (*stmt).type_0 = 104 as i32;
        (*stmt).u.dpy = _glp_mpl_display_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"printf\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        (*stmt).type_0 = 121 as i32;
        (*stmt).u.prt = _glp_mpl_printf_statement(mpl);
    } else if _glp_mpl_is_keyword(mpl, b"for\0" as *const u8 as *const i8 as *mut i8)
        != 0
    {
        (*stmt).type_0 = 109 as i32;
        (*stmt).u.fur = _glp_mpl_for_statement(mpl);
    } else if (*mpl).token == 202 as i32 {
        if spec != 0 {
            _glp_mpl_error(
                mpl,
                b"constraint statement not allowed here\0" as *const u8 as *const i8
                    as *mut i8,
            );
        }
        (*stmt).type_0 = 103 as i32;
        (*stmt).u.con = _glp_mpl_constraint_statement(mpl);
    } else if _glp_mpl_is_reserved(mpl) != 0 {
        _glp_mpl_error(
            mpl,
            b"invalid use of reserved keyword %s\0" as *const u8 as *const i8 as *mut i8,
            (*mpl).image,
        );
    } else {
        _glp_mpl_error(
            mpl,
            b"syntax error in model section\0" as *const u8 as *const i8 as *mut i8,
        );
    }
    return stmt;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_model_section(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    let mut last_stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    (((*mpl).model).is_null()
        || {
            glp_assert_(
                b"mpl->model == NULL\0" as *const u8 as *const i8,
                b"mpl/mpl1.c\0" as *const u8 as *const i8,
                4700 as i32,
            );
            1 as i32 != 0
        }) as i32;
    last_stmt = 0 as *mut STATEMENT;
    while !((*mpl).token == 201 as i32
        || _glp_mpl_is_keyword(mpl, b"data\0" as *const u8 as *const i8 as *mut i8) != 0
        || _glp_mpl_is_keyword(mpl, b"end\0" as *const u8 as *const i8 as *mut i8) != 0)
    {
        stmt = _glp_mpl_simple_statement(mpl, 0 as i32);
        if last_stmt.is_null() {
            (*mpl).model = stmt;
        } else {
            (*last_stmt).next = stmt;
        }
        last_stmt = stmt;
    }
}