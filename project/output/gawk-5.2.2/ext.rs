#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn pma_free(ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut Nnull_string: *mut NODE;
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static awk_namespace: [libc::c_char; 0];
    static mut stack_ptr: *mut STACK_ITEM;
    static mut frame_ptr: *mut NODE;
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn track_ext_func(name: *const libc::c_char);
    fn check_special(name: *const libc::c_char) -> libc::c_int;
    fn is_letter(c: libc::c_int) -> bool;
    fn is_identchar(c: libc::c_int) -> bool;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    static mut api_impl: gawk_api_t;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn install_symbol(name: *const libc::c_char, type_0: NODETYPE) -> *mut NODE;
    fn bcalloc(op: OPCODE, size: libc::c_int, srcline: libc::c_int) -> *mut INSTRUCTION;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
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
pub type __off64_t = libc::c_long;
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
    awk_false = 0,
    awk_true,
impl awk_bool {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            awk_bool::awk_false => 0,
            awk_bool::awk_true => 1,
        }
    }
}

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
pub struct awk_input_parser {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_input_parser,
}
pub type awk_input_parser_t = awk_input_parser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const libc::c_char,
    pub mode: *const libc::c_char,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_ferror: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_fclose: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_wrapper {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_output_wrapper,
}
pub type awk_output_wrapper_t = awk_output_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_two_way_processor {
    pub name: *const libc::c_char,
    pub can_take_two_way: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut awk_input_buf_t,
            *mut awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub next: *mut awk_two_way_processor,
}
pub type awk_two_way_processor_t = awk_two_way_processor;
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
impl AWK_NUMBER_TYPE {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE => 0,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR => 1,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ => 2,
        }
    }
}

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
impl awk_valtype_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            awk_valtype_t::AWK_UNDEFINED => 0,
            awk_valtype_t::AWK_NUMBER => 1,
            awk_valtype_t::AWK_STRING => 2,
            awk_valtype_t::AWK_REGEX => 3,
            awk_valtype_t::AWK_STRNUM => 4,
            awk_valtype_t::AWK_ARRAY => 5,
            awk_valtype_t::AWK_SCALAR => 6,
            awk_valtype_t::AWK_VALUE_COOKIE => 7,
            awk_valtype_t::AWK_BOOL => 8,
        }
    }
}

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
pub struct awk_element {
    pub next: *mut awk_element,
    pub flags: C2RustUnnamed_0,
    pub index: awk_value_t,
    pub value: awk_value_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    AWK_ELEMENT_DELETE = 1,
    AWK_ELEMENT_DEFAULT = 0,
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_0::AWK_ELEMENT_DELETE => 1,
            C2RustUnnamed_0::AWK_ELEMENT_DEFAULT => 0,
        }
    }
}

