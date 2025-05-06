#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type AVL;
    pub type DMP;
    pub type glp_file;
    static mut stdout: *mut _IO_FILE;
    fn vsprintf(_: *mut i8, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn _glp_avl_strcmp(
        info: *mut libc::c_void,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> i32;
    fn _glp_avl_create_tree(
        fcmp: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                *const libc::c_void,
            ) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> *mut AVL;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_in_use(pool: *mut DMP) -> size_t;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_get_err_msg() -> *const i8;
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_getc(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_rng_create_rand() -> *mut RNG;
    fn _glp_rng_delete_rand(rand: *mut RNG);
    fn _glp_mpl_print_context(mpl: *mut MPL);
    fn _glp_mpl_get_char(mpl: *mut MPL);
    fn _glp_mpl_get_token(mpl: *mut MPL);
    fn _glp_mpl_is_keyword(mpl: *mut MPL, keyword: *mut i8) -> i32;
    fn _glp_mpl_end_statement(mpl: *mut MPL);
    fn _glp_mpl_model_section(mpl: *mut MPL);
    fn _glp_mpl_is_literal(mpl: *mut MPL, literal: *mut i8) -> i32;
    fn _glp_mpl_data_section(mpl: *mut MPL);
    fn _glp_mpl_format_tuple(mpl: *mut MPL, c: i32, tuple: *mut TUPLE) -> *mut i8;
    fn _glp_mpl_create_array(mpl: *mut MPL, type_0: i32, dim: i32) -> *mut ARRAY;
    fn _glp_mpl_free_dca(mpl: *mut MPL);
    fn _glp_mpl_execute_statement(mpl: *mut MPL, stmt: *mut STATEMENT);
    fn _glp_mpl_clean_statement(mpl: *mut MPL, stmt: *mut STATEMENT);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type va_list = __builtin_va_list;
pub type size_t = u64;
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
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
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
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_alloc_content(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        match (*stmt).type_0 {
            122 => {
                (((*(*stmt).u.set).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.set->array == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            48 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*stmt).u.set).array = _glp_mpl_create_array(
                    mpl,
                    106 as i32,
                    (*(*stmt).u.set).dim,
                );
            }
            120 => {
                (((*(*stmt).u.par).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.par->array == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            54 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                match (*(*stmt).u.par).type_0 {
                    118 | 113 | 101 => {
                        (*(*stmt).u.par).array = _glp_mpl_create_array(
                            mpl,
                            118 as i32,
                            (*(*stmt).u.par).dim,
                        );
                    }
                    124 => {
                        (*(*stmt).u.par).array = _glp_mpl_create_array(
                            mpl,
                            124 as i32,
                            (*(*stmt).u.par).dim,
                        );
                    }
                    _ => {
                        (stmt != stmt
                            || {
                                glp_assert_(
                                    b"stmt != stmt\0" as *const u8 as *const i8,
                                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                                    67 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
            }
            127 => {
                (((*(*stmt).u.var).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.var->array == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            72 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*stmt).u.var).array = _glp_mpl_create_array(
                    mpl,
                    107 as i32,
                    (*(*stmt).u.var).dim,
                );
            }
            103 => {
                (((*(*stmt).u.con).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.con->array == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            78 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*(*stmt).u.con).array = _glp_mpl_create_array(
                    mpl,
                    105 as i32,
                    (*(*stmt).u.con).dim,
                );
            }
            125 | 123 | 102 | 104 | 121 | 109 => {}
            _ => {
                (stmt != stmt
                    || {
                        glp_assert_(
                            b"stmt != stmt\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            93 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        stmt = (*stmt).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_generate_model(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    ((*mpl).flag_p == 0
        || {
            glp_assert_(
                b"!mpl->flag_p\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                107 as i32,
            );
            1 as i32 != 0
        }) as i32;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        _glp_mpl_execute_statement(mpl, stmt);
        if (*(*mpl).stmt).type_0 == 123 as i32 {
            break;
        }
        stmt = (*stmt).next;
    }
    (*mpl).stmt = stmt;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_build_problem(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut v: *mut VARIABLE = 0 as *mut VARIABLE;
    let mut c: *mut CONSTRAINT = 0 as *mut CONSTRAINT;
    let mut t: *mut FORMULA = 0 as *mut FORMULA;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    ((*mpl).m == 0 as i32
        || {
            glp_assert_(
                b"mpl->m == 0\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                129 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*mpl).n == 0 as i32
        || {
            glp_assert_(
                b"mpl->n == 0\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                130 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (((*mpl).row).is_null()
        || {
            glp_assert_(
                b"mpl->row == NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                131 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (((*mpl).col).is_null()
        || {
            glp_assert_(
                b"mpl->col == NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                132 as i32,
            );
            1 as i32 != 0
        }) as i32;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 127 as i32 {
            v = (*stmt).u.var;
            memb = (*(*v).array).head;
            while !memb.is_null() {
                ((*(*memb).value.var).j == 0 as i32
                    || {
                        glp_assert_(
                            b"memb->value.var->j == 0\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            138 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 103 as i32 {
            c = (*stmt).u.con;
            memb = (*(*c).array).head;
            while !memb.is_null() {
                ((*(*memb).value.con).i == 0 as i32
                    || {
                        glp_assert_(
                            b"memb->value.con->i == 0\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            146 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*mpl).m += 1;
                (*(*memb).value.con).i = (*mpl).m;
                t = (*(*memb).value.con).form;
                while !t.is_null() {
                    (!((*t).var).is_null()
                        || {
                            glp_assert_(
                                b"t->var != NULL\0" as *const u8 as *const i8,
                                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                                151 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*(*(*(*t).var).memb).value.var).j = -(1 as i32);
                    t = (*t).next;
                }
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 127 as i32 {
            v = (*stmt).u.var;
            memb = (*(*v).array).head;
            while !memb.is_null() {
                if (*(*memb).value.var).j != 0 as i32 {
                    (*mpl).n += 1;
                    (*(*memb).value.var).j = (*mpl).n;
                }
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    (*mpl).row = glp_alloc(
        1 as i32 + (*mpl).m,
        ::core::mem::size_of::<*mut ELEMCON>() as u64 as i32,
    ) as *mut *mut ELEMCON;
    i = 1 as i32;
    while i <= (*mpl).m {
        let ref mut fresh0 = *((*mpl).row).offset(i as isize);
        *fresh0 = 0 as *mut ELEMCON;
        i += 1;
        i;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 103 as i32 {
            c = (*stmt).u.con;
            memb = (*(*c).array).head;
            while !memb.is_null() {
                i = (*(*memb).value.con).i;
                (1 as i32 <= i && i <= (*mpl).m
                    || {
                        glp_assert_(
                            b"1 <= i && i <= mpl->m\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            174 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                ((*((*mpl).row).offset(i as isize)).is_null()
                    || {
                        glp_assert_(
                            b"mpl->row[i] == NULL\0" as *const u8 as *const i8,
                            b"mpl/mpl4.c\0" as *const u8 as *const i8,
                            175 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                let ref mut fresh1 = *((*mpl).row).offset(i as isize);
                *fresh1 = (*memb).value.con;
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    i = 1 as i32;
    while i <= (*mpl).m {
        (!(*((*mpl).row).offset(i as isize)).is_null()
            || {
                glp_assert_(
                    b"mpl->row[i] != NULL\0" as *const u8 as *const i8,
                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                    180 as i32,
                );
                1 as i32 != 0
            }) as i32;
        i += 1;
        i;
    }
    (*mpl).col = glp_alloc(
        1 as i32 + (*mpl).n,
        ::core::mem::size_of::<*mut ELEMVAR>() as u64 as i32,
    ) as *mut *mut ELEMVAR;
    j = 1 as i32;
    while j <= (*mpl).n {
        let ref mut fresh2 = *((*mpl).col).offset(j as isize);
        *fresh2 = 0 as *mut ELEMVAR;
        j += 1;
        j;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 127 as i32 {
            v = (*stmt).u.var;
            memb = (*(*v).array).head;
            while !memb.is_null() {
                j = (*(*memb).value.var).j;
                if !(j == 0 as i32) {
                    (1 as i32 <= j && j <= (*mpl).n
                        || {
                            glp_assert_(
                                b"1 <= j && j <= mpl->n\0" as *const u8 as *const i8,
                                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                                190 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ((*((*mpl).col).offset(j as isize)).is_null()
                        || {
                            glp_assert_(
                                b"mpl->col[j] == NULL\0" as *const u8 as *const i8,
                                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                                191 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    let ref mut fresh3 = *((*mpl).col).offset(j as isize);
                    *fresh3 = (*memb).value.var;
                }
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    j = 1 as i32;
    while j <= (*mpl).n {
        (!(*((*mpl).col).offset(j as isize)).is_null()
            || {
                glp_assert_(
                    b"mpl->col[j] != NULL\0" as *const u8 as *const i8,
                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                    196 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_postsolve_model(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    ((*mpl).flag_p == 0
        || {
            glp_assert_(
                b"!mpl->flag_p\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                208 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*mpl).flag_p = 1 as i32;
    stmt = (*mpl).stmt;
    while !stmt.is_null() {
        _glp_mpl_execute_statement(mpl, stmt);
        stmt = (*stmt).next;
    }
    (*mpl).stmt = 0 as *mut STATEMENT;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_clean_model(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        _glp_mpl_clean_statement(mpl, stmt);
        stmt = (*stmt).next;
    }
    if _glp_dmp_in_use((*mpl).strings) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d string segment(s) were lost\0" as *const u8
                as *const i8 as *mut i8,
            _glp_dmp_in_use((*mpl).strings),
        );
    }
    if _glp_dmp_in_use((*mpl).symbols) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d symbol(s) were lost\0" as *const u8 as *const i8
                as *mut i8,
            _glp_dmp_in_use((*mpl).symbols),
        );
    }
    if _glp_dmp_in_use((*mpl).tuples) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d n-tuple component(s) were lost\0" as *const u8
                as *const i8 as *mut i8,
            _glp_dmp_in_use((*mpl).tuples),
        );
    }
    if _glp_dmp_in_use((*mpl).arrays) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d array(s) were lost\0" as *const u8 as *const i8
                as *mut i8,
            _glp_dmp_in_use((*mpl).arrays),
        );
    }
    if _glp_dmp_in_use((*mpl).members) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d array member(s) were lost\0" as *const u8
                as *const i8 as *mut i8,
            _glp_dmp_in_use((*mpl).members),
        );
    }
    if _glp_dmp_in_use((*mpl).elemvars) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d elemental variable(s) were lost\0" as *const u8
                as *const i8 as *mut i8,
            _glp_dmp_in_use((*mpl).elemvars),
        );
    }
    if _glp_dmp_in_use((*mpl).formulae) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d linear term(s) were lost\0" as *const u8
                as *const i8 as *mut i8,
            _glp_dmp_in_use((*mpl).formulae),
        );
    }
    if _glp_dmp_in_use((*mpl).elemcons) != 0 as i32 as u64 {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d elemental constraint(s) were lost\0" as *const u8
                as *const i8 as *mut i8,
            _glp_dmp_in_use((*mpl).elemcons),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_open_input(mut mpl: *mut MPL, mut file: *mut i8) {
    (*mpl).line = 0 as i32;
    (*mpl).c = '\n' as i32;
    (*mpl).token = 0 as i32;
    (*mpl).imlen = 0 as i32;
    *((*mpl).image).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).value = 0.0f64;
    (*mpl).b_token = 201 as i32;
    (*mpl).b_imlen = 0 as i32;
    *((*mpl).b_image).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).b_value = 0.0f64;
    (*mpl).f_dots = 0 as i32;
    (*mpl).f_scan = 0 as i32;
    (*mpl).f_token = 0 as i32;
    (*mpl).f_imlen = 0 as i32;
    *((*mpl).f_image).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).f_value = 0.0f64;
    memset((*mpl).context as *mut libc::c_void, ' ' as i32, 60 as i32 as u64);
    (*mpl).c_ptr = 0 as i32;
    (((*mpl).in_fp).is_null()
        || {
            glp_assert_(
                b"mpl->in_fp == NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                289 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*mpl).in_fp = _glp_open(file, b"r\0" as *const u8 as *const i8);
    if ((*mpl).in_fp).is_null() {
        _glp_mpl_error(
            mpl,
            b"unable to open %s - %s\0" as *const u8 as *const i8 as *mut i8,
            file,
            _glp_get_err_msg(),
        );
    }
    (*mpl).in_file = file;
    _glp_mpl_get_char(mpl);
    _glp_mpl_get_token(mpl);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_read_char(mut mpl: *mut MPL) -> i32 {
    let mut c: i32 = 0;
    (!((*mpl).in_fp).is_null()
        || {
            glp_assert_(
                b"mpl->in_fp != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                309 as i32,
            );
            1 as i32 != 0
        }) as i32;
    c = _glp_getc((*mpl).in_fp);
    if c < 0 as i32 {
        if _glp_ioerr((*mpl).in_fp) != 0 {
            _glp_mpl_error(
                mpl,
                b"read error on %s - %s\0" as *const u8 as *const i8 as *mut i8,
                (*mpl).in_file,
                _glp_get_err_msg(),
            );
        }
        c = -(1 as i32);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_close_input(mut mpl: *mut MPL) {
    (!((*mpl).in_fp).is_null()
        || {
            glp_assert_(
                b"mpl->in_fp != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                326 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_close((*mpl).in_fp);
    (*mpl).in_fp = 0 as *mut glp_file;
    (*mpl).in_file = 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_open_output(mut mpl: *mut MPL, mut file: *mut i8) {
    (((*mpl).out_fp).is_null()
        || {
            glp_assert_(
                b"mpl->out_fp == NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                340 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if file.is_null() {
        file = b"<stdout>\0" as *const u8 as *const i8 as *mut i8;
        (*mpl).out_fp = stdout as *mut libc::c_void as *mut glp_file;
    } else {
        (*mpl).out_fp = _glp_open(file, b"w\0" as *const u8 as *const i8);
        if ((*mpl).out_fp).is_null() {
            _glp_mpl_error(
                mpl,
                b"unable to create %s - %s\0" as *const u8 as *const i8 as *mut i8,
                file,
                _glp_get_err_msg(),
            );
        }
    }
    (*mpl).out_file = glp_alloc(
        1 as i32,
        (strlen(file)).wrapping_add(1 as i32 as u64) as i32,
    ) as *mut i8;
    strcpy((*mpl).out_file, file);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_write_char(mut mpl: *mut MPL, mut c: i32) {
    (!((*mpl).out_fp).is_null()
        || {
            glp_assert_(
                b"mpl->out_fp != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                361 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).out_fp == stdout as *mut libc::c_void as *mut glp_file {
        glp_printf(b"%c\0" as *const u8 as *const i8, c);
    } else {
        _glp_format((*mpl).out_fp, b"%c\0" as *const u8 as *const i8, c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_write_text(
    mut mpl: *mut MPL,
    mut fmt: *mut i8,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut c: *mut i8 = 0 as *mut i8;
    arg = args.clone();
    vsprintf(buf.as_mut_ptr(), fmt, arg.as_va_list());
    (strlen(buf.as_mut_ptr()) < ::core::mem::size_of::<[i8; 1024]>() as u64
        || {
            glp_assert_(
                b"strlen(buf) < sizeof(buf)\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                380 as i32,
            );
            1 as i32 != 0
        }) as i32;
    c = buf.as_mut_ptr();
    while *c as i32 != '\0' as i32 {
        _glp_mpl_write_char(mpl, *c as i32);
        c = c.offset(1);
        c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_flush_output(mut mpl: *mut MPL) {
    (!((*mpl).out_fp).is_null()
        || {
            glp_assert_(
                b"mpl->out_fp != NULL\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                392 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*mpl).out_fp != stdout as *mut libc::c_void as *mut glp_file {
        if _glp_ioerr((*mpl).out_fp) != 0 {
            _glp_mpl_error(
                mpl,
                b"write error on %s - %s\0" as *const u8 as *const i8 as *mut i8,
                (*mpl).out_file,
                _glp_get_err_msg(),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_error(
    mut mpl: *mut MPL,
    mut fmt: *mut i8,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    let mut msg: [i8; 4096] = [0; 4096];
    arg = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, arg.as_va_list());
    (strlen(msg.as_mut_ptr()) < ::core::mem::size_of::<[i8; 4096]>() as u64
        || {
            glp_assert_(
                b"strlen(msg) < sizeof(msg)\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                420 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*mpl).phase {
        1 | 2 => {
            glp_printf(
                b"%s:%d: %s\n\0" as *const u8 as *const i8,
                if ((*mpl).in_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const i8
                } else {
                    (*mpl).in_file
                },
                (*mpl).line,
                msg.as_mut_ptr(),
            );
            _glp_mpl_print_context(mpl);
        }
        3 => {
            glp_printf(
                b"%s:%d: %s\n\0" as *const u8 as *const i8,
                if ((*mpl).mod_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const i8
                } else {
                    (*mpl).mod_file
                },
                if ((*mpl).stmt).is_null() { 0 as i32 } else { (*(*mpl).stmt).line },
                msg.as_mut_ptr(),
            );
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        438 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    (*mpl).phase = 4 as i32;
    longjmp(((*mpl).jump).as_mut_ptr(), 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_warning(
    mut mpl: *mut MPL,
    mut fmt: *mut i8,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    let mut msg: [i8; 4096] = [0; 4096];
    arg = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, arg.as_va_list());
    (strlen(msg.as_mut_ptr()) < ::core::mem::size_of::<[i8; 4096]>() as u64
        || {
            glp_assert_(
                b"strlen(msg) < sizeof(msg)\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                456 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*mpl).phase {
        1 | 2 => {
            glp_printf(
                b"%s:%d: warning: %s\n\0" as *const u8 as *const i8,
                if ((*mpl).in_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const i8
                } else {
                    (*mpl).in_file
                },
                (*mpl).line,
                msg.as_mut_ptr(),
            );
        }
        3 => {
            glp_printf(
                b"%s:%d: warning: %s\n\0" as *const u8 as *const i8,
                if ((*mpl).mod_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const i8
                } else {
                    (*mpl).mod_file
                },
                if ((*mpl).stmt).is_null() { 0 as i32 } else { (*(*mpl).stmt).line },
                msg.as_mut_ptr(),
            );
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        473 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_initialize() -> *mut MPL {
    let mut mpl: *mut MPL = 0 as *mut MPL;
    mpl = glp_alloc(1 as i32, ::core::mem::size_of::<MPL>() as u64 as i32) as *mut MPL;
    (*mpl).line = 0 as i32;
    (*mpl).c = 0 as i32;
    (*mpl).token = 0 as i32;
    (*mpl).imlen = 0 as i32;
    (*mpl).image = glp_alloc(
        100 as i32 + 1 as i32,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    *((*mpl).image).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).value = 0.0f64;
    (*mpl).b_token = 0 as i32;
    (*mpl).b_imlen = 0 as i32;
    (*mpl).b_image = glp_alloc(
        100 as i32 + 1 as i32,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    *((*mpl).b_image).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).b_value = 0.0f64;
    (*mpl).f_dots = 0 as i32;
    (*mpl).f_scan = 0 as i32;
    (*mpl).f_token = 0 as i32;
    (*mpl).f_imlen = 0 as i32;
    (*mpl).f_image = glp_alloc(
        100 as i32 + 1 as i32,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    *((*mpl).f_image).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).f_value = 0.0f64;
    (*mpl).context = glp_alloc(60 as i32, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    memset((*mpl).context as *mut libc::c_void, ' ' as i32, 60 as i32 as u64);
    (*mpl).c_ptr = 0 as i32;
    (*mpl).flag_d = 0 as i32;
    (*mpl).pool = _glp_dmp_create_pool();
    (*mpl).tree = _glp_avl_create_tree(
        Some(
            _glp_avl_strcmp
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> i32,
        ),
        0 as *mut libc::c_void,
    );
    (*mpl).model = 0 as *mut STATEMENT;
    (*mpl).flag_x = 0 as i32;
    (*mpl).as_within = 0 as i32;
    (*mpl).as_in = 0 as i32;
    (*mpl).as_binary = 0 as i32;
    (*mpl).flag_s = 0 as i32;
    (*mpl).strings = _glp_dmp_create_pool();
    (*mpl).symbols = _glp_dmp_create_pool();
    (*mpl).tuples = _glp_dmp_create_pool();
    (*mpl).arrays = _glp_dmp_create_pool();
    (*mpl).members = _glp_dmp_create_pool();
    (*mpl).elemvars = _glp_dmp_create_pool();
    (*mpl).formulae = _glp_dmp_create_pool();
    (*mpl).elemcons = _glp_dmp_create_pool();
    (*mpl).a_list = 0 as *mut ARRAY;
    (*mpl).sym_buf = glp_alloc(
        255 as i32 + 1 as i32,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    *((*mpl).sym_buf).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).tup_buf = glp_alloc(
        255 as i32 + 1 as i32,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    *((*mpl).tup_buf).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*mpl).rand = _glp_rng_create_rand();
    (*mpl).flag_p = 0 as i32;
    (*mpl).stmt = 0 as *mut STATEMENT;
    (*mpl).dca = 0 as *mut TABDCA;
    (*mpl).m = 0 as i32;
    (*mpl).n = 0 as i32;
    (*mpl).row = 0 as *mut *mut ELEMCON;
    (*mpl).col = 0 as *mut *mut ELEMVAR;
    (*mpl).in_fp = 0 as *mut glp_file;
    (*mpl).in_file = 0 as *mut i8;
    (*mpl).out_fp = 0 as *mut glp_file;
    (*mpl).out_file = 0 as *mut i8;
    (*mpl).prt_fp = 0 as *mut glp_file;
    (*mpl).prt_file = 0 as *mut i8;
    if _setjmp(((*mpl).jump).as_mut_ptr()) != 0 {
        (mpl != mpl
            || {
                glp_assert_(
                    b"mpl != mpl\0" as *const u8 as *const i8,
                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                    564 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    (*mpl).phase = 0 as i32;
    (*mpl).mod_file = 0 as *mut i8;
    (*mpl).mpl_buf = glp_alloc(
        255 as i32 + 1 as i32,
        ::core::mem::size_of::<i8>() as u64 as i32,
    ) as *mut i8;
    *((*mpl).mpl_buf).offset(0 as i32 as isize) = '\0' as i32 as i8;
    return mpl;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_read_model(
    mut mpl: *mut MPL,
    mut file: *mut i8,
    mut skip_data: i32,
) -> i32 {
    let mut current_block: u64;
    if (*mpl).phase != 0 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 611 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_read_model: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if file.is_null() {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 613 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_read_model: no input filename specified\n\0" as *const u8 as *const i8,
        );
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        (*mpl).phase = 1 as i32;
        glp_printf(
            b"Reading model section from %s...\n\0" as *const u8 as *const i8,
            file,
        );
        _glp_mpl_open_input(mpl, file);
        _glp_mpl_model_section(mpl);
        if ((*mpl).model).is_null() {
            _glp_mpl_error(
                mpl,
                b"empty model section not allowed\0" as *const u8 as *const i8 as *mut i8,
            );
        }
        (*mpl).mod_file = glp_alloc(
            (strlen(file)).wrapping_add(1 as i32 as u64) as i32,
            ::core::mem::size_of::<i8>() as u64 as i32,
        ) as *mut i8;
        strcpy((*mpl).mod_file, (*mpl).in_file);
        _glp_mpl_alloc_content(mpl);
        if _glp_mpl_is_keyword(mpl, b"data\0" as *const u8 as *const i8 as *mut i8) != 0
        {
            if skip_data != 0 {
                _glp_mpl_warning(
                    mpl,
                    b"data section ignored\0" as *const u8 as *const i8 as *mut i8,
                );
                current_block = 17697489417475666352;
            } else {
                (*mpl).flag_d = 1 as i32;
                _glp_mpl_get_token(mpl);
                if (*mpl).token != 241 as i32 {
                    _glp_mpl_error(
                        mpl,
                        b"semicolon missing where expected\0" as *const u8 as *const i8
                            as *mut i8,
                    );
                }
                _glp_mpl_get_token(mpl);
                (*mpl).phase = 2 as i32;
                glp_printf(
                    b"Reading data section from %s...\n\0" as *const u8 as *const i8,
                    file,
                );
                _glp_mpl_data_section(mpl);
                current_block = 5948590327928692120;
            }
        } else {
            current_block = 5948590327928692120;
        }
        match current_block {
            5948590327928692120 => {
                _glp_mpl_end_statement(mpl);
            }
            _ => {}
        }
        glp_printf(
            b"%d line%s were read\n\0" as *const u8 as *const i8,
            (*mpl).line,
            if (*mpl).line == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
        );
        _glp_mpl_close_input(mpl);
    }
    return (*mpl).phase;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_read_data(
    mut mpl: *mut MPL,
    mut file: *mut i8,
) -> i32 {
    if !((*mpl).phase == 1 as i32 || (*mpl).phase == 2 as i32) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 686 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_read_data: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if file.is_null() {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 688 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_read_data: no input filename specified\n\0" as *const u8 as *const i8,
        );
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        (*mpl).phase = 2 as i32;
        glp_printf(
            b"Reading data section from %s...\n\0" as *const u8 as *const i8,
            file,
        );
        (*mpl).flag_d = 1 as i32;
        _glp_mpl_open_input(mpl, file);
        if _glp_mpl_is_literal(mpl, b"data\0" as *const u8 as *const i8 as *mut i8) != 0
        {
            _glp_mpl_get_token(mpl);
            if (*mpl).token != 241 as i32 {
                _glp_mpl_error(
                    mpl,
                    b"semicolon missing where expected\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
            _glp_mpl_get_token(mpl);
        }
        _glp_mpl_data_section(mpl);
        _glp_mpl_end_statement(mpl);
        glp_printf(
            b"%d line%s were read\n\0" as *const u8 as *const i8,
            (*mpl).line,
            if (*mpl).line == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
        );
        _glp_mpl_close_input(mpl);
    }
    return (*mpl).phase;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_generate(mut mpl: *mut MPL, mut file: *mut i8) -> i32 {
    if !((*mpl).phase == 1 as i32 || (*mpl).phase == 2 as i32) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 750 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_generate: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        (*mpl).phase = 3 as i32;
        _glp_mpl_open_output(mpl, file);
        _glp_mpl_generate_model(mpl);
        _glp_mpl_flush_output(mpl);
        _glp_mpl_build_problem(mpl);
        glp_printf(
            b"Model has been successfully generated\n\0" as *const u8 as *const i8,
        );
    }
    return (*mpl).phase;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_prob_name(mut mpl: *mut MPL) -> *mut i8 {
    let mut name: *mut i8 = (*mpl).mpl_buf;
    let mut file: *mut i8 = (*mpl).mod_file;
    let mut k: i32 = 0;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 791 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_prob_name: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    loop {
        if !(strchr(file, '/' as i32)).is_null() {
            file = (strchr(file, '/' as i32)).offset(1 as i32 as isize);
        } else if !(strchr(file, '\\' as i32)).is_null() {
            file = (strchr(file, '\\' as i32)).offset(1 as i32 as isize);
        } else {
            if (strchr(file, ':' as i32)).is_null() {
                break;
            }
            file = (strchr(file, ':' as i32)).offset(1 as i32 as isize);
        }
    }
    k = 0 as i32;
    while !(k == 255 as i32) {
        if !(*(*__ctype_b_loc()).offset(*file as u8 as i32 as isize) as i32
            & _ISalnum as i32 as libc::c_ushort as i32 != 0
            || *file as i32 == '_' as i32)
        {
            break;
        }
        let fresh4 = file;
        file = file.offset(1);
        *name.offset(k as isize) = *fresh4;
        k += 1;
        k;
    }
    if k == 0 as i32 {
        strcpy(name, b"Unknown\0" as *const u8 as *const i8);
    } else {
        *name.offset(k as isize) = '\0' as i32 as i8;
    }
    (strlen(name) <= 255 as i32 as u64
        || {
            glp_assert_(
                b"strlen(name) <= 255\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                811 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_num_rows(mut mpl: *mut MPL) -> i32 {
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 830 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_num_rows: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    return (*mpl).m;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_num_cols(mut mpl: *mut MPL) -> i32 {
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 849 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_num_cols: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    return (*mpl).n;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_row_name(
    mut mpl: *mut MPL,
    mut i: i32,
) -> *mut i8 {
    let mut name: *mut i8 = (*mpl).mpl_buf;
    let mut t: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 870 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_row_name: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= i && i <= (*mpl).m) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 872 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_row_name: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    strcpy(name, (*(**((*mpl).row).offset(i as isize)).con).name);
    len = strlen(name) as i32;
    (len <= 255 as i32
        || {
            glp_assert_(
                b"len <= 255\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                876 as i32,
            );
            1 as i32 != 0
        }) as i32;
    t = _glp_mpl_format_tuple(
        mpl,
        '[' as i32,
        (*(**((*mpl).row).offset(i as isize)).memb).tuple,
    );
    while *t != 0 {
        if len == 255 as i32 {
            break;
        }
        let fresh5 = t;
        t = t.offset(1);
        let fresh6 = len;
        len = len + 1;
        *name.offset(fresh6 as isize) = *fresh5;
    }
    *name.offset(len as isize) = '\0' as i32 as i8;
    if len == 255 as i32 {
        strcpy(name.offset(252 as i32 as isize), b"...\0" as *const u8 as *const i8);
    }
    (strlen(name) <= 255 as i32 as u64
        || {
            glp_assert_(
                b"strlen(name) <= 255\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                884 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_row_kind(mut mpl: *mut MPL, mut i: i32) -> i32 {
    let mut kind: i32 = 0;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 908 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_row_kind: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= i && i <= (*mpl).m) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 910 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_row_kind: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    match (*(**((*mpl).row).offset(i as isize)).con).type_0 {
        103 => {
            kind = 411 as i32;
        }
        116 => {
            kind = 412 as i32;
        }
        115 => {
            kind = 413 as i32;
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        920 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return kind;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_row_bnds(
    mut mpl: *mut MPL,
    mut i: i32,
    mut _lb: *mut libc::c_double,
    mut _ub: *mut libc::c_double,
) -> i32 {
    let mut con: *mut ELEMCON = 0 as *mut ELEMCON;
    let mut type_0: i32 = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 969 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_row_bnds: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= i && i <= (*mpl).m) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 971 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_row_bnds: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    con = *((*mpl).row).offset(i as isize);
    lb = if ((*(*con).con).lbnd).is_null() {
        -1.7976931348623157e+308f64
    } else {
        (*con).lbnd
    };
    ub = if ((*(*con).con).ubnd).is_null() {
        1.7976931348623157e+308f64
    } else {
        (*con).ubnd
    };
    if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64 {
        type_0 = 401 as i32;
        ub = 0.0f64;
        lb = ub;
    } else if ub == 1.7976931348623157e+308f64 {
        type_0 = 402 as i32;
        ub = 0.0f64;
    } else if lb == -1.7976931348623157e+308f64 {
        type_0 = 403 as i32;
        lb = 0.0f64;
    } else if (*(*con).con).lbnd != (*(*con).con).ubnd {
        type_0 = 404 as i32;
    } else {
        type_0 = 405 as i32;
    }
    if !_lb.is_null() {
        *_lb = lb;
    }
    if !_ub.is_null() {
        *_ub = ub;
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_mat_row(
    mut mpl: *mut MPL,
    mut i: i32,
    mut ndx: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut term: *mut FORMULA = 0 as *mut FORMULA;
    let mut len: i32 = 0 as i32;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1037 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_mat_row: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= i && i <= (*mpl).m) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1039 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_mat_row: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    term = (**((*mpl).row).offset(i as isize)).form;
    while !term.is_null() {
        (!((*term).var).is_null()
            || {
                glp_assert_(
                    b"term->var != NULL\0" as *const u8 as *const i8,
                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                    1042 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len += 1;
        len;
        (len <= (*mpl).n
            || {
                glp_assert_(
                    b"len <= mpl->n\0" as *const u8 as *const i8,
                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                    1044 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if !ndx.is_null() {
            *ndx.offset(len as isize) = (*(*term).var).j;
        }
        if !val.is_null() {
            *val.offset(len as isize) = (*term).coef;
        }
        term = (*term).next;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_row_c0(
    mut mpl: *mut MPL,
    mut i: i32,
) -> libc::c_double {
    let mut con: *mut ELEMCON = 0 as *mut ELEMCON;
    let mut c0: libc::c_double = 0.;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1071 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_row_c0: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= i && i <= (*mpl).m) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1073 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_row_c0: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    con = *((*mpl).row).offset(i as isize);
    if ((*(*con).con).lbnd).is_null() && ((*(*con).con).ubnd).is_null() {
        c0 = -(*con).lbnd;
    } else {
        c0 = 0.0f64;
    }
    return c0;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_col_name(
    mut mpl: *mut MPL,
    mut j: i32,
) -> *mut i8 {
    let mut name: *mut i8 = (*mpl).mpl_buf;
    let mut t: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1100 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_col_name: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= j && j <= (*mpl).n) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1102 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_col_name: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    strcpy(name, (*(**((*mpl).col).offset(j as isize)).var).name);
    len = strlen(name) as i32;
    (len <= 255 as i32
        || {
            glp_assert_(
                b"len <= 255\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                1106 as i32,
            );
            1 as i32 != 0
        }) as i32;
    t = _glp_mpl_format_tuple(
        mpl,
        '[' as i32,
        (*(**((*mpl).col).offset(j as isize)).memb).tuple,
    );
    while *t != 0 {
        if len == 255 as i32 {
            break;
        }
        let fresh7 = t;
        t = t.offset(1);
        let fresh8 = len;
        len = len + 1;
        *name.offset(fresh8 as isize) = *fresh7;
    }
    *name.offset(len as isize) = '\0' as i32 as i8;
    if len == 255 as i32 {
        strcpy(name.offset(252 as i32 as isize), b"...\0" as *const u8 as *const i8);
    }
    (strlen(name) <= 255 as i32 as u64
        || {
            glp_assert_(
                b"strlen(name) <= 255\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                1114 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_col_kind(mut mpl: *mut MPL, mut j: i32) -> i32 {
    let mut kind: i32 = 0;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1144 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_col_kind: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= j && j <= (*mpl).n) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1146 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_col_kind: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    match (*(**((*mpl).col).offset(j as isize)).var).type_0 {
        118 => {
            kind = 421 as i32;
        }
        113 => {
            kind = 422 as i32;
        }
        101 => {
            kind = 423 as i32;
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        1156 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return kind;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_get_col_bnds(
    mut mpl: *mut MPL,
    mut j: i32,
    mut _lb: *mut libc::c_double,
    mut _ub: *mut libc::c_double,
) -> i32 {
    let mut var: *mut ELEMVAR = 0 as *mut ELEMVAR;
    let mut type_0: i32 = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1205 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_get_col_bnds: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= j && j <= (*mpl).n) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1207 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_get_col_bnds: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    var = *((*mpl).col).offset(j as isize);
    lb = if ((*(*var).var).lbnd).is_null() {
        -1.7976931348623157e+308f64
    } else {
        (*var).lbnd
    };
    ub = if ((*(*var).var).ubnd).is_null() {
        1.7976931348623157e+308f64
    } else {
        (*var).ubnd
    };
    if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64 {
        type_0 = 401 as i32;
        ub = 0.0f64;
        lb = ub;
    } else if ub == 1.7976931348623157e+308f64 {
        type_0 = 402 as i32;
        ub = 0.0f64;
    } else if lb == -1.7976931348623157e+308f64 {
        type_0 = 403 as i32;
        lb = 0.0f64;
    } else if (*(*var).var).lbnd != (*(*var).var).ubnd {
        type_0 = 404 as i32;
    } else {
        type_0 = 405 as i32;
    }
    if !_lb.is_null() {
        *_lb = lb;
    }
    if !_ub.is_null() {
        *_ub = ub;
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_has_solve_stmt(mut mpl: *mut MPL) -> i32 {
    if (*mpl).phase != 3 as i32 {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1255 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpl_has_solve_stmt: invalid call sequence\n\0" as *const u8 as *const i8,
        );
    }
    return (*mpl).flag_s;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_put_row_soln(
    mut mpl: *mut MPL,
    mut i: i32,
    mut stat: i32,
    mut prim: libc::c_double,
    mut dual: libc::c_double,
) {
    ((*mpl).phase == 3 as i32
        || {
            glp_assert_(
                b"mpl->phase == 3\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                1263 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= i && i <= (*mpl).m
        || {
            glp_assert_(
                b"1 <= i && i <= mpl->m\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                1264 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (**((*mpl).row).offset(i as isize)).stat = stat;
    (**((*mpl).row).offset(i as isize)).prim = prim;
    (**((*mpl).row).offset(i as isize)).dual = dual;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_put_col_soln(
    mut mpl: *mut MPL,
    mut j: i32,
    mut stat: i32,
    mut prim: libc::c_double,
    mut dual: libc::c_double,
) {
    ((*mpl).phase == 3 as i32
        || {
            glp_assert_(
                b"mpl->phase == 3\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                1276 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= j && j <= (*mpl).n
        || {
            glp_assert_(
                b"1 <= j && j <= mpl->n\0" as *const u8 as *const i8,
                b"mpl/mpl4.c\0" as *const u8 as *const i8,
                1277 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (**((*mpl).col).offset(j as isize)).stat = stat;
    (**((*mpl).col).offset(j as isize)).prim = prim;
    (**((*mpl).col).offset(j as isize)).dual = dual;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_postsolve(mut mpl: *mut MPL) -> i32 {
    if !((*mpl).phase == 3 as i32 && (*mpl).flag_p == 0) {
        (glp_error_(b"mpl/mpl4.c\0" as *const u8 as *const i8, 1339 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpl_postsolve: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        _glp_mpl_postsolve_model(mpl);
        _glp_mpl_flush_output(mpl);
        glp_printf(
            b"Model has been successfully processed\n\0" as *const u8 as *const i8,
        );
    }
    return (*mpl).phase;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_terminate(mut mpl: *mut MPL) {
    if _setjmp(((*mpl).jump).as_mut_ptr()) != 0 {
        (mpl != mpl
            || {
                glp_assert_(
                    b"mpl != mpl\0" as *const u8 as *const i8,
                    b"mpl/mpl4.c\0" as *const u8 as *const i8,
                    1365 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    match (*mpl).phase {
        0 | 1 | 2 | 3 => {
            _glp_mpl_clean_model(mpl);
            (((*mpl).a_list).is_null()
                || {
                    glp_assert_(
                        b"mpl->a_list == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        1373 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (((*mpl).dca).is_null()
                || {
                    glp_assert_(
                        b"mpl->dca == NULL\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        1375 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        4 => {
            let mut a: *mut ARRAY = 0 as *mut ARRAY;
            a = (*mpl).a_list;
            while !a.is_null() {
                if !((*a).tree).is_null() {
                    _glp_avl_delete_tree((*a).tree);
                }
                a = (*a).next;
            }
            _glp_mpl_free_dca(mpl);
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const i8,
                        b"mpl/mpl4.c\0" as *const u8 as *const i8,
                        1390 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    glp_free((*mpl).image as *mut libc::c_void);
    glp_free((*mpl).b_image as *mut libc::c_void);
    glp_free((*mpl).f_image as *mut libc::c_void);
    glp_free((*mpl).context as *mut libc::c_void);
    _glp_dmp_delete_pool((*mpl).pool);
    _glp_avl_delete_tree((*mpl).tree);
    _glp_dmp_delete_pool((*mpl).strings);
    _glp_dmp_delete_pool((*mpl).symbols);
    _glp_dmp_delete_pool((*mpl).tuples);
    _glp_dmp_delete_pool((*mpl).arrays);
    _glp_dmp_delete_pool((*mpl).members);
    _glp_dmp_delete_pool((*mpl).elemvars);
    _glp_dmp_delete_pool((*mpl).formulae);
    _glp_dmp_delete_pool((*mpl).elemcons);
    glp_free((*mpl).sym_buf as *mut libc::c_void);
    glp_free((*mpl).tup_buf as *mut libc::c_void);
    _glp_rng_delete_rand((*mpl).rand);
    if !((*mpl).row).is_null() {
        glp_free((*mpl).row as *mut libc::c_void);
    }
    if !((*mpl).col).is_null() {
        glp_free((*mpl).col as *mut libc::c_void);
    }
    if !((*mpl).in_fp).is_null() {
        _glp_close((*mpl).in_fp);
    }
    if !((*mpl).out_fp).is_null()
        && (*mpl).out_fp != stdout as *mut libc::c_void as *mut glp_file
    {
        _glp_close((*mpl).out_fp);
    }
    if !((*mpl).out_file).is_null() {
        glp_free((*mpl).out_file as *mut libc::c_void);
    }
    if !((*mpl).prt_fp).is_null() {
        _glp_close((*mpl).prt_fp);
    }
    if !((*mpl).prt_file).is_null() {
        glp_free((*mpl).prt_file as *mut libc::c_void);
    }
    if !((*mpl).mod_file).is_null() {
        glp_free((*mpl).mod_file as *mut libc::c_void);
    }
    glp_free((*mpl).mpl_buf as *mut libc::c_void);
    glp_free(mpl as *mut libc::c_void);
}