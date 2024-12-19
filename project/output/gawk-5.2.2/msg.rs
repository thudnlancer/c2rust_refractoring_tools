#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut _IO_FILE;
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut FNR: libc::c_long;
    static mut FILENAME_node: *mut NODE;
    static mut do_flags: do_flag_values;
    static mut exit_val: libc::c_int;
    static mut myname: *const libc::c_char;
    fn close_extensions();
    fn run_ext_exit_handlers(exitval: libc::c_int);
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    static mut output_fp: *mut FILE;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type va_list = __builtin_va_list;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
    awk_true,
    awk_false,
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
    AWK_NUMBER_TYPE_MPZ,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_DOUBLE,
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
    AWK_BOOL,
    AWK_VALUE_COOKIE,
    AWK_SCALAR,
    AWK_ARRAY,
    AWK_STRNUM,
    AWK_REGEX,
    AWK_STRING,
    AWK_NUMBER,
    AWK_UNDEFINED,
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
    Node_final,
    Node_instruction,
    Node_frame,
    Node_arrayfor,
    Node_dump_array,
    Node_array_leaf,
    Node_array_tree,
    Node_array_ref,
    Node_builtin_func,
    Node_ext_func,
    Node_func,
    Node_param_list,
    Node_elem_new,
    Node_var_new,
    Node_var_array,
    Node_var,
    Node_dynregex,
    Node_regex,
    Node_val,
    Node_illegal,
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
    FOR_COMMENT,
    BLOCK_COMMENT,
    EOL_COMMENT,
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
    FS_DFLT,
    CONSTANT,
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

#[no_mangle]
pub static mut sourceline: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut source: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut srcfile: *const libc::c_char = 0 as *const libc::c_char;
static mut srcline: libc::c_int = 0;
#[no_mangle]
pub static mut fatal_tag: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut fatal_tag_valid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn err(
    mut isfatal: bool,
    mut s: *const libc::c_char,
    mut emsg: *const libc::c_char,
    mut argp: ::core::ffi::VaList,
) {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut me: *const libc::c_char = 0 as *const libc::c_char;
    static mut first: bool = 1 as libc::c_int != 0;
    static mut add_src_info: bool = 0 as libc::c_int != 0;
    static mut lineno_val: libc::c_long = 0 as libc::c_int as libc::c_long;
    if first {
        first = 0 as libc::c_int != 0;
        add_src_info = !(getenv(b"GAWK_MSG_SRC\0" as *const u8 as *const libc::c_char))
            .is_null();
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
        {
            let mut n: *mut NODE = lookup(
                b"LINENO\0" as *const u8 as *const libc::c_char,
            );
            if !n.is_null()
                && (*n).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
            {
                lineno_val = (*(*n).sub.nodep.l.lptr).sub.val.fltnum as libc::c_long;
            }
        }
    }
    fflush(output_fp);
    me = myname;
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, me);
    if !srcfile.is_null() && add_src_info as libc::c_int != 0 {
        fprintf(
            stderr,
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            srcfile,
            srcline,
        );
        srcfile = 0 as *const libc::c_char;
    }
    if sourceline > 0 as libc::c_int {
        if !source.is_null() {
            fprintf(stderr, b"%s:\0" as *const u8 as *const libc::c_char, source);
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cmd. line:\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        fprintf(
            stderr,
            b"%ld: \0" as *const u8 as *const libc::c_char,
            sourceline as libc::c_long + lineno_val,
        );
    }
    if FNR > 0 as libc::c_int as libc::c_long {
        let mut len: libc::c_int = (*(*FILENAME_node).sub.nodep.l.lptr).sub.val.slen
            as libc::c_int;
        file = (*(*FILENAME_node).sub.nodep.l.lptr).sub.val.sp;
        _IO_putc('(' as i32, stderr);
        if !file.is_null() {
            fprintf(
                stderr,
                b"FILENAME=%.*s \0" as *const u8 as *const libc::c_char,
                len,
                file,
            );
        }
        fprintf(stderr, b"FNR=%ld) \0" as *const u8 as *const libc::c_char, FNR);
    }
    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, s);
    vfprintf(stderr, emsg, argp.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(stderr);
    if isfatal {
        gawk_exit(2 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn msg(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        b"\0" as *const u8 as *const libc::c_char,
        mesg,
        args_0.as_va_list(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn r_warning(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"warning: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        mesg,
        args_0.as_va_list(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn error(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"error: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        mesg,
        args_0.as_va_list(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn set_loc(mut file: *const libc::c_char, mut line: libc::c_int) {
    srcfile = file;
    srcline = line;
    file = srcfile;
    line = srcline;
}
#[no_mangle]
pub unsafe extern "C" fn r_fatal(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        1 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"fatal: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        mesg,
        args_0.as_va_list(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn gawk_exit(mut status: libc::c_int) {
    if fatal_tag_valid != 0 {
        exit_val = status;
        longjmp(fatal_tag.as_mut_ptr(), 1 as libc::c_int);
    }
    final_exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn final_exit(mut status: libc::c_int) -> ! {
    run_ext_exit_handlers(status);
    close_extensions();
    exit(status);
}
