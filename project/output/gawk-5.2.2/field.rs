#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    static mut NF: libc::c_long;
    static mut IGNORECASE: bool;
    static mut RS_is_null: bool;
    static mut OFS: *mut libc::c_char;
    static mut OFSlen: libc::c_int;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut FIELDWIDTHS_node: *mut NODE;
    static mut FS_node: *mut NODE;
    static mut NF_node: *mut NODE;
    static mut RS_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut FPAT_node: *mut NODE;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: libc::c_int;
    static mut stack_ptr: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn check_symtab_functab(
        dest: *mut NODE,
        fname: *const libc::c_char,
        msg: *const libc::c_char,
    );
    fn check_args_min_max(
        nargs: libc::c_int,
        fname: *const libc::c_char,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn research(
        rp: *mut Regexp,
        str: *mut libc::c_char,
        start: libc::c_int,
        len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn r_free_wstr(n: *mut NODE);
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    fn make_regexp(
        s: *const libc::c_char,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn refree(rp: *mut Regexp);
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type mbstate_t = __mbstate_t;
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
    awk_true = 1,
    awk_false = 0,
}  // end of enum

pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
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
    AWK_NUMBER_TYPE_MPZ = 2,
    AWK_NUMBER_TYPE_MPFR = 1,
    AWK_NUMBER_TYPE_DOUBLE = 0,
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
    AWK_BOOL = 8,
    AWK_VALUE_COOKIE = 7,
    AWK_SCALAR = 6,
    AWK_ARRAY = 5,
    AWK_STRNUM = 4,
    AWK_REGEX = 3,
    AWK_STRING = 2,
    AWK_NUMBER = 1,
    AWK_UNDEFINED = 0,
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
pub enum defrule {
    MAXRULE = 6,
    ENDFILE = 5,
    BEGINFILE = 4,
    END = 3,
    Rule = 2,
    BEGIN = 1,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nodevals {
    Node_final = 19,
    Node_instruction = 18,
    Node_frame = 17,
    Node_arrayfor = 16,
    Node_dump_array = 15,
    Node_array_leaf = 14,
    Node_array_tree = 13,
    Node_array_ref = 12,
    Node_builtin_func = 11,
    Node_ext_func = 10,
    Node_func = 9,
    Node_param_list = 8,
    Node_elem_new = 7,
    Node_var_new = 6,
    Node_var_array = 5,
    Node_var = 4,
    Node_dynregex = 3,
    Node_regex = 2,
    Node_val = 1,
    Node_illegal = 0,
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
    FOR_COMMENT = 3,
    BLOCK_COMMENT = 2,
    EOL_COMMENT = 1,
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
    FS_DFLT = 2,
    CONSTANT = 1,
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
pub type Func_ptr = Option::<unsafe extern "C" fn() -> ()>;
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
    BLOCK_MAX = 2,
    BLOCK_BUCKET = 1,
    BLOCK_NODE = 0,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
pub type Setfunc = Option::<
    unsafe extern "C" fn(libc::c_long, *mut libc::c_char, libc::c_long, *mut NODE) -> (),
>;
pub type parse_field_func_t = Option::<
    unsafe extern "C" fn(
        libc::c_long,
        *mut *mut libc::c_char,
        libc::c_int,
        *mut NODE,
        *mut Regexp,
        Setfunc,
        *mut NODE,
        *mut NODE,
        bool,
    ) -> libc::c_long,
>;
pub const Using_FPAT: field_sep_type = 2;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum field_sep_type {
    Using_FPAT = 2,
    Using_API = 3,
    Using_FIELDWIDTHS = 1,
    Using_FS = 0,
}  // end of enum

#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as libc::c_int as libc::c_long {
        return;
    }
    r_unref(r);
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
unsafe extern "C" fn assoc_set(
    mut array: *mut NODE,
    mut sub: *mut NODE,
    mut value: *mut NODE,
) {
    let mut lhs: *mut *mut NODE = ((*(*array).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(array, sub);
    unref(*lhs);
    *lhs = value;
    if ((*(*array).sub.nodep.l.lp).store).is_some() {
        (Some(((*(*array).sub.nodep.l.lp).store).expect("non-null function pointer")))
            .expect("non-null function pointer")(array, sub);
    }
    unref(sub);
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
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh0 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh0).rptr;
    if (*t).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            1881 as libc::c_int,
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
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        t = elem_new_to_scalar(t);
    }
    return t;
}
#[inline]
unsafe extern "C" fn TOP_SCALAR() -> *mut NODE {
    let mut t: *mut NODE = (*stack_ptr).rptr;
    if (*t).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            1896 as libc::c_int,
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
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        t = elem_new_to_scalar(t);
        (*stack_ptr).rptr = t;
    }
    return t;
}
#[inline]
unsafe extern "C" fn POP_PARAM() -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    return if (*t).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
    {
        t
    } else {
        force_array(t, 0 as libc::c_int != 0)
    };
}
#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const libc::c_char,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
unsafe extern "C" fn is_blank(mut c: libc::c_int) -> libc::c_int {
    return (c == ' ' as i32 || c == '\t' as i32) as libc::c_int;
}
static mut api_parser_override: bool = 0 as libc::c_int != 0;
static mut parse_field: parse_field_func_t = None;
static mut normal_parse_field: parse_field_func_t = None;
static mut api_fw: *const awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t;
static mut parse_extent: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut parse_high_water: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut nf_high_water: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut resave_fs: bool = false;
static mut save_FS: *mut NODE = 0 as *const NODE as *mut NODE;
static mut save_FPAT: *mut NODE = 0 as *const NODE as *mut NODE;
static mut FIELDWIDTHS: *mut awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t
    as *mut awk_fieldwidth_info_t;
#[no_mangle]
pub static mut fields_arr: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
#[no_mangle]
pub static mut field0_valid: bool = false;
static mut default_FS: libc::c_int = 0;
static mut FS_re_yes_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FS_re_no_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FS_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_re_yes_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_re_no_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
#[no_mangle]
pub static mut Null_field: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub unsafe extern "C" fn init_fields() {
    fields_arr = emalloc_real(
        ::core::mem::size_of::<*mut NODE>() as libc::c_ulong,
        b"init_fields\0" as *const u8 as *const libc::c_char,
        b"fields_arr\0" as *const u8 as *const libc::c_char,
        b"field.c\0" as *const u8 as *const libc::c_char,
        100 as libc::c_int,
    ) as *mut *mut NODE;
    let ref mut fresh2 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh2 = make_str_node(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    let ref mut fresh3 = (**fields_arr.offset(0 as libc::c_int as isize)).flags;
    *fresh3 = ::core::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(*fresh3 as libc::c_uint | NULL_FIELD as libc::c_int as libc::c_uint);
    parse_extent = (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp;
    save_FS = dupnode((*FS_node).sub.nodep.l.lptr);
    Null_field = make_str_node(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    (*Null_field)
        .flags = (STRCUR as libc::c_int | STRING as libc::c_int
        | NULL_FIELD as libc::c_int) as flagvals;
    field0_valid = 1 as libc::c_int != 0;
}
unsafe extern "C" fn grow_fields_arr(mut num: libc::c_long) {
    let mut t: libc::c_int = 0;
    let mut n: *mut NODE = 0 as *mut NODE;
    fields_arr = erealloc_real(
        fields_arr as *mut libc::c_void,
        ((num + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"grow_fields_arr\0" as *const u8 as *const libc::c_char,
        b"fields_arr\0" as *const u8 as *const libc::c_char,
        b"field.c\0" as *const u8 as *const libc::c_char,
        122 as libc::c_int,
    ) as *mut *mut NODE;
    t = (nf_high_water + 1 as libc::c_int as libc::c_long) as libc::c_int;
    while t as libc::c_long <= num {
        n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
        if !n.is_null() {
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = (*(n as *mut block_item)).freep;
        } else {
            n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
        };
        *n = *Null_field;
        let ref mut fresh4 = *fields_arr.offset(t as isize);
        *fresh4 = n;
        t += 1;
        t;
    }
    nf_high_water = num;
}
unsafe extern "C" fn set_field(
    mut num: libc::c_long,
    mut str: *mut libc::c_char,
    mut len: libc::c_long,
    mut dummy: *mut NODE,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    if num > nf_high_water {
        grow_fields_arr(num);
    }
    n = *fields_arr.offset(num as isize);
    (*n).sub.val.sp = str;
    (*n).sub.val.slen = len as size_t;
    (*n)
        .flags = (STRCUR as libc::c_int | STRING as libc::c_int
        | USER_INPUT as libc::c_int) as flagvals;
}
#[no_mangle]
pub unsafe extern "C" fn rebuild_record() {
    let mut tlen: libc::c_ulong = 0;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ops: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cops: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_long = 0;
    tlen = 0 as libc::c_int as libc::c_ulong;
    i = NF;
    while i > 0 as libc::c_int as libc::c_long {
        tmp = *fields_arr.offset(i as isize);
        tmp = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        tlen = tlen.wrapping_add((*tmp).sub.val.slen);
        i -= 1;
        i;
    }
    tlen = tlen
        .wrapping_add(
            ((NF - 1 as libc::c_int as libc::c_long) * OFSlen as libc::c_long)
                as libc::c_ulong,
        );
    if (tlen as libc::c_long) < 0 as libc::c_int as libc::c_long {
        tlen = 0 as libc::c_int as libc::c_ulong;
    }
    ops = emalloc_real(
        tlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"rebuild_record\0" as *const u8 as *const libc::c_char,
        b"ops\0" as *const u8 as *const libc::c_char,
        b"field.c\0" as *const u8 as *const libc::c_char,
        177 as libc::c_int,
    ) as *mut libc::c_char;
    cops = ops;
    *ops.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    i = 1 as libc::c_int as libc::c_long;
    while i <= NF {
        if (**fields_arr.offset(i as isize)).flags as libc::c_uint
            & WSTRCUR as libc::c_int as libc::c_uint != 0
        {
            r_free_wstr(*fields_arr.offset(i as isize));
        }
        tmp = *fields_arr.offset(i as isize);
        if (*tmp).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
            let fresh5 = cops;
            cops = cops.offset(1);
            *fresh5 = *((*tmp).sub.val.sp).offset(0 as libc::c_int as isize);
        } else if (*tmp).sub.val.slen != 0 as libc::c_int as libc::c_ulong {
            memcpy(
                cops as *mut libc::c_void,
                (*tmp).sub.val.sp as *const libc::c_void,
                (*tmp).sub.val.slen,
            );
            cops = cops.offset((*tmp).sub.val.slen as isize);
        }
        if i != NF {
            if OFSlen == 1 as libc::c_int {
                let fresh6 = cops;
                cops = cops.offset(1);
                *fresh6 = *OFS;
            } else if OFSlen != 0 as libc::c_int {
                memcpy(
                    cops as *mut libc::c_void,
                    OFS as *const libc::c_void,
                    OFSlen as libc::c_ulong,
                );
                cops = cops.offset(OFSlen as isize);
            }
        }
        i += 1;
        i;
    }
    tmp = make_str_node(ops, tlen, 2 as libc::c_int);
    cops = ops;
    i = 1 as libc::c_int as libc::c_long;
    while i <= NF {
        let mut r: *mut NODE = *fields_arr.offset(i as isize);
        if (*r).sub.val.slen > 0 as libc::c_int as libc::c_ulong
            && (*r).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            let mut n: *mut NODE = 0 as *mut NODE;
            n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*(n as *mut block_item)).freep;
            } else {
                n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
            };
            *n = *r;
            if (*r).valref > 1 as libc::c_int as libc::c_long {
                (*r)
                    .sub
                    .val
                    .sp = emalloc_real(
                    ((*r).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    b"rebuild_record\0" as *const u8 as *const libc::c_char,
                    b"r->stptr\0" as *const u8 as *const libc::c_char,
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    226 as libc::c_int,
                ) as *mut libc::c_char;
                memcpy(
                    (*r).sub.val.sp as *mut libc::c_void,
                    cops as *const libc::c_void,
                    (*r).sub.val.slen,
                );
                *((*r).sub.val.sp)
                    .offset((*r).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
                (*r)
                    .flags = ::core::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >((*r).flags as libc::c_uint | MALLOC as libc::c_int as libc::c_uint);
                (*n).valref = 1 as libc::c_int as libc::c_long;
            }
            (*n).sub.val.sp = cops;
            (*n)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >(
                (*n).flags as libc::c_uint
                    & !(MPFN as libc::c_int | MPZN as libc::c_int
                        | NUMCUR as libc::c_int) as libc::c_uint,
            );
            unref(r);
            let ref mut fresh7 = *fields_arr.offset(i as isize);
            *fresh7 = n;
        }
        cops = cops
            .offset(
                ((**fields_arr.offset(i as isize)).sub.val.slen)
                    .wrapping_add(OFSlen as libc::c_ulong) as isize,
            );
        i += 1;
        i;
    }
    unref(*fields_arr.offset(0 as libc::c_int as isize));
    let ref mut fresh8 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh8 = tmp;
    field0_valid = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_record(
    mut buf: *const libc::c_char,
    mut cnt: size_t,
    mut fw: *const awk_fieldwidth_info_t,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    static mut databuf: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut databuf_size: size_t = 0;
    purge_record();
    if databuf_size == 0 as libc::c_int as libc::c_ulong {
        databuf = ezalloc_real(
            512 as libc::c_int as size_t,
            b"set_record\0" as *const u8 as *const libc::c_char,
            b"databuf\0" as *const u8 as *const libc::c_char,
            b"field.c\0" as *const u8 as *const libc::c_char,
            276 as libc::c_int,
        ) as *mut libc::c_char;
        databuf_size = 512 as libc::c_int as size_t;
    }
    if cnt >= databuf_size {
        loop {
            if databuf_size
                > (!(0 as libc::c_int) as size_t)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    287 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"input record too large\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            databuf_size = (databuf_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if !(cnt >= databuf_size) {
                break;
            }
        }
        databuf = erealloc_real(
            databuf as *mut libc::c_void,
            databuf_size,
            b"set_record\0" as *const u8 as *const libc::c_char,
            b"databuf\0" as *const u8 as *const libc::c_char,
            b"field.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
        ) as *mut libc::c_char;
        memset(databuf as *mut libc::c_void, '\0' as i32, databuf_size);
    }
    if cnt != 0 as libc::c_int as libc::c_ulong {
        memcpy(databuf as *mut libc::c_void, buf as *const libc::c_void, cnt);
    }
    *databuf.offset(cnt as isize) = '\0' as i32 as libc::c_char;
    unref(*fields_arr.offset(0 as libc::c_int as isize));
    n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(n as *mut block_item)).freep;
    } else {
        n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*n).sub.val.sp = databuf;
    (*n).sub.val.slen = cnt;
    (*n).valref = 1 as libc::c_int as libc::c_long;
    (*n).type_0 = Node_val;
    (*n).sub.val.idx = -(1 as libc::c_int);
    (*n)
        .flags = (STRING as libc::c_int | STRCUR as libc::c_int
        | USER_INPUT as libc::c_int) as flagvals;
    let ref mut fresh9 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh9 = n;
    if fw != api_fw {
        api_fw = fw;
        if !api_fw.is_null() {
            if !api_parser_override {
                api_parser_override = 1 as libc::c_int != 0;
                parse_field = Some(
                    fw_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                );
                update_PROCINFO_str(
                    b"FS\0" as *const u8 as *const libc::c_char,
                    b"API\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if api_parser_override {
            api_parser_override = 0 as libc::c_int != 0;
            parse_field = normal_parse_field;
            update_PROCINFO_str(
                b"FS\0" as *const u8 as *const libc::c_char,
                current_field_sep_str(),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn reset_record() {
    let ref mut fresh10 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh10 = force_string_fmt(
        *fields_arr.offset(0 as libc::c_int as isize),
        CONVFMT,
        CONVFMTidx,
    );
    purge_record();
    if api_parser_override {
        api_parser_override = 0 as libc::c_int != 0;
        parse_field = normal_parse_field;
        update_PROCINFO_str(
            b"FS\0" as *const u8 as *const libc::c_char,
            current_field_sep_str(),
        );
    }
}
unsafe extern "C" fn purge_record() {
    let mut i: libc::c_int = 0;
    NF = -(1 as libc::c_int) as libc::c_long;
    i = 1 as libc::c_int;
    while i as libc::c_long <= parse_high_water {
        let mut n: *mut NODE = 0 as *mut NODE;
        let mut r: *mut NODE = *fields_arr.offset(i as isize);
        if (*r).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            && (*r).valref > 1 as libc::c_int as libc::c_long
        {
            let mut save: *const libc::c_char = (*r).sub.val.sp;
            (*r)
                .sub
                .val
                .sp = emalloc_real(
                ((*r).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"purge_record\0" as *const u8 as *const libc::c_char,
                b"r->stptr\0" as *const u8 as *const libc::c_char,
                b"field.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int,
            ) as *mut libc::c_char;
            memcpy(
                (*r).sub.val.sp as *mut libc::c_void,
                save as *const libc::c_void,
                (*r).sub.val.slen,
            );
            *((*r).sub.val.sp)
                .offset((*r).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
            (*r)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*r).flags as libc::c_uint | MALLOC as libc::c_int as libc::c_uint);
        }
        unref(r);
        n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
        if !n.is_null() {
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = (*(n as *mut block_item)).freep;
        } else {
            n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
        };
        *n = *Null_field;
        let ref mut fresh11 = *fields_arr.offset(i as isize);
        *fresh11 = n;
        i += 1;
        i;
    }
    parse_high_water = 0 as libc::c_int as libc::c_long;
    if resave_fs {
        resave_fs = 0 as libc::c_int != 0;
        unref(save_FS);
        save_FS = dupnode((*FS_node).sub.nodep.l.lptr);
    }
    field0_valid = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_NF() {
    let mut i: libc::c_int = 0;
    let mut nf: libc::c_long = 0;
    let mut n: *mut NODE = 0 as *mut NODE;
    force_number((*NF_node).sub.nodep.l.lptr);
    nf = (*(*NF_node).sub.nodep.l.lptr).sub.val.fltnum as libc::c_long;
    if nf < 0 as libc::c_int as libc::c_long {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
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
                b"NF set to negative value\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    static mut warned: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && NF > nf && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int,
        );
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"decrementing NF is not portable to many awk versions\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    NF = nf;
    if NF > nf_high_water {
        grow_fields_arr(NF);
    }
    if parse_high_water < NF {
        i = (parse_high_water + 1 as libc::c_int as libc::c_long) as libc::c_int;
        while i >= 0 as libc::c_int && i as libc::c_long <= NF {
            unref(*fields_arr.offset(i as isize));
            n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*(n as *mut block_item)).freep;
            } else {
                n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
            };
            *n = *Null_field;
            let ref mut fresh12 = *fields_arr.offset(i as isize);
            *fresh12 = n;
            i += 1;
            i;
        }
        parse_high_water = NF;
    } else if parse_high_water > 0 as libc::c_int as libc::c_long {
        i = (NF + 1 as libc::c_int as libc::c_long) as libc::c_int;
        while i >= 0 as libc::c_int && i as libc::c_long <= parse_high_water {
            unref(*fields_arr.offset(i as isize));
            n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*(n as *mut block_item)).freep;
            } else {
                n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
            };
            *n = *Null_field;
            let ref mut fresh13 = *fields_arr.offset(i as isize);
            *fresh13 = n;
            i += 1;
            i;
        }
        parse_high_water = NF;
    }
    field0_valid = 0 as libc::c_int != 0;
}
unsafe extern "C" fn re_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut regex_flags: libc::c_int = 1 as libc::c_int;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if in_middle {
        regex_flags |= 2 as libc::c_int;
    }
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    let mut default_field_splitting: bool = RS_is_null as libc::c_int != 0
        && default_FS != 0;
    if default_field_splitting {
        sep = scan;
        while scan < end
            && (*scan as libc::c_int == ' ' as i32 || *scan as libc::c_int == '\t' as i32
                || *scan as libc::c_int == '\n' as i32)
        {
            scan = scan.offset(1);
            scan;
        }
        if !sep_arr.is_null() && sep < scan {
            set_element(nf, sep, scan.offset_from(sep) as libc::c_long, sep_arr);
        }
    }
    if rp.is_null() {
        rp = FS_regexp;
    }
    field = scan;
    while scan < end
        && research(
            rp,
            scan,
            0 as libc::c_int,
            end.offset_from(scan) as libc::c_long as size_t,
            regex_flags,
        ) != -(1 as libc::c_int) && nf < up_to
    {
        regex_flags |= 2 as libc::c_int;
        if *((*rp).regs.end).offset(0 as libc::c_int as isize)
            == *((*rp).regs.start).offset(0 as libc::c_int as isize)
        {
            if gawk_mb_cur_max > 1 as libc::c_int {
                mbclen = mbrlen(
                    scan,
                    end.offset_from(scan) as libc::c_long as size_t,
                    &mut mbs,
                );
                if mbclen == 1 as libc::c_int as libc::c_ulong
                    || mbclen == -(1 as libc::c_int) as size_t
                    || mbclen == -(2 as libc::c_int) as size_t
                    || mbclen == 0 as libc::c_int as libc::c_ulong
                {
                    mbclen = 1 as libc::c_int as size_t;
                }
                scan = scan.offset(mbclen as isize);
            } else {
                scan = scan.offset(1);
                scan;
            }
            if !(scan == end) {
                continue;
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(nf, field, scan.offset_from(field) as libc::c_long, n);
            up_to = nf;
            break;
        } else {
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                nf,
                field,
                scan
                    .offset(
                        *((*rp).regs.start).offset(0 as libc::c_int as isize) as isize,
                    )
                    .offset_from(field) as libc::c_long,
                n,
            );
            if !sep_arr.is_null() {
                set_element(
                    nf,
                    scan
                        .offset(
                            *((*rp).regs.start).offset(0 as libc::c_int as isize)
                                as isize,
                        ),
                    (*((*rp).regs.end).offset(0 as libc::c_int as isize)
                        - *((*rp).regs.start).offset(0 as libc::c_int as isize))
                        as libc::c_long,
                    sep_arr,
                );
            }
            scan = scan
                .offset(*((*rp).regs.end).offset(0 as libc::c_int as isize) as isize);
            field = scan;
            if scan == end && !default_field_splitting {
                nf += 1;
                (Some(set.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(nf, field, 0 as libc::c_long, n);
            }
        }
    }
    if nf != up_to && scan < end {
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nf, scan, end.offset_from(scan) as libc::c_long, n);
        scan = end;
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn def_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut sav: libc::c_char = 0;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect("non-null function pointer")(nf, *buf, len as libc::c_long, n);
        *buf = (*buf).offset(len as isize);
        return nf;
    }
    sav = *end;
    *end = ' ' as i32 as libc::c_char;
    sep = scan;
    while nf < up_to {
        while scan < end
            && (*scan as libc::c_int == ' ' as i32 || *scan as libc::c_int == '\t' as i32
                || *scan as libc::c_int == '\n' as i32)
        {
            scan = scan.offset(1);
            scan;
        }
        if !sep_arr.is_null() && scan > sep {
            set_element(nf, sep, scan.offset_from(sep) as libc::c_long, sep_arr);
        }
        if scan >= end {
            break;
        }
        field = scan;
        while *scan as libc::c_int != ' ' as i32 && *scan as libc::c_int != '\t' as i32
            && *scan as libc::c_int != '\n' as i32
        {
            scan = scan.offset(1);
            scan;
        }
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nf, field, scan.offset_from(field) as libc::c_long, n);
        if scan == end {
            break;
        }
        sep = scan;
        scan = scan.offset(1);
        scan;
    }
    *end = sav;
    *buf = scan;
    return nf;
}
unsafe extern "C" fn null_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if gawk_mb_cur_max > 1 as libc::c_int {
        let mut mbs: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        while nf < up_to && scan < end {
            let mut mbclen: size_t = mbrlen(
                scan,
                end.offset_from(scan) as libc::c_long as size_t,
                &mut mbs,
            );
            if mbclen == 1 as libc::c_int as libc::c_ulong
                || mbclen == -(1 as libc::c_int) as size_t
                || mbclen == -(2 as libc::c_int) as size_t
                || mbclen == 0 as libc::c_int as libc::c_ulong
            {
                mbclen = 1 as libc::c_int as size_t;
            }
            if !sep_arr.is_null() && nf > 0 as libc::c_int as libc::c_long {
                set_element(nf, scan, 0 as libc::c_long, sep_arr);
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(nf, scan, mbclen as libc::c_long, n);
            scan = scan.offset(mbclen as isize);
        }
    } else {
        while nf < up_to && scan < end {
            if !sep_arr.is_null() && nf > 0 as libc::c_int as libc::c_long {
                set_element(nf, scan, 0 as libc::c_long, sep_arr);
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, 1 as libc::c_long, n);
            scan = scan.offset(1);
            scan;
        }
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn sc_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut fschar: libc::c_char = 0;
    let mut nf: libc::c_long = parse_high_water;
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut sav: libc::c_char = 0;
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if RS_is_null as libc::c_int != 0
        && (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong
    {
        fschar = '\n' as i32 as libc::c_char;
    } else {
        fschar = *((*fs).sub.val.sp).offset(0 as libc::c_int as isize);
    }
    sav = *end;
    *end = fschar;
    while nf < up_to {
        field = scan;
        if gawk_mb_cur_max > 1 as libc::c_int {
            while *scan as libc::c_int != fschar as libc::c_int {
                mbclen = mbrlen(
                    scan,
                    end.offset_from(scan) as libc::c_long as size_t,
                    &mut mbs,
                );
                if mbclen == 1 as libc::c_int as libc::c_ulong
                    || mbclen == -(1 as libc::c_int) as size_t
                    || mbclen == -(2 as libc::c_int) as size_t
                    || mbclen == 0 as libc::c_int as libc::c_ulong
                {
                    mbclen = 1 as libc::c_int as size_t;
                }
                scan = scan.offset(mbclen as isize);
            }
        } else {
            while *scan as libc::c_int != fschar as libc::c_int {
                scan = scan.offset(1);
                scan;
            }
        }
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(nf, field, scan.offset_from(field) as libc::c_long, n);
        if scan == end {
            break;
        }
        if !sep_arr.is_null() {
            set_element(nf, scan, 1 as libc::c_long, sep_arr);
        }
        scan = scan.offset(1);
        scan;
        if !(scan == end) {
            continue;
        }
        nf += 1;
        (Some(set.expect("non-null function pointer")))
            .expect("non-null function pointer")(nf, field, 0 as libc::c_long, n);
        break;
    }
    *end = sav;
    *buf = scan;
    return nf;
}
unsafe extern "C" fn calc_mbslen(
    mut scan: *mut libc::c_char,
    mut end: *mut libc::c_char,
    mut len: size_t,
    mut mbs: *mut mbstate_t,
) -> size_t {
    let mut mbclen: size_t = 0;
    let mut mbscan: *mut libc::c_char = scan;
    loop {
        let fresh14 = len;
        len = len.wrapping_sub(1);
        if !(fresh14 > 0 as libc::c_int as libc::c_ulong && mbscan < end) {
            break;
        }
        mbclen = mbrlen(mbscan, end.offset_from(mbscan) as libc::c_long as size_t, mbs);
        if !(mbclen > 0 as libc::c_int as libc::c_ulong
            && mbclen <= end.offset_from(mbscan) as libc::c_long as size_t)
        {
            mbclen = 1 as libc::c_int as size_t;
        }
        mbscan = mbscan.offset(mbclen as isize);
    }
    return mbscan.offset_from(scan) as libc::c_long as size_t;
}
unsafe extern "C" fn fw_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut dummy: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut fw: *const awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut skiplen: size_t = 0;
    let mut flen: size_t = 0;
    fw = if api_parser_override as libc::c_int != 0 { api_fw } else { FIELDWIDTHS };
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if gawk_mb_cur_max > 1 as libc::c_int && (*fw).use_chars as libc::c_uint != 0 {
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        while nf < up_to && scan < end {
            if nf as libc::c_ulong >= (*fw).nf {
                *buf = end;
                return nf;
            }
            scan = scan
                .offset(
                    calc_mbslen(
                        scan,
                        end,
                        (*((*fw).fields).as_ptr().offset(nf as isize)).skip,
                        &mut mbs,
                    ) as isize,
                );
            flen = calc_mbslen(
                scan,
                end,
                (*((*fw).fields).as_ptr().offset(nf as isize)).len,
                &mut mbs,
            );
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, flen as libc::c_long, n);
            scan = scan.offset(flen as isize);
        }
    } else {
        while nf < up_to && scan < end {
            if nf as libc::c_ulong >= (*fw).nf {
                *buf = end;
                return nf;
            }
            skiplen = (*((*fw).fields).as_ptr().offset(nf as isize)).skip;
            if skiplen > end.offset_from(scan) as libc::c_long as libc::c_ulong {
                skiplen = end.offset_from(scan) as libc::c_long as size_t;
            }
            scan = scan.offset(skiplen as isize);
            flen = (*((*fw).fields).as_ptr().offset(nf as isize)).len;
            if flen > end.offset_from(scan) as libc::c_long as libc::c_ulong {
                flen = end.offset_from(scan) as libc::c_long as size_t;
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect("non-null function pointer")(nf, scan, flen as libc::c_long, n);
            scan = scan.offset(flen as isize);
        }
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn invalidate_field0() {
    field0_valid = 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_field(
    mut requested: libc::c_long,
    mut assign: *mut Func_ptr,
) -> *mut *mut NODE {
    let mut in_middle: bool = 0 as libc::c_int != 0;
    static mut warned: bool = 0 as libc::c_int != 0;
    extern "C" {
        static mut currule: libc::c_int;
    }
    let mut saved_fs: *mut NODE = 0 as *mut NODE;
    let mut fs_regexp: *mut Regexp = 0 as *mut Regexp;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && currule == END as libc::c_int && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            861 as libc::c_int,
        );
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"accessing fields from an END rule may not be portable\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if requested == 0 as libc::c_int as libc::c_long {
        if !field0_valid {
            if NF == -(1 as libc::c_int) as libc::c_long {
                in_middle = parse_high_water != 0 as libc::c_int as libc::c_long;
                if current_field_sep() as libc::c_uint
                    == Using_FPAT as libc::c_int as libc::c_uint
                {
                    saved_fs = save_FPAT;
                    fs_regexp = FPAT_regexp;
                } else {
                    saved_fs = save_FS;
                    fs_regexp = FS_regexp;
                }
                NF = (Some(parse_field.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    9223372036854775807 as libc::c_long
                        - 1 as libc::c_int as libc::c_long,
                    &mut parse_extent,
                    ((**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen)
                        .wrapping_sub(
                            parse_extent
                                .offset_from(
                                    (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp,
                                ) as libc::c_long as libc::c_ulong,
                        ) as libc::c_int,
                    saved_fs,
                    fs_regexp,
                    Some(
                        set_field
                            as unsafe extern "C" fn(
                                libc::c_long,
                                *mut libc::c_char,
                                libc::c_long,
                                *mut NODE,
                            ) -> (),
                    ),
                    0 as *mut libc::c_void as *mut NODE,
                    0 as *mut libc::c_void as *mut NODE,
                    in_middle,
                );
                parse_high_water = NF;
            }
            rebuild_record();
        }
        if !assign.is_null() {
            *assign = Some(reset_record as unsafe extern "C" fn() -> ());
        }
        return &mut *fields_arr.offset(0 as libc::c_int as isize) as *mut *mut NODE;
    }
    if !assign.is_null() {
        *assign = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Func_ptr,
        >(
            Some(
                ::core::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(invalidate_field0),
            ),
        );
    }
    if requested <= parse_high_water {
        return &mut *fields_arr.offset(requested as isize) as *mut *mut NODE;
    }
    if NF == -(1 as libc::c_int) as libc::c_long {
        if parse_high_water == 0 as libc::c_int as libc::c_long {
            parse_extent = (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp;
        } else {
            in_middle = 1 as libc::c_int != 0;
        }
        parse_high_water = (Some(parse_field.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            requested,
            &mut parse_extent,
            ((**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen)
                .wrapping_sub(
                    parse_extent
                        .offset_from(
                            (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp,
                        ) as libc::c_long as libc::c_ulong,
                ) as libc::c_int,
            save_FS,
            0 as *mut Regexp,
            Some(
                set_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut libc::c_char,
                        libc::c_long,
                        *mut NODE,
                    ) -> (),
            ),
            0 as *mut libc::c_void as *mut NODE,
            0 as *mut libc::c_void as *mut NODE,
            in_middle,
        );
        if parse_extent
            == ((**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp)
                .offset(
                    (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen
                        as isize,
                )
        {
            NF = parse_high_water;
        }
        if requested
            == 9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long
        {
            requested = parse_high_water;
        }
    }
    if parse_high_water < requested {
        if !assign.is_null() {
            if requested > nf_high_water {
                grow_fields_arr(requested);
            }
            NF = requested;
            parse_high_water = requested;
        } else {
            return &mut Null_field
        }
    }
    return &mut *fields_arr.offset(requested as isize) as *mut *mut NODE;
}
unsafe extern "C" fn set_element(
    mut num: libc::c_long,
    mut s: *mut libc::c_char,
    mut len: libc::c_long,
    mut n: *mut NODE,
) {
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    it = make_str_node(s, len as size_t, 0 as libc::c_int);
    (*it)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*it).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
    sub = make_number.expect("non-null function pointer")(num as libc::c_double);
    assoc_set(n, sub, it);
}
#[no_mangle]
pub unsafe extern "C" fn do_split(mut nargs: libc::c_int) -> *mut NODE {
    let mut src: *mut NODE = 0 as *mut NODE;
    let mut arr: *mut NODE = 0 as *mut NODE;
    let mut sep: *mut NODE = 0 as *mut NODE;
    let mut fs: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sep_arr: *mut NODE = 0 as *mut NODE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parseit: Option::<
        unsafe extern "C" fn(
            libc::c_long,
            *mut *mut libc::c_char,
            libc::c_int,
            *mut NODE,
            *mut Regexp,
            Setfunc,
            *mut NODE,
            *mut NODE,
            bool,
        ) -> libc::c_long,
    > = None;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    check_args_min_max(
        nargs,
        b"split\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        4 as libc::c_int,
    );
    if nargs == 4 as libc::c_int {
        static mut warned: bool = 0 as libc::c_int != 0;
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
            || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                988 as libc::c_int,
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
                    b"split: fourth argument is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        sep_arr = POP_PARAM();
        if (*sep_arr).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                992 as libc::c_int,
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
                    b"split: fourth argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        check_symtab_functab(
            sep_arr,
            b"split\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot use %s as fourth argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if (do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint
            != 0
            || do_flags as libc::c_uint & DO_LINT_OLD as libc::c_int as libc::c_uint
                != 0) && !warned
        {
            warned = 1 as libc::c_int != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                997 as libc::c_int,
            );
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"split: fourth argument is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    let fresh15 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    sep = (*fresh15).rptr;
    arr = POP_PARAM();
    if (*arr).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1004 as libc::c_int,
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
                b"split: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    check_symtab_functab(
        arr,
        b"split\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: cannot use %s as second argument\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if !sep_arr.is_null() {
        if sep_arr == arr {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1010 as libc::c_int,
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
                    b"split: cannot use the same array for second and fourth args\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        tmp = (*sep_arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1015 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"split: cannot use a subarray of second arg for fourth arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        tmp = (*arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == sep_arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1018 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"split: cannot use a subarray of fourth arg for second arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        ((*(*sep_arr).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(sep_arr, 0 as *mut exp_node);
    }
    ((*(*arr).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(arr, 0 as *mut exp_node);
    src = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
    if (*src).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        tmp = POP_SCALAR();
        DEREF(tmp);
        return make_number
            .expect("non-null function pointer")(0 as libc::c_int as libc::c_double);
    }
    if (*sep).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        sep = (*sep).sub.val.typre;
    }
    if (*sep).sub.nodep.reflags as libc::c_uint & FS_DFLT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && current_field_sep() as libc::c_uint == Using_FS as libc::c_int as libc::c_uint
        && !RS_is_null
    {
        parseit = parse_field;
        fs = force_string_fmt((*FS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
        rp = FS_regexp;
    } else {
        fs = (*sep).sub.nodep.x.extra;
        if (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
            static mut warned_0: bool = 0 as libc::c_int != 0;
            parseit = Some(
                null_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            );
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0 && !warned_0
            {
                warned_0 = 1 as libc::c_int != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1052 as libc::c_int,
                );
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"split: null string for third arg is a non-standard extension\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong
            && (*sep).sub.nodep.reflags as libc::c_uint
                & CONSTANT as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32
            {
                parseit = Some(
                    def_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                );
            } else {
                parseit = Some(
                    sc_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                );
            }
        } else {
            parseit = Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            );
            rp = re_update(sep);
        }
    }
    s = (*src).sub.val.sp;
    tmp = make_number
        .expect(
            "non-null function pointer",
        )(
        (Some(parseit.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            9223372036854775807 as libc::c_long,
            &mut s,
            (*src).sub.val.slen as libc::c_int,
            fs,
            rp,
            Some(
                set_element
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut libc::c_char,
                        libc::c_long,
                        *mut NODE,
                    ) -> (),
            ),
            arr,
            sep_arr,
            0 as libc::c_int != 0,
        ) as libc::c_double,
    );
    src = POP_SCALAR();
    DEREF(src);
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn do_patsplit(mut nargs: libc::c_int) -> *mut NODE {
    let mut src: *mut NODE = 0 as *mut NODE;
    let mut arr: *mut NODE = 0 as *mut NODE;
    let mut sep: *mut NODE = 0 as *mut NODE;
    let mut fpat: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sep_arr: *mut NODE = 0 as *mut NODE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    check_args_min_max(
        nargs,
        b"patsplit\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        4 as libc::c_int,
    );
    if nargs == 4 as libc::c_int {
        sep_arr = POP_PARAM();
        if (*sep_arr).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1091 as libc::c_int,
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
                    b"patsplit: fourth argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        check_symtab_functab(
            sep_arr,
            b"patsplit\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot use %s as fourth argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let fresh16 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    sep = (*fresh16).rptr;
    arr = POP_PARAM();
    if (*arr).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1098 as libc::c_int,
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
                b"patsplit: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    check_symtab_functab(
        arr,
        b"patsplit\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: cannot use %s as second argument\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    src = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
    if (*sep).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        sep = (*sep).sub.val.typre;
    }
    fpat = (*sep).sub.nodep.x.extra;
    if (*fpat).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1109 as libc::c_int,
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
                b"patsplit: third argument must be non-null\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !sep_arr.is_null() {
        if sep_arr == arr {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1113 as libc::c_int,
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
                    b"patsplit: cannot use the same array for second and fourth args\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        tmp = (*sep_arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1118 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"patsplit: cannot use a subarray of second arg for fourth arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        tmp = (*arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == sep_arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1121 as libc::c_int,
                );
                (Some(
                    (Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"patsplit: cannot use a subarray of fourth arg for second arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        ((*(*sep_arr).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(sep_arr, 0 as *mut exp_node);
    }
    ((*(*arr).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(arr, 0 as *mut exp_node);
    if (*src).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        tmp = make_number
            .expect("non-null function pointer")(0 as libc::c_int as libc::c_double);
    } else {
        rp = re_update(sep);
        s = (*src).sub.val.sp;
        tmp = make_number
            .expect(
                "non-null function pointer",
            )(
            fpat_parse_field(
                9223372036854775807 as libc::c_long,
                &mut s,
                (*src).sub.val.slen as libc::c_int,
                fpat,
                rp,
                Some(
                    set_element
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut libc::c_char,
                            libc::c_long,
                            *mut NODE,
                        ) -> (),
                ),
                arr,
                sep_arr,
                0 as libc::c_int != 0,
            ) as libc::c_double,
        );
    }
    src = POP_SCALAR();
    DEREF(src);
    return tmp;
}
unsafe extern "C" fn set_parser(mut func: parse_field_func_t) {
    normal_parse_field = func;
    if !api_parser_override && parse_field != func {
        parse_field = func;
        update_PROCINFO_str(
            b"FS\0" as *const u8 as *const libc::c_char,
            current_field_sep_str(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_FIELDWIDTHS() {
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    static mut fw_alloc: libc::c_int = 4 as libc::c_int;
    static mut warned: bool = 0 as libc::c_int != 0;
    let mut fatal_error: bool = 0 as libc::c_int != 0;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
        && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1171 as libc::c_int,
        );
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"`FIELDWIDTHS' is a gawk extension\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        return;
    }
    if !fields_arr.is_null() {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
    }
    set_parser(
        Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        ),
    );
    tmp = force_string_fmt((*FIELDWIDTHS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    scan = (*tmp).sub.val.sp;
    if FIELDWIDTHS.is_null() {
        FIELDWIDTHS = emalloc_real(
            (::core::mem::size_of::<awk_fieldwidth_info_t>() as libc::c_ulong)
                .wrapping_add(
                    ((fw_alloc - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<awk_field_info>() as libc::c_ulong,
                        ),
                ),
            b"set_FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
            b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
            b"field.c\0" as *const u8 as *const libc::c_char,
            1188 as libc::c_int,
        ) as *mut awk_fieldwidth_info_t;
        (*FIELDWIDTHS).use_chars = awk_true;
    }
    (*FIELDWIDTHS).nf = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    loop {
        let mut tmp_0: libc::c_ulong = 0;
        if i >= fw_alloc {
            fw_alloc *= 2 as libc::c_int;
            FIELDWIDTHS = erealloc_real(
                FIELDWIDTHS as *mut libc::c_void,
                (::core::mem::size_of::<awk_fieldwidth_info_t>() as libc::c_ulong)
                    .wrapping_add(
                        ((fw_alloc - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<awk_field_info>() as libc::c_ulong,
                            ),
                    ),
                b"set_FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
                b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
                b"field.c\0" as *const u8 as *const libc::c_char,
                1196 as libc::c_int,
            ) as *mut awk_fieldwidth_info_t;
        }
        while is_blank(*scan as libc::c_int) != 0 {
            scan = scan.offset(1);
            scan;
        }
        if *scan as libc::c_int == '-' as i32 {
            fatal_error = 1 as libc::c_int != 0;
            break;
        } else {
            if *scan as libc::c_int == '\0' as i32 {
                break;
            }
            *__errno_location() = 0 as libc::c_int;
            tmp_0 = strtoul(scan, &mut end, 10 as libc::c_int);
            if *__errno_location() == 0 as libc::c_int
                && *end as libc::c_int == ':' as i32
                && ((0 as libc::c_int as libc::c_ulong) < tmp_0
                    && tmp_0
                        <= (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
            {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).skip = tmp_0;
                scan = end.offset(1 as libc::c_int as isize);
                if *scan as libc::c_int == '-' as i32
                    || is_blank(*scan as libc::c_int) != 0
                {
                    fatal_error = 1 as libc::c_int != 0;
                    break;
                } else {
                    tmp_0 = strtoul(scan, &mut end, 10 as libc::c_int);
                }
            } else {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize))
                    .skip = 0 as libc::c_int as size_t;
            }
            if *__errno_location() != 0 as libc::c_int
                || *end as libc::c_int != '\0' as i32
                    && is_blank(*end as libc::c_int) == 0
                || !((0 as libc::c_int as libc::c_ulong) < tmp_0
                    && tmp_0
                        <= (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
            {
                if *scan as libc::c_int == '*' as i32 {
                    scan = scan.offset(1);
                    scan;
                    while is_blank(*scan as libc::c_int) != 0 {
                        scan = scan.offset(1);
                        scan;
                    }
                    if *scan as libc::c_int != '\0' as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"field.c\0" as *const u8 as *const libc::c_char,
                            1240 as libc::c_int,
                        );
                        (Some(
                            (Some(
                                r_fatal
                                    as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                            ))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"`*' must be the last designator in FIELDWIDTHS\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize))
                        .len = (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) as size_t;
                    (*FIELDWIDTHS).nf = (i + 1 as libc::c_int) as size_t;
                } else {
                    fatal_error = 1 as libc::c_int != 0;
                }
                break;
            } else {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).len = tmp_0;
                (*FIELDWIDTHS).nf = (i + 1 as libc::c_int) as size_t;
                scan = end;
                while is_blank(*scan as libc::c_int) != 0 {
                    scan = scan.offset(1);
                    scan;
                }
                if *scan as libc::c_int == '\0' as i32 {
                    break;
                }
                i += 1;
                i;
            }
        }
    }
    if fatal_error {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1261 as libc::c_int,
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
                b"invalid FIELDWIDTHS value, for field %d, near `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            i + 1 as libc::c_int,
            scan,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_FS() {
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut fs: *mut NODE = 0 as *mut NODE;
    static mut save_fs: *mut NODE = 0 as *const NODE as *mut NODE;
    static mut save_rs: *mut NODE = 0 as *const NODE as *mut NODE;
    let mut remake_re: bool = 1 as libc::c_int != 0;
    if !fields_arr.is_null() {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
    }
    if !save_fs.is_null()
        && (*(*FS_node).sub.nodep.l.lptr).sub.val.slen == (*save_fs).sub.val.slen
        && memcmp(
            (*(*FS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_fs).sub.val.sp as *const libc::c_void,
            (*save_fs).sub.val.slen,
        ) == 0 as libc::c_int && !save_rs.is_null()
        && (*(*RS_node).sub.nodep.l.lptr).sub.val.slen == (*save_rs).sub.val.slen
        && memcmp(
            (*(*RS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.slen,
        ) == 0 as libc::c_int
    {
        if !FS_regexp.is_null() {
            FS_regexp = if IGNORECASE as libc::c_int != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
        }
        if current_field_sep() as libc::c_uint == Using_FS as libc::c_int as libc::c_uint
        {
            return
        } else {
            remake_re = 0 as libc::c_int != 0;
        }
    } else {
        unref(save_fs);
        save_fs = dupnode((*FS_node).sub.nodep.l.lptr);
        unref(save_rs);
        save_rs = dupnode((*RS_node).sub.nodep.l.lptr);
        resave_fs = 1 as libc::c_int != 0;
        refree(FS_re_yes_case);
        refree(FS_re_no_case);
        FS_regexp = 0 as *mut Regexp;
        FS_re_no_case = FS_regexp;
        FS_re_yes_case = FS_re_no_case;
    }
    buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    default_FS = 0 as libc::c_int;
    fs = force_string_fmt((*FS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
        && (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong
    {
        static mut warned: bool = 0 as libc::c_int != 0;
        set_parser(
            Some(
                null_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
        if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint
            != 0 && !warned
        {
            warned = 1 as libc::c_int != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1334 as libc::c_int,
            );
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"null string for `FS' is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    } else if (*fs).sub.val.slen > 1 as libc::c_int as libc::c_ulong
        || (*fs).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        if do_flags as libc::c_uint & DO_LINT_OLD as libc::c_int as libc::c_uint != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1338 as libc::c_int,
            );
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"old awk does not support regexps as value of `FS'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        set_parser(
            Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
    } else if RS_is_null {
        set_parser(
            Some(
                sc_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
        if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
            if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32
            {
                default_FS = 1 as libc::c_int;
                strcpy(
                    buf.as_mut_ptr(),
                    b"[ \t\n]+\0" as *const u8 as *const libc::c_char,
                );
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == '\\' as i32
            {
                strcpy(
                    buf.as_mut_ptr(),
                    b"[\\\\\n]\0" as *const u8 as *const libc::c_char,
                );
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32
            {
                strcpy(
                    buf.as_mut_ptr(),
                    b"[\\000\n]\0" as *const u8 as *const libc::c_char,
                );
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int != '\n' as i32
            {
                sprintf(
                    buf.as_mut_ptr(),
                    b"[%c\n]\0" as *const u8 as *const libc::c_char,
                    *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int,
                );
            }
        }
    } else {
        set_parser(
            Some(
                def_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
        if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
            if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32
            {
                default_FS = 1 as libc::c_int;
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == '\\' as i32
            {
                strcpy(
                    buf.as_mut_ptr(),
                    b"[\\\\]\0" as *const u8 as *const libc::c_char,
                );
            } else {
                set_parser(
                    Some(
                        sc_parse_field
                            as unsafe extern "C" fn(
                                libc::c_long,
                                *mut *mut libc::c_char,
                                libc::c_int,
                                *mut NODE,
                                *mut Regexp,
                                Setfunc,
                                *mut NODE,
                                *mut NODE,
                                bool,
                            ) -> libc::c_long,
                    ),
                );
            }
        }
    }
    if remake_re {
        refree(FS_re_yes_case);
        refree(FS_re_no_case);
        FS_regexp = 0 as *mut Regexp;
        FS_re_no_case = FS_regexp;
        FS_re_yes_case = FS_re_no_case;
        if buf[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
            FS_re_yes_case = make_regexp(
                buf.as_mut_ptr(),
                strlen(buf.as_mut_ptr()),
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_re_no_case = make_regexp(
                buf.as_mut_ptr(),
                strlen(buf.as_mut_ptr()),
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_regexp = if IGNORECASE as libc::c_int != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
            set_parser(
                Some(
                    re_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                ),
            );
        } else if parse_field
            == Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            )
        {
            FS_re_yes_case = make_regexp(
                (*fs).sub.val.sp,
                (*fs).sub.val.slen,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_re_no_case = make_regexp(
                (*fs).sub.val.sp,
                (*fs).sub.val.slen,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_regexp = if IGNORECASE as libc::c_int != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
        } else {
            FS_regexp = 0 as *mut Regexp;
            FS_re_no_case = FS_regexp;
            FS_re_yes_case = FS_re_no_case;
        }
    }
    if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong
        && parse_field
            == Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            )
    {
        FS_regexp = FS_re_yes_case;
    }
}
#[no_mangle]
pub unsafe extern "C" fn current_field_sep() -> field_sep_type {
    if api_parser_override {
        return Using_API
    } else if parse_field
        == Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return Using_FIELDWIDTHS
    } else if parse_field
        == Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return Using_FPAT
    } else {
        return Using_FS
    };
}
#[no_mangle]
pub unsafe extern "C" fn current_field_sep_str() -> *const libc::c_char {
    if api_parser_override {
        return b"API\0" as *const u8 as *const libc::c_char
    } else if parse_field
        == Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char
    } else if parse_field
        == Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return b"FPAT\0" as *const u8 as *const libc::c_char
    } else {
        return b"FS\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn update_PROCINFO_str(
    mut subscript: *const libc::c_char,
    mut str: *const libc::c_char,
) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if PROCINFO_node.is_null() {
        return;
    }
    tmp = make_str_node(subscript, strlen(subscript), 0 as libc::c_int);
    assoc_set(PROCINFO_node, tmp, make_str_node(str, strlen(str), 0 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn update_PROCINFO_num(
    mut subscript: *const libc::c_char,
    mut val: libc::c_double,
) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if PROCINFO_node.is_null() {
        return;
    }
    tmp = make_str_node(subscript, strlen(subscript), 0 as libc::c_int);
    assoc_set(PROCINFO_node, tmp, make_number.expect("non-null function pointer")(val));
}
#[no_mangle]
pub unsafe extern "C" fn set_FPAT() {
    static mut warned: bool = 0 as libc::c_int != 0;
    let mut remake_re: bool = 1 as libc::c_int != 0;
    let mut fpat: *mut NODE = 0 as *mut NODE;
    if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
        && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1464 as libc::c_int,
        );
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"`FPAT' is a gawk extension\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        return;
    }
    if !fields_arr.is_null() {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
    }
    if !save_FPAT.is_null()
        && (*(*FPAT_node).sub.nodep.l.lptr).sub.val.slen == (*save_FPAT).sub.val.slen
        && memcmp(
            (*(*FPAT_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_FPAT).sub.val.sp as *const libc::c_void,
            (*save_FPAT).sub.val.slen,
        ) == 0 as libc::c_int
    {
        if !FPAT_regexp.is_null() {
            FPAT_regexp = if IGNORECASE as libc::c_int != 0 {
                FPAT_re_no_case
            } else {
                FPAT_re_yes_case
            };
        }
        if current_field_sep() as libc::c_uint
            == Using_FPAT as libc::c_int as libc::c_uint
        {
            return
        } else {
            remake_re = 0 as libc::c_int != 0;
        }
    } else {
        unref(save_FPAT);
        save_FPAT = dupnode((*FPAT_node).sub.nodep.l.lptr);
        refree(FPAT_re_yes_case);
        refree(FPAT_re_no_case);
        FPAT_regexp = 0 as *mut Regexp;
        FPAT_re_no_case = FPAT_regexp;
        FPAT_re_yes_case = FPAT_re_no_case;
    }
    fpat = force_string_fmt((*FPAT_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    set_parser(
        Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        ),
    );
    if remake_re {
        refree(FPAT_re_yes_case);
        refree(FPAT_re_no_case);
        FPAT_regexp = 0 as *mut Regexp;
        FPAT_re_no_case = FPAT_regexp;
        FPAT_re_yes_case = FPAT_re_no_case;
        FPAT_re_yes_case = make_regexp(
            (*fpat).sub.val.sp,
            (*fpat).sub.val.slen,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        FPAT_re_no_case = make_regexp(
            (*fpat).sub.val.sp,
            (*fpat).sub.val.slen,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        FPAT_regexp = if IGNORECASE as libc::c_int != 0 {
            FPAT_re_no_case
        } else {
            FPAT_re_yes_case
        };
    }
}
unsafe extern "C" fn incr_scan(
    mut scanp: *mut *mut libc::c_char,
    mut len: size_t,
    mut mbs: *mut mbstate_t,
) {
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    if gawk_mb_cur_max > 1 as libc::c_int {
        mbclen = mbrlen(*scanp, len, mbs);
        if mbclen == 1 as libc::c_int as libc::c_ulong
            || mbclen == -(1 as libc::c_int) as size_t
            || mbclen == -(2 as libc::c_int) as size_t
            || mbclen == 0 as libc::c_int as libc::c_ulong
        {
            mbclen = 1 as libc::c_int as size_t;
        }
        *scanp = (*scanp).offset(mbclen as isize);
    } else {
        *scanp = (*scanp).offset(1);
        *scanp;
    };
}
unsafe extern "C" fn fpat_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut regex_flags: libc::c_int = 1 as libc::c_int;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut field_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut field_found: bool = 0 as libc::c_int != 0;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if rp.is_null() {
        rp = FPAT_regexp;
    }
    while scan < end && nf < up_to {
        start = scan;
        field_found = research(
            rp,
            scan,
            0 as libc::c_int,
            end.offset_from(scan) as libc::c_long as size_t,
            regex_flags,
        ) != -(1 as libc::c_int);
        if nf > 0 as libc::c_int as libc::c_long && field_found as libc::c_int != 0
            && *((*rp).regs.end).offset(0 as libc::c_int as isize) == 0 as libc::c_int
        {
            incr_scan(
                &mut scan,
                end.offset_from(scan) as libc::c_long as size_t,
                &mut mbs,
            );
            field_found = research(
                rp,
                scan,
                0 as libc::c_int,
                end.offset_from(scan) as libc::c_long as size_t,
                regex_flags,
            ) != -(1 as libc::c_int);
        }
        if field_found {
            field_start = scan
                .offset(*((*rp).regs.start).offset(0 as libc::c_int as isize) as isize);
            if !sep_arr.is_null() {
                if field_start == start {
                    set_element(nf, start, 0 as libc::c_long, sep_arr);
                } else {
                    set_element(
                        nf,
                        start,
                        field_start.offset_from(start) as libc::c_long,
                        sep_arr,
                    );
                }
            }
            nf += 1;
            (Some(set.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                nf,
                field_start,
                (*((*rp).regs.end).offset(0 as libc::c_int as isize)
                    - *((*rp).regs.start).offset(0 as libc::c_int as isize))
                    as libc::c_long,
                n,
            );
            scan = scan
                .offset(*((*rp).regs.end).offset(0 as libc::c_int as isize) as isize);
        } else {
            if !sep_arr.is_null() {
                set_element(nf, start, end.offset_from(start) as libc::c_long, sep_arr);
            }
            scan = end;
        }
    }
    if !sep_arr.is_null() && scan == end && field_found as libc::c_int != 0 {
        set_element(nf, scan, 0 as libc::c_long, sep_arr);
    }
    *buf = scan;
    return nf;
}
