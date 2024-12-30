#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut Nnull_string: *mut NODE;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    fn r_unref(tmp: *mut NODE);
    fn assoc_info(
        subs: *mut NODE,
        val: *mut NODE,
        p: *mut NODE,
        aname: *const libc::c_char,
    );
    fn make_aname(symbol: *const NODE) -> *const libc::c_char;
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn make_array() -> *mut NODE;
    fn assoc_copy(symbol: *mut NODE, newsymb: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    static mut nextfree: [block_header; 2];
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    static mut success_node: *mut NODE;
    static mut do_flags: do_flag_values;
    fn new_array_element() -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn getenv_long(name: *const libc::c_char) -> libc::c_long;
    static mut output_fp: *mut FILE;
    fn indent(indent_level: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum,
    _ISpunct,
    _IScntrl,
    _ISblank,
    _ISgraph,
    _ISprint,
    _ISspace,
    _ISxdigit,
    _ISdigit,
    _ISalpha,
    _ISlower,
    _ISupper,
}  // end of enum

pub type uint32_t = __uint32_t;
pub type __re_size_t = libc::c_uint;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
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
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub has_meta: bool,
    pub maybe_long: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_bool {
    awk_false = 0,
    awk_true,
}  // end of enum

pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_MPZ,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_valtype_t {
    AWK_UNDEFINED,
    AWK_NUMBER,
    AWK_STRING,
    AWK_REGEX,
    AWK_STRNUM,
    AWK_ARRAY,
    AWK_SCALAR,
    AWK_VALUE_COOKIE,
    AWK_BOOL,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const libc::c_char,
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nodevals {
    Node_illegal,
    Node_val,
    Node_regex,
    Node_dynregex,
    Node_var,
    Node_var_array,
    Node_var_new,
    Node_elem_new,
    Node_param_list,
    Node_func,
    Node_ext_func,
    Node_builtin_func,
    Node_array_ref,
    Node_array_tree,
    Node_array_leaf,
    Node_dump_array,
    Node_arrayfor,
    Node_frame,
    Node_instruction,
    Node_final,
}  // end of enum

pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_1,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: libc::c_long,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flagvals {
    REGEX,
    NUMCONSTSTR,
    XARRAY,
    HALFHAT,
    ARRAYMAXED,
    NULL_FIELD,
    NO_EXT_SET,
    MPZN,
    MPFN,
    WSTRCUR,
    INTIND,
    NUMINT,
    INTLSTR,
    BOOLVAL,
    USER_INPUT,
    NUMBER,
    NUMCUR,
    STRCUR,
    STRING,
    MALLOC,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nodep: C2RustUnnamed_3,
    pub val: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fltnum: libc::c_double,
    pub sp: *mut libc::c_char,
    pub slen: size_t,
    pub idx: libc::c_int,
    pub wsp: *mut wchar_t,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum commenttype {
    FOR_COMMENT,
    BLOCK_COMMENT,
    EOL_COMMENT,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_5,
    pub x: C2RustUnnamed_4,
    pub name: *mut libc::c_char,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: libc::c_ulong,
    pub reflags: reflagvals,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum reflagvals {
    CONSTANT = 1,
    FS_DFLT = 2,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option::<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum opcodeval {
    Op_final,
    Op_parens,
    Op_cond_exp,
    Op_K_function,
    Op_K_else,
    Op_K_if,
    Op_K_switch,
    Op_K_while,
    Op_K_arrayfor,
    Op_K_for,
    Op_K_do,
    Op_list,
    Op_symbol,
    Op_token,
    Op_stop,
    Op_atexit,
    Op_lint_plus,
    Op_lint,
    Op_breakpoint,
    Op_exec_count,
    Op_comment,
    Op_func,
    Op_after_endfile,
    Op_after_beginfile,
    Op_subscript_assign,
    Op_field_assign,
    Op_var_assign,
    Op_var_update,
    Op_arrayfor_final,
    Op_arrayfor_incr,
    Op_arrayfor_init,
    Op_newfile,
    Op_get_record,
    Op_jmp_false,
    Op_jmp_true,
    Op_jmp,
    Op_pop,
    Op_no_op,
    Op_field_spec_lhs,
    Op_subscript_lhs,
    Op_push_lhs,
    Op_push_param,
    Op_push_array,
    Op_push_re,
    Op_push_i,
    Op_push_arg_untyped,
    Op_push_arg,
    Op_push,
    Op_indirect_func_call,
    Op_func_call,
    Op_in_array,
    Op_ext_builtin,
    Op_sub_builtin,
    Op_builtin,
    Op_K_namespace,
    Op_K_nextfile,
    Op_K_getline,
    Op_K_getline_redir,
    Op_K_delete_loop,
    Op_K_delete,
    Op_K_return_from_eval,
    Op_K_return,
    Op_K_exit,
    Op_K_next,
    Op_K_printf,
    Op_K_print_rec,
    Op_K_print,
    Op_K_continue,
    Op_K_break,
    Op_K_default,
    Op_K_case,
    Op_rule,
    Op_nomatch,
    Op_match_rec,
    Op_match,
    Op_geq,
    Op_leq,
    Op_greater,
    Op_less,
    Op_notequal,
    Op_equal,
    Op_or_final,
    Op_or,
    Op_and_final,
    Op_and,
    Op_assign_concat,
    Op_assign_exp,
    Op_assign_minus,
    Op_assign_plus,
    Op_assign_mod,
    Op_assign_quotient,
    Op_assign_times,
    Op_store_field_exp,
    Op_store_field,
    Op_store_sub,
    Op_store_var,
    Op_assign,
    Op_not,
    Op_field_spec,
    Op_unary_plus,
    Op_unary_minus,
    Op_postdecrement,
    Op_postincrement,
    Op_predecrement,
    Op_preincrement,
    Op_sub_array,
    Op_subscript,
    Op_cond_pair,
    Op_line_range,
    Op_concat,
    Op_exp_i,
    Op_exp,
    Op_minus_i,
    Op_minus,
    Op_plus_i,
    Op_plus,
    Op_mod_i,
    Op_mod,
    Op_quotient_i,
    Op_quotient,
    Op_times_i,
    Op_times,
    Op_illegal,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub xl: libc::c_long,
    pub xn: *mut NODE,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
    pub efptr: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: libc::c_long,
    pub ldl: exec_count_t,
    pub name: *mut libc::c_char,
}
pub type exec_count_t = libc::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2RustUnnamed_9,
    pub hi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: libc::c_long,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const libc::c_char,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option::<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_item {
    pub freep: *mut block_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_header {
    pub freep: *mut block_item,
    pub size: size_t,
    pub name: *const libc::c_char,
    pub highwater: libc::c_long,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum block_id {
    BLOCK_NODE = 0,
    BLOCK_BUCKET,
    BLOCK_MAX,
}  // end of enum

pub const AVSTR: assoc_kind_t = 32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum assoc_kind_t {
    ANONE = 0x0,
    AINDEX = 0x1,
    AVALUE = 0x2,
    AINUM = 0x4,
    AISTR = 0x8,
    AVNUM = 0x10,
    AVSTR = 0x20,
    AASC = 0x40,
    ADESC = 0x80,
    ADELETE = 0x100,
}  // end of enum

pub const DO_MPFR: do_flag_values = 32768;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum do_flag_values {
    DO_MPFR,
    DO_DEBUG,
    DO_PROFILE,
    DO_SANDBOX,
    DO_TIDY_MEM,
    DO_DUMP_VARS,
    DO_PRETTY_PRINT,
    DO_INTERVALS,
    DO_NON_DEC_DATA,
    DO_INTL,
    DO_POSIX,
    DO_TRADITIONAL,
    DO_LINT_OLD,
    DO_LINT_ALL,
    DO_LINT_EXTENSIONS,
    DO_LINT_INVALID,
    DO_FLAG_NONE,
}  // end of enum

#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as libc::c_int as libc::c_long
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn ezalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2070 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = pma_calloc(1 as libc::c_int as size_t, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2074 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const libc::c_char,
    mut fmtidx: libc::c_int,
) -> *mut NODE {
    if (*s).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint {
        (*s).type_0 = Node_val;
        (*s)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*s).flags as libc::c_uint & !(NUMBER as libc::c_int) as libc::c_uint);
        return s;
    }
    if (*s).flags as libc::c_uint & STRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && ((*s).sub.val.idx == -(1 as libc::c_int) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.expect("non-null function pointer")(fmtstr, fmtidx, s);
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as libc::c_uint & NUMCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2052 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = pma_malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2056 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2088 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = pma_realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2092 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
static mut INT_CHAIN_MAX: size_t = 2 as libc::c_int as size_t;
#[no_mangle]
pub static mut int_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"int\0" as *const u8 as *const libc::c_char,
            init: Some(
                int_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: Some(
                is_integer
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            lookup: Some(
                int_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                int_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                int_clear as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                int_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                int_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                int_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                int_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
unsafe extern "C" fn int_array_init(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if symbol.is_null() {
        let mut newval: libc::c_long = 0;
        newval = getenv_long(b"INT_CHAIN_MAX\0" as *const u8 as *const libc::c_char);
        if newval > 0 as libc::c_int as libc::c_long {
            INT_CHAIN_MAX = newval as size_t;
        }
    } else {
        null_array(symbol);
    }
    return &mut success_node;
}
unsafe extern "C" fn standard_integer_string(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> bool {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    if *s as libc::c_int == '0' as i32 && len == 1 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    end = s.offset(len as isize);
    if *s as libc::c_int == '-' as i32
        && {
            s = s.offset(1);
            s == end
        }
    {
        return 0 as libc::c_int != 0;
    }
    if (*s as libc::c_int) < '1' as i32 || *s as libc::c_int > '9' as i32 {
        return 0 as libc::c_int != 0;
    }
    loop {
        s = s.offset(1);
        if !(s < end) {
            break;
        }
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_integer(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut l: libc::c_long = 0;
    let mut d: libc::c_double = 0.;
    if (*subs).flags as libc::c_uint & NUMINT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return &mut success_node;
    }
    if subs == Nnull_string
        || do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint != 0
    {
        return 0 as *mut *mut NODE;
    }
    if (*subs).flags as libc::c_uint & NUMCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        d = (*subs).sub.val.fltnum;
        if d <= 2147483647 as libc::c_int as libc::c_double
            && d >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
            && d == d as int32_t as libc::c_double
        {
            if (*subs).flags as libc::c_uint & STRCUR as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
                || standard_integer_string((*subs).sub.val.sp, (*subs).sub.val.slen)
                    as libc::c_int != 0
            {
                (*subs)
                    .flags = ::core::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >((*subs).flags as libc::c_uint | NUMINT as libc::c_int as libc::c_uint);
                return &mut success_node;
            }
        }
        return 0 as *mut *mut NODE;
    }
    let mut cp: *mut libc::c_char = (*subs).sub.val.sp;
    let mut cpend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    let mut len: size_t = (*subs).sub.val.slen;
    if len == 0 as libc::c_int as libc::c_ulong
        || *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0 && *cp as libc::c_int != '-' as i32
    {
        return 0 as *mut *mut NODE;
    }
    if len > 1 as libc::c_int as libc::c_ulong
        && (*cp as libc::c_int == '0' as i32
            || *cp as libc::c_int == '-' as i32
                && *cp.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32)
    {
        return 0 as *mut *mut NODE;
    }
    if len == 1 as libc::c_int as libc::c_ulong && *cp as libc::c_int != '-' as i32 {
        (*subs)
            .sub
            .val
            .fltnum = (*cp as libc::c_int - '0' as i32) as libc::c_long
            as libc::c_double;
        if (*subs).flags as libc::c_uint & USER_INPUT as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (*subs)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*subs).flags as libc::c_uint & !(STRING as libc::c_int) as libc::c_uint);
            (*subs)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*subs).flags as libc::c_uint | NUMBER as libc::c_int as libc::c_uint);
        }
        (*subs)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*subs).flags as libc::c_uint
                | (NUMCUR as libc::c_int | NUMINT as libc::c_int) as libc::c_uint,
        );
        return &mut success_node;
    }
    cpend = cp.offset(len as isize);
    save = *cpend;
    *cpend = '\0' as i32 as libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    l = strtol(cp, &mut ptr, 10 as libc::c_int);
    *cpend = save;
    if *__errno_location() != 0 as libc::c_int || ptr != cpend {
        return 0 as *mut *mut NODE;
    }
    (*subs).sub.val.fltnum = l as libc::c_double;
    if (*subs).flags as libc::c_uint & USER_INPUT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        (*subs)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*subs).flags as libc::c_uint & !(STRING as libc::c_int) as libc::c_uint);
        (*subs)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*subs).flags as libc::c_uint | NUMBER as libc::c_int as libc::c_uint);
    }
    (*subs)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*subs).flags as libc::c_uint | NUMCUR as libc::c_int as libc::c_uint);
    if l <= 2147483647 as libc::c_int as libc::c_long
        && l >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
    {
        (*subs)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*subs).flags as libc::c_uint | NUMINT as libc::c_int as libc::c_uint);
        return &mut success_node;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn int_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: uint32_t = 0;
    let mut k: libc::c_long = 0;
    let mut size: libc::c_ulong = 0;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    if (is_integer(symbol, subs)).is_null() {
        xn = (*symbol).sub.nodep.rn;
        if xn.is_null() {
            (*symbol).sub.nodep.rn = make_array();
            xn = (*symbol).sub.nodep.rn;
            (*xn).sub.nodep.name = (*symbol).sub.nodep.name;
            (*xn)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*xn).flags as libc::c_uint | XARRAY as libc::c_int as libc::c_uint);
        } else {
            lhs = ((*(*xn).sub.nodep.l.lp).exists)
                .expect("non-null function pointer")(xn, subs);
            if !lhs.is_null() {
                return lhs;
            }
        }
        (*symbol).sub.nodep.reflags += 1;
        (*symbol).sub.nodep.reflags;
        return ((*(*xn).sub.nodep.l.lp).lookup)
            .expect("non-null function pointer")(xn, subs);
    }
    k = (*subs).sub.val.fltnum as libc::c_long;
    if ((*symbol).sub.nodep.r.bv).is_null() {
        grow_int_table(symbol);
    }
    hash1 = int_hash(k as uint32_t, (*symbol).sub.nodep.cnt as uint32_t);
    lhs = int_find(symbol, k, hash1);
    if !lhs.is_null() {
        return lhs;
    }
    (*symbol).sub.nodep.reflags += 1;
    (*symbol).sub.nodep.reflags;
    size = (*symbol).sub.nodep.reflags as libc::c_ulong;
    xn = (*symbol).sub.nodep.rn;
    if !xn.is_null() {
        size = size.wrapping_sub((*xn).sub.nodep.reflags as libc::c_ulong);
    }
    if (*symbol).flags as libc::c_uint & ARRAYMAXED as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
        && size.wrapping_div((*symbol).sub.nodep.cnt) > INT_CHAIN_MAX
    {
        grow_int_table(symbol);
        hash1 = int_hash(k as uint32_t, (*symbol).sub.nodep.cnt as uint32_t);
    }
    return int_insert(symbol, k, hash1);
}
unsafe extern "C" fn int_exists(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut k: libc::c_long = 0;
    let mut hash1: uint32_t = 0;
    if (is_integer(symbol, subs)).is_null() {
        let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
        if xn.is_null() {
            return 0 as *mut *mut NODE;
        }
        return ((*(*xn).sub.nodep.l.lp).exists)
            .expect("non-null function pointer")(xn, subs);
    }
    if ((*symbol).sub.nodep.r.bv).is_null() {
        return 0 as *mut *mut NODE;
    }
    k = (*subs).sub.val.fltnum as libc::c_long;
    hash1 = int_hash(k as uint32_t, (*symbol).sub.nodep.cnt as uint32_t);
    return int_find(symbol, k, hash1);
}
unsafe extern "C" fn int_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: libc::c_ulong = 0;
    let mut j: libc::c_int = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut next: *mut BUCKET = 0 as *mut BUCKET;
    let mut r: *mut NODE = 0 as *mut NODE;
    if !((*symbol).sub.nodep.rn).is_null() {
        let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
        ((*(*xn).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(xn, 0 as *mut exp_node);
        let ref mut fresh0 = (*(xn as *mut block_item)).freep;
        *fresh0 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = xn as *mut block_item;
        (*symbol).sub.nodep.rn = 0 as *mut exp_node;
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            next = (*b).hi.next;
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < (*b).hi.cnt {
                r = (*b).hi.val[j as usize];
                if (*r).type_0 as libc::c_uint
                    == Node_var_array as libc::c_int as libc::c_uint
                {
                    ((*(*r).sub.nodep.l.lp).clear)
                        .expect("non-null function pointer")(r, 0 as *mut exp_node);
                    pma_free((*r).sub.nodep.name as *mut libc::c_void);
                    let ref mut fresh1 = (*(r as *mut block_item)).freep;
                    *fresh1 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
                    nextfree[BLOCK_NODE as libc::c_int as usize]
                        .freep = r as *mut block_item;
                } else {
                    unref(r);
                }
                j += 1;
                j;
            }
            let ref mut fresh2 = (*(b as *mut block_item)).freep;
            *fresh2 = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep;
            nextfree[BLOCK_BUCKET as libc::c_int as usize].freep = b as *mut block_item;
            b = next;
        }
        let ref mut fresh3 = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        *fresh3 = 0 as *mut BUCKET;
        i = i.wrapping_add(1);
        i;
    }
    if !((*symbol).sub.nodep.r.bv).is_null() {
        pma_free((*symbol).sub.nodep.r.bv as *mut libc::c_void);
    }
    ((*(*symbol).sub.nodep.l.lp).init)
        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn int_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut current_block: u64;
    let mut hash1: uint32_t = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut prev: *mut BUCKET = 0 as *mut BUCKET;
    let mut k: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint
        || ((*symbol).sub.nodep.r.bv).is_null()
    {
        return 0 as *mut *mut NODE;
    }
    if (is_integer(symbol, subs)).is_null() {
        if xn.is_null()
            || (((*(*xn).sub.nodep.l.lp).remove)
                .expect("non-null function pointer")(xn, subs))
                .is_null()
        {
            return 0 as *mut *mut NODE;
        }
        if (*xn).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            let ref mut fresh4 = (*(xn as *mut block_item)).freep;
            *fresh4 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = xn as *mut block_item;
            (*symbol).sub.nodep.rn = 0 as *mut exp_node;
        }
        (*symbol).sub.nodep.reflags -= 1;
        (*symbol).sub.nodep.reflags;
        return &mut success_node;
    }
    k = (*subs).sub.val.fltnum as libc::c_long;
    hash1 = int_hash(k as uint32_t, (*symbol).sub.nodep.cnt as uint32_t);
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    's_63: loop {
        if b.is_null() {
            current_block = 5601891728916014340;
            break;
        }
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*b).hi.cnt {
            if k != (*b).hi.li[i as usize] {
                i += 1;
                i;
            } else {
                if i == 0 as libc::c_int
                    && (*b).hi.cnt == 2 as libc::c_int as libc::c_ulong
                {
                    (*b)
                        .hi
                        .li[0 as libc::c_int
                        as usize] = (*b).hi.li[1 as libc::c_int as usize];
                    (*b)
                        .hi
                        .val[0 as libc::c_int
                        as usize] = (*b).hi.val[1 as libc::c_int as usize];
                }
                current_block = 12221781404543286498;
                break 's_63;
            }
        }
        prev = b;
        b = (*b).hi.next;
    }
    match current_block {
        5601891728916014340 => {
            if b.is_null() {
                return 0 as *mut *mut NODE;
            }
        }
        _ => {}
    }
    (*b).hi.cnt = ((*b).hi.cnt).wrapping_sub(1);
    (*b).hi.cnt;
    if (*b).hi.cnt == 0 as libc::c_int as libc::c_ulong {
        if !prev.is_null() {
            (*prev).hi.next = (*b).hi.next;
        } else {
            let ref mut fresh5 = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
            *fresh5 = (*b).hi.next;
        }
        let ref mut fresh6 = (*(b as *mut block_item)).freep;
        *fresh6 = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep;
        nextfree[BLOCK_BUCKET as libc::c_int as usize].freep = b as *mut block_item;
    } else if b != *((*symbol).sub.nodep.r.bv).offset(hash1 as isize) {
        let mut head: *mut BUCKET = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
        (*head).hi.cnt = ((*head).hi.cnt).wrapping_sub(1);
        i = (*head).hi.cnt as libc::c_int;
        (*b).hi.li[1 as libc::c_int as usize] = (*head).hi.li[i as usize];
        (*b).hi.val[1 as libc::c_int as usize] = (*head).hi.val[i as usize];
        (*b).hi.cnt = ((*b).hi.cnt).wrapping_add(1);
        (*b).hi.cnt;
        if i == 0 as libc::c_int {
            let ref mut fresh7 = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
            *fresh7 = (*head).hi.next;
            let ref mut fresh8 = (*(head as *mut block_item)).freep;
            *fresh8 = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep;
            nextfree[BLOCK_BUCKET as libc::c_int as usize]
                .freep = head as *mut block_item;
        }
    }
    (*symbol).sub.nodep.reflags -= 1;
    (*symbol).sub.nodep.reflags;
    if xn.is_null()
        && (*symbol).sub.nodep.reflags as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        pma_free((*symbol).sub.nodep.r.bv as *mut libc::c_void);
        ((*(*symbol).sub.nodep.l.lp).init)
            .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
    } else if !xn.is_null()
        && (*symbol).sub.nodep.reflags as libc::c_uint
            == (*xn).sub.nodep.reflags as libc::c_uint
    {
        (*xn)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*xn).flags as libc::c_uint & !(XARRAY as libc::c_int) as libc::c_uint);
        (*xn).sub.nodep.x.extra = (*symbol).sub.nodep.x.extra;
        pma_free((*symbol).sub.nodep.r.bv as *mut libc::c_void);
        *symbol = *xn;
        let ref mut fresh9 = (*(xn as *mut block_item)).freep;
        *fresh9 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = xn as *mut block_item;
    }
    return &mut success_node;
}
unsafe extern "C" fn int_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut *mut NODE {
    let mut old: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut new: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut pnew: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut chain: *mut BUCKET = 0 as *mut BUCKET;
    let mut newchain: *mut BUCKET = 0 as *mut BUCKET;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_ulong = 0;
    let mut cursize: libc::c_ulong = 0;
    cursize = (*symbol).sub.nodep.cnt;
    new = ezalloc_real(
        cursize.wrapping_mul(::core::mem::size_of::<*mut BUCKET>() as libc::c_ulong),
        b"int_copy\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"int_array.c\0" as *const u8 as *const libc::c_char,
        461 as libc::c_int,
    ) as *mut *mut BUCKET;
    old = (*symbol).sub.nodep.r.bv;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < cursize {
        chain = *old.offset(i as isize);
        pnew = &mut *new.offset(i as isize) as *mut *mut BUCKET;
        while !chain.is_null() {
            newchain = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep
                as *mut BUCKET;
            if !newchain.is_null() {
                nextfree[BLOCK_BUCKET as libc::c_int as usize]
                    .freep = (*(newchain as *mut block_item)).freep;
            } else {
                newchain = more_blocks(BLOCK_BUCKET as libc::c_int) as *mut BUCKET;
            };
            (*newchain).hi.cnt = (*chain).hi.cnt;
            (*newchain).hi.next = 0 as *mut bucket_item;
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < (*chain).hi.cnt {
                let mut oldval: *mut NODE = 0 as *mut NODE;
                (*newchain).hi.li[j as usize] = (*chain).hi.li[j as usize];
                oldval = (*chain).hi.val[j as usize];
                if (*oldval).type_0 as libc::c_uint
                    == Node_val as libc::c_int as libc::c_uint
                {
                    (*newchain).hi.val[j as usize] = dupnode(oldval);
                } else {
                    let mut r: *mut NODE = 0 as *mut NODE;
                    r = make_array();
                    (*r)
                        .sub
                        .nodep
                        .name = estrdup(
                        (*oldval).sub.nodep.name,
                        strlen((*oldval).sub.nodep.name),
                    );
                    (*r).sub.nodep.x.extra = newsymb;
                    (*newchain).hi.val[j as usize] = assoc_copy(oldval, r);
                }
                j += 1;
                j;
            }
            *pnew = newchain;
            (*newchain).hi.next = 0 as *mut bucket_item;
            pnew = &mut (*newchain).hi.next;
            chain = (*chain).hi.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !((*symbol).sub.nodep.rn).is_null() {
        let mut xn: *mut NODE = 0 as *mut NODE;
        let mut n: *mut NODE = 0 as *mut NODE;
        xn = (*symbol).sub.nodep.rn;
        n = make_array();
        (*n).sub.nodep.name = (*newsymb).sub.nodep.name;
        ((*(*xn).sub.nodep.l.lp).copy).expect("non-null function pointer")(xn, n);
        (*newsymb).sub.nodep.rn = n;
    } else {
        (*newsymb).sub.nodep.rn = 0 as *mut exp_node;
    }
    (*newsymb).sub.nodep.reflags = (*symbol).sub.nodep.reflags;
    (*newsymb).sub.nodep.r.bv = new;
    (*newsymb).sub.nodep.cnt = cursize;
    (*newsymb).flags = (*symbol).flags;
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn int_list(
    mut symbol: *mut NODE,
    mut t: *mut NODE,
) -> *mut *mut NODE {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut num_elems: libc::c_ulong = 0;
    let mut list_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut j: libc::c_int = 0;
    let mut elem_size: libc::c_int = 1 as libc::c_int;
    let mut num: libc::c_long = 0;
    static mut buf: [libc::c_char; 100] = [0; 100];
    let mut assoc_kind: assoc_kind_t = ANONE;
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut *mut NODE;
    }
    assoc_kind = (*t).flags as assoc_kind_t;
    num_elems = (*symbol).sub.nodep.reflags as libc::c_ulong;
    if assoc_kind as libc::c_uint
        & (AINDEX as libc::c_int | AVALUE as libc::c_int | ADELETE as libc::c_int)
            as libc::c_uint
        == (AINDEX as libc::c_int | ADELETE as libc::c_int) as libc::c_uint
    {
        num_elems = 1 as libc::c_int as libc::c_ulong;
    }
    if assoc_kind as libc::c_uint
        & (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
        == (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
    {
        elem_size = 2 as libc::c_int;
    }
    list_size = (elem_size as libc::c_ulong).wrapping_mul(num_elems);
    if !((*symbol).sub.nodep.rn).is_null() {
        xn = (*symbol).sub.nodep.rn;
        list = ((*(*xn).sub.nodep.l.lp).list).expect("non-null function pointer")(xn, t);
        if num_elems == 1 as libc::c_int as libc::c_ulong
            || num_elems == (*xn).sub.nodep.reflags as libc::c_ulong
        {
            return list;
        }
        list = erealloc_real(
            list as *mut libc::c_void,
            list_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"int_list\0" as *const u8 as *const libc::c_char,
            b"list\0" as *const u8 as *const libc::c_char,
            b"int_array.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
        ) as *mut *mut NODE;
        k = (elem_size as libc::c_uint)
            .wrapping_mul((*xn).sub.nodep.reflags as libc::c_uint) as libc::c_ulong;
    } else {
        list = emalloc_real(
            list_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"int_list\0" as *const u8 as *const libc::c_char,
            b"list\0" as *const u8 as *const libc::c_char,
            b"int_array.c\0" as *const u8 as *const libc::c_char,
            553 as libc::c_int,
        ) as *mut *mut NODE;
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < (*b).hi.cnt {
                num = (*b).hi.li[j as usize];
                if assoc_kind as libc::c_uint & AISTR as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    sprintf(
                        buf.as_mut_ptr(),
                        b"%ld\0" as *const u8 as *const libc::c_char,
                        num,
                    );
                    subs = make_str_node(
                        buf.as_mut_ptr(),
                        strlen(buf.as_mut_ptr()),
                        0 as libc::c_int,
                    );
                    (*subs).sub.val.fltnum = num as libc::c_double;
                    (*subs)
                        .flags = ::core::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*subs).flags as libc::c_uint
                            | (NUMCUR as libc::c_int | NUMINT as libc::c_int)
                                as libc::c_uint,
                    );
                } else {
                    subs = make_number
                        .expect("non-null function pointer")(num as libc::c_double);
                    (*subs)
                        .flags = ::core::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*subs).flags as libc::c_uint
                            | (INTIND as libc::c_int | NUMINT as libc::c_int)
                                as libc::c_uint,
                    );
                }
                let fresh10 = k;
                k = k.wrapping_add(1);
                let ref mut fresh11 = *list.offset(fresh10 as isize);
                *fresh11 = subs;
                if assoc_kind as libc::c_uint & AVALUE as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    r = (*b).hi.val[j as usize];
                    if (*r).type_0 as libc::c_uint
                        == Node_val as libc::c_int as libc::c_uint
                    {
                        if assoc_kind as libc::c_uint
                            & AVNUM as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                        {
                            force_number(r);
                        } else if assoc_kind as libc::c_uint
                            & AVSTR as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                        {
                            r = force_string_fmt(r, CONVFMT, CONVFMTidx);
                        }
                    }
                    let fresh12 = k;
                    k = k.wrapping_add(1);
                    let ref mut fresh13 = *list.offset(fresh12 as isize);
                    *fresh13 = r;
                }
                if k >= list_size {
                    return list;
                }
                j += 1;
                j;
            }
            b = (*b).hi.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn int_kilobytes(mut symbol: *mut NODE) -> libc::c_double {
    let mut i: libc::c_ulong = 0;
    let mut bucket_cnt: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut kb: libc::c_double = 0.;
    extern "C" {
        #[link_name = "str_kilobytes"]
        fn str_kilobytes_0(symbol_0: *mut NODE) -> libc::c_double;
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            bucket_cnt = bucket_cnt.wrapping_add(1);
            bucket_cnt;
            b = (*b).hi.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    kb = (bucket_cnt as libc::c_double
        * ::core::mem::size_of::<BUCKET>() as libc::c_ulong as libc::c_double
        + (*symbol).sub.nodep.cnt as libc::c_double
            * ::core::mem::size_of::<*mut BUCKET>() as libc::c_ulong as libc::c_double)
        / 1024.0f64;
    if !((*symbol).sub.nodep.rn).is_null() {
        kb += str_kilobytes_0((*symbol).sub.nodep.rn);
    }
    return kb;
}
unsafe extern "C" fn int_dump(
    mut symbol: *mut NODE,
    mut ndump: *mut NODE,
) -> *mut *mut NODE {
    let mut indent_level: libc::c_int = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut str_size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut int_size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_ulong = 0;
    let mut j: size_t = 0;
    let mut bucket_cnt: size_t = 0;
    static mut hash_dist: [size_t; 32] = [0; 32];
    indent_level = (*ndump).sub.nodep.x.xl as libc::c_int;
    if !((*symbol).sub.nodep.rn).is_null() {
        xn = (*symbol).sub.nodep.rn;
        str_size = (*xn).sub.nodep.reflags as libc::c_ulong;
    }
    int_size = ((*symbol).sub.nodep.reflags as libc::c_ulong).wrapping_sub(str_size);
    if (*symbol).flags as libc::c_uint & XARRAY as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        fprintf(
            output_fp,
            b"%s `%s'\n\0" as *const u8 as *const libc::c_char,
            if ((*symbol).sub.nodep.x.extra).is_null() {
                b"array\0" as *const u8 as *const libc::c_char
            } else {
                b"sub-array\0" as *const u8 as *const libc::c_char
            },
            array_vname(symbol),
        );
    }
    indent_level += 1;
    indent_level;
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_func: int_array_func\n\0" as *const u8 as *const libc::c_char,
    );
    if (*symbol).flags as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        indent(indent_level);
        fprintf(
            output_fp,
            b"flags: %s\n\0" as *const u8 as *const libc::c_char,
            flags2str((*symbol).flags as libc::c_int),
        );
    }
    indent(indent_level);
    fprintf(
        output_fp,
        b"INT_CHAIN_MAX: %lu\n\0" as *const u8 as *const libc::c_char,
        INT_CHAIN_MAX,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_size: %lu (int)\n\0" as *const u8 as *const libc::c_char,
        (*symbol).sub.nodep.cnt,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"table_size: %lu (total), %lu (int), %lu (str)\n\0" as *const u8
            as *const libc::c_char,
        (*symbol).sub.nodep.reflags as libc::c_ulong,
        int_size,
        str_size,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"Avg # of items per chain (int): %.2g\n\0" as *const u8 as *const libc::c_char,
        int_size as libc::c_double / (*symbol).sub.nodep.cnt as libc::c_double,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"memory: %.2g kB (total)\n\0" as *const u8 as *const libc::c_char,
        int_kilobytes(symbol),
    );
    memset(
        hash_dist.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ((31 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        bucket_cnt = 0 as libc::c_int as size_t;
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            bucket_cnt = (bucket_cnt as libc::c_ulong).wrapping_add((*b).hi.cnt)
                as size_t as size_t;
            b = (*b).hi.next;
        }
        if bucket_cnt >= 31 as libc::c_int as libc::c_ulong {
            bucket_cnt = 31 as libc::c_int as size_t;
        }
        hash_dist[bucket_cnt
            as usize] = (hash_dist[bucket_cnt as usize]).wrapping_add(1);
        hash_dist[bucket_cnt as usize];
        i = i.wrapping_add(1);
        i;
    }
    indent(indent_level);
    fprintf(output_fp, b"Hash distribution:\n\0" as *const u8 as *const libc::c_char);
    indent_level += 1;
    indent_level;
    j = 0 as libc::c_int as size_t;
    while j <= 31 as libc::c_int as libc::c_ulong {
        if hash_dist[j as usize] > 0 as libc::c_int as libc::c_ulong {
            indent(indent_level);
            if j == 31 as libc::c_int as libc::c_ulong {
                fprintf(
                    output_fp,
                    b"[>=%lu]:%lu\n\0" as *const u8 as *const libc::c_char,
                    31 as libc::c_int as libc::c_ulong,
                    hash_dist[j as usize],
                );
            } else {
                fprintf(
                    output_fp,
                    b"[%lu]:%lu\n\0" as *const u8 as *const libc::c_char,
                    j,
                    hash_dist[j as usize],
                );
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    indent_level -= 1;
    indent_level;
    if (*ndump).sub.nodep.l.ll >= 0 as libc::c_int as libc::c_long {
        let mut subs: *mut NODE = 0 as *mut NODE;
        let mut aname: *const libc::c_char = 0 as *const libc::c_char;
        fprintf(output_fp, b"\n\0" as *const u8 as *const libc::c_char);
        aname = make_aname(symbol);
        subs = make_number
            .expect("non-null function pointer")(0 as libc::c_int as libc::c_double);
        (*subs)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*subs).flags as libc::c_uint
                | (INTIND as libc::c_int | NUMINT as libc::c_int) as libc::c_uint,
        );
        i = 0 as libc::c_int as libc::c_ulong;
        while i < (*symbol).sub.nodep.cnt {
            b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
            while !b.is_null() {
                j = 0 as libc::c_int as size_t;
                while j < (*b).hi.cnt {
                    (*subs).sub.val.fltnum = (*b).hi.li[j as usize] as libc::c_double;
                    assoc_info(subs, (*b).hi.val[j as usize], ndump, aname);
                    j = j.wrapping_add(1);
                    j;
                }
                b = (*b).hi.next;
            }
            i = i.wrapping_add(1);
            i;
        }
        unref(subs);
    }
    if !xn.is_null() {
        fprintf(output_fp, b"\n\0" as *const u8 as *const libc::c_char);
        ((*(*xn).sub.nodep.l.lp).dump).expect("non-null function pointer")(xn, ndump);
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn int_hash(mut k: uint32_t, mut hsize: uint32_t) -> uint32_t {
    k ^= k << 3 as libc::c_int;
    k = (k as libc::c_uint).wrapping_add(k >> 5 as libc::c_int) as uint32_t as uint32_t;
    k ^= k << 4 as libc::c_int;
    k = (k as libc::c_uint).wrapping_add(k >> 17 as libc::c_int) as uint32_t as uint32_t;
    k ^= k << 25 as libc::c_int;
    k = (k as libc::c_uint).wrapping_add(k >> 6 as libc::c_int) as uint32_t as uint32_t;
    if k >= hsize {
        k = (k as libc::c_uint).wrapping_rem(hsize) as uint32_t as uint32_t;
    }
    return k;
}
#[inline]
unsafe extern "C" fn int_find(
    mut symbol: *mut NODE,
    mut k: libc::c_long,
    mut hash1: uint32_t,
) -> *mut *mut NODE {
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut i: libc::c_int = 0;
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    while !b.is_null() {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*b).hi.cnt {
            if (*b).hi.li[i as usize] == k {
                return ((*b).hi.val).as_mut_ptr().offset(i as isize);
            }
            i += 1;
            i;
        }
        b = (*b).hi.next;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn int_insert(
    mut symbol: *mut NODE,
    mut k: libc::c_long,
    mut hash1: uint32_t,
) -> *mut *mut NODE {
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut i: libc::c_int = 0;
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    if b.is_null()
        || {
            i = (*b).hi.cnt as libc::c_int;
            i == 2 as libc::c_int
        }
    {
        b = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep as *mut BUCKET;
        if !b.is_null() {
            nextfree[BLOCK_BUCKET as libc::c_int as usize]
                .freep = (*(b as *mut block_item)).freep;
        } else {
            b = more_blocks(BLOCK_BUCKET as libc::c_int) as *mut BUCKET;
        };
        (*b).hi.cnt = 0 as libc::c_int as size_t;
        (*b).hi.next = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
        let ref mut fresh14 = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
        *fresh14 = b;
        i = 0 as libc::c_int;
    }
    (*b).hi.li[i as usize] = k;
    (*b).hi.val[i as usize] = new_array_element();
    (*b).hi.cnt = ((*b).hi.cnt).wrapping_add(1);
    (*b).hi.cnt;
    return &mut *((*b).hi.val).as_mut_ptr().offset(i as isize) as *mut *mut exp_node;
}
unsafe extern "C" fn grow_int_table(mut symbol: *mut NODE) {
    let mut old: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut new: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut chain: *mut BUCKET = 0 as *mut BUCKET;
    let mut next: *mut BUCKET = 0 as *mut BUCKET;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut oldsize: libc::c_ulong = 0;
    let mut newsize: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0;
    static mut sizes: [libc::c_ulong; 21] = [
        13 as libc::c_int as libc::c_ulong,
        127 as libc::c_int as libc::c_ulong,
        1021 as libc::c_int as libc::c_ulong,
        8191 as libc::c_int as libc::c_ulong,
        16381 as libc::c_int as libc::c_ulong,
        32749 as libc::c_int as libc::c_ulong,
        65497 as libc::c_int as libc::c_ulong,
        131101 as libc::c_int as libc::c_ulong,
        262147 as libc::c_int as libc::c_ulong,
        524309 as libc::c_int as libc::c_ulong,
        1048583 as libc::c_int as libc::c_ulong,
        2097169 as libc::c_int as libc::c_ulong,
        4194319 as libc::c_int as libc::c_ulong,
        8388617 as libc::c_int as libc::c_ulong,
        16777259 as libc::c_int as libc::c_ulong,
        33554467 as libc::c_int as libc::c_ulong,
        67108879 as libc::c_int as libc::c_ulong,
        134217757 as libc::c_int as libc::c_ulong,
        268435459 as libc::c_int as libc::c_ulong,
        536870923 as libc::c_int as libc::c_ulong,
        1073741827 as libc::c_int as libc::c_ulong,
    ];
    oldsize = (*symbol).sub.nodep.cnt;
    newsize = oldsize;
    i = 0 as libc::c_int;
    j = (::core::mem::size_of::<[libc::c_ulong; 21]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        as libc::c_int;
    while i < j {
        if oldsize < sizes[i as usize] {
            newsize = sizes[i as usize];
            break;
        } else {
            i += 1;
            i;
        }
    }
    if newsize == oldsize {
        (*symbol)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*symbol).flags as libc::c_uint | ARRAYMAXED as libc::c_int as libc::c_uint);
        return;
    }
    new = ezalloc_real(
        newsize.wrapping_mul(::core::mem::size_of::<*mut BUCKET>() as libc::c_ulong),
        b"grow_int_table\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"int_array.c\0" as *const u8 as *const libc::c_char,
        844 as libc::c_int,
    ) as *mut *mut BUCKET;
    old = (*symbol).sub.nodep.r.bv;
    (*symbol).sub.nodep.r.bv = new;
    (*symbol).sub.nodep.cnt = newsize;
    if old.is_null() {
        return;
    }
    k = 0 as libc::c_int as libc::c_ulong;
    while k < oldsize {
        let mut num: libc::c_long = 0;
        chain = *old.offset(k as isize);
        while !chain.is_null() {
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) < (*chain).hi.cnt {
                num = (*chain).hi.li[i as usize];
                let ref mut fresh15 = *int_insert(
                    symbol,
                    num,
                    int_hash(num as uint32_t, newsize as uint32_t),
                );
                *fresh15 = (*chain).hi.val[i as usize];
                i += 1;
                i;
            }
            next = (*chain).hi.next;
            let ref mut fresh16 = (*(chain as *mut block_item)).freep;
            *fresh16 = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep;
            nextfree[BLOCK_BUCKET as libc::c_int as usize]
                .freep = chain as *mut block_item;
            chain = next;
        }
        k = k.wrapping_add(1);
        k;
    }
    pma_free(old as *mut libc::c_void);
}
