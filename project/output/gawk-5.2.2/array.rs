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
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut IGNORECASE: bool;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut SUBSEP_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut Null_field: *mut NODE;
    static mut interpret: Option::<
        unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
    >;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut cmp_numbers: Option::<
        unsafe extern "C" fn(*const NODE, *const NODE) -> libc::c_int,
    >;
    static str_array_func: array_funcs_t;
    static cint_array_func: array_funcs_t;
    static int_array_func: array_funcs_t;
    static mut nextfree: [block_header; 2];
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: libc::c_int;
    static mut casetable: [libc::c_char; 0];
    static mut stack_ptr: *mut STACK_ITEM;
    static mut frame_ptr: *mut NODE;
    static mut stack_top: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn bcfree(_: *mut INSTRUCTION);
    fn POP_CODE() -> *mut INSTRUCTION;
    fn grow_stack() -> *mut STACK_ITEM;
    fn PUSH_CODE(cp: *mut INSTRUCTION);
    fn bcalloc(op: OPCODE, size: libc::c_int, srcline: libc::c_int) -> *mut INSTRUCTION;
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn strncasecmpmbs(
        _: *const libc::c_uchar,
        _: *const libc::c_uchar,
        _: size_t,
    ) -> libc::c_int;
    fn nodetype2str(type_0: NODETYPE) -> *const libc::c_char;
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    static mut func_table: *mut NODE;
    static mut symbol_table: *mut NODE;
    fn check_symtab_functab(
        dest: *mut NODE,
        fname: *const libc::c_char,
        msg: *const libc::c_char,
    );
    static mut output_fp: *mut FILE;
    static mut fmt_list: *mut *mut NODE;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
}  // end of enum

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
pub type INSTRUCTION = exp_instruction;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sort_context_t {
    ASORTI = 3,
    ASORT = 2,
    SORTED_IN = 1,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum assoc_kind_t {
    ADELETE = 256,
    ADESC = 128,
    AASC = 64,
    AVSTR = 32,
    AVNUM = 16,
    AISTR = 8,
    AINUM = 4,
    AVALUE = 2,
    AINDEX = 1,
    ANONE = 0,
}  // end of enum

pub type qsort_compfunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qsort_funcs {
    pub name: *const libc::c_char,
    pub comp_func: qsort_compfunc,
    pub kind: assoc_kind_t,
}
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as libc::c_int as libc::c_long {
        return;
    }
    r_unref(r);
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
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint
        & (NUMCUR as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
        == USER_INPUT as libc::c_int as libc::c_uint
    {
        return force_number(n);
    }
    if (*n).flags as libc::c_uint & INTIND as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
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
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
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
#[no_mangle]
pub static mut success_node: *mut NODE = 0 as *const NODE as *mut NODE;
static mut SUBSEPlen: size_t = 0;
static mut SUBSEP: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut indent_char: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"    \0")
};
static mut null_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"null\0" as *const u8 as *const libc::c_char,
            init: None,
            type_of: None,
            lookup: Some(
                null_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                null_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
static mut array_types: [*const array_funcs_t; 10] = [0 as *const array_funcs_t; 10];
static mut num_array_types: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn register_array_func(
    mut afunc: *const array_funcs_t,
) -> libc::c_int {
    if !afunc.is_null() && num_array_types < 10 as libc::c_int {
        if afunc != &str_array_func as *const array_funcs_t
            && ((*afunc).type_of).is_none()
        {
            return 0 as libc::c_int;
        }
        let fresh2 = num_array_types;
        num_array_types = num_array_types + 1;
        array_types[fresh2 as usize] = afunc;
        if ((*afunc).init).is_some() {
            (Some(((*afunc).init).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(0 as *mut exp_node, 0 as *mut exp_node);
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_init() {
    register_array_func(&str_array_func);
    if do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint == 0 {
        register_array_func(&int_array_func);
        register_array_func(&cint_array_func);
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_array() -> *mut NODE {
    let mut array: *mut NODE = 0 as *mut NODE;
    array = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !array.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(array as *mut block_item)).freep;
    } else {
        array = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        array as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as libc::c_ulong,
    );
    (*array).type_0 = Node_var_array;
    (*array).sub.nodep.l.lp = &null_array_func;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn null_array(mut symbol: *mut NODE) {
    (*symbol).type_0 = Node_var_array;
    (*symbol).sub.nodep.l.lp = &null_array_func;
    (*symbol).sub.nodep.r.bv = 0 as *mut *mut BUCKET;
    (*symbol).sub.nodep.reflags = 0 as reflagvals;
    (*symbol).sub.nodep.cnt = 0 as libc::c_int as libc::c_ulong;
    (*symbol).sub.nodep.reserved = 0 as libc::c_int as size_t;
    (*symbol).flags = 0 as flagvals;
}
unsafe extern "C" fn null_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: libc::c_int = 0;
    let mut afunc: *const array_funcs_t = 0 as *const array_funcs_t;
    i = num_array_types - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        afunc = array_types[i as usize];
        if !(((*afunc).type_of).expect("non-null function pointer")(symbol, subs))
            .is_null()
        {
            break;
        }
        i -= 1;
        i;
    }
    if i == 0 as libc::c_int || afunc.is_null() {
        afunc = array_types[0 as libc::c_int as usize];
    }
    (*symbol).sub.nodep.l.lp = afunc;
    return ((*(*symbol).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(symbol, subs);
}
#[no_mangle]
pub unsafe extern "C" fn null_afunc(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn null_dump(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    fprintf(
        output_fp,
        b"array `%s' is empty\n\0" as *const u8 as *const libc::c_char,
        array_vname(symbol),
    );
    return 0 as *mut *mut NODE;
}
#[no_mangle]
pub unsafe extern "C" fn assoc_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut NODE {
    ((*(*newsymb).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(newsymb, 0 as *mut exp_node);
    ((*(*symbol).sub.nodep.l.lp).copy)
        .expect("non-null function pointer")(symbol, newsymb);
    (*newsymb).sub.nodep.l.lp = (*symbol).sub.nodep.l.lp;
    (*newsymb).flags = (*symbol).flags;
    return newsymb;
}
#[no_mangle]
pub unsafe extern "C" fn assoc_dump(mut symbol: *mut NODE, mut ndump: *mut NODE) {
    if ((*(*symbol).sub.nodep.l.lp).dump).is_some() {
        ((*(*symbol).sub.nodep.l.lp).dump)
            .expect("non-null function pointer")(symbol, ndump);
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_aname(mut symbol: *const NODE) -> *const libc::c_char {
    static mut aname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut alen: size_t = 0;
    static mut max_alen: size_t = 0;
    if !((*symbol).sub.nodep.x.extra).is_null() {
        let mut slen: size_t = 0;
        make_aname((*symbol).sub.nodep.x.extra);
        slen = strlen((*symbol).sub.nodep.name);
        if alen.wrapping_add(slen).wrapping_add(4 as libc::c_int as libc::c_ulong)
            > max_alen
        {
            max_alen = alen
                .wrapping_add(slen)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(256 as libc::c_int as libc::c_ulong);
            aname = erealloc_real(
                aname as *mut libc::c_void,
                max_alen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
                b"make_aname\0" as *const u8 as *const libc::c_char,
                b"aname\0" as *const u8 as *const libc::c_char,
                b"array.c\0" as *const u8 as *const libc::c_char,
                213 as libc::c_int,
            ) as *mut libc::c_char;
        }
        alen = (alen as libc::c_ulong)
            .wrapping_add(
                sprintf(
                    aname.offset(alen as isize),
                    b"[\"%s\"]\0" as *const u8 as *const libc::c_char,
                    (*symbol).sub.nodep.name,
                ) as libc::c_ulong,
            ) as size_t as size_t;
    } else {
        alen = strlen((*symbol).sub.nodep.name);
        if aname.is_null() {
            max_alen = alen.wrapping_add(256 as libc::c_int as libc::c_ulong);
            aname = emalloc_real(
                max_alen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
                b"make_aname\0" as *const u8 as *const libc::c_char,
                b"aname\0" as *const u8 as *const libc::c_char,
                b"array.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int,
            ) as *mut libc::c_char;
        } else if alen > max_alen {
            max_alen = alen.wrapping_add(256 as libc::c_int as libc::c_ulong);
            aname = erealloc_real(
                aname as *mut libc::c_void,
                max_alen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
                b"make_aname\0" as *const u8 as *const libc::c_char,
                b"aname\0" as *const u8 as *const libc::c_char,
                b"array.c\0" as *const u8 as *const libc::c_char,
                223 as libc::c_int,
            ) as *mut libc::c_char;
        }
        memcpy(
            aname as *mut libc::c_void,
            (*symbol).sub.nodep.name as *const libc::c_void,
            alen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    return aname;
}
#[no_mangle]
pub unsafe extern "C" fn array_vname(mut symbol: *const NODE) -> *const libc::c_char {
    static mut message: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut msglen: size_t = 0 as libc::c_int as size_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut n: libc::c_int = 0;
    let mut save_symbol: *const NODE = symbol;
    let mut from: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"from %s\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut aname: *const libc::c_char = 0 as *const libc::c_char;
    if (*symbol).type_0 as libc::c_uint != Node_array_ref as libc::c_int as libc::c_uint
        || (*(*symbol).sub.nodep.l.lptr).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
    {
        if (*symbol).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
            || ((*symbol).sub.nodep.x.extra).is_null()
        {
            return (*symbol).sub.nodep.name;
        }
        return make_aname(symbol);
    }
    len = 2 as libc::c_int as size_t;
    n = 0 as libc::c_int;
    while (*symbol).type_0 as libc::c_uint
        == Node_array_ref as libc::c_int as libc::c_uint
    {
        len = (len as libc::c_ulong).wrapping_add(strlen((*symbol).sub.nodep.name))
            as size_t as size_t;
        n += 1;
        n;
        symbol = (*symbol).sub.nodep.r.rptr;
    }
    if ((*symbol).sub.nodep.x.extra).is_null() {
        aname = (*symbol).sub.nodep.name;
    } else {
        aname = make_aname(symbol);
    }
    len = (len as libc::c_ulong).wrapping_add(strlen(aname)) as size_t as size_t;
    len = (len as libc::c_ulong)
        .wrapping_add((n as libc::c_ulong).wrapping_mul(strlen(from))) as size_t
        as size_t;
    if message.is_null() {
        message = emalloc_real(
            len,
            b"array_vname\0" as *const u8 as *const libc::c_char,
            b"message\0" as *const u8 as *const libc::c_char,
            b"array.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
        ) as *mut libc::c_char;
        msglen = len;
    } else if len > msglen {
        message = erealloc_real(
            message as *mut libc::c_void,
            len,
            b"array_vname\0" as *const u8 as *const libc::c_char,
            b"message\0" as *const u8 as *const libc::c_char,
            b"array.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
        ) as *mut libc::c_char;
        msglen = len;
    }
    symbol = save_symbol;
    s = message;
    s = s
        .offset(
            sprintf(
                s,
                b"%s (\0" as *const u8 as *const libc::c_char,
                (*symbol).sub.nodep.name,
            ) as isize,
        );
    loop {
        symbol = (*symbol).sub.nodep.r.rptr;
        if (*symbol).type_0 as libc::c_uint
            != Node_array_ref as libc::c_int as libc::c_uint
        {
            break;
        }
        s = s.offset(sprintf(s, from, (*symbol).sub.nodep.name) as isize);
        s = s.offset(sprintf(s, b", \0" as *const u8 as *const libc::c_char) as isize);
    }
    s = s.offset(sprintf(s, from, aname) as isize);
    strcpy(s, b")\0" as *const u8 as *const libc::c_char);
    return message;
}
#[no_mangle]
pub unsafe extern "C" fn force_array(
    mut symbol: *mut NODE,
    mut canfatal: bool,
) -> *mut NODE {
    let mut save_symbol: *mut NODE = symbol;
    let mut isparam: bool = 0 as libc::c_int != 0;
    if (*symbol).type_0 as libc::c_uint == Node_param_list as libc::c_int as libc::c_uint
    {
        symbol = *((*frame_ptr).sub.nodep.r.av)
            .offset((*symbol).sub.nodep.l.ll as isize);
        save_symbol = symbol;
        isparam = 1 as libc::c_int != 0;
        if (*symbol).type_0 as libc::c_uint
            == Node_array_ref as libc::c_int as libc::c_uint
        {
            symbol = (*symbol).sub.nodep.l.lptr;
        }
    }
    let mut current_block_17: u64;
    match (*symbol).type_0 as libc::c_uint {
        7 => {
            pma_free((*symbol).sub.val.sp as *mut libc::c_void);
            (*symbol).sub.val.sp = 0 as *mut libc::c_char;
            (*symbol).sub.val.slen = 0 as libc::c_int as size_t;
            current_block_17 = 10124197035094333396;
        }
        6 => {
            current_block_17 = 10124197035094333396;
        }
        5 => {
            current_block_17 = 5689001924483802034;
        }
        12 | _ => {
            if canfatal {
                if (*symbol).type_0 as libc::c_uint
                    == Node_val as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"array.c\0" as *const u8 as *const libc::c_char,
                        355 as libc::c_int,
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
                            b"attempt to use a scalar value as array\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if isparam {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"array.c\0" as *const u8 as *const libc::c_char,
                        357 as libc::c_int,
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
                            b"attempt to use scalar parameter `%s' as an array\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*save_symbol).sub.nodep.name,
                    );
                } else {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"array.c\0" as *const u8 as *const libc::c_char,
                        360 as libc::c_int,
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
                            b"attempt to use scalar `%s' as an array\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*save_symbol).sub.nodep.name,
                    );
                }
                current_block_17 = 5689001924483802034;
            } else {
                current_block_17 = 5689001924483802034;
            }
        }
    }
    match current_block_17 {
        10124197035094333396 => {
            (*symbol).sub.nodep.rn = 0 as *mut exp_node;
            null_array(symbol);
            (*symbol).sub.nodep.x.extra = 0 as *mut exp_node;
        }
        _ => {}
    }
    return symbol;
}
#[no_mangle]
pub unsafe extern "C" fn set_SUBSEP() {
    (*SUBSEP_node)
        .sub
        .nodep
        .l
        .lptr = force_string_fmt((*SUBSEP_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    SUBSEP = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.sp;
    SUBSEPlen = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.slen;
}
#[no_mangle]
pub unsafe extern "C" fn concat_exp(
    mut nargs: libc::c_int,
    mut do_subsep: bool,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut subseplen: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    extern "C" {
        static mut args_array: *mut *mut NODE;
    }
    if nargs == 1 as libc::c_int {
        return force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    }
    if do_subsep {
        subseplen = SUBSEPlen;
    }
    len = 0 as libc::c_int as size_t;
    i = 1 as libc::c_int;
    while i <= nargs {
        r = (*stack_ptr).rptr;
        if (*r).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
            loop {
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
                DEREF(*args_array.offset(i as isize));
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"array.c\0" as *const u8 as *const libc::c_char,
                407 as libc::c_int,
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
                array_vname(r),
            );
        }
        r = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        let ref mut fresh3 = *args_array.offset(i as isize);
        *fresh3 = r;
        len = (len as libc::c_ulong).wrapping_add((*r).sub.val.slen) as size_t as size_t;
        i += 1;
        i;
    }
    len = (len as libc::c_ulong)
        .wrapping_add(
            ((nargs - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(subseplen),
        ) as size_t as size_t;
    str = emalloc_real(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"concat_exp\0" as *const u8 as *const libc::c_char,
        b"str\0" as *const u8 as *const libc::c_char,
        b"array.c\0" as *const u8 as *const libc::c_char,
        415 as libc::c_int,
    ) as *mut libc::c_char;
    r = *args_array.offset(nargs as isize);
    memcpy(
        str as *mut libc::c_void,
        (*r).sub.val.sp as *const libc::c_void,
        (*r).sub.val.slen,
    );
    s = str.offset((*r).sub.val.slen as isize);
    DEREF(r);
    i = nargs - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        if subseplen == 1 as libc::c_int as libc::c_ulong {
            let fresh4 = s;
            s = s.offset(1);
            *fresh4 = *SUBSEP;
        } else if subseplen > 0 as libc::c_int as libc::c_ulong {
            memcpy(s as *mut libc::c_void, SUBSEP as *const libc::c_void, subseplen);
            s = s.offset(subseplen as isize);
        }
        r = *args_array.offset(i as isize);
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.val.sp as *const libc::c_void,
            (*r).sub.val.slen,
        );
        s = s.offset((*r).sub.val.slen as isize);
        DEREF(r);
        i -= 1;
        i;
    }
    return make_str_node(str, len, 2 as libc::c_int);
}
unsafe extern "C" fn adjust_fcall_stack(mut symbol: *mut NODE, mut nsubs: libc::c_int) {
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut pcount: libc::c_int = 0;
    func = (*frame_ptr).sub.nodep.x.extra;
    if func.is_null() {
        return;
    }
    pcount = (*func).sub.nodep.l.ll as libc::c_int;
    sp = (*frame_ptr).sub.nodep.r.av;
    while pcount > 0 as libc::c_int {
        let fresh5 = sp;
        sp = sp.offset(1);
        r = *fresh5;
        if !((*r).type_0 as libc::c_uint != Node_array_ref as libc::c_int as libc::c_uint
            || (*(*r).sub.nodep.l.lptr).type_0 as libc::c_uint
                != Node_var_array as libc::c_int as libc::c_uint)
        {
            n = (*r).sub.nodep.l.lptr;
            if n == symbol && !((*symbol).sub.nodep.x.extra).is_null()
                && nsubs > 0 as libc::c_int
            {
                null_array(r);
                (*r).sub.nodep.x.extra = 0 as *mut exp_node;
            } else {
                n = (*n).sub.nodep.x.extra;
                while !n.is_null() {
                    if n == symbol {
                        null_array(r);
                        (*r).sub.nodep.x.extra = 0 as *mut exp_node;
                        break;
                    } else {
                        n = (*n).sub.nodep.x.extra;
                    }
                }
            }
        }
        pcount -= 1;
        pcount;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_delete(mut symbol: *mut NODE, mut nsubs: libc::c_int) {
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    val = 0 as *mut NODE;
    subs = val;
    if nsubs == 0 as libc::c_int {
        adjust_fcall_stack(symbol, 0 as libc::c_int);
        ((*(*symbol).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
        return;
    }
    i = nsubs;
    while i > 0 as libc::c_int {
        subs = (*stack_ptr.offset(-((i - 1 as libc::c_int) as isize))).rptr;
        if (*subs).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint {
            loop {
                let mut s: *mut NODE = (*stack_ptr
                    .offset(-((i - 1 as libc::c_int) as isize)))
                    .rptr;
                if (*s).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
                {
                    force_string_fmt(s, CONVFMT, CONVFMTidx);
                    DEREF(s);
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"array.c\0" as *const u8 as *const libc::c_char,
                574 as libc::c_int,
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
                array_vname(subs),
            );
        }
        val = in_array(symbol, subs);
        if val.is_null() {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"array.c\0" as *const u8 as *const libc::c_char,
                    581 as libc::c_int,
                );
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"delete: index `%.*s' not in array `%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*subs).sub.val.slen as libc::c_int,
                    (*subs).sub.val.sp,
                    array_vname(symbol),
                );
            }
            loop {
                let mut s_0: *mut NODE = (*stack_ptr
                    .offset(-((i - 1 as libc::c_int) as isize)))
                    .rptr;
                if (*s_0).type_0 as libc::c_uint
                    == Node_val as libc::c_int as libc::c_uint
                {
                    force_string_fmt(s_0, CONVFMT, CONVFMTidx);
                    DEREF(s_0);
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
            return;
        }
        if i > 1 as libc::c_int {
            if (*val).type_0 as libc::c_uint
                != Node_var_array as libc::c_int as libc::c_uint
            {
                loop {
                    let mut s_1: *mut NODE = (*stack_ptr
                        .offset(-((i - 1 as libc::c_int) as isize)))
                        .rptr;
                    if (*s_1).type_0 as libc::c_uint
                        == Node_val as libc::c_int as libc::c_uint
                    {
                        force_string_fmt(s_1, CONVFMT, CONVFMTidx);
                        DEREF(s_1);
                    }
                    i -= 1;
                    if !(i > 0 as libc::c_int) {
                        break;
                    }
                }
                subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"array.c\0" as *const u8 as *const libc::c_char,
                    595 as libc::c_int,
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
                        b"attempt to use scalar `%s[\"%.*s\"]' as an array\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    array_vname(symbol),
                    (*subs).sub.val.slen as libc::c_int,
                    (*subs).sub.val.sp,
                );
            }
            symbol = val;
            DEREF(subs);
        }
        i -= 1;
        i;
    }
    if (*val).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        adjust_fcall_stack(val, nsubs);
        ((*(*val).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(val, 0 as *mut exp_node);
        pma_free((*val).sub.nodep.name as *mut libc::c_void);
        let ref mut fresh6 = (*(val as *mut block_item)).freep;
        *fresh6 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = val as *mut block_item;
    } else {
        unref(val);
    }
    ((*(*symbol).sub.nodep.l.lp).remove)
        .expect("non-null function pointer")(symbol, subs);
    DEREF(subs);
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        null_array(symbol);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_delete_loop(mut symbol: *mut NODE, mut lhs: *mut *mut NODE) {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut akind: NODE = NODE {
        sub: C2RustUnnamed_1 {
            nodep: C2RustUnnamed_3 {
                l: C2RustUnnamed_10 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_5 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_4 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut libc::c_char,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: 0 as reflagvals,
            },
        },
        type_0: Node_illegal,
        flags: 0 as flagvals,
        valref: 0,
    };
    akind.flags = (AINDEX as libc::c_int | ADELETE as libc::c_int) as flagvals;
    list = ((*(*symbol).sub.nodep.l.lp).list)
        .expect("non-null function pointer")(symbol, &mut akind);
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return;
    }
    unref(*lhs);
    *lhs = *list.offset(0 as libc::c_int as isize);
    pma_free(list as *mut libc::c_void);
    adjust_fcall_stack(symbol, 0 as libc::c_int);
    ((*(*symbol).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
}
unsafe extern "C" fn value_info(mut n: *mut NODE) {
    if n == Nnull_string || n == Null_field {
        fprintf(output_fp, b"<(null)>\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*n).flags as libc::c_uint
        & (STRING as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        fprintf(output_fp, b"<\0" as *const u8 as *const libc::c_char);
        fprintf(
            output_fp,
            b"\"%.*s\"\0" as *const u8 as *const libc::c_char,
            (*n).sub.val.slen as libc::c_int,
            (*n).sub.val.sp,
        );
        if (*n).flags as libc::c_uint
            & (NUMBER as libc::c_int | NUMCUR as libc::c_int) as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            fprintf(
                output_fp,
                b":%.*g\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
                (*n).sub.val.fltnum,
            );
        }
        fprintf(output_fp, b">\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            output_fp,
            b"<%.*g>\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            (*n).sub.val.fltnum,
        );
    }
    fprintf(
        output_fp,
        b":%s\0" as *const u8 as *const libc::c_char,
        flags2str((*n).flags as libc::c_int),
    );
    if (*n).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        fprintf(output_fp, b":%ld\0" as *const u8 as *const libc::c_char, (*n).valref);
    } else {
        fprintf(output_fp, b":\0" as *const u8 as *const libc::c_char);
    }
    if (*n).flags as libc::c_uint
        & (STRING as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
        == STRCUR as libc::c_int as libc::c_uint
    {
        let mut len: size_t = 0;
        fprintf(output_fp, b"][\0" as *const u8 as *const libc::c_char);
        fprintf(
            output_fp,
            b"stfmt=%d, \0" as *const u8 as *const libc::c_char,
            (*n).sub.val.idx,
        );
        len = (**fmt_list.offset((*n).sub.val.idx as isize)).sub.val.slen;
        *((**fmt_list.offset((*n).sub.val.idx as isize)).sub.val.sp)
            .offset(len as isize) = '\0' as i32 as libc::c_char;
        fprintf(
            output_fp,
            b"FMT=\"%s\"\0" as *const u8 as *const libc::c_char,
            if (*n).sub.val.idx == -(1 as libc::c_int) {
                b"<unused>\0" as *const u8 as *const libc::c_char
            } else {
                (**fmt_list.offset((*n).sub.val.idx as isize)).sub.val.sp
            },
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn indent(mut indent_level: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < indent_level {
        fprintf(
            output_fp,
            b"%s\0" as *const u8 as *const libc::c_char,
            indent_char.as_mut_ptr(),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn assoc_info(
    mut subs: *mut NODE,
    mut val: *mut NODE,
    mut ndump: *mut NODE,
    mut aname: *const libc::c_char,
) {
    let mut indent_level: libc::c_int = (*ndump).sub.nodep.x.xl as libc::c_int;
    indent_level += 1;
    indent_level;
    indent(indent_level);
    fprintf(output_fp, b"I: [%s:\0" as *const u8 as *const libc::c_char, aname);
    if (*subs).flags as libc::c_uint
        & (MPFN as libc::c_int | MPZN as libc::c_int | INTIND as libc::c_int)
            as libc::c_uint == INTIND as libc::c_int as libc::c_uint
    {
        fprintf(
            output_fp,
            b"<%ld>\0" as *const u8 as *const libc::c_char,
            (*subs).sub.val.fltnum as libc::c_long,
        );
    } else {
        value_info(subs);
    }
    fprintf(output_fp, b"]\n\0" as *const u8 as *const libc::c_char);
    indent(indent_level);
    match (*val).type_0 as libc::c_uint {
        1 => {
            fprintf(output_fp, b"V: [scalar: \0" as *const u8 as *const libc::c_char);
            value_info(val);
        }
        4 => {
            fprintf(output_fp, b"V: [scalar: \0" as *const u8 as *const libc::c_char);
            value_info((*val).sub.nodep.l.lptr);
        }
        5 => {
            fprintf(output_fp, b"V: [\0" as *const u8 as *const libc::c_char);
            (*ndump).sub.nodep.x.xl += 1;
            (*ndump).sub.nodep.x.xl;
            (*ndump).sub.nodep.l.ll -= 1;
            (*ndump).sub.nodep.l.ll;
            assoc_dump(val, ndump);
            (*ndump).sub.nodep.l.ll += 1;
            (*ndump).sub.nodep.l.ll;
            (*ndump).sub.nodep.x.xl -= 1;
            (*ndump).sub.nodep.x.xl;
            indent(indent_level);
        }
        9 => {
            fprintf(
                output_fp,
                b"V: [user_defined_function\0" as *const u8 as *const libc::c_char,
            );
        }
        10 => {
            fprintf(
                output_fp,
                b"V: [external_function\0" as *const u8 as *const libc::c_char,
            );
        }
        11 => {
            fprintf(
                output_fp,
                b"V: [builtin_function\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            r_fatal(
                b"internal error: file %s, line %d: unexpected node type %s\0"
                    as *const u8 as *const libc::c_char,
                b"array.c\0" as *const u8 as *const libc::c_char,
                779 as libc::c_int,
                nodetype2str((*val).type_0),
            );
        }
    }
    fprintf(output_fp, b"]\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn do_adump(mut nargs: libc::c_int) -> *mut NODE {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    static mut ndump: NODE = NODE {
        sub: C2RustUnnamed_1 {
            nodep: C2RustUnnamed_3 {
                l: C2RustUnnamed_10 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_5 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_4 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut libc::c_char,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: 0 as reflagvals,
            },
        },
        type_0: Node_illegal,
        flags: 0 as flagvals,
        valref: 0,
    };
    let mut depth: libc::c_long = 0 as libc::c_int as libc::c_long;
    if nargs == 2 as libc::c_int {
        tmp = force_number(POP_SCALAR());
        depth = (*tmp).sub.val.fltnum as libc::c_long;
        DEREF(tmp);
    }
    symbol = POP_PARAM();
    if (*symbol).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"array.c\0" as *const u8 as *const libc::c_char,
            808 as libc::c_int,
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
                b"%s: first argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"adump\0" as *const u8 as *const libc::c_char,
        );
    }
    ndump.type_0 = Node_dump_array;
    ndump.sub.nodep.l.ll = depth;
    ndump.sub.nodep.x.xl = 0 as libc::c_int as libc::c_long;
    assoc_dump(symbol, &mut ndump);
    return make_number
        .expect("non-null function pointer")(0 as libc::c_int as libc::c_double);
}
unsafe extern "C" fn asort_actual(
    mut nargs: libc::c_int,
    mut ctxt: sort_context_t,
) -> *mut NODE {
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut dest: *mut NODE = 0 as *mut NODE;
    let mut result: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut s: *mut NODE = 0 as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut ptr: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut num_elems: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut sort_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut save: libc::c_char = 0;
    let mut name: *const libc::c_char = if ctxt as libc::c_uint
        == ASORT as libc::c_int as libc::c_uint
    {
        b"asort\0" as *const u8 as *const libc::c_char
    } else {
        b"asorti\0" as *const u8 as *const libc::c_char
    };
    if nargs == 3 as libc::c_int {
        s = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    } else {
        s = dupnode(Nnull_string);
    }
    s = force_string_fmt(s, CONVFMT, CONVFMTidx);
    sort_str = (*s).sub.val.sp;
    save = *((*s).sub.val.sp).offset((*s).sub.val.slen as isize);
    *((*s).sub.val.sp).offset((*s).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
    if (*s).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        if ctxt as libc::c_uint == ASORT as libc::c_int as libc::c_uint {
            sort_str = b"@val_type_asc\0" as *const u8 as *const libc::c_char;
        } else {
            sort_str = b"@ind_str_asc\0" as *const u8 as *const libc::c_char;
        }
    }
    if nargs >= 2 as libc::c_int {
        dest = POP_PARAM();
        if (*dest).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"array.c\0" as *const u8 as *const libc::c_char,
                850 as libc::c_int,
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
                    b"%s: second argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        check_symtab_functab(
            dest,
            name,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot use %s as second argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    array = POP_PARAM();
    if (*array).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"array.c\0" as *const u8 as *const libc::c_char,
            858 as libc::c_int,
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
                b"%s: first argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    } else if array == symbol_table && dest.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"array.c\0" as *const u8 as *const libc::c_char,
            861 as libc::c_int,
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
                b"%s: first argument cannot be SYMTAB without a second argument\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    } else if array == func_table && dest.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"array.c\0" as *const u8 as *const libc::c_char,
            863 as libc::c_int,
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
                b"%s: first argument cannot be FUNCTAB without a second argument\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    if !dest.is_null() {
        static mut warned: bool = 0 as libc::c_int != 0;
        if nargs == 2 as libc::c_int && array == dest && !warned {
            warned = 1 as libc::c_int != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"array.c\0" as *const u8 as *const libc::c_char,
                870 as libc::c_int,
            );
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"asort/asorti: using the same array as source and destination without a third argument is silly.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        r = (*dest).sub.nodep.x.extra;
        while !r.is_null() {
            if r == array {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"array.c\0" as *const u8 as *const libc::c_char,
                    875 as libc::c_int,
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
                        b"%s: cannot use a subarray of first argument for second argument\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
            }
            r = (*r).sub.nodep.x.extra;
        }
        r = (*array).sub.nodep.x.extra;
        while !r.is_null() {
            if r == dest {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"array.c\0" as *const u8 as *const libc::c_char,
                    880 as libc::c_int,
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
                        b"%s: cannot use a subarray of second argument for first argument\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
            }
            r = (*r).sub.nodep.x.extra;
        }
    }
    list = assoc_list(array, sort_str, ctxt);
    *((*s).sub.val.sp).offset((*s).sub.val.slen as isize) = save;
    DEREF(s);
    num_elems = (*array).sub.nodep.reflags as libc::c_ulong;
    if num_elems == 0 as libc::c_int as libc::c_ulong || list.is_null() {
        if !dest.is_null() && dest != array {
            ((*(*dest).sub.nodep.l.lp).clear)
                .expect("non-null function pointer")(dest, 0 as *mut exp_node);
        }
        if !list.is_null() {
            pma_free(list as *mut libc::c_void);
        }
        return make_number
            .expect("non-null function pointer")(0 as libc::c_int as libc::c_double);
    }
    if !dest.is_null() && dest != array {
        ((*(*dest).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(dest, 0 as *mut exp_node);
        result = dest;
    } else {
        result = make_array();
        (*result).sub.nodep.name = (*array).sub.nodep.name;
        (*result).sub.nodep.x.extra = (*array).sub.nodep.x.extra;
    }
    if ctxt as libc::c_uint == ASORTI as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as libc::c_ulong;
        ptr = list;
        while i <= num_elems {
            subs = make_number.expect("non-null function pointer")(i as libc::c_double);
            assoc_set(result, subs, *ptr);
            i = i.wrapping_add(1);
            i;
            ptr = ptr.offset(2 as libc::c_int as isize);
        }
    } else {
        i = 1 as libc::c_int as libc::c_ulong;
        ptr = list;
        while i <= num_elems {
            subs = make_number.expect("non-null function pointer")(i as libc::c_double);
            let fresh7 = ptr;
            ptr = ptr.offset(1);
            r = *fresh7;
            unref(r);
            let fresh8 = ptr;
            ptr = ptr.offset(1);
            r = *fresh8;
            let mut value: *mut NODE = 0 as *mut NODE;
            match (*r).type_0 as libc::c_uint {
                1 => {
                    value = dupnode(r);
                }
                4 => {
                    value = dupnode((*r).sub.nodep.l.lptr);
                }
                6 | 7 => {
                    value = dupnode(Nnull_string);
                }
                11 | 9 | 10 => {
                    value = make_str_node(
                        (*r).sub.nodep.name,
                        strlen((*r).sub.nodep.name),
                        0 as libc::c_int,
                    );
                }
                5 => {
                    let mut arr: *mut NODE = 0 as *mut NODE;
                    arr = make_array();
                    subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
                    (*arr).sub.nodep.name = (*subs).sub.val.sp;
                    *((*arr).sub.nodep.name)
                        .offset(
                            (*subs).sub.val.slen as isize,
                        ) = '\0' as i32 as libc::c_char;
                    (*subs).sub.val.sp = 0 as *mut libc::c_char;
                    (*subs)
                        .flags = ::core::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*subs).flags as libc::c_uint
                            & !(STRCUR as libc::c_int) as libc::c_uint,
                    );
                    (*arr).sub.nodep.x.extra = array;
                    value = assoc_copy(r, arr);
                }
                _ => {
                    r_fatal(
                        b"internal error: file %s, line %d: asort_actual: got unexpected type %s\0"
                            as *const u8 as *const libc::c_char,
                        b"array.c\0" as *const u8 as *const libc::c_char,
                        972 as libc::c_int,
                        nodetype2str((*r).type_0),
                    );
                }
            }
            assoc_set(result, subs, value);
            i = i.wrapping_add(1);
            i;
        }
    }
    pma_free(list as *mut libc::c_void);
    if result != dest {
        ((*(*array).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(array, 0 as *mut exp_node);
        *array = *result;
        let ref mut fresh9 = (*(result as *mut block_item)).freep;
        *fresh9 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = result as *mut block_item;
    }
    return make_number.expect("non-null function pointer")(num_elems as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_asort(mut nargs: libc::c_int) -> *mut NODE {
    return asort_actual(nargs, ASORT);
}
#[no_mangle]
pub unsafe extern "C" fn do_asorti(mut nargs: libc::c_int) -> *mut NODE {
    return asort_actual(nargs, ASORTI);
}
unsafe extern "C" fn cmp_strings(
    mut n1: *const NODE,
    mut n2: *const NODE,
) -> libc::c_int {
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut ret: libc::c_int = 0;
    s1 = (*n1).sub.val.sp;
    len1 = (*n1).sub.val.slen;
    s2 = (*n2).sub.val.sp;
    len2 = (*n2).sub.val.slen;
    if len1 == 0 as libc::c_int as libc::c_ulong {
        return if len2 == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if len2 == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    let lmin: size_t = if len1 < len2 { len1 } else { len2 };
    if IGNORECASE {
        let mut cp1: *const libc::c_uchar = s1 as *const libc::c_uchar;
        let mut cp2: *const libc::c_uchar = s2 as *const libc::c_uchar;
        if gawk_mb_cur_max > 1 as libc::c_int {
            ret = strncasecmpmbs(cp1, cp2, lmin);
        } else {
            let mut count: size_t = lmin;
            ret = 0 as libc::c_int;
            loop {
                let fresh10 = count;
                count = count.wrapping_sub(1);
                if !(fresh10 > 0 as libc::c_int as libc::c_ulong
                    && ret == 0 as libc::c_int)
                {
                    break;
                }
                ret = *casetable.as_mut_ptr().offset(*cp1 as isize) as libc::c_int
                    - *casetable.as_mut_ptr().offset(*cp2 as isize) as libc::c_int;
                cp1 = cp1.offset(1);
                cp1;
                cp2 = cp2.offset(1);
                cp2;
            }
        }
        if ret != 0 as libc::c_int {
            return ret;
        }
    }
    ret = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, lmin);
    if ret != 0 as libc::c_int || len1 == len2 {
        return ret;
    }
    return if len1 < len2 { -(1 as libc::c_int) } else { 1 as libc::c_int };
}
unsafe extern "C" fn sort_up_index_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut t1: *const NODE = 0 as *const NODE;
    let mut t2: *const NODE = 0 as *const NODE;
    t1 = *(p1 as *const *const NODE);
    t2 = *(p2 as *const *const NODE);
    return cmp_strings(t1, t2);
}
unsafe extern "C" fn sort_down_index_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return -sort_up_index_string(p1, p2);
}
unsafe extern "C" fn sort_up_index_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut t1: *const NODE = 0 as *const NODE;
    let mut t2: *const NODE = 0 as *const NODE;
    let mut ret: libc::c_int = 0;
    t1 = *(p1 as *const *const NODE);
    t2 = *(p2 as *const *const NODE);
    ret = cmp_numbers.expect("non-null function pointer")(t1, t2);
    if ret != 0 as libc::c_int {
        return ret;
    }
    t1 = force_string_fmt(t1 as *mut NODE, CONVFMT, CONVFMTidx);
    t2 = force_string_fmt(t2 as *mut NODE, CONVFMT, CONVFMTidx);
    return cmp_strings(t1, t2);
}
unsafe extern "C" fn sort_down_index_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return -sort_up_index_number(p1, p2);
}
unsafe extern "C" fn sort_up_value_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut t1: *const NODE = 0 as *const NODE;
    let mut t2: *const NODE = 0 as *const NODE;
    let mut ret: libc::c_int = 0;
    t1 = *(p1 as *const *const NODE).offset(1 as libc::c_int as isize);
    t2 = *(p2 as *const *const NODE).offset(1 as libc::c_int as isize);
    if (*t1).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
        || (*t2).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
    {
        return sort_up_value_type(p1, p2);
    }
    ret = cmp_strings(t1, t2);
    if ret != 0 as libc::c_int {
        return ret;
    }
    return sort_up_index_string(p1, p2);
}
unsafe extern "C" fn sort_down_value_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return -sort_up_value_string(p1, p2);
}
unsafe extern "C" fn sort_up_value_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_int = 0;
    t1 = *(p1 as *const *mut NODE).offset(1 as libc::c_int as isize);
    t2 = *(p2 as *const *mut NODE).offset(1 as libc::c_int as isize);
    if (*t1).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
        || (*t2).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
    {
        return sort_up_value_type(p1, p2);
    }
    ret = cmp_numbers.expect("non-null function pointer")(t1, t2);
    if ret != 0 as libc::c_int {
        return ret;
    }
    ret = cmp_strings(
        force_string_fmt(t1, CONVFMT, CONVFMTidx),
        force_string_fmt(t2, CONVFMT, CONVFMTidx),
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    return sort_up_index_string(p1, p2);
}
unsafe extern "C" fn sort_down_value_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return -sort_up_value_number(p1, p2);
}
unsafe extern "C" fn do_sort_up_value_type(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut n1: *mut NODE = 0 as *mut NODE;
    let mut n2: *mut NODE = 0 as *mut NODE;
    static mut element_types: [NODETYPE; 9] = [
        Node_builtin_func,
        Node_func,
        Node_ext_func,
        Node_var_new,
        Node_elem_new,
        Node_var,
        Node_var_array,
        Node_val,
        Node_illegal,
    ];
    n1 = *(p1 as *const *mut NODE).offset(1 as libc::c_int as isize);
    n2 = *(p2 as *const *mut NODE).offset(1 as libc::c_int as isize);
    if (*n1).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
        && (*n2).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
    {
        n1 = (*n1).sub.nodep.l.lptr;
        n2 = (*n2).sub.nodep.l.lptr;
    }
    if (*n1).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        return ((*n2).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint) as libc::c_int;
    }
    if (*n2).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    if (*n1).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
        || (*n2).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint
    {
        let mut n1_pos: libc::c_int = 0;
        let mut n2_pos: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        n2_pos = -(1 as libc::c_int);
        n1_pos = n2_pos;
        i = 0 as libc::c_int;
        while element_types[i as usize] as libc::c_uint
            != Node_illegal as libc::c_int as libc::c_uint
        {
            if (*n1).type_0 as libc::c_uint == element_types[i as usize] as libc::c_uint
            {
                n1_pos = i;
            }
            if (*n2).type_0 as libc::c_uint == element_types[i as usize] as libc::c_uint
            {
                n2_pos = i;
            }
            i += 1;
            i;
        }
        return n1_pos - n2_pos;
    }
    fixtype(n1);
    fixtype(n2);
    if (*n1).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (*n2).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return cmp_numbers.expect("non-null function pointer")(n1, n2);
    }
    if (*n1).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (*n2).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int)
    } else if (*n1).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (*n2).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int
    }
    return cmp_strings(n1, n2);
}
unsafe extern "C" fn sort_up_value_type(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut rc: libc::c_int = do_sort_up_value_type(p1, p2);
    return if rc != 0 { rc } else { sort_up_index_string(p1, p2) };
}
unsafe extern "C" fn sort_down_value_type(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    return -sort_up_value_type(p1, p2);
}
unsafe extern "C" fn sort_user_func(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut idx1: *mut NODE = 0 as *mut NODE;
    let mut idx2: *mut NODE = 0 as *mut NODE;
    let mut val1: *mut NODE = 0 as *mut NODE;
    let mut val2: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_int = 0;
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    idx1 = *(p1 as *const *mut NODE);
    idx2 = *(p2 as *const *mut NODE);
    val1 = *(p1 as *const *mut NODE).offset(1 as libc::c_int as isize);
    val2 = *(p2 as *const *mut NODE).offset(1 as libc::c_int as isize);
    code = (*(*stack_ptr).rptr).sub.nodep.r.iptr;
    (*idx1).valref += 1;
    (*idx1).valref;
    let ref mut fresh11 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh11 = idx1;
    if (*val1).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint {
        (*val1).valref += 1;
        (*val1).valref;
    }
    let ref mut fresh12 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh12 = val1;
    (*idx2).valref += 1;
    (*idx2).valref;
    let ref mut fresh13 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh13 = idx2;
    if (*val2).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint {
        (*val2).valref += 1;
        (*val2).valref;
    }
    let ref mut fresh14 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh14 = val2;
    (Some(interpret.expect("non-null function pointer")))
        .expect("non-null function pointer")(code);
    r = force_number(POP_SCALAR());
    ret = if (*r).sub.val.fltnum < 0.0f64 {
        -(1 as libc::c_int)
    } else {
        ((*r).sub.val.fltnum > 0.0f64) as libc::c_int
    };
    DEREF(r);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn assoc_list(
    mut symbol: *mut NODE,
    mut sort_str: *const libc::c_char,
    mut sort_ctxt: sort_context_t,
) -> *mut *mut NODE {
    static mut sort_funcs: [qsort_funcs; 11] = unsafe {
        [
            {
                let mut init = qsort_funcs {
                    name: b"@ind_str_asc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_up_index_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AINDEX as libc::c_int | AISTR as libc::c_int
                        | AASC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@ind_num_asc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_up_index_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AINDEX as libc::c_int | AINUM as libc::c_int
                        | AASC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_str_asc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_up_value_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AVALUE as libc::c_int | AVSTR as libc::c_int
                        | AASC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_num_asc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_up_value_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AVALUE as libc::c_int | AVNUM as libc::c_int
                        | AASC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@ind_str_desc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_down_index_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AINDEX as libc::c_int | AISTR as libc::c_int
                        | ADESC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@ind_num_desc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_down_index_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AINDEX as libc::c_int | AINUM as libc::c_int
                        | ADESC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_str_desc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_down_value_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AVALUE as libc::c_int | AVSTR as libc::c_int
                        | ADESC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_num_desc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_down_value_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AVALUE as libc::c_int | AVNUM as libc::c_int
                        | ADESC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_type_asc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_up_value_type
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AVALUE as libc::c_int | AASC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_type_desc\0" as *const u8 as *const libc::c_char,
                    comp_func: Some(
                        sort_down_value_type
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                    kind: (AVALUE as libc::c_int | ADESC as libc::c_int) as assoc_kind_t,
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@unsorted\0" as *const u8 as *const libc::c_char,
                    comp_func: None,
                    kind: AINDEX,
                };
                init
            },
        ]
    };
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut akind: NODE = NODE {
        sub: C2RustUnnamed_1 {
            nodep: C2RustUnnamed_3 {
                l: C2RustUnnamed_10 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_5 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_4 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut libc::c_char,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: 0 as reflagvals,
            },
        },
        type_0: Node_illegal,
        flags: 0 as flagvals,
        valref: 0,
    };
    let mut num_elems: libc::c_ulong = 0;
    let mut j: libc::c_ulong = 0;
    let mut elem_size: libc::c_int = 0;
    let mut qi: libc::c_int = 0;
    let mut cmp_func: qsort_compfunc = None;
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    extern "C" {
        static mut currule: libc::c_int;
    }
    let mut save_rule: libc::c_int = 0 as libc::c_int;
    let mut assoc_kind: assoc_kind_t = ANONE;
    elem_size = 1 as libc::c_int;
    qi = 0 as libc::c_int;
    j = (::core::mem::size_of::<[qsort_funcs; 11]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<qsort_funcs>() as libc::c_ulong);
    while (qi as libc::c_ulong) < j {
        if strcmp(sort_funcs[qi as usize].name, sort_str) == 0 as libc::c_int {
            break;
        }
        qi += 1;
        qi;
    }
    if (qi as libc::c_ulong) < j {
        cmp_func = sort_funcs[qi as usize].comp_func;
        assoc_kind = sort_funcs[qi as usize].kind;
        if (*symbol).sub.nodep.l.lp != &cint_array_func as *const array_funcs_t {
            assoc_kind = ::core::mem::transmute::<
                libc::c_uint,
                assoc_kind_t,
            >(
                assoc_kind as libc::c_uint
                    & !(AASC as libc::c_int | ADESC as libc::c_int) as libc::c_uint,
            );
        }
        if sort_ctxt as libc::c_uint != SORTED_IN as libc::c_int as libc::c_uint
            || assoc_kind as libc::c_uint & AVALUE as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            assoc_kind = ::core::mem::transmute::<
                libc::c_uint,
                assoc_kind_t,
            >(
                assoc_kind as libc::c_uint
                    | (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint,
            );
            elem_size = 2 as libc::c_int;
        }
    } else {
        let mut f: *mut NODE = 0 as *mut NODE;
        let mut sp: *const libc::c_char = 0 as *const libc::c_char;
        sp = sort_str;
        while *sp as libc::c_int != '\0' as i32
            && *(*__ctype_b_loc()).offset(*sp as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            sp = sp.offset(1);
            sp;
        }
        if sp == sort_str || *sp as libc::c_int != '\0' as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"array.c\0" as *const u8 as *const libc::c_char,
                1410 as libc::c_int,
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
                    b"`%s' is invalid as a function name\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                sort_str,
            );
        }
        f = lookup(sort_str);
        if f.is_null()
            || (*f).type_0 as libc::c_uint != Node_func as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"array.c\0" as *const u8 as *const libc::c_char,
                1414 as libc::c_int,
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
                    b"sort comparison function `%s' is not defined\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                sort_str,
            );
        }
        cmp_func = Some(
            sort_user_func
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        );
        assoc_kind = ::core::mem::transmute::<
            libc::c_uint,
            assoc_kind_t,
        >(
            assoc_kind as libc::c_uint
                | (AVALUE as libc::c_int | AINDEX as libc::c_int) as libc::c_uint,
        );
        elem_size = 2 as libc::c_int;
        code = bcalloc(Op_func_call, 2 as libc::c_int, 0 as libc::c_int);
        (*code).x.xn = f;
        (*code).d.name = 0 as *mut libc::c_char;
        (*code.offset(1 as libc::c_int as isize))
            .x
            .xl = 4 as libc::c_int as libc::c_long;
        (*code).nexti = bcalloc(Op_stop, 1 as libc::c_int, 0 as libc::c_int);
        save_rule = currule;
        currule = 0 as libc::c_int;
        PUSH_CODE(code);
    }
    akind.flags = assoc_kind as libc::c_uint as flagvals;
    list = ((*(*symbol).sub.nodep.l.lp).list)
        .expect("non-null function pointer")(symbol, &mut akind);
    assoc_kind = akind.flags as assoc_kind_t;
    if !list.is_null() && cmp_func.is_some()
        && assoc_kind as libc::c_uint
            & (AASC as libc::c_int | ADESC as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        num_elems = (*symbol).sub.nodep.reflags as libc::c_ulong;
        qsort(
            list as *mut libc::c_void,
            num_elems,
            (elem_size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as libc::c_ulong),
            cmp_func,
        );
        if sort_ctxt as libc::c_uint == SORTED_IN as libc::c_int as libc::c_uint
            && assoc_kind as libc::c_uint
                & (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
                == (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
        {
            j = 1 as libc::c_int as libc::c_ulong;
            while j < num_elems {
                let ref mut fresh15 = *list.offset(j as isize);
                *fresh15 = *list
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize,
                    );
                j = j.wrapping_add(1);
                j;
            }
            list = erealloc_real(
                list as *mut libc::c_void,
                num_elems
                    .wrapping_mul(::core::mem::size_of::<*mut NODE>() as libc::c_ulong),
                b"assoc_list\0" as *const u8 as *const libc::c_char,
                b"list\0" as *const u8 as *const libc::c_char,
                b"array.c\0" as *const u8 as *const libc::c_char,
                1458 as libc::c_int,
            ) as *mut *mut NODE;
        }
    }
    if cmp_func
        == Some(
            sort_user_func
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        )
    {
        code = POP_CODE();
        currule = save_rule;
        bcfree((*code).nexti);
        bcfree(code);
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn new_array_element() -> *mut NODE {
    let mut n: *mut NODE = make_number.expect("non-null function pointer")(0.0f64);
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    sp = emalloc_real(
        2 as libc::c_int as size_t,
        b"new_array_element\0" as *const u8 as *const libc::c_char,
        b"sp\0" as *const u8 as *const libc::c_char,
        b"array.c\0" as *const u8 as *const libc::c_char,
        1480 as libc::c_int,
    ) as *mut libc::c_char;
    let ref mut fresh16 = *sp.offset(1 as libc::c_int as isize);
    *fresh16 = '\0' as i32 as libc::c_char;
    *sp.offset(0 as libc::c_int as isize) = *fresh16;
    (*n).sub.val.sp = sp;
    (*n).sub.val.slen = 0 as libc::c_int as size_t;
    (*n).sub.val.idx = -(1 as libc::c_int);
    (*n)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(
        (*n).flags as libc::c_uint
            | (MALLOC as libc::c_int | STRING as libc::c_int | STRCUR as libc::c_int)
                as libc::c_uint,
    );
    (*n).type_0 = Node_elem_new;
    (*n).valref = 1 as libc::c_int as libc::c_long;
    return n;
}
