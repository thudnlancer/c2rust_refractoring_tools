#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub struct awk_input {
    pub name: *const libc::c_char,
    pub fd: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub get_record: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *mut awk_input,
            *mut libc::c_int,
            *mut *mut libc::c_char,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> libc::c_int,
    >,
    pub read_func: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option::<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
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
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut libc::c_char,
    pub fullpath: *mut libc::c_char,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: libc::c_int,
    pub bufsize: size_t,
    pub buf: *mut libc::c_char,
    pub line_offset: *mut libc::c_int,
    pub fd: libc::c_int,
    pub maxlen: libc::c_int,
    pub fini_func: Option::<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut libc::c_char,
    pub lexend: *mut libc::c_char,
    pub lexeme: *mut libc::c_char,
    pub lexptr_begin: *mut libc::c_char,
    pub lasttok: libc::c_int,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const libc::c_char,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum srctype {
    SRC_EXTLIB = 5,
    SRC_INC = 4,
    SRC_FILE = 3,
    SRC_STDIN = 2,
    SRC_CMDLINE = 1,
}  // end of enum

pub type SRCFILE = srcfile;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut quote: libc::c_char = '\'' as i32 as libc::c_char;
#[no_mangle]
pub static mut defpath: *const libc::c_char = b".:/usr/local/share/awk\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut deflibpath: *const libc::c_char = b"/usr/local/lib/gawk\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut envsep: libc::c_char = ':' as i32 as libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn gawk_name(
    mut filespec: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = strrchr(filespec, '/' as i32);
    return if p.is_null() {
        filespec as *mut libc::c_char
    } else {
        p.offset(1 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn os_arg_fixup(
    mut argcp: *mut libc::c_int,
    mut argvp: *mut *mut *mut libc::c_char,
) {}
#[no_mangle]
pub unsafe extern "C" fn os_devopen(
    mut name: *const libc::c_char,
    mut flag: libc::c_int,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn optimal_bufsize(
    mut fd: libc::c_int,
    mut stb: *mut stat,
) -> size_t {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut env_val: size_t = 0 as libc::c_int as size_t;
    static mut first: bool = 1 as libc::c_int != 0;
    static mut exact: bool = 0 as libc::c_int != 0;
    memset(
        stb as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<stat>() as libc::c_ulong,
    );
    if fstat(fd, stb) == -(1 as libc::c_int) {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./posix/gawkmisc.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"can't stat fd %d (%s)\0" as *const u8 as *const libc::c_char,
            fd,
            strerror(*__errno_location()),
        );
    }
    if first {
        first = 0 as libc::c_int != 0;
        val = getenv(b"AWKBUFSIZE\0" as *const u8 as *const libc::c_char);
        if !val.is_null() {
            if strcmp(val, b"exact\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                exact = 1 as libc::c_int != 0;
            } else if *(*__ctype_b_loc())
                .offset(*val as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                while *val as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*val as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    env_val = env_val
                        .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        .wrapping_add(*val as libc::c_ulong)
                        .wrapping_sub('0' as i32 as libc::c_ulong);
                    val = val.offset(1);
                    val;
                }
                return env_val;
            }
        }
    } else if !exact && env_val > 0 as libc::c_int as libc::c_ulong {
        return env_val
    }
    if (*stb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        && (0 as libc::c_int as libc::c_long) < (*stb).st_size
        && ((*stb).st_size
            < (if (*stb).st_blksize > 0 as libc::c_int as libc::c_long {
                (*stb).st_blksize
            } else {
                8192 as libc::c_int as libc::c_long
            }) || exact as libc::c_int != 0)
    {
        return (*stb).st_size as size_t;
    }
    return (if (*stb).st_blksize > 0 as libc::c_int as libc::c_long {
        (*stb).st_blksize
    } else {
        8192 as libc::c_int as libc::c_long
    }) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ispath(mut file: *const libc::c_char) -> libc::c_int {
    return (strchr(file, '/' as i32) != 0 as *mut libc::c_void as *mut libc::c_char)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isdirpunct(mut c: libc::c_int) -> libc::c_int {
    return (c == '/' as i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn os_close_on_exec(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut what: *const libc::c_char,
    mut dir: *const libc::c_char,
) {
    let mut curflags: libc::c_int = 0 as libc::c_int;
    if fd <= 2 as libc::c_int {
        return;
    }
    curflags = fcntl(fd, 1 as libc::c_int);
    if curflags < 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./posix/gawkmisc.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s %s `%s': could not get fd flags: (fcntl F_GETFD: %s)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            what,
            dir,
            name,
            strerror(*__errno_location()),
        );
        return;
    }
    curflags |= 1 as libc::c_int;
    if fcntl(fd, 2 as libc::c_int, curflags) < 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./posix/gawkmisc.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s %s `%s': could not set close-on-exec: (fcntl F_SETFD: %s)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            what,
            dir,
            name,
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn os_isdir(mut fd: libc::c_int) -> libc::c_int {
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return (fstat(fd, &mut sbuf) == 0 as libc::c_int
        && sbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn os_isreadable(
    mut iobuf: *const awk_input_buf_t,
    mut isdir: *mut bool,
) -> libc::c_int {
    *isdir = 0 as libc::c_int != 0;
    if (*iobuf).fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    match (*iobuf).sbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint {
        32768 | 8192 | 49152 | 4096 => return 1 as libc::c_int,
        16384 => {
            *isdir = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn os_is_setuid() -> libc::c_int {
    let mut uid: libc::c_long = 0;
    let mut euid: libc::c_long = 0;
    uid = getuid() as libc::c_long;
    euid = geteuid() as libc::c_long;
    return (euid == 0 as libc::c_int as libc::c_long && euid != uid) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn os_setbinmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn os_restore_mode(mut fd: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn os_isatty(mut fd: libc::c_int) -> libc::c_int {
    return isatty(fd);
}
#[no_mangle]
pub unsafe extern "C" fn files_are_same(
    mut path: *mut libc::c_char,
    mut src: *mut SRCFILE,
) -> libc::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return (stat(path, &mut st) == 0 as libc::c_int && st.st_dev == (*src).sbuf.st_dev
        && st.st_ino == (*src).sbuf.st_ino) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_sockets() {}
#[no_mangle]
pub unsafe extern "C" fn os_maybe_set_errno() {}
