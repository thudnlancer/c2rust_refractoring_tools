#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut Nnull_string: *mut NODE;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    fn assoc_info(
        subs: *mut NODE,
        val: *mut NODE,
        p: *mut NODE,
        aname: *const libc::c_char,
    );
    fn make_aname(symbol: *const NODE) -> *const libc::c_char;
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn assoc_copy(symbol: *mut NODE, newsymb: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn make_array() -> *mut NODE;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    static mut nextfree: [block_header; 2];
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    static mut success_node: *mut NODE;
    fn r_unref(tmp: *mut NODE);
    fn new_array_element() -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn getenv_long(name: *const libc::c_char) -> libc::c_long;
    static mut do_flags: do_flag_values;
    static mut output_fp: *mut FILE;
    fn indent(indent_level: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub sub: C2RustUnnamed_0,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: libc::c_long,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flagvals {
    REGEX = 524288,
    NUMCONSTSTR = 262144,
    XARRAY = 131072,
    HALFHAT = 65536,
    ARRAYMAXED = 32768,
    NULL_FIELD = 16384,
    NO_EXT_SET = 8192,
    MPZN = 4096,
    MPFN = 2048,
    WSTRCUR = 1024,
    INTIND = 512,
    NUMINT = 256,
    INTLSTR = 128,
    BOOLVAL = 64,
    USER_INPUT = 32,
    NUMBER = 16,
    NUMCUR = 8,
    STRCUR = 4,
    STRING = 2,
    MALLOC = 1,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub nodep: C2RustUnnamed_2,
    pub val: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    FOR_COMMENT = 3,
    BLOCK_COMMENT = 2,
    EOL_COMMENT = 1,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub l: C2RustUnnamed_9,
    pub r: C2RustUnnamed_4,
    pub x: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub d: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum opcodeval {
    Op_final = 122,
    Op_parens = 121,
    Op_cond_exp = 120,
    Op_K_function = 119,
    Op_K_else = 118,
    Op_K_if = 117,
    Op_K_switch = 116,
    Op_K_while = 115,
    Op_K_arrayfor = 114,
    Op_K_for = 113,
    Op_K_do = 112,
    Op_list = 111,
    Op_symbol = 110,
    Op_token = 109,
    Op_stop = 108,
    Op_atexit = 107,
    Op_lint_plus = 106,
    Op_lint = 105,
    Op_breakpoint = 104,
    Op_exec_count = 103,
    Op_comment = 102,
    Op_func = 101,
    Op_after_endfile = 100,
    Op_after_beginfile = 99,
    Op_subscript_assign = 98,
    Op_field_assign = 97,
    Op_var_assign = 96,
    Op_var_update = 95,
    Op_arrayfor_final = 94,
    Op_arrayfor_incr = 93,
    Op_arrayfor_init = 92,
    Op_newfile = 91,
    Op_get_record = 90,
    Op_jmp_false = 89,
    Op_jmp_true = 88,
    Op_jmp = 87,
    Op_pop = 86,
    Op_no_op = 85,
    Op_field_spec_lhs = 84,
    Op_subscript_lhs = 83,
    Op_push_lhs = 82,
    Op_push_param = 81,
    Op_push_array = 80,
    Op_push_re = 79,
    Op_push_i = 78,
    Op_push_arg_untyped = 77,
    Op_push_arg = 76,
    Op_push = 75,
    Op_indirect_func_call = 74,
    Op_func_call = 73,
    Op_in_array = 72,
    Op_ext_builtin = 71,
    Op_sub_builtin = 70,
    Op_builtin = 69,
    Op_K_namespace = 68,
    Op_K_nextfile = 67,
    Op_K_getline = 66,
    Op_K_getline_redir = 65,
    Op_K_delete_loop = 64,
    Op_K_delete = 63,
    Op_K_return_from_eval = 62,
    Op_K_return = 61,
    Op_K_exit = 60,
    Op_K_next = 59,
    Op_K_printf = 58,
    Op_K_print_rec = 57,
    Op_K_print = 56,
    Op_K_continue = 55,
    Op_K_break = 54,
    Op_K_default = 53,
    Op_K_case = 52,
    Op_rule = 51,
    Op_nomatch = 50,
    Op_match_rec = 49,
    Op_match = 48,
    Op_geq = 47,
    Op_leq = 46,
    Op_greater = 45,
    Op_less = 44,
    Op_notequal = 43,
    Op_equal = 42,
    Op_or_final = 41,
    Op_or = 40,
    Op_and_final = 39,
    Op_and = 38,
    Op_assign_concat = 37,
    Op_assign_exp = 36,
    Op_assign_minus = 35,
    Op_assign_plus = 34,
    Op_assign_mod = 33,
    Op_assign_quotient = 32,
    Op_assign_times = 31,
    Op_store_field_exp = 30,
    Op_store_field = 29,
    Op_store_sub = 28,
    Op_store_var = 27,
    Op_assign = 26,
    Op_not = 25,
    Op_field_spec = 24,
    Op_unary_plus = 23,
    Op_unary_minus = 22,
    Op_postdecrement = 21,
    Op_postincrement = 20,
    Op_predecrement = 19,
    Op_preincrement = 18,
    Op_sub_array = 17,
    Op_subscript = 16,
    Op_cond_pair = 15,
    Op_line_range = 14,
    Op_concat = 13,
    Op_exp_i = 12,
    Op_exp = 11,
    Op_minus_i = 10,
    Op_minus = 9,
    Op_plus_i = 8,
    Op_plus = 7,
    Op_mod_i = 6,
    Op_mod = 5,
    Op_quotient_i = 4,
    Op_quotient = 3,
    Op_times_i = 2,
    Op_times = 1,
    Op_illegal = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
pub union C2RustUnnamed_6 {
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
    pub hs: C2RustUnnamed_8,
    pub hi: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum do_flag_values {
    DO_MPFR = 32768,
    DO_DEBUG = 16384,
    DO_PROFILE = 8192,
    DO_SANDBOX = 4096,
    DO_TIDY_MEM = 2048,
    DO_DUMP_VARS = 1024,
    DO_PRETTY_PRINT = 512,
    DO_INTERVALS = 256,
    DO_NON_DEC_DATA = 128,
    DO_INTL = 64,
    DO_POSIX = 32,
    DO_TRADITIONAL = 16,
    DO_LINT_OLD = 8,
    DO_LINT_ALL = 4,
    DO_LINT_EXTENSIONS = 2,
    DO_LINT_INVALID = 1,
    DO_FLAG_NONE = 0,
}  // end of enum

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
unsafe extern "C" fn str_terminate_f(mut n: *mut NODE, mut savep: *mut libc::c_char) {
    *savep = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
}
static mut STR_CHAIN_MAX: size_t = 2 as libc::c_int as size_t;
#[no_mangle]
pub static mut str_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"str\0" as *const u8 as *const libc::c_char,
            init: Some(
                str_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: None,
            lookup: Some(
                str_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                str_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                str_clear as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                str_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                str_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                str_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                str_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
static mut env_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"env\0" as *const u8 as *const libc::c_char,
            init: Some(
                str_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: None,
            lookup: Some(
                str_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                str_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                env_clear as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                env_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                str_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                str_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                str_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: Some(
                env_store as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut hash: Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        size_t,
        libc::c_ulong,
        *mut size_t,
    ) -> libc::c_ulong,
> = unsafe {
    Some(
        awk_hash
            as unsafe extern "C" fn(
                *const libc::c_char,
                size_t,
                libc::c_ulong,
                *mut size_t,
            ) -> libc::c_ulong,
    )
};
unsafe extern "C" fn str_array_init(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if symbol.is_null() {
        let mut newval: libc::c_long = 0;
        let mut val: *const libc::c_char = 0 as *const libc::c_char;
        newval = getenv_long(b"STR_CHAIN_MAX\0" as *const u8 as *const libc::c_char);
        if newval > 0 as libc::c_int as libc::c_long {
            STR_CHAIN_MAX = newval as size_t;
        }
        val = getenv(b"AWK_HASH\0" as *const u8 as *const libc::c_char);
        if !val.is_null() {
            if strcmp(val, b"gst\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                hash = Some(
                    gst_hash_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            size_t,
                            libc::c_ulong,
                            *mut size_t,
                        ) -> libc::c_ulong,
                );
            } else if strcmp(val, b"fnv1a\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                hash = Some(
                    fnv1a_hash_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            size_t,
                            libc::c_ulong,
                            *mut size_t,
                        ) -> libc::c_ulong,
                );
            }
        }
    } else {
        null_array(symbol);
    }
    return &mut success_node;
}
unsafe extern "C" fn str_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: libc::c_ulong = 0;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut code1: size_t = 0;
    subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
    if ((*symbol).sub.nodep.r.bv).is_null() {
        grow_table(symbol);
    }
    hash1 = hash
        .expect(
            "non-null function pointer",
        )((*subs).sub.val.sp, (*subs).sub.val.slen, (*symbol).sub.nodep.cnt, &mut code1);
    lhs = str_find(symbol, subs, code1, hash1);
    if !lhs.is_null() {
        return lhs;
    }
    (*symbol).sub.nodep.reflags += 1;
    (*symbol).sub.nodep.reflags;
    if (*symbol).flags as libc::c_uint & ARRAYMAXED as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
        && ((*symbol).sub.nodep.reflags as libc::c_ulong)
            .wrapping_div((*symbol).sub.nodep.cnt) > STR_CHAIN_MAX
    {
        grow_table(symbol);
        hash1 = code1.wrapping_rem((*symbol).sub.nodep.cnt);
    }
    if (*subs).sub.val.idx != -(1 as libc::c_int) || subs == Nnull_string
        || (*subs).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        || (*subs).flags as libc::c_uint & NULL_FIELD as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut tmp: *mut NODE = 0 as *mut NODE;
        tmp = make_str_node((*subs).sub.val.sp, (*subs).sub.val.slen, 0 as libc::c_int);
        if (*subs).flags as libc::c_uint
            & (MPFN as libc::c_int | MPZN as libc::c_int | NUMCUR as libc::c_int)
                as libc::c_uint == NUMCUR as libc::c_int as libc::c_uint
        {
            (*tmp).sub.val.fltnum = (*subs).sub.val.fltnum;
            (*tmp)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*tmp).flags as libc::c_uint | NUMCUR as libc::c_int as libc::c_uint);
        }
        subs = tmp;
    } else {
        subs = dupnode(subs);
    }
    b = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep as *mut BUCKET;
    if !b.is_null() {
        nextfree[BLOCK_BUCKET as libc::c_int as usize]
            .freep = (*(b as *mut block_item)).freep;
    } else {
        b = more_blocks(BLOCK_BUCKET as libc::c_int) as *mut BUCKET;
    };
    (*b).hs.next = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    let ref mut fresh0 = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    *fresh0 = b;
    (*b).hs.name = subs;
    (*b).hs.str_0 = (*subs).sub.val.sp;
    (*b).hs.len = (*subs).sub.val.slen;
    (*b).hs.val = new_array_element();
    (*b).hs.code = code1;
    return &mut (*b).hs.val;
}
unsafe extern "C" fn str_exists(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: libc::c_ulong = 0;
    let mut code1: size_t = 0;
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut *mut NODE;
    }
    subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
    hash1 = hash
        .expect(
            "non-null function pointer",
        )((*subs).sub.val.sp, (*subs).sub.val.slen, (*symbol).sub.nodep.cnt, &mut code1);
    return str_find(symbol, subs, code1, hash1);
}
unsafe extern "C" fn str_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: libc::c_ulong = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut next: *mut BUCKET = 0 as *mut BUCKET;
    let mut r: *mut NODE = 0 as *mut NODE;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            next = (*b).hs.next;
            r = (*b).hs.val;
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
            unref((*b).hs.name);
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
unsafe extern "C" fn str_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut hash1: libc::c_ulong = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut prev: *mut BUCKET = 0 as *mut BUCKET;
    let mut s2: *mut NODE = 0 as *mut NODE;
    let mut s1_len: size_t = 0;
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut *mut NODE;
    }
    s2 = force_string_fmt(subs, CONVFMT, CONVFMTidx);
    hash1 = hash
        .expect(
            "non-null function pointer",
        )(
        (*s2).sub.val.sp,
        (*s2).sub.val.slen,
        (*symbol).sub.nodep.cnt,
        0 as *mut size_t,
    );
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    prev = 0 as *mut BUCKET;
    while !b.is_null() {
        s1_len = (*b).hs.len;
        if !(s1_len != (*s2).sub.val.slen) {
            if s1_len == 0 as libc::c_int as libc::c_ulong
                || memcmp(
                    (*b).hs.str_0 as *const libc::c_void,
                    (*s2).sub.val.sp as *const libc::c_void,
                    s1_len,
                ) == 0 as libc::c_int
            {
                unref((*b).hs.name);
                if !prev.is_null() {
                    (*prev).hs.next = (*b).hs.next;
                } else {
                    let ref mut fresh4 = *((*symbol).sub.nodep.r.bv)
                        .offset(hash1 as isize);
                    *fresh4 = (*b).hs.next;
                }
                let ref mut fresh5 = (*(b as *mut block_item)).freep;
                *fresh5 = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep;
                nextfree[BLOCK_BUCKET as libc::c_int as usize]
                    .freep = b as *mut block_item;
                (*symbol).sub.nodep.reflags -= 1;
                if (*symbol).sub.nodep.reflags as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    if !((*symbol).sub.nodep.r.bv).is_null() {
                        pma_free((*symbol).sub.nodep.r.bv as *mut libc::c_void);
                    }
                    ((*(*symbol).sub.nodep.l.lp).init)
                        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
                }
                return &mut success_node;
            }
        }
        prev = b;
        b = (*b).hs.next;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn str_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut *mut NODE {
    let mut old: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut new: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut pnew: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut chain: *mut BUCKET = 0 as *mut BUCKET;
    let mut newchain: *mut BUCKET = 0 as *mut BUCKET;
    let mut cursize: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    cursize = (*symbol).sub.nodep.cnt;
    new = ezalloc_real(
        cursize.wrapping_mul(::core::mem::size_of::<*mut BUCKET>() as libc::c_ulong),
        b"str_copy\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"str_array.c\0" as *const u8 as *const libc::c_char,
        342 as libc::c_int,
    ) as *mut *mut BUCKET;
    old = (*symbol).sub.nodep.r.bv;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < cursize {
        chain = *old.offset(i as isize);
        pnew = &mut *new.offset(i as isize) as *mut *mut BUCKET;
        while !chain.is_null() {
            let mut oldval: *mut NODE = 0 as *mut NODE;
            let mut newsubs: *mut NODE = 0 as *mut NODE;
            newchain = nextfree[BLOCK_BUCKET as libc::c_int as usize].freep
                as *mut BUCKET;
            if !newchain.is_null() {
                nextfree[BLOCK_BUCKET as libc::c_int as usize]
                    .freep = (*(newchain as *mut block_item)).freep;
            } else {
                newchain = more_blocks(BLOCK_BUCKET as libc::c_int) as *mut BUCKET;
            };
            (*newchain).hs.name = dupnode((*chain).hs.name);
            newsubs = (*newchain).hs.name;
            (*newchain).hs.str_0 = (*newsubs).sub.val.sp;
            (*newchain).hs.len = (*newsubs).sub.val.slen;
            oldval = (*chain).hs.val;
            if (*oldval).type_0 as libc::c_uint
                == Node_val as libc::c_int as libc::c_uint
            {
                (*newchain).hs.val = dupnode(oldval);
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
                (*newchain).hs.val = assoc_copy(oldval, r);
            }
            (*newchain).hs.code = (*chain).hs.code;
            *pnew = newchain;
            (*newchain).hs.next = 0 as *mut bucket_item;
            pnew = &mut (*newchain).hs.next;
            chain = (*chain).hs.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*newsymb).sub.nodep.reflags = (*symbol).sub.nodep.reflags;
    (*newsymb).sub.nodep.r.bv = new;
    (*newsymb).sub.nodep.cnt = cursize;
    (*newsymb).flags = (*symbol).flags;
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn str_list(
    mut symbol: *mut NODE,
    mut t: *mut NODE,
) -> *mut *mut NODE {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut num_elems: libc::c_ulong = 0;
    let mut list_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut elem_size: libc::c_int = 1 as libc::c_int;
    let mut assoc_kind: assoc_kind_t = ANONE;
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut *mut NODE;
    }
    assoc_kind = (*t).flags as assoc_kind_t;
    if assoc_kind as libc::c_uint
        & (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
        == (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
    {
        elem_size = 2 as libc::c_int;
    }
    num_elems = (*symbol).sub.nodep.reflags as libc::c_ulong;
    if assoc_kind as libc::c_uint
        & (AINDEX as libc::c_int | AVALUE as libc::c_int | ADELETE as libc::c_int)
            as libc::c_uint
        == (AINDEX as libc::c_int | ADELETE as libc::c_int) as libc::c_uint
    {
        num_elems = 1 as libc::c_int as libc::c_ulong;
    }
    list_size = (elem_size as libc::c_ulong).wrapping_mul(num_elems);
    list = emalloc_real(
        list_size.wrapping_mul(::core::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"str_list\0" as *const u8 as *const libc::c_char,
        b"list\0" as *const u8 as *const libc::c_char,
        b"str_array.c\0" as *const u8 as *const libc::c_char,
        415 as libc::c_int,
    ) as *mut *mut NODE;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            subs = (*b).hs.name;
            if assoc_kind as libc::c_uint & AINUM as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                force_number(subs);
            }
            let fresh6 = k;
            k = k.wrapping_add(1);
            let ref mut fresh7 = *list.offset(fresh6 as isize);
            *fresh7 = dupnode(subs);
            if assoc_kind as libc::c_uint & AVALUE as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                val = (*b).hs.val;
                if (*val).type_0 as libc::c_uint
                    == Node_val as libc::c_int as libc::c_uint
                {
                    if assoc_kind as libc::c_uint & AVNUM as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        force_number(val);
                    } else if assoc_kind as libc::c_uint
                        & AVSTR as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        val = force_string_fmt(val, CONVFMT, CONVFMTidx);
                    }
                }
                let fresh8 = k;
                k = k.wrapping_add(1);
                let ref mut fresh9 = *list.offset(fresh8 as isize);
                *fresh9 = val;
            }
            if k >= list_size {
                return list;
            }
            b = (*b).hs.next;
        }
        i = i.wrapping_add(1);
        i;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn str_kilobytes(mut symbol: *mut NODE) -> libc::c_double {
    let mut bucket_cnt: libc::c_ulong = 0;
    let mut kb: libc::c_double = 0.;
    bucket_cnt = (*symbol).sub.nodep.reflags as libc::c_ulong;
    kb = (bucket_cnt as libc::c_double
        * ::core::mem::size_of::<BUCKET>() as libc::c_ulong as libc::c_double
        + (*symbol).sub.nodep.cnt as libc::c_double
            * ::core::mem::size_of::<*mut BUCKET>() as libc::c_ulong as libc::c_double)
        / 1024.0f64;
    return kb;
}
unsafe extern "C" fn str_dump(
    mut symbol: *mut NODE,
    mut ndump: *mut NODE,
) -> *mut *mut NODE {
    let mut indent_level: libc::c_int = 0;
    let mut i: libc::c_ulong = 0;
    let mut bucket_cnt: libc::c_ulong = 0;
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    static mut hash_dist: [size_t; 32] = [0; 32];
    indent_level = (*ndump).sub.nodep.x.xl as libc::c_int;
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
        b"array_func: str_array_func\n\0" as *const u8 as *const libc::c_char,
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
        b"STR_CHAIN_MAX: %lu\n\0" as *const u8 as *const libc::c_char,
        STR_CHAIN_MAX,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_size: %lu\n\0" as *const u8 as *const libc::c_char,
        (*symbol).sub.nodep.cnt,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"table_size: %lu\n\0" as *const u8 as *const libc::c_char,
        (*symbol).sub.nodep.reflags as libc::c_ulong,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"Avg # of items per chain: %.2g\n\0" as *const u8 as *const libc::c_char,
        (*symbol).sub.nodep.reflags as libc::c_double
            / (*symbol).sub.nodep.cnt as libc::c_double,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"memory: %.2g kB\n\0" as *const u8 as *const libc::c_char,
        str_kilobytes(symbol),
    );
    memset(
        hash_dist.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ((31 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*symbol).sub.nodep.cnt {
        bucket_cnt = 0 as libc::c_int as libc::c_ulong;
        b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
        while !b.is_null() {
            bucket_cnt = bucket_cnt.wrapping_add(1);
            bucket_cnt;
            b = (*b).hs.next;
        }
        if bucket_cnt >= 31 as libc::c_int as libc::c_ulong {
            bucket_cnt = 31 as libc::c_int as libc::c_ulong;
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
    i = 0 as libc::c_int as libc::c_ulong;
    while i <= 31 as libc::c_int as libc::c_ulong {
        if hash_dist[i as usize] > 0 as libc::c_int as libc::c_ulong {
            indent(indent_level);
            if i == 31 as libc::c_int as libc::c_ulong {
                fprintf(
                    output_fp,
                    b"[>=%lu]:%lu\n\0" as *const u8 as *const libc::c_char,
                    31 as libc::c_int as libc::c_ulong,
                    hash_dist[i as usize],
                );
            } else {
                fprintf(
                    output_fp,
                    b"[%lu]:%lu\n\0" as *const u8 as *const libc::c_char,
                    i,
                    hash_dist[i as usize],
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    indent_level -= 1;
    indent_level;
    if (*ndump).sub.nodep.l.ll >= 0 as libc::c_int as libc::c_long {
        let mut aname: *const libc::c_char = 0 as *const libc::c_char;
        fprintf(output_fp, b"\n\0" as *const u8 as *const libc::c_char);
        aname = make_aname(symbol);
        i = 0 as libc::c_int as libc::c_ulong;
        while i < (*symbol).sub.nodep.cnt {
            b = *((*symbol).sub.nodep.r.bv).offset(i as isize);
            while !b.is_null() {
                assoc_info((*b).hs.name, (*b).hs.val, ndump, aname);
                b = (*b).hs.next;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn awk_hash(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut hsize: libc::c_ulong,
    mut code: *mut size_t,
) -> libc::c_ulong {
    let mut h: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut htmp: libc::c_ulong = 0;
    h = 0 as libc::c_int as libc::c_ulong;
    if len > 0 as libc::c_int as libc::c_ulong {
        let mut loop_0: size_t = len
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int;
        's_139: {
            let mut current_block_32: u64;
            match len & (8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
                0 => {
                    current_block_32 = 10680521327981672866;
                }
                7 => {
                    current_block_32 = 5499897176314894263;
                }
                6 => {
                    current_block_32 = 14343490084333691418;
                }
                5 => {
                    current_block_32 = 3909645888961320850;
                }
                4 => {
                    current_block_32 = 10260081853797976808;
                }
                3 => {
                    current_block_32 = 3347075717393403157;
                }
                2 => {
                    current_block_32 = 12327483733956094797;
                }
                1 => {
                    current_block_32 = 11609910033890719537;
                }
                _ => {
                    current_block_32 = 14763689060501151050;
                }
            }
            loop {
                match current_block_32 {
                    14763689060501151050 => {
                        break 's_139;
                    }
                    12327483733956094797 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh16 = s;
                        s = s.offset(1);
                        h = (*fresh16 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 11609910033890719537;
                    }
                    3347075717393403157 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh15 = s;
                        s = s.offset(1);
                        h = (*fresh15 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 12327483733956094797;
                    }
                    10260081853797976808 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh14 = s;
                        s = s.offset(1);
                        h = (*fresh14 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 3347075717393403157;
                    }
                    3909645888961320850 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh13 = s;
                        s = s.offset(1);
                        h = (*fresh13 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 10260081853797976808;
                    }
                    14343490084333691418 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh12 = s;
                        s = s.offset(1);
                        h = (*fresh12 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 3909645888961320850;
                    }
                    5499897176314894263 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh11 = s;
                        s = s.offset(1);
                        h = (*fresh11 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 14343490084333691418;
                    }
                    10680521327981672866 => {
                        htmp = h << 6 as libc::c_int;
                        let fresh10 = s;
                        s = s.offset(1);
                        h = (*fresh10 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        current_block_32 = 5499897176314894263;
                    }
                    _ => {
                        htmp = h << 6 as libc::c_int;
                        let fresh17 = s;
                        s = s.offset(1);
                        h = (*fresh17 as libc::c_ulong)
                            .wrapping_add(htmp)
                            .wrapping_add(htmp << 10 as libc::c_int)
                            .wrapping_sub(h);
                        htmp &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        h &= 0xffffffff as libc::c_uint as libc::c_ulong;
                        loop_0 = loop_0.wrapping_sub(1);
                        if loop_0 != 0 {
                            current_block_32 = 10680521327981672866;
                        } else {
                            current_block_32 = 14763689060501151050;
                        }
                    }
                }
            }
        }
    }
    if !code.is_null() {
        *code = h;
    }
    if h >= hsize {
        h = h.wrapping_rem(hsize);
    }
    return h;
}
#[inline]
unsafe extern "C" fn str_find(
    mut symbol: *mut NODE,
    mut s1: *mut NODE,
    mut code1: size_t,
    mut hash1: libc::c_ulong,
) -> *mut *mut NODE {
    let mut b: *mut BUCKET = 0 as *mut BUCKET;
    let mut s2_len: size_t = 0;
    b = *((*symbol).sub.nodep.r.bv).offset(hash1 as isize);
    while !b.is_null() {
        s2_len = (*b).hs.len;
        if code1 == (*b).hs.code && (*s1).sub.val.slen == s2_len
            && (s2_len == 0 as libc::c_int as libc::c_ulong
                || memcmp(
                    (*s1).sub.val.sp as *const libc::c_void,
                    (*b).hs.str_0 as *const libc::c_void,
                    s2_len,
                ) == 0 as libc::c_int)
        {
            return &mut (*b).hs.val;
        }
        b = (*b).hs.next;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn grow_table(mut symbol: *mut NODE) {
    let mut old: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut new: *mut *mut BUCKET = 0 as *mut *mut BUCKET;
    let mut chain: *mut BUCKET = 0 as *mut BUCKET;
    let mut next: *mut BUCKET = 0 as *mut BUCKET;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut oldsize: libc::c_ulong = 0;
    let mut newsize: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0;
    let mut hash1: libc::c_ulong = 0;
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
        b"grow_table\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"str_array.c\0" as *const u8 as *const libc::c_char,
        682 as libc::c_int,
    ) as *mut *mut BUCKET;
    old = (*symbol).sub.nodep.r.bv;
    (*symbol).sub.nodep.r.bv = new;
    (*symbol).sub.nodep.cnt = newsize;
    if old.is_null() {
        (*symbol).sub.nodep.reflags = 0 as reflagvals;
        return;
    }
    k = 0 as libc::c_int as libc::c_ulong;
    while k < oldsize {
        chain = *old.offset(k as isize);
        while !chain.is_null() {
            next = (*chain).hs.next;
            hash1 = ((*chain).hs.code).wrapping_rem(newsize);
            (*chain).hs.next = *new.offset(hash1 as isize);
            let ref mut fresh18 = *new.offset(hash1 as isize);
            *fresh18 = chain;
            chain = next;
        }
        k = k.wrapping_add(1);
        k;
    }
    pma_free(old as *mut libc::c_void);
}
unsafe extern "C" fn gst_hash_string(
    mut str: *const libc::c_char,
    mut len: size_t,
    mut hsize: libc::c_ulong,
    mut code: *mut size_t,
) -> libc::c_ulong {
    let mut hashVal: libc::c_ulong = 1497032417 as libc::c_int as libc::c_ulong;
    let mut ret: libc::c_ulong = 0;
    loop {
        let fresh19 = len;
        len = len.wrapping_sub(1);
        if !(fresh19 != 0) {
            break;
        }
        let fresh20 = str;
        str = str.offset(1);
        hashVal = hashVal.wrapping_add(*fresh20 as libc::c_ulong);
        hashVal = hashVal.wrapping_add(hashVal << 10 as libc::c_int);
        hashVal ^= hashVal >> 6 as libc::c_int;
    }
    ret = scramble(hashVal);
    if !code.is_null() {
        *code = ret;
    }
    if ret >= hsize {
        ret = ret.wrapping_rem(hsize);
    }
    return ret;
}
unsafe extern "C" fn scramble(mut x: libc::c_ulong) -> libc::c_ulong {
    if ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
    {
        let mut y: libc::c_int = !x as libc::c_int;
        x = x
            .wrapping_add(
                (y << 10 as libc::c_int | y >> 22 as libc::c_int) as libc::c_ulong,
            );
        x = x.wrapping_add(x << 6 as libc::c_int | x >> 26 as libc::c_int);
        x = x.wrapping_sub(x << 16 as libc::c_int | x >> 16 as libc::c_int);
    } else {
        x ^= !x >> 31 as libc::c_int;
        x = x.wrapping_add(x << 21 as libc::c_int | x >> 11 as libc::c_int);
        x = x.wrapping_add(x << 5 as libc::c_int | x >> 27 as libc::c_int);
        x = x.wrapping_add(x << 27 as libc::c_int | x >> 5 as libc::c_int);
        x = x.wrapping_add(x << 31 as libc::c_int);
    }
    return x;
}
unsafe extern "C" fn fnv1a_hash_string(
    mut str: *const libc::c_char,
    mut len: size_t,
    mut hsize: libc::c_ulong,
    mut code: *mut size_t,
) -> libc::c_ulong {
    let mut ret: libc::c_uint = 2166136261 as libc::c_uint;
    while len > 0 as libc::c_int as libc::c_ulong {
        let fresh21 = str;
        str = str.offset(1);
        ret ^= *fresh21 as libc::c_uchar as libc::c_uint;
        ret = ret.wrapping_mul(16777619 as libc::c_uint);
        len = len.wrapping_sub(1);
        len;
    }
    if !code.is_null() {
        *code = ret as size_t;
    }
    if ret as libc::c_ulong >= hsize {
        ret = (ret as libc::c_ulong).wrapping_rem(hsize) as libc::c_uint as libc::c_uint;
    }
    return ret as libc::c_ulong;
}
unsafe extern "C" fn env_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut val: *mut *mut NODE = str_remove(symbol, subs);
    let mut save: libc::c_char = 0;
    if !val.is_null() {
        str_terminate_f(subs, &mut save);
        unsetenv((*subs).sub.val.sp);
        *((*subs).sub.val.sp).offset((*subs).sub.val.slen as isize) = save;
    }
    return val;
}
unsafe extern "C" fn env_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    extern "C" {
        #[link_name = "environ"]
        static mut environ_0: *mut *mut libc::c_char;
    }
    let mut val: *mut *mut NODE = str_clear(symbol, subs);
    environ = 0 as *mut *mut libc::c_char;
    (*symbol).sub.nodep.l.lp = &env_array_func;
    return val;
}
unsafe extern "C" fn env_store(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut val: *mut *mut NODE = str_exists(symbol, subs);
    let mut newval: *const libc::c_char = 0 as *const libc::c_char;
    newval = (**val).sub.val.sp;
    if newval.is_null() {
        newval = b"\0" as *const u8 as *const libc::c_char;
    }
    setenv((*subs).sub.val.sp, newval, 1 as libc::c_int);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn init_env_array(mut env_node: *mut NODE) {
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
        return;
    }
    (*env_node).sub.nodep.l.lp = &env_array_func;
}