pub type awk_element_t = awk_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_flat_array {
    pub opaque1: *mut libc::c_void,
    pub opaque2: *mut libc::c_void,
    pub count: size_t,
    pub elements: [awk_element_t; 1],
}
pub type awk_flat_array_t = awk_flat_array;
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
pub type awk_ext_id_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gawk_api {
    pub major_version: libc::c_int,
    pub minor_version: libc::c_int,
    pub gmp_major_version: libc::c_int,
    pub gmp_minor_version: libc::c_int,
    pub mpfr_major_version: libc::c_int,
    pub mpfr_minor_version: libc::c_int,
    pub do_flags: [libc::c_int; 6],
    pub api_add_ext_func: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *mut awk_ext_func_t,
        ) -> awk_bool_t,
    >,
    pub api_register_input_parser: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_input_parser_t) -> (),
    >,
    pub api_register_output_wrapper: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_output_wrapper_t) -> (),
    >,
    pub api_register_two_way_processor: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_two_way_processor_t) -> (),
    >,
    pub api_awk_atexit: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
            *mut libc::c_void,
        ) -> (),
    >,
    pub api_register_ext_version: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
    >,
    pub api_fatal: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_warning: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_lintwarn: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_nonfatal: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_update_ERRNO_int: Option::<
        unsafe extern "C" fn(awk_ext_id_t, libc::c_int) -> (),
    >,
    pub api_update_ERRNO_string: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
    >,
    pub api_unset_ERRNO: Option::<unsafe extern "C" fn(awk_ext_id_t) -> ()>,
    pub api_get_argument: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            size_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_argument: Option::<
        unsafe extern "C" fn(awk_ext_id_t, size_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_sym_lookup: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *const libc::c_char,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *const libc::c_char,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_lookup_scalar: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_scalar_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update_scalar: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_scalar_t, *mut awk_value_t) -> awk_bool_t,
    >,
    pub api_create_value: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *mut awk_value_t,
            *mut awk_value_cookie_t,
        ) -> awk_bool_t,
    >,
    pub api_release_value: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_value_cookie_t) -> awk_bool_t,
    >,
    pub api_get_element_count: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *mut size_t) -> awk_bool_t,
    >,
    pub api_get_array_element: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_array_element: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            *const awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_del_array_element: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *const awk_value_t) -> awk_bool_t,
    >,
    pub api_create_array: Option::<unsafe extern "C" fn(awk_ext_id_t) -> awk_array_t>,
    pub api_clear_array: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_flatten_array_typed: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut *mut awk_flat_array_t,
            awk_valtype_t,
            awk_valtype_t,
        ) -> awk_bool_t,
    >,
    pub api_release_flattened_array: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut awk_flat_array_t,
        ) -> awk_bool_t,
    >,
    pub api_malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub api_calloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub api_realloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub api_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub api_get_mpfr: Option::<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_mpz: Option::<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_file: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            libc::c_int,
            *mut *const awk_input_buf_t,
            *mut *const awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub api_destroy_array: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
}
pub type gawk_api_t = gawk_api;
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
impl nodevals {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            nodevals::Node_illegal => 0,
            nodevals::Node_val => 1,
            nodevals::Node_regex => 2,
            nodevals::Node_dynregex => 3,
            nodevals::Node_var => 4,
            nodevals::Node_var_array => 5,
            nodevals::Node_var_new => 6,
            nodevals::Node_elem_new => 7,
            nodevals::Node_param_list => 8,
            nodevals::Node_func => 9,
            nodevals::Node_ext_func => 10,
            nodevals::Node_builtin_func => 11,
            nodevals::Node_array_ref => 12,
            nodevals::Node_array_tree => 13,
            nodevals::Node_array_leaf => 14,
            nodevals::Node_dump_array => 15,
            nodevals::Node_arrayfor => 16,
            nodevals::Node_frame => 17,
            nodevals::Node_instruction => 18,
            nodevals::Node_final => 19,
        }
    }
}

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
impl flagvals {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            flagvals::REGEX => 524288,
            flagvals::NUMCONSTSTR => 262144,
            flagvals::XARRAY => 131072,
            flagvals::HALFHAT => 65536,
            flagvals::ARRAYMAXED => 32768,
            flagvals::NULL_FIELD => 16384,
            flagvals::NO_EXT_SET => 8192,
            flagvals::MPZN => 4096,
            flagvals::MPFN => 2048,
            flagvals::WSTRCUR => 1024,
            flagvals::INTIND => 512,
            flagvals::NUMINT => 256,
            flagvals::INTLSTR => 128,
            flagvals::BOOLVAL => 64,
            flagvals::USER_INPUT => 32,
            flagvals::NUMBER => 16,
            flagvals::NUMCUR => 8,
            flagvals::STRCUR => 4,
            flagvals::STRING => 2,
            flagvals::MALLOC => 1,
        }
    }
}

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
impl commenttype {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            commenttype::FOR_COMMENT => 3,
            commenttype::BLOCK_COMMENT => 2,
            commenttype::EOL_COMMENT => 1,
        }
    }
}

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
impl reflagvals {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            reflagvals::CONSTANT => 1,
            reflagvals::FS_DFLT => 2,
        }
    }
}

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
impl opcodeval {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            opcodeval::Op_final => 122,
            opcodeval::Op_parens => 121,
            opcodeval::Op_cond_exp => 120,
            opcodeval::Op_K_function => 119,
            opcodeval::Op_K_else => 118,
            opcodeval::Op_K_if => 117,
            opcodeval::Op_K_switch => 116,
            opcodeval::Op_K_while => 115,
            opcodeval::Op_K_arrayfor => 114,
            opcodeval::Op_K_for => 113,
            opcodeval::Op_K_do => 112,
            opcodeval::Op_list => 111,
            opcodeval::Op_symbol => 110,
            opcodeval::Op_token => 109,
            opcodeval::Op_stop => 108,
            opcodeval::Op_atexit => 107,
            opcodeval::Op_lint_plus => 106,
            opcodeval::Op_lint => 105,
            opcodeval::Op_breakpoint => 104,
            opcodeval::Op_exec_count => 103,
            opcodeval::Op_comment => 102,
            opcodeval::Op_func => 101,
            opcodeval::Op_after_endfile => 100,
            opcodeval::Op_after_beginfile => 99,
            opcodeval::Op_subscript_assign => 98,
            opcodeval::Op_field_assign => 97,
            opcodeval::Op_var_assign => 96,
            opcodeval::Op_var_update => 95,
            opcodeval::Op_arrayfor_final => 94,
            opcodeval::Op_arrayfor_incr => 93,
            opcodeval::Op_arrayfor_init => 92,
            opcodeval::Op_newfile => 91,
            opcodeval::Op_get_record => 90,
            opcodeval::Op_jmp_false => 89,
            opcodeval::Op_jmp_true => 88,
            opcodeval::Op_jmp => 87,
            opcodeval::Op_pop => 86,
            opcodeval::Op_no_op => 85,
            opcodeval::Op_field_spec_lhs => 84,
            opcodeval::Op_subscript_lhs => 83,
            opcodeval::Op_push_lhs => 82,
            opcodeval::Op_push_param => 81,
            opcodeval::Op_push_array => 80,
            opcodeval::Op_push_re => 79,
            opcodeval::Op_push_i => 78,
            opcodeval::Op_push_arg_untyped => 77,
            opcodeval::Op_push_arg => 76,
            opcodeval::Op_push => 75,
            opcodeval::Op_indirect_func_call => 74,
            opcodeval::Op_func_call => 73,
            opcodeval::Op_in_array => 72,
            opcodeval::Op_ext_builtin => 71,
            opcodeval::Op_sub_builtin => 70,
            opcodeval::Op_builtin => 69,
            opcodeval::Op_K_namespace => 68,
            opcodeval::Op_K_nextfile => 67,
            opcodeval::Op_K_getline => 66,
            opcodeval::Op_K_getline_redir => 65,
            opcodeval::Op_K_delete_loop => 64,
            opcodeval::Op_K_delete => 63,
            opcodeval::Op_K_return_from_eval => 62,
            opcodeval::Op_K_return => 61,
            opcodeval::Op_K_exit => 60,
            opcodeval::Op_K_next => 59,
            opcodeval::Op_K_printf => 58,
            opcodeval::Op_K_print_rec => 57,
            opcodeval::Op_K_print => 56,
            opcodeval::Op_K_continue => 55,
            opcodeval::Op_K_break => 54,
            opcodeval::Op_K_default => 53,
            opcodeval::Op_K_case => 52,
            opcodeval::Op_rule => 51,
            opcodeval::Op_nomatch => 50,
            opcodeval::Op_match_rec => 49,
            opcodeval::Op_match => 48,
            opcodeval::Op_geq => 47,
            opcodeval::Op_leq => 46,
            opcodeval::Op_greater => 45,
            opcodeval::Op_less => 44,
            opcodeval::Op_notequal => 43,
            opcodeval::Op_equal => 42,
            opcodeval::Op_or_final => 41,
            opcodeval::Op_or => 40,
            opcodeval::Op_and_final => 39,
            opcodeval::Op_and => 38,
            opcodeval::Op_assign_concat => 37,
            opcodeval::Op_assign_exp => 36,
            opcodeval::Op_assign_minus => 35,
            opcodeval::Op_assign_plus => 34,
            opcodeval::Op_assign_mod => 33,
            opcodeval::Op_assign_quotient => 32,
            opcodeval::Op_assign_times => 31,
            opcodeval::Op_store_field_exp => 30,
            opcodeval::Op_store_field => 29,
            opcodeval::Op_store_sub => 28,
            opcodeval::Op_store_var => 27,
            opcodeval::Op_assign => 26,
            opcodeval::Op_not => 25,
            opcodeval::Op_field_spec => 24,
            opcodeval::Op_unary_plus => 23,
            opcodeval::Op_unary_minus => 22,
            opcodeval::Op_postdecrement => 21,
            opcodeval::Op_postincrement => 20,
            opcodeval::Op_predecrement => 19,
            opcodeval::Op_preincrement => 18,
            opcodeval::Op_sub_array => 17,
            opcodeval::Op_subscript => 16,
            opcodeval::Op_cond_pair => 15,
            opcodeval::Op_line_range => 14,
            opcodeval::Op_concat => 13,
            opcodeval::Op_exp_i => 12,
            opcodeval::Op_exp => 11,
            opcodeval::Op_minus_i => 10,
            opcodeval::Op_minus => 9,
            opcodeval::Op_plus_i => 8,
            opcodeval::Op_plus => 7,
            opcodeval::Op_mod_i => 6,
            opcodeval::Op_mod => 5,
            opcodeval::Op_quotient_i => 4,
            opcodeval::Op_quotient => 3,
            opcodeval::Op_times_i => 2,
            opcodeval::Op_times => 1,
            opcodeval::Op_illegal => 0,
        }
    }
}

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
    SRC_CMDLINE = 1,
    SRC_STDIN,
    SRC_FILE,
    SRC_INC,
    SRC_EXTLIB,
impl srctype {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            srctype::SRC_CMDLINE => 1,
            srctype::SRC_STDIN => 2,
            srctype::SRC_FILE => 3,
            srctype::SRC_INC => 4,
            srctype::SRC_EXTLIB => 5,
        }
    }
}

pub type SRCFILE = srcfile;
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
impl do_flag_values {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            do_flag_values::DO_MPFR => 32768,
            do_flag_values::DO_DEBUG => 16384,
            do_flag_values::DO_PROFILE => 8192,
            do_flag_values::DO_SANDBOX => 4096,
            do_flag_values::DO_TIDY_MEM => 2048,
            do_flag_values::DO_DUMP_VARS => 1024,
            do_flag_values::DO_PRETTY_PRINT => 512,
            do_flag_values::DO_INTERVALS => 256,
            do_flag_values::DO_NON_DEC_DATA => 128,
            do_flag_values::DO_INTL => 64,
            do_flag_values::DO_POSIX => 32,
            do_flag_values::DO_TRADITIONAL => 16,
            do_flag_values::DO_LINT_OLD => 8,
            do_flag_values::DO_LINT_ALL => 4,
            do_flag_values::DO_LINT_EXTENSIONS => 2,
            do_flag_values::DO_LINT_INVALID => 1,
            do_flag_values::DO_FLAG_NONE => 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
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
pub unsafe extern "C" fn load_ext(mut lib_name: *const libc::c_char) {
    let mut install_func: Option::<
        unsafe extern "C" fn(*const gawk_api_t, awk_ext_id_t) -> libc::c_int,
    > = None;
    let mut dl: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut flags: libc::c_int = 0x1 as libc::c_int;
    let mut gpl_compat: *mut libc::c_int = 0 as *mut libc::c_int;
    if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint != 0 {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 51 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"extensions are not allowed in sandbox mode\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 54 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"-l / @load are gawk extensions\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if lib_name.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 57 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"load_ext: received NULL lib_name\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    dl = dlopen(lib_name, flags);
    if dl.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 60 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"load_ext: cannot open library `%s': %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            lib_name,
            dlerror(),
        );
    }
    gpl_compat = dlsym(
        dl,
        b"plugin_is_GPL_compatible\0" as *const u8 as *const libc::c_char,
    ) as *mut libc::c_int;
    if gpl_compat.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 66 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"load_ext: library `%s': does not define `plugin_is_GPL_compatible': %s\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lib_name,
            dlerror(),
        );
    }
    install_func = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*const gawk_api_t, awk_ext_id_t) -> libc::c_int>,
    >(dlsym(dl, b"dl_load\0" as *const u8 as *const libc::c_char));
    if install_func.is_none() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 72 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"load_ext: library `%s': cannot call function `%s': %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            lib_name,
            b"dl_load\0" as *const u8 as *const libc::c_char,
            dlerror(),
        );
    }
    if install_func
        .expect("non-null function pointer")(&mut api_impl, 0 as *mut libc::c_void)
        == 0 as libc::c_int
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 76 as libc::c_int);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"load_ext: library `%s' initialization routine `%s' failed\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lib_name,
            b"dl_load\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_builtin(
    mut name_space: *const libc::c_char,
    mut funcinfo: *const awk_ext_func_t,
) -> awk_bool_t {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut b: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut name: *const libc::c_char = (*funcinfo).name;
    let mut count: libc::c_int = (*funcinfo).max_expected_args as libc::c_int;
    let mut install_name: *const libc::c_char = 0 as *const libc::c_char;
    if name.is_null() || *name as libc::c_int == '\0' as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 92 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"make_builtin: missing function name\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !is_valid_identifier(name) {
        return awk_false;
    }
    if *name_space.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || strcmp(name_space, awk_namespace.as_ptr()) == 0 as libc::c_int
    {
        if check_special(name) >= 0 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"ext.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
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
                    b"make_builtin: cannot use gawk built-in `%s' as function name\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        f = lookup(name);
        install_name = estrdup(name, strlen(name));
    } else {
        if !is_valid_identifier(name_space) {
            return awk_false;
        }
        if check_special(name_space) >= 0 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"ext.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int,
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
                    b"make_builtin: cannot use gawk built-in `%s' as namespace name\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name_space,
            );
        }
        if check_special(name) >= 0 as libc::c_int {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"ext.c\0" as *const u8 as *const libc::c_char,
                111 as libc::c_int,
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
                    b"make_builtin: cannot use gawk built-in `%s' as function name\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
        let mut len: size_t = (strlen(name_space))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(name))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        buf = emalloc_real(
            len,
            b"make_builtin\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"ext.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
        ) as *mut libc::c_char;
        sprintf(buf, b"%s::%s\0" as *const u8 as *const libc::c_char, name_space, name);
        install_name = buf;
        f = lookup(install_name);
    }
    if !f.is_null() {
        if (*f).type_0 as libc::c_uint == Node_func as libc::c_int as libc::c_uint {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"ext.c\0" as *const u8 as *const libc::c_char,
                126 as libc::c_int,
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
                    b"make_builtin: cannot redefine function `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        } else if (*f).type_0 as libc::c_uint
            == Node_ext_func as libc::c_int as libc::c_uint
        {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"ext.c\0" as *const u8 as *const libc::c_char,
                    130 as libc::c_int,
                );
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"make_builtin: function `%s' already defined\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
            }
            pma_free(install_name as *mut libc::c_void);
            return awk_false;
        } else {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"ext.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
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
                    b"make_builtin: function name `%s' previously defined\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
    }
    if count < 0 as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 139 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"make_builtin: negative argument count for function `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    b = bcalloc(Op_symbol, 1 as libc::c_int, 0 as libc::c_int);
    (*b).d.efptr = (*funcinfo).function;
    (*b).x.exf = funcinfo as *mut awk_ext_func_t;
    symbol = install_symbol(install_name, Node_ext_func);
    (*symbol).sub.nodep.r.iptr = b;
    track_ext_func(name);
    return awk_true;
}
#[no_mangle]
pub unsafe extern "C" fn get_argument(mut i: libc::c_int) -> *mut NODE {
    let mut t: *mut NODE = 0 as *mut NODE;
    let mut arg_count: libc::c_int = 0;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    pc = (*(*stack_ptr).rptr).sub.nodep.r.iptr;
    arg_count = (*pc).x.xl as libc::c_int;
    if i < 0 as libc::c_int || i >= arg_count {
        return 0 as *mut NODE;
    }
    t = (*stack_ptr.offset(-((arg_count - i) as isize))).rptr;
    if (*t).type_0 as libc::c_uint == Node_param_list as libc::c_int as libc::c_uint {
        t = *((*frame_ptr).sub.nodep.r.av).offset((*t).sub.nodep.l.ll as isize);
    }
    if (*t).type_0 as libc::c_uint == Node_array_ref as libc::c_int as libc::c_uint {
        if (*(*t).sub.nodep.l.lptr).type_0 as libc::c_uint
            == Node_var as libc::c_int as libc::c_uint
        {
            (*t).type_0 = Node_var;
            (*t).sub.nodep.l.lptr = Nnull_string;
            return t;
        }
        return (*t).sub.nodep.l.lptr;
    }
    if (*t).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint {
        return Nnull_string;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn get_actual_argument(
    mut t: *mut NODE,
    mut i: libc::c_int,
    mut want_array: bool,
) -> *mut NODE {
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    pc = (*(*stack_ptr).rptr).sub.nodep.r.iptr;
    fname = (*pc.offset(1 as libc::c_int as isize)).d.name;
    if (*t).type_0 as libc::c_uint == Node_var_new as libc::c_int as libc::c_uint
        || (*t).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        if want_array {
            return force_array(t, 0 as libc::c_int != 0)
        } else {
            (*t).type_0 = Node_var;
            (*t).sub.nodep.l.lptr = dupnode(Nnull_string);
            return (*t).sub.nodep.l.lptr;
        }
    }
    if want_array {
        if (*t).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"ext.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int,
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
                    b"function `%s': argument #%d: attempt to use scalar as an array\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                i + 1 as libc::c_int,
            );
        }
    } else if (*t).type_0 as libc::c_uint != Node_val as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"ext.c\0" as *const u8 as *const libc::c_char, 219 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"function `%s': argument #%d: attempt to use array as a scalar\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            i + 1 as libc::c_int,
        );
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn close_extensions() {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    if srcfiles.is_null() {
        return;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as libc::c_uint == SRC_EXTLIB as libc::c_int as libc::c_uint
            && ((*s).fini_func).is_some()
        {
            ::core::mem::transmute::<
                _,
                fn(),
            >(
                (Some(((*s).fini_func).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )();
        }
        s = (*s).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_identifier(mut name: *const libc::c_char) -> bool {
    let mut sp: *const libc::c_char = name;
    let mut c: libc::c_int = 0;
    if !is_letter(*sp as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    sp = sp.offset(1);
    sp;
    loop {
        let fresh0 = sp;
        sp = sp.offset(1);
        c = *fresh0 as libc::c_int;
        if !(c != '\0' as i32) {
            break;
        }
        if !is_identchar(c) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
